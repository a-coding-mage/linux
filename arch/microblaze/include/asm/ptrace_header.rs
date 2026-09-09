/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// Dependency provided by the corresponding uapi header:
// #include <uapi/asm/ptrace.h>

// These macros are unavailable to assembler sources in the original header.
// The expressions intentionally retain raw-pointer and field-access semantics.
#[macro_export]
macro_rules! kernel_mode {
    ($regs:expr) => {{ unsafe { (*($regs)).pt_mode } }};
}

#[macro_export]
macro_rules! user_mode {
    ($regs:expr) => {{ !kernel_mode!($regs) }};
}

#[macro_export]
macro_rules! instruction_pointer {
    ($regs:expr) => {{ unsafe { (*($regs)).pc } }};
}

#[macro_export]
macro_rules! profile_pc {
    ($regs:expr) => {{ instruction_pointer!($regs) }};
}

#[macro_export]
macro_rules! user_stack_pointer {
    ($regs:expr) => {{ unsafe { (*($regs)).r1 } }};
}

/// Return the value placed in the return-value register.
///
/// `crate::pt_regs` is supplied by the translated uapi dependency.
#[inline]
pub unsafe fn regs_return_value(regs: *const crate::pt_regs) -> isize {
    (*regs).r3 as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
