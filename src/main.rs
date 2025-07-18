use std::env;
use scramble_gen::{Scramble, Cube};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut cube_type = Cube::ThreeByThree;
    let mut length: Option<usize> = None;
    let mut amount = 1;
    
    for arg in &args[1..] {
        match arg.as_str() {
            "--3x3" => cube_type = Cube::ThreeByThree,
            "--4x4" => cube_type = Cube::FourByFour,
            "--2x2" => cube_type = Cube::TwoByTwo,
            "--5x5" => cube_type = Cube::FiveByFive,
            "--6x6" => cube_type = Cube::SixBySix,
            "--7x7" => cube_type = Cube::SevenBySeven,
            _ if arg.starts_with("--length=") => {
                if let Some(len_str) = arg.strip_prefix("--length=") {
                    length = len_str.parse().ok();
                }
            }
            _ if arg.starts_with("--amount=") => {
                if let Some(amt_str) = arg.strip_prefix("--amount=") {
                    amount = amt_str.parse().unwrap_or(1);
                }
            }
            _ => {}
        }
    }
    
    for i in 0..amount {
        let scramble = Scramble::generate(cube_type, length);
        println!("{}", scramble);
    }
}
