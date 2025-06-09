# Variables

## Variables and Mutability

- Variables
- Constants
- Shadowing

## Data Types

### Scalar Types

A _scalar_ type represents a single value.

- Integer Types
  - Signed (i8, i16, i32, i64, i128, isize)
  - Unsigned (u8, u16, u32, u64, u128, usize)

    See doc integer overflow, for possibilies to handle the overflow.
- Floating-Point Types
  - All Signed(f32, f64)

Numeric operations (+, -, *, /, %)

- Boolean Type (let is_num: bool = false, true)
- Character Type (let charact: char = 'Z', Unicode)

### Compount Types

A _compound_ type can group multiple values into one type.

- Tuple Type
  - multiples types
  - fixed size

Example:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
// Access value by destructuring
let (x, y, z) = tup;
println!("The value of y is: {y}");
// Acess value by index
let five_hundred = x.0;
let one = x.2;
```

(), _unit_ type/value represent an empty value or an empty return type. [Unit documentation](https://doc.rust-lang.org/std/primitive.unit.html)

- Array Type
  - multiples types
  - fixed size

```rust
let a = [1,2,3,4,5]; // type infered
let a: [i32; 5] = [1,2,3,4,5]; // type/length specified
let a = [3; 5]; // Array of length 5 filled with 3 [3, 3, 3, 3, 3]
let first = a[0];
```

At runtime panic if index out of bounds -> no invalid memory access.
