/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 ARM Limited
 */

// This code is meant to be used from the compat vDSO only.
// The C header rejects __arch64__; retain that build-time intent here.

#[inline(always)]
pub unsafe fn dmb_ish() {
    core::arch::asm!("dmb ish", options(nostack));
}

#[inline(always)]
pub unsafe fn dmb_ishld() {
    core::arch::asm!("dmb ishld", options(nostack));
}

#[inline(always)]
pub unsafe fn dmb_ishst() {
    core::arch::asm!("dmb ishst", options(nostack));
}

#[inline(always)]
pub unsafe fn aarch32_smp_mb() {
    dmb_ish();
}

#[inline(always)]
pub unsafe fn aarch32_smp_rmb() {
    dmb_ishld();
}

#[inline(always)]
pub unsafe fn aarch32_smp_wmb() {
    dmb_ishst();
}

#[inline(always)]
pub unsafe fn smp_mb() {
    aarch32_smp_mb();
}

#[inline(always)]
pub unsafe fn smp_rmb() {
    aarch32_smp_rmb();
}

#[inline(always)]
pub unsafe fn smp_wmb() {
    aarch32_smp_wmb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
