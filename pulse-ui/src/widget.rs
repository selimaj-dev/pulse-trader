use crate::render::RenderScope;

pub trait Widget {
    fn render(&self, scope: &mut RenderScope);
}
