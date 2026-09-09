/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent: declarations supplied by <linux/linkage.h> and related
// headers are expected to be provided by the surrounding translation unit.

use core::ffi::c_void;

pub enum pt_regs {}
pub enum switch_stack {}

extern "C" {
    pub fn do_notify_resume(regs: *mut pt_regs);
    pub fn do_sigreturn(regs: *mut pt_regs, sw: *mut switch_stack) -> *mut c_void;
    pub fn do_rt_sigreturn(regs: *mut pt_regs, sw: *mut switch_stack) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
