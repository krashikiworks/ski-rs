use std::convert::TryFrom;
use std::env::args;

use ski_rs::sequence::Sequence;
use ski_rs::stack::Stax;

fn main() {
    let arg = args().nth(1).unwrap();
    let arg = arg.as_str();
    let mut stax = Stax::from(Sequence::try_from(arg).unwrap());
    let result = stax.eval().unwrap();
    println!("{}", Into::<String>::into(result));
}
