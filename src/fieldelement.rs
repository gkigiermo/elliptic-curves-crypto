use std::ops; // to overload the operators
#[derive(Debug)]
pub struct FieldElement {
    num: i64,
    prime: i64,
}
// A constructor for FieldElement
impl FieldElement{
    pub fn new(num: i64, prime: i64) -> FieldElement {
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
impl ops::Add for FieldElement {
    type Output = FieldElement;
    fn add(self, other: FieldElement) -> FieldElement {
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
impl ops::Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, other: FieldElement) -> FieldElement {
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
impl ops::Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: FieldElement) -> FieldElement {
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
impl ops::Div for FieldElement {
    type Output = FieldElement;
    fn div(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        let num = self.num * other.num.pow(self.prime as u32 - 2) % self.prime;
        FieldElement {
            num: num,
            prime: self.prime
        }
    }
}

//Overloading the operator ==
impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

//Overloading the operator the print the FieldElement
impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

//Function that calculates the power of a FieldElement 
impl FieldElement {
    pub fn pow(&self, exponent: i64) -> FieldElement {
        let field_exponent = (exponent % (self.prime - 1) + self.prime) % self.prime;
        let result = self.num.pow(field_exponent as u32) % self.prime;
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

