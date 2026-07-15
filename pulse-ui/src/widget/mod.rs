pub mod align;
pub mod outline;
pub mod spaced;
pub mod dynamic;
pub mod input;

use crate::render::RenderScope;

pub trait Widget {
    fn render(&self, scope: &mut RenderScope);
}

macro_rules! display_widget {
    ($t:ty) => {
        impl Widget for $t {
            fn render(&self, scope: &mut RenderScope) {
                scope.draw_text(0, self);
            }
        }
    };
}

display_widget!(&str);
display_widget!(String);
display_widget!(i8);
display_widget!(i16);
display_widget!(i32);
display_widget!(i64);
display_widget!(i128);
display_widget!(u8);
display_widget!(u16);
display_widget!(u32);
display_widget!(u64);
display_widget!(u128);
display_widget!(f32);
display_widget!(f64);

impl Widget for &dyn Widget {
    fn render(&self, scope: &mut RenderScope) {
        (*self).render(scope);
    }
}
