# Generic types and traits

No runtime overhead with generics. Code is generated at compile time.
```rust
fn id<T>(val:T) -> T { val }

struct Pair<K,V>
{
  key: K,
  value: V,
}

enum Option<T>
{
  Some(val:T),
  None,
}

impl<K,V> Pair<K,V>
{
  fn key( &self ) -> &K
  {
    &self.key
  }
}

impl Pair<u8, f64>
{
  // Specific implementation for Pair with the types
}
```

## Traits
```rust
trait Stringifiable
{
  fn to_string(&self) -> String
  {
    // default implementation
    ""
  }
}

impl<K,V> Stringifiable for Pair<K, V> where K: Stringifiable, V:Stringifiable
{
  fn to_string(&self) -> String
  {

  }
}

impl<type params> <Trait_name> for <Type_name> where <constraints on type args>
```
If we require certain traits:
```rust
fn to_json<T: ToJSON + Debug>(val:T) -> String
{
  // T is required to implement both ToJSON and Debug
}
```

## Rules with traits
You can implement trait `T` for type `S`, then at least one of them should be declared in our own crate

## Static/Dynamic dispatch
```rust
trait Stuff {}

impl Stuff for () {}

let veriable: &dyn Stuff = &();
```
With dynamic dispatch the binary is not filled with implementations of generic functions, but generates a `vtable` at runtime aaand it becomes complicated. Also not all traits can be made into objects, if it returns `self` for example

## Generic traits
```rust
trait Graph<N, E> {}
// OR
trait Graph
{
  type N = Node;
  type E = Edge;
}

fn distance<G: Graph<N=Node, E=Edge>(...)
```
Associated types are not included in the uniqueness of the trait implementation
