use tapl_rust::chapter_7::*;
use term::unnamed::{Context, Term as UnnamedTerm};

fn main() {
    let src = r#"(\a. b a) c"#;
    let ctx = &["b", "c"];
    let unnamed =
        UnnamedTerm::from_named(parse(src).expect("Cannot parse."), &Context::from_strs(ctx))
            .expect("Cannot remove names.");
    println!("Unnamed: {:?}", unnamed);
    let res = eval1(unnamed);
    println!("Eval1: {:?}", res);
}
