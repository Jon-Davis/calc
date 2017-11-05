/* ============================================================================================ */
/*     Description                                                                              */
/* ============================================================================================ */
// This Module implements arbitrary percision arithmetic in rust.
// The methodology used is each Integer creates a vector of u64 numbers.
// The numbers are stored from least significant 64bits to most significant 64bits.


/* ============================================================================================ */
/*     Modules                                                                                  */
/* ============================================================================================ */
use std::ops::{Add, Sub, Div, Mul, Rem, Neg};
use std::cmp::{PartialEq,PartialOrd,Ordering};
use std::fmt;
use std::{u64,i64};

/* ============================================================================================ */
/*     Struct                                                                                   */
/* ============================================================================================ */
pub struct Integer {
    value : Vec<u64>,   // a vector of 64bit ints that grows in size to hold larger numbers.
    negative: bool,     // determines whether the number is positive or not.
}

// Implementation
impl Integer {
/* ============================================================================================ */
/*     Constructors                                                                             */
/* ============================================================================================ */

    // generates a Integer from a unsigned 32bit integer.
    #[inline]
    pub fn from_u32(init : u32) -> Integer {
        Integer::from_u64(init as u64)
    } // End of from_u64 function

    // generate a Integer from a signed 32bit integer
    #[inline]
    pub fn from_i32(init : i32) -> Integer {
        Integer::from_i64(init as i64)
    }

    // generates a Integer from a unsigned 64bit integer.
    pub fn from_u64(init : u64) -> Integer {
        Integer {
            value: vec![init],
            negative: false,
        }
    } // End of from_u64 function

    // generate a Integer from a signed 64bit integer
    pub fn from_i64(init : i64) -> Integer {
        let negative =  if init < 0 {true} else {false};
        let init : u64 = init.abs() as u64;
        Integer {
            value: vec![init],
            negative: negative,
        }
    } // End of from_i64 function

/* ============================================================================================ */
/*     Methods                                                                                  */
/* ============================================================================================ */
    // returns a new Integer that is the absolute value of the calling Integer.
    pub fn abs(&self) -> Integer {
        Integer {
            value: self.value.clone(),
            negative: false,
        }
    } // End of abs function

    // returns a Ordering reguardless of sign
    pub fn abs_cmp(&self,other : &Integer) -> Option<Ordering> {
        if self.value.len() > other.value.len() {
            return Some(Ordering::Greater);
        } else if self.value.len() < other.value.len() {
            return Some(Ordering::Less);
        }

        for i in self.value.iter().rev().zip(other.value.iter().rev()){
            let (a,b) = i;
            if a < b {return Some(Ordering::Less)}
            if a > b {return Some(Ordering::Greater)}
        }
        return Some(Ordering::Equal);
    } // End of abs_cmp function

    // returns a new Integer that is the self ^ other
    pub fn pow(&self,other :&Integer) -> Integer {
        if (other.negative) {
            panic!("Integers are not rational numbers!");
        }

        //check special cases for quicker execution
        if other.is_zero() {
            return Integer::from_i32(1);
        } else if self.is_zero() {
            return Integer::from_i32(0);
        }

        // calculate the result
        let mut result = Integer::from_i32(1);
        let mut count = Integer::from_i32(0);
        let one = Integer::from_u64(1u64);
        while &count < other {
            result = &result * &self;
            count = &count + &one;
        }
        result
    }

    // check to see if zero
    pub fn is_zero(&self) -> bool {
        if self.value.len() == 1 && self.value[0] == 0 {
            true
        } else {
            false
        }
    } // End of is_zero function
} // End of Integer Implementation

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
        // Sort the two Integers by absolute value
        let (min,max) = match self.abs_cmp(num) {
            Some(Ordering::Less) => (self,num),
            _ => (num,self)
        };
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
                let (new_num,c) = if b < a || b < (a + carry) {
                    ((u64::MAX-a)+(b+1),1)
                } else {
                    (b-a-carry,0)
                };
                carry = c;
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
    } // End of add function
} // End of Add Implementation

// Overloads the unary - operator such that -Integer is the negation of the number.
// Returns a new Integer that is the negation of the input numbers.
impl<'a> Neg for &'a Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        if self.is_zero(){
            Integer::from_u64(0)
        } else {
            Integer{
                value: self.value.clone(),
                negative: !self.negative,
            }
        }
    } // End of neg function
} // End of Neg implementation

// Overloads the - operator such that Integer - Integer is the summation of the negation.
// Returns a new Integer that is the summation of the negation.
impl<'a,'b> Sub<&'b Integer> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn sub(self, num : &'b Integer) -> Integer {
        self + &(-num)
    } // End of sub function
} // End of Sub implementation

// Overloads the * operator such that Intger - Integer is the product of the two numbers
// Returns a new Integer that is the product of the input numbers.
impl<'a,'b> Mul<&'b Integer> for &'a Integer {
    type Output = Integer;

    fn mul(self, num :&'b Integer) -> Integer {
        let (min,max) = match self.abs_cmp(num) {
            Some(Ordering::Less) => (self,num),
            _ => (num,self)
        };
        let mut new_value = Vec::with_capacity(self.value.len()+num.value.len());
        let mut carry : u64 = 0;
        for min_i in min.value.iter() {
            // break the 64 bit number into two 32 bit numbers
            let (min_low_32, min_high_32) = (min_i & 0xFFFFFFFF, min_i >> 32);
            let mut init_index = 0;
            // run the broken up numbers seperately so that 32bit * 32bit numbers produce 64bit
            for i in 0..2 {
                //determine which number will run in this loop
                let min_num = if i == 0 {min_low_32} else {min_high_32};
                let mut index = init_index + i;
                for max_i in max.value.iter() {
                    // break the 64 bit number into two 32 bit numbers
                    let (max_low_32, max_high_32) = (max_i & 0xFFFFFFFF, max_i >> 32);
                    // multiply out the two sections
                    let low_multiplied = min_num * max_low_32;
                    let high_multiplied = min_num * max_high_32;
                    // prepare the high_multiplied value for summation
                    let high_value = (high_multiplied << 32) + carry;
                    // perform the addition and check for overflow, add the overflow to the carry
                    let (sum,overflow) = low_multiplied.overflowing_add(high_value);
                    carry = (high_multiplied >> 32) + if overflow {1u64} else {0u64};
                    // sum the newly calculated value on the vector
                    if new_value.len() < index + 1 {
                        // if the vector is empty, push the number onto the vector
                        new_value.push(sum);
                    } else {
                        // if the vector is !empty, sum the calculated value with the old value
                        let (sum,overflow) = sum.overflowing_add(new_value[index]);
                        carry = carry + if overflow {1u64} else {0u64};
                        new_value[index] = sum;
                    }
                    // Iterate through the index so the min_high_32 knows where to begin
                    index = index + 1;
                }
                // Iterate the init_index such that future iterations know where to begin
                init_index = init_index + 2;
            }
        }
        // Calculates the sign for the new Integer
        let mut is_zero = false;
        if new_value.len() == 2 && new_value[0] == 0u64 && new_value[1] == 0u64{
            is_zero = true;
            new_value = vec!(0);
        }
        let sign = if is_zero {false} else {min.negative != max.negative};
        Integer {
            value: new_value,
            negative: sign,
        }
    } // End of mul function
} // End of Mul implementation

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
    } // End of eq function

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
    } // End of ne function
} // End of PartialEq implementation

// Implements Partial Ordering [< > <= >=] for type Integer
impl PartialOrd for Integer {

    // Comapres two Integers and returns an Ordering.
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.negative && !self.negative {
            Some(Ordering::Greater)
        } else if other.negative != self.negative {
            Some(Ordering::Less)
        } else {
            self.abs_cmp(other)
        }
    } // End of partial_cmp function

    // Compares two Integers and returns true if the other number is
    // Greater Than or Equal in value false otherwise.
    fn lt(&self, other : &Integer) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => true,
            _ => false,
        }
    } // End of lt function

    // Compares two Integers and returns false if the other number is
    // Less Than or Equal in value true otherwise.
    fn gt(&self, other : &Integer) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            _ => false,
        }
    } // End of gt function

    // Compares two Integers and returns true if the other number is
    // Less Than or Equal in value false otherwise.
    fn le(&self, other : &Integer) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => true,
            Some(Ordering::Equal) => true,
            _ => false,
        }
    } // End of le function

    // Compares two Integers and returns false if the other number is
    // Less Than or Equal in value true otherwise.
    fn ge(&self, other : &Integer) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => true,
            _ => false,
        }
    } // End of ge function
} // End of PartialOrd implementation

/*
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
    } // End of fmt function
} // End of fmt::UpperHex implementation

/* ============================================================================================ */
/*      Test cases                                                                              */
/* ============================================================================================ */


// Tests to ensure that the Partial Equality operators return the proper value
// If two Integer have the same value then == returns true, other wise != returns true
#[test]
fn integer_equality_test() {
    let zero = Integer::from_u64(0);
    let ten1 = Integer::from_u64(10);
    let ten2 = Integer::from_u64(10);
    let neg_ten = -&ten1;
    let twenty = Integer::from_u64(20);
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
} // End of integer_equality_test

// Tests to ensure that the Partial Ordering operators return the proper value
// If an Integer has a value less than another than the < operator should return true
// If an Integer has a value greater than another than the > operator should return true
// If an Integer has a value less than or equal to another than the <= operator should return true
// If an Integer has a value greather than or equal to another than the >= operator should return true
#[test]
fn integer_ordering_test() {
    let zero = Integer::from_u64(0);
    let ten1 = Integer::from_u64(10);
    let ten2 = Integer::from_u64(10);
    let neg_ten = -&ten1;
    let max = Integer::from_u64(u64::MAX);
    let one = Integer::from_u64(1);
    let max_one = &max+&one;

    assert!(!(zero < zero), format!("\nEvaluated zero < zero, when they should be equal.\nzero = {:X}\nzero = {:X}\n", zero, zero));
    assert!(!(ten1 > ten1), format!("\nEvaluated ten1 > ten1, when they should be equal.\nten1 = {:X}\nten1 = {:X}\n", ten1, ten1));
    assert!(ten1 <= ten2, format!("\nEvaluated ten1 > ten1, when they should be equal.\nten1 = {:X}\nten1 = {:X}\n", ten1, ten2));
    assert!(zero < ten1, format!("\nEvaluated zero >= ten1, when it should be less than.\nzero = {:X}\nten1 = {:X}\n", zero, ten1));
    assert!(ten1 > zero, format!("\nEvaluated zero >= ten1, when it should be less than.\nzero = {:X}\nten1 = {:X}\n", zero, ten1));
    assert!(neg_ten < ten1, format!("\nEvaluated neg_ten >= ten1, when it should be less than.\nneg_ten = {:X}\nten1 = {:X}\n", neg_ten, ten1));
    assert!(!(neg_ten > ten1), format!("\nEvaluated neg_ten >= ten1, when it should be less than.\nneg_ten = {:X}\nten1 = {:X}\n", neg_ten, ten1));
    assert!(&max_one > &max, format!("\nEvaluated max_one <= max, when it should be greater than than.\nmax_one = {:X}\nmax = {:X}\n", max_one, max));
    assert!(&max < &max_one, format!("\nEvaluated max_one <= max, when it should be greater than than.\nmax_one = {:X}\nmax = {:X}\n", max_one, max));
} // End of integer_ordering_test

// Tests to ensure that addition of Integers is working properly
// Tests to ensure a positive Integer and a positive Integer add together properly
// Tests to ensure a positive Integer and a negative Integer add together properly
// Tests to ensure a negative Integer and a negative Integer add together properly
// Tests to ensure that whenever the sum equals zero, that zero is non-negative
// Tests when overflow of the initial 64bit number occurs that the Integer carries over properly
// Tests when underflow of the Integer occurs that the Integer borrows properly
#[test]
fn integer_add_test(){
    let zero = Integer::from_u64(0);
    let one = Integer::from_u64(1);
    let max = Integer::from_u64(u64::MAX);
    let max_one = &max + &one;
    let two_max = &max + &max;
    let ten = Integer::from_u64(10);
    let nine = Integer::from_u64(9);
    let neg_two = -&Integer::from_u64(2);
    let neg_one = -&one;
    let mut ten_from_one = Integer::from_u64(0);
    for _ in 0..10 {
        ten_from_one = &ten_from_one + &one;
    }

    assert!(&one + &neg_one == zero, format!("\nEvaluated one + neg_one being != zero, when it should be zero\n"));
    assert!(&one - &one == zero, format!("\nEvaluated one - one being != zero, when it should be zero\n"));
    assert!(&neg_one + &neg_one == neg_two, format!("\nEvaluated neg_one + neg_one being != neg_two, when they should be equal\n"));
    assert!(&neg_one + &one == zero, format!("\nEvaluated neg_one + one being != zero, when it should be zero\n"));
    assert!(&ten - &nine == one, format!("\nEvaluated ten - nine to being != one, when it should be one\n"));
    assert!(&ten - &one == nine, format!("\nEvaluated ten - one to being != nine, when it should be nine\n"));
    assert!(&neg_one + &one == zero, format!("\nEvaluated one + neg_one being != zero, when it should be zero\n"));
    assert!(&zero + &zero == zero, format!("\nEvaluated zero + zero being != zero, when it should be zero\n"));
    assert!(&ten == &ten_from_one, format!("\nEvaluated ten != to &ten_from_one when it should be equal\nten = {:X}\nten_from_one = {:X}",ten,ten_from_one));
    assert!(&max_one - &max == one, format!("\nEvaluated max_one - max to be != one\nmax_one = {:X}\nmax = {:X}",max_one,max));
    assert!(&max_one - &max_one == zero, format!("\nEvaluated max_one - max_one to be != zero\nmax_one = {:X}\nmax_one = {:X}",max_one,max_one));
    assert!(&two_max - &max == max, format!("\nEvaluated two_max - max != max, when it should be max\ntwo_max = {:X}\nmax = {:X}\n",two_max,max));
} // End of integer_add_test

// Tests to ensure that small values are added together properly
#[test]
fn integer_loop_add_test() {
    for i in -10i64..10i64 {
        for j in -10i64..10i64 {
            let answer = Integer::from_i64(i+j);
            let i_num = Integer::from_i64(i);
            let j_num = Integer::from_i64(j);
            let result = &i_num + &j_num;
            assert!(answer == result, format!("\nEvaluated {} + {} != {}",i,j,i+j));
        }
    }
}

// Test to ensure that the Multiplication of an Integer and Zero is Zero
// Test to ensure that the Multiplication of an Integer and an Integer is the product
// Test to ensure that the Multiplication of an Integer and a negative Integer is negative
// Test to ensure that the Multiplication of two negative Integers are positive
// Test to ensure that when u64 numbers overflow, the overflow is handled properly
#[test]
fn interger_mul_test(){
    let zero = Integer::from_u64(0);
    let two = Integer::from_u64(2);
    let ten = Integer::from_u64(10);
    let neg_ten = -&ten;
    let hundred = Integer::from_u64(100);
    let neg_hundred = -&hundred;
    let max = Integer::from_u64(u64::MAX);
    let max_ten = &(&(&(&max + &max) + &(&max + &max)) + &(&(&max + &max) + &(&max + &max))) + &(&max + &max);

    assert!(&ten * &ten == hundred, format!("\nEvaluated ten * ten != hundred\nresult = {:X}\n",&ten * &ten));
    assert!(&zero * &ten == zero, format!("\nEvaluated zero * ten != zero\nresult = {:X}\n",&ten * &ten));
    assert!(&ten * &neg_ten == neg_hundred, format!("\nEvaluated neg_ten * ten != neg_hundred\nresult = {:X}\n",&ten * &neg_ten));
    assert!(&neg_ten * &neg_ten == hundred, format!("\nEvaluated neg_ten * neg_ten != hundred\nresult = {:X}\n",&neg_ten * &neg_ten));
    assert!(&max * &two == &max + &max, format!("\nEvaluated max * two != max + max\nproduct = {:X}\nsum = {:X}\n",&max * &two,&max + &max));
    assert!(&max * &ten == max_ten, format!("\nEvaluated max * ten != max_ten\nproduct = {:X}\nsum = {:X}\n",&max * &ten,max_ten));
} // End of interger_mul_test

// Tests to ensure that small values are multiplied together properly
#[test]
fn integer_loop_mul_test() {
    for i in -10i64..10i64 {
        for j in -10i64..10i64 {
            let answer = Integer::from_i64(i*j);
            let i_num = Integer::from_i64(i);
            let j_num = Integer::from_i64(j);
            let result = &i_num * &j_num;
            assert!(answer == result, format!("\nEvaluated {} * {} != {}",i,j,i*j));
        }
    }
}

// Tests to ensure that the pow function is working properly
#[test]
fn integer_loop_pow_test() {
    for i in 0u32..10u32 {
        for j in 0u32..10u32 {
            let answer = Integer::from_u32(i.pow(j));
            let i_num = Integer::from_u32(i);
            let j_num = Integer::from_u32(j);
            let result = i_num.pow(&j_num);
            assert!(answer == result, format!("\nEvaluated {} ^ {} != {}, was {:X}",i,j,i.pow(j),result));
        }
    }
}
