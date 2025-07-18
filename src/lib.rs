use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use std::fmt;

#[derive(Debug)]
pub struct Scramble {
    pub moves: Vec<Move>,
}

impl Scramble {
    pub fn generate(length: usize) -> Scramble {
        let mut scramble_moves = Vec::new();
        let mut rng = thread_rng();

        for i in 0..length {
            let move_face: MoveFace = rng.gen();
            let move_type: MoveType = rng.gen();
            let move_ = Move {
                move_face,
                move_type,
            };

            scramble_moves.push(move_);
        }

        Scramble {
            moves: scramble_moves,
        }
    }
}

impl fmt::Display for Scramble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for i in 0..self.moves.len() {
            write!(f, "{} ", self.moves[i])?;
        })
    }
}
#[derive(Debug)]
pub struct Move {
    pub move_face: MoveFace,
    pub move_type: MoveType,
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.move_face, self.move_type)
    }
}
#[derive(Debug)]
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
impl Distribution<MoveFace> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveFace {
        match rng.gen_range(0..6) {
            0 => MoveFace::Left,
            1 => MoveFace::Right,
            2 => MoveFace::Up,
            3 => MoveFace::Down,
            4 => MoveFace::Front,
            _ => MoveFace::Back,
        }
    }
}

#[derive(Debug)]
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
impl Distribution<MoveType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveType {
        match rng.gen_range(0..3) {
            0 => MoveType::Double,
            1 => MoveType::Normal,
            _ => MoveType::Prime,
        }
    }
}