# Traits, iterators and anonymous functions

## Traits
- Clone
- Copy
  - Changes the semantics from move to copy
- Drop
  - Cleanup after the variable
  - Called when the thread `panic`s
- Default
- Hash
- Display
- Debug

## Operator override
- With all the operators available

## Iterators

## Adapters
- filter
- map
- reduce
- IN THAT ORDER

## Closures
```rust
| <args> | <body/expression>
```
- `move` keyword moves the values in and takes ownership
- `Fn`
- `FnMut`
- `FnOnce`
