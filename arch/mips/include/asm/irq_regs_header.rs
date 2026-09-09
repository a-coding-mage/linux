/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 * Copyright (C) 2006 Ralf Baechle (ralf@linux-mips.org)
 */

// Translated from the C header __ASM_IRQ_REGS_H.
// Dependency: linux/thread_info.h supplies the thread_info and pt_regs types
// and current_thread_info().

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_info {
    pub regs: *mut pt_regs,
}

extern "C" {
    pub fn current_thread_info() -> *mut thread_info;
}

#[inline]
pub unsafe fn get_irq_regs() -> *mut pt_regs {
    (*current_thread_info()).regs
}

#[inline]
pub unsafe fn set_irq_regs(new_regs: *mut pt_regs) -> *mut pt_regs {
    let old_regs: *mut pt_regs;

    old_regs = get_irq_regs();
    (*current_thread_info()).regs = new_regs;

    old_regs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
