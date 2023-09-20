use std::env;
use brainfuck_from_text::generate;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        panic!("missing argument");
    }

    let result = generate(&args[1]);
    println!("{}", result);

}