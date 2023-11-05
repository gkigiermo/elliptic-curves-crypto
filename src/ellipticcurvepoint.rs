use std::ops; // to overload the operators

//Elliptic curves have the form y^2 = x^3 + ax + b
//first draft considers only integer64 numbers

#[derive(Debug, Clone, Copy)]
pub struct EllipticCurvePoint {
    a: i64,
    b: i64,
    x: i64,
    y: i64
}

// A constructor for the EllipticCurvePoint
impl EllipticCurvePoint{
    pub fn new(x: i64, y: i64, a: i64, b: i64) -> EllipticCurvePoint {
        if y*y != x*x*x + a*x + b && !(x == 0 && y == 0) { //check if the point is in the curve, point 0,0 is considered infinity
            panic!("Point ({},{}) is not in the curve y^2 = x^3 + {}x + {}", x, y, a, b);
        }
        EllipticCurvePoint {
            x: x,
            y: y,
            a: a,
            b: b
        }
    }
}

//Overloading the operator ==
impl PartialEq for EllipticCurvePoint {
    fn eq(&self, other: &EllipticCurvePoint) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
    fn ne(&self, other: &EllipticCurvePoint) -> bool {
        self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b
    }
}

//Overloading the operator the print the EllipticCurvePoint
impl std::fmt::Display for EllipticCurvePoint {
    fn fmt(&self, c: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.x == 0 && self.y == 0 {
            return write!(c, "Point at infinity in the elliptic curve y^2 = x^3 + {}x + {} ", self.a, self.b);
        }
        write!(c, "Point ({},{}) in the elliptic curve y^2 = x^3 + {}x + {}", self.x, self.y, self.a, self.b)
    }
}

//Overloading the operator + with point addition
impl ops::Add for EllipticCurvePoint {
    type Output = EllipticCurvePoint;
    fn add(self, other: EllipticCurvePoint) -> EllipticCurvePoint {
        if self.a != other.a || self.b != other.b {
            panic!("Points {}, {} are not in the same elliptic curve", self, other);
        }
        if self.x == 0 && self.y == 0 {
            return other;
        }
        if other.x == 0 && other.y == 0 {
            return self;
        }
        if self.x == other.x && self.y != other.y {
            return EllipticCurvePoint::new(0, 0, self.a, self.b);
        }
        if self.x != other.x {
            let s = (other.y - self.y) / (other.x - self.x);
            let x = s*s - self.x - other.x;
            let y = s*(self.x - x) - self.y;
            return EllipticCurvePoint::new(x, y, self.a, self.b);
        }
        if self == other && self.y == 0 * self.x {
            return EllipticCurvePoint::new(0, 0, self.a, self.b);
        }
        if self == other {
            let s = (3*self.x*self.x + self.a) / (2*self.y);
            let x = s*s - 2*self.x;
            let y = s*(self.x - x) - self.y;
            return EllipticCurvePoint::new(x, y, self.a, self.b);
        }
        return EllipticCurvePoint::new(0, 0, self.a, self.b);
    }
}

#[cfg(test)]
mod tests{
    use crate::EllipticCurvePoint;
    #[test]
    fn addition1(){
        let point1 = EllipticCurvePoint::new(0, 0, 5, 7);
        let point2 = EllipticCurvePoint::new(2, 5, 5, 7);
        let sum_of_points = point1 + point2;
        let expected_result = EllipticCurvePoint::new(2, 5, 5, 7);
        assert_eq!( sum_of_points, expected_result);
    }
    #[test]
    fn addition2(){
        let point1 = EllipticCurvePoint::new(0, 0, 5, 7);
        let point2 = EllipticCurvePoint::new(2, 5, 5, 7);
        let sum_of_points = point2 + point1;
        let expected_result = EllipticCurvePoint::new(2, 5, 5, 7);
        assert_eq!( sum_of_points, expected_result);
    }
    #[test]
    fn addition3(){
        let point1 = EllipticCurvePoint::new(2, -5, 5, 7);
        let point2 = EllipticCurvePoint::new(2, 5, 5, 7);
        let sum_of_points = point2 + point1;
        let expected_result = EllipticCurvePoint::new(0, 0, 5, 7);
        assert_eq!( sum_of_points, expected_result);
    }
    #[test]
    fn addition4(){
        let point1 = EllipticCurvePoint::new(3, 7, 5, 7);
        let point2 = EllipticCurvePoint::new(-1, -1, 5, 7);
        let sum_of_points = point2 + point1;
        let expected_result = EllipticCurvePoint::new(2, -5, 5, 7);
        assert_eq!( sum_of_points, expected_result);
    }
    #[test]
    fn addition5(){
        let point1 = EllipticCurvePoint::new(-1, -1, 5, 7);
        let point2 = EllipticCurvePoint::new(-1, -1, 5, 7);
        let sum_of_points = point2 + point1;
        let expected_result = EllipticCurvePoint::new(18, 77, 5, 7);
        assert_eq!( sum_of_points, expected_result);
    }
    #[test]
    fn not_equal(){
        let point1 = EllipticCurvePoint::new(3, -7, 5, 7);
        let point2 = EllipticCurvePoint::new(18, 77, 5, 7);
        assert!( point1 != point2);
    }
    #[test]
    fn equal(){
        let point1 = EllipticCurvePoint::new(3, -7, 5, 7);
        let point2 = EllipticCurvePoint::new(3, -7, 5, 7);
        assert!( point1 == point2);
    }

}
