use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::{canvas::Canvas, Block, BorderType, Widget},
};

use crate::apple::Apple;
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
    fn step(&mut self) {
        self.snake.step();

        if self.apple.position() == self.snake.head() {
            self.snake.grow();

            self.apple = Apple::new(self.snake.body());
        }

        if !self.snake.is_alive() {
            self.quit();
        }
    }

    fn score(&self) -> usize {
        self.snake.len() - 3
    }

    pub fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> std::io::Result<()> {
        terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

        while self.is_running() {
            let now = Instant::now();
            let timeout = Duration::from_secs_f64(1.0 / self.frame_rate);

            while let elapsed = now.elapsed()
                && elapsed < timeout
            {
                if event::poll(timeout - elapsed)? {
                    self.handle_events()?;
                }
            }

            self.step();

            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
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

                    let score_text = format!("Score: {}", self.score());
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
