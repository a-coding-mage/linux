// SPDX-License-Identifier: GPL-2.0
//! Native C ABI and module metadata for rational approximation.
//!
//! Pure Rust consumers use `kernel::math` or the translated rational header.
//! Like the original library, this owner requires no init or exit function.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[allow(unreachable_pub)]
#[path = "rational.rs"]
mod fractions;

/// Writes the best approximation using the kernel's unsigned-long C ABI.
///
/// # Safety
///
/// Both output pointers must be aligned, valid for writing a `usize`, and
/// exclusively accessible for the duration of this call. They may point to
/// the same location; as in C, the denominator is written last.
#[no_mangle]
pub unsafe extern "C" fn rational_best_approximation(
    given_numerator: usize,
    given_denominator: usize,
    max_numerator: usize,
    max_denominator: usize,
    best_numerator: *mut usize,
    best_denominator: *mut usize,
) {
    let (numerator, denominator) = fractions::rational_best_approximation(
        given_numerator,
        given_denominator,
        max_numerator,
        max_denominator,
    );
    // SAFETY: The caller guarantees valid output storage. Sequential raw
    // writes preserve permitted aliasing without creating aliased references.
    unsafe {
        best_numerator.write(numerator);
        best_denominator.write(denominator);
    }
}

ffi_export::export_symbol!(
    rational_best_approximation,
    rational_best_approximation,
    "",
    ""
);

// Match MODULE_DESCRIPTION and MODULE_LICENSE, including the built-in file
// record used by modules.builtin.modinfo. Use byte arrays, not Rust references,
// so .modinfo contains only the NUL-terminated strings expected by Kbuild.
#[cfg(MODULE)]
const MODINFO: &str = "description=Rational fraction support library\0license=GPL v2\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "rational.description=Rational fraction support library\0",
    "rational.license=GPL v2\0",
    "rational.file=",
    env!("RUST_MODFILE"),
    "\0",
);

#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut i = 0;
    while i < bytes.len() {
        bytes[i] = MODINFO.as_bytes()[i];
        i += 1;
    }
    bytes
};

// Match the standard Rust loadable-module marker without adding initialization
// state or entry points to a stateless library.
#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();
