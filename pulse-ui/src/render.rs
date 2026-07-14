use std::fmt::Display;

use crate::unit::{Point, Rect};

pub enum Instr {
    DrawText(Point, String),
    DrawOutline(Rect),
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

    pub fn draw_outline<P: Into<Point>>(&mut self, rect: Rect) {
        self.draw_instructions.push(Instr::DrawOutline(rect));
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

impl From<Rect> for Point {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
