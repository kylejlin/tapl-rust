use super::{parse, term_builder as tb, Term};
use Term::{False, True, Zero};

#[test]
fn simple_if() {
    let actual = parse("if true then false else true");
    let expected = Some(tb::if_(True, False, True));
    assert_eq!(actual, expected);
}

#[test]
fn simple_if_extraneous_whitespace() {
    let actual = parse("   if  true        then false \n else true ");
    let expected = Some(tb::if_(True, False, True));
    assert_eq!(actual, expected);
}

#[test]
fn simple_if_extraneous_leading_tokens() {
    let actual = parse("true if true then false else true");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn simple_if_extraneous_trailing_tokens() {
    let actual = parse("if true then false else true true");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn illegal_token() {
    let actual = parse("if tru then false else true");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn simple_if_parens() {
    let actual = parse("if (true) then false else true");
    let expected = Some(tb::if_(True, False, True));
    assert_eq!(actual, expected);
}

#[test]
fn if_complex_guard() {
    let actual = parse("if iszero succ 0 then false else true");
    let expected = Some(tb::if_(tb::is_zero(tb::succ(Zero)), False, True));
    assert_eq!(actual, expected);
}

#[test]
fn if_complex_guard_then_else() {
    let actual = parse(
        "if iszero succ 0 then if true then true else true else if false then succ 0 else false",
    );
    let expected = Some(tb::if_(
        tb::is_zero(tb::succ(Zero)),
        tb::if_(True, True, True),
        tb::if_(False, tb::succ(Zero), False),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn if_complex_guard_then_else_parens() {
    let actual = parse(
        "if (iszero succ 0) then (if true then true else true) else (if false then succ 0 else false)",
    );
    let expected = Some(tb::if_(
        tb::is_zero(tb::succ(Zero)),
        tb::if_(True, True, True),
        tb::if_(False, tb::succ(Zero), False),
    ));
    assert_eq!(actual, expected);
}
