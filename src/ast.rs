use std::convert::TryFrom;

use crate::error::{FormulaError, InvalidError};
use crate::sequence::Sequence;
use crate::term::Ski;
use crate::token::{Atom, Token};

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

/// Abstract syntax tree which represents Valid Ski Formula
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

impl TryFrom<Sequence> for Ast {
    type Error = FormulaError;

    fn try_from(mut seq: Sequence) -> Result<Self, Self::Error> {
        seq.is_valid()?;
        assert_ne!(seq.len(), 0);
        match seq.dequeue().unwrap() {
            Token::Atom(atom) => Ok(Ast::Leaf(atom)),
            Token::Apply => {
                let p = seq.valid_point()?;
                let (function, argument) = seq.split(p + 1);
                let function = Box::new(Ast::try_from(function)?);
                let argument = Box::new(Ast::try_from(argument)?);
                Ok(Ast::Apply(AstInner { function, argument }))
            }
        }
    }
}

impl TryFrom<&str> for Ast {
    type Error = InvalidError;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match Sequence::try_from(str) {
            Ok(seq) => match Ast::try_from(seq) {
                Ok(ast) => Ok(ast),
                Err(e) => Err(InvalidError::from(e)),
            },
            Err(e) => Err(InvalidError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Atom::{K, S};

    #[test]
    fn try_from_sequence() {
        let str = "`sk";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::try_from(seq);

        let target = Ast::Apply(AstInner {
            function: Box::new(Ast::Leaf(S)),
            argument: Box::new(Ast::Leaf(K)),
        });
        assert_eq!(ast, Ok(target));

        let str = "`s";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::try_from(seq);

        assert_eq!(ast, Err(FormulaError::NotEnoughAtoms));

        let str = "`ski";
        let seq = Sequence::try_from(str).unwrap();
        let ast = Ast::try_from(seq);

        assert_eq!(ast, Err(FormulaError::SurplusTokens));
    }
}
