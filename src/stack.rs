use crate::ast::Ast;
use crate::error::FormulaError;
use crate::lambda::Apply;
use crate::sequence::Sequence;
use crate::term::Ski;
use crate::token::{Atom, Token};

pub struct Stax {
    program: Sequence,
    stack: Vec<Ski>,
}

impl From<Sequence> for Stax {
    fn from(program: Sequence) -> Self {
        Stax {
            program,
            stack: Vec::new(),
        }
    }
}

impl Stax {
    // programからargumentに全て移し終わった時にargumentに2つ以上残っていたらInvalidError::SurplusTokens
    // argumentからpopしようとしてできなかったならInvalidError::NotEnoughAtoms
    pub fn eval(&mut self) -> Result<Sequence, FormulaError> {
        while let Some(top) = self.program.pop() {
            match top {
                Token::Atom(a) => match a {
                    Atom::S => self.stack.push(Ski::from(Atom::S)),
                    Atom::K => self.stack.push(Ski::from(Atom::K)),
                    Atom::I => self.stack.push(Ski::from(Atom::I)),
                },
                Token::Apply => {
                    let result = self.apply()?;
                    self.stack.push(result);
                }
            }
        }

        if self.stack.len() != 1 {
            Err(FormulaError::SurplusTokens)
        } else {
            Ok(Sequence::from(Ast::from(self.stack.pop().unwrap())))
        }
    }

    fn apply(&mut self) -> Result<Ski, FormulaError> {
        let function = self.pop()?;
        let argument = self.pop()?;
        let ret = function.apply(argument);

        Ok(ret)
    }

    fn pop(&mut self) -> Result<Ski, FormulaError> {
        match self.stack.pop() {
            Some(ski) => Ok(ski),
            None => Err(FormulaError::NotEnoughAtoms),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn eval() {
        let str = "```s``kii```skiis";
        let seq = Sequence::try_from(str).unwrap();
        let mut stax = Stax::from(seq);
        let result = stax.eval();

        let target = Sequence::try_from("`ss").unwrap();

        assert_eq!(result, Ok(target));
    }
}
