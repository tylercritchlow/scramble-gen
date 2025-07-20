use crate::generators::validator::generate_validated_scramble;
use crate::moves::MoveWidth;
use crate::scramble::Scramble;
use rand::{Rng, rng};

pub fn generate(length: Option<usize>) -> Scramble {
    let mut rng = rng();
    let moves = generate_validated_scramble(&mut rng, length.unwrap_or(40), |rng| {
        match rng.random_range(0..10) {
            0..=7 => MoveWidth::Single, // 80% single moves
            _ => MoveWidth::Wide,       // 20% wide moves
        }
    });
    Scramble { moves }
}
