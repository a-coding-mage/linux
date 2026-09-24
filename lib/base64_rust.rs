// SPDX-License-Identifier: GPL-2.0
//! Original Base64 C ABI, with real generated enum and kernel character types.

use kernel::{bindings, ffi};

#[allow(dead_code, unreachable_pub)]
#[path = "base64.rs"]
mod codec;
#[path = "../rust/ffi_export.rs"]
mod ffi_export;

/// The original C enum's generated binding; never the safe Rust API enum.
pub type Base64Variant = bindings::base64_variant;

unsafe fn to_variant(value: Base64Variant) -> codec::Base64Variant {
    match value {
        bindings::base64_variant::BASE64_STD => codec::Base64Variant::Standard,
        bindings::base64_variant::BASE64_URLSAFE => codec::Base64Variant::UrlSafe,
        bindings::base64_variant::BASE64_IMAP => codec::Base64Variant::Imap,
        // SAFETY: The original C indexes its alphabet arrays by this value.
        _ => unsafe { core::hint::unreachable_unchecked() },
    }
}

/// Encode through the original GPL-only C interface without appending NUL.
///
/// # Safety
/// `variant` must be one of the three original values, and the pointers valid
/// for every byte the original C reads/writes. Output
/// storage must fit the encoded bytes.
/// A negative length returns zero without accessing either byte buffer.
/// Overlapping storage follows the original read-before-write group order;
/// callers must not assume it supports general in-place encoding.
#[no_mangle]
pub unsafe extern "C" fn base64_encode(
    src: *const u8,
    srclen: ffi::c_int,
    dst: *mut ffi::c_char,
    padding: bool,
    variant: Base64Variant,
) -> ffi::c_int {
    // SAFETY: The caller provides the original C domain and allocated extents.
    let alphabet = unsafe { to_variant(variant) };
    let length = match usize::try_from(srclen) {
        Ok(length) => length,
        // The original loop and tail switch both skip every negative length.
        Err(_) => return 0,
    };
    let result = codec::encode_with(
        length,
        // SAFETY: Each index is a source byte that the original C reads.
        |index| unsafe { src.add(index).read() },
        |index, byte| {
            // SAFETY: Each index is an output byte the caller provides.
            unsafe { dst.add(index).write(byte as ffi::c_char) };
            Ok(())
        },
        padding,
        alphabet,
    );
    match result {
        Ok(count) => count as ffi::c_int,
        // SAFETY: The native writer never returns an output-capacity error.
        Err(_) => unsafe { core::hint::unreachable_unchecked() },
    }
}

/// Decode through the original GPL-only C interface; invalid input returns -1.
///
/// # Safety
/// `variant` must be one of the original three values, and the pointers valid
/// for the source and actual output bytes. In-place
/// decoding is supported. Complete groups preceding an error remain written.
/// A negative padded length returns -1 without buffer accesses. For a negative
/// unpadded length, the original C evaluates its three-character tail: the
/// source must hold three bytes and the destination up to two output bytes.
#[no_mangle]
pub unsafe extern "C" fn base64_decode(
    src: *const ffi::c_char,
    srclen: ffi::c_int,
    dst: *mut u8,
    padding: bool,
    variant: Base64Variant,
) -> ffi::c_int {
    // SAFETY: The caller provides the original C domain and allocated extents.
    let alphabet = unsafe { to_variant(variant) };
    let length = match usize::try_from(srclen) {
        Ok(length) => length,
        Err(_) if padding => return -1,
        // Preserve the original negative, unpadded three-character tail.
        Err(_) => 3,
    };
    match codec::decode_with(
        length,
        // SAFETY: Raw byte reads preserve valid in-place decoding without
        // inventing aliased shared/mutable slice references.
        |index| unsafe { src.add(index).read() as u8 },
        |index, byte| {
            // SAFETY: The caller supplies sufficient space for actual output.
            unsafe { dst.add(index).write(byte) };
            Ok(())
        },
        padding,
        alphabet,
    ) {
        Ok(count) => count as ffi::c_int,
        Err(codec::Base64Error::InvalidEncoding) => -1,
        // SAFETY: The native writer never reports capacity or length errors.
        Err(_) => unsafe { core::hint::unreachable_unchecked() },
    }
}

ffi_export::export_symbol!(base64_encode, base64_encode, "GPL", "");
ffi_export::export_symbol!(base64_decode, base64_decode, "GPL", "");
