# interpolate_idents

You cannot currently define a struct, enum, function, or field using
`concat_idents!` due to the way macros are parsed by the Rust compiler. This
will hopefully change in the future, but `interpolate_idents!` sloppily solves
a side effect of the currently lacking macro system *today*.

```rust
#![feature(plugin)]
#![plugin(interpolate_idents)]

macro_rules! make_fn {
    ($x:ident) => ( interpolate_idents! {
        fn [my_ $x _fn]() -> u32 { 1000 }
    } )
}
```

Now `make_fn!(favorite);` is equivalent to
`fn my_favorite_fn() -> u32 { 1000 }`.

In short, surround multiple space-separated identifiers (or macro identifer
variables) with square brackets to concatenate the identifiers. Check
`tests/tests.rs` for another example.

This plugin was quickly hacked together. It is likely not performant and most
certainly not readable.

## Crate upkeep

I'm not actively developing on nightly, so I haven't been using this plugin too often. I understand that `libsyntax` is a fickle beast, so please file an issue or PR if `interpolate_idents` fails to compile on the latest nightly!
