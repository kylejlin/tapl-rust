use super::super::{
    eval, parse, term_builder as tb,
    Term::{self, False, True, Zero},
};

#[test]
fn true_() {
    let actual = evaluate_parsed("true");
    let expected = Some(True);
    assert_eq!(actual, expected);
}

#[test]
fn false_() {
    let actual = evaluate_parsed("false");
    let expected = Some(False);
    assert_eq!(actual, expected);
}

#[test]
fn zero() {
    let actual = evaluate_parsed("0");
    let expected = Some(Zero);
    assert_eq!(actual, expected);
}

#[test]
fn simple_e_if_true() {
    let actual = evaluate_parsed("if true then succ 0 else pred 0");
    let expected = Some(tb::succ(Zero));
    assert_eq!(actual, expected);
}

#[test]
fn simple_e_if_false() {
    let actual = evaluate_parsed("if false then succ 0 else pred 0");
    let expected = Some(Zero);
    assert_eq!(actual, expected);
}

#[test]
fn simple_e_if() {
    let actual = evaluate_parsed("if (if true then false else true) then succ 0 else pred 0");
    let expected = Some(Zero);
    assert_eq!(actual, expected);
}

#[test]
fn succ_zero() {
    let actual = evaluate_parsed("succ 0");
    let expected = Some(tb::succ(Zero));
    assert_eq!(actual, expected);
}

#[test]
fn succ_succ_zero() {
    let actual = evaluate_parsed("succ succ 0");
    let expected = Some(tb::succ(tb::succ(Zero)));
    assert_eq!(actual, expected);
}

#[test]
fn pred_zero_equals_zero() {
    let actual = evaluate_parsed("pred 0");
    let expected = Some(Zero);
    assert_eq!(actual, expected);
}

#[test]
fn pred_succ() {
    let actual = evaluate_parsed("pred succ 0");
    let expected = Some(Zero);
    assert_eq!(actual, expected);
}

#[test]
fn pred_succ_succ() {
    let actual = evaluate_parsed("pred succ succ 0");
    let expected = Some(tb::succ(Zero));
    assert_eq!(actual, expected);
}

#[test]
fn pred_pred() {
    let actual = evaluate_parsed("pred pred 0");
    let expected = Some(Zero);
    assert_eq!(actual, expected);
}

#[test]
fn complex_pred_succ() {
    let actual = evaluate_parsed("pred succ succ pred pred succ 0");
    let expected = Some(tb::succ(Zero));
    assert_eq!(actual, expected);
}

#[test]
fn is_zero_zero() {
    let actual = evaluate_parsed("iszero 0");
    let expected = Some(True);
    assert_eq!(actual, expected);
}

#[test]
fn is_zero_succ() {
    let actual = evaluate_parsed("iszero succ 0");
    let expected = Some(False);
    assert_eq!(actual, expected);
}

#[test]
fn is_zero_pred() {
    let actual = evaluate_parsed("iszero pred 0");
    let expected = Some(True);
    assert_eq!(actual, expected);
}

#[test]
fn is_zero_pred_succ() {
    let actual = evaluate_parsed("iszero pred succ 0");
    let expected = Some(True);
    assert_eq!(actual, expected);
}

#[test]
fn if_zero() {
    let actual = evaluate_parsed("if 0 then true else false");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn if_succ() {
    let actual = evaluate_parsed("if succ 0 then true else false");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn if_iszero() {
    let actual = evaluate_parsed("if iszero 0 then true else false");
    let expected = Some(True);
    assert_eq!(actual, expected);
}

#[test]
fn succ_true() {
    let actual = evaluate_parsed("succ true");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn succ_false() {
    let actual = evaluate_parsed("succ false");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn pred_true() {
    let actual = evaluate_parsed("pred true");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn pred_false() {
    let actual = evaluate_parsed("pred false");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn is_zero_true() {
    let actual = evaluate_parsed("iszero true");
    let expected = None;
    assert_eq!(actual, expected);
}

#[test]
fn is_zero_false() {
    let actual = evaluate_parsed("iszero false");
    let expected = None;
    assert_eq!(actual, expected);
}

fn evaluate_parsed(src: &str) -> Option<Term> {
    parse(src).and_then(eval)
}
