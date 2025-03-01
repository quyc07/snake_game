// First, create a new project with:
// cargo new snake_game
// cd snake_game

// In Cargo.toml, add dependencies:
// [dependencies]
// crossterm = "0.25"
// rand = "0.8"

use crossterm::{
    cursor, execute, queue,
    style::{self, Color, SetBackgroundColor},
    terminal::{self, ClearType},
    event::{self, Event, KeyCode},
};
use rand::Rng;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

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
            | (Direction::Right, Direction::Left) => return,
            _ => self.direction = direction,
        }
    }
}

fn generate_food(snake: &VecDeque<(u16, u16)>) -> (u16, u16) {
    let mut rng = rand::thread_rng();
    let mut food;

    loop {
        food = (rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT));
        if !snake.contains(&food) {
            break;
        }
    }

    food
}

fn render(game: &Game) -> crossterm::Result<()> {
    let mut stdout = stdout();

    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    // Draw border
    for y in 0..HEIGHT+2 {
        for x in 0..WIDTH+2 {
            if y == 0 || y == HEIGHT+1 || x == 0 || x == WIDTH+1 {
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
            cursor::MoveTo(x+1, y+1),
            SetBackgroundColor(Color::Green),
            style::Print(" ")
        )?;
    }

    // Draw food
    queue!(
        stdout,
        cursor::MoveTo(game.food.0+1, game.food.1+1),
        SetBackgroundColor(Color::Red),
        style::Print(" ")
    )?;

    // Draw score
    queue!(
        stdout,
        cursor::MoveTo(0, HEIGHT+3),
        SetBackgroundColor(Color::Black),
        style::Print(format!("Score: {}", game.score))
    )?;

    if game.game_over {
        queue!(
            stdout,
            cursor::MoveTo(WIDTH/2 - 4, HEIGHT/2),
            SetBackgroundColor(Color::Black),
            style::Print("GAME OVER")
        )?;
    }

    stdout.flush()?;
    Ok(())
}

fn main() -> crossterm::Result<()> {
    // Set up terminal
    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    let mut game = Game::new();
    let mut last_update = Instant::now();

    loop {
        // Handle input
        if event::poll(Duration::from_millis(100))? {
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

        // Update game state at a fixed interval
        let now = Instant::now();
        if now - last_update >= Duration::from_millis(150) {
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
    execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
