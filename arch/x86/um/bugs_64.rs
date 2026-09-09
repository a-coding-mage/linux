/*
 * Copyright 2003 PathScale, Inc.
 *
 * Licensed under the GPL
 */

// Dependencies supplied by the surrounding UML implementation:
// #include <arch.h>
// #include <sysdep/ptrace.h>

use core::ffi::c_int;

/// External register structure declared by `sysdep/ptrace.h`.
#[repr(C)]
pub struct uml_pt_regs {
    _private: [u8; 0],
}

pub fn arch_check_bugs() {}

pub fn arch_examine_signal(_sig: c_int, _regs: *mut uml_pt_regs) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
