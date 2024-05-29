#![no_std]
#![allow(non_snake_case)]

use core::ffi::c_char;

#[link(wasm_import_module = "host")]
extern "C" {
    pub fn Print(s: *const c_char, len: usize);
    pub fn Print_line(s: *const c_char, len: usize);
}
