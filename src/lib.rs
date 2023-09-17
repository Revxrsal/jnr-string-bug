use std::ffi::{c_char, CString};
use std::mem;

type JavaString = *const c_char;

/// Converts a Rust string to a Java string
pub fn to_java_string(string: &str) -> *const c_char {
    let cs = CString::new(string.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    // Tell Rust not to clean up the string while we still have a pointer to it.
    // Otherwise, we'll get a segfault.
    mem::forget(cs);
    ptr
}

#[no_mangle]
extern fn simple_callback(callback: extern fn(JavaString)) {
    let value = "Any string value";
    callback(to_java_string(&value));
}