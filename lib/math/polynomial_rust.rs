// SPDX-License-Identifier: GPL-2.0-only
//! Native C flexible-array ABI and module metadata for polynomial evaluation.
//!
//! Independent Rust callers use the bounded, checked `kernel::math` API.
//! This stateless library has no initialization or cleanup entry points.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[allow(dead_code, unreachable_pub)]
#[path = "polynomial.rs"]
mod calculation;

/// Evaluate the original native-long, degree-terminated C descriptor.
///
/// # Safety
///
/// `poly` must point to an aligned, live original C `struct polynomial` with
/// readable inline terms through the first degree-zero term. The descriptor
/// must not be modified during this call. Its total divisor and each term's
/// degree, coefficient, and leftover divisor must be initialized; the per-step
/// divider needs initialization only for nonconstant terms. Padding and terms
/// after the first constant are never inspected.
///
/// Every division evaluated by the original algorithm must be defined: its
/// divisor must be nonzero and it must not divide signed `MIN` by `-1`.
/// A zero total divisor means one. All unsigned degrees retain their original
/// meaning and running time; callers must bound work for untrusted descriptors.
#[no_mangle]
pub unsafe extern "C" fn polynomial_calc(
    poly: *const kernel::bindings::polynomial,
    data: kernel::ffi::c_long,
) -> kernel::ffi::c_long {
    // SAFETY: The caller supplies the live original header and inline array.
    // Form field addresses without constructing a reference or an unbounded
    // slice. In particular, a flexible array is not a separately stored pointer.
    let total_divider = unsafe { core::ptr::addr_of!((*poly).total_divider).read() };
    let mut term =
        unsafe { core::ptr::addr_of!((*poly).terms).cast::<kernel::bindings::polynomial_term>() };
    let mut sum = 0isize;
    loop {
        // SAFETY: Every reached term has initialized fields used by C. Read
        // them individually: a constant's unused divider may be uninitialized.
        let degree = unsafe { core::ptr::addr_of!((*term).deg).read() };
        let coefficient = unsafe { core::ptr::addr_of!((*term).coef).read() };
        let divider = if degree == 0 {
            1
        } else {
            // SAFETY: This nonconstant term's divider is initialized.
            unsafe { core::ptr::addr_of!((*term).divider).read() }
        };
        // SAFETY: All reached terms evaluate their leftover divisor.
        let leftover = unsafe { core::ptr::addr_of!((*term).divider_leftover).read() };
        let value = calculation::polynomial_term_value(
            &calculation::PolynomialTerm {
                deg: degree,
                coef: coefficient,
                divider,
                divider_leftover: leftover,
            },
            data,
        );
        let value = match value {
            Some(value) => value,
            // SAFETY: Undefined evaluated C divisions are excluded above.
            None => unsafe { core::hint::unreachable_unchecked() },
        };
        sum = sum.wrapping_add(value);
        if degree == 0 {
            return match calculation::polynomial_finalize(sum, total_divider) {
                Some(value) => value,
                // SAFETY: The final evaluated division is defined by contract.
                None => unsafe { core::hint::unreachable_unchecked() },
            };
        }
        // SAFETY: The valid inline array contains the next term before its
        // required constant terminator. No pointer advances past that term.
        term = unsafe { term.add(1) };
    }
}

ffi_export::export_symbol!(polynomial_calc, polynomial_calc, "GPL", "");

#[cfg(MODULE)]
const MODINFO: &str = "description=Generic polynomial calculations\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "polynomial.description=Generic polynomial calculations\0",
    "polynomial.license=GPL\0",
    "polynomial.file=",
    env!("RUST_MODFILE"),
    "\0",
);

#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] = MODINFO.as_bytes()[index];
        index += 1;
    }
    bytes
};

#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();
