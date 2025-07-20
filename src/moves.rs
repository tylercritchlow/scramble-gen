use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Move {
    pub move_face: MoveFace,
    pub move_type: MoveType,
    pub move_width: MoveWidth,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.move_width {
            MoveWidth::Single => write!(f, "{}{}", self.move_face, self.move_type),
            MoveWidth::Wide => write!(f, "{}w{}", self.move_face, self.move_type),
            MoveWidth::ThreeWide => write!(f, "3{}{}", self.move_face, self.move_type),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveFace {
    Left,
    Right,
    Up,
    Down,
    Front,
    Back,
}

impl fmt::Display for MoveFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveFace::Left => write!(f, "L"),
            MoveFace::Right => write!(f, "R"),
            MoveFace::Up => write!(f, "U"),
            MoveFace::Down => write!(f, "D"),
            MoveFace::Front => write!(f, "F"),
            MoveFace::Back => write!(f, "B"),
        }
    }
}

impl MoveFace {
    pub fn same_axis(&self, other: &MoveFace) -> bool {
        matches!(
            (self, other),
            (MoveFace::Right, MoveFace::Left)
                | (MoveFace::Left, MoveFace::Right)
                | (MoveFace::Up, MoveFace::Down)
                | (MoveFace::Down, MoveFace::Up)
                | (MoveFace::Front, MoveFace::Back)
                | (MoveFace::Back, MoveFace::Front)
        )
    }
}

impl Distribution<MoveFace> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveFace {
        match rng.random_range(0..6) {
            0 => MoveFace::Left,
            1 => MoveFace::Right,
            2 => MoveFace::Up,
            3 => MoveFace::Down,
            4 => MoveFace::Front,
            _ => MoveFace::Back,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveType {
    Double,
    Prime,
    Normal,
}

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveType::Double => write!(f, "2"),
            MoveType::Prime => write!(f, "'"),
            MoveType::Normal => write!(f, ""),
        }
    }
}

impl Distribution<MoveType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveType {
        match rng.random_range(0..3) {
            0 => MoveType::Double,
            1 => MoveType::Normal,
            _ => MoveType::Prime,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveWidth {
    Single,
    Wide,
    ThreeWide,
}

impl Distribution<MoveWidth> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveWidth {
        match rng.random_range(0..10) {
            0..=7 => MoveWidth::Single,
            _ => MoveWidth::Wide,
        }
    }
}

impl fmt::Display for MoveWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveWidth::Single => write!(f, ""),
            MoveWidth::Wide => write!(f, "w"),
            MoveWidth::ThreeWide => write!(f, ""),
        }
    }
}
