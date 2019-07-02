# Overflow

A convenience macro for changing the overflow properties of math expressions without having to change those expressions (mostly).

## Some Background

By default Rust's math expressions (e.g., 2 * 9) panic on overflow in debug builds and overflow silently in release builds. While this is a fine default, you may want different behaivor. For instance, integer overflow may be expected behaivor. In that case you'll want to reach for the various overflow APIs available for Rust numbers (e.g., [`wrapping_mul`](https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_mul)). The issue with this however, is that you can no longer use normal math notation, instead needing to use cumbersome methods like `a.wrapping_add(b)` instead of `a + b`.

Overflow lets you keep using normal math notation but still change the way that overflows are handled.

## Example

By default the following will fail in debug builds at runtime:

```rust
(2u8.pow(20) << 20) + 2 * 2;
```

In order to make this wrap in debug and release builds you would need to write it this way:

```rust
(2u8.wrapping_pow(20).wrapping_shl(20)).wrapping_add(2u8.wrapping_mul(2))
```

Or you could use Overflow and write the following:

```rust
overflow::wrapping! { (2u8.pow(20) << 20) + 2u8 * 2 }
```

The above converts the normal meth expression syntax directly into the `wrapping` variant from above.

## Limitations

Overflow is currently limited in the following:

* The crate currently requires nightly because proc macros in expressions are not currently stable.
* Because math operations can more easily propogate type inference information than method calls, you may have to add type information when using the macros that were not neceesary before.
* Overflow behaivor is only affected at the top level (or within parenthesis) meaning that if you have math expressions inside of method invocations or inside of other macros, those expressions will not be converted.
* Conversion of `pow` is extremely naive so if you call a `pow` method on some type, this will be converted to `wrapping_pow` even if that makes no sense for that type.
