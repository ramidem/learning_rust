# Common Programming Concepts

## 1. Variables and Mutability

### Variables
- Variables are immutable by default. A variable's value can not be changed.
- Adding `mut` in from of a variable name will make it mutable.

```rust
fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}
```

### Constants
- Constants - immutable by default - are values that are bound to a name and are not allowed to change.
- Always annotate the contant's type.
- Constants can be declared in any scope, including global scope.
- Constants may be set only to a contant expression, not the result of a value that could only be computed at runtime.
- Constants use uppercase with underscore between words.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 + 60 + 3;
```

### Shadowing
note: read more on shadowing
