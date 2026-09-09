/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <asm/ptrace.h>

/* This structure matches the layout of the data saved to the stack
   following a device-not-present interrupt, part of it saved
   automatically by the 80386/80486.
   */

#[repr(C)]
pub struct math_emu_info {
    pub ___orig_eip: core::ffi::c_long,
    pub regs: *mut pt_regs,
}

// Supplied by asm/ptrace.h.
pub struct pt_regs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
