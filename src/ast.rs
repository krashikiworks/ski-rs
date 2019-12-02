use crate::combinator::Ski;
use crate::error::{FormulaError, InvalidError};
use crate::sequence::Sequence;
use crate::token::{Atom, Token};

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct RawAst {
    data: Token,
    function: Option<Box<RawAst>>,
    argument: Option<Box<RawAst>>,
}

impl From<Token> for RawAst {
    fn from(t: Token) -> Self {
        RawAst {
            data: t,
            function: None,
            argument: None,
        }
    }
}

impl From<Ski> for RawAst {
    fn from(ski: Ski) -> Self {
        match ski {
            Ski::S => RawAst::from(Token::s()),
            Ski::K => RawAst::from(Token::k()),
            Ski::I => RawAst::from(Token::i()),
            Ski::Sp(sp) => {
                let mut ast = RawAst::from(Token::a());
                ast.set_function(RawAst::from(Token::s()));
                ast.set_argument(RawAst::from(sp.get()));
                ast
            }
            Ski::Kp(kp) => {
                let mut ast = RawAst::from(Token::a());
                ast.set_function(RawAst::from(Token::s()));
                ast.set_argument(RawAst::from(kp.get()));
                ast
            }
            Ski::Spp(spp) => {
                //  Spp(i, i) = ``sii
                let mut ast = RawAst::from(Token::a());
                ast.set_function({
                    let mut ast = RawAst::from(Token::a());
                    ast.set_function(RawAst::from(Token::s()));
                    ast.set_argument(RawAst::from(spp.first()));
                    ast
                });
                ast.set_argument(RawAst::from(spp.second()));
                ast
            }
        }
    }
}

impl TryFrom<Sequence> for RawAst {
    type Error = FormulaError;

    fn try_from(value: Sequence) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl FromStr for RawAst {
    type Err = InvalidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequence = match Sequence::from_str(s) {
            Ok(seq) => seq,
            Err(err) => return Err(InvalidError::LexiconError(err)),
        };
        match RawAst::try_from(sequence) {
            Ok(ast) => Ok(ast),
            Err(inverr) => Err(InvalidError::FormulaError(inverr)),
        }
    }
}

impl RawAst {
    // no set_data because data is need by constructor

    pub fn get_data(&self) -> Token {
        self.data.clone()
    }

    pub fn get_function(&self) -> Option<Box<RawAst>> {
        self.function.clone()
    }

    fn set_function(&mut self, ast: RawAst) {
        self.function = Some(Box::new(ast));
    }

    pub fn get_argument(&self) -> Option<Box<RawAst>> {
        self.argument.clone()
    }

    fn set_argument(&mut self, ast: RawAst) {
        self.argument = Some(Box::new(ast));
    }

    fn is_valid_simple(seq: &Sequence) -> Result<(), FormulaError> {
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
            Err(FormulaError::NotEnoughAtoms)
        } else {
            // counter < 0
            Err(FormulaError::SurplusTokens)
        }
    }

    fn search_valid_point(seq: &Sequence) -> Result<usize, FormulaError> {
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
            Err(FormulaError::NotEnoughAtoms)
        } else {
            // counter < 0
            Err(FormulaError::SurplusTokens)
        }
    }

    fn parse(seq: Sequence) -> Result<RawAst, FormulaError> {
        let ast = Self::parse_recursive(seq)?;
        Ok(ast)
    }

    fn parse_recursive(mut seq: Sequence) -> Result<RawAst, FormulaError> {
        RawAst::is_valid_simple(&seq)?;
        // seq can be unwraped safety because seq must not be Option::None (because seq is valid Sequence)
        let token = seq.dequeue().unwrap();

        match token {
            Token::Apply => {
                let mut ast = RawAst::from(Token::a());

                let split_point = RawAst::search_valid_point(&seq)?;
                let (first_half, second_half) = seq.split(split_point + 1);

                let function = RawAst::parse_recursive(first_half)?;
                ast.set_function(function);

                let argument = RawAst::parse_recursive(second_half)?;
                ast.set_argument(argument);
                Ok(ast)
            }
            Token::Atom(atom) => match atom {
                Atom::S => {
                    let ast = RawAst::from(Token::s());
                    Ok(ast)
                }
                Atom::K => {
                    let ast = RawAst::from(Token::k());
                    Ok(ast)
                }
                Atom::I => {
                    let ast = RawAst::from(Token::i());
                    Ok(ast)
                }
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct AstInner {
    function: Box<Ast>,
    argument: Box<Ast>,
}

impl AstInner {
    pub fn to_function(&self) -> Ast {
        *self.function.clone()
    }

    pub fn to_argument(&self) -> Ast {
        *self.function.clone()
    }

    pub fn into_function(self) -> Ast {
        *self.function
    }

    pub fn into_argument(self) -> Ast {
        *self.argument
    }
}

/// Abstruct syntax tree which represents Valid Ski Formula
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Ast {
    Leaf(Atom),
    Apply(AstInner),
}

impl From<Ski> for Ast {
    fn from(ski: Ski) -> Self {
        match ski {
            Ski::S => Ast::Leaf(Atom::S),
            Ski::K => Ast::Leaf(Atom::K),
            Ski::I => Ast::Leaf(Atom::I),
            // Sp(x) = `sx
            Ski::Sp(sp) => Ast::Apply(AstInner {
                function: Box::new(Ast::Leaf(Atom::S)),
                argument: Box::new(Ast::from(sp.get())),
            }),
            // Kp(x) = `kx
            Ski::Kp(kp) => Ast::Apply(AstInner {
                function: Box::new(Ast::Leaf(Atom::S)),
                argument: Box::new(Ast::from(kp.get())),
            }),
            // Spp(x, y) = `sp(x)y = ``sxy = `(`sx)y
            Ski::Spp(spp) => Ast::Apply(AstInner {
                function: Box::new(Ast::Apply(AstInner {
                    function: Box::new(Ast::Leaf(Atom::S)),
                    argument: Box::new(Ast::from(spp.first())),
                })),
                argument: Box::new(Ast::from(spp.second())),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinator::Ski;
    use crate::token::Token;

    #[test]
    fn with_token() {
        let ast = RawAst {
            data: Token::i(),
            function: None,
            argument: None,
        };

        assert_eq!(ast, RawAst::from(Token::i()));
    }

    #[test]
    fn set_function() {
        let mut ast = RawAst::from(Token::i());
        let func = RawAst::from(Token::i());

        let target = RawAst {
            data: Token::i(),
            function: Some(Box::new(RawAst::from(Token::i()))),
            argument: None,
        };

        ast.set_function(func);

        assert_eq!(ast, target);
    }

    #[test]
    fn set_argument() {
        let mut ast = RawAst::from(Token::i());
        let arg = RawAst::from(Token::i());

        let target = RawAst {
            data: Token::i(),
            function: None,
            argument: Some(Box::new(RawAst {
                data: Token::i(),
                function: None,
                argument: None,
            })),
        };

        ast.set_argument(arg);

        assert_eq!(ast, target);
    }

    #[test]
    fn simple_check_valid_token() {
        let t = Sequence::try_from("``sii").unwrap();
        let result = RawAst::is_valid_simple(&t);
        assert_eq!(result, Ok(()));
        let t = Sequence::try_from("`si").unwrap();
        let result = RawAst::is_valid_simple(&t);
        assert_eq!(result, Ok(()));
        let t = Sequence::try_from("`ki").unwrap();
        let result = RawAst::is_valid_simple(&t);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn simple_check_invalid_token() {
        // too much apply token
        let t = Sequence::try_from("```").unwrap();
        let result = RawAst::is_valid_simple(&t);
        assert_eq!(result, Err(FormulaError::NotEnoughAtoms));

        // too much atom token
        let t = Sequence::try_from("`sss").unwrap();
        let result = RawAst::is_valid_simple(&t);
        assert_eq!(result, Err(FormulaError::SurplusTokens));

        // empty token (equals to too much apply)
        let t = Sequence::try_from("").unwrap();
        let result = RawAst::is_valid_simple(&t);
        assert_eq!(result, Err(FormulaError::NotEnoughAtoms));
    }

    #[test]
    fn search_valid_point() {
        let s = Sequence::try_from("``sii").unwrap();
        assert_eq!(RawAst::search_valid_point(&s), Ok(4));
        let s = Sequence::try_from("``siii").unwrap();
        assert_eq!(RawAst::search_valid_point(&s), Ok(4));
        let s = Sequence::try_from("``si").unwrap();
        assert_eq!(
            RawAst::search_valid_point(&s),
            Err(FormulaError::NotEnoughAtoms)
        );

        let mut s = Sequence::try_from("`sk").unwrap();
        assert_eq!(RawAst::search_valid_point(&s), Ok(2));
        s.dequeue(); // will be ["s", "k"]
        assert_eq!(RawAst::search_valid_point(&s), Ok(0));
    }

    #[test]
    fn parse_recursive_valid() {
        let str = "`sk";
        let seq = Sequence::try_from(str).unwrap();
        let ast = RawAst::parse_recursive(seq);

        let mut target = RawAst::from(Token::a());
        let func = RawAst::from(Token::s());
        let arg = RawAst::from(Token::k());
        target.set_function(func);
        target.set_argument(arg);

        assert_eq!(ast, Ok(target));
    }

    #[test]
    fn parse_recursive_invalid() {
        let str = "`s";
        let seq = Sequence::try_from(str).unwrap();
        let ast = RawAst::parse_recursive(seq);

        assert_eq!(ast, Err(FormulaError::NotEnoughAtoms));

        let str = "`ski";
        let seq = Sequence::try_from(str).unwrap();
        let ast = RawAst::parse_recursive(seq);

        assert_eq!(ast, Err(FormulaError::SurplusTokens));
    }

    #[test]
    fn parse_valid() {
        let str = "`sk";
        let seq = Sequence::try_from(str).unwrap();
        let ast = RawAst::parse(seq);

        let mut target = RawAst::from(Token::a());
        let func = RawAst::from(Token::s());
        let arg = RawAst::from(Token::k());
        target.set_function(func);
        target.set_argument(arg);

        assert_eq!(ast, Ok(target));
    }

    #[test]
    fn parse_invalid() {
        let str = "`s";
        let seq = Sequence::try_from(str).unwrap();
        let ast = RawAst::parse_recursive(seq);

        assert_eq!(ast, Err(FormulaError::NotEnoughAtoms));

        let str = "`ski";
        let seq = Sequence::try_from(str).unwrap();
        let ast = RawAst::parse_recursive(seq);

        assert_eq!(ast, Err(FormulaError::SurplusTokens));
    }

    #[test]
    fn from_str() {
        {
            let str = "`sk";
            let ast = RawAst::from_str(str);
            let mut target = RawAst::from(Token::a());
            let func = RawAst::from(Token::s());
            let arg = RawAst::from(Token::k());
            target.set_function(func);
            target.set_argument(arg);

            assert_eq!(ast, Ok(target));
        }

        {
            let str = "`s";
            let ast = RawAst::from_str(str);

            assert_eq!(
                ast,
                Err(InvalidError::FormulaError(FormulaError::NotEnoughAtoms))
            );
        }
    }
}
