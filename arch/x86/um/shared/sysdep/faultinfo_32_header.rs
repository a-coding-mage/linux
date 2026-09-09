/*
 * Copyright (C) 2004 Fujitsu Siemens Computers GmbH
 * Author: Bodo Stroesser <bstroesser@fujitsu-siemens.com>
 * Licensed under the GPL
 */

/*
 * This structure contains the full arch-specific faultinfo
 * from the traps.
 * On i386, ptrace_faultinfo unfortunately doesn't provide
 * all the info, since trap_no is missing.
 * All common elements are defined at the same position in
 * both structures, thus making it easy to copy the
 * contents without knowledge about the structure elements.
 */
#[repr(C)]
pub struct faultinfo {
    pub error_code: i32, /* in ptrace_faultinfo misleadingly called is_write */
    pub cr2: usize,      /* in ptrace_faultinfo called addr */
    pub trap_no: i32,    /* missing in ptrace_faultinfo */
}

#[inline]
pub const fn FAULT_WRITE(fi: &faultinfo) -> i32 {
    fi.error_code & 2
}

#[inline]
pub const fn FAULT_ADDRESS(fi: &faultinfo) -> usize {
    fi.cr2
}

/* This is Page Fault */
#[inline]
pub unsafe fn SEGV_IS_FIXABLE(fi: *const faultinfo) -> bool {
    (*fi).trap_no == 14
}

pub const PTRACE_FULL_FAULTINFO: i32 = 0;

/*
 * The C macro uses x86 inline assembly to set the faulted result and
 * current->thread.segv_continue.  The surrounding translation supplies
 * the corresponding memory operand through `segv_continue`.
 */
#[macro_export]
macro_rules! ___backtrack_faulted {
    ($faulted:expr, $segv_continue:expr) => {{
        let mut __faulted: i32;
        unsafe {
            core::arch::asm!(
                "movl $__get_kernel_nofault_faulted_{0}, {faulted}\n",
                "mov $0, {faulted}\n",
                "jmp _end_{0}\n",
                "__get_kernel_nofault_faulted_{0}:\n",
                "mov $1, {faulted}",
                "_end_{0}:",
                faulted = out(reg) __faulted,
                segv_continue = lateout(reg) $segv_continue,
                options(nostack)
            );
        }
        $faulted = __faulted;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
