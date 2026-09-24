// SPDX-License-Identifier: LGPL-2.1-or-later
//! Native C ABI and warning behavior for fixed-point integer logarithms.
//!
//! Pure Rust consumers use the checked, constant-evaluable `kernel::math` API.
//! Both native entry points retain the original warning and zero return value
//! for zero input, without introducing allocation or initialization state.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[allow(unreachable_pub)]
#[path = "int_log.rs"]
mod logarithms;

/// Approximate the base-two logarithm with the original Q24 fixed-point scale.
///
/// As in C, zero input produces a non-once kernel warning and returns zero.
#[no_mangle]
pub extern "C" fn intlog2(value: u32) -> u32 {
    match logarithms::intlog2(value) {
        Some(result) => result,
        None => {
            #[cfg(CONFIG_BUG)]
            kernel::warn_on!(true);
            0
        }
    }
}

/// Approximate the base-ten logarithm with the original Q24 fixed-point scale.
///
/// As in C, zero input produces one non-once warning, not a second warning
/// from the base-two calculation, and returns zero.
#[no_mangle]
pub extern "C" fn intlog10(value: u32) -> u32 {
    match logarithms::intlog10(value) {
        Some(result) => result,
        None => {
            #[cfg(CONFIG_BUG)]
            kernel::warn_on!(true);
            0
        }
    }
}

ffi_export::export_symbol!(intlog2, intlog2, "", "");
ffi_export::export_symbol!(intlog10, intlog10, "", "");
