# Control Flow

## if - else if - else

```rust
let size = 7;

if size < 5 {
    println!("small");
} else if size < 10 {
    println!("medium");
} else {
    println!("Large");
}

let size_str = if size < 5 {
    "Small"
} else if size < 10 {
    "Medium"
} else {
    "Large"
};

println!("Size:{}", size_str);

let below_eighteen = if size < 18 { true } else { false };
```
