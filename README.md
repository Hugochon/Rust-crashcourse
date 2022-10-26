## Course 1: generic types, trait and impl

* We want to compute the area of some basic 2D figure, and the volume of 3D solids
* We want to be able to create those geometric forms with parameters of different types (u32, f64, String...).
* We want to implement the same operation for all those different forms
* Some forms may be composed of other forms

## Course 2: declarative macros

* we have a macro `simple_vec` that takes 0 or more arguments of the same type and return a vector
* we want to add an arm to this macro to return a vector with `n` times values `x`