// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//! Native owner of the Linux glob library's C ABI and metadata.
#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "glob.rs"]
mod implementation;
pub use implementation::*;

ffi_export::export_symbol!(glob_match, glob_match, "", "");
ffi_export::export_symbol!(glob_match_len, glob_match_len, "", "");

/*
 * The only reason this code can be compiled as a module is because the
 * ATA code that depends on it can be as well.  In practice, they're
 * both usually compiled in and the module overhead goes away.
 */
#[cfg(MODULE)]
const MODINFO: &str = "description=glob(7) matching\0license=Dual MIT/GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "glob.description=glob(7) matching\0glob.license=Dual MIT/GPL\0glob.file=",
    env!("RUST_MODFILE"),
    "\0"
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
#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();
