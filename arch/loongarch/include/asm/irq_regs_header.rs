/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// ARCH_HAS_OWN_IRQ_REGS
pub const ARCH_HAS_OWN_IRQ_REGS: bool = true;

// Dependency corresponding to <linux/thread_info.h>.
extern "C" {
    fn current_thread_info() -> *mut thread_info;
}

// `struct pt_regs` and `struct thread_info` are supplied by the corresponding
// dependencies.  The `regs` field of `thread_info` has type `*mut pt_regs`.

pub unsafe fn get_irq_regs() -> *mut pt_regs {
    (*current_thread_info()).regs
}

pub unsafe fn set_irq_regs(new_regs: *mut pt_regs) -> *mut pt_regs {
    let old_regs: *mut pt_regs;

    old_regs = get_irq_regs();
    (*current_thread_info()).regs = new_regs;

    old_regs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
