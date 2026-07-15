use crate::widget::Widget;

pub struct Center(pub String);

impl Widget for Center {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        let mut width = 0;
        let mut height = 0;

        for line in self.0.lines() {
            height += 1;
            width = width.max(line.len() as u16);
        }

        let x = (scope.rect.width - width) / 2;
        let y = (scope.rect.height - height) / 2;

        scope.draw_text((x, y), &self.0);
    }
}
