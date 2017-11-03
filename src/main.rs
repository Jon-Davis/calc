mod number;
use number::*;

fn main() {
    let num1 = Number::new(1);
    let num2 = Number::new(9223372036854775807);

    println!("{:x}", &num1 + &num2);
}
