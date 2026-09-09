// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// External kernel dependencies supplied by the surrounding translation unit.

/* Non-VHE specific context */
static mut KVM_HOST_DATA: PerCpu<kvm_host_data> = PerCpu::new();
static mut KVM_HYP_CTXT: PerCpu<kvm_cpu_context> = PerCpu::new();
static mut KVM_HYP_VECTOR: PerCpu<c_ulong> = PerCpu::new();

static mut HFGRTR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HFGWTR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HFGITR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HDFGRTR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HDFGWTR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HAFGRTR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HFGRTR2_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HFGWTR2_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HFGITR2_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HDFGRTR2_MASKS: fgt_masks = fgt_masks::ZERO;
static mut HDFGWTR2_MASKS: fgt_masks = fgt_masks::ZERO;
static mut ICH_HFGRTR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut ICH_HFGWTR_MASKS: fgt_masks = fgt_masks::ZERO;
static mut ICH_HFGITR_MASKS: fgt_masks = fgt_masks::ZERO;

extern "C" {
    fn kvm_nvhe_prepare_backtrace(fp: c_ulong, pc: c_ulong);
}

unsafe fn __activate_traps(vcpu: *mut kvm_vcpu) {
    ___activate_traps(vcpu, (*vcpu).arch.hcr_el2);
    *host_data_ptr(host_debug_state.mdcr_el2) = read_sysreg(mdcr_el2);
    write_sysreg((*vcpu).arch.mdcr_el2, mdcr_el2);
    __activate_traps_common(vcpu);
    __activate_cptr_traps(vcpu);
    write_sysreg(this_cpu_read(kvm_hyp_vector), vbar_el2);

    if cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT) {
        let ctxt = &(*vcpu).arch.ctxt;
        isb();
        write_sysreg_el1(ctxt_sys_reg(ctxt, SCTLR_EL1), SYS_SCTLR);
        isb();
        write_sysreg_el1(ctxt_sys_reg(ctxt, TCR_EL1), SYS_TCR);
    }
}

unsafe fn __deactivate_traps(vcpu: *mut kvm_vcpu) {
    extern "C" {
        static __kvm_hyp_host_vector: c_char;
    }
    ___deactivate_traps(vcpu);
    if cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT) {
        let mut val: u64;
        val = read_sysreg_el1(SYS_TCR);
        write_sysreg_el1(val | TCR_EPD1_MASK | TCR_EPD0_MASK, SYS_TCR);
        isb();
        val = read_sysreg_el1(SYS_SCTLR);
        write_sysreg_el1(val | SCTLR_ELx_M, SYS_SCTLR);
        isb();
    }
    write_sysreg(*host_data_ptr(host_debug_state.mdcr_el2), mdcr_el2);
    __deactivate_traps_common(vcpu);
    write_sysreg((*this_cpu_ptr(&kvm_init_params)).hcr_el2);
    __deactivate_cptr_traps(vcpu);
    write_sysreg(&__kvm_hyp_host_vector, vbar_el2);
}

unsafe fn __hyp_vgic_save_state(vcpu: *mut kvm_vcpu) {
    if vgic_is_v5(kern_hyp_va((*vcpu).kvm)) {
        __vgic_v5_save_state(&mut (*vcpu).arch.vgic_cpu.vgic_v5);
        __vgic_v5_save_ppi_state(&mut (*vcpu).arch.vgic_cpu.vgic_v5);
        return;
    }
    if static_branch_unlikely(&kvm_vgic_global_state.gicv3_cpuif) {
        __vgic_v3_save_state(&mut (*vcpu).arch.vgic_cpu.vgic_v3);
        __vgic_v3_deactivate_traps(&mut (*vcpu).arch.vgic_cpu.vgic_v3);
    }
}

unsafe fn __hyp_vgic_restore_state(vcpu: *mut kvm_vcpu) {
    if vgic_is_v5(kern_hyp_va((*vcpu).kvm)) {
        __vgic_v5_restore_state(&mut (*vcpu).arch.vgic_cpu.vgic_v5);
        __vgic_v5_restore_ppi_state(&mut (*vcpu).arch.vgic_cpu.vgic_v5);
        return;
    }
    if static_branch_unlikely(&kvm_vgic_global_state.gicv3_cpuif) {
        __vgic_v3_activate_traps(&mut (*vcpu).arch.vgic_cpu.vgic_v3);
        __vgic_v3_restore_state(&mut (*vcpu).arch.vgic_cpu.vgic_v3);
    }
}

#[cfg(feature = "CONFIG_HW_PERF_EVENTS")]
unsafe fn __pmu_switch_to_guest(vcpu: *mut kvm_vcpu) -> bool {
    let pmu = &(*vcpu).arch.pmu.events;
    if pmu.events_host != 0 { write_sysreg(pmu.events_host, pmcntenclr_el0); }
    if pmu.events_guest != 0 { write_sysreg(pmu.events_guest, pmcntenset_el0); }
    pmu.events_host != 0 || pmu.events_guest != 0
}

#[cfg(feature = "CONFIG_HW_PERF_EVENTS")]
unsafe fn __pmu_switch_to_host(vcpu: *mut kvm_vcpu) {
    let pmu = &(*vcpu).arch.pmu.events;
    if pmu.events_guest != 0 { write_sysreg(pmu.events_guest, pmcntenclr_el0); }
    if pmu.events_host != 0 { write_sysreg(pmu.events_host, pmcntenset_el0); }
}

#[cfg(not(feature = "CONFIG_HW_PERF_EVENTS"))]
unsafe fn __pmu_switch_to_guest(_vcpu: *mut kvm_vcpu) -> bool { false }
#[cfg(not(feature = "CONFIG_HW_PERF_EVENTS"))]
unsafe fn __pmu_switch_to_host(_vcpu: *mut kvm_vcpu) {}

unsafe fn kvm_handle_pvm_sys64(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool {
    kvm_hyp_handle_sysreg(vcpu, exit_code) || kvm_handle_pvm_sysreg(vcpu, exit_code)
}

static hyp_exit_handlers: [exit_handler_fn; ESR_ELx_EC_MAX as usize + 1] = [
    /* designated initializers are supplied by the surrounding bindings */
];
static pvm_exit_handlers: [exit_handler_fn; ESR_ELx_EC_MAX as usize + 1] = [
];

unsafe fn kvm_get_exit_handler_array(vcpu: *mut kvm_vcpu) -> *const exit_handler_fn {
    if unlikely(vcpu_is_protected(vcpu)) { pvm_exit_handlers.as_ptr() } else { hyp_exit_handlers.as_ptr() }
}

unsafe fn fixup_guest_exit(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool {
    let handlers = kvm_get_exit_handler_array(vcpu);
    synchronize_vcpu_pstate(vcpu);
    if unlikely(vcpu_is_protected(vcpu) && vcpu_mode_is_32bit(vcpu)) {
        vcpu_clear_flag(vcpu, VCPU_INITIALIZED);
        *exit_code &= BIT(ARM_EXIT_WITH_SERROR_BIT);
        *exit_code |= ARM_EXCEPTION_IL;
    }
    __fixup_guest_exit(vcpu, exit_code, handlers)
}

unsafe fn __kvm_vcpu_run(vcpu: *mut kvm_vcpu) -> u64 {
    let host_ctxt: *mut kvm_cpu_context;
    let guest_ctxt: *mut kvm_cpu_context;
    let mut mmu: *mut kvm_s2_mmu;
    let pmu_switch_needed: bool;
    let mut exit_code: u64;
    if system_uses_irq_prio_masking() { gic_write_pmr(GIC_PRIO_IRQON | GIC_PRIO_PSR_I_SET); pmr_sync(); }
    host_ctxt = host_data_ptr(host_ctxt);
    (*host_ctxt).__hyp_running_vcpu = vcpu;
    guest_ctxt = &mut (*vcpu).arch.ctxt;
    pmu_switch_needed = __pmu_switch_to_guest(vcpu);
    __sysreg_save_state_nvhe(host_ctxt);
    __debug_save_host_buffers_nvhe(vcpu);
    dsb(nsh);
    __kvm_adjust_pc(vcpu);
    __sysreg32_restore_state(vcpu);
    __sysreg_restore_state_nvhe(guest_ctxt);
    mmu = kern_hyp_va((*vcpu).arch.hw_mmu);
    __load_stage2(mmu);
    __activate_traps(vcpu);
    __hyp_vgic_restore_state(vcpu);
    __timer_enable_traps(vcpu);
    __debug_switch_to_guest(vcpu);
    loop {
        trace_hyp_exit(host_ctxt, HYP_REASON_ERET_GUEST);
        exit_code = __guest_enter(vcpu);
        trace_hyp_enter(host_ctxt, HYP_REASON_GUEST_EXIT);
        if !fixup_guest_exit(vcpu, &mut exit_code) { break; }
    }
    __sysreg_save_state_nvhe(guest_ctxt);
    __sysreg32_save_state(vcpu);
    __timer_disable_traps(vcpu);
    __hyp_vgic_save_state(vcpu);
    dsb(nsh);
    __deactivate_traps(vcpu);
    __load_host_stage2();
    __sysreg_restore_state_nvhe(host_ctxt);
    if guest_owns_fp_regs() { __fpsimd_save_fpexc32(vcpu); }
    __debug_switch_to_host(vcpu);
    __debug_restore_host_buffers_nvhe(vcpu);
    if pmu_switch_needed { __pmu_switch_to_host(vcpu); }
    if system_uses_irq_prio_masking() { gic_write_pmr(GIC_PRIO_IRQOFF); }
    (*host_ctxt).__hyp_running_vcpu = core::ptr::null_mut();
    exit_code
}

unsafe extern "C" fn hyp_panic() -> ! {
    let spsr = read_sysreg_el2(SYS_SPSR);
    let elr = read_sysreg_el2(SYS_ELR);
    let par = read_sysreg_par();
    let host_ctxt = host_data_ptr(host_ctxt);
    let vcpu = (*host_ctxt).__hyp_running_vcpu;
    if !vcpu.is_null() {
        __timer_disable_traps(vcpu);
        __deactivate_traps(vcpu);
        __load_host_stage2();
        __sysreg_restore_state_nvhe(host_ctxt);
    }
    kvm_nvhe_prepare_backtrace(__builtin_frame_address(0) as c_ulong, _THIS_IP_);
    __hyp_do_panic(host_ctxt, spsr, elr, par);
    core::hint::unreachable_unchecked()
}

unsafe extern "C" fn hyp_panic_bad_stack() -> ! { hyp_panic() }

unsafe extern "C" fn kvm_unexpected_el2_exception() {
    __kvm_unexpected_el2_exception();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
