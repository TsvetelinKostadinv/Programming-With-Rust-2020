# Structures, modules and external packets

## Structs
```rust
struct User
{
  id: i32,
  username: string,
  password: string,
  email: string, // note the trailing comma(my fav feature)
}
```

## Methods and Associated functions
Data and functionality are segregated in different blocks.
```rust
struct User{...}
impl User
{
  // <associated functions> with the type (think of static functions)
  fn name( /*(&mut)*/ self) {} // function for a given structure (think of instance methods)
}
```
Implementation blocks are not limited to one per structure(there can be many, and they can be in a lot of files, but the compiler guarantees that there are no 2 functions with the same name)

## Tuple Structures
Just named tuples. Access to fields through index
```rust
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
let black = Color(0,0,0);
let redVal = black.0;
let greenVal = black.1;
let blueVal = black.2;
```

## Empty Structures
- 0 bytes.
- Can be used as markers.
- They may disappear in compilation.

```rust
struct Electron {};
struct Photon;
let x = Electron {};
let y = Photon; // note the missing {}
```

## Modules
Modules = Hierarchy.
```rust
mod name {
  ...
}
```

```rust
mod name1;
mod name2;
// these 2 lines include the modules
```
```
src
 +-- main.rs / lib.rs
 +-- module_name.rs
 +-- parent_module
     +-- child_module.rs
     +-- mod.rs // implementation
```

Access to something in a module -> `<crate/crate_name>::module_name:: ... ::function_or_Struct;`

Everything **public** in rust **has** to have `pub` modifier

### Use syntax for easier access
- `pub use crate::module::{func1, func2...}` -> imports specific functions/structs
- `pub use crate::module::*` -> imports everything

### Shorthand references to modules
- `self:: ...` -> same module
- `super:: ...` -> parent module
- `:: ...` -> root

### Access modifiers
- By default - everything private, unless specified `pub`
- You can access everything declared in the current module and all parent modules
- All fields in structs are private by default

## Packets
Classic example is a random number generator. Curiously Rust does not have one. Crates can be found at `https://crates.io` or just google it, for god's sake.

Crates can be added to `Cargo.toml` manually like:
```toml
[dependencies]
name = 'semantic_version'
```
Or installed by `cargo`:
```bash
cargo install <crate>
```
Packets are compiled locally, but at a different stage than our project.

Documentation for crates can be found at `https://docs.rs`
