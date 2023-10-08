use std::ops; // to overload the operators

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
            panic!("Cannot substract two numbers in different Fields");
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
