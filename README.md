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
