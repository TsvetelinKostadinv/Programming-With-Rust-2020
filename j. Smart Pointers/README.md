# Smart Pointers

## Sized
- Knows the type at compile-time
```rust
fn foo<T>() {}

/// T CAN implement Sized, but it is not REQUIRED
fn bar<T: ?Sized> () {}
```

```rust
fn foo<T>( t: T ) {} // fine T implements Sized

/// T CAN implement Sized, but it is not REQUIRED
fn bar<T: ?Sized> ( t : &t) {} // fine &T is Sized

fn baz<T: ?Sized> (t: T) {} // not fine
```

## Raw reference `&`
- no ownership, someone else has to own the memory

## Box
- Smart pointer of a reference and ownership
- But is on the heap(slower)
- `*` - dereferences the inner value
```rust
Box<dyn SomeTrait>
```

## Deref
- defines what the `*` does to the value
- Variant is: `DerefMut`
- From `&T` to `&U`, when there is `T: Deref<Target = U>`
- From `&mut T` to `&mut U`, when there is `T: DerefMut<Target = U>`
- From `&mut T` to `&U`, when there is `T: Deref<Target = U>`
- From `&mut T` to `&T`, this is just coercion

## Rc
- Reference counted smart pointer
```rust
let fst = Rc::new( String::from( "Foo" ) );
let scnd = Rc::clone(&fst);
```
- Heap allocated
- Does not actually clone the value, but returns a view of the value in the heap( and increments the counter )
- Read only tho
- Solved by COW(copy on write)
  - Which takes a `&mut Rc<T>` and returns a `&mut T`
  - If we are the owner then the value is returned
  - Otherwise the value is cloned and the new reference is returned

## Cell and RefCell
- Internal mutability
- Mainly for `Copy` types
- `get`
- `set`
- `replace` - replaces the value and returns the old one
- `into_inner` - consumes the cell and returns the inner value
- On `RefCell`
  - `borrow` - get a reference to the inner value
  - `borrow_mut` - gets a mutable reference(will panic if we call it 2 times in the same scope)

## Weak references
- `Rc::downgrade` returns a weak reference, which if called upon `upgrade` will return an `Option<Rc<T>>`, which may contain the value if it is still alive, but will return `None` if the value has been cleared already.

## Raw pointers
```
let raw_ptr: *<const/mut> <type>;
```
- `is_null`
- `as_ref` -> Option
- `offset`
