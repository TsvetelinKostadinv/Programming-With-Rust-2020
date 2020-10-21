# Te(s/x)ts

## Strings
Coded in UTF-8

## Documentation
Supports MarkDown
```rust
/// this is documentation, note the 3 '/'
/// **this shall be bolded**
/// This comment the element after
fn something() -> ()
{
  //! This comments the element it is in
}
```
Documentation HTML generated with `cargo doc`

### Tests in documentation
Test that the code compiles and executes without error. These examples are pasted in code blocks as in MarkDown. The tests are run in `cargo test` and will have section doc tests.

There is more info in the rustdoc book.

## Attributes
```
#[attribute]  // for the element after
#![attribute] // for the element it is in
```
- Flags
- Conditional compilation
- Code generation

Example code generation `#[derive(Debug)]`

## Tests
- The test module is annotated with `#[cfg(test)]` .
- Tests annotated with `#[test]`.
- Tests that should panic are annotated with `#[should_panic]`
- Tests are run with `cargo test`

Tests are considered success if the function exits, and considered failed if something panicked.(unless specified otherwise)

## Panic
Kills the current thread. **NOT EXCEPTIONS**. This is unrecoverable error. Panic sources:
```rust
panic!();
assert_eq!(1,2);
None.unwrap();
```
