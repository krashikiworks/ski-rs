/// Unlambda-style SKI expression の構成子として有効な記号

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Token {
    Apply,
    S,
    K,
    I,
}
