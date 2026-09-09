/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
 */

// The original header guard and include directives are C-only.
// CONFIG_PPC64 selects the declarations and operations below at build time.

#[cfg(CONFIG_PPC64)]
extern "C" {
    pub fn __ppc64_runlatch_on();
    pub fn __ppc64_runlatch_off();
}

/*
 * We manually hard enable-disable, this is called
 * in the idle loop and we don't want to mess up
 * with soft-disable/enable & interrupt replay.
 */
#[cfg(CONFIG_PPC64)]
#[inline(always)]
pub unsafe fn ppc64_runlatch_off() {
    if cpu_has_feature(CPU_FTR_CTRL) && test_thread_local_flags(_TLF_RUNLATCH) {
        __hard_irq_disable();
        __ppc64_runlatch_off();
        if !(local_paca.irq_happened & PACA_IRQ_HARD_DIS) {
            __hard_irq_enable();
        }
    }
}

#[cfg(CONFIG_PPC64)]
#[inline(always)]
pub unsafe fn ppc64_runlatch_on() {
    if cpu_has_feature(CPU_FTR_CTRL) && !test_thread_local_flags(_TLF_RUNLATCH) {
        __hard_irq_disable();
        __ppc64_runlatch_on();
        if !(local_paca.irq_happened & PACA_IRQ_HARD_DIS) {
            __hard_irq_enable();
        }
    }
}

#[cfg(not(CONFIG_PPC64))]
#[inline(always)]
pub fn ppc64_runlatch_on() {}

#[cfg(not(CONFIG_PPC64))]
#[inline(always)]
pub fn ppc64_runlatch_off() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
