use crate::{unit::Rect, widget::Widget};

pub struct Dynamic<W: Widget>(pub fn(Rect) -> W);

impl<W: Widget> Widget for Dynamic<W> {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        (self.0)(scope.rect).render(scope);
    }
}
