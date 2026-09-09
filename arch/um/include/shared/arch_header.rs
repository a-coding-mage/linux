/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_int, c_ulong, c_void};

// Supplied by the corresponding ptrace dependency.
#[allow(non_camel_case_types)]
pub enum uml_pt_regs {}

unsafe extern "C" {
    pub fn arch_check_bugs();
    pub fn arch_fixup(address: c_ulong, regs: *mut uml_pt_regs) -> c_int;
    pub fn arch_examine_signal(sig: c_int, regs: *mut uml_pt_regs);
    pub fn mc_set_rip(mc: *mut c_void, target: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
