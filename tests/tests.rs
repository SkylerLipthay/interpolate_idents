#![feature(plugin)]
#![plugin(interpolate_idents)]

macro_rules! define_foo {
    ($x:ident) => ( interpolate_idents! {
        fn [foo_ $x _1]() -> u32 { 1 }

        struct [Foo $x] { [$x _30]: u32 }
        impl [Foo $x] {
            pub fn new() -> [Foo $x] {
                [Foo $x] { [$x _30]: 30 }
            }
        }
    } )
}

define_foo!(bar);

#[test]
fn test_macro() {
    assert_eq!(foo_bar_1(), 1);
    assert_eq!(Foobar::new().bar_30, 30);
}
