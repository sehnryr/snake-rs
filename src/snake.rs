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
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new_point(self, point: Point) -> Point {
        match self {
            Direction::Up => Point::new(point.x, point.y + 1),
            Direction::Down => Point::new(point.x, point.y - 1),
            Direction::Left => Point::new(point.x - 1, point.y),
            Direction::Right => Point::new(point.x + 1, point.y),
        }
    }
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

    pub fn tail(&self) -> &VecDeque<Point> {
        &self.tail
    }

    pub fn len(&self) -> usize {
        self.tail().len() + 1
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

        let new_head = self.direction.new_point(self.head);

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

    pub fn up(&mut self) {
        if self.tail.front().unwrap() == &Direction::Up.new_point(self.head) {
            return;
        }
        self.direction = Direction::Up;
    }

    pub fn down(&mut self) {
        if self.tail.front().unwrap() == &Direction::Down.new_point(self.head) {
            return;
        }
        self.direction = Direction::Down;
    }

    pub fn left(&mut self) {
        if self.tail.front().unwrap() == &Direction::Left.new_point(self.head) {
            return;
        }
        self.direction = Direction::Left;
    }

    pub fn right(&mut self) {
        if self.tail.front().unwrap() == &Direction::Right.new_point(self.head) {
            return;
        }
        self.direction = Direction::Right;
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
