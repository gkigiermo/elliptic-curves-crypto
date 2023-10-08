mod fieldelement;

use crate::fieldelement::FieldElement;

fn main() {
    //Simple operations displayed
    let p0 = FieldElement::new(6,7);
    let p1 = FieldElement::new(2,7);
    println!("p0 = {}, p1 = {}", p0 , p1);
    println!("p0 * p1  = {} \n", p0 * p1);

    let p2 = FieldElement::new(2,7);
    let p3 = FieldElement::new(5,7);

    println!("p2 = {}, p3 = {}", p2 , p3);
    println!("p2 - p3  = {} \n", p2 - p3);
    
    
    let p5 = FieldElement::new(5, 7);
    println!("p5 = {}", p5);
    println!("p5 * * 3 = {} \n", p5.pow(-3));
    

    let p6 = FieldElement::new(7, 19);
    let p7 = FieldElement::new(5, 19);
    println!("p6 = {}, p7 = {}", p6 , p7);
    println!("p6 / p7  = {} \n", p6 / p7);

    println!{"For additional testing use 'cargo test'"};
}

