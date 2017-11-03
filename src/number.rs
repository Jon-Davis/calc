use std::ops::{Add, Sub, Div, Mul, Rem};
use std::fmt;

pub struct Number {
    value : Vec<u64>,
    negative : bool,
}

impl Number {
    pub fn new(init : u64) -> Number {
        Number {
            value: vec![init],
            negative: false,
        }
    }
}


impl<'a,'b> Add<&'b Number> for &'a Number {
    type Output = Number;

    fn add(self, num :&'b Number) -> Number {
        let capacity = self.value.len().max(num.value.len()) + 1;
        let mut new_value = Vec::with_capacity(capacity);
        let mut carry = 0;
        let mut negative = false;
        for i in self.value.iter().zip(num.value.iter()){
            let (a,b) = i;
            if self.negative == num.negative {
                let new_num = a+b+carry;
                carry = if &new_num < a || &new_num < b {1} else {0};
                new_value.push(new_num);
                negative = self.negative;
            } else {
                //TODO implement subtraction
            }
        }

        Number {
            value: new_value,
            negative: negative,
        }
    }

}


/*
impl<'a,'b> Sub<&'b Number> for &'a Number {
    type Output = Number;

    fn sub(self, num :&'b Number) -> Number {
        Number {num: self.num - num.num}
    }
}

impl<'a,'b> Mul<&'b Number> for &'a Number {
    type Output = Number;

    fn mul(self, num :&'b Number) -> Number {
        Number {num: self.num * num.num}
    }
}

impl<'a,'b> Div<&'b Number> for &'a Number {
    type Output = Number;

    fn div(self, num :&'b Number) -> Number {
        Number {num: self.num / num.num}
    }
}

impl<'a,'b> Rem<&'b Number> for &'a Number {
    type Output = Number;

    fn rem(self, num :&'b Number) -> Number {
        Number {num: self.num % num.num}
    }
}


impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:064b}", self.num)
    }
}
*/
impl fmt::LowerHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_value = String::with_capacity(self.value.len()*16);
        for i in self.value.iter().rev() {
            string_value.push_str(&format!("{:016x}",i));
        }
        write!(f, "{}", string_value)
    }
}
