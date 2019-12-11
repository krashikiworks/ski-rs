use crate::lambda::{Apply, Eval, Lambda};
use crate::token::Atom;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct SkiContainer1 {
    content: Box<Ski>,
}

impl From<Ski> for SkiContainer1 {
    fn from(content: Ski) -> Self {
        Self {
            content: Box::new(content),
        }
    }
}

impl SkiContainer1 {
    pub fn get(&self) -> Ski {
        *self.content.clone()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct SkiContainer2 {
    content_1: Box<Ski>,
    content_2: Box<Ski>,
}

impl From<(Ski, Ski)> for SkiContainer2 {
    fn from(content: (Ski, Ski)) -> Self {
        Self {
            content_1: Box::new(content.0),
            content_2: Box::new(content.1),
        }
    }
}

impl SkiContainer2 {
    pub fn first(&self) -> Ski {
        *self.content_1.clone()
    }

    pub fn second(&self) -> Ski {
        *self.content_2.clone()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Ski {
    S,
    K,
    I,
    Sp(SkiContainer1),
    Kp(SkiContainer1),
    Spp(SkiContainer2),
}

impl From<Atom> for Ski {
    fn from(a: Atom) -> Self {
        match a {
            Atom::S => Ski::S,
            Atom::K => Ski::K,
            Atom::I => Ski::I,
        }
    }
}

impl Apply for Ski {
    type Argument = Self;
    type Target = Self;

    fn apply(&self, arg: Self::Argument) -> Self::Target {
        match self {
            Ski::S => Ski::Sp(SkiContainer1::from(arg)),
            Ski::K => Ski::Kp(SkiContainer1::from(arg)),
            Ski::I => arg,
            Ski::Sp(sp) => Ski::Spp(SkiContainer2::from((sp.get(), arg))),
            Ski::Kp(kp) => kp.get(),
            Ski::Spp(spp) => {
                let arg_1 = arg.clone();
                let arg_2 = arg;

                let eval_first = spp.first().apply(arg_1);
                let eval_second = spp.second().apply(arg_2);
                eval_first.apply(eval_second)
            }
        }
    }
}

impl Eval for Ski {
    type Target = Ski;

    fn eval(&self) -> Self::Target {
        match self {
            Ski::S => Ski::S,
            Ski::K => Ski::K,
            Ski::I => Ski::I,
            Ski::Sp(_sp) => self.clone(),
            Ski::Kp(_kp) => self.clone(),
            Ski::Spp(_spp) => self.clone(),
        }
    }
}

impl Lambda for Ski {}

#[cfg(test)]
mod tests {}
