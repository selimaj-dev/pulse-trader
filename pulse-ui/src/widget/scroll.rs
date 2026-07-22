use crate::widget::Widget;

pub struct ScrollState<const N: usize>(pub usize, pub [usize; N]);

pub struct ScrollText {
    pub scroll: usize,
    pub title: String,
    pub text: String,
}

impl Widget for ScrollText {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        scope.draw_text(0, &self.title);

        let title_lines = self.title.lines().count();

        for (y, line) in self
            .text
            .lines()
            .skip(self.scroll)
            .take(scope.rect.height as usize - title_lines)
            .enumerate()
        {
            scope.draw_text((0, (y + title_lines) as u16), line);
        }
    }
}

impl<const N: usize> ScrollState<N> {
    pub fn get_selected(&self, index: usize) -> &'static str {
        if index == self.0 { "\x1b[32m" } else { "" }
    }

    pub fn scroll(&self, index: usize, title: String, text: String) -> ScrollText {
        ScrollText {
            scroll: self.1[index],
            title,
            text,
        }
    }

    pub fn up(&mut self) {
        if self.1[self.0] > 0 {
            self.1[self.0] -= 1;
        }
    }

    pub fn down(&mut self) {
        self.1[self.0] += 1;
    }

    pub fn back_tab(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        } else {
            self.0 = N - 1;
        }
    }

    pub fn tab(&mut self) {
        if self.0 + 1 < N {
            self.0 += 1;
        } else {
            self.0 = 0;
        }
    }
}
