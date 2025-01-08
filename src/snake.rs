use std::collections::VecDeque;

use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

use crate::point::Point;
use crate::GRID_HEIGHT;

#[derive(Debug, Clone)]
pub struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
    is_growing: bool,
    is_dead: bool,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 1,
    #[default]
    Right = 2,
    Down = 3,
    Left = 4,
}

impl Default for Snake {
    fn default() -> Self {
        let default_y = (GRID_HEIGHT as f64 / 2.0) as isize;

        Self {
            body: VecDeque::from([
                Point::new(3, default_y),
                Point::new(2, default_y),
                Point::new(1, default_y),
            ]),
            direction: Direction::default(),
            is_growing: false,
            is_dead: false,
        }
    }
}

impl Snake {
    pub fn head(&self) -> &Point {
        self.body.front().unwrap()
    }

    pub fn body(&self) -> impl IntoIterator<Item = &Point> {
        &self.body
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn grow(&mut self) {
        self.is_growing = true;
    }

    pub fn step(&mut self) {
        #[rustfmt::skip]
        let new_head = match self.direction {
            Direction::Up =>    Point::new(self.head().x    , self.head().y + 1),
            Direction::Right => Point::new(self.head().x + 1, self.head().y    ),
            Direction::Down =>  Point::new(self.head().x    , self.head().y - 1),
            Direction::Left =>  Point::new(self.head().x - 1, self.head().y    ),
        };

        if self.body.iter().rev().skip(1).any(|p| p == &new_head) {
            self.is_dead = true;
            return;
        }

        self.body.push_front(new_head);

        if self.is_growing {
            self.is_growing = false;
        } else {
            self.body.pop_back();
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        let d1 = self.direction as i32;
        let d2 = direction as i32;

        if (d1 - d2).abs() != 2 {
            self.direction = direction
        }
    }
}

impl Shape for Snake {
    fn draw(&self, painter: &mut Painter) {
        self.body
            .iter()
            .skip(1)
            .for_each(|p| p.draw(painter, Color::DarkGray));
        self.head().draw(painter, Color::White);
    }
}
