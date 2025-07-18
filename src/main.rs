use scramble_gen::Scramble;

fn main() {
    for i in 0..20 {
        let scramble = Scramble::generate(20);

        println!("{}", scramble)
    }
}
