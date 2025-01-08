use std::collections::VecDeque;

use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Points, Shape};

use crate::point::Point;
use crate::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone)]
pub struct Snake {
    head: Point,
    tail: VecDeque<Point>,
    direction: Direction,
    status: SnakeStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum SnakeStatus {
    #[default]
    None,
    Growing,
    Dead,
}

impl Default for Snake {
    fn default() -> Self {
        let default_y = (GRID_HEIGHT as f64 / 2.0) as u16;

        Self {
            head: Point::new(3, default_y),
            tail: VecDeque::from([Point::new(2, default_y), Point::new(1, default_y)]),
            direction: Direction::Right,
            status: SnakeStatus::default(),
        }
    }
}

impl Snake {
    pub fn head(&self) -> &Point {
        &self.head
    }

    pub fn tail(&self) -> Vec<&Point> {
        let (slice1, slice2) = self.tail.as_slices();
        [slice1.iter().collect::<Vec<_>>(), slice2.iter().collect()].concat()
    }

    pub fn body(&self) -> Vec<&Point> {
        [vec![self.head()], self.tail()].concat()
    }

    pub fn len(&self) -> usize {
        self.tail.len() + 1
    }

    pub fn is_alive(&self) -> bool {
        self.status != SnakeStatus::Dead
    }

    fn is_growing(&self) -> bool {
        self.status == SnakeStatus::Growing
    }

    pub fn grow(&mut self) {
        self.status = SnakeStatus::Growing;
    }

    pub fn step(&mut self) {
        // Check if head collided with a wall
        if (self.head.x == 0 && self.direction == Direction::Left)
            || (self.head.x == GRID_WIDTH - 1 && self.direction == Direction::Right)
            || (self.head.y == 0 && self.direction == Direction::Down)
            || (self.head.y == GRID_HEIGHT - 1 && self.direction == Direction::Up)
        {
            self.status = SnakeStatus::Dead;
            return;
        }

        #[rustfmt::skip]
        let new_head = match self.direction {
            Direction::Up =>    Point::new(self.head.x    , self.head.y + 1),
            Direction::Right => Point::new(self.head.x + 1, self.head.y    ),
            Direction::Down =>  Point::new(self.head.x    , self.head.y - 1),
            Direction::Left =>  Point::new(self.head.x - 1, self.head.y    ),
        };

        // Check if head collided with the tail
        if self.tail.contains(&new_head) {
            self.status = SnakeStatus::Dead;
            return;
        }

        self.tail.push_front(self.head);

        if !self.is_growing() {
            self.tail.pop_back();
        } else {
            self.status = SnakeStatus::None;
        }

        self.head = new_head;
    }

    fn turn(&mut self, direction: Direction) {
        let d1 = self.direction as i32;
        let d2 = direction as i32;

        if (d1 - d2).abs() != 2 {
            self.direction = direction
        }
    }

    pub fn up(&mut self) {
        self.turn(Direction::Up);
    }

    pub fn down(&mut self) {
        self.turn(Direction::Down);
    }

    pub fn left(&mut self) {
        self.turn(Direction::Left);
    }

    pub fn right(&mut self) {
        self.turn(Direction::Right);
    }
}

impl Shape for Snake {
    fn draw(&self, painter: &mut Painter) {
        self.draw_head(painter);
        self.draw_tail(painter);
    }
}

impl Snake {
    fn draw_head(&self, painter: &mut Painter) {
        Points {
            coords: &[
                ((self.head.x * 2) as f64, self.head.y as f64),
                ((self.head.x * 2 + 1) as f64, self.head.y as f64),
            ],
            color: Color::White,
        }
        .draw(painter);
    }

    fn draw_tail(&self, painter: &mut Painter) {
        let tail_points: Vec<(f64, f64)> = self
            .tail
            .iter()
            .flat_map(|point| {
                [
                    ((point.x * 2) as f64, point.y as f64),
                    ((point.x * 2 + 1) as f64, point.y as f64),
                ]
            })
            .collect();

        Points {
            coords: &tail_points,
            color: Color::DarkGray,
        }
        .draw(painter);
    }
}
