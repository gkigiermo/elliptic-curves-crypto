
use std::ops; // to overload the operators
use core::ops::Rem;
use core::ops::Mul;
use core::ops::Add;
use core::ops::Sub;

//Add the Pow trait temporarily
pub trait Pow {
    fn pow(self, rhs: Self) -> Self;
}

impl Pow for i32 {
    fn pow(self, rhs: Self) -> Self {
        i32::pow(self, rhs as u32)
    }
}

impl Pow for i64 {
    fn pow(self, rhs: Self) -> Self {
        i64::pow(self, rhs as u32)
    }
}

pub trait DivInFiniteField {
    fn div_in_finite_field(self, rhs: Self, prime: Self) -> Self;
}

impl DivInFiniteField for i32 {
    fn div_in_finite_field(self, rhs: Self, prime: Self) -> Self {      
        (self as i64 * i64::pow(rhs as i64, (prime - 2) as u32) % prime as i64 ) as i32
    }
}

impl DivInFiniteField for i64 {
    fn div_in_finite_field(self, rhs: Self, prime: Self) -> Self {      
        (self as i128 * i128::pow(rhs as i128, (prime - 2) as u32) % prime as i128 ) as i64
    }
}

//This function might overflow needs more treatment
impl Pow for f32 {
    fn pow(self, rhs: Self) -> Self {
        f32::powf(self, rhs)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FieldElement<T> {
    num: T,
    prime: T,
}
// A constructor for FieldElement
impl<T: std::fmt::Display + std::ops::Sub<i32> + std::cmp::PartialOrd<T> + std::cmp::PartialOrd<i32> > FieldElement<T> {
    pub fn new(num: T, prime: T) -> FieldElement<T> where <T as Sub<i32>>::Output: std::fmt::Display {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime-1);
        }
        FieldElement {
            num: num,
            prime: prime
        }
    }
}

//Overloading the operator +
impl<T: Copy + std::cmp::PartialEq  + Add<Output = T> + Rem<Output = T>  > ops::Add for FieldElement<T>   {
    type Output = FieldElement<T>;
    fn add(self, other: FieldElement<T>) -> FieldElement<T> where <T as Add>::Output: Rem<T> {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        FieldElement {
            num: (self.num + other.num) % self.prime,
            prime: self.prime
        }
    }
}

//Overloading the operator -
impl<T: Copy + std::cmp::PartialEq + Sub<Output = T> + Rem<Output = T> + Add<Output = T>  > ops::Sub for FieldElement<T>  {
    type Output = FieldElement<T>;
    fn sub(self, other: FieldElement<T>) -> FieldElement<T> where <T as Add>::Output: Rem<T> {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        FieldElement {
            num: ((self.num - other.num)% self.prime + self.prime) % self.prime,         
            prime: self.prime
        }
    }
}

//Overloading the operator *
impl<T: Copy + std::cmp::PartialEq + Mul<Output = T> + Rem<Output = T>> ops::Mul for FieldElement<T>  {
    type Output = FieldElement<T>;
    fn mul(self, other: FieldElement<T>) -> FieldElement<T> where <T as Mul>::Output: Rem<T>  {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        FieldElement {
            num: (self.num * other.num) % self.prime,
            prime: self.prime
        }
    }
}

//Overload the division operator
impl<T: Copy + Pow + std::cmp::PartialEq + Mul<Output = T> + Rem<Output = T>  + std::ops::Sub<i32, Output = T> + DivInFiniteField > ops::Div for FieldElement<T> {
    type Output = FieldElement<T>;
    fn div(self, other: FieldElement<T>) -> FieldElement<T> {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        let num = self.num.div_in_finite_field(other.num, self.prime);
        FieldElement {
            num: num,
            prime: self.prime
        }
    }
}

//Overloading the operator ==
impl<T: std::cmp::PartialEq> PartialEq for FieldElement<T> {
    fn eq(&self, other: &FieldElement<T>) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

//Overloading the operator the print the FieldElement
impl<T: std::fmt::Display> std::fmt::Display for FieldElement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

//Function that calculates the power of a FieldElement 
impl<T: Copy + Add<Output = T> + Rem<Output = T> + Sub<i32, Output = T> + Pow> FieldElement<T> {
    pub fn pow(&self, exponent: T) -> FieldElement<T> {
        let field_exponent = (exponent % (self.prime - 1) + self.prime) % self.prime;  
        let result = self.num.pow(field_exponent) % self.prime;
        FieldElement {
            num: result,
            prime: self.prime
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::FieldElement; 
    #[test]
    fn addition(){
        let field_element1 = FieldElement::new(2,7);
        let field_element2 = FieldElement::new(5,7);
        let sum_of_elements = field_element1 + field_element2;
        let expected_result = FieldElement::new(0,7);
        assert_eq!( sum_of_elements, expected_result);
    }

    #[test]
    fn subtraction(){
        let field_element1 = FieldElement::new(2,7);
        let field_element2 = FieldElement::new(5,7);
        let sum_of_elements = field_element1 - field_element2;
        let expected_result = FieldElement::new(4,7);
        assert_eq!( sum_of_elements, expected_result);
    }

    #[test]
    fn multiply(){
        let field_element1 = FieldElement::new(2,7);
        let field_element2 = FieldElement::new(5,7);
        let sum_of_elements = field_element1 * field_element2;
        let expected_result = FieldElement::new(3,7);
        assert_eq!( sum_of_elements, expected_result);
    }

    #[test]
    fn divide(){
        let field_element1 = FieldElement::new(2,7);
        let field_element2 = FieldElement::new(5,7);
        let sum_of_elements = field_element1 / field_element2;
        let expected_result = FieldElement::new(6,7);
        assert_eq!( sum_of_elements, expected_result);
    }

    #[test]
    fn pow(){
        let field_element1 = FieldElement::new(2,7);
        let sum_of_elements = field_element1.pow(4);
        let expected_result = FieldElement::new(2,7);
        assert_eq!( sum_of_elements, expected_result);
    } 
}

