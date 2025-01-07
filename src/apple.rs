use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Points, Shape};

use crate::point::Point;
use crate::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Apple(Point);

impl Default for Apple {
    fn default() -> Self {
        Self::new(
            (GRID_WIDTH as f64 * 3.0 / 4.0) as u16,
            (GRID_HEIGHT as f64 / 2.0) as u16,
        )
    }
}

impl Apple {
    pub fn new(x: u16, y: u16) -> Self {
        Self(Point::new(x, y))
    }

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
        Points {
            coords: &[
                ((self.0.x * 2) as f64, self.0.y as f64),
                ((self.0.x * 2 + 1) as f64, self.0.y as f64),
            ],
            color: Color::Green,
        }
        .draw(painter);
    }
}
