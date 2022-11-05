mod cube;
use cube::*;

mod parser;

fn main() {
    println!("{:?}", parser::parse_333_algorithm(&"M' U ".repeat(4)));
}
