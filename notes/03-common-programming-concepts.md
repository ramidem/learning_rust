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

## 3 Functions

- most prevalent in Rust code
- `main` is one of the most important functions and is the entry point of many
  programs
- uses *snake_case* as the conventional style for functions and variable names

```rust
fn main() {
    println!("Hello, World!");

    give_me_measurement(3, "hrs");
}
```

### Parameters

- When a has parameters, you can provide it with concrete values for those
  parameters.
- _parameters_ and _arguments_ can be used interchangeably.
- types of parameters must be declared deliberately.

```rust
fn give_me_measurement(n: i32, unit_label: &str) {
    println!("The measurement is: {}{}", n, unit_label);
}
```

### Statements and Expressions

> Rust is primarily an expression language. This means that most forms of
> value-producing or effect-causing evaluation are directed by the uniform
> syntax category of expressions.

Function bodies are made up of a series of statements optionally ending in an
expression.

_Statements_ are intructions that perform some action and do not return a value.

```rust
fn main() {
    let y = 6;

    // Statements do not return values
    // Therefore, we can not assign a `let` to another variable
    // let x = (let y = 6); will throw an error
}
```

_Expressions_ evaluate to a resulting value.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

Expressions:
- `6 + 6` evaluates to `12`
- Expressions can be part of statements;
  - `6` is an expression in the statement `let y = 6;`
- Calling a function is an expression
- Calling a macro is an expression
- a new scope block created with curly brackets is an expression

> Expressions do not include ending semicolons. Doing so will turn it into a
> statement and will not return a value. eg `x + 1`

### Functions with Return Values

- We do not name return values, but we do declare their type after an arrow (`->`)
- You can return early from a function with `return`

```rust
fn five() -> i32 {
    // expression
    // no semicolon because we want it to return
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

## 4. Comments

```rust
// This is a comment

fn main() {
    // comment
    // comment
    println!("{}", 1); // comment
}
```

## 5. Control Flow

```rust
fn main() {
    let number = 3;

    // this should evaluate to bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

