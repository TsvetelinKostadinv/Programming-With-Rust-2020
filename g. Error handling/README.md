# Error Handling

## Casting interfaces
`U:From<T>` and `T:Into<U>`, if either is implemented then the other is done.
- Important: The value is moved, not copied, so the ownership is transferred to the caller.

But what is the case with parsing, do we do `impl From<String> for Foo`, well no. Because it can fail. In that case we use `impl FromStr for Foo`, but that is highly coupled.

## FromStr
```rust
trait FromStr
{
  type Err;
  fn from_str(s: str&) -> Result<Self, Self::Err>;
}

enum Result<T, E>
{
  Ok(T),
  Err(E),
}
```
- For basic types there are ready string parsers in `std::str::FromStr`.
- There is `parse` method in `str`, which is return type polymorphic to be able to parse any type which implements the `FromStr` trait.

## Error handling
The result type is used for error handling with the `match` statement:
```rust
match fn_that_returns_result(...)
{
  Ok(val) => ...,
  Err(err) => ...,
}
```
Or with the `?` operator after the result

## Panicking
```rust
panic!(...)
```
- Terminates the **thread**, deallocates the memory and every `drop` implementation is called.

### When to panic
- There was an error in the logic(we cannot recover)
- From a third party library
- When **the error is not connected to IO**(either user, file or system streams)
- In tests
- In examples, demos etc.
- In rapid prototyping of functionality(you can postpone the error handling to a later point in time)

## 2 types of errors
- Type level: `Option`, `Result`, etc. Errors from which we **can** recover
- Errors from which we **cannot** recover: `panic`, `todo`, `unreachable`
