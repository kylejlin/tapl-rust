use tapl_rust::chapter_7::*;
use term::unnamed::{Context, Term as UnnamedTerm};

fn main() {
    let src = r#"(\a. a \_. b) \x.x"#;
    let ctx = &["b", "c"];
    let unnamed =
        UnnamedTerm::from_named(parse(src).expect("Cannot parse."), &Context::from_strs(ctx))
            .expect("Cannot remove names.");
    println!("Unnamed: {}", unnamed);
    let res1 = eval1(unnamed.clone()).expect("Cannot eval 1");
    println!("Eval1: {}", res1);
    let res2 = eval1(res1).expect("Cannot eval 2");
    println!("Eval2: {}", res2);
    let (normal_form, count) = eval_count(unnamed);
    println!("Eval ({}): {}", count, normal_form);
}
