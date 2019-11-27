use std::convert::TryFrom;

use crate::ast::Ast;
use crate::combinator::Ski;
use crate::error::InvalidError;
use crate::sequence::Sequence;
use crate::token::{Atom, Token};

struct Stax {
    program: Sequence,
    stack: Vec<Ski>,
}

impl Stax {
    // programからargumentに全て移し終わった時にargumentに2つ以上残っていたらInvalidError::SurplusTokens
    // argumentからpopしようとしてできなかったならInvalidError::NotEnoughAtoms
    fn eval(&mut self) -> Result<Sequence, InvalidError> {
        while let Some(top) = self.program.pop() {
            match top {
                Token::Atom(a) => match a {
                    Atom::S => self.stack.push(Ski::try_from(Token::s()).unwrap()),
                    Atom::K => self.stack.push(Ski::try_from(Token::k()).unwrap()),
                    Atom::I => self.stack.push(Ski::try_from(Token::i()).unwrap()),
                },
                Token::Apply(_) => {
                    let result = self.apply()?;
                    self.program.join(&Sequence::from(Ast::from(result)))
                }
            }
        }
        if self.stack.len() != 1 {
            Err(InvalidError::SurplusTokens)
        } else {
            Ok(Sequence::from(Ast::from(self.stack.pop().unwrap())))
        }
    }

    fn apply(&mut self) -> Result<Ski, InvalidError> {
        let function = self.pop()?;
        let argument = self.pop()?;

        Ok(function.apply_pub(argument))
    }

    fn pop(&mut self) -> Result<Ski, InvalidError> {
        match self.stack.pop() {
            Some(ski) => Ok(ski),
            None => Err(InvalidError::NotEnoughAtoms),
        }
    }
}
