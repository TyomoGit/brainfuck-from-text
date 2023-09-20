use brainfuck_from_text::*;

fn main() {
    let result = generate("Hello, World!");
    println!("{}", result);
}