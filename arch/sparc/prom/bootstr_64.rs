// SPDX-License-Identifier: GPL-2.0
/*
 * bootstr.c:  Boot string/argument acquisition from the PROM.
 *
 * Copyright(C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright(C) 1996,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

// Dependencies supplied by the surrounding kernel translation unit.

/* WARNING: The boot loader knows that these next three variables come one right
 *          after another in the .data section.  Do not move this stuff into the
 *          .bss section or it will break things.
 */

/* We limit BARG_LEN to 1024 because this is the size of the
 * 'barg_out' command line buffer in the SILO bootloader.
 */
pub const BARG_LEN: usize = 1024;

#[repr(C)]
pub struct BootstrInfo {
    pub bootstr_len: core::ffi::c_int,
    pub bootstr_valid: core::ffi::c_int,
    pub bootstr_buf: [core::ffi::c_char; BARG_LEN],
}

#[cfg(CONFIG_CMDLINE)]
pub static mut bootstr_info: BootstrInfo = BootstrInfo {
    bootstr_len: BARG_LEN as core::ffi::c_int,
    bootstr_valid: 1,
    // The C build substitutes the CONFIG_CMDLINE string literal here.
    bootstr_buf: [0; BARG_LEN],
};

#[cfg(not(CONFIG_CMDLINE))]
pub static mut bootstr_info: BootstrInfo = BootstrInfo {
    bootstr_len: BARG_LEN as core::ffi::c_int,
    bootstr_valid: 0,
    bootstr_buf: [0; BARG_LEN],
};

extern "C" {
    static mut prom_chosen_node: core::ffi::c_int;
    fn prom_getstring(
        node: core::ffi::c_int,
        property: *const core::ffi::c_char,
        buf: *mut core::ffi::c_char,
        bufsize: core::ffi::c_int,
    );
}

#[inline]
pub unsafe fn prom_getbootargs() -> *mut core::ffi::c_char {
    /* This check saves us from a panic when bootfd patches args. */
    if bootstr_info.bootstr_valid != 0 {
        return bootstr_info.bootstr_buf.as_mut_ptr();
    }
    prom_getstring(
        prom_chosen_node,
        b"bootargs\0".as_ptr() as *const core::ffi::c_char,
        bootstr_info.bootstr_buf.as_mut_ptr(),
        BARG_LEN as core::ffi::c_int,
    );
    bootstr_info.bootstr_valid = 1;
    bootstr_info.bootstr_buf.as_mut_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
