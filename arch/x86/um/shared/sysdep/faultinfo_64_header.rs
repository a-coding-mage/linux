/*
 * Copyright (C) 2004 Fujitsu Siemens Computers GmbH
 * Author: Bodo Stroesser <bstroesser@fujitsu-siemens.com>
 * Licensed under the GPL
 */

/*
 * This is the Rust translation of the x86_64 faultinfo header.  The original
 * include guard is not needed in Rust.
 */

/* this structure contains the full arch-specific faultinfo
 * from the traps.
 * On i386, ptrace_faultinfo unfortunately doesn't provide
 * all the info, since trap_no is missing.
 * All common elements are defined at the same position in
 * both structures, thus making it easy to copy the
 * contents without knowledge about the structure elements.
 */
#[repr(C)]
pub struct faultinfo {
    pub error_code: core::ffi::c_int, /* in ptrace_faultinfo misleadingly called is_write */
    pub cr2: core::ffi::c_ulong, /* in ptrace_faultinfo called addr */
    pub trap_no: core::ffi::c_int, /* missing in ptrace_faultinfo */
}

#[macro_export]
macro_rules! FAULT_WRITE {
    ($fi:expr) => {
        (($fi).error_code & 2)
    };
}

#[macro_export]
macro_rules! FAULT_ADDRESS {
    ($fi:expr) => {
        ($fi).cr2
    };
}

/* This is Page Fault */
#[macro_export]
macro_rules! SEGV_IS_FIXABLE {
    ($fi:expr) => {
        ((*($fi)).trap_no == 14)
    };
}

pub const PTRACE_FULL_FAULTINFO: core::ffi::c_int = 1;

/*
 * The C macro uses architecture-specific inline assembly and the external
 * `current->thread.segv_continue` object.  The assembly is retained here;
 * the memory operand is supplied by the caller's surrounding architecture
 * definitions.
 */
#[macro_export]
macro_rules! ___backtrack_faulted {
    ($faulted:expr, $segv_continue:expr) => {
        unsafe {
            core::arch::asm!(
                "movq $__get_kernel_nofault_faulted_{0}, {segv_continue}",
                "mov $0, {faulted}",
                "jmp _end_{0}",
                "__get_kernel_nofault_faulted_{0}:",
                "mov $1, {faulted}",
                "_end_{0}:",
                faulted = out(reg) $faulted,
                segv_continue = inout(reg) $segv_continue,
            );
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
