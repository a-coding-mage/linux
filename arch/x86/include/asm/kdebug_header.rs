/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation.

pub struct pt_regs;

/* Grossly misnamed. */
#[repr(C)]
pub enum die_val {
    DIE_OOPS = 1,
    DIE_INT3,
    DIE_DEBUG,
    DIE_PANIC,
    DIE_NMI,
    DIE_DIE,
    DIE_KERNELDEBUG,
    DIE_TRAP,
    DIE_GPF,
    DIE_CALL,
    DIE_PAGE_FAULT,
    DIE_NMIUNKNOWN,
}

#[repr(C)]
pub enum show_regs_mode {
    SHOW_REGS_SHORT,
    /*
     * For when userspace crashed, but we don't think it's our fault, and
     * therefore don't print kernel registers.
     */
    SHOW_REGS_USER,
    SHOW_REGS_ALL,
}

extern "C" {
    pub fn die(
        str_: *const ::core::ffi::c_char,
        regs: *mut pt_regs,
        err: ::core::ffi::c_long,
    );
    pub fn die_addr(
        str_: *const ::core::ffi::c_char,
        regs: *mut pt_regs,
        err: ::core::ffi::c_long,
        gp_addr: ::core::ffi::c_long,
    );
    // __must_check
    pub fn __die(
        str_: *const ::core::ffi::c_char,
        regs: *mut pt_regs,
        err: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    pub fn show_stack_regs(regs: *mut pt_regs);
    pub fn __show_regs(
        regs: *mut pt_regs,
        mode: show_regs_mode,
        log_lvl: *const ::core::ffi::c_char,
    );
    pub fn show_iret_regs(regs: *mut pt_regs, log_lvl: *const ::core::ffi::c_char);
    pub fn oops_begin() -> ::core::ffi::c_ulong;
    pub fn oops_end(
        arg: ::core::ffi::c_ulong,
        regs: *mut pt_regs,
        signr: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
