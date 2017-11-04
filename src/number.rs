use std::ops::{Add, Sub, Div, Mul, Rem, Neg};
use std::cmp::{PartialEq,PartialOrd,Ordering};
use std::fmt;
use std::u64;

/* ============================================================================================ */
/*     Struct                                                                                   */
/* ============================================================================================ */
pub struct Integer {
    value : Vec<u64>,   // a vector of 64bit ints that grows in size to hold larger numbers.
    negative: bool,     // determines whether the number is positive or not.
}

/* ============================================================================================ */
/*     Implementation                                                                           */
/* ============================================================================================ */
impl Integer {

    // generates a Integer from a unsigned 64bit integer.
    pub fn from_int(init : u64) -> Integer {
        Integer {
            value: vec![init],
            negative: false,
        }
    }

    // returns a new Integer that is the absolute value of the calling Integer.
    pub fn abs(&self) -> Integer {
        Integer {
            value: self.value.clone(),
            negative: false,
        }
    }

    // check to see if zero
    pub fn is_zero(&self) -> bool {
        if self.value.len() == 1 && self.value[0] == 0 {
            true
        } else {
            false
        }
    }
}

/* ============================================================================================ */
/*     Operatiors [+ - / %]                                                                     */
/* ============================================================================================ */

// Overloads the + operator for two numbers such that Integer + Integer = Integer.
// Returns a new Integer that is the summation of the two input numbers.
impl<'a,'b> Add<&'b Integer> for &'a Integer {
    type Output = Integer;

    fn add(self, num :&'b Integer) -> Integer {
        // Initialize the parameters for the new Integer
        let capacity = self.value.len().max(num.value.len()) + 1;
        let mut new_value = Vec::with_capacity(capacity);
        let mut carry = 0;
        let (min,max) = if &self.abs() < &num.abs() {(self,num)} else {(num,self)};

        // Loop through the two numbers and add or subtract depending on sign
        for i in min.value.iter().zip(max.value.iter()){
            let (a,b) = (i.0.clone(),i.1.clone());

            // If the sign is the same, add the two, else subtract the two
            if min.negative == max.negative {
                let (new_num,overflow1) = a.overflowing_add(b);
                let (new_num,overflow2) = new_num.overflowing_add(carry);
                carry = if overflow1 || overflow2 {1} else {0};
                new_value.push(new_num);
            } else {
                let (a,carry) = if b < (a + carry) {(18446744073709551615-a,1)} else {(a,0)};
                let new_num = b-a-carry;
                new_value.push(new_num);
            }
        }

        // If there is a carry after the loop then a new block of bits is needed to store the Int
        if carry > 0 && max.negative == min.negative {
            new_value.push(carry);
        }

        // Calculates the sign for the new Integer
        let sign = if new_value.len() == 1 && new_value[0] == 0 {
            false
        } else {
            max.negative
        };

        // Create and return the new Integer
        Integer {
            value: new_value,
            negative: sign,
        }
    }
}

// Overloads the unary - operator such that -Integer is the negation of the number.
// Returns a new Integer that is the negation of the input number.
impl<'a> Neg for &'a Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        if self.is_zero(){
            Integer::from_int(0)
        } else {
            Integer{
                value: self.value.clone(),
                negative: !self.negative,
            }
        }
    }
}

// Overloads the - operator such that Integer - Integer is the summation of the negation.
// Returns a new Integer that is the summation of the negation.
impl<'a,'b> Sub<&'b Integer> for &'a Integer {
    type Output = Integer;

    fn sub(self, num : &'b Integer) -> Integer {
        self + &(-num)
    }
}

/* ============================================================================================ */
/*     Operatiors and Ordering [== != > < >= <=]                                                */
/* ============================================================================================ */

// Implements Partial Equals [== !=] for type Integers
impl PartialEq for Integer {

    // Overloads the == operator and checks if two Integers are equal.
    // Returns true if the two Integers are equal to one another, false otherwise.
    fn eq(&self, num : &Integer) -> bool{
        if self.negative != num.negative {
            return false;
        }
        for i in self.value.iter().zip(num.value.iter()){
            let (a,b) = i;
            if a != b {return false};
        }
        return true;
    }

    // Overloads the != operator and checks if two Integers are equal.
    // Returns false if the two Integers are equal to one another, true otherwise.
    fn ne(&self, num : &Integer) -> bool{
        for i in self.value.iter().zip(num.value.iter()){
            if self.negative != num.negative {
                return true;
            }
            let (a,b) = i;
            if a == b {return false};
        }
        return true;
    }
}

// Implements Partial Ordering [< > <= >=] for type Integer
impl PartialOrd for Integer {

    // Comapres two Integers and returns an Ordering.
    fn partial_cmp(&self, num: &Integer) -> Option<Ordering> {
        if self == num {
            Some(Ordering::Equal)
        } else if self < num {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }

    // Compares two Integers and returns true if the other number is
    // Greater Than or Equal in value false otherwise.
    fn lt(&self, num : &Integer) -> bool {
        if num.negative && !self.negative {
            return false;
        } else if num.negative != self.negative {
            return true;
        }

        for i in self.value.iter().rev().zip(num.value.iter().rev()){
            let (a,b) = i;
            if a < b {return true}
            if a > b {return false}
        }
        return false;
    }

    // Compares two Integers and returns false if the other number is
    // Less Than or Equal in value true otherwise.
    fn gt(&self, num : &Integer) -> bool {
        if num.negative && !self.negative {
            return true;
        } else if num.negative != self.negative {
            return false;
        }
        for i in self.value.iter().rev().zip(num.value.iter().rev()){
            let (a,b) = i;
            if a < b {return false}
            if a > b {return true}
        }
        return false;
    }

    // Compares two Integers and returns true if the other number is
    // Less Than or Equal in value false otherwise.
    fn le(&self, num : &Integer) -> bool {
        self < num || self == num
    }

    // Compares two Integers and returns false if the other number is
    // Less Than or Equal in value true otherwise.
    fn ge(&self, num : &Integer) -> bool {
        self > num || self == num
    }
}
/*

impl<'a,'b> Mul<&'b Integer> for &'a Integer {
    type Output = Integer;

    fn mul(self, num :&'b Integer) -> Integer {
        Integer {num: self.num * num.num}
    }
}

impl<'a,'b> Div<&'b Integer> for &'a Integer {
    type Output = Integer;

    fn div(self, num :&'b Integer) -> Integer {
        Integer {num: self.num / num.num}
    }
}

impl<'a,'b> Rem<&'b Integer> for &'a Integer {
    type Output = Integer;

    fn rem(self, num :&'b Integer) -> Integer {
        Integer {num: self.num % num.num}
    }
}


impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl fmt::Binary for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:064b}", self.num)
    }
}
*/

/* ============================================================================================ */
/*      Formatters                                                                              */
/* ============================================================================================ */

// Defines the {:X} fmt and converts the two integers to Hexadecimal values
impl fmt::UpperHex for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Initialize the two strings that will be used to format the number
        let mut string_value = String::with_capacity(self.value.len()*16);
        let mut fmt_string = String::with_capacity(self.value.len()*24+1);

        // Convert the number into Hex : FFFFFFFFFFFFFFFF
        for i in self.value.iter().rev() {
            string_value.push_str(&format!("{:016X}",i));
        }

        // Add the '-' char if the number is a negative number : -FFFFFFFFFFFFFFFF
        if self.negative  {
            fmt_string.push('-');
        }

        // Space out the number to group it in twos : FF FF FF FF FF FF FF FF
        for (i,c) in string_value.chars().enumerate(){
            fmt_string.push(c);
            if i % 2 != 0 {
                fmt_string.push(' ');
            }
        }

        // Writes the formated string back
        write!(f, "{}", fmt_string)
    }
}

/* ============================================================================================ */
/*      Test cases                                                                              */
/* ============================================================================================ */


// Tests to ensure that the Partial Equality operators return the proper value
// If two Integer have the same value then == returns true, other wise != returns true
#[test]
fn integer_equality_test() {
    let zero = Integer::from_int(0);
    let ten1 = Integer::from_int(10);
    let ten2 = Integer::from_int(10);
    let neg_ten = -&ten1;
    let twenty = Integer::from_int(20);
    let neg_twenty = -&twenty;

    assert!(zero.is_zero(), format!("\nEvaluated zero as not being zero\nzero = {:X}\n", zero));
    assert!((-&zero).is_zero(), format!("\nEvaluated zero as not being zero\nzero = {:X}\n", (-&zero)));
    assert!(zero == zero, format!("\nEvaluated zero != zero, when they should be equal.\nzero = {:X}\nzero = {:X}\n", zero, zero));
    assert!(zero == -&zero, format!("\nEvaluated zero != -zero, when they should be equal.\nzero = {:X}\nzero = {:X}\n", zero, zero));
    assert!(zero == zero, format!("\nEvaluated zero != zero, when they should be equal.\nzero = {:X}\nzero = {:X}\n", zero, zero));
    assert!(ten1 == ten2, format!("\nEvaluated ten1 != ten2, when they should be equal.\nten1 = {:X}\nten2 = {:X}\n", ten1, ten2));
    assert!(zero != ten1, format!("\nEvaluated zero == ten1, when they should not be equal.\nzero = {:X}\nten1 = {:X}\n", zero, ten1));
    assert!(ten1 != neg_ten, format!("\nEvaluated ten1 == neg_ten, when they should not be equal.\nten1 = {:X}\nneg_ten = {:X}\n", ten1, neg_ten));
    assert!(neg_twenty != neg_ten, format!("\nEvaluated neg_twenty == neg_ten, when they should not be equal.\neg_twenty = {:X}\nneg_ten = {:X}\n", ten1, neg_ten));
}

// Tests to ensure that the Partial Ordering operators return the proper value
// If an Integer has a value less than another than the < operator should return true
// If an Integer has a value greater than another than the > operator should return true
// If an Integer has a value less than or equal to another than the <= operator should return true
// If an Integer has a value greather than or equal to another than the >= operator should return true
#[test]
fn integer_ordering_test() {
    let zero = Integer::from_int(0);
    let ten1 = Integer::from_int(10);
    let ten2 = Integer::from_int(10);
    let neg_ten = -&ten1;

    assert!(!(zero < zero), format!("\nEvaluated zero < zero, when they should be equal.\nzero = {:X}\nzero = {:X}\n", zero, zero));
    assert!(!(ten1 > ten1), format!("\nEvaluated ten1 > ten1, when they should be equal.\nten1 = {:X}\nten1 = {:X}\n", ten1, ten1));
    assert!(ten1 <= ten2, format!("\nEvaluated ten1 > ten1, when they should be equal.\nten1 = {:X}\nten1 = {:X}\n", ten1, ten2));
    assert!(zero < ten1, format!("\nEvaluated zero >= ten1, when it should be less than.\nzero = {:X}\nten1 = {:X}\n", zero, ten1));
    assert!(ten1 > zero, format!("\nEvaluated zero >= ten1, when it should be less than.\nzero = {:X}\nten1 = {:X}\n", zero, ten1));
    assert!(neg_ten < ten1, format!("\nEvaluated neg_ten >= ten1, when it should be less than.\nneg_ten = {:X}\nten1 = {:X}\n", neg_ten, ten1));
    assert!(!(neg_ten > ten1), format!("\nEvaluated neg_ten >= ten1, when it should be less than.\nneg_ten = {:X}\nten1 = {:X}\n", neg_ten, ten1));
}

#[test]
fn integer_add_test(){
    let zero = Integer::from_int(0);
    let one = Integer::from_int(1);
    //let max = Integer::from_int(u64::MAX);
    let ten = Integer::from_int(10);
    let neg_two = -&Integer::from_int(2);
    let neg_one = -&one;
    let mut ten_from_one = Integer::from_int(0);
    for i in 0..10 {
        ten_from_one = &ten_from_one + &one;
    }

    assert!(&one + &neg_one == zero, format!("\nEvaluated one + neg_one being != zero, when it should be zero\n"));
    assert!(&one - &one == zero, format!("\nEvaluated one - one being != zero, when it should be zero\n"));
    assert!(&neg_one + &neg_one == neg_two, format!("\nEvaluated neg_one + neg_one being != neg_two, when they should be equal\n"));
    assert!(&neg_one + &one == zero, format!("\nEvaluated neg_one + one being != zero, when it should be zero\n"));
    assert!(&neg_one + &one == zero, format!("\nEvaluated one + neg_one being != zero, when it should be zero\n"));
    assert!(&zero + &zero == zero, format!("\nEvaluated zero + zero being != zero, when it should be zero\n"));
    assert!(&ten == &ten_from_one, format!("\nEvaluated ten != to &ten_from_one when it should be equal\nten = {:X}\nten_from_one = {:X}",ten,ten_from_one))
}
