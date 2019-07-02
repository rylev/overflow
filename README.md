# Overflow

A convenience macro for changing the overflow properties of math expressions without having to change those expressions (mostly).

## Example

By default the following will fail in dev builds at runtime:

```rust
(2u8.pow(20) << 20) + 2 * 2;
```

In order to make this wrap in dev and release builds you would need to write it this way:

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
