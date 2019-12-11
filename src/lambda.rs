pub trait Apply {
    type Argument: Lambda;
    type Target: Lambda;

    fn apply(&self, arg: Self::Argument) -> Self::Target;
}

pub trait Eval {
    type Target: Lambda;

    fn eval(&self) -> Self::Target;
}

pub trait Lambda: Apply + Eval {}
