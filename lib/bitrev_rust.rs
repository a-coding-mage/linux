// SPDX-License-Identifier: GPL-2.0-only
//! Native table owner, GPL export and original bitrev module metadata.
//!
//! Like the C implementation, this stateless library has no init or exit hook.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "bitrev.rs"]
mod implementation;

pub use implementation::byte_rev_table;

ffi_export::export_symbol!(byte_rev_table, byte_rev_table, "GPL", "");

#[cfg(MODULE)]
const MODINFO: &str = concat!(
    "author=Akinobu Mita <akinobu.mita@gmail.com>\0",
    "description=Bit ordering reversal functions\0license=GPL\0",
);
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "bitrev.author=Akinobu Mita <akinobu.mita@gmail.com>\0",
    "bitrev.description=Bit ordering reversal functions\0",
    "bitrev.license=GPL\0bitrev.file=", env!("RUST_MODFILE"), "\0",
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

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
