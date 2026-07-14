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
    Row {
        width: Size,
        height: Size,
        items: Vec<LayoutItem>,
    },
    Column {
        width: Size,
        height: Size,
        items: Vec<LayoutItem>,
    },
    Widget {
        width: Size,
        height: Size,
    },
}

impl LayoutItem {
    pub fn allocate(&self, current_alloc: &Rect, alloc: &mut Vec<Rect>) {
        match self {
            Self::Row { items, .. } => {
                let remaining = current_alloc.height
                    - items
                        .iter()
                        .map(|item| item.get_size().1.get_fixed(current_alloc.height))
                        .sum::<u16>();

                let total_weight = current_alloc.height
                    - items
                        .iter()
                        .map(|item| item.get_size().1.get_flex())
                        .sum::<u16>();

                let mut curr = current_alloc.y;

                for item in items {
                    let size = item.get_size();

                    let item_alloc = Rect {
                        width: current_alloc.width,
                        height: size.1.get(current_alloc.height, remaining, total_weight),
                        x: current_alloc.x,
                        y: curr,
                    };

                    curr += item_alloc.height;

                    item.allocate(&item_alloc, alloc);

                    alloc.push(item_alloc);
                }
            }

            Self::Column { items, .. } => {
                let remaining = current_alloc.width
                    - items
                        .iter()
                        .map(|item| item.get_size().0.get_fixed(current_alloc.width))
                        .sum::<u16>();

                let total_weight = current_alloc.height
                    - items
                        .iter()
                        .map(|item| item.get_size().0.get_flex())
                        .sum::<u16>();

                let mut curr = current_alloc.x;

                for item in items {
                    let size = item.get_size();

                    let item_alloc = Rect {
                        width: size.1.get(current_alloc.width, remaining, total_weight),
                        height: current_alloc.height,
                        x: curr,
                        y: current_alloc.y,
                    };

                    curr += item_alloc.width;

                    item.allocate(&item_alloc, alloc);

                    alloc.push(item_alloc);
                }
            }

            Self::Widget { .. } => {}
        }
    }

    pub fn get_size<'a>(&'a self) -> (&'a Size, &'a Size) {
        match self {
            Self::Row { width, height, .. } => (width, height),
            Self::Column { width, height, .. } => (width, height),
            Self::Widget { width, height } => (width, height),
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
