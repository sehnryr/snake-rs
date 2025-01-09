#[cfg(feature = "tui")]
use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Points, Shape},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[cfg(feature = "tui")]
impl Point {
    pub fn draw(&self, painter: &mut Painter, color: Color) {
        Points {
            coords: &[
                ((self.x * 2) as f64, self.y as f64),
                ((self.x * 2 + 1) as f64, self.y as f64),
            ],
            color,
        }
        .draw(painter);
    }
}
