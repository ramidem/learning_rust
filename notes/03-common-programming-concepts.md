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
- Constants - immutable by default - are values that are bound to a name and are
  not allowed to change.
- Always annotate the contant's type.
- Constants can be declared in any scope, including global scope.
- Constants may be set only to a contant expression, not the result of a value
  that could only be computed at runtime.
- Constants use uppercase with underscore between words.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 + 60 + 3;
```

### Shadowing
note: read more on shadowing

## 2. Data Types

Rust is _statically typed_, it must know the types of all variables at compile
time.
Rust usually infers what type the variable has. Otherwise, variable should be
annotated

### Data Type Subsets

#### 1. Scalar

A _scalar_ type represents a single value.

##### Primary Scalar Types

a. Integers
  - numbers without fractional components
  - Signed integer types start with `i`, Unsigned with `u`
  - Signed and Unsigned refer to whether it is possible for the number to be
    negative.

| Length  | Signed | Unsigned |
| ---     | ---    | ---      |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

| Number Literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

When unsure which type of integer to use, integer types default to `i32`.
Use `isize` or `usize` when indexing some sort of collection.

> [Rust does not prevent the overflow/underflow](https://bit.ly/3tNYvdK) behavior but silently ignores it
> and computes [_two's complement wrapping_].

    ```rust
    fn main() {
      let (x, y) = (2345, 6789);

      let will_overflow = multiply_will_flow(x, y);

      println!(
          "{} * {} {}",
          x,
          y,
          if will_overflow {
              "overflows"
          } else {
              "does not overflow"
          }
      )
    }

    fn multiply_will_flow(x: i64, y: i64) -> bool {
        x.checked_mul(y).is_none()
    }
    ```

b. Floating-point numbers

2 Primitive types of floating-point numbers;
  - `f32`
  - `f64` - default

c. Booleans

- `true` and `false`
- one byte in size
- specified using `bool`

d. Characters

`char` type is Rust's most primitive alphabetic type.

Note:
  - `char` literals are specified with single quotes
  - string literals with double quotes

#### 2. Compound

_Compound types_ can group multiple values into one

a. Tuple

Tuples are fixed. Once declared, they can not grow or shrink in size.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    ///
    let (x, y, z) = tup;
}
```

A **unit type** is a tuple without values, `()`, and has only one value "**unit
value**" also written `()`.

b. Array

Array is not as flexible as the vector type. Array is the one to use when you
know the number of elements needed.

```rust
// Write an array's type using square brackets with the type of each element, a
// semicolon, and then the number of elements in the array.
let a: [i32, 5] = [1, 2, 3, 4, 5];

// initialize an array
let b = [3; 5]; // array with 5 elements of `3`
let similar = [3, 3, 3, 3, 3];

// access
let first = a[0];
let second = a[1];
```


