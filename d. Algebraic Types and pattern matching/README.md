# Algebraic Types and Pattern-matching

## Algebraic types / Sum Type / Enums
All their possible values are stated at declaration.
```rust
enum  IpAddress
{
  v4,
  v6, // <- note the trailing comma, easily my fav feature in every language
}
```
But that's not all, they can contain information:
```rust
enum  IpAddress
{
  v4(String),
  v6(String),
}
```
**OR**
```rust
enum  IpAddress
{
  v4(u8, u8, u8, u8),
  v6(String),
}

impl IpAddress
{
  fn ping(&self)
  {
    // ping the IP address
  }
}
```
The structure has the size of `8 + the largest variant`! But when the structure is of just discriminant values it has the size to accommodate all the values.

## Enum Option
Fairly simple enum, but quite a powerful one:
```rust
enum Option<T>
{
  Some(T), // Represents a value
  None,    // Represents absence of a value
}
```

## Pattern matching
From functional world, useful for enums. The construction is: (both statement and expression)
```rust
let x = Some(42_u32);
match x {
  Some(val) => ... ,
  None      => ... ,
}
```
Unpacks the values in the cases.

As for the expression example:
```rust
let x = Some(42_u32);
let y = match x {
  Some(val) => ... ,
  None      => ... ,
}
```
In the cases there can also be blocks.

To match all other cases we use `_`(or any name for that matter):
```rust
match num {
  1      => ...,
  2..10  => ...,
  _      =>
}
```

## If-let construction
```rust
let some_val = Some(8);

if let Some(8) = some_val
{
  // code from here is executed in the case that some_val is Some(8)
}
```

## While-let construction
```rust
let some_val = Some(8);

while let Some(8) = some_val
{
  // code from here is executed while some_val is Some(8)
}
```

## Pattern matching guards
```rust
match x
{
  <destructuring> if <bool expressions> => <action>,
  ...
}
```

```rust
match age
{
  n if n<0 => ..., // less than zerp
  n @ 0 ... 15 => ..., // binds if it is in range
  18 | 21 => ..., // if it is 18 or 21
}
```

## Pattern matching structs
```rust
match user
{
  User { name: "Name", age: _ } => ... , // matches the name
  User { name: _, age: 18 }     => ... , // matches the age
  User { name: x, ..}           => ... , /// binds the name
  _                             => ... , // everything else
}
```

## Destructuring with `ref`
Pattern matching takes ownership of the value.(For example the String type)
```rust
enum Token
{
  Text(String),
  Number(f64),
}

fn main()
{
  let token = Token::Text(String::from("Whatever"));
  match &token
  {
    Token::Text(text) => ... ,
    Token::Number(num) => ... ,
  }
}
```
Note the `&`, this is a reference to the token, if it weren't for it, then the ownership of the values is taken inside the match.

## Pattern matching in assignments
```rust
let (mut a, b) = (1,2); // a = 1, b = 2
let User { name: name_var, .. } = user; // name_var = user.name
```

## Refutable and irrefutable patterns
The cases where the pattern is not exhaustive - refutable pattern
