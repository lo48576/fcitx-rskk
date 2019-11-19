// About `improper_ctypes`: cbindgen generates `u128` for `long double` type.
// However, long double won't be used for fcitx-related stuff and this would be hard to fix, so
// ignore the warnings here.
//
// See <https://github.com/rust-lang/rust-bindgen/issues/1560> and
// <https://github.com/rust-lang/rust/issues/54341>.
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
