mod util;

use super::parse;
use unp::IntoUnpositioned;
use util::unpositioned as unp;

#[test]
fn identity() {
    let actual = parse(r"\x.x").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::abs("x", unp::var("x")));
    assert_eq!(actual, expected);
}

#[test]
fn open() {
    let actual = parse(r"\x.x y").into_unpositioned();
    let expected: Result<_, unp::ParseErr> =
        Ok(unp::abs("x", unp::app(unp::var("x"), unp::var("y"))));
    assert_eq!(actual, expected);
}
