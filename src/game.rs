use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::{canvas::Canvas, Block, BorderType, Widget},
};

use crate::apple::Apple;
use crate::point::Point;
use crate::snake::Snake;
use crate::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone)]
pub struct Game {
    frame_rate: f64,
    apple: Apple,
    snake: Snake,
    state: GameState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GameState {
    #[default]
    Running,
    Quit,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            frame_rate: 10.0,
            apple: Apple::default(),
            snake: Snake::default(),
            state: GameState::default(),
        }
    }
}

impl Game {
    fn new_apple(&mut self) -> Apple {
        let head = vec![self.snake.head().to_owned()];
        let tail = self.snake.tail().as_slices();
        let tail = [tail.0, tail.1].concat();

        // Create a vector with all points of the snake
        let mut snake = [head, tail].concat();

        // Get a random position index minus the snake length
        let snake_length = snake.len();
        let possible_positions = GRID_WIDTH as usize * GRID_HEIGHT as usize - snake_length;
        let mut i = fastrand::usize(1..possible_positions);

        // Find the random point
        let mut new_point = Point::new(0, 0);
        'outer: for x in 0..GRID_WIDTH {
            new_point.x = x;
            for y in 0..GRID_HEIGHT {
                new_point.y = y;

                // If the point is on the snake, skip it and remove the point from the snake
                if let Some(index) = snake.iter().position(|x| x == &new_point) {
                    snake.remove(index);
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

    pub fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> std::io::Result<()> {
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            if !self.snake.is_alive() {
                self.quit();
                break;
            }

            let mut now = Instant::now();
            let mut timeout = Duration::from_secs_f64(1.0 / self.frame_rate);

            while now.elapsed() < timeout {
                if event::poll(timeout)? {
                    self.handle_events()?;

                    let elapsed = now.elapsed();

                    if timeout >= elapsed {
                        timeout -= now.elapsed();
                    }

                    now = Instant::now();
                }
            }

            self.snake.step();

            if self.apple.position() == self.snake.head() {
                self.snake.grow();
                self.apple = self.new_apple();
            }
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == GameState::Running
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                KeyCode::Up => self.snake.up(),
                KeyCode::Down => self.snake.down(),
                KeyCode::Left => self.snake.left(),
                KeyCode::Right => self.snake.right(),
                _ => (),
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = GameState::Quit;
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [area, _] =
            Layout::horizontal([Constraint::Length(GRID_WIDTH * 2 + 2), Constraint::Min(0)])
                .areas(area);

        Block::bordered()
            .border_type(BorderType::Thick)
            .render(area, buf);

        self.render_game(area.inner(Margin::new(1, 1)), buf);
    }
}

impl Game {
    fn render_game(&self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .x_bounds([0.0, (GRID_WIDTH * 2 - 1) as f64])
            .y_bounds([0.0, (GRID_HEIGHT - 1) as f64])
            .marker(symbols::Marker::Block)
            .paint(|ctx| {
                ctx.draw(&self.apple);
                ctx.draw(&self.snake);

                if !self.snake.is_alive() {
                    let game_over_text = "Game Over";
                    ctx.print(
                        ((GRID_WIDTH * 2 - 1) as f64 - game_over_text.len() as f64) / 2.0,
                        (GRID_HEIGHT - 1) as f64 / 2.0,
                        "Game Over",
                    );

                    let score_text = format!("Score: {}", self.snake.len() - 3);
                    ctx.print(
                        ((GRID_WIDTH * 2 - 1) as f64 - score_text.len() as f64) / 2.0,
                        (GRID_HEIGHT - 1) as f64 / 2.0 - 1.0,
                        score_text,
                    );
                }
            })
            .render(area, buf);
    }
}
