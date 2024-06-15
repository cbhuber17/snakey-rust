use crate::draw::draw_block;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

/// Represents the possible directions the snake can move.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    /// Returns the opposite direction.
    ///
    /// # Returns
    ///
    /// A `Direction` enum representing the opposite direction.
    ///
    /// # Example
    ///
    /// ```
    /// let dir = Direction::Up;
    /// let opposite_dir = dir.opposite();
    /// assert_eq!(opposite_dir, Direction::Down);
    /// ```
    ///
    /// This function takes the current direction and returns its opposite.
    /// For example, if the current direction is `Up`, it returns `Down`.
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

/// Represents the snake in the game.
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    /// Creates a new snake instance starting at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the initial position of the snake's head.
    /// * `y` - The y-coordinate of the initial position of the snake's head.
    ///
    /// # Returns
    ///
    /// A new `Snake` instance with the body positioned horizontally starting at `(x, y)`
    /// and extending to the right.
    ///
    /// # Example
    ///
    /// ```rust
    /// let snake = Snake::new(2, 2);
    /// assert_eq!(snake.direction, Direction::Right);
    /// assert_eq!(snake.body.len(), 3);
    /// ```
    ///
    /// This function initializes the snake with a body of three blocks, starting from
    /// the given `(x, y)` coordinates and extending to the right.
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    /// Draws the snake on the screen.
    ///
    /// # Arguments
    ///
    /// * `con` - A reference to the `Context` for rendering.
    /// * `g` - A mutable reference to the `G2d` graphics backend.
    ///
    /// This function iterates over each block in the snake's body and draws it using the `draw_block` function.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Assuming you have a valid Context and G2d instance:
    /// // snake.draw(&con, &mut g);
    /// ```
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    /// Returns the position of the snake's head.
    ///
    /// # Returns
    ///
    /// A tuple `(i32, i32)` representing the x and y coordinates of the snake's head.
    ///
    /// This function retrieves the first block in the snake's body (the head) and returns its coordinates.
    ///
    /// # Example
    ///
    /// ```rust
    /// let snake = Snake::new(2, 2);
    /// let (x, y) = snake.head_position();
    /// assert_eq!(x, 2);
    /// assert_eq!(y, 2);
    /// ```
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    /// Moves the snake forward in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `dir` - An optional `Direction` indicating the direction in which the snake will move next.
    ///           If `None`, the snake continues moving in its current direction.
    ///
    /// This function updates the snake's direction if a new direction is provided,
    /// calculates the new position of the head based on the current direction,
    /// and moves the snake's body forward by adding a new block at the head's new position
    /// and removing the block at the tail. The removed block is stored in `tail` for growth purposes.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut snake = Snake::new(2, 2);
    /// snake.move_forward(Some(Direction::Up));
    /// ```
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    /// Returns the current direction of the snake's head.
    ///
    /// # Returns
    ///
    /// A `Direction` enum representing the current direction of the snake's head.
    ///
    /// This function simply returns the direction in which the snake is currently moving.
    ///
    /// # Example
    ///
    /// ```rust
    /// let snake = Snake::new(2, 2);
    /// let direction = snake.head_direction();
    /// assert_eq!(direction, Direction::Right);
    /// ```
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    /// Calculates the next position of the snake's head based on the given direction.
    ///
    /// # Arguments
    ///
    /// * `dir` - An optional `Direction` indicating the direction in which the snake will move next.
    ///           If `None`, the snake continues moving in its current direction.
    ///
    /// # Returns
    ///
    /// A tuple `(i32, i32)` representing the x and y coordinates of the snake's head after moving in the specified direction.
    ///
    /// This function determines the next position of the snake's head based on the current direction or the provided direction.
    ///
    /// # Example
    ///
    /// ```rust
    /// let snake = Snake::new(2, 2);
    /// let (next_x, next_y) = snake.next_head(Some(Direction::Up));
    /// assert_eq!((next_x, next_y), (2, 1));
    /// ```
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        if let Some(d) = dir {
            moving_dir = d
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    /// Restores the snake's tail, effectively growing the snake by one block.
    ///
    /// This function takes the block stored in `tail` (if it exists) and appends it to the end of the snake's body.
    /// This is used to simulate the snake growing after eating food.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut snake = Snake::new(2, 2);
    /// snake.move_forward(Some(Direction::Up));
    /// snake.restore_tail();
    /// assert_eq!(snake.body.len(), 4);
    /// ```
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    /// Checks if the given coordinates overlap with the snake's body, excluding the head.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate to check for overlap.
    /// * `y` - The y-coordinate to check for overlap.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the given coordinates overlap with any part of the snake's body, excluding the head.
    ///
    /// This function iterates over the snake's body blocks (excluding the head) to check if any block
    /// is located at the given coordinates.
    ///
    /// # Example
    ///
    /// ```rust
    /// let snake = Snake::new(2, 2);
    /// assert!(snake.overlap_tail(2, 2)); // true because initial position of snake has a block at (2, 2)
    /// assert!(!snake.overlap_tail(0, 0)); // false because no block is at (0, 0)
    /// ```
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}
