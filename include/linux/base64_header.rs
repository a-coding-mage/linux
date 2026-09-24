// SPDX-License-Identifier: GPL-2.0
/*
 * base64 encoding, lifted from fs/crypto/fname.c.
 */
//! Safe, allocation-free Base64 interfaces shared by Rust consumers.
//!
//! These slice functions are not alternate declarations of the C exports.
//! The `ffi` module calls the selected C or Rust provider through its actual
//! generated C enum and pointer bindings; it never duplicates the provider.

#[path = "../../lib/base64.rs"]
mod implementation;

pub use implementation::{
    base64_chars, base64_decode, base64_encode, encoded_len, Base64Error, Base64Variant,
};

/// Original unsafe native API and nominal C enum, from generated bindings.
///
/// Pointer extents, valid variants and output capacity are the caller's
/// responsibility. The decoder permits in-place buffers. Negative lengths
/// retain C behavior: encode returns zero, padded decode returns -1, and
/// unpadded decode reads a three-character tail and may write two bytes.
#[cfg(CONFIG_RUST)]
pub mod ffi {
    pub use kernel::bindings::{base64_decode, base64_encode, base64_variant};
}
/// Standard RFC 4648 alphabet.
pub const BASE64_STD: Base64Variant = Base64Variant::Standard;
/// URL-safe RFC 4648 alphabet.
pub const BASE64_URLSAFE: Base64Variant = Base64Variant::UrlSafe;
/// Modified RFC 3501 alphabet.
pub const BASE64_IMAP: Base64Variant = Base64Variant::Imap;
/// Checked equivalent of the original size macro (unpadded output).
pub use base64_chars as BASE64_CHARS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
