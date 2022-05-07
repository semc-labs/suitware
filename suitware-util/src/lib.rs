pub mod macros {
    #[macro_export]
    macro_rules! ok {
        ( $x:expr ) => {
            tracing::trace!("OK {}", $x);
        };
        ( $x:expr,$($y:expr)* ) => {
            ok!(format!($x, $($y)*));
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracing_ok_compiles() {
        ok!("");
    }
}
