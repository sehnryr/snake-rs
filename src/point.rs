use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Points, Shape},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

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
