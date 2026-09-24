// SPDX-License-Identifier: GPL-2.0
//! Native C ABI owner for reciprocal division constructors.
//!
//! Pure Rust consumers use the checked, allocation-free `kernel::math` API.
//! This owner retains the original C preconditions and structure-return ABI.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[allow(dead_code, unreachable_pub)]
#[path = "reciprocal_div.rs"]
mod reciprocals;

// Native CFI encodes the original C struct tags, not only the field layout.
// Keep the pure Rust types behind the safe API and use actual header bindings
// at this boundary so indirect C calls have the same protected signatures.
/// Original C basic-reciprocal result, including its native CFI type identity.
pub type ReciprocalValue = kernel::bindings::reciprocal_value;
/// Original C advanced-reciprocal result, including its native CFI type identity.
pub type ReciprocalValueAdv = kernel::bindings::reciprocal_value_adv;

#[cfg(CONFIG_BUG)]
const __LOG_PREFIX: &[u8] = b"reciprocal_div\0";

/// Construct a reciprocal multiplier through the original C interface.
///
/// # Safety
///
/// `divisor` must be nonzero, as required by the original C constructor.
#[no_mangle]
pub unsafe extern "C" fn reciprocal_value(divisor: u32) -> ReciprocalValue {
    match reciprocals::reciprocal_value(divisor) {
        Some(value) => ReciprocalValue {
            m: value.m,
            sh1: value.sh1,
            sh2: value.sh2,
        },
        // SAFETY: The caller promises the original C constructor's domain.
        None => unsafe { core::hint::unreachable_unchecked() },
    }
}

/// Construct an advanced reciprocal through the original C interface.
///
/// # Safety
///
/// `divisor` must be in `1..=2^31` and `precision` must not exceed
/// `32 + ceil(log2(divisor))`. Outside this domain the original C performs
/// a division by zero or an invalid shift. Pure Rust callers should use the
/// checked API instead of assuming a result for those inputs.
#[no_mangle]
pub unsafe extern "C" fn reciprocal_value_adv(divisor: u32, precision: u8) -> ReciprocalValueAdv {
    // Retain the original non-once divisor warning before arithmetic. This
    // diagnostic does not make inputs outside the C domain supported.
    #[cfg(CONFIG_BUG)]
    if divisor == 0 || divisor > (1u32 << 31) {
        kernel::pr_warn!(
            "ceil(log2(0x{:08x})) == 32, reciprocal_value_adv doesn't support such divisor",
            divisor
        );
        kernel::warn_on!(true);
    }
    match reciprocals::reciprocal_value_adv(divisor, precision) {
        Some(value) => ReciprocalValueAdv {
            m: value.m,
            sh: value.sh,
            exp: value.exp,
            is_wide_m: value.is_wide_m,
        },
        // SAFETY: The caller promises the original C constructor's domain.
        None => unsafe { core::hint::unreachable_unchecked() },
    }
}

ffi_export::export_symbol!(reciprocal_value, reciprocal_value, "", "");
ffi_export::export_symbol!(reciprocal_value_adv, reciprocal_value_adv, "", "");
