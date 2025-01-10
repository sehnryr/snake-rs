#[cfg(all(feature = "tui", not(feature = "rl")))]
use std::time::{Duration, Instant};

#[cfg(all(feature = "tui", feature = "rl"))]
use ratatui::widgets::WidgetRef;

#[cfg(all(feature = "tui", not(feature = "rl")))]
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[cfg(feature = "tui")]
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, BorderType, Widget},
};
#[cfg(feature = "rl")]
use rl::env::{DiscreteActionSpace, Environment, Report};

use crate::apple::Apple;
use crate::point::Point;
use crate::snake::{Direction, Snake};

#[cfg(all(feature = "tui", feature = "rl"))]
use crate::TERMINAL;

#[derive(Debug)]
pub struct Game<const WIDTH: usize, const HEIGHT: usize> {
    #[cfg(all(feature = "tui", not(feature = "rl")))]
    frame_rate: f64,
    apple: Apple,
    snake: Snake,
    direction: Direction,
    state: GameState,
    #[cfg(feature = "rl")]
    pub report: Report,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GameState {
    #[default]
    Running,
    Quit,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Game<WIDTH, HEIGHT> {
    fn default() -> Self {
        let initial_direction = Direction::default();

        let apple = Point::new(
            (WIDTH as f64 * 3.0 / 4.0) as isize,
            (HEIGHT as f64 / 2.0) as isize,
        )
        .into();

        let snake = Snake::new(
            Point::new(3, (HEIGHT as f64 / 2.0) as isize),
            2,
            initial_direction,
        );

        Self {
            #[cfg(all(feature = "tui", not(feature = "rl")))]
            frame_rate: 10.0,
            apple,
            snake,
            direction: initial_direction,
            state: GameState::default(),
            #[cfg(feature = "rl")]
            report: Report::new(vec!["score", "reward", "steps"]),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Game<WIDTH, HEIGHT> {
    fn step(&mut self) {
        self.snake.turn(self.direction);

        if self.is_facing_bound(self.snake.head(), self.snake.direction()) {
            self.quit();
            return;
        }

        self.snake.step();

        if self.snake.is_dead() {
            self.quit();
            return;
        }

        if self.apple.position() == self.snake.head() {
            self.snake.grow();

            self.spawn_apple();
        }
    }

    #[cfg(feature = "rl")]
    fn state(&mut self) -> [f32; 4] {
        let apple = self.apple.position();
        let snake_head = self.snake.head();

        [
            apple.x as f32,
            apple.y as f32,
            snake_head.x as f32,
            snake_head.y as f32,
        ]
    }

    #[cfg(feature = "tui")]
    fn score(&self) -> usize {
        self.snake.len() - 3
    }

    #[rustfmt::skip]
    fn is_facing_bound(&self, point: &Point, direction: Direction) -> bool {
        match direction {
            Direction::Up    => point.y == HEIGHT as isize - 1,
            Direction::Right => point.x == WIDTH as isize - 1,
            Direction::Down  => point.y == 0,
            Direction::Left  => point.x == 0,
        }
    }

    fn spawn_apple(&mut self) {
        let mut obstructions: Vec<&Point> = self.snake.body().into_iter().collect();

        // Get a random position index minus obstructions count
        let possible_positions = WIDTH * HEIGHT - obstructions.len();
        let mut i = fastrand::usize(1..possible_positions);

        // Find the random point
        let mut new_point = Point::new(0, 0);
        'outer: for x in 0..WIDTH as isize {
            new_point.x = x;
            for y in 0..HEIGHT as isize {
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

        self.apple = new_point.into();
    }

    #[cfg(all(feature = "tui", feature = "rl"))]
    pub fn run(&mut self) -> std::io::Result<()> {
        let mut terminal = TERMINAL.lock().unwrap();

        terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

        // Reset terminal cursor at the end of viewport
        let area = terminal.get_frame().area();
        terminal.set_cursor_position((0, area.height + area.y + 1))?;

        Ok(())
    }

    #[cfg(all(feature = "tui", not(feature = "rl")))]
    pub fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> std::io::Result<()> {
        terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

        while self.is_running() {
            let now = Instant::now();
            let timeout = Duration::from_secs_f64(1.0 / self.frame_rate);
            let mut elapsed = now.elapsed();

            while elapsed < timeout {
                if event::poll(timeout - elapsed)? {
                    self.handle_events()?;
                }
                elapsed = now.elapsed();
            }

            self.step();

            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }

        // Reset terminal cursor at the end of viewport
        let area = terminal.get_frame().area();
        terminal.set_cursor_position((0, area.height + area.y + 1))?;

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == GameState::Running
    }

    #[cfg(all(feature = "tui", not(feature = "rl")))]
    #[rustfmt::skip]
    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                KeyCode::Up    => self.direction = Direction::Up,
                KeyCode::Right => self.direction = Direction::Right,
                KeyCode::Down  => self.direction = Direction::Down,
                KeyCode::Left  => self.direction = Direction::Left,
                _ => (),
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = GameState::Quit;
    }
}

#[cfg(all(feature = "tui", feature = "rl"))]
impl<const WIDTH: usize, const HEIGHT: usize> WidgetRef for &mut Game<WIDTH, HEIGHT> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let [area, _] = Layout::horizontal([
            Constraint::Length((WIDTH * 2 + 2) as u16),
            Constraint::Min(0),
        ])
        .areas(area);

        Block::bordered()
            .border_type(BorderType::Thick)
            .render(area, buf);

        self.render_game(area.inner(Margin::new(1, 1)), buf);
    }
}

#[cfg(feature = "tui")]
impl<const WIDTH: usize, const HEIGHT: usize> Widget for &Game<WIDTH, HEIGHT> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [area, _] = Layout::horizontal([
            Constraint::Length((WIDTH * 2 + 2) as u16),
            Constraint::Min(0),
        ])
        .areas(area);

        Block::bordered()
            .border_type(BorderType::Thick)
            .render(area, buf);

        self.render_game(area.inner(Margin::new(1, 1)), buf);
    }
}

#[cfg(feature = "tui")]
impl<const WIDTH: usize, const HEIGHT: usize> Game<WIDTH, HEIGHT> {
    fn render_game(&self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .x_bounds([0.0, (WIDTH * 2 - 1) as f64])
            .y_bounds([0.0, (HEIGHT - 1) as f64])
            .marker(symbols::Marker::Block)
            .paint(|ctx| {
                ctx.draw(&self.apple);
                ctx.draw(&self.snake);

                if !self.is_running() {
                    let game_over_text = "Game Over";
                    ctx.print(
                        ((WIDTH * 2 - 1) as f64 - game_over_text.len() as f64) / 2.0,
                        (HEIGHT - 1) as f64 / 2.0,
                        "Game Over",
                    );

                    let score_text = format!("Score: {}", self.score());
                    ctx.print(
                        ((WIDTH * 2 - 1) as f64 - score_text.len() as f64) / 2.0,
                        (HEIGHT - 1) as f64 / 2.0 - 1.0,
                        score_text,
                    );
                }
            })
            .render(area, buf);
    }
}

#[cfg(feature = "rl")]
impl<const WIDTH: usize, const HEIGHT: usize> DiscreteActionSpace for Game<WIDTH, HEIGHT> {
    fn actions(&self) -> Vec<Self::Action> {
        Direction::VARIANTS.to_vec()
    }
}

#[cfg(feature = "rl")]
impl<const WIDTH: usize, const HEIGHT: usize> Environment for Game<WIDTH, HEIGHT> {
    type State = [f32; 4];
    type Action = Direction;

    fn is_active(&self) -> bool {
        self.is_running()
    }

    fn reset(&mut self) -> Self::State {
        let default = Self::default();
        self.apple = default.apple;
        self.snake = default.snake;
        self.direction = default.direction;
        self.state = default.state;

        self.state()
    }

    fn random_action(&self) -> Self::Action {
        *fastrand::choice(self.actions().iter()).unwrap()
    }

    fn step(&mut self, action: Self::Action) -> (Option<Self::State>, f32) {
        self.report.entry("steps").and_modify(|x| *x += 1.0);
        let mut reward = -0.01;

        self.direction = action;
        self.step();

        if self.snake.is_growing() {
            self.report.entry("score").and_modify(|x| *x += 1.0);
            reward += 1.0;
        }

        let next_state = if self.is_active() {
            Some(self.state())
        } else {
            reward += if self.snake.len() == WIDTH * HEIGHT {
                1.0
            } else {
                -10.0
            };
            None
        };

        self.run().unwrap();

        self.report.entry("reward").and_modify(|x| *x += reward);
        (next_state, reward as f32)
    }
}
