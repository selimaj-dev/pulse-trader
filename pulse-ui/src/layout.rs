use crate::unit::{Rect, Size};

#[derive(Debug, Clone)]
pub enum LayoutItem {
    Rows { unit: Size, items: Vec<LayoutItem> },
    Columns { unit: Size, items: Vec<LayoutItem> },
    Widget(Size),
}

#[derive(Debug, Clone)]
pub struct Allocation {
    pub widget_alloc: Vec<Rect>,
    pub frame_alloc: Vec<Rect>,
}

impl LayoutItem {
    pub fn allocate(&self, alloc: &Rect, frame: &mut Vec<Rect>, widgets: &mut Vec<Rect>) {
        match self {
            Self::Rows { items, .. } | Self::Columns { items, .. } => {
                frame.push(*alloc);

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

                    item.allocate(&item_alloc, frame, widgets);
                }
            }

            Self::Widget(_) => {
                widgets.push(*alloc);
            }
        }
    }

    pub fn get_size<'a>(&'a self) -> &'a Size {
        match self {
            Self::Rows { unit, .. } => unit,
            Self::Columns { unit, .. } => unit,
            Self::Widget(unit) => unit,
        }
    }
}

impl Rect {
    pub fn allocate(&self, layout: &LayoutItem) -> Allocation {
        let mut widget_alloc = Vec::new();
        let mut frame_alloc = Vec::new();

        layout.allocate(self, &mut frame_alloc, &mut widget_alloc);

        Allocation {
            widget_alloc,
            frame_alloc,
        }
    }
}

pub fn layout(rows: Vec<LayoutItem>) -> LayoutItem {
    LayoutItem::Rows {
        unit: Size::Percent(100),
        items: rows,
    }
}
