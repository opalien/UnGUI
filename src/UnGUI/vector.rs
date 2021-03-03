//! vectors.
use std::ops;

/// vector
///
/// vector doesn't have setters or getters because x and y have to be use directly by the direct way
#[derive(Copy, Clone)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32
}

impl Vector2D {
    /// create a new Vector2D from the given values.
    ///
    /// The x and y can be negative to set widgets behind the screen.
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;
    /// vectors can be added between them
    /// the function get an other Vector2D and return a new Vector2D
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

