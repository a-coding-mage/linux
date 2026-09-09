// SPDX-License-Identifier: GPL-2.0
/* Exception handling code; translated from entry-common.c. */

// Kernel-provided types, constants, functions, macros, and configuration symbols
// are intentionally referenced here as external dependencies.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn arm64_enter_from_kernel_mode(regs: *mut pt_regs) -> irqentry_state_t {
    let state = irqentry_enter_from_kernel_mode(regs); mte_check_tfsr_entry(); mte_disable_tco_entry(current); state
}
pub unsafe fn __arm64_exit_to_kernel_mode(regs: *mut pt_regs, state: irqentry_state_t) {
    local_daif_mask(); mte_check_tfsr_exit(); irqentry_exit_to_kernel_mode_after_preempt(regs, state);
}
pub unsafe fn arm64_exit_to_kernel_mode_preempt(regs: *mut pt_regs, state: irqentry_state_t) {
    irqentry_exit_to_kernel_mode_preempt(regs, state); __arm64_exit_to_kernel_mode(regs, state);
}
pub unsafe fn arm64_exit_to_kernel_mode(regs: *mut pt_regs, state: irqentry_state_t) {
    if !regs_irqs_disabled(regs) { local_irq_disable(); arm64_exit_to_kernel_mode_preempt(regs, state); return; }
    __arm64_exit_to_kernel_mode(regs, state);
}
pub unsafe fn arm64_syscall_enter_from_user_mode(regs: *mut pt_regs) { enter_from_user_mode(regs); mte_disable_tco_entry(current); sme_enter_from_user_mode(); }
pub unsafe fn arm64_enter_from_user_mode(regs: *mut pt_regs) { enter_from_user_mode(regs); rseq_note_user_irq_entry(); mte_disable_tco_entry(current); sme_enter_from_user_mode(); }
pub unsafe fn arm64_syscall_exit_to_user_mode(regs: *mut pt_regs) { local_irq_disable(); syscall_exit_to_user_mode_prepare(regs); local_daif_mask(); sme_exit_to_user_mode(); mte_check_tfsr_exit(); exit_to_user_mode(); }
pub unsafe fn arm64_exit_to_user_mode(regs: *mut pt_regs) { local_irq_disable(); irqentry_exit_to_user_mode_prepare(regs); local_daif_mask(); sme_exit_to_user_mode(); mte_check_tfsr_exit(); exit_to_user_mode(); }
pub unsafe extern "C" fn asm_exit_to_user_mode(regs: *mut pt_regs) { arm64_syscall_exit_to_user_mode(regs); }

pub unsafe fn arm64_enter_el1_dbg(_regs: *mut pt_regs) -> irqentry_state_t {
    let mut state = irqentry_state_t { lockdep: lockdep_hardirqs_enabled() };
    lockdep_hardirqs_off(CALLER_ADDR0); ct_nmi_enter(); trace_hardirqs_off_finish(); state
}
pub unsafe fn arm64_exit_el1_dbg(_regs: *mut pt_regs, state: irqentry_state_t) { if state.lockdep { trace_hardirqs_on_prepare(); lockdep_hardirqs_on_prepare(); } ct_nmi_exit(); if state.lockdep { lockdep_hardirqs_on(CALLER_ADDR0); } }
pub unsafe fn do_interrupt_handler(regs: *mut pt_regs, handler: unsafe extern "C" fn(*mut pt_regs)) { let old_regs = set_irq_regs(regs); if on_thread_stack() { call_on_irq_stack(regs, handler); } else { handler(regs); } set_irq_regs(old_regs); }
extern "C" { pub static mut handle_arch_irq: unsafe extern "C" fn(*mut pt_regs); pub static mut handle_arch_fiq: unsafe extern "C" fn(*mut pt_regs); }
pub unsafe fn __panic_unhandled(regs: *mut pt_regs, vector: *const u8, esr: u64) { irqentry_nmi_enter(regs); console_verbose(); pr_crit(vector, smp_processor_id(), esr, esr_get_class_string(esr)); __show_regs(regs); panic("Unhandled exception"); }

#[cfg(feature = "CONFIG_ARM64_ERRATUM_1463225")]
pub unsafe fn cortex_a76_erratum_1463225_svc_handler() { if !unlikely(test_thread_flag(TIF_SINGLESTEP)) || !unlikely(this_cpu_has_cap(ARM64_WORKAROUND_1463225)) { return; } __this_cpu_write(__in_cortex_a76_erratum_1463225_wa, 1); let reg = read_sysreg(mdscr_el1); write_sysreg(reg | MDSCR_EL1_SS | MDSCR_EL1_KDE, mdscr_el1); asm!("msr daifclr, #8"); isb(); write_sysreg(reg, mdscr_el1); __this_cpu_write(__in_cortex_a76_erratum_1463225_wa, 0); }
#[cfg(not(feature = "CONFIG_ARM64_ERRATUM_1463225"))] pub unsafe fn cortex_a76_erratum_1463225_svc_handler() {}
pub unsafe fn cortex_a76_erratum_1463225_debug_handler(regs: *mut pt_regs) -> bool { #[cfg(feature="CONFIG_ARM64_ERRATUM_1463225")] { if !__this_cpu_read(__in_cortex_a76_erratum_1463225_wa) { return false; } (*regs).pstate |= PSR_D_BIT; return true; } #[cfg(not(feature="CONFIG_ARM64_ERRATUM_1463225"))] { let _ = regs; false } }

pub unsafe fn fpsimd_syscall_enter() { if system_supports_sme() { sme_smstop_sm(); } if !system_supports_sve() { return; } if test_thread_flag(TIF_SVE) { sve_flush_live(); } __this_cpu_write(fpsimd_last_state.to_save, FP_STATE_FPSIMD); }
pub unsafe fn fpsimd_syscall_exit() { if system_supports_sve() { __this_cpu_write(fpsimd_last_state.to_save, FP_STATE_CURRENT); } }
pub unsafe fn debug_exception_enter(_regs: *mut pt_regs) { preempt_disable(); RCU_LOCKDEP_WARN(!rcu_is_watching(), "exception_enter didn't work"); }
pub unsafe fn debug_exception_exit(_regs: *mut pt_regs) { preempt_enable_no_resched(); }

macro_rules! kernel_abort { ($name:ident, $call:ident) => { pub unsafe fn $name(regs:*mut pt_regs, esr:u64) { let far=read_sysreg(far_el1); let state=arm64_enter_from_kernel_mode(regs); local_daif_inherit(regs); $call(far, esr, regs); arm64_exit_to_kernel_mode(regs,state); } }; }
kernel_abort!(el1_abort, do_mem_abort); kernel_abort!(el1_pc, do_sp_pc_abort);
pub unsafe fn el1_undef(regs:*mut pt_regs, esr:u64){let s=arm64_enter_from_kernel_mode(regs);local_daif_inherit(regs);do_el1_undef(regs,esr);arm64_exit_to_kernel_mode(regs,s)}
pub unsafe fn el1_bti(regs:*mut pt_regs,esr:u64){let s=arm64_enter_from_kernel_mode(regs);local_daif_inherit(regs);do_el1_bti(regs,esr);arm64_exit_to_kernel_mode(regs,s)}
pub unsafe fn el1_gcs(regs:*mut pt_regs,esr:u64){let s=arm64_enter_from_kernel_mode(regs);local_daif_inherit(regs);do_el1_gcs(regs,esr);arm64_exit_to_kernel_mode(regs,s)}
pub unsafe fn el1_mops(regs:*mut pt_regs,esr:u64){let s=arm64_enter_from_kernel_mode(regs);local_daif_inherit(regs);do_el1_mops(regs,esr);arm64_exit_to_kernel_mode(regs,s)}
pub unsafe fn el1_fpac(regs:*mut pt_regs,esr:u64){let s=arm64_enter_from_kernel_mode(regs);local_daif_inherit(regs);do_el1_fpac(regs,esr);arm64_exit_to_kernel_mode(regs,s)}

pub unsafe fn el1_breakpt(regs:*mut pt_regs,esr:u64){let s=arm64_enter_el1_dbg(regs);debug_exception_enter(regs);do_breakpoint(esr,regs);debug_exception_exit(regs);arm64_exit_el1_dbg(regs,s)}
pub unsafe fn el1_softstp(regs:*mut pt_regs,esr:u64){let s=arm64_enter_el1_dbg(regs);if !cortex_a76_erratum_1463225_debug_handler(regs){debug_exception_enter(regs);if !try_step_suspended_breakpoints(regs){do_el1_softstep(esr,regs)}debug_exception_exit(regs)}arm64_exit_el1_dbg(regs,s)}
pub unsafe fn el1_watchpt(regs:*mut pt_regs,esr:u64){let far=read_sysreg(far_el1);let s=arm64_enter_el1_dbg(regs);debug_exception_enter(regs);do_watchpoint(far,esr,regs);debug_exception_exit(regs);arm64_exit_el1_dbg(regs,s)}
pub unsafe fn el1_brk64(regs:*mut pt_regs,esr:u64){let s=arm64_enter_el1_dbg(regs);debug_exception_enter(regs);do_el1_brk64(esr,regs);debug_exception_exit(regs);arm64_exit_el1_dbg(regs,s)}

// Remaining exception entry points retain the C dispatch structure and external kernel calls.
pub unsafe extern "C" fn el1h_64_sync_handler(regs:*mut pt_regs){let esr=read_sysreg(esr_el1);match ESR_ELx_EC(esr){ESR_ELx_EC_DABT_CUR|ESR_ELx_EC_IABT_CUR=>el1_abort(regs,esr),ESR_ELx_EC_PC_ALIGN=>el1_pc(regs,esr),ESR_ELx_EC_SYS64|ESR_ELx_EC_UNKNOWN=>el1_undef(regs,esr),ESR_ELx_EC_BTI=>el1_bti(regs,esr),ESR_ELx_EC_GCS=>el1_gcs(regs,esr),ESR_ELx_EC_MOPS=>el1_mops(regs,esr),ESR_ELx_EC_BREAKPT_CUR=>el1_breakpt(regs,esr),ESR_ELx_EC_SOFTSTP_CUR=>el1_softstp(regs,esr),ESR_ELx_EC_WATCHPT_CUR=>el1_watchpt(regs,esr),ESR_ELx_EC_BRK64=>el1_brk64(regs,esr),ESR_ELx_EC_FPAC=>el1_fpac(regs,esr),_=>__panic_unhandled(regs,b"64-bit el1h sync\\0".as_ptr(),esr)}}

pub unsafe fn __el1_irq(regs:*mut pt_regs,handler:unsafe extern "C" fn(*mut pt_regs)){let s=arm64_enter_from_kernel_mode(regs);irq_enter_rcu();do_interrupt_handler(regs,handler);irq_exit_rcu();arm64_exit_to_kernel_mode_preempt(regs,s)}
pub unsafe fn __el1_pnmi(regs:*mut pt_regs,handler:unsafe extern "C" fn(*mut pt_regs)){let s=irqentry_nmi_enter(regs);do_interrupt_handler(regs,handler);local_daif_mask();irqentry_nmi_exit(regs,s)}
pub unsafe fn el1_interrupt(regs:*mut pt_regs,handler:unsafe extern "C" fn(*mut pt_regs)){write_sysreg(DAIF_PROCCTX_NOIRQ,daif);if IS_ENABLED(CONFIG_ARM64_PSEUDO_NMI)&&regs_irqs_disabled(regs){__el1_pnmi(regs,handler)}else{__el1_irq(regs,handler)}}
pub unsafe extern "C" fn el1h_64_irq_handler(r:*mut pt_regs){el1_interrupt(r,handle_arch_irq)} pub unsafe extern "C" fn el1h_64_fiq_handler(r:*mut pt_regs){el1_interrupt(r,handle_arch_fiq)}
pub unsafe extern "C" fn el1h_64_error_handler(r:*mut pt_regs){let e=read_sysreg(esr_el1);local_daif_restore(DAIF_ERRCTX);let s=irqentry_nmi_enter(r);do_serror(r,e);local_daif_mask();irqentry_nmi_exit(r,s)}

macro_rules! user_handler { ($name:ident,$call:ident) => { pub unsafe fn $name(r:*mut pt_regs,e:u64){arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);$call(e,r);arm64_exit_to_user_mode(r)} }; }
user_handler!(el0_fpsimd_acc,do_fpsimd_acc); user_handler!(el0_sve_acc,do_sve_acc); user_handler!(el0_sme_acc,do_sme_acc); user_handler!(el0_fpsimd_exc,do_fpsimd_exc); user_handler!(el0_sys,do_el0_sys); user_handler!(el0_undef,do_el0_undef); user_handler!(el0_mops,do_el0_mops); user_handler!(el0_gcs,do_el0_gcs); user_handler!(el0_fpac,do_el0_fpac);
pub unsafe fn el0_da(r:*mut pt_regs,e:u64){let f=read_sysreg(far_el1);arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);do_mem_abort(f,e,r);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_ia(r:*mut pt_regs,e:u64){let f=read_sysreg(far_el1);if !is_ttbr0_addr(f){arm64_apply_bp_hardening()}el0_da(r,e)}
pub unsafe fn el0_pc(r:*mut pt_regs,e:u64){let f=read_sysreg(far_el1);if !is_ttbr0_addr(instruction_pointer(r)){arm64_apply_bp_hardening()}arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);do_sp_pc_abort(f,e,r);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_sp(r:*mut pt_regs,e:u64){arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);do_sp_pc_abort((*r).sp,e,r);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_bti(r:*mut pt_regs){arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);do_el0_bti(r);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_inv(r:*mut pt_regs,e:u64){arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);bad_el0_sync(r,0,e);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_breakpt(r:*mut pt_regs,e:u64){if !is_ttbr0_addr((*r).pc){arm64_apply_bp_hardening()}arm64_enter_from_user_mode(r);debug_exception_enter(r);do_breakpoint(e,r);debug_exception_exit(r);local_daif_restore(DAIF_PROCCTX);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_watchpt(r:*mut pt_regs,e:u64){let f=read_sysreg(far_el1);arm64_enter_from_user_mode(r);debug_exception_enter(r);do_watchpoint(f,e,r);debug_exception_exit(r);local_daif_restore(DAIF_PROCCTX);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_brk64(r:*mut pt_regs,e:u64){arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);do_el0_brk64(e,r);arm64_exit_to_user_mode(r)}
pub unsafe fn el0_softstp(r:*mut pt_regs,e:u64){if !is_ttbr0_addr((*r).pc){arm64_apply_bp_hardening()}arm64_enter_from_user_mode(r);let done=try_step_suspended_breakpoints(r);local_daif_restore(DAIF_PROCCTX);if !done{do_el0_softstep(e,r)}arm64_exit_to_user_mode(r)}
pub unsafe fn el0_svc(r:*mut pt_regs){arm64_syscall_enter_from_user_mode(r);cortex_a76_erratum_1463225_svc_handler();fpsimd_syscall_enter();local_daif_restore(DAIF_PROCCTX);do_el0_svc(r);arm64_syscall_exit_to_user_mode(r);fpsimd_syscall_exit()}

pub unsafe extern "C" fn el0t_64_sync_handler(r:*mut pt_regs){let e=read_sysreg(esr_el1);match ESR_ELx_EC(e){ESR_ELx_EC_SVC64=>el0_svc(r),ESR_ELx_EC_DABT_LOW=>el0_da(r,e),ESR_ELx_EC_IABT_LOW=>el0_ia(r,e),ESR_ELx_EC_FP_ASIMD=>el0_fpsimd_acc(r,e),ESR_ELx_EC_SVE=>el0_sve_acc(r,e),ESR_ELx_EC_SME=>el0_sme_acc(r,e),ESR_ELx_EC_FP_EXC64=>el0_fpsimd_exc(r,e),ESR_ELx_EC_SYS64|ESR_ELx_EC_WFx=>el0_sys(r,e),ESR_ELx_EC_SP_ALIGN=>el0_sp(r,e),ESR_ELx_EC_PC_ALIGN=>el0_pc(r,e),ESR_ELx_EC_UNKNOWN=>el0_undef(r,e),ESR_ELx_EC_BTI=>el0_bti(r),ESR_ELx_EC_MOPS=>el0_mops(r,e),ESR_ELx_EC_GCS=>el0_gcs(r,e),ESR_ELx_EC_BREAKPT_LOW=>el0_breakpt(r,e),ESR_ELx_EC_SOFTSTP_LOW=>el0_softstp(r,e),ESR_ELx_EC_WATCHPT_LOW=>el0_watchpt(r,e),ESR_ELx_EC_BRK64=>el0_brk64(r,e),ESR_ELx_EC_FPAC=>el0_fpac(r,e),_=>el0_inv(r,e)}}
pub unsafe fn el0_interrupt(r:*mut pt_regs,h:unsafe extern "C" fn(*mut pt_regs)){arm64_enter_from_user_mode(r);write_sysreg(DAIF_PROCCTX_NOIRQ,daif);if (*r).pc & BIT(55)!=0{arm64_apply_bp_hardening()}irq_enter_rcu();do_interrupt_handler(r,h);irq_exit_rcu();arm64_exit_to_user_mode(r)}
pub unsafe extern "C" fn el0t_64_irq_handler(r:*mut pt_regs){el0_interrupt(r,handle_arch_irq)} pub unsafe extern "C" fn el0t_64_fiq_handler(r:*mut pt_regs){el0_interrupt(r,handle_arch_fiq)}
pub unsafe extern "C" fn el0t_64_error_handler(r:*mut pt_regs){let e=read_sysreg(esr_el1);arm64_enter_from_user_mode(r);local_daif_restore(DAIF_ERRCTX);let s=irqentry_nmi_enter(r);do_serror(r,e);irqentry_nmi_exit(r,s);local_daif_restore(DAIF_PROCCTX);arm64_exit_to_user_mode(r)}
pub unsafe extern "C" fn handle_bad_stack(r:*mut pt_regs)->!{let e=read_sysreg(esr_el1);let f=read_sysreg(far_el1);irqentry_nmi_enter(r);panic_bad_stack(r,e,f)}

#[cfg(feature="CONFIG_COMPAT")]
pub unsafe fn el0_cp15(r:*mut pt_regs,e:u64){arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);do_el0_cp15(e,r);arm64_exit_to_user_mode(r)}
#[cfg(feature="CONFIG_COMPAT")]
pub unsafe fn el0_svc_compat(r:*mut pt_regs){arm64_syscall_enter_from_user_mode(r);cortex_a76_erratum_1463225_svc_handler();local_daif_restore(DAIF_PROCCTX);do_el0_svc_compat(r);arm64_syscall_exit_to_user_mode(r)}
#[cfg(feature="CONFIG_COMPAT")]
pub unsafe fn el0_bkpt32(r:*mut pt_regs,e:u64){arm64_enter_from_user_mode(r);local_daif_restore(DAIF_PROCCTX);do_bkpt32(e,r);arm64_exit_to_user_mode(r)}
#[cfg(feature="CONFIG_COMPAT")]
pub unsafe extern "C" fn el0t_32_sync_handler(r:*mut pt_regs){let e=read_sysreg(esr_el1);match ESR_ELx_EC(e){ESR_ELx_EC_SVC32=>el0_svc_compat(r),ESR_ELx_EC_DABT_LOW=>el0_da(r,e),ESR_ELx_EC_IABT_LOW=>el0_ia(r,e),ESR_ELx_EC_FP_ASIMD=>el0_fpsimd_acc(r,e),ESR_ELx_EC_FP_EXC32=>el0_fpsimd_exc(r,e),ESR_ELx_EC_PC_ALIGN=>el0_pc(r,e),ESR_ELx_EC_UNKNOWN|ESR_ELx_EC_CP14_MR|ESR_ELx_EC_CP14_LS|ESR_ELx_EC_CP14_64=>el0_undef(r,e),ESR_ELx_EC_CP15_32|ESR_ELx_EC_CP15_64=>el0_cp15(r,e),ESR_ELx_EC_BREAKPT_LOW=>el0_breakpt(r,e),ESR_ELx_EC_SOFTSTP_LOW=>el0_softstp(r,e),ESR_ELx_EC_WATCHPT_LOW=>el0_watchpt(r,e),ESR_ELx_EC_BKPT32=>el0_bkpt32(r,e),_=>el0_inv(r,e)}}
#[cfg(feature="CONFIG_COMPAT")] pub unsafe extern "C" fn el0t_32_irq_handler(r:*mut pt_regs){el0t_64_irq_handler(r)}
#[cfg(feature="CONFIG_COMPAT")] pub unsafe extern "C" fn el0t_32_fiq_handler(r:*mut pt_regs){el0t_64_fiq_handler(r)}
#[cfg(feature="CONFIG_COMPAT")] pub unsafe extern "C" fn el0t_32_error_handler(r:*mut pt_regs){el0t_64_error_handler(r)}

#[cfg(feature="CONFIG_ARM_SDE_INTERFACE")]
pub unsafe extern "C" fn __sdei_handler(r:*mut pt_regs,arg:*mut sdei_registered_event)->u64{if system_uses_hw_pan(){set_pstate_pan(1)}else if cpu_has_pan(){set_pstate_pan(0)}let s=irqentry_nmi_enter(r);let ret=do_sdei_event(r,arg);irqentry_nmi_exit(r,s);ret}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
