use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Apple(Point);

impl Apple {
    pub fn position(&self) -> &Point {
        &self.0
    }
}

impl From<Point> for Apple {
    fn from(value: Point) -> Self {
        Self(value)
    }
}

impl Shape for Apple {
    fn draw(&self, painter: &mut Painter) {
        self.0.draw(painter, Color::Green);
    }
}
