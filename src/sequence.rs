use crate::ast::Ast;
use crate::error::LexiconError;
use crate::token::Token;

use std::collections::VecDeque;
use std::convert::{From, TryFrom};
use std::str::FromStr;

/// Unlambda-style SKI expression として有効な記号の列。Unlambda-style SKI expression として有効な式はこの中に含まれる。

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Sequence(VecDeque<Token>);

impl From<VecDeque<Token>> for Sequence {
    fn from(inner: VecDeque<Token>) -> Self {
        Sequence(inner)
    }
}

impl From<Vec<Token>> for Sequence {
    fn from(vec: Vec<Token>) -> Self {
        Sequence(VecDeque::from(vec))
    }
}

// TODO: TryFromにするべき?
impl From<Ast> for Sequence {
    fn from(ast: Ast) -> Self {
        // TODO: 非常に実装が汚い
        let mut vec = VecDeque::new();
        vec.push_back(ast.get_data());
        let mut seq = Sequence::from(vec);
        match ast.get_function() {
            Some(f) => seq.join(&Sequence::from(*f)),
            None => (),
        };
        match ast.get_argument() {
            Some(a) => seq.join(&Sequence::from(*a)),
            None => (),
        };
        seq
    }
}

impl TryFrom<&str> for Sequence {
    type Error = LexiconError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut v = VecDeque::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '`' => v.push_back(Token::a()),
                's' => v.push_back(Token::s()),
                'k' => v.push_back(Token::k()),
                'i' => v.push_back(Token::i()),
                _ => return Err(LexiconError::new(i as u32, c)),
            }
        }
        Ok(Sequence::from(v))
    }
}

impl FromStr for Sequence {
    type Err = LexiconError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Sequence::try_from(s)
    }
}

impl IntoIterator for Sequence {
    type Item = Token;
    type IntoIter = std::collections::vec_deque::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// TODO: 何で動いているのか我々にも分からんのです
impl<'a> IntoIterator for &'a Sequence {
    type Item = &'a Token;
    type IntoIter = std::collections::vec_deque::Iter<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Sequence {
    pub fn pop(&mut self) -> Option<Token> {
        self.0.pop_back()
    }

    pub fn split(mut self, index: usize) -> (Sequence, Sequence) {
        let t2 = self.0.split_off(index);
        (self, Sequence::from(t2))
    }

    pub fn dequeue(&mut self) -> Option<Token> {
        self.0.pop_front()
    }

    pub fn join(&mut self, seq: &Self) {
        for t in seq {
            self.0.push_back(*t)
        }
    }
}

mod tests {

    use super::*;

    #[test]
    fn from_str_valid() {
        let string = "```s``kii";
        let vec = vec![
            Token::a(),
            Token::a(),
            Token::a(),
            Token::s(),
            Token::a(),
            Token::a(),
            Token::k(),
            Token::i(),
            Token::i(),
        ];
        let vec = VecDeque::from(vec);
        let tokens = Sequence::from(vec);
        println!("{:?}", tokens);
        assert_eq!(Sequence::try_from(string).unwrap(), tokens);
    }

    #[test]
    fn from_str_invalid() {
        let string = "`ski-kk";
        assert_eq!(Sequence::try_from(string), Err(LexiconError::new(4, '-')));
    }

    #[test]
    fn from_str_print() {
        let str = "```s``kii";
        let tokens = Sequence::try_from(str).unwrap();
        println!("tokenize_print: '{}' -> {:?}", str, tokens);
    }

    #[test]
    fn split() {
        let seq = Sequence::try_from("```iii").unwrap();
        let res = Sequence::split(seq, 3);
        let t1 = Sequence::try_from("```").unwrap();
        let t2 = Sequence::try_from("iii").unwrap();
        assert_eq!(res, (t1, t2));

        let mut seq = Sequence::try_from("`sk").unwrap();
        seq.dequeue(); // will be ["s", "k"]
        let res = Sequence::split(seq, 0); // will be ("", "s, k")
        let t1 = Sequence::try_from("").unwrap();
        let t2 = Sequence::try_from("sk").unwrap();
        assert_eq!(res, (t1, t2));
    }

    #[test]
    fn dequeue() {
        let mut seq = Sequence::try_from("`sk").unwrap();
        assert_eq!(seq.dequeue(), Some(Token::a()));
        assert_eq!(seq.dequeue(), Some(Token::s()));
        assert_eq!(seq.dequeue(), Some(Token::k()));
        assert_eq!(seq.dequeue(), None);
    }

    #[test]
    fn into_iter() {
        let mut seq = Sequence::try_from("`sk").unwrap();
        for (u, t) in (&seq).into_iter().enumerate() {
            if u == 0 {
                assert_eq!(t, &Token::a());
            }
            if u == 1 {
                assert_eq!(t, &Token::s());
            }
            if u == 2 {
                assert_eq!(t, &Token::k());
            }
        }

        seq.dequeue();

        for (u, t) in (&seq).into_iter().enumerate() {
            if u == 0 {
                assert_eq!(t, &Token::s());
            }
            if u == 1 {
                assert_eq!(t, &Token::k());
            }
        }
    }
}
