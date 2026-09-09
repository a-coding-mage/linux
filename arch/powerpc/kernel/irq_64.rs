// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of irq_64.c. */

pub static mut distribute_irqs: i32 = 1;

#[inline]
unsafe fn next_interrupt(regs: *mut pt_regs) {
    if cfg!(feature = "CONFIG_PPC_IRQ_SOFT_MASK_DEBUG") {
        WARN_ON(!(local_paca->irq_happened & PACA_IRQ_HARD_DIS));
        WARN_ON(irq_soft_mask_return() != IRQS_ALL_DISABLED);
    }
    lockdep_hardirq_exit();
    trace_hardirqs_on();
    trace_hardirqs_off();
    lockdep_hardirq_enter();
}

#[inline]
unsafe fn irq_happened_test_and_clear(irq: u8) -> bool {
    if local_paca->irq_happened & irq != 0 {
        local_paca->irq_happened &= !irq;
        true
    } else { false }
}

#[no_mangle]
pub unsafe fn __replay_soft_interrupts() {
    let mut regs: pt_regs = core::mem::zeroed();
    if cfg!(feature = "CONFIG_PPC_IRQ_SOFT_MASK_DEBUG") {
        WARN_ON_ONCE(mfmsr() & MSR_EE != 0);
        WARN_ON(!(local_paca->irq_happened & PACA_IRQ_HARD_DIS));
        WARN_ON(local_paca->irq_happened & PACA_IRQ_REPLAYING != 0);
    }
    local_paca->irq_happened |= PACA_IRQ_REPLAYING;
    ppc_save_regs(&mut regs);
    regs.softe = IRQS_ENABLED;
    regs.msr |= MSR_EE;
    if firmware_has_feature(FW_FEATURE_PS3_LV1) {
        let (mut tmp, mut tmp2) = (0u64, 0u64);
        lv1_get_version_info(&mut tmp, &mut tmp2);
    }
    if cfg!(feature = "CONFIG_PPC_BOOK3S") && irq_happened_test_and_clear(PACA_IRQ_HMI) {
        regs.trap = INTERRUPT_HMI; handle_hmi_exception(&mut regs); next_interrupt(&mut regs);
    }
    if irq_happened_test_and_clear(PACA_IRQ_DEC) {
        regs.trap = INTERRUPT_DECREMENTER; timer_interrupt(&mut regs); next_interrupt(&mut regs);
    }
    if irq_happened_test_and_clear(PACA_IRQ_EE) {
        regs.trap = INTERRUPT_EXTERNAL; do_IRQ(&mut regs); next_interrupt(&mut regs);
    }
    if cfg!(feature = "CONFIG_PPC_DOORBELL") && irq_happened_test_and_clear(PACA_IRQ_DBELL) {
        regs.trap = INTERRUPT_DOORBELL; doorbell_exception(&mut regs); next_interrupt(&mut regs);
    }
    if cfg!(feature = "CONFIG_PPC_BOOK3S") && irq_happened_test_and_clear(PACA_IRQ_PMI) {
        regs.trap = INTERRUPT_PERFMON; performance_monitor_exception(&mut regs); next_interrupt(&mut regs);
    }
    local_paca->irq_happened &= !PACA_IRQ_REPLAYING;
}

pub unsafe fn replay_soft_interrupts() {
    irq_enter(); __replay_soft_interrupts(); irq_exit();
}

#[cfg(all(feature = "CONFIG_PPC_BOOK3S_64", feature = "CONFIG_PPC_KUAP"))]
unsafe fn replay_soft_interrupts_irqrestore() {
    let kuap_state = get_kuap();
    kuap_assert_locked();
    if kuap_state != AMR_KUAP_BLOCKED { set_kuap(AMR_KUAP_BLOCKED); }
    __replay_soft_interrupts();
    if kuap_state != AMR_KUAP_BLOCKED { set_kuap(kuap_state); }
}
#[cfg(not(all(feature = "CONFIG_PPC_BOOK3S_64", feature = "CONFIG_PPC_KUAP")))]
unsafe fn replay_soft_interrupts_irqrestore() { __replay_soft_interrupts(); }

pub unsafe fn arch_local_irq_restore(mask: u64) {
    if mask != 0 { irq_soft_mask_set(mask); return; }
    if cfg!(feature = "CONFIG_PPC_IRQ_SOFT_MASK_DEBUG") {
        WARN_ON_ONCE(in_nmi()); WARN_ON_ONCE(in_hardirq());
        WARN_ON_ONCE(local_paca->irq_happened & PACA_IRQ_REPLAYING != 0);
    }
    'again: loop {
        // C asm goto atomically unmasks and detects a pending interrupt.
        if local_paca->irq_happened == 0 { local_paca->irq_soft_mask = 0; }
        else { break 'again; }
        if cfg!(feature = "CONFIG_PPC_IRQ_SOFT_MASK_DEBUG") { WARN_ON_ONCE(mfmsr() & MSR_EE == 0); }
        preempt_check_resched(); return;
    }
    let mut irq_happened = core::ptr::read_volatile(&local_paca->irq_happened);
    if irq_happened == PACA_IRQ_HARD_DIS {
        irq_soft_mask_set(IRQS_ENABLED); local_paca->irq_happened = 0; __hard_irq_enable(); preempt_check_resched(); return;
    }
    if irq_happened & PACA_IRQ_HARD_DIS == 0 {
        __hard_irq_disable(); local_paca->irq_happened |= PACA_IRQ_HARD_DIS;
    } else if cfg!(feature = "CONFIG_PPC_IRQ_SOFT_MASK_DEBUG") && mfmsr() & MSR_EE != 0 { __hard_irq_disable(); }
    preempt_disable(); irq_soft_mask_set(IRQS_ALL_DISABLED); trace_hardirqs_off(); irq_enter();
    replay_soft_interrupts_irqrestore(); irq_exit();
    if local_paca->irq_happened != PACA_IRQ_HARD_DIS { trace_hardirqs_on(); preempt_enable_no_resched(); continue; }
    trace_hardirqs_on(); irq_soft_mask_set(IRQS_ENABLED); local_paca->irq_happened = 0; __hard_irq_enable(); preempt_enable();
}

pub unsafe fn prep_irq_for_idle() -> bool {
    __hard_irq_disable(); local_paca->irq_happened |= PACA_IRQ_HARD_DIS;
    if lazy_irq_pending() { return false; }
    local_paca->irq_happened &= !PACA_IRQ_HARD_DIS; irq_soft_mask_set(IRQS_ENABLED); true
}

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
pub unsafe fn prep_irq_for_idle_irqsoff() -> bool {
    WARN_ON(!irqs_disabled()); __hard_irq_disable(); local_paca->irq_happened |= PACA_IRQ_HARD_DIS;
    if lazy_irq_pending() { return false; } trace_hardirqs_on(); true
}

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
pub const IRQ_SYSTEM_RESET: u8 = 0xff;
#[cfg(feature = "CONFIG_PPC_BOOK3S")]
pub static srr1_to_lazyirq: [u8; 16] = [0,0,0,PACA_IRQ_DBELL,IRQ_SYSTEM_RESET,PACA_IRQ_DBELL,PACA_IRQ_DEC,0,PACA_IRQ_EE,PACA_IRQ_EE,PACA_IRQ_HMI,0,0,0,0,0];

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
pub unsafe fn replay_system_reset() {
    let mut regs: pt_regs = core::mem::zeroed(); ppc_save_regs(&mut regs); regs.trap = 0x100;
    get_paca()->in_nmi = 1; system_reset_exception(&mut regs); get_paca()->in_nmi = 0;
}

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
pub unsafe fn irq_set_pending_from_srr1(srr1: u64) {
    let idx = ((srr1 & SRR1_WAKEMASK_P8) >> 18) as usize; let reason = srr1_to_lazyirq[idx];
    if reason == IRQ_SYSTEM_RESET { replay_system_reset(); return; }
    if reason == PACA_IRQ_DBELL { ppc_msgclr(PPC_DBELL_MSGTYPE); }
    local_paca->irq_happened |= reason;
}

pub unsafe fn force_external_irq_replay() {
    WARN_ON(!arch_irqs_disabled()); __hard_irq_disable(); local_paca->irq_happened |= PACA_IRQ_HARD_DIS; local_paca->irq_happened |= PACA_IRQ_EE;
}

unsafe fn setup_noirqdistrib(_str: *mut u8) -> i32 { distribute_irqs = 0; 1 }

// External kernel types, globals, constants, and functions are supplied by dependent translation units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
