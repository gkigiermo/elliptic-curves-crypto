mod fieldelement;

use crate::fieldelement::FieldElement;

fn main() {
    let p1 = FieldElement::new(2,7);
    let p2 = FieldElement::new(2,7);
    let p3 = FieldElement::new(5,7);
    
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

