use crate::generators;
use crate::{cube::Cube, moves::Move};
use std::fmt;

#[derive(Debug)]
pub struct Scramble {
    pub moves: Vec<Move>,
}

impl Scramble {
    pub fn generate(cube: Cube, length: Option<usize>) -> Scramble {
        match cube {
            Cube::ThreeByThree => generators::three::generate(length),
            Cube::FourByFour => generators::four::generate(length),
            Cube::FiveByFive => generators::five::generate(length),
            Cube::SixBySix => generators::six::generate(length),
            Cube::SevenBySeven => generators::seven::generate(length),
            _ => unimplemented!(),
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
