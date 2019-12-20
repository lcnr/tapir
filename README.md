# Tapir

A simple rust library adding tapping functionality to all types.

The `tap` operation takes full ownership of a variable, calls the given function with a mutable
reference to the given variable and then returns full ownership of it.
This allows for easy mutation without having to declare the variable as mutable.

This library is partially inspired by [tap-rs].

## Examples

```rust
fn get_unsorted_values() -> Vec<u32> {
    vec![42, 7, 1337, 69]
}

fn use_sorted_values(values: &[u32]) {
    assert_eq!(&[7, 42, 69, 1337], values);
}

// without tap one often needs mutable variables
let mut old = get_unsorted_values();
old.sort();
use_sorted_values(&old);

// using tap, this can be simplified
use tapir::Tap;

let tapped = get_unsorted_values().tap(|v| v.sort());
use_sorted_values(&tapped);
```

[tap-rs]: https://github.com/darfink/tap-rs