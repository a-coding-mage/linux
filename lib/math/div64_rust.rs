// SPDX-License-Identifier: GPL-2.0
//! Native C ABI owner for the generic wide-division library.
//!
//! Checked, pointer-free Rust callers use `kernel::math`. This owner retains
//! the original nonzero-divisor precondition and architecture export set.
//! Architecture-specific assembly and inline implementations are not replaced.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[allow(dead_code, unreachable_pub)]
#[path = "div64.rs"]
mod division;

/// Extract a result whose original C arithmetic preconditions are satisfied.
///
/// # Safety
///
/// `result` must be `Some`, as guaranteed by the native caller's contract.
#[inline]
unsafe fn defined<T>(result: Option<T>) -> T {
    match result {
        Some(value) => value,
        // SAFETY: The caller promises that the checked arithmetic succeeded.
        None => unsafe { core::hint::unreachable_unchecked() },
    }
}

/// Divide a mutable 64-bit dividend by a 32-bit divisor.
///
/// The generic definition remains weak so, for example, PowerPC's strong
/// assembly implementation wins at link time. Kbuild enables `linkage` only
/// for this native owner; the shared arithmetic needs no unstable features.
///
/// # Safety
///
/// `divisor` must be nonzero. `dividend` must be aligned, initialized, and
/// valid for exclusive reading and writing of one `u64` for this call.
#[cfg(all(
    target_pointer_width = "32",
    not(any(
        CONFIG_X86_32,
        CONFIG_ARM,
        CONFIG_MIPS,
        all(CONFIG_M68K, not(CONFIG_CPU_HAS_NO_MULDIV64))
    ))
))]
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn __div64_32(dividend: *mut u64, divisor: u32) -> u32 {
    // SAFETY: The caller provides an initialized, aligned dividend pointer.
    let mut quotient = unsafe { dividend.read() };
    // SAFETY: A nonzero divisor is required by the original C interface.
    let remainder = unsafe { defined(division::do_div(&mut quotient, divisor)) };
    // SAFETY: The caller grants exclusive write access to the dividend.
    unsafe { dividend.write(quotient) };
    remainder
}

/// Signed 64/32 division with a remainder having the dividend's sign.
///
/// As in the original 32-bit generic implementation, `i64::MIN / -1`
/// produces the wrapped quotient and a zero remainder.
///
/// # Safety
///
/// `divisor` must be nonzero. `remainder` must be aligned and valid for an
/// exclusive write of one `i32` for the duration of this call.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn div_s64_rem(dividend: i64, divisor: i32, remainder: *mut i32) -> i64 {
    // SAFETY: The wrapping native variant accepts every nonzero divisor.
    let (quotient, rest) = unsafe { defined(division::div_s64_rem_wrapping(dividend, divisor)) };
    // SAFETY: The caller provides an aligned, writable remainder pointer.
    unsafe { remainder.write(rest) };
    quotient
}

/// Unsigned full-width division with remainder.
///
/// # Safety
///
/// `divisor` must be nonzero. `remainder` must be aligned and valid for an
/// exclusive write of one `u64` for the duration of this call.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn div64_u64_rem(dividend: u64, divisor: u64, remainder: *mut u64) -> u64 {
    // SAFETY: The original C interface requires a nonzero divisor.
    let (quotient, rest) = unsafe { defined(division::div64_u64_rem(dividend, divisor)) };
    // SAFETY: The caller provides an aligned, writable remainder pointer.
    unsafe { remainder.write(rest) };
    quotient
}

/// Unsigned full-width division without calculating an unused remainder.
///
/// # Safety
///
/// `divisor` must be nonzero, as required by the original C interface.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn div64_u64(dividend: u64, divisor: u64) -> u64 {
    // SAFETY: The caller promises a nonzero divisor.
    unsafe { defined(division::div64_u64(dividend, divisor)) }
}

/// Signed full-width division with the original 32-bit wrapping result.
///
/// # Safety
///
/// `divisor` must be nonzero. `remainder` must be aligned and valid for an
/// exclusive write of one `i64` for the duration of this call.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn div64_s64_rem(dividend: i64, divisor: i64, remainder: *mut i64) -> i64 {
    // SAFETY: The wrapping native variant accepts every nonzero divisor.
    let (quotient, rest) = unsafe { defined(division::div64_s64_rem_wrapping(dividend, divisor)) };
    // SAFETY: The caller provides an aligned, writable remainder pointer.
    unsafe { remainder.write(rest) };
    quotient
}

/// Signed full-width division with the original 32-bit wrapping quotient.
///
/// # Safety
///
/// `divisor` must be nonzero, as required by the original C interface.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn div64_s64(dividend: i64, divisor: i64) -> i64 {
    // SAFETY: The wrapping native variant accepts every nonzero divisor.
    unsafe { defined(division::div64_s64_wrapping(dividend, divisor)) }
}

/// Iteratively divide inputs whose quotient is expected to be small.
///
/// The subtraction loop and wrapping `u32` quotient match the original helper.
///
/// # Safety
///
/// `divisor` must be nonzero. `remainder` must be aligned and valid for an
/// exclusive write of one `u64` for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn iter_div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u64) -> u32 {
    // SAFETY: The caller promises the original nonzero-divisor precondition.
    let (quotient, rest) = unsafe { defined(division::iter_div_u64_rem(dividend, divisor)) };
    // SAFETY: The caller provides an aligned, writable remainder pointer.
    unsafe { remainder.write(rest) };
    quotient
}

/// Divide a full-width product plus addend, saturating an oversized quotient.
///
/// This generic export is absent on x86-64, including UML, where the original
/// inline assembly retains its trapping overflow behavior instead.
///
/// # Safety
///
/// `divisor` must be nonzero, as required by the original C interface.
#[cfg(not(CONFIG_X86_64))]
#[no_mangle]
pub unsafe extern "C" fn mul_u64_add_u64_div_u64(a: u64, b: u64, add: u64, divisor: u64) -> u64 {
    // SAFETY: The caller promises a nonzero divisor. Overflow is supported.
    unsafe { defined(division::mul_u64_add_u64_div_u64(a, b, add, divisor)) }
}

#[cfg(all(
    target_pointer_width = "32",
    not(any(
        CONFIG_X86_32,
        CONFIG_ARM,
        CONFIG_MIPS,
        all(CONFIG_M68K, not(CONFIG_CPU_HAS_NO_MULDIV64))
    ))
))]
ffi_export::export_symbol!(__div64_32, __div64_32, "", "");
#[cfg(target_pointer_width = "32")]
ffi_export::export_symbol!(div_s64_rem, div_s64_rem, "", "");
#[cfg(target_pointer_width = "32")]
ffi_export::export_symbol!(div64_u64_rem, div64_u64_rem, "", "");
#[cfg(target_pointer_width = "32")]
ffi_export::export_symbol!(div64_u64, div64_u64, "", "");
#[cfg(target_pointer_width = "32")]
ffi_export::export_symbol!(div64_s64_rem, div64_s64_rem, "", "");
#[cfg(target_pointer_width = "32")]
ffi_export::export_symbol!(div64_s64, div64_s64, "", "");
ffi_export::export_symbol!(iter_div_u64_rem, iter_div_u64_rem, "", "");
#[cfg(not(CONFIG_X86_64))]
ffi_export::export_symbol!(mul_u64_add_u64_div_u64, mul_u64_add_u64_div_u64, "", "");
