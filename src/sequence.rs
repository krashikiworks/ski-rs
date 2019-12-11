use std::collections::VecDeque;
use std::convert::{From, TryFrom};
use std::ops::Add;

use crate::ast::Ast;
use crate::error::{FormulaError, LexiconError};
use crate::token::Token;

/// Unlambda-style SKI expression として有効なTokenの列。有効なUnlambda-style SKI formula はこの中に含まれる。
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Sequence(VecDeque<Token>);

impl From<Token> for Sequence {
    fn from(token: Token) -> Self {
        Sequence(VecDeque::from(vec![token]))
    }
}

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

impl From<Ast> for Sequence {
    fn from(ast: Ast) -> Self {
        match ast {
            Ast::Leaf(leaf) => Sequence::from(Token::from(leaf)),
            Ast::Apply(child) => {
                Sequence::from(Token::a())
                    + Sequence::from(child.to_function())
                    + Sequence::from(child.to_argument())
            }
        }
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

impl Add for Sequence {
    type Output = Self;

    fn add(self, arg: Self) -> Self::Output {
        let mut v = VecDeque::new();
        for t in self.0 {
            v.push_back(t)
        }
        for t in arg.0 {
            v.push_back(t)
        }
        Sequence::from(v)
        // この後argは使用不可…だよな?
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
    pub fn new() -> Self {
        Sequence(VecDeque::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

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

    pub fn is_valid(&self) -> Result<bool, FormulaError> {
        let mut counter = 1;
        for t in self {
            match t {
                Token::Apply => counter += 1,
                Token::Atom(_) => counter -= 1,
            }
        }
        if counter == 0 {
            Ok(true)
        } else if counter > 0 {
            Err(FormulaError::NotEnoughAtoms)
        } else {
            // counter < 0
            Err(FormulaError::SurplusTokens)
        }
    }

    pub fn valid_point(&self) -> Result<usize, FormulaError> {
        let mut counter = 1;
        for (ord, t) in self.into_iter().enumerate() {
            match t {
                Token::Apply => counter += 1,
                Token::Atom(_) => counter -= 1,
            }
            if counter == 0 {
                return Ok(ord);
            }
        }
        if counter > 0 {
            Err(FormulaError::NotEnoughAtoms)
        } else {
            // counter < 0
            Err(FormulaError::SurplusTokens)
        }
    }

    pub fn cut_formula(self) -> Result<Self, FormulaError> {
        let point = self.valid_point()?;
        let (ret, _) = self.split(point);
        Ok(ret)
    }

    pub fn formula_and_rest(self) -> Result<(Self, Self), FormulaError> {
        let point = self.valid_point()?;
        let ret = self.split(point);
        Ok(ret)
    }
}

#[cfg(test)]
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

    #[test]
    fn simple_check_valid_token() {
        let seq = Sequence::try_from("``sii").unwrap();
        let result = seq.is_valid();
        assert_eq!(result, Ok(true));
        let seq = Sequence::try_from("`si").unwrap();
        let result = seq.is_valid();
        assert_eq!(result, Ok(true));
        let seq = Sequence::try_from("`ki").unwrap();
        let result = seq.is_valid();
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn simple_check_invalid_token() {
        // too much apply token
        let seq = Sequence::try_from("```").unwrap();
        let result = seq.is_valid();
        assert_eq!(result, Err(FormulaError::NotEnoughAtoms));

        // too much atom token
        let seq = Sequence::try_from("`sss").unwrap();
        let result = seq.is_valid();
        assert_eq!(result, Err(FormulaError::SurplusTokens));

        // empty token (equals to too much apply)
        let seq = Sequence::try_from("").unwrap();
        let result = seq.is_valid();
        assert_eq!(result, Err(FormulaError::NotEnoughAtoms));
    }

    #[test]
    fn search_valid_point() {
        let s = Sequence::try_from("``sii").unwrap();
        assert_eq!(s.valid_point(), Ok(4));
        let s = Sequence::try_from("``siii").unwrap();
        assert_eq!(s.valid_point(), Ok(4));
        let s = Sequence::try_from("``si").unwrap();
        assert_eq!(s.valid_point(), Err(FormulaError::NotEnoughAtoms));

        let mut s = Sequence::try_from("`sk").unwrap();
        assert_eq!(s.valid_point(), Ok(2));
        s.dequeue(); // will be ["s", "k"]
        assert_eq!(s.valid_point(), Ok(0));
    }
}
