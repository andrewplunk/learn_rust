# Variable Bindings, Constants & Statics

## Mutabality

* Immutable by default
* variable bindings require `mut` keyword to make mutable

## Type inference
* At comple time the data type is checked
* Only have to declare type for constants and statics
* These types come after a colon `:`

```rust
let a = true;
let b: bool = true;

let (x, y) = (1, 2);

let mut z = 5;
z = 6;
```

## Constants

```rust
const N: i32 = 5;
```

## Statics
```rust
static N: i32 = 5;
```

The `let` keyword is used in `binding expressions`. We can bind a name to a variable or function. The left-hand side of the let expression is a `pattern`, which enables one to bind multiple names to a set of values or function values.

The `const` keyword is used to define constants. It lives for the entire lifetime of a programe but has no fixed address in memory. `static` is used to define a global variable. There is only one instance for each value and it has a _fixed location in memory_.

* Prefer **const** to **static**. It's rare that you actually want a fixed memory location & using const allows for optimizations like constant propagation.

* By convention statics are placed at the top of a code file, outside of functions.