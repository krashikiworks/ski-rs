/// Unlambda-style SKI expression の関数適用記号
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Apply;

/// Unlambda-style SKI expression の原子コンビネータ記号
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Atom {
    S,
    K,
    I,
}

/// Unlambda-style SKI expression の構成子として有効な記号
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Token {
    Apply(Apply),
    Atom(Atom),
}

impl Token {
    pub fn a() -> Self {
        Token::Apply(Apply)
    }

    pub fn s() -> Self {
        Token::Atom(Atom::S)
    }

    pub fn k() -> Self {
        Token::Atom(Atom::K)
    }

    pub fn i() -> Self {
        Token::Atom(Atom::I)
    }
}
