# Threads

## Creation
```rust
use std::thread

thread::spawn(<closure>);
```
- Created at OS level
- we can wait for threads with `.join()`
- With `spawn` we get a handle
  - If we don't get the handle then it is detached

## Panicking
- `panic` in main thread will terminate the whole program
- `panic` in a thread will terminate only that thread

## Results from threads
- After joining the thread returns a `Result<T, Box<Any>>`

## Sharing data
- Newly spawned thread can outlive the function it is spawned in
- How can we solve it?
  1. We can move the values in the closure like => `move || ...` - works only with a single thread
  1. We can copy...
  1. Use smart pointer - `Rc` for example - clone the `Rc` in the closure, but `Rc` canot be sent between threads safely(it is not syncronized)
  1. The `Send` trait ensures that the value can be send safely between threads
  1. `Sync` is alike, but ensures that it can be shared through reference

## Send trait
Can easily be send between threads.
- Types with internal mutability cannot be sent - `Rc`
- raw pointers
- thread local types - `rand::rngs::ThreadRng`

## Sync trait
Can share references between threads
- `T: Sync` <=> `&T: Send`
- Types that cannot be synced - `Cell`, `RefCell`

## Is `Vec<T>` a `Sync`
- If `T` is `Sync` then the whole `Vec<T>` is `Sync`

## `Send` and `Sync` impl
They are auto implemented for all `structs`, in which all fields are `Send` and `Sync`
- `unsafe` for manual implementation

## Atomic Reference Counter
- Same as `Rc`, but is atomic
- the type `T` needs to be `Send + Sync`

## Synctronization
`std::sync`
- `Mutex`
  - If a `Mutex` is locked and the thread `panic`ed, then the mutex is considered `poisoned`

## RwLock
- Read from many places
- Write from one only

## Alternative
`parking_lot`

## Atomic types
- implemented with special instructions in the CPU
