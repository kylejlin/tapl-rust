mod util;

use super::parse;
use unp::IntoUnpositioned;
use util::unpositioned as unp;

#[test]
fn var() {
    let actual = parse(r"x").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::var("x"));
    assert_eq!(actual, expected);
}

#[test]
fn var_names() {
    const LEGAL_NAMES: [&str; 10] = ["x", "_", "x_", "_x", "x1", "_1", "x'", "x''", "X", "__"];

    for name in &LEGAL_NAMES {
        let actual = parse(name).into_unpositioned();
        let expected: Result<_, unp::ParseErr> = Ok(unp::var(name));
        assert_eq!(actual, expected);
    }
}

#[test]
fn var_whitespace() {
    let actual = parse(r" x  ").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::var("x"));
    assert_eq!(actual, expected);
}

#[test]
fn var_extraneous_parens() {
    let actual = parse(r"(x)").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::var("x"));
    assert_eq!(actual, expected);
}

#[test]
fn identity() {
    let actual = parse(r"\x.x").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::abs("x", unp::var("x")));
    assert_eq!(actual, expected);
}

#[test]
fn identity_whitespace() {
    let actual = parse(
        r" \    x 
    .  x  ",
    )
    .into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::abs("x", unp::var("x")));
    assert_eq!(actual, expected);
}

#[test]
fn identity_top_level_extraneous_parens() {
    let actual = parse(r"(\x.x)").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::abs("x", unp::var("x")));
    assert_eq!(actual, expected);
}

#[test]
fn identity_body_extraneous_parens() {
    let actual = parse(r"\x.(x)").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::abs("x", unp::var("x")));
    assert_eq!(actual, expected);
}

#[test]
fn open_abs() {
    let actual = parse(r"\x.x y").into_unpositioned();
    let expected: Result<_, unp::ParseErr> =
        Ok(unp::abs("x", unp::app(unp::var("x"), unp::var("y"))));
    assert_eq!(actual, expected);
}

#[test]
fn app() {
    let actual = parse(r"x y").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(unp::var("x"), unp::var("y")));
    assert_eq!(actual, expected);
}

#[test]
fn app_extraneous_top_level_parens() {
    let actual = parse(r"(x y)").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(unp::var("x"), unp::var("y")));
    assert_eq!(actual, expected);
}

#[test]
fn app_extraneous_lhs_parens() {
    let actual = parse(r"(x) y").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(unp::var("x"), unp::var("y")));
    assert_eq!(actual, expected);
}

#[test]
fn app_extraneous_rhs_parens() {
    let actual = parse(r"x (y)").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(unp::var("x"), unp::var("y")));
    assert_eq!(actual, expected);
}

#[test]
fn left_assoc_app() {
    let actual = parse(r"x y z").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(
        unp::app(unp::var("x"), unp::var("y")),
        unp::var("z"),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn complex_app() {
    let actual = parse(r"v w (x y) z").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(
        unp::app(
            unp::app(unp::var("v"), unp::var("w")),
            unp::app(unp::var("x"), unp::var("y")),
        ),
        unp::var("z"),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn top_level_abs_extend_as_far_as_possible() {
    let actual = parse(r"\x. \y. \z. x y z").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::abs(
        "x",
        unp::abs(
            "y",
            unp::abs(
                "z",
                unp::app(unp::app(unp::var("x"), unp::var("y")), unp::var("z")),
            ),
        ),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn inner_abs_extend_as_far_as_possible() {
    let actual = parse(r"f \x. \y. \z. x y z").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(
        unp::var("f"),
        unp::abs(
            "x",
            unp::abs(
                "y",
                unp::abs(
                    "z",
                    unp::app(unp::app(unp::var("x"), unp::var("y")), unp::var("z")),
                ),
            ),
        ),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn top_level_redex() {
    let actual = parse(r"(\x. \y. \z. x y z) \x. x").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(
        unp::abs(
            "x",
            unp::abs(
                "y",
                unp::abs(
                    "z",
                    unp::app(unp::app(unp::var("x"), unp::var("y")), unp::var("z")),
                ),
            ),
        ),
        unp::abs("x", unp::var("x")),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn top_level_redex_extraneous_parens() {
    let actual = parse(r"(\x. \y. \z. x y z) (\x. x)").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(
        unp::abs(
            "x",
            unp::abs(
                "y",
                unp::abs(
                    "z",
                    unp::app(unp::app(unp::var("x"), unp::var("y")), unp::var("z")),
                ),
            ),
        ),
        unp::abs("x", unp::var("x")),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn multiple_abs_args() {
    let actual = parse(r"(\x. \y. \z. x y z) (\x. x) \y. y").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(
        unp::app(
            unp::abs(
                "x",
                unp::abs(
                    "y",
                    unp::abs(
                        "z",
                        unp::app(unp::app(unp::var("x"), unp::var("y")), unp::var("z")),
                    ),
                ),
            ),
            unp::abs("x", unp::var("x")),
        ),
        unp::abs("y", unp::var("y")),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn multiple_abs_args_extraneous_parens() {
    let actual = parse(r"(\x. \y. \z. x y z) (\x. x) (\y. y)").into_unpositioned();
    let expected: Result<_, unp::ParseErr> = Ok(unp::app(
        unp::app(
            unp::abs(
                "x",
                unp::abs(
                    "y",
                    unp::abs(
                        "z",
                        unp::app(unp::app(unp::var("x"), unp::var("y")), unp::var("z")),
                    ),
                ),
            ),
            unp::abs("x", unp::var("x")),
        ),
        unp::abs("y", unp::var("y")),
    ));
    assert_eq!(actual, expected);
}
