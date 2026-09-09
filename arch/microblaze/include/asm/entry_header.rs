/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions used by low-level trap handlers
 *
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 */

/* Dependencies: asm/percpu.h, asm/ptrace.h, and linux/linkage.h. */

/*
 * These are per-cpu variables required in entry.S, among other
 * places.
 */

macro_rules! PER_CPU {
    ($var:ident) => {
        $var
    };
}

extern "C" {
    static mut KSP: u32; /* Saved kernel stack pointer */
    static mut KM: u32; /* Kernel/user mode */
    static mut ENTRY_SP: u32; /* Saved SP on kernel entry */
    static mut R11_SAVE: u32; /* Temp variable for entry */
    static mut CURRENT_SAVE: u32; /* Saved current pointer */

    fn do_notify_resume(regs: *mut pt_regs, in_syscall: i32);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
