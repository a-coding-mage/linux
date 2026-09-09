// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn kaslr_disabled_cmdline() -> bool;
    fn kaslr_offset() -> u64;
    fn pr_info(format: *const core::ffi::c_char, ...);
    fn pr_warn(format: *const core::ffi::c_char, ...);
}

#[no_mangle]
pub static mut __kaslr_is_enabled: bool = false;

pub unsafe extern "C" fn kaslr_init() {
    if kaslr_disabled_cmdline() {
        pr_info(b"KASLR disabled on command line\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    /*
     * The KASLR offset modulo MIN_KIMG_ALIGN is taken from the physical
     * placement of the image rather than from the seed, so a displacement
     * of less than MIN_KIMG_ALIGN means that no seed was provided.
     */
    if kaslr_offset() < MIN_KIMG_ALIGN {
        pr_warn(b"KASLR disabled due to lack of seed\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    pr_info(b"KASLR enabled\n\0".as_ptr() as *const core::ffi::c_char);
    __kaslr_is_enabled = true;
}

unsafe extern "C" fn parse_nokaslr(_unused: *mut core::ffi::c_char) -> i32 {
    /* nokaslr param handling is done by early cpufeature code */
    0
}

// early_param("nokaslr", parse_nokaslr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
