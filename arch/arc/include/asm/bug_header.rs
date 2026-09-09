/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// The declarations below are only available for non-assembler builds.
// Dependency corresponding to <asm/ptrace.h>.

pub struct pt_regs;

pub struct task_struct;

extern "C" {
    pub fn show_regs(regs: *mut pt_regs);
    pub fn show_stacktrace(
        tsk: *mut task_struct,
        regs: *mut pt_regs,
        loglvl: *const core::ffi::c_char,
    );
    pub fn show_kernel_fault_diag(str_: *const core::ffi::c_char, regs: *mut pt_regs, address: usize);
    pub fn die(str_: *const core::ffi::c_char, regs: *mut pt_regs, address: usize);
}

// Equivalent to the C BUG() macro. The pr_warn! and barrier_before_unreachable!
// macros are supplied by the surrounding kernel translation.
#[macro_export]
macro_rules! BUG {
    () => {{
        pr_warn!("BUG: failure at {}:{}/{}()!\n", file!(), line!(), module_path!());
        barrier_before_unreachable!();
        unsafe { core::intrinsics::abort() }
    }};
}

// HAVE_ARCH_BUG

// Dependency corresponding to <asm-generic/bug.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
