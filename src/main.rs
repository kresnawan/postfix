use postfix::Postfix;

fn main() {
    let pf = Postfix::new("10 ^ 2").unwrap();
    println!("{}", pf.as_string());
    println!("{}", pf.calculate());
}
