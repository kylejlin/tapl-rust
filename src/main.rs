use std::convert::TryFrom;
use tapl_rust::chapter_7::*;
use term::unnamed::Term as UnnamedTerm;

fn main() {
    let src = r#"(\a. a \b. a) \b.b"#;
    let unnamed =
        UnnamedTerm::try_from(parse(src).expect("Cannot parse.")).expect("Cannot remove names.");
    println!("Unnamed: {}", unnamed.clone().into_unpositioned_named());
    let res1 = eval1(unnamed.clone()).expect("Cannot eval 1");
    println!("Eval1: {}", res1.clone().into_unpositioned_named());
    let res2 = eval1(res1).expect("Cannot eval 2");
    println!("Eval2: {}", res2.clone().into_unpositioned_named());
    let (normal_form, count) = eval_count(unnamed);
    println!("Eval ({}): {}", count, normal_form);
}
