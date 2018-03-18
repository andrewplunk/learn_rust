# Functions

* Declared by `fn` keyword
* When using `arguments` you **must declare data types**.
* By default functions **return an empty tuple ()**. If you want to return a value a return type must be sepcified after `->`.

## Passing arguments

```rust
fn print_sum(a: i8, b: i8) {
    println!("sum is: {}, a + b);
}
```

## Returning Values

```rust
fn plus_one(a: i32) -> i32 {
    a + 1 // no ; means an expression, return a+1
}

fn plus_two(a: i32) -> i32 {
    return a + 2; // bad practice, should only use on conditional returns
}
```