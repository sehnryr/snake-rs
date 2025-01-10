use std::collections::VecDeque;

#[cfg(feature = "tui")]
use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};
use rl::burn::tensor::TensorData;

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
    is_growing: bool,
    is_dead: bool,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    #[default]
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub const VARIANTS: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}

impl Into<TensorData> for Direction {
    fn into(self) -> TensorData {
        TensorData::new(vec![self as u8], vec![1])
    }
}

impl From<isize> for Direction {
    fn from(value: isize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl Snake {
    pub fn new(head: Point, tail_length: usize, direction: Direction) -> Self {
        let mut body = VecDeque::with_capacity(tail_length + 1);

        body.push_front(head);

        for i in 1..tail_length as isize + 1 {
            #[rustfmt::skip]
            let point = match direction {
                Direction::Up =>    Point::new(head.x    , head.y - i),
                Direction::Right => Point::new(head.x - i, head.y    ),
                Direction::Down =>  Point::new(head.x    , head.y + i),
                Direction::Left =>  Point::new(head.x + i, head.y    ),
            };
            body.push_back(point);
        }

        Self {
            body,
            direction,
            is_growing: false,
            is_dead: false,
        }
    }

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

    pub fn is_growing(&mut self) -> bool {
        self.is_growing
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

#[cfg(feature = "tui")]
impl Shape for Snake {
    fn draw(&self, painter: &mut Painter) {
        self.body
            .iter()
            .skip(1)
            .for_each(|p| p.draw(painter, Color::DarkGray));
        self.head().draw(painter, Color::White);
    }
}
