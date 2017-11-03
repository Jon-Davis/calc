mod number;
//mod number_test;
use number::*;

fn main() {
    let num1 = Number::new(10);
    let num2 = Number::new(20);
    let sub = &num1 - &num2;

    println!("{:X} > {:X} = {}",&num1,&sub,&num1 > &sub);
    println!("{:X} < {:X} = {}",&num1,&sub,&num1 < &sub);
    println!("{:X} > {:X} = {}",&num1,&num2,&num1 > &num2);
    println!("{:X} < {:X} = {}",&num1,&num2,&num1 < &num2);
}
