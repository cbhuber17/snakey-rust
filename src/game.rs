use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};
use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

/// Represents the game state for the Snake game.
///
/// The `Game` struct holds the state of the game including the snake,
/// the presence and position of the food, the dimensions of the game area,
/// the game over status, and the waiting time for game updates.
pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    /// Creates a new game instance with the specified width and height.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the game area.
    /// * `height` - The height of the game area.
    ///
    /// # Returns
    ///
    /// A new `Game` instance with initial settings.
    ///
    /// # Example
    ///
    /// ```
    /// let game = Game::new(20, 20);
    /// assert_eq!(game.width, 20);
    /// assert_eq!(game.height, 20);
    /// assert!(game.food_exists);
    /// ```
    ///
    /// This function initializes a new `Game` instance with a snake starting
    /// at position (2, 2), food at position (6, 4), and the game not being over.
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    /// Handles key press events to control the snake.
    ///
    /// # Arguments
    ///
    /// * `key` - A `Key` representing the key that was pressed.
    ///
    /// If the game is over, this function does nothing.
    /// Otherwise, it checks the key pressed and sets the direction of the snake accordingly.
    /// If the new direction is directly opposite to the current direction of the snake, it ignores the input.
    ///
    /// # Example
    ///
    /// ```rust
    /// use piston_window::Key;
    ///
    /// let mut game = Game::new(20, 20);
    /// game.key_pressed(Key::Up);
    /// assert_eq!(game.snake.head_direction(), Direction::Up);
    /// ```
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    /// Draws the game state on the screen.
    ///
    /// # Arguments
    ///
    /// * `con` - A reference to the `Context` for rendering.
    /// * `g` - A mutable reference to the `G2d` graphics backend.
    ///
    /// This function draws the snake, food (if it exists), borders, and a game over screen
    /// if the game is over.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Assuming you have a valid Context and G2d instance:
    /// // game.draw(&con, &mut g);
    /// ```
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    /// Updates the game state based on the elapsed time.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - A floating-point number representing the time elapsed since the last update.
    ///
    /// This function updates the waiting time and performs several actions based on the game state:
    /// - If the game is over and the waiting time exceeds `RESTART_TIME`, the game is restarted.
    /// - If food does not exist, new food is added to the game.
    /// - If the waiting time exceeds `MOVING_PERIOD`, the snake is updated.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut game = Game::new(20, 20);
    /// game.update(0.1);
    /// ```
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// Checks if the snake's head is at the position of the food.
    ///
    /// If the snake's head is at the same position as the food, this function:
    /// - Sets `food_exists` to `false`.
    /// - Calls `restore_tail` on the snake to make it grow.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut game = Game::new(20, 20);
    /// game.check_eating();
    /// ```
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    /// Checks if the snake is alive based on its next head position.
    ///
    /// # Arguments
    ///
    /// * `dir` - An optional `Direction` indicating the direction in which the snake will move next.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the snake is alive. The snake is considered alive if:
    /// - Its next head position does not overlap with its tail.
    /// - Its next head position is within the boundaries of the game area.
    ///
    /// # Example
    ///
    /// ```rust
    /// let game = Game::new(20, 20);
    /// let is_alive = game.check_if_snake_alive(Some(Direction::Up));
    /// assert!(is_alive);
    /// ```
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    /// Adds food to the game at a random position that does not overlap with the snake's tail.
    ///
    /// This function generates random coordinates within the game area and ensures that the food
    /// does not overlap with the snake's tail. Once a valid position is found, it sets the `food_x`
    /// and `food_y` coordinates and marks `food_exists` as `true`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut game = Game::new(20, 20);
    /// game.add_food();
    /// assert!(game.food_exists);
    /// ```
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    /// Updates the snake's position and checks for game over conditions.
    ///
    /// # Arguments
    ///
    /// * `dir` - An optional `Direction` indicating the direction in which the snake will move next.
    ///
    /// This function moves the snake forward in the specified direction if it's alive.
    /// It also checks if the snake has eaten food and updates the game over status
    /// if the snake is no longer alive. Finally, it resets the waiting time for the next update.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut game = Game::new(20, 20);
    /// game.update_snake(Some(Direction::Right));
    /// ```
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// Restarts the game by resetting all necessary state variables.
    ///
    /// This function resets the snake to its initial position, resets the waiting time,
    /// repositions the food, and marks the game as not over.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut game = Game::new(20, 20);
    /// game.restart();
    /// ```
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}
