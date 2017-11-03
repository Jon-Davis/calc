mod number;
use number::*;

fn main() {
    let num1 = Number::new(11);
    let num2 = Number::new(9223372036854775807);
    let num3 = Number::new(9223372036854775807);
    let num4 = &num1 + &num2;

    println!("{:X} + {:X} =  {:X}",&num1,&num2,&num4);
    println!("{:X} == {:X} => {}",&num1,&num2,&num1 == &num2);
    println!("{:X} != {:X} => {}",&num1,&num2,&num1 != &num2);
    println!("{:X} == {:X} => {}",&num2,&num3,&num2 == &num3);
    println!("{:X} != {:X} => {}",&num2,&num3,&num2 != &num3);
    println!("{:X} < {:X} => {}",&num1,&num2,&num1 < &num2);
    println!("{:X} > {:X} => {}",&num1,&num2,&num1 > &num2);
    println!("{:X} < {:X} => {}",&num2,&num3,&num2 < &num4);
    println!("{:X} <= {:X} => {}",&num2,&num3,&num2 <= &num4);
    println!("{:X} > {:X} => {}",&num2,&num3,&num2 > &num4);
    println!("{:X} >= {:X} => {}",&num2,&num3,&num2 >= &num4);
    println!("{:X} >= {:X} => {}",&num1,&num1,&num1 >= &num1);
    println!("{:X} <= {:X} => {}",&num1,&num1,&num1 <= &num1);
    println!("{:X} > {:X} => {}",&num1,&num1,&num1 > &num1);
    println!("{:X} < {:X} => {}",&num1,&num1,&num1 < &num1);
}
