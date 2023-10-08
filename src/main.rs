use std::ops; // to overload the operators

struct FieldElement {
    num: i64,
    prime: i64,
}
// A constructor for FieldElement
impl FieldElement{
    fn new(num: i64, prime: i64) -> FieldElement {
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
    fn pow(&self, exponent: i64) -> FieldElement {
        let field_exponent = (exponent % (self.prime - 1) + self.prime) % self.prime;
        let result = self.num.pow(field_exponent as u32) % self.prime;
        FieldElement {
            num: result,
            prime: self.prime
        }
    }
}

fn main() {
    let p1 = FieldElement { num: 2, prime: 7 };
    let p2 = FieldElement { num: 2, prime: 7 };
    let p3 = FieldElement { num: 15, prime: 7 };
    
    //let p4 = p2 + p3;
   // println!("p4.num = {}, p4.prime = {}", p4.num, p4.prime);

    // create an instance of FieldElement with num=1 and prime=0
    let p0 = FieldElement::new(6, 7);
    println!("p2 - p3 = {} ",p2 - p3);
    println!("p0 * p1 = {} ",p1 * p0);
    
    let p5 = FieldElement::new(5, 7);
    println!("p5 * * 3 = {} ",p5.pow(-3));
    

    let p6 = FieldElement::new(7, 19);
    let p7 = FieldElement::new(5, 19);
    println!("p6 / p7 = {} ",p6/p7);

    //println!("check if p4==p0 returns {}, p4 {}, p0 {}", p4 == p0, p4, p0);  
    
    //let p5 = p1 + p4;
    //println!("p1.num = {}, p2.num = {}", p1.num, p2.num);
    //println!("p1.prime = {}, p2.prime = {}", p1.prime, p2.prime);
    
   // let p3 = p1 + p2;
   // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

