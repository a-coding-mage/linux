/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm/entry-common.h. Required kernel symbols are supplied externally.

#[cfg(CONFIG_PPC_IRQ_SOFT_MASK_DEBUG)]
macro_rules! INT_SOFT_MASK_BUG_ON { ($regs:expr, $cond:expr) => {{
    if user_mode($regs) || TRAP($regs) != INTERRUPT_PROGRAM { BUG_ON($cond); }
}} }
#[cfg(not(CONFIG_PPC_IRQ_SOFT_MASK_DEBUG))]
macro_rules! INT_SOFT_MASK_BUG_ON { ($regs:expr, $cond:expr) => {{}} }

#[cfg(CONFIG_PPC_BOOK3S_64)]
extern "C" {
    static mut __end_soft_masked: u8;
    fn search_kernel_soft_mask_table(addr: c_ulong) -> bool;
    fn search_kernel_restart_table(addr: c_ulong) -> c_ulong;
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe fn is_implicit_soft_masked(regs: *mut pt_regs) -> bool {
    if user_mode(regs) || (*regs).nip >= (&__end_soft_masked as *const _ as c_ulong) { return false; }
    search_kernel_soft_mask_table((*regs).nip)
}
#[cfg(not(CONFIG_PPC_BOOK3S_64))]
unsafe fn search_kernel_restart_table(_addr: c_ulong) -> c_ulong { 0 }
#[cfg(not(CONFIG_PPC_BOOK3S_64))]
unsafe fn is_implicit_soft_masked(_regs: *mut pt_regs) -> bool { false }

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe fn srr_regs_clobbered() { (*local_paca).srr_valid = 0; (*local_paca).hsrr_valid = 0; }
#[cfg(not(CONFIG_PPC_BOOK3S_64))]
unsafe fn srr_regs_clobbered() {}

unsafe fn nap_adjust_return(regs: *mut pt_regs) {
    #[cfg(CONFIG_PPC_970_NAP)]
    if unlikely(test_thread_local_flags(_TLF_NAPPING)) {
        clear_thread_local_flags(_TLF_NAPPING);
        regs_set_return_ip(regs, power4_idle_nap_return as c_ulong);
    }
}

unsafe fn booke_load_dbcr0() {
    #[cfg(CONFIG_PPC_ADV_DEBUG_REGS)]
    { let dbcr0 = current->thread.debug.dbcr0;
      if likely(!(dbcr0 & DBCR0_IDM)) { return; }
      mtmsr(mfmsr() & !MSR_DE);
      if IS_ENABLED(CONFIG_PPC32) { isync(); global_dbcr0[smp_processor_id()] = mfspr(SPRN_DBCR0); }
      mtspr(SPRN_DBCR0, dbcr0); mtspr(SPRN_DBSR, -1); }
}
unsafe fn booke_restore_dbcr0() {
    #[cfg(CONFIG_PPC_ADV_DEBUG_REGS)]
    { let dbcr0 = current->thread.debug.dbcr0;
      if IS_ENABLED(CONFIG_PPC32) && unlikely(dbcr0 & DBCR0_IDM) { mtspr(SPRN_DBSR, -1); mtspr(SPRN_DBCR0, global_dbcr0[smp_processor_id()]); } }
}

unsafe fn check_return_regs_valid(regs: *mut pt_regs) {
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    { static mut warned: bool = false; let mut trap: c_ulong; let (srr0, srr1, h);
      if trap_is_scv(regs) { return; }
      trap = TRAP(regs); if cpu_has_feature(CPU_FTR_HVMODE) && trap == INTERRUPT_EXTERNAL { trap = 0xea0; }
      let validp: *mut u8;
      match trap { 0x980 | INTERRUPT_H_DATA_STORAGE | 0xe20 | 0xe40 | INTERRUPT_HMI | 0xe80 | 0xea0 | INTERRUPT_H_FAC_UNAVAIL | 0x1200 | 0x1500 | 0x1600 | 0x1800 => { validp = &mut (*local_paca).hsrr_valid; if !READ_ONCE(*validp) { return; } srr0=mfspr(SPRN_HSRR0); srr1=mfspr(SPRN_HSRR1); h="H"; }, _ => { validp=&mut (*local_paca).srr_valid; if !READ_ONCE(*validp) { return; } srr0=mfspr(SPRN_SRR0); srr1=mfspr(SPRN_SRR1); h=""; } }
      if srr0 == (*regs).nip && srr1 == (*regs).msr { return; }
      if !READ_ONCE(*validp) { return; }
      if !data_race(warned) { data_race(warned=true); pr_warn("%sSRR0 was: %lx should be: %lx\n",h,srr0,(*regs).nip); pr_warn("%sSRR1 was: %lx should be: %lx\n",h,srr1,(*regs).msr); show_regs(regs); }
      WRITE_ONCE(*validp, 0); }
}

unsafe fn arch_interrupt_enter_prepare(regs: *mut pt_regs) {
    #[cfg(CONFIG_PPC64)] { irq_soft_mask_set(IRQS_ALL_DISABLED); if !((*local_paca).irq_happened & PACA_IRQ_HARD_DIS) { INT_SOFT_MASK_BUG_ON!(regs, !((*regs).msr & MSR_EE)); __hard_irq_enable(); } else { __hard_RI_enable(); } }
    if !regs_irqs_disabled(regs) { trace_hardirqs_off(); }
    if user_mode(regs) { kuap_lock(); account_cpu_user_entry(); account_stolen_time(); } else { kuap_save_and_lock(regs); if TRAP(regs) != INTERRUPT_PROGRAM { CT_WARN_ON(ct_state()!=CT_STATE_KERNEL && ct_state()!=CT_STATE_IDLE); } INT_SOFT_MASK_BUG_ON!(regs,is_implicit_soft_masked(regs)); INT_SOFT_MASK_BUG_ON!(regs,regs_irqs_disabled(regs) && search_kernel_restart_table((*regs).nip)); }
    INT_SOFT_MASK_BUG_ON!(regs,!regs_irqs_disabled(regs) && !((*regs).msr & MSR_EE)); booke_restore_dbcr0();
}

unsafe fn arch_interrupt_exit_prepare(regs: *mut pt_regs) { if user_mode(regs) { BUG_ON(regs_is_unrecoverable(regs)); BUG_ON(regs_irqs_disabled(regs)); kuap_assert_locked(); } local_irq_disable(); }
unsafe fn arch_interrupt_async_enter_prepare(regs: *mut pt_regs) { #[cfg(CONFIG_PPC64)] { (*local_paca).irq_happened |= PACA_IRQ_HARD_DIS; } arch_interrupt_enter_prepare(regs); #[cfg(CONFIG_PPC_BOOK3S_64)] if cpu_has_feature(CPU_FTR_CTRL) && !test_thread_local_flags(_TLF_RUNLATCH) { __ppc64_runlatch_on(); } }
unsafe fn arch_interrupt_async_exit_prepare(regs: *mut pt_regs) { arch_interrupt_exit_prepare(regs); }

#[repr(C)] pub struct interrupt_nmi_state { #[cfg(CONFIG_PPC64)] pub irq_soft_mask:u8, #[cfg(CONFIG_PPC64)] pub irq_happened:u8, #[cfg(CONFIG_PPC64)] pub ftrace_enabled:u8, #[cfg(CONFIG_PPC64)] pub softe:u64 }
unsafe fn nmi_disables_ftrace(regs:*mut pt_regs)->bool { if IS_ENABLED(CONFIG_PPC_BOOK3S_64) && (TRAP(regs)==INTERRUPT_DECREMENTER || TRAP(regs)==INTERRUPT_PERFMON) { return false; } if IS_ENABLED(CONFIG_PPC_BOOK3E_64) && TRAP(regs)==INTERRUPT_PERFMON { return false; } true }

unsafe fn arch_interrupt_nmi_enter_prepare(regs:*mut pt_regs, state:*mut interrupt_nmi_state) {
    #[cfg(CONFIG_PPC64)] {
        (*state).irq_soft_mask=(*local_paca).irq_soft_mask; (*state).irq_happened=(*local_paca).irq_happened; (*state).softe=(*regs).softe;
        (*local_paca).irq_soft_mask=IRQS_ALL_DISABLED; (*local_paca).irq_happened|=PACA_IRQ_HARD_DIS;
        if !((*regs).msr&MSR_EE)!=0 || is_implicit_soft_masked(regs) { (*regs).softe=IRQS_ALL_DISABLED; }
        __hard_RI_enable();
        if nmi_disables_ftrace(regs) { (*state).ftrace_enabled=this_cpu_get_ftrace_enabled(); this_cpu_set_ftrace_enabled(0); }
    }
}
unsafe fn arch_interrupt_nmi_exit_prepare(regs:*mut pt_regs, state:*mut interrupt_nmi_state) {
    #[cfg(CONFIG_PPC64)] {
        #[cfg(CONFIG_PPC_BOOK3S)] if regs_irqs_disabled(regs) { let rst=search_kernel_restart_table((*regs).nip); if rst!=0 {regs_set_return_ip(regs,rst);} }
        if nmi_disables_ftrace(regs) {this_cpu_set_ftrace_enabled((*state).ftrace_enabled);}
        WARN_ON_ONCE(((*state).irq_happened|PACA_IRQ_HARD_DIS)!=(*local_paca).irq_happened);
        (*regs).softe=(*state).softe; (*local_paca).irq_happened=(*state).irq_happened; (*local_paca).irq_soft_mask=(*state).irq_soft_mask;
    }
}

// The remaining entry/exit helpers retain the C implementation's conditional kernel behavior.
unsafe fn arch_enter_from_user_mode(regs:*mut pt_regs) { kuap_lock(); if IS_ENABLED(CONFIG_PPC_IRQ_SOFT_MASK_DEBUG) { BUG_ON(irq_soft_mask_return()!=IRQS_ALL_DISABLED); } BUG_ON(regs_is_unrecoverable(regs)); BUG_ON(!user_mode(regs)); BUG_ON(regs_irqs_disabled(regs)); #[cfg(CONFIG_PPC_PKEY)] if mmu_has_feature(MMU_FTR_PKEY) && trap_is_syscall(regs) { (*regs).amr=mfspr(SPRN_AMR); (*regs).iamr=mfspr(SPRN_IAMR); if mmu_has_feature(MMU_FTR_KUAP){mtspr(SPRN_AMR,AMR_KUAP_BLOCKED);isync();} if mmu_has_feature(MMU_FTR_BOOK3S_KUEP){mtspr(SPRN_IAMR,AMR_KUEP_BLOCKED);isync();} } kuap_assert_locked(); booke_restore_dbcr0(); account_cpu_user_entry(); account_stolen_time(); irq_soft_mask_regs_set_state(regs,IRQS_ENABLED); }
unsafe fn arch_exit_to_user_mode_prepare(regs:*mut pt_regs, ti_work:c_ulong) { if IS_ENABLED(CONFIG_PPC_BOOK3S_64)&&IS_ENABLED(CONFIG_PPC_FPU) { if IS_ENABLED(CONFIG_PPC_TRANSACTIONAL_MEM)&&unlikely(ti_work&_TIF_RESTORE_TM){restore_tm_state(regs);} else { let mut mathflags=MSR_FP; if cpu_has_feature(CPU_FTR_VSX){mathflags|=MSR_VEC|MSR_VSX;} else if cpu_has_feature(CPU_FTR_ALTIVEC){mathflags|=MSR_VEC;} if (*regs).msr&mathflags!=mathflags {restore_math(regs);} } } check_return_regs_valid(regs); kuap_user_restore(regs); }
unsafe fn arch_exit_to_user_mode() { booke_load_dbcr0(); account_cpu_user_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
