use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Points, Shape};

use crate::point::Point;
use crate::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Apple(Point);

impl Default for Apple {
    fn default() -> Self {
        Self(Point::new(
            (GRID_WIDTH as f64 * 3.0 / 4.0) as u16,
            (GRID_HEIGHT as f64 / 2.0) as u16,
        ))
    }
}

impl Apple {
    pub fn new(mut obstructions: Vec<&Point>) -> Self {
        // Get a random position index minus obstructions count
        let possible_positions = GRID_WIDTH as usize * GRID_HEIGHT as usize - obstructions.len();
        let mut i = fastrand::usize(1..possible_positions);

        // Find the random point
        let mut new_point = Point::new(0, 0);
        'outer: for x in 0..GRID_WIDTH {
            new_point.x = x;
            for y in 0..GRID_HEIGHT {
                new_point.y = y;

                // If the point is on the snake, skip it and remove the point from the snake
                if let Some(index) = obstructions.iter().position(|x| *x == &new_point) {
                    obstructions.remove(index);
                } else {
                    i -= 1;
                }

                if i == 0 {
                    break 'outer;
                }
            }
        }

        new_point.into()
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
