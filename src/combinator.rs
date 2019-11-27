use std::convert::TryFrom;
use std::marker::PhantomData;

use crate::error::InvalidTokenAsArgument;
use crate::token::{Atom, Token};

trait Combinator {
    type Argument: Combinator;
    type Target: Combinator;

    fn apply(&self, arg: Self::Argument) -> Self::Target;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Container1<T> {
    content: Box<Ski>,
    _phantom_data: PhantomData<fn() -> T>,
}

impl<T> From<Ski> for Container1<T> {
    fn from(content: Ski) -> Self {
        Self {
            content: Box::new(content),
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Container1<T> {
    fn get(&self) -> Ski {
        *self.content.clone()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Container2<T> {
    content_1: Box<Ski>,
    content_2: Box<Ski>,
    _phantom_data: PhantomData<fn() -> T>,
}

impl<T> From<(Ski, Ski)> for Container2<T> {
    fn from(content: (Ski, Ski)) -> Self {
        Self {
            content_1: Box::new(content.0),
            content_2: Box::new(content.1),
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Container2<T> {
    fn first(&self) -> Ski {
        *self.content_1.clone()
    }

    fn second(&self) -> Ski {
        *self.content_2.clone()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Ski {
    S(S),
    K(K),
    I(I),
    Sp(Sp),
    Kp(Kp),
    Spp(Spp),
}

impl TryFrom<Token> for Ski {
    type Error = InvalidTokenAsArgument;

    fn try_from(t: Token) -> Result<Self, Self::Error> {
        match t {
            Token::Atom(a) => match a {
                Atom::S => Ok(Ski::S(S {})),
                Atom::K => Ok(Ski::K(K {})),
                Atom::I => Ok(Ski::I(I {})),
            },
            Token::Apply(_) => Err(InvalidTokenAsArgument::new(Token::a())),
        }
    }
}

impl Combinator for Ski {
    type Argument = Self;
    type Target = Self;

    fn apply(&self, arg: Self::Argument) -> Self::Target {
        match self {
            Ski::S(s) => Ski::Sp(s.apply(arg)),
            Ski::K(k) => Ski::Kp(k.apply(arg)),
            Ski::I(i) => i.apply(arg),
            Ski::Sp(sp) => Ski::Spp(sp.apply(arg)),
            Ski::Kp(kp) => kp.apply(arg),
            Ski::Spp(spp) => spp.apply(arg),
        }
    }
}

impl Ski {
    // FIXME: あまりに不格好な実装
    pub fn apply_pub(&self, arg: Self) -> Self {
        self.apply(arg)
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]

pub struct S {}

impl Combinator for S {
    type Argument = Ski;
    type Target = Sp;

    fn apply(&self, arg: Self::Argument) -> Self::Target {
        Sp {
            container: Container1::from(arg),
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]

pub struct K {}

impl Combinator for K {
    type Argument = Ski;
    type Target = Kp;

    fn apply(&self, arg: Self::Argument) -> Self::Target {
        Kp {
            container: Container1::from(arg),
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]

pub struct I {}

impl Combinator for I {
    type Argument = Ski;
    type Target = Ski;

    fn apply(&self, arg: Self::Argument) -> Self::Target {
        arg
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]

pub struct Sp {
    container: Container1<Sp>,
}

impl Combinator for Sp {
    type Argument = Ski;
    type Target = Spp;

    fn apply(&self, arg: Self::Argument) -> Self::Target {
        Spp {
            content: Container2::from((self.container.get(), arg)),
        }
    }
}

impl Sp {
    pub fn get(&self) -> Ski {
        self.container.get()
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]

pub struct Kp {
    container: Container1<Kp>,
}

impl Combinator for Kp {
    type Argument = Ski;
    type Target = Ski;

    fn apply(&self, _arg: Self::Argument) -> Self::Target {
        self.container.get()
    }
}

impl Kp {
    pub fn get(&self) -> Ski {
        self.container.get()
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]

pub struct Spp {
    content: Container2<Spp>,
}

impl Combinator for Spp {
    type Argument = Ski;
    type Target = Ski;

    fn apply(&self, arg: Self::Argument) -> Self::Target {
        self.content
            .first()
            .apply(arg.clone())
            .apply(self.content.second().apply(arg))
    }
}

impl Spp {
    pub fn get_first(&self) -> Ski {
        self.content.first()
    }

    pub fn get_second(&self) -> Ski {
        self.content.second()
    }
}

mod tests {

    use super::*;

    #[test]
    fn s_apply() {}
}
