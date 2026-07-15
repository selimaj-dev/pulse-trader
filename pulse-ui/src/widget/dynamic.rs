use crate::{unit::Rect, widget::Widget};

pub struct Dynamic(pub fn(Rect) -> Box<dyn Widget>);

impl Widget for Dynamic {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        (self.0)(scope.rect).render(scope);
    }
}
