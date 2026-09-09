/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

/*
 * The following functions are exported (but prefixed). Declare them here so
 * that LLVM does not complain it lacks the 'static' keyword (which, if
 * added, makes LLVM complain because the function is unused).
 */

unsafe extern "C" {
    pub fn get_kaslr_seed(dtb_pa: usize) -> u64;
    pub fn get_kaslr_seed_zkr(dtb_pa: usize) -> u64;
    pub fn set_nokaslr_from_cmdline(dtb_pa: usize) -> bool;
    pub fn set_satp_mode_from_cmdline(dtb_pa: usize) -> u64;
    pub fn set_satp_mode_from_fdt(dtb_pa: usize) -> u64;

    pub fn fdt_early_match_extension_isa(fdt: *const c_void, ext_name: *const c_char) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
