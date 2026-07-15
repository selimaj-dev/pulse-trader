use crate::widget::Widget;

pub struct Outline;

impl Widget for Outline {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        scope.draw_text(0, "─".repeat(scope.rect.width as usize));

        scope.draw_text((0, 1), "│\n".repeat(scope.rect.height as usize - 2));
        scope.draw_text(
            (scope.rect.width - 1, 1),
            "│\n".repeat(scope.rect.height as usize - 2),
        );

        scope.draw_text(
            (0, scope.rect.height - 1),
            "─".repeat(scope.rect.width as usize),
        );

        scope.draw_text(0, "┌");
        scope.draw_text((scope.rect.width - 1, 0), "┐");
        scope.draw_text((0, scope.rect.height - 1), "└");
        scope.draw_text((scope.rect.width - 1, scope.rect.height - 1), "┘");
    }
}

pub struct TVLine;

impl Widget for TVLine {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        scope.draw_text(0, "─".repeat(scope.rect.width as usize));

        scope.draw_text(0, "┌");
        scope.draw_text((scope.rect.width - 1, 0), "┐");
    }
}

pub struct BVLine;

impl Widget for BVLine {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        scope.draw_text(0, "─".repeat(scope.rect.width as usize));

        scope.draw_text(0, "└");
        scope.draw_text((scope.rect.width - 1, 0), "┘");
    }
}

pub struct VLine;

impl Widget for VLine {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        scope.draw_text(0, "─".repeat(scope.rect.width as usize));
    }
}
