#[macro_export]
macro_rules! observe_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                // do something with `e`, e.g. log, handle, etc.
                return Err(e.into());
            }
        }
    };
}
