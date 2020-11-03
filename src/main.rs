use tapl_rust::chapter_7::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", parser::parse("if true then false else false"));
}
