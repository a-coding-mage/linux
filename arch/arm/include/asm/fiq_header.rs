/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  arch/arm/include/asm/fiq.h
 *
 * Support for FIQ on ARM architectures.
 * Written by Philip Blundell <philb@gnu.org>, 1998
 * Re-written by Russell King
 *
 * NOTE: The FIQ mode registers are not magically preserved across
 * suspend/resume.
 *
 * Drivers which require these registers to be preserved across power
 * management operations must implement appropriate suspend/resume handlers to
 * save and restore them.
 */

// Dependency supplied by the ARM ptrace definitions: `pt_regs`.

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

#[repr(C)]
pub struct fiq_handler {
    pub next: *mut fiq_handler,
    /* Name
     */
    pub name: *const c_char,
    /* Called to ask driver to relinquish/
     * reacquire FIQ
     * return zero to accept, or -<errno>
     */
    pub fiq_op: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    /* data for the relinquish/reacquire functions
     */
    pub dev_id: *mut c_void,
}

extern "C" {
    pub fn claim_fiq(f: *mut fiq_handler) -> c_int;
    pub fn release_fiq(f: *mut fiq_handler);
    pub fn set_fiq_handler(start: *mut c_void, length: c_uint);
    pub fn enable_fiq(fiq: c_int);
    pub fn disable_fiq(fiq: c_int);

    /* helpers defined in fiqasm.S: */
    pub fn __set_fiq_regs(regs: *const c_ulong);
    pub fn __get_fiq_regs(regs: *mut c_ulong);
}

pub unsafe fn set_fiq_regs(regs: *const pt_regs) {
    __set_fiq_regs(&(*regs).ARM_r8 as *const _);
}

pub unsafe fn get_fiq_regs(regs: *mut pt_regs) {
    __get_fiq_regs(&mut (*regs).ARM_r8 as *mut _);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
