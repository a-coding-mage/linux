// SPDX-License-Identifier: GPL-2.0
/*
 * base64 encoding, lifted from fs/crypto/fname.c.
 */

// Dependency: `u8` corresponds directly to the Linux `u8` type.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum base64_variant {
    /// RFC 4648 (standard)
    BASE64_STD,
    /// RFC 4648 (base64url)
    BASE64_URLSAFE,
    /// RFC 3501
    BASE64_IMAP,
}

#[macro_export]
macro_rules! BASE64_CHARS {
    ($nbytes:expr) => {
        (($nbytes) * 4 + 2) / 3
    };
}

unsafe extern "C" {
    pub fn base64_encode(
        src: *const u8,
        len: core::ffi::c_int,
        dst: *mut core::ffi::c_char,
        padding: bool,
        variant: base64_variant,
    ) -> core::ffi::c_int;

    pub fn base64_decode(
        src: *const core::ffi::c_char,
        len: core::ffi::c_int,
        dst: *mut u8,
        padding: bool,
        variant: base64_variant,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
