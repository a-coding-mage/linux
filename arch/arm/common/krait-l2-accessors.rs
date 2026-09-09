// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// Dependencies supplied by the corresponding kernel headers:
// linux/spinlock.h, linux/export.h, asm/barrier.h, and
// asm/krait-l2-accessors.h.

use core::arch::asm;

// Equivalent to DEFINE_RAW_SPINLOCK(krait_l2_lock).
static mut krait_l2_lock: crate::raw_spinlock_t = crate::raw_spinlock_t::new();

#[inline]
unsafe fn isb() {
    asm!("isb", options(nomem, nostack, preserves_flags));
}

pub unsafe fn krait_set_l2_indirect_reg(addr: u32, val: u32) {
    let mut flags: crate::c_ulong;

    crate::raw_spin_lock_irqsave(&raw mut krait_l2_lock, &raw mut flags);
    /*
     * Select the L2 window by poking l2cpselr, then write to the window
     * via l2cpdr.
     */
    asm!(
        "mcr p15, 3, {addr}, c15, c0, 6 @ l2cpselr",
        addr = in(reg) addr,
        options(nostack, preserves_flags)
    );
    isb();
    asm!(
        "mcr p15, 3, {val}, c15, c0, 7 @ l2cpdr",
        val = in(reg) val,
        options(nostack, preserves_flags)
    );
    isb();

    crate::raw_spin_unlock_irqrestore(&raw mut krait_l2_lock, flags);
}

pub unsafe fn krait_get_l2_indirect_reg(addr: u32) -> u32 {
    let mut val: u32;
    let mut flags: crate::c_ulong;

    crate::raw_spin_lock_irqsave(&raw mut krait_l2_lock, &raw mut flags);
    /*
     * Select the L2 window by poking l2cpselr, then read from the window
     * via l2cpdr.
     */
    asm!(
        "mcr p15, 3, {addr}, c15, c0, 6 @ l2cpselr",
        addr = in(reg) addr,
        options(nostack, preserves_flags)
    );
    isb();
    asm!(
        "mrc p15, 3, {val}, c15, c0, 7 @ l2cpdr",
        val = out(reg) val,
        options(nostack, preserves_flags)
    );

    crate::raw_spin_unlock_irqrestore(&raw mut krait_l2_lock, flags);

    val
}

// EXPORT_SYMBOL(krait_set_l2_indirect_reg);
// EXPORT_SYMBOL(krait_get_l2_indirect_reg);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
