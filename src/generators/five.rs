use crate::generators::validator::generate_validated_scramble;
use crate::moves::MoveWidth;
use crate::scramble::Scramble;
use rand::{Rng, rng};

pub fn generate(length: Option<usize>) -> Scramble {
    let mut rng = rng();
    let moves = generate_validated_scramble(&mut rng, length.unwrap_or(60), |rng| {
        match rng.random_range(0..10) {
            0..=5 => MoveWidth::Single, // 60% single moves (R, U, etc.)
            _ => MoveWidth::Wide,       // 40% wide moves (Rw, Uw, etc.)
        }
    });
    Scramble { moves }
}
