// SPDX-License-Identifier: ISC
//! Native C ABI and module metadata for the translated CORDIC library.
//!
//! Independent Rust consumers use the pure `kernel::math` API. Like the
//! original stateless library, this owner needs no init or exit function.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[allow(dead_code, unreachable_pub)]
#[path = "cordic.rs"]
mod coordinates;

// Use the original header's nominal type as well as its layout at the C ABI
// boundary. KCFI distinguishes `struct cordic_iq` from the pure Rust API's
// `CordicIq`, even though both have identical fields and calling conventions.
pub use kernel::bindings::cordic_iq as CordicIq;

/// Calculate the original Q16 coordinate, preserving the C aggregate ABI.
///
/// The signed degree input is converted to fixed point before normalization,
/// including the original 32-bit wrapping behavior for out-of-range inputs.
#[no_mangle]
pub extern "C" fn cordic_calc_iq(theta: i32) -> CordicIq {
    let coordinate = coordinates::cordic_calc_iq(theta);
    CordicIq {
        i: coordinate.i,
        q: coordinate.q,
    }
}

ffi_export::export_symbol!(cordic_calc_iq, cordic_calc_iq, "", "");

// Preserve the original author, description, license and built-in file record.
// The byte array emits actual strings, not pointers, in the .modinfo section.
#[cfg(MODULE)]
const MODINFO: &str = concat!(
    "description=CORDIC algorithm\0",
    "author=Broadcom Corporation\0",
    "license=Dual BSD/GPL\0",
);
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "cordic.description=CORDIC algorithm\0",
    "cordic.author=Broadcom Corporation\0",
    "cordic.license=Dual BSD/GPL\0",
    "cordic.file=",
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

// Identify loadable Rust code without inventing module initialization state.
#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();
