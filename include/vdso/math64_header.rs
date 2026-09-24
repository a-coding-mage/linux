/* SPDX-License-Identifier: GPL-2.0 */

//! Shared division and multiply/add/shift arithmetic for vDSO-style users.
//!
//! The checked shift interface follows the original configuration-specific
//! domain. No foreign calls, allocation, or architecture-specific ABI layout
//! are needed.

#[allow(dead_code, unused_imports, unreachable_pub)]
#[path = "../asm-generic/div64_header.rs"]
pub(crate) mod division;

pub use division::implementation::iter_div_u64_rem as __iter_div_u64_rem;

/// Multiply two 32-bit words without losing any product bits.
pub const fn mul_u32_u32(a: u32, b: u32) -> u64 {
    (a as u64).wrapping_mul(b as u64)
}

/// Multiply, add, shift, and retain the low 64 bits of the shifted result.
///
/// With `CONFIG_ARCH_SUPPORTS_INT128` on a 64-bit target, shifts below 128
/// are accepted. The original limb fallback requires a shift below 64 and,
/// when `a` has a nonzero high word, a shift no greater than 32. Other shifts
/// return `None`, rather than evaluating an undefined C shift.
pub const fn mul_u64_u32_add_u64_shr(a: u64, mul: u32, b: u64, shift: u32) -> Option<u64> {
    if cfg!(all(
        CONFIG_ARCH_SUPPORTS_INT128,
        target_pointer_width = "64"
    )) {
        if shift >= 128 {
            return None;
        }
    } else if shift >= 64 || (a >> 32 != 0 && shift > 32) {
        return None;
    }

    let (low, high) = division::implementation::mul_u64_wide(a, mul as u64);
    let (low, carry) = low.overflowing_add(b);
    Some(shift_product(low, high.wrapping_add(carry as u64), shift))
}

/// Shift a double-word product using the generic C helper's high-word mask.
pub(crate) const fn shift_product(low: u64, high: u64, shift: u32) -> u64 {
    if shift == 0 {
        low
    } else if shift < 64 {
        (low >> shift) | (high << (64 - shift))
    } else {
        high >> (shift & 63)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
