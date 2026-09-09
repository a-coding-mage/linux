/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The included kernel types and alternative
// macros are supplied by other translation units.

#[repr(C)]
pub struct alt_instr {
    pub orig_offset: s32, /* offset to original instruction */
    pub alt_offset: s32,  /* offset to replacement instruction */
    pub cpucap: u16,      /* cpucap bit set for replacement */
    pub orig_len: u8,     /* size of original instruction(s) */
    pub alt_len: u8,      /* size of new instruction(s), <= orig_len */
}

pub type alternative_cb_t = unsafe extern "C" fn(
    alt: *mut alt_instr,
    origptr: *mut __le32,
    updptr: *mut __le32,
    nr_inst: i32,
);

// __init
unsafe extern "C" {
    pub fn apply_boot_alternatives();
    // __init
    pub fn apply_alternatives_all();
    pub fn alternative_is_applied(cpucap: u16) -> bool;

    #[cfg(feature = "CONFIG_MODULES")]
    pub fn apply_alternatives_module(start: *mut core::ffi::c_void, length: usize) -> i32;

    pub fn alt_cb_patch_nops(
        alt: *mut alt_instr,
        origptr: *mut __le32,
        updptr: *mut __le32,
        nr_inst: i32,
    );
}

#[cfg(not(feature = "CONFIG_MODULES"))]
#[inline]
pub unsafe fn apply_alternatives_module(
    _start: *mut core::ffi::c_void,
    _length: usize,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
