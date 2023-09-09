#![no_std]

use core::convert::Infallible;

fn assert_is_from_infallible<T: From<Infallible>>() {}

#[test]
fn regular_struct() {
    #[derive(nevermore::FromNever)]
    #[allow(dead_code)]
    struct Something {
        foo: &'static str,
        bar: i32,
    }

    assert_is_from_infallible::<Something>();
}
