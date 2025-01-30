use std::io;

use thiserror::Error;

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

#[allow(unused)]
fn function_implicit_conversion() {
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    let op: fn(i32, i32) -> i32 = sum;
}

#[allow(unused)]
fn closure_fn_trait() {
    fn take_callback<F>(f: F)
    where
        F: Fn(i32) -> (),
    {
        let _ = f(1);
    }
    take_callback(|v| println!("{v}"));
}

#[derive(Error, Debug)]
#[allow(unused)]
enum ErrorKind {
    #[error("resource not found at {location}")]
    NotFound { location: String },
    #[error("data corruption: hint: {hint:?}")]
    DataCorruption { hint: Option<String> },
    #[error("{0}")]
    IOFailure(#[from] io::Error),
}

#[allow(unused)]
fn this_error_derives_std_error_that_enables_short_circuit_behavior(
) -> Result<(), Box<dyn std::error::Error>> {
    let it = Err(ErrorKind::NotFound {
        location: "area 51".to_string(),
    })?; // conversion from ErrorKind::NotFound to Box<std::error::Error>
    Ok(())
}

#[cfg(test)]
#[test]
fn error_handling_by_this_error() {
    let error = ErrorKind::NotFound {
        location: "area 51".to_string(),
    };
    assert_eq!(
        format!("{error}"),
        "resource not found at area 51",
        "thiserror derives Display implementation"
    );
}
