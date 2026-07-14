#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

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

impl Size {
    pub fn get_fixed(&self, alloc: u16) -> u16 {
        match self {
            Self::Fixed(v) => *v,
            Self::Percent(v) => alloc * (*v) / 100,
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
            Self::Percent(v) => alloc * (*v) / 100,
            Self::Flex(v) => {
                if total_weight == 0 {
                    0
                } else {
                    remaining * v / total_weight
                }
            }
        }
    }
}
