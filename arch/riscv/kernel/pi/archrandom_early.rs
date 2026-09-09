// SPDX-License-Identifier: GPL-2.0-only

// Dependency intent from <asm/csr.h>, <linux/processor.h>, and "pi.h".

// To avoid rewriting code include asm/archrandom.h and create macros
// for the functions that won't be included.
//
// The C preprocessor configuration makes riscv_has_extension_likely(...) always
// false and pr_err_once(...) a no-op; neither is used in this translation.

extern "C" {
    fn fdt_early_match_extension_isa(
        dtb: *const core::ffi::c_void,
        extension: *const core::ffi::c_char,
    ) -> bool;
    fn csr_seed_long(seed: *mut usize) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn get_kaslr_seed_zkr(dtb_pa: usize) -> u64 {
    let mut seed: usize = 0;

    if !fdt_early_match_extension_isa(
        dtb_pa as *const core::ffi::c_void,
        b"zkr\0".as_ptr() as *const core::ffi::c_char,
    ) {
        return 0;
    }

    if !csr_seed_long(&mut seed as *mut usize) {
        return 0;
    }

    seed as u64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
