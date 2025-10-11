use avian2d::math::Scalar;
use bevy::prelude::*;
use std::fmt;

#[derive(Reflect, Clone, Copy, Debug)]
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

    pub fn opposite_y(self) -> Self {
        use Direction::*;
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Left,
            Right => Right,
            TopLeft => BottomLeft,
            TopRight => BottomRight,
            BottomLeft => TopLeft,
            BottomRight => TopRight,
        }
    }

    pub fn opposite(self) -> Self {
        use Direction::*;
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
            TopLeft => BottomRight,
            TopRight => BottomLeft,
            BottomLeft => TopRight,
            BottomRight => TopLeft,
        }
    }

    pub fn vec(self) -> Vec2 {
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

impl TryFrom<Vec2> for Direction {
    type Error = ();
    fn try_from(vec: Vec2) -> Result<Self, Self::Error> {
        use Direction::*;

        const MIN_POS: Scalar = 0.1;
        const MAX_POS: Scalar = 1.0;
        const MIN_NEG: Scalar = -MIN_POS;
        const MAX_NEG: Scalar = -MAX_POS;

        let vec = vec.try_normalize().ok_or(())?;

        match (vec.x, vec.y) {
            (MAX_NEG ..= MIN_NEG, MIN_POS ..= MAX_POS) => Ok(TopLeft),
            (MIN_POS ..= MAX_POS, MIN_POS ..= MAX_POS) => Ok(TopRight),
            (MAX_NEG ..= MIN_NEG, MAX_NEG ..= MIN_NEG) => Ok(BottomLeft),
            (MIN_POS ..= MAX_POS, MAX_NEG ..= MIN_NEG) => Ok(BottomRight),
            (MIN_NEG .. MIN_POS, MIN_POS ..= MAX_POS) => Ok(Top),
            (MIN_NEG .. MIN_POS, MAX_NEG ..= MIN_NEG) => Ok(Bottom),
            (MAX_NEG ..= MIN_NEG, MIN_NEG .. MIN_POS) => Ok(Left),
            (MIN_POS ..= MAX_POS, MIN_NEG .. MIN_POS) => Ok(Right),
            (_, _) => Err(()),
        }
    }
}
