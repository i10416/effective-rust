#[allow(unused)]
fn function_with_poly_arguments_compiles() {
    #[derive(Debug)]
    pub struct IanaAllocated(pub u64);
    impl From<u64> for IanaAllocated {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    fn is_iana_reserved_concrete(s: IanaAllocated) -> bool {
        s.0 == 0 || s.0 == 65535
    }
    fn is_iana_reserved_generic<T: Into<IanaAllocated>>(t: T) -> bool {
        let t = t.into();
        is_iana_reserved_concrete(t)
    }

    let with_numeric_literal = is_iana_reserved_generic(42);
    let with_tpe = is_iana_reserved_generic(IanaAllocated(42));
}

#[allow(unused)]
fn conversion_between_numeric_primitives() {
    let one: i32 = 128;
    // it compiles to assign i32 to i64
    let another: i64 = one.into();
    // however, it won't compile
    // let the_another: i32 = another;
    // => you can convert an `i64` to an `i32` and panic if
    //    the converted value doesn't fit: `.try_into().unwrap()`

    // it won't compile.
    // let the_another: i32 = another.into();
    // => the trait bound `i32: From<i64>` is not satisfied
}
