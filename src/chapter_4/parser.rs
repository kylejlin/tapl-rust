use super::*;
use lexer::Token;

#[cfg(test)]
mod test;

pub fn parse(src: &str) -> Option<Term> {
    lexer::tokenize(src).and_then(|tokens| parse_tokens(&tokens))
}

fn parse_tokens(tokens: &[Token]) -> Option<Term> {
    Parser::from_tokens(tokens).parse()
}

struct Parser<'a> {
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    fn from_tokens(tokens: &'a [Token]) -> Parser<'a> {
        Parser { tokens }
    }

    fn parse(mut self) -> Option<Term> {
        let term = self.accept_term();
        if self.is_exhausted() {
            term
        } else {
            None
        }
    }

    fn accept_term(&mut self) -> Option<Term> {
        if self.accept_token(Token::True) {
            Some(Term::True)
        } else if self.accept_token(Token::False) {
            Some(Term::False)
        } else if let Some(t) = self.accept_if() {
            Some(t)
        } else if self.accept_token(Token::Zero) {
            Some(Term::Zero)
        } else if let Some(t) = self.accept_succ() {
            Some(t)
        } else if let Some(t) = self.accept_pred() {
            Some(t)
        } else if let Some(t) = self.accept_is_zero() {
            Some(t)
        } else {
            None
        }
    }

    fn accept_token(&mut self, expected: Token) -> bool {
        let Parser { tokens } = self;
        if !tokens.is_empty() && tokens[0] == expected {
            self.tokens = &tokens[1..];
            true
        } else {
            false
        }
    }

    fn accept_if(&mut self) -> Option<Term> {
        if self.accept_token(Token::If) {
            if let Some(guard) = self.accept_term() {
                if self.accept_token(Token::Then) {
                    if let Some(t1) = self.accept_term() {
                        if self.accept_token(Token::Else) {
                            if let Some(t2) = self.accept_term() {
                                return Some(term_builder::if_(guard, t1, t2));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn accept_succ(&mut self) -> Option<Term> {
        if self.accept_token(Token::Succ) {
            if let Some(t) = self.accept_term() {
                return Some(term_builder::succ(t));
            }
        }

        None
    }

    fn accept_pred(&mut self) -> Option<Term> {
        if self.accept_token(Token::Pred) {
            if let Some(t) = self.accept_term() {
                return Some(term_builder::pred(t));
            }
        }

        None
    }

    fn accept_is_zero(&mut self) -> Option<Term> {
        if self.accept_token(Token::IsZero) {
            if let Some(t) = self.accept_term() {
                return Some(term_builder::is_zero(t));
            }
        }

        None
    }

    fn is_exhausted(&self) -> bool {
        self.tokens.is_empty()
    }
}
