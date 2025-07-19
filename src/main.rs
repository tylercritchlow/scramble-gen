use scramble_gen::{Cube, Scramble};
use std::env;

fn help() -> &'static str {
    "scramble-gen - Generate practice scrambles for Rubik's cubes

USAGE:
    scramble-gen [OPTIONS]

OPTIONS:
    --2x2                Generate 2x2 scrambles
    --3x3                Generate 3x3 scrambles (default)
    --4x4                Generate 4x4 scrambles
    --5x5                Generate 5x5 scrambles
    --6x6                Generate 6x6 scrambles
    --7x7                Generate 7x7 scrambles
    --length=<num>       Set scramble length (default varies by cube)
    --amount=<num>       Generate multiple scrambles (default: 1)
    --help, -h           Show this help message

EXAMPLES:
    scramble-gen --3x3
    scramble-gen --5x5 --amount=5
    scramble-gen --7x7 --length=120"
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("{}", help());
        return;
    }

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

    for _ in 0..amount {
        let scramble = Scramble::generate(cube_type, length);
        println!("{}", scramble);
    }
}
