/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency intent from the original header:
// #include <asm/qspinlock.h>
// #include <asm/qrwlock.h>

/* See include/linux/spinlock.h */
#[inline(always)]
pub unsafe fn smp_mb__after_spinlock() {
    smp_mb();
}

/*
 * Changing this will break osq_lock() thanks to the call inside
 * smp_cond_load_relaxed().
 *
 * See:
 * https://lore.kernel.org/lkml/20200110100612.GC2827@hirez.programming.kicks-ass.net
 */
// The original self-referential macro preserves the vcpu_is_preempted symbol.
#[inline(always)]
pub fn vcpu_is_preempted(_cpu: i32) -> bool {
    false
}

// External dependency supplied by the surrounding kernel translation.
unsafe extern "C" {
    fn smp_mb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
