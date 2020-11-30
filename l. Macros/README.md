# Macros

```rust
macro_rules! macro_name {
  ( <the whole expression of tokens> ) => ();
}

macro_rules! macro_name {
  ( $expr1:expr ) => ();
}

macro_rules! map {
  $( $key:expr => $value:expr ),* $(,)? => (
    {
      let mut map = ::std::collections::HashMap::new();
      $( map.insert( $key, $value ); )*
      map
    }
  );
}
```

## Quantifiers
- `*` - 0 or more
- `+` - 1 or more
- `?` - 0 or 1

## "Types"
- `expr`
- `ident`

## Invocation
- `macro! ()`
- `macro! []`
- `macro! {}`

## Use of macros from other crates
- `#[macro_export]` - on the macro
- `#[macro_use]` - to use a macro from a crate
