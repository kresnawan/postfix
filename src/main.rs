use crate::postfix::Postfix;

mod err;
mod postfix;
mod token;

fn main() {
    let pf = Postfix::new("(5 * (2 + 2)) ^ 2").unwrap();
    println!("{}", pf.as_string());
    println!("{}", pf.calculate());
}
