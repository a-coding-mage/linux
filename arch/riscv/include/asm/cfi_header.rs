/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Clang Control Flow Integrity (CFI) support.
 *
 * Copyright (C) 2023 Google LLC
 */

// Dependency supplied by the kernel's bug-trap definitions.

#[repr(C)]
pub struct pt_regs;

#[cfg(CONFIG_CFI)]
extern "C" {
    pub fn handle_cfi_failure(regs: *mut pt_regs) -> bug_trap_type;
}

#[cfg(not(CONFIG_CFI))]
#[inline]
pub unsafe fn handle_cfi_failure(_regs: *mut pt_regs) -> bug_trap_type {
    BUG_TRAP_TYPE_NONE
}

// Under CONFIG_CFI, __bpfcall is an empty C preprocessor macro.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
