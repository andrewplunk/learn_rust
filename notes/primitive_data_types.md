# Primitive Data Types

## Boolean

```rust
let x = true;
let y: bool = false;
```

## char

```rust
let x = 'x';
let y: char = 'y';
```

## i8, i16, i32, i64

Fixed size(bit) signed(+/-) integer types

## u8, u16, u32, u64

Fixed size(bit) unsigned(+) integer types

## isize

Variable sized signed(+/-) integer
This data type is the number of bytes it takes to reference any location in memory.

## usize

Variable sized unsigned(+) integer
The size of this primitive is how many bytes it takes to reference any location in memory.

## f32

32 bit floating point

## f64

64 bit floating point

## Arrays

Fixed sized list of elements of the same data type

```yaml
let a = [1, 2, 3];
let mut b = [1, 2, 3];

let c: [i32, 0] = [];
let d: [i32; 3] = [1, 2, 3];
let e = ["my value"; 3];
```

Arrays are immutable by default even with the `mut` modifier it's size cannot be modified. Use the `Vec` data type for a growable collection.

## Tuples

Fixed size ordered list of elements of different(or same) data types.

```rust
let a = (1, 1.5, true, 'a', "Hello world");

let b: (i32, i64) = (1, 1.5);
```

Tuples are immutable by default. Even with mut size cannot be changed.

## Slice

Dynamically sized reference to another data structure, can be mutable or not.

```rust

let a: [i32; 4] = [1, 2, 3, 4];

let b: &[i32] = &a; // slice whole array

let c = &a[0..4]; // From [0,4)

let f = &a[1..];

let g = &a[..3];
```

## str

Unsized UTF=8 sequence of unicode string slices
