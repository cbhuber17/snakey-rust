use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

/// Converts game coordinates to screen coordinates.
///
/// # Arguments
///
/// * `game_coord` - An integer representing the coordinate in the game's grid.
///
/// # Returns
///
/// A floating-point number representing the coordinate in the screen's pixel grid.
///
/// # Example
///
/// ```
/// let screen_coord = to_coord(2);
/// assert_eq!(screen_coord, 50.0);
/// ```
///
/// This function takes a game coordinate (which is an integer value) and
/// multiplies it by the constant `BLOCK_SIZE` (25.0) to convert it into
/// screen coordinates.
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// Converts game coordinates to screen coordinates as a `u32`.
///
/// # Arguments
///
/// * `game_coord` - An integer representing the coordinate in the game's grid.
///
/// # Returns
///
/// An unsigned 32-bit integer representing the coordinate in the screen's pixel grid.
///
/// # Example
///
/// ```
/// let screen_coord = to_coord_u32(2);
/// assert_eq!(screen_coord, 50);
/// ```
///
/// This function takes a game coordinate (which is an integer value),
/// converts it to screen coordinates using the `to_coord` function,
/// and then casts the result to a `u32`.
pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

/// Draws a block on the screen at the specified game coordinates.
///
/// # Arguments
///
/// * `color` - A `Color` representing the color of the block.
/// * `x` - An integer representing the x-coordinate in the game's grid.
/// * `y` - An integer representing the y-coordinate in the game's grid.
/// * `con` - A reference to the `Context` for rendering.
/// * `g` - A mutable reference to the `G2d` graphics backend.
///
/// # Example
///
/// ```rust
/// use piston_window::types::Color;
/// use piston_window::{Context, G2d};
///
/// let color: Color = [1.0, 0.0, 0.0, 1.0]; // Red color
/// let x = 2;
/// let y = 3;
///
/// // Assuming you have a valid Context and G2d instance:
/// // draw_block(color, x, y, &con, &mut g);
/// ```
///
/// This function draws a block of the specified color at the given (x, y)
/// game coordinates. It converts the game coordinates to screen coordinates
/// using the `to_coord` function and then uses the `rectangle` function
/// from the `piston_window` crate to draw the block.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

/// Draws a rectangle on the screen at the specified game coordinates.
///
/// # Arguments
///
/// * `color` - A `Color` representing the color of the rectangle.
/// * `x` - An integer representing the x-coordinate of the top-left corner in the game's grid.
/// * `y` - An integer representing the y-coordinate of the top-left corner in the game's grid.
/// * `width` - An integer representing the width of the rectangle in blocks.
/// * `height` - An integer representing the height of the rectangle in blocks.
/// * `con` - A reference to the `Context` for rendering.
/// * `g` - A mutable reference to the `G2d` graphics backend.
///
/// # Example
///
/// ```rust
/// use piston_window::types::Color;
/// use piston_window::{Context, G2d};
///
/// let color: Color = [0.0, 1.0, 0.0, 1.0]; // Green color
/// let x = 1;
/// let y = 2;
/// let width = 3;
/// let height = 4;
///
/// // Assuming you have a valid Context and G2d instance:
/// // draw_rectangle(color, x, y, width, height, &con, &mut g);
/// ```
///
/// This function draws a rectangle of the specified color and dimensions
/// at the given (x, y) game coordinates. It converts the game coordinates
/// to screen coordinates using the `to_coord` function and then uses the
/// `rectangle` function from the `piston_window` crate to draw the rectangle.
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
