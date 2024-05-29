#![allow(non_snake_case)]

pub use Raw_bindings;

#[macro_export]
macro_rules! Print {
    ($($arg:tt)*) => {
        unsafe {
            let formatted = format!($($arg)*);
            Rust_SDK::Raw_bindings::Print(formatted.as_ptr() as *const c_char, formatted.len());
        }
    }
}

#[macro_export]
macro_rules! Print_line {
    ($($arg:tt)*) => {
        unsafe {
            let formatted = format!($($arg)*);
            Rust_SDK::Raw_bindings::Print_line(formatted.as_ptr() as *const c_char, formatted.len());
        }
    }
}