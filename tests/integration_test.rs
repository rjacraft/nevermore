#![allow(dead_code)] // the fields are not used

use std::convert::Infallible;

/// Checks that the type is convertible [from][`From`] [`Infallible`].
fn assert_is_from_infallible<T: From<Infallible>>() {}

#[test]
fn regular_struct() {
    #[derive(nevermore::FromNever)]
    #[allow(dead_code)]
    struct Something {
        foo: String,
        bar: i32,
    }

    assert_is_from_infallible::<Something>();
}

#[test]
fn empty_struct() {
    #[derive(nevermore::FromNever)]
    #[allow(dead_code)]
    struct Nothing;

    assert_is_from_infallible::<Nothing>();
}

#[test]
fn tuple_struct() {
    #[derive(nevermore::FromNever)]
    #[allow(dead_code)]
    struct Values(i32, Box<[u8]>, Vec<&'static str>);

    assert_is_from_infallible::<Values>();
}

#[test]
fn never_struct() {
    #[derive(nevermore::FromNever)]
    #[allow(dead_code)]
    enum MyNever {}

    assert_is_from_infallible::<MyNever>();
}

#[test]
fn generic_struct() {
    fn never_struct() {
        #[derive(nevermore::FromNever)]
        #[allow(dead_code)]
        struct Container<'a, const N: usize, T> {
            str_ref: &'a str,
            value: T,
            array: [u8; N],
        }

        assert_is_from_infallible::<Container<'static, 10, String>>();
    }
}
