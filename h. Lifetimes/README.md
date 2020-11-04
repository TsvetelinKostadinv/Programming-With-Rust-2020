# Lifetimes

When you borrow(reference a value), you have to ensure that value lives to the last use of the reference
```rust
let r;
{
  let x = 5; // -----------+ x lives to the end of the scope
  r = &x; //---+           |
}// <----------|-----------+
//             v
println!("{}", r)
```

## Can we break the borrow checker

## Lifetime annotations
We need to specify how much the value will live.(If we want to return a reference from a function for example)
```rust
fn longer<'a>(s1:&'a str , s2:&'a str) -> &'a str
{
  if s1.len() > s2.len()
  {
    s1
  }else{
    s2
  }
}
```
Here `'a` annotates that the 2 values need to live equally long

`'a` - lifetime parameter or lifetime annotation. They always come before other generics

```rust
// lifetimes can be different
fn longer<'a, 'b>(s:&'a str , patt:&'b str) -> &'a str
{
  s.matches(patt).next()
}
```

## Lifetime ellipses
Every reference has a lifetime param, but it is not required, if it can be inferred.
- when the lifetime param is omitted and the function returns a reference then the lifetime is applied to the return value as well.
- if the function declares `self` the the lifetime of `self` is applied

## When NOT to write Lifetimes
- in a block of code(the compiler is smart enough to know it)
- **sometimes** in the signature
- structure definition

## `'static` lifetime parameter
Specifies that the reference lives as long as the program is running.

## References in `struct`s
The question is: Is the type owned or is it not?

```rust
struct Words<'a>
{
  text: &'a str,
}

impl<'a> Words<'a>
{
  fn new(text: &str) -> Words
  {
    Words{text}
  }
  // ...
}

let w1 = Words::new("abc");
{
  let s = String::from("lives till the end of the scope");
  let w2 = Words::new(s.as_str());
}
```

## Lifetimes and generics
You can add lifetime params with the `+` with generics
