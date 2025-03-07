// First, create a new project with:
// cargo new snake_game_cli
// cd snake_game_cli

// In Cargo.toml, add dependencies:
// [dependencies]
// crossterm = "0.25"
// rand = "0.8"

mod config;

use crate::config::DataConfig;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{self, Color, SetBackgroundColor},
    terminal::{self, ClearType},
};
use rand::Rng;
use std::cmp::max;
use std::collections::VecDeque;
use std::io;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

// Game constants
const WIDTH: u16 = 30;
const HEIGHT: u16 = 20;

// Snake direction
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Game state
struct Game {
    snake: VecDeque<(u16, u16)>,
    food: (u16, u16),
    direction: Direction,
    score: u32,
    game_over: bool,
    data_config: DataConfig,
}

impl Game {
    fn new() -> Self {
        // Initialize the snake in the middle of the screen
        let mut snake = VecDeque::new();
        snake.push_back((WIDTH / 2, HEIGHT / 2));

        // Random initial food position
        let food = generate_food(&snake);

        Game {
            snake,
            food,
            direction: Direction::Right,
            score: 0,
            game_over: false,
            data_config: DataConfig::new().unwrap(),
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Move the snake
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1.saturating_sub(1)),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0.saturating_sub(1), head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        // Check for collisions
        if new_head.0 >= WIDTH || new_head.1 >= HEIGHT || self.snake.contains(&new_head) {
            self.game_over = true;
            self.data_config.write_score(self.score);
            return;
        }

        // Add new head to snake
        self.snake.push_front(new_head);

        // Check if food was eaten
        if new_head == self.food {
            self.score += 1;
            self.food = generate_food(&self.snake);
        } else {
            // If no food was eaten, remove the tail
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        // Prevent moving in the opposite direction
        match (&self.direction, &direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => (),
            _ => self.direction = direction,
        }
    }
}

fn generate_food(snake: &VecDeque<(u16, u16)>) -> (u16, u16) {
    let mut rng = rand::rng();
    let mut food;

    loop {
        food = (rng.random_range(0..WIDTH), rng.random_range(0..HEIGHT));
        if !snake.contains(&food) {
            break;
        }
    }

    food
}

fn render(game: &Game) -> io::Result<()> {
    let mut stdout = stdout();

    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    // Draw border
    for y in 0..HEIGHT + 2 {
        for x in 0..WIDTH + 2 {
            if y == 0 || y == HEIGHT + 1 || x == 0 || x == WIDTH + 1 {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    SetBackgroundColor(Color::Grey),
                    style::Print(" ")
                )?;
            }
        }
    }

    // Draw snake
    for &(x, y) in &game.snake {
        queue!(
            stdout,
            cursor::MoveTo(x + 1, y + 1),
            SetBackgroundColor(Color::Green),
            style::Print(" ")
        )?;
    }

    // Draw food
    queue!(
        stdout,
        cursor::MoveTo(game.food.0 + 1, game.food.1 + 1),
        SetBackgroundColor(Color::Red),
        style::Print(" ")
    )?;

    // Draw score
    queue!(
        stdout,
        cursor::MoveTo(0, HEIGHT + 3),
        SetBackgroundColor(Color::Black),
        style::Print(format!("Score: {}", game.score))
    )?;

    // Draw score history
    queue!(
        stdout,
        cursor::MoveTo(0, HEIGHT + 4),
        SetBackgroundColor(Color::Black),
        style::Print("历史得分: ")
    )?;
    game.data_config
        .scores
        .iter()
        .enumerate()
        .for_each(|(i, &score)| {
            match queue!(
                stdout,
                cursor::MoveTo(0, HEIGHT + 5 + i as u16),
                SetBackgroundColor(Color::Black),
                style::Print(format!("第{}名: {}", i + 1, &score))
            ) {
                Ok(_) => (),
                Err(e) => panic!("fail to show score history, {}", e),
            };
        });

    if game.game_over {
        queue!(
            stdout,
            cursor::MoveTo(WIDTH / 2 - 4, HEIGHT / 2),
            SetBackgroundColor(Color::Black),
            style::Print("GAME OVER")
        )?;
    }

    stdout.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Set up terminal
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut game = Game::new();
    let mut last_update = Instant::now();
    loop {
        // Handle input
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    _ => {}
                }
            }
        }

        // 得分越多，速度越快，上限是50ms
        let speed_interval = Duration::from_millis(max(150 - game.score * 5, 50) as u64);

        // Update game state at a fixed interval
        let now = Instant::now();
        if now - last_update >= speed_interval {
            game.update();
            last_update = now;
        }

        // Render the game
        render(&game)?;

        // Exit if game over
        if game.game_over {
            std::thread::sleep(Duration::from_secs(2));
            break;
        }
    }

    // Clean up terminal
    execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
