use crate::draw::draw_block;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
