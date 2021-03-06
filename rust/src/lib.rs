//! `Miscreant`: Misuse resistant symmetric encryption library providing the
//! AES-SIV (RFC 5297), AES-PMAC-SIV, and STREAM constructions
//!
//! # Build Notes
//!
//! miscreant.rs works on stable rust since `1.27`. By default it is built with aesni
//! support which requires an x86 instruction set. You can disable this with 
//! the `aes-soft` feature flag which enables usage on other architectures.
//! 
//! The default configuration uses the `core::arch` API for stable access to 
//! CPU intrinsics, namely the [Intel AES-NI]  instructions which provide a 
//! hardware implementation of AES.
//! 
//!
//! To access these features, you will need both a relatively recent
//! Rust nightly and to pass the following as RUSTFLAGS:
//!
//! `RUSTFLAGS=-Ctarget-feature=+aes`
//!
//! You can configure your `~/.cargo/config` to always pass these flags:
//!
//! ```toml
//! [build]
//! rustflags = ["-Ctarget-feature=+aes"]
//! ```
//!
#![crate_name = "miscreant"]
#![crate_type = "lib"]
#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unused_import_braces, unused_qualifications)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "bench", feature(test))]
#![cfg_attr(feature = "staticlib", feature(lang_items))]
#![doc(html_root_url = "https://docs.rs/miscreant/0.4.0-beta2")]

extern crate aes;
extern crate block_modes;
extern crate byteorder;
extern crate clear_on_drop;
extern crate cmac;
extern crate crypto_mac;
extern crate dbl;
extern crate pmac;
extern crate subtle;

#[cfg(feature = "std")]
extern crate core;

#[cfg(all(feature = "bench", test))]
extern crate test;

pub mod aead;
mod ctr;
pub mod error;
pub mod ffi;
mod s2v;
pub mod siv;
pub mod stream;

#[cfg(feature = "bench")]
mod bench;

// no_std boilerplate for building a static library
#[cfg(feature = "staticlib")]
#[allow(unsafe_code)]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt(_args: ::core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}
