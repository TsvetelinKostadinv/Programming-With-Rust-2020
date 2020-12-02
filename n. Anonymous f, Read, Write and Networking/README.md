# Anonymous functions
- Returning closures

```rust
fn incrementer() -> fn(i32) -> i32
{
  |x| x + 1
}

fn incrementer_with( a:i32 ) -> Box< dyn Fn(i32) -> i32 >
{
  Box::new(move |x| x + a)
}
```

# Networking
`std::net`
- UDP Sockets
- TCP Streams
