use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug)]
pub struct Scramble {
    pub moves: Vec<Move>,
}

impl Scramble {
    pub fn generate(cube: Cube, length: Option<usize>) -> Scramble {
        match cube {
            Cube::ThreeByThree => generate_3x3(length),
            Cube::FourByFour => generate_4x4(length),
            Cube::FiveByFive => generate_5x5(length),
            Cube::SixBySix => generate_6x6(length),
            Cube::SevenBySeven => generate_7x7(length),
            _ => unimplemented!(),
        }
    }
}

fn generate_3x3(length: Option<usize>) -> Scramble {
    let mut scramble_moves = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..length.unwrap_or(20) {
        let move_face: MoveFace = rng.gen();
        let move_type: MoveType = rng.gen();
        let move_ = Move {
            move_face,
            move_type,
            move_width: MoveWidth::Single,
        };

        scramble_moves.push(move_);
    }
    Scramble {
        moves: scramble_moves,
    }
}

fn generate_4x4(length: Option<usize>) -> Scramble {
    let mut scramble_moves = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..length.unwrap_or(40) {
        let move_face: MoveFace = rng.gen();
        let move_type: MoveType = rng.gen();

        let move_width = match rng.gen_range(0..10) {
            0..=7 => MoveWidth::Single, // 80% single moves
            _ => MoveWidth::Wide,       // 20% wide moves
        };

        let move_ = Move {
            move_face,
            move_type,
            move_width,
        };

        scramble_moves.push(move_);
    }
    Scramble {
        moves: scramble_moves,
    }
}

fn generate_5x5(length: Option<usize>) -> Scramble {
    let mut scramble_moves = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..length.unwrap_or(60) {
        let move_face: MoveFace = rng.gen();
        let move_type: MoveType = rng.gen();

        // 5x5 has different move width distribution
        let move_width = match rng.gen_range(0..10) {
            0..=5 => MoveWidth::Single, // 60% single moves (R, U, etc.)
            _ => MoveWidth::Wide,       // 40% wide moves (Rw, Uw, etc.)
        };

        let move_ = Move {
            move_face,
            move_type,
            move_width,
        };

        scramble_moves.push(move_);
    }
    Scramble {
        moves: scramble_moves,
    }
}

fn generate_6x6(length: Option<usize>) -> Scramble {
    let mut scramble_moves = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..length.unwrap_or(80) {
        let move_face: MoveFace = rng.gen();
        let move_type: MoveType = rng.gen();

        // 6x6 uses single, wide, and 3-wide moves
        let move_width = match rng.gen_range(0..10) {
            0..=4 => MoveWidth::Single, // 50% single moves
            5..=7 => MoveWidth::Wide,   // 30% wide moves
            _ => MoveWidth::ThreeWide,  // 20% 3-wide moves
        };

        let move_ = Move {
            move_face,
            move_type,
            move_width,
        };

        scramble_moves.push(move_);
    }
    Scramble {
        moves: scramble_moves,
    }
}

fn generate_7x7(length: Option<usize>) -> Scramble {
    let mut scramble_moves = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..length.unwrap_or(100) {
        let move_face: MoveFace = rng.gen();
        let move_type: MoveType = rng.gen();

        // 7x7 uses all move widths
        let move_width = match rng.gen_range(0..10) {
            0..=4 => MoveWidth::Single, // 50% single moves
            5..=7 => MoveWidth::Wide,   // 30% wide moves
            _ => MoveWidth::ThreeWide,  // 20% 3-wide moves
        };

        let move_ = Move {
            move_face,
            move_type,
            move_width,
        };

        scramble_moves.push(move_);
    }
    Scramble {
        moves: scramble_moves,
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

#[derive(Debug)]
pub enum MoveWidth {
    Single,
    Wide,
    ThreeWide,
}

impl Distribution<MoveWidth> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveWidth {
        match rng.gen_range(0..10) {
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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Cube {
    TwoByTwo,
    ThreeByThree,
    FourByFour,
    FiveByFive,
    SixBySix,
    SevenBySeven,
}
