use crate::unit::{Rect, Size};

#[derive(Debug, Clone)]
pub enum LayoutItem {
    Rows { unit: Size, items: Vec<LayoutItem> },
    Columns { unit: Size, items: Vec<LayoutItem> },

    Frame { padding: u16, item: Box<LayoutItem> },
    Widget(Size),
    Spacing(Size),
}

#[derive(Debug, Clone)]
pub struct Allocation {
    pub widgets: Vec<Rect>,
    pub frame: Vec<Rect>,
}

impl LayoutItem {
    pub fn allocate(&self, alloc: &Rect, full_alloc: &mut Allocation) {
        match self {
            Self::Rows { items, .. } | Self::Columns { items, .. } => {
                full_alloc.frame.push(*alloc);

                let is_row = matches!(self, Self::Rows { .. });

                let allocation = if is_row { alloc.height } else { alloc.width };

                let fixed_total: u16 = items
                    .iter()
                    .map(|item| item.get_size().get_fixed(allocation))
                    .sum();

                let remaining = allocation.saturating_sub(fixed_total);

                let total_weight = items
                    .iter()
                    .map(|item| item.get_size().get_flex())
                    .sum::<u16>();

                let base_allocs: Vec<u16> = items
                    .iter()
                    .map(|item| item.get_size().get(allocation, remaining, total_weight))
                    .collect();

                let flex_alloc_total: u16 = items
                    .iter()
                    .zip(base_allocs.iter())
                    .filter(|(item, _)| item.get_size().get_flex() > 0)
                    .map(|(_, a)| a)
                    .sum();

                let gap = remaining.saturating_sub(flex_alloc_total);

                let fill_count = items
                    .iter()
                    .filter(|item| item.get_size().is_fill())
                    .count() as u16;

                let gap_per_fill = if fill_count > 0 { gap / fill_count } else { 0 };
                let gap_remainder = if fill_count > 0 { gap % fill_count } else { 0 };

                let mut curr = if is_row { alloc.y } else { alloc.x };
                let mut fill_idx = 0u16;

                for (i, item) in items.iter().enumerate() {
                    let size = item.get_size();
                    let mut unit_alloc = base_allocs[i];

                    if size.is_fill() {
                        unit_alloc += gap_per_fill;
                        if fill_idx == 0 {
                            unit_alloc += gap_remainder;
                        }
                        fill_idx += 1;
                    }

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

            Self::Spacing(_) => {
                full_alloc.frame.push(*alloc);
            }
        }
    }

    pub fn get_size<'a>(&'a self) -> &'a Size {
        match self {
            Self::Rows { unit, .. } => unit,
            Self::Columns { unit, .. } => unit,
            Self::Widget(unit) | Self::Spacing(unit) => unit,
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
    LayoutItem::Rows {
        unit: Size::Percent(100),
        items: rows,
    }
}
