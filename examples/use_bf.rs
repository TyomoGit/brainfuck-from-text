use std::io::{stdin, stdout};
use std::env;

use brainfuck_from_text::generate;
use brainfuck_interpreter::run;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        panic!("missing argument");
    }

    let result = generate(&args[1]);
    run(&result, &mut stdin(), &mut stdout());
}