use crate::generators::validator::generate_validated_scramble;
use crate::moves::MoveWidth;
use crate::scramble::Scramble;
use rand::rng;

pub fn generate(length: Option<usize>) -> Scramble {
    let mut rng = rng();
    let moves = generate_validated_scramble(&mut rng, length.unwrap_or(20), |_| MoveWidth::Single);
    Scramble { moves }
}
