# Ati, ergonomic indexing of `Vec`

The `ati` crate introduces the `At` trait, and implements it for `Vec`.
The `At` trait adds a `at` and `at_mut` method, which allows a `Vec` to
be indexed by `u8, u16, u32, u64, u128`, as well as `i8, i16, i32, i64, i128, isize`.

Negative indexes allows for indexing in the reverse direction, exactly how the
Javascript
[`at`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/at)
function works, or Python indexing.

## Examples

```rust
use ati::At;

fn main() {
    let mut v = vec![1,2,3];

    assert_eq!(1, *v.at(0u8));
    assert_eq!(3, *v.at(-1u128));
    
    *v.at_mut(-1) = 5;

    assert_eq!(&[1, 2, 5], &v[..]);
}
```

