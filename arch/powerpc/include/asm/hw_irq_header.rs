/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu> */

/* Translated from the PowerPC Linux kernel header. Configuration-dependent
 * C preprocessor branches are represented with Rust cfg attributes where
 * possible; referenced architecture symbols are supplied by other modules.
 */

#[cfg(feature = "CONFIG_PPC64")]
pub const PACA_IRQ_HARD_DIS: u8 = 0x01;
#[cfg(feature = "CONFIG_PPC64")]
pub const PACA_IRQ_DBELL: u8 = 0x02;
#[cfg(feature = "CONFIG_PPC64")]
pub const PACA_IRQ_EE: u8 = 0x04;
#[cfg(feature = "CONFIG_PPC64")]
pub const PACA_IRQ_DEC: u8 = 0x08; /* Or FIT */
#[cfg(feature = "CONFIG_PPC64")]
pub const PACA_IRQ_HMI: u8 = 0x10;
#[cfg(feature = "CONFIG_PPC64")]
pub const PACA_IRQ_PMI: u8 = 0x20;
#[cfg(feature = "CONFIG_PPC64")]
pub const PACA_IRQ_REPLAYING: u8 = 0x40;

#[cfg(all(feature = "CONFIG_PPC64", feature = "CONFIG_PPC_BOOK3S"))]
pub const PACA_IRQ_MUST_HARD_MASK: u8 = PACA_IRQ_EE | PACA_IRQ_PMI | PACA_IRQ_REPLAYING;
#[cfg(all(feature = "CONFIG_PPC64", not(feature = "CONFIG_PPC_BOOK3S")))]
pub const PACA_IRQ_MUST_HARD_MASK: u8 = PACA_IRQ_EE | PACA_IRQ_REPLAYING;

pub const IRQS_ENABLED: usize = 0;
pub const IRQS_DISABLED: usize = 1;
pub const IRQS_PMI_DISABLED: usize = 2;
pub const IRQS_ALL_DISABLED: usize = IRQS_DISABLED | IRQS_PMI_DISABLED;

pub unsafe fn __hard_irq_enable() {
    if cfg!(feature = "CONFIG_BOOKE") { wrtee(MSR_EE); }
    else if cfg!(feature = "CONFIG_PPC_8xx") { wrtspr(SPRN_EIE); }
    else if cfg!(feature = "CONFIG_PPC_BOOK3S_64") { __mtmsrd(MSR_EE | MSR_RI, 1); }
    else { mtmsr(mfmsr() | MSR_EE); }
}

pub unsafe fn __hard_irq_disable() {
    if cfg!(feature = "CONFIG_BOOKE") { wrtee(0); }
    else if cfg!(feature = "CONFIG_PPC_8xx") { wrtspr(SPRN_EID); }
    else if cfg!(feature = "CONFIG_PPC_BOOK3S_64") { __mtmsrd(MSR_RI, 1); }
    else { mtmsr(mfmsr() & !MSR_EE); }
}

pub unsafe fn __hard_EE_RI_disable() {
    if cfg!(feature = "CONFIG_BOOKE") { wrtee(0); }
    else if cfg!(feature = "CONFIG_PPC_8xx") { wrtspr_sync(SPRN_NRI); }
    else if cfg!(feature = "CONFIG_PPC_BOOK3S_64") { __mtmsrd(0, 1); }
    else { mtmsr(mfmsr() & !(MSR_EE | MSR_RI)); }
}

pub unsafe fn __hard_RI_enable() {
    if cfg!(feature = "CONFIG_BOOKE") { return; }
    if cfg!(feature = "CONFIG_PPC_8xx") { wrtspr(SPRN_EID); }
    else if cfg!(feature = "CONFIG_PPC_BOOK3S_64") { __mtmsrd(MSR_RI, 1); }
    else { mtmsr(mfmsr() | MSR_RI); }
}

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn irq_soft_mask_return() -> usize { get_paca().irq_soft_mask as usize }

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn irq_soft_mask_set(mask: usize) {
    if cfg!(feature = "CONFIG_PPC_IRQ_SOFT_MASK_DEBUG") { WARN_ON(mask != 0 && (mask & IRQS_DISABLED) == 0); }
    get_paca().irq_soft_mask = mask as _;
}

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn irq_soft_mask_set_return(mask: usize) -> usize { let flags = irq_soft_mask_return(); irq_soft_mask_set(mask); flags }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn irq_soft_mask_or_return(mask: usize) -> usize { let flags = irq_soft_mask_return(); irq_soft_mask_set(flags | mask); flags }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn irq_soft_mask_andc_return(mask: usize) -> usize { let flags = irq_soft_mask_return(); irq_soft_mask_set(flags & !mask); flags }

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn arch_local_save_flags() -> usize { irq_soft_mask_return() }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn arch_local_irq_disable() { irq_soft_mask_set(IRQS_DISABLED); }
extern "C" { pub fn arch_local_irq_restore(flags: usize); }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn arch_local_irq_enable() { arch_local_irq_restore(IRQS_ENABLED); }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn arch_local_irq_save() -> usize { irq_soft_mask_or_return(IRQS_DISABLED) }
#[cfg(feature = "CONFIG_PPC64")]
pub fn arch_irqs_disabled_flags(flags: usize) -> bool { flags & IRQS_DISABLED != 0 }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn arch_irqs_disabled() -> bool { arch_irqs_disabled_flags(arch_local_save_flags()) }

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn set_pmi_irq_pending() { get_paca().irq_happened |= PACA_IRQ_PMI; }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn clear_pmi_irq_pending() { get_paca().irq_happened &= !PACA_IRQ_PMI; }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn pmi_irq_pending() -> bool { get_paca().irq_happened & PACA_IRQ_PMI != 0 }

extern "C" { pub fn power_pmu_wants_prompt_pmi() -> bool; }

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn __lazy_irq_pending(irq_happened: u8) -> bool { irq_happened & !PACA_IRQ_HARD_DIS != 0 }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn lazy_irq_pending() -> bool { __lazy_irq_pending(get_paca().irq_happened) }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn lazy_irq_pending_nocheck() -> bool { __lazy_irq_pending(local_paca.irq_happened) }

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn should_hard_irq_enable(regs: *mut pt_regs) -> bool {
    if !cfg!(feature = "CONFIG_PERF_EVENTS") { return false; }
    if cfg!(feature = "CONFIG_PPC_BOOK3S_64") {
        if !power_pmu_wants_prompt_pmi() { return false; }
        if ((*regs).softe & IRQS_PMI_DISABLED) != 0 { return false; }
    }
    get_paca().irq_happened & PACA_IRQ_MUST_HARD_MASK == 0
}

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn do_hard_irq_enable() {
    if cfg!(feature = "CONFIG_PPC_BOOK3S_64") { irq_soft_mask_andc_return(IRQS_PMI_DISABLED); }
    get_paca().irq_happened &= !PACA_IRQ_HARD_DIS;
    __hard_irq_enable();
}

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn regs_irqs_disabled(regs: *mut pt_regs) -> bool { ((*regs).softe & IRQS_DISABLED) != 0 }
extern "C" { pub fn prep_irq_for_idle() -> bool; pub fn prep_irq_for_idle_irqsoff() -> bool; pub fn irq_set_pending_from_srr1(srr1: usize); pub fn force_external_irq_replay(); }
pub unsafe fn fini_irq_for_idle_irqsoff() { trace_hardirqs_off(); }
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn irq_soft_mask_regs_set_state(regs: *mut pt_regs, val: usize) { (*regs).softe = val; }

#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn irq_soft_mask_return() -> usize { 0 }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn arch_local_save_flags() -> usize { mfmsr() }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn arch_local_irq_restore(flags: usize) { if cfg!(feature = "CONFIG_BOOKE") { wrtee(flags); } else { mtmsr(flags); } }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn arch_local_irq_save() -> usize { let flags = arch_local_save_flags(); if cfg!(feature = "CONFIG_BOOKE") { wrtee(0); } else if cfg!(feature = "CONFIG_PPC_8xx") { wrtspr(SPRN_EID); } else { mtmsr(flags & !MSR_EE); } flags }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn arch_local_irq_disable() { __hard_irq_disable(); }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn arch_local_irq_enable() { __hard_irq_enable(); }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub fn arch_irqs_disabled_flags(flags: usize) -> bool { flags & MSR_EE == 0 }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn arch_irqs_disabled() -> bool { arch_irqs_disabled_flags(arch_local_save_flags()) }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn hard_irq_disable() { arch_local_irq_disable(); }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn regs_irqs_disabled(regs: *mut pt_regs) -> bool { ((*regs).msr & MSR_EE) == 0 }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn should_hard_irq_enable(_regs: *mut pt_regs) -> bool { false }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn do_hard_irq_enable() { BUILD_BUG(); }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn clear_pmi_irq_pending() {}
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn set_pmi_irq_pending() {}
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn pmi_irq_pending() -> bool { false }
#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn irq_soft_mask_regs_set_state(_regs: *mut pt_regs, _val: usize) {}

pub unsafe fn mtmsr_isync_irqsafe(mut msr: usize) -> usize {
    #[cfg(feature = "CONFIG_PPC64")]
    if arch_irqs_disabled() { msr &= !MSR_EE; mtmsr_isync(msr); irq_soft_mask_set(IRQS_ALL_DISABLED); local_paca.irq_happened |= PACA_IRQ_HARD_DIS; }
    #[cfg(not(feature = "CONFIG_PPC64"))]
    mtmsr_isync(msr);
    msr
}

pub const ARCH_IRQ_INIT_FLAGS: usize = IRQ_NOREQUEST;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
