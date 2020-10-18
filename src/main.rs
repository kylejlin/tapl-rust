use tapl_rust::chapter_4::*;

fn main() {
    println!("Hello, world!");
    println!(
        "{:?}",
        eval(term_builder::is_zero(term_builder::pred(Term::Zero)))
    );
}
