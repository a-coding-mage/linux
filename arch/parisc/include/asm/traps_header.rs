/* SPDX-License-Identifier: GPL-2.0 */

// #include and header-guard directives from the C header are omitted.

use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

pub const PARISC_ITLB_TRAP: c_ulong = 6; // defined by architecture. Do not change.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    // traps.c
    #[cold]
    pub fn parisc_terminate(
        msg: *mut c_char,
        regs: *mut pt_regs,
        code: c_int,
        offset: c_ulong,
    ) -> !;

    pub fn die_if_kernel(str_: *mut c_char, regs: *mut pt_regs, err: c_long);

    // mm/fault.c
    pub fn parisc_acctyp(code: c_ulong, inst: c_uint) -> c_ulong;
    pub fn trap_name(code: c_ulong) -> *const c_char;
    pub fn do_page_fault(regs: *mut pt_regs, code: c_ulong, address: c_ulong);
    pub fn handle_nadtlb_fault(regs: *mut pt_regs) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
