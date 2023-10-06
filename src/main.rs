use std::ops; // to overload the operators

struct FieldElement {
    num: i32,
    prime: i32,
}
// A constructor for FieldElement
impl FieldElement{
    fn new(num: i32, prime: i32) -> FieldElement {
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

fn main() {
    let p1 = FieldElement { num: 1, prime: 3 };
    let p2 = FieldElement { num: 2, prime: 7 };
    let p3 = FieldElement { num: 8, prime: 7 };
    
    let p4 = p2 + p3;
    println!("p4.num = {}, p4.prime = {}", p4.num, p4.prime);

    // create an instance of FieldElement with num=1 and prime=0
    let p0 = FieldElement::new(1, 3);
    println!("check if p4==p0 returns {}, p4 {}, p0 {}", p4 == p0, p4, p0);  
    
    //let p5 = p1 + p4;
    //println!("p1.num = {}, p2.num = {}", p1.num, p2.num);
    //println!("p1.prime = {}, p2.prime = {}", p1.prime, p2.prime);
    
   // let p3 = p1 + p2;
   // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

