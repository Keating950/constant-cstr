//! `constant-cstr` exists to enable the safe creation of [`CStr`](std::ffi::CStr) instances at
//! compile time, enabling safer and more efficient FFI.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitByteStr, LitStr};

const fn find_first_null(bytes: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0 {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// `cstr` checks its input for null bytes and, if none are found, generates an [`&'static
/// CStr`](std::ffi::CStr). It must be provided a string literal.
/// # Examples
/// ```
/// use std::ffi::CStr;
/// use constant_cstr::cstr;
///
/// const DEV_PTMX: &'static CStr = cstr!("/dev/ptmx");
/// ```
/// Passing an input string with a null byte will cause a compile error with a message indicating
/// the position of the null byte:
/// ```compile_fail
/// use std::ffi::CStr;
/// use constant_cstr::cstr;
///
/// const HELLO: &'static CStr = cstr!("Hell\0, world");
/// ```
/// ```text,no_run
/// error: proc macro panicked
///   --> src/example.rs:4:34
///    |
///  4 |     const HELLO: &'static CStr = cstr!("Hell\0, world");
///    |                                  ^^^^^^^^^^^^^^^^^^^^^^
///    |
///    = help: message: "Hell\0, world" contains a null byte at position 4
/// ```
#[proc_macro]
pub fn cstr(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let input_str = parse_macro_input!(input as LitStr);
    let mut input_bytes = input_str.value().into_bytes();
    if let Some(null_idx) = find_first_null(&input_bytes) {
        panic!(
            "{} contains a null byte at position {}",
            input_clone, null_idx
        );
    }
    input_bytes.push(0);
    let output_bytes = LitByteStr::new(&input_bytes, input_str.span());
    quote! { unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(#output_bytes) } }.into()
}
