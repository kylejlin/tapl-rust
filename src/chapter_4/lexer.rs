#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    True,
    False,
    If,
    Then,
    Else,
    Zero,
    Succ,
    Pred,
    IsZero,
}

static TOKENS: [(&str, Token); 9] = [
    ("true", Token::True),
    ("false", Token::False),
    ("if", Token::If),
    ("then", Token::Then),
    ("else", Token::Else),
    ("0", Token::Zero),
    ("succ", Token::Succ),
    ("pred", Token::Pred),
    ("iszero", Token::IsZero),
];

pub fn tokenize(mut src: &str) -> Option<Vec<Token>> {
    let mut out = Vec::new();

    while !src.is_empty() {
        if src.starts_with(char::is_whitespace) || src.starts_with('(') || src.starts_with(')') {
            src = &src[1..];
        } else if let Some((substr, token)) = get_leading_token(&src) {
            out.push(token);
            src = &src[substr.len()..];
        } else {
            return None;
        }
    }

    Some(out)
}

fn get_leading_token(src: &str) -> Option<(&str, Token)> {
    for (substr, token) in &TOKENS {
        if src.starts_with(substr) {
            return Some((substr, *token));
        }
    }
    None
}
