use std::ops::{Add, Sub, Div, Mul, Rem};
use std::cmp::{PartialEq,PartialOrd,Ordering};
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

impl PartialEq for Number {

    fn eq(&self, num : &Number) -> bool{
        for i in self.value.iter().zip(num.value.iter()){
            let (a,b) = i;
            if a != b {return false};
        }
        return true;
    }

    fn ne(&self, num : &Number) -> bool{
        for i in self.value.iter().zip(num.value.iter()){
            let (a,b) = i;
            if a == b {return false};
        }
        return true;
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, num: &Number) -> Option<Ordering> {
        if self == num {
            Some(Ordering::Equal)
        } else if self < num {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }

    fn lt(&self, num : &Number) -> bool {
        for i in self.value.iter().rev().zip(num.value.iter().rev()){
            let (a,b) = i;
            if a < b {return true}
            if a > b {return false}
        }
        return false;
    }
    fn gt(&self, num : &Number) -> bool {
        for i in self.value.iter().rev().zip(num.value.iter().rev()){
            let (a,b) = i;
            if a < b {return false}
            if a > b {return true}
        }
        return false;
    }
    fn le(&self, num : &Number) -> bool {
        self < num || self == num
    }
    fn ge(&self, num : &Number) -> bool {
        self > num || self == num
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
impl fmt::UpperHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_value = String::with_capacity(self.value.len()*16);
        let mut fmt_string = String::with_capacity(self.value.len()*24);

        //convert the number into Hex
        for i in self.value.iter().rev() {
            string_value.push_str(&format!("{:X}",i));
        }

        //space out the numbers
        for (i,c) in string_value.chars().enumerate(){
            fmt_string.push(c);
            if i % 2 != 0 {
                fmt_string.push(' ');
            }
        }
        write!(f, "{}", fmt_string)
    }
}
