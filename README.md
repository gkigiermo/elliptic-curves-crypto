# Crypto Basics
The idea is to implement some of the building blocks of cryptography
# Finite Fields
A field is an algebraic structure that consists of a set of numbers (elements) with two defined operations (addition and multiplication) that satisfy the following properties:
 - Closure
 - Additive identity
 - Multiplicative identity
 - Additive inverse
 - Multiplicative inverse 

A finite field is a field with a finite number of elements and looks like:
F_p = {0, 1, 2, ..., p-1}
where p is called the order of the field and, in cryptography, it is a prime number.

Field elements must satisfy the abovementioned properties so we cannot use primitives.

# Elliptic curves
An elliptic curve is a set of points that satisfy an equation like this:
```math
y^2 = x^3 + ax + b
```
Elliptic curves are an essential part of blockchain, Bitcoin for instance utilizes a curve called *secp256k1* that looks like $y^2 = x^3 + 7$.

The point addition consists of performing an operation on two points of the curve and obtaining a third point in the curve. It is a nonlinear operation in which predicting the result of the addition is not trivial. Point addition satisfies certain properties such as Identity, Commutativity, Associativity, and Invertibility.

For two points A and B we can get a line that intersects the elliptic curve. Commonly, the line  will intersect three points in the curve. The result of the point addition comes from reflecting the third point over the x-axis. Special cases are considered when the line intersects only at one point, or at two points.
