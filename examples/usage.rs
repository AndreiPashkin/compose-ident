use compose_idents::compose_idents;

compose_idents!(
    my_fn_1 = [foo, _, "baz"];
    my_fn_2 = [spam, _, eggs];
    my_const = [upper(foo), _, lower(BAR)];
    my_static = [upper(lower(BAR))]; {
    fn my_fn_1() -> u32 {
        123
    }

    fn my_fn_2() -> u32 {
        321
    }

    const my_const: u32 = 42;
    static my_static: u32 = 42;
});

macro_rules! outer_macro {
    ($name:tt) => {
        compose_idents!(my_nested_fn = [nested, _, $name]; {
            fn my_nested_fn() -> u32{
                42
            }
        });
    };
}

outer_macro!(foo);

fn main() {
    assert_eq!(foo_baz(), 123);
    assert_eq!(spam_eggs(), 321);
    assert_eq!(nested_foo(), 42);
    assert_eq!(FOO_bar, 42);
    assert_eq!(BAR, 42);
}
