/* SPDX-License-Identifier: GPL-2.0 */

/* Grossly misnamed. */
#[repr(C)]
pub enum die_val {
    DIE_TRAP,
    DIE_NMI,
    DIE_OOPS,
    DIE_BREAKPOINT,
    DIE_SSTEP,
}

/* arch/sh/kernel/dumpstack.c */
unsafe extern "C" {
    pub fn printk_address(address: ::core::ffi::c_ulong, reliable: ::core::ffi::c_int);
    pub fn dump_mem(
        str_: *const ::core::ffi::c_char,
        loglvl: *const ::core::ffi::c_char,
        bottom: ::core::ffi::c_ulong,
        top: ::core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
