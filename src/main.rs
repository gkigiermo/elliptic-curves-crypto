mod fieldelement;
mod ellipticcurvepoint;

use crate::fieldelement::FieldElement;
use crate::ellipticcurvepoint::EllipticCurvePoint;

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

    let p1 = EllipticCurvePoint::new(-1, -1, 5, 7);
    let p2 = EllipticCurvePoint::new(-1, -1, 5, 7);
    println!("p1 = {}", p1);
    println!("p2 = {}", p2);
    println!("p1 + p2 = {} \n", p1 + p2);

    let p3 = EllipticCurvePoint::new(-1, -1, 5, 7);
    let p4 = EllipticCurvePoint::new(2, 5, 5, 7);
    println!("p3 = {}", p3);
    println!("p4 = {}", p4);
    println!("p3 + p4 = {} \n", p3 + p4);

    let inf = EllipticCurvePoint::new(0, 0, 5, 7);
    let p5 = EllipticCurvePoint::new(2, 5, 5, 7);
    println!("inf = {}", inf);
    println!("p5 = {}", p5);
    println!("inf + p5 = {} \n", inf + p5);

    println!{"For additional testing use 'cargo test'"};
}

