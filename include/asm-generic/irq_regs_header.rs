/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Fallback per-CPU frame pointer holder
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// #include <linux/percpu.h>

/*
 * Per-cpu current frame pointer - the location of the last exception frame on
 * the stack
 */
// DECLARE_PER_CPU(struct pt_regs *, __irq_regs);
extern "C" {
    static mut __irq_regs: *mut pt_regs;
}

#[inline]
pub unsafe fn get_irq_regs() -> *mut pt_regs {
    // __this_cpu_read(__irq_regs)
    __irq_regs
}

#[inline]
pub unsafe fn set_irq_regs(new_regs: *mut pt_regs) -> *mut pt_regs {
    let old_regs: *mut pt_regs;

    // old_regs = __this_cpu_read(__irq_regs);
    old_regs = __irq_regs;
    // __this_cpu_write(__irq_regs, new_regs);
    __irq_regs = new_regs;
    old_regs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
