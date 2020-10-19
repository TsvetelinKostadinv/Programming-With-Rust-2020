# Hello Rust
Opening lecture with first coding experience with Rust

## Hello World
```rust
fn main() {
    println!("Hello, world!");
}
```
To run it we can execute the compiler directly with `rustc <file>.rs`, but we can do better - `cargo run` is the command that calls Rust's package manager Cargo and tells it to execute the project.

## Variables
```rust
let x = 5;
let y: i32 = 3;
let <name>: <type> = <value>;
```
- The rust compiler is smart enough to deduce variable types so you will not have to declare them explicitly, most of the time
- Variables are constant by default, if you want to make them mutable, you have to use the keyword `mut`

### Shadowing
Shadowing is the process of redeclaring a variable with a new value(however the previous value is **not** lost as long as there are references to it)
```rust
let x = 10;
let x = x + 10;
let x = x * 3; // from now on the value of x is 60
```

### Basic types
#### The integer types
```rust
i8, i16, i32, i64, i128, isize // singned
u8, u16, u32, u64, u128, usize // unsigned
```
- `isize` and `usize` have the number of bytes as the architecture.
- After a number literal you can specify what the size is: `42u8`, or separate them with an `_` and become: `42_u8`, which is more readable(there are no limits to the number of underscores in a number)
- The numbers can be in different bases:
```rust
let deadbeef = 0xDEADBEEF;
let cafebabe = 0xCAFEBABE;
let oct = 0o77;
let binary = 0b101;
```

#### Overflow for numeric types
```rust
let x = 255_u8;
let y = x + 1;                // 💥
println!("{}", y);
```
This will cause a runtime exception that will terminate the program

#### The floating point types
```rust
f32, f64
```

#### Boolean type
`bool` -> true or false, nothing interesting really

#### Unit
Like `void`, but cooler.
```rust
let x: () = (); // 0 bytes in memory
```

#### Strings
The type is `str`, but it's a bit complicated. The following 3 are exactly the same thing:
```rust
let s = "Something";
```
```rust
let s: &str = "Something";
```
```rust
let s: &'static str = "Something";
```
Well it is what it is and we have to accept it. Also strings are encoded in `UTF-8`

#### Characters
`char` -> a single character

#### Arrays
Fixed size, have to be known at compile time.
```rust
let arr: [i32; 3] = [1, 2, 3];

let nested: [[i32; 3]; 2] = [
    [1, 2, 3],
    [4, 5, 6],
];

let <name>: [<type>; length] = <value>;
```

#### Tuples
Different types. Practically ordered set of `n` elements. Access to each elements through it's index.
```rust
let tuple: (i32, u32, bool) = (1, 2, false);
let unit: () = ();

println!("{}", tuple.0);
println!("{}", tuple.1);
println!("{}", tuple.2);
```

### Specifics
Rust cannot convert from one type to another. So it will not.
```rust
let x: i32 = 1;
let y: u64 = x; // <- error for it cannot convert
```

### Casting
The keyword `as`
```rust
let x: i32 = 12_u8 as i32;
```

## Statements
### The if statement
Same as in every language. Just that the brackets around the condition are not needed, but the block **is**.
```rust
if bool_expression {
    // ...
} else if another_bool_expression {
    // ...
} else {
    // ...
}
```

### For-each loop
```rust
for var in iterable {
    // ...
}
```

### While loop
```rust
while bool_expression {
    // ...
}
```
### The loop loop
Same as a `while true`, but readable
```rust
loop {
    // ...
}
```

## Statements vs. Expressions
Some constructions can be used as either. For example the if:
```rust
let bigger = if a > b {
    a
} else {
    b
};
```
A block can be used as expression also. Then the `return` is not needed(the last expression in the block will determine it's value)
```rust

```

## Macros
Code generation tools, denoted by the `!` at the end.
```rust
println!(...)
print!(...)
dbg!(...)
```
