#[derive(Debug, Clone, Copy)]
pub enum Size {
    Fixed(u16),
    Flex(u16),
    Percent(u16),
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub enum LayoutItem {
    Rows { unit: Size, items: Vec<LayoutItem> },
    Columns { unit: Size, items: Vec<LayoutItem> },
    Widget(Size),
}

impl LayoutItem {
    pub fn allocate(&self, alloc: &Rect, layout: &mut Vec<Rect>) {
        match self {
            Self::Rows { items, .. } | Self::Columns { items, .. } => {
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

                    item.allocate(&item_alloc, layout);

                    layout.push(item_alloc);
                }
            }

            Self::Widget(_) => {}
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

impl Size {
    pub fn get_fixed(&self, alloc: u16) -> u16 {
        match self {
            Self::Fixed(v) => *v,
            Self::Percent(v) => alloc / (v / 100),
            Self::Flex(_) => 0,
        }
    }

    pub fn get_flex(&self) -> u16 {
        match self {
            Self::Flex(f) => *f,
            _ => 0,
        }
    }

    pub fn get(&self, alloc: u16, remaining: u16, total_weight: u16) -> u16 {
        match self {
            Self::Fixed(v) => *v,
            Self::Percent(v) => alloc / (v / 100),
            Self::Flex(v) => remaining * v / total_weight,
        }
    }
}

pub fn layout(rows: Vec<LayoutItem>) -> LayoutItem {
    LayoutItem::Rows {
        unit: Size::Percent(100),
        items: rows,
    }
}
