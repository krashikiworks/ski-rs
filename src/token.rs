/// Unlambda-style SKI expression の原子コンビネータ記号
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Atom {
    S,
    K,
    I,
}

impl Into<String> for Atom {
    fn into(self) -> String {
        match self {
            Self::S => String::from("s"),
            Self::K => String::from("k"),
            Self::I => String::from("i"),
        }
    }
}

/// Unlambda-style SKI expression の構成子として有効な記号
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Token {
    Apply,
    Atom(Atom),
}

impl From<Atom> for Token {
    fn from(atom: Atom) -> Self {
        Token::Atom(atom)
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Self::Apply => String::from("`"),
            Self::Atom(atom) => atom.into(),
        }
    }
}

impl Token {
    pub fn a() -> Self {
        Token::Apply
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

// rule 3. 例外: 同等の関数に比べて、 呼び出し側が楽になるならば、マクロを書いても構わない

#[allow(unused_macros)]
macro_rules! a {
    () => {
        Token::Apply
    };
}

#[allow(unused_macros)]
macro_rules! s {
    () => {
        Token::Atom(Atom::S)
    };
}

#[allow(unused_macros)]
macro_rules! k {
    () => {
        Token::Atom(Atom::K)
    };
}

#[allow(unused_macros)]
macro_rules! i {
    () => {
        Token::Atom(Atom::I)
    };
}

mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn constractor() {
        assert_eq!(Token::a(), Token::Apply);
        assert_eq!(Token::s(), Token::Atom(Atom::S));
        assert_eq!(Token::k(), Token::Atom(Atom::K));
        assert_eq!(Token::i(), Token::Atom(Atom::I));
    }

    #[test]
    fn macro_rules() {
        assert_eq!(a!(), Token::Apply);
        assert_eq!(s!(), Token::Atom(Atom::S));
        assert_eq!(k!(), Token::Atom(Atom::K));
        assert_eq!(i!(), Token::Atom(Atom::I));
    }
}
