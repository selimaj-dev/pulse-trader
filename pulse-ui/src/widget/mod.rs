pub mod outline;

use crate::render::RenderScope;

pub trait Widget {
    fn render(&self, scope: &mut RenderScope);
}

impl Widget for &str {
    fn render(&self, scope: &mut RenderScope) {
        scope.draw_text(0, self);
    }
}
