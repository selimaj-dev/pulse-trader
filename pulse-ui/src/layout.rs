use crate::unit::{Rect, Size};

#[derive(Debug, Clone)]
pub enum LayoutItem {
    Rows { unit: Size, items: Vec<LayoutItem> },
    SpacedRows { unit: Size, items: Vec<LayoutItem> },

    Columns { unit: Size, items: Vec<LayoutItem> },
    SpacedColumns { unit: Size, items: Vec<LayoutItem> },

    Frame { padding: u16, item: Box<LayoutItem> },
    Widget(Size),
}

#[derive(Debug, Clone)]
pub struct Allocation {
    pub widgets: Vec<Rect>,
    pub frame: Vec<Rect>,
}

impl LayoutItem {
    pub fn allocate(&self, alloc: &Rect, full_alloc: &mut Allocation) {
        match self {
            Self::Rows { items, .. }
            | Self::Columns { items, .. }
            | Self::SpacedRows { items, .. }
            | Self::SpacedColumns { items, .. } => {
                full_alloc.frame.push(*alloc);

                let is_row = matches!(self, Self::Rows { .. });

                let allocation = if is_row { alloc.height } else { alloc.width };

                let remaining = allocation
                    - items
                        .iter()
                        .map(|item| item.get_size().get_fixed(alloc.height))
                        .sum::<u16>();

                let total_weight = items
                    .iter()
                    .map(|item| item.get_size().get_flex())
                    .sum::<u16>();

                let mut curr = if is_row { alloc.y } else { alloc.x };

                for item in items {
                    let size = item.get_size();

                    let unit_alloc = size.get(allocation, remaining, total_weight);

                    let item_alloc = if is_row {
                        Rect {
                            width: alloc.width,
                            height: unit_alloc,
                            x: alloc.x,
                            y: curr,
                        }
                    } else {
                        Rect {
                            width: unit_alloc,
                            height: alloc.height,
                            x: curr,
                            y: alloc.y,
                        }
                    };

                    curr += unit_alloc;

                    item.allocate(&item_alloc, full_alloc);
                }
            }

            Self::Frame { padding, item } => {
                full_alloc.frame.push(*alloc);

                let mut item_alloc = alloc.clone();

                item_alloc.width -= padding * 2;
                item_alloc.height -= padding * 2;

                item_alloc.x += padding;
                item_alloc.y += padding;

                item.allocate(&item_alloc, full_alloc);
            }

            Self::Widget(_) => {
                full_alloc.widgets.push(*alloc);
            }
        }
    }

    pub fn get_size<'a>(&'a self) -> &'a Size {
        match self {
            Self::Rows { unit, .. } | Self::SpacedRows { unit, .. } => unit,
            Self::Columns { unit, .. } | Self::SpacedColumns { unit, .. } => unit,
            Self::Widget(unit) => unit,
            Self::Frame { item, .. } => item.get_size(),
        }
    }
}

impl Rect {
    pub fn allocate(&self, layout: &LayoutItem) -> Allocation {
        let mut alloc = Allocation {
            widgets: Vec::new(),
            frame: Vec::new(),
        };

        layout.allocate(self, &mut alloc);

        alloc
    }
}

pub fn layout(rows: Vec<LayoutItem>) -> LayoutItem {
    LayoutItem::Columns {
        unit: Size::Percent(100),
        items: rows,
    }
}
