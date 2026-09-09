/* SPDX-License-Identifier: GPL-2.0 */
/*
 * kdebug.h: Defines and definitions for debugging the Linux kernel
 *           under various kernel debuggers.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependency intent from the original header:
// #include <asm/openprom.h>
// #include <asm/vaddrs.h>

/* Breakpoints are entered through trap table entry 126. */
pub const DEBUG_BP_TRAP: u32 = 126;

/* The debug vector is passed in %o1 at boot time. */
pub type debugger_funct = unsafe extern "C" fn() -> u32;

#[repr(C)]
pub struct kernel_debug {
    /* First the entry point into the debugger. */
    pub kdebug_entry: u32,
    pub kdebug_trapme: u32, /* Figure out later... */
    /* Number of pages taken from the total pool. */
    pub kdebug_stolen_pages: *mut u32,
    /* Synchronization function for the debugger. */
    pub teach_debugger: debugger_funct,
} /* I think that is it... */

unsafe extern "C" {
    pub static mut linux_dbvec: *mut kernel_debug;
}

/* Use this function in C-code to enter the debugger. */
#[inline]
pub unsafe fn sp_enter_debugger() {
    // Original instruction sequence:
    // __asm__ __volatile__("jmpl %0, %%o7\n\t" "nop\n\t"
    //                      : : "r" (linux_dbvec) : "o7", "memory");
    #[cfg(target_arch = "sparc")]
    core::arch::asm!(
        "jmpl {target}, %o7",
        "nop",
        target = in(reg) linux_dbvec,
        out("o7") _,
        options(nostack),
    );
}

/* Use this macro in C-code to enter the debugger. */
#[inline]
pub unsafe fn SP_ENTER_DEBUGGER() {
    if !linux_dbvec.is_null()
        && *(linux_dbvec as *const i16) != -1
    {
        sp_enter_debugger();
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum die_val {
    DIE_UNUSED,
    DIE_OOPS,
}

/* Some nice offset defines for assembler code. */
pub const KDEBUG_ENTRY_OFF: u32 = 0x0;
pub const KDEBUG_DUNNO_OFF: u32 = 0x4;
pub const KDEBUG_DUNNO2_OFF: u32 = 0x8;
pub const KDEBUG_TEACH_OFF: u32 = 0xc;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
