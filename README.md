# large
large fixed-size const-generic numeric types for rust

## why not just use the `num_*` crates?
`num_bigint` and `num_rational` provide arbitrary precision, while this library only gives ou **fixed** precision without allocating anything to the heap.  
This can be usefull for embedded environments where you can't allocate, or if you precisely know the bounds of your numbers.  

## `Uint<S>`  
The type `Uint<S>` represents an unsigned integer with `S` digits of the base 2³².  
It is represented as `[u32; S]` internally.

## `Decimal<S>`
The type `Rational<S>`represents a rational number, represented as `(+-) Uint<S> / Uint<S>`.  

## todo
- [ ] implement `Decimal`, representing numbers as `a * b^c` where `b` is either 2 or 10 (?)
- [ ] make it fast