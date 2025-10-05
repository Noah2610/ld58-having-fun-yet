use bevy::prelude::*;
use std::fmt;

#[derive(Reflect, Clone, Copy)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Direction {
    /// Returns the absolute direction for x, converting left to right variants
    pub fn abs_x(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::TopLeft => Direction::TopRight,
            Direction::BottomLeft => Direction::BottomRight,
            _ => self,
        }
    }

    pub fn vec(&self) -> Vec2 {
        use Direction::*;
        match self {
            Top => Vec2::Y,
            Bottom => -Vec2::Y,
            Left => -Vec2::X,
            Right => Vec2::X,
            TopLeft => Vec2::new(-1.0, 1.0).normalize(),
            TopRight => Vec2::new(1.0, 1.0).normalize(),
            BottomLeft => Vec2::new(-1.0, -1.0).normalize(),
            BottomRight => Vec2::new(1.0, -1.0).normalize(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::Top => "top",
            Direction::Bottom => "bottom",
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::TopLeft => "top-left",
            Direction::TopRight => "top-right",
            Direction::BottomLeft => "bottom-left",
            Direction::BottomRight => "bottom-right",
        })
    }
}
