use crate::combinator::Ski;
use crate::error::{InvalidError, ParseAstError};
use crate::sequence::Sequence;
use crate::token::Token;

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Ast {
    data: Token,
    // TODO: functionは無いけどargumentはあるAstの取り扱いについて
    function: Option<Box<Ast>>,
    argument: Option<Box<Ast>>,
}

impl From<Token> for Ast {
    fn from(t: Token) -> Self {
        Ast {
            data: t,
            function: None,
            argument: None,
        }
    }
}

impl From<Ski> for Ast {
    fn from(ski: Ski) -> Self {
        match ski {
            Ski::S(_) => Ast::from(Token::S),
            Ski::K(_) => Ast::from(Token::K),
            Ski::I(_) => Ast::from(Token::I),
            Ski::Sp(sp) => {
                let mut ast = Ast::from(Token::Apply);
                ast.set_function(Ast::from(Token::S));
                ast.set_argument(Ast::from(sp.get()));
                ast
            }
            Ski::Kp(kp) => {
                let mut ast = Ast::from(Token::Apply);
                ast.set_function(Ast::from(Token::S));
                ast.set_argument(Ast::from(kp.get()));
                ast
            }
            Ski::Spp(spp) => {
                //  Spp(i, i) = ``sii
                let mut ast = Ast::from(Token::Apply);
                ast.set_function({
                    let mut ast = Ast::from(Token::Apply);
                    ast.set_function(Ast::from(Token::S));
                    ast.set_argument(Ast::from(spp.get_first()));
                    ast
                });
                ast.set_argument(Ast::from(spp.get_second()));
                ast
            }
        }
    }
}

impl TryFrom<Sequence> for Ast {
    type Error = InvalidError;

    fn try_from(value: Sequence) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seqence = match Sequence::from_str(s) {
            Ok(seq) => seq,
            Err(lexerr) => return Err(ParseAstError::LexiconError(lexerr)),
        };
        match Ast::try_from(seqence) {
            Ok(ast) => Ok(ast),
            Err(inverr) => Err(ParseAstError::InvalidError(inverr)),
        }
    }
}

impl Ast {
    // no set_data because data is need by constructor

    pub fn get_data(&self) -> Token {
        self.data.clone()
    }

    pub fn get_function(&self) -> Option<Box<Ast>> {
        match self.function {
            Some(b) => Some(b),
            None => None,
        }
    }

    fn set_function(&mut self, ast: Ast) {
        self.function = Some(Box::new(ast));
    }

    pub fn get_argument(&self) -> Option<Box<Ast>> {
        match self.argument {
            Some(b) => Some(b),
            None => None,
        }
    }

    fn set_argument(&mut self, ast: Ast) {
        self.argument = Some(Box::new(ast));
    }

    fn is_valid_simple(seq: &Sequence) -> Result<(), InvalidError> {
        let mut counter = 1;
        for t in seq {
            match t {
                Token::Apply => counter += 1,
                _ => counter -= 1,
            }
        }
        if counter == 0 {
            Ok(())
        } else if counter > 0 {
            Err(InvalidError::NotEnoughAtoms)
        } else {
            // counter < 0
            Err(InvalidError::SurplusTokens)
        }
    }

    fn search_valid_point(seq: &Sequence) -> Result<usize, InvalidError> {
        let mut counter = 1;
        for (ord, t) in seq.into_iter().enumerate() {
            match t {
                Token::Apply => counter += 1,
                _ => counter -= 1,
            }
            if counter == 0 {
                return Ok(ord);
            }
        }
        if counter > 0 {
            Err(InvalidError::NotEnoughAtoms)
        } else {
            // counter < 0
            Err(InvalidError::SurplusTokens)
        }
    }

    fn parse(seq: Sequence) -> Result<Ast, InvalidError> {
        let ast = Self::parse_recursive(seq)?;
        Ok(ast)
    }

    fn parse_recursive(mut seq: Sequence) -> Result<Ast, InvalidError> {
        Ast::is_valid_simple(&seq)?;
        // seq can be unwraped safety because seq must not be Option::None (because seq is valid Sequence)
        let token = seq.dequeue().unwrap();

        match token {
            Token::Apply => {
                let mut ast = Ast::from(Token::Apply);

                let split_point = Ast::search_valid_point(&seq)?;
                let (first_half, second_half) = seq.split(split_point + 1);

                let function = Ast::parse_recursive(first_half)?;
                ast.set_function(function);

                let argument = Ast::parse_recursive(second_half)?;
                ast.set_argument(argument);
                Ok(ast)
            }
            Token::S => {
                let ast = Ast::from(Token::S);
                Ok(ast)
            }
            Token::K => {
                let ast = Ast::from(Token::K);
                Ok(ast)
            }
            Token::I => {
                let ast = Ast::from(Token::I);
                Ok(ast)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token::*;

    #[test]
    fn with_token() {
        let ast = Ast {
            data: I,
            function: None,
            argument: None,
        };

        assert_eq!(ast, Ast::from(I));
    }

    #[test]
    fn set_function() {
        let mut ast = Ast::from(Token::I);
        let func = Ast::from(Token::I);

        let target = Ast {
            data: I,
            function: Some(Box::new(Ast::from(I))),
            argument: None,
        };

        ast.set_function(func);

        assert_eq!(ast, target);
    }

    #[test]
    fn set_argument() {
        let mut ast = Ast::from(Token::I);
        let arg = Ast::from(Token::I);

        let target = Ast {
            data: I,
            function: None,
            argument: Some(Box::new(Ast {
                data: I,
                function: None,
                argument: None,
            })),
        };

        ast.set_argument(arg);

        assert_eq!(ast, target);
    }

    #[test]
    fn simple_check_valid_tokens() {
        let t = Sequence::try_from("``sii").unwrap();
        let result = Ast::is_valid_simple(&t);
        assert_eq!(result, Ok(()));
        let t = Sequence::try_from("`si").unwrap();
        let result = Ast::is_valid_simple(&t);
        assert_eq!(result, Ok(()));
        let t = Sequence::try_from("`ki").unwrap();
        let result = Ast::is_valid_simple(&t);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn simple_check_invalid_tokens() {
        // too much apply token
        let t = Sequence::try_from("```").unwrap();
        let result = Ast::is_valid_simple(&t);
        assert_eq!(result, Err(InvalidError::NotEnoughAtoms));

        // too much atom token
        let t = Sequence::try_from("`sss").unwrap();
        let result = Ast::is_valid_simple(&t);
        assert_eq!(result, Err(InvalidError::SurplusTokens));

        // empty token (equals to too much apply)
        let t = Sequence::try_from("").unwrap();
        let result = Ast::is_valid_simple(&t);
        assert_eq!(result, Err(InvalidError::NotEnoughAtoms));
    }

    #[test]
    fn search_valid_point() {
        let s = Sequence::try_from("``sii").unwrap();
        assert_eq!(Ast::search_valid_point(&s), Ok(4));
        let s = Sequence::try_from("``siii").unwrap();
        assert_eq!(Ast::search_valid_point(&s), Ok(4));
        let s = Sequence::try_from("``si").unwrap();
        assert_eq!(
            Ast::search_valid_point(&s),
            Err(InvalidError::NotEnoughAtoms)
        );

        let mut s = Sequence::try_from("`sk").unwrap();
        assert_eq!(Ast::search_valid_point(&s), Ok(2));
        s.dequeue(); // will be ["s", "k"]
        assert_eq!(Ast::search_valid_point(&s), Ok(0));
    }

    #[test]
    fn parse_recursive_valid() {
        let str = "`sk";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::parse_recursive(seq);

        let mut target = Ast::from(Apply);
        let func = Ast::from(S);
        let arg = Ast::from(K);
        target.set_function(func);
        target.set_argument(arg);

        assert_eq!(ast, Ok(target));
    }

    #[test]
    fn parse_recursive_invalid() {
        let str = "`s";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::parse_recursive(seq);

        assert_eq!(ast, Err(InvalidError::NotEnoughAtoms));

        let str = "`ski";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::parse_recursive(seq);

        assert_eq!(ast, Err(InvalidError::SurplusTokens));
    }

    #[test]
    fn parse_valid() {
        let str = "`sk";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::parse(seq);

        let mut target = Ast::from(Apply);
        let func = Ast::from(S);
        let arg = Ast::from(K);
        target.set_function(func);
        target.set_argument(arg);

        assert_eq!(ast, Ok(target));
    }

    #[test]
    fn parse_invalid() {
        let str = "`s";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::parse_recursive(seq);

        assert_eq!(ast, Err(InvalidError::NotEnoughAtoms));

        let str = "`ski";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::parse_recursive(seq);

        assert_eq!(ast, Err(InvalidError::SurplusTokens));
    }

    #[test]
    fn from_str() {
        {
            let str = "`sk";
            let ast = Ast::from_str(str);
            let mut target = Ast::from(Apply);
            let func = Ast::from(S);
            let arg = Ast::from(K);
            target.set_function(func);
            target.set_argument(arg);

            assert_eq!(ast, Ok(target));
        }

        {
            let str = "`s";
            let ast = Ast::from_str(str);

            assert_eq!(
                ast,
                Err(ParseAstError::InvalidError(InvalidError::NotEnoughAtoms))
            );
        }
    }
}
