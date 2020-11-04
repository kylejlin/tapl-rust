use parser::err::*;
use tapl_rust::chapter_7::*;

fn main() {
    let src = r#"\a. b c"#;

    match parser::parse(src) {
        Ok(t) => println!("{:?}", t),
        Err(ParseErr::Tokenization(e)) => println!("Tokenization error:\n{:?}", e),
        Err(ParseErr::UnexpectedTokenOrEof(e)) => println!("Unexpected:\n{:?}", e),
    }
}
