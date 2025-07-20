use crate::generators::validator::generate_validated_scramble;
use crate::moves::MoveWidth;
use crate::scramble::Scramble;
use rand::{Rng, rng};

pub fn generate(length: Option<usize>) -> Scramble {
    let mut rng = rng();
    let moves = generate_validated_scramble(&mut rng, length.unwrap_or(100), |rng| {
        match rng.random_range(0..10) {
            0..=4 => MoveWidth::Single, // 50% single moves
            5..=7 => MoveWidth::Wide,   // 30% wide moves
            _ => MoveWidth::ThreeWide,  // 20% 3-wide moves
        }
    });
    Scramble { moves }
}
