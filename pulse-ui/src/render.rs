use std::{fmt::Display, io::stdout};

use crate::{
    layout::Allocation,
    unit::{Point, Rect},
    widget::Widget,
};

pub enum Instr {
    DrawText(Point, String),
}

pub struct RenderScope {
    pub rect: Rect,
    draw_instructions: Vec<Instr>,
}

impl RenderScope {
    pub fn draw_text<P: Into<Point>, T: Display>(&mut self, at: P, text: T) {
        self.draw_instructions
            .push(Instr::DrawText(at.into(), text.to_string()));
    }
}

impl RenderScope {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            draw_instructions: Vec::new(),
        }
    }
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<u16> for Point {
    fn from(value: u16) -> Self {
        Self { x: value, y: value }
    }
}

impl From<Rect> for Point {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Allocation {
    pub fn draw<W: Widget>(&self, index: usize, widget: W) {
        let mut scope = RenderScope::new(self.widget_alloc[index]);
        widget.render(&mut scope);
        scope.draw();
    }

    pub fn draw_frame<W: Widget>(&self, index: usize, widget: W) {
        let mut scope = RenderScope::new(self.frame_alloc[index]);
        widget.render(&mut scope);
        scope.draw();
    }
}

impl RenderScope {
    pub fn draw(self) {
        for inst in self.draw_instructions {
            match inst {
                Instr::DrawText(point, text) => {
                    for (i, line) in text.lines().enumerate() {
                        print_at(
                            self.rect.x + point.x,
                            self.rect.y + point.y + i as u16,
                            line,
                        );
                    }
                }
            }
        }
    }
}

pub fn print_at(x: u16, y: u16, text: &str) {
    crossterm::execute!(stdout(), crossterm::cursor::MoveTo(x, y)).unwrap();
    print!("{text}");
}
