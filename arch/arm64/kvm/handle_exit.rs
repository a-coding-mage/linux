// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Derived from arch/arm/kvm/handle_exit.c:
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 */

// Kernel headers and trace_handle_exit.h provide the external types, constants,
// functions, and tracepoints referenced below.

type ExitHandleFn = unsafe fn(*mut kvm_vcpu) -> i32;

unsafe fn kvm_handle_guest_serror(vcpu: *mut kvm_vcpu, esr: u64) {
    if !arm64_is_ras_serror(esr) || arm64_is_fatal_ras_serror(core::ptr::null_mut(), esr) {
        kvm_inject_serror(vcpu);
    }
}

unsafe fn handle_hvc(vcpu: *mut kvm_vcpu) -> i32 {
    trace_kvm_hvc_arm64(*vcpu_pc(vcpu), vcpu_get_reg(vcpu, 0), kvm_vcpu_hvc_get_imm(vcpu));
    (*vcpu).stat.hvc_exit_stat += 1;
    if vcpu_has_nv(vcpu) {
        if vcpu_read_sys_reg(vcpu, HCR_EL2) & HCR_HCD != 0 {
            kvm_inject_undefined(vcpu);
        } else {
            kvm_inject_nested_sync(vcpu, kvm_vcpu_get_esr(vcpu));
        }
        return 1;
    }
    kvm_smccc_call_handler(vcpu)
}

unsafe fn handle_smc(vcpu: *mut kvm_vcpu) -> i32 {
    if forward_smc_trap(vcpu) != 0 { return 1; }
    kvm_incr_pc(vcpu);
    if kvm_vcpu_hvc_get_imm(vcpu) != 0 {
        vcpu_set_reg(vcpu, 0, !0usize as u64);
        return 1;
    }
    kvm_smccc_call_handler(vcpu)
}

unsafe fn kvm_handle_fpasimd(vcpu: *mut kvm_vcpu) -> i32 {
    if guest_hyp_fpsimd_traps_enabled(vcpu) { return kvm_inject_nested_sync(vcpu, kvm_vcpu_get_esr(vcpu)); }
    kvm_inject_undefined(vcpu); 1
}

unsafe fn kvm_handle_wfx(vcpu: *mut kvm_vcpu) -> i32 {
    let mut esr = kvm_vcpu_get_esr(vcpu);
    let is_wfe = (esr & ESR_ELx_WFx_ISS_WFE) != 0;
    if guest_hyp_wfx_traps_enabled(vcpu) { return kvm_inject_nested_sync(vcpu, kvm_vcpu_get_esr(vcpu)); }
    if is_wfe { trace_kvm_wfx_arm64(*vcpu_pc(vcpu), true); (*vcpu).stat.wfe_exit_stat += 1; }
    else { trace_kvm_wfx_arm64(*vcpu_pc(vcpu), false); (*vcpu).stat.wfi_exit_stat += 1; }
    if esr & ESR_ELx_WFx_ISS_WFxT != 0 {
        if esr & ESR_ELx_WFx_ISS_RV != 0 {
            let mut now = kvm_phys_timer_read();
            if is_hyp_ctxt(vcpu) && vcpu_el2_e2h_is_set(vcpu) { now -= timer_get_offset(vcpu_hvtimer(vcpu)); }
            else { now -= timer_get_offset(vcpu_vtimer(vcpu)); }
            let val = vcpu_get_reg(vcpu, kvm_vcpu_sys_get_rt(vcpu));
            if now >= val { kvm_incr_pc(vcpu); return 1; }
        } else { esr &= !ESR_ELx_WFx_ISS_WFxT; }
    }
    if esr & ESR_ELx_WFx_ISS_WFE != 0 { kvm_vcpu_on_spin(vcpu, vcpu_mode_priv(vcpu)); }
    else { if esr & ESR_ELx_WFx_ISS_WFxT != 0 { vcpu_set_flag(vcpu, IN_WFIT); } kvm_vcpu_wfi(vcpu); }
    kvm_incr_pc(vcpu); 1
}

unsafe fn kvm_handle_guest_debug(vcpu: *mut kvm_vcpu) -> i32 {
    let run = (*vcpu).run;
    let esr = kvm_vcpu_get_esr(vcpu);
    if !(*vcpu).guest_debug && forward_debug_exception(vcpu) { return 1; }
    (*run).exit_reason = KVM_EXIT_DEBUG;
    (*run).debug.arch.hsr = lower_32_bits(esr);
    (*run).debug.arch.hsr_high = upper_32_bits(esr);
    (*run).flags = KVM_DEBUG_ARCH_HSR_HIGH_VALID;
    match ESR_ELx_EC(esr) {
        ESR_ELx_EC_WATCHPT_LOW => (*run).debug.arch.far = (*vcpu).arch.fault.far_el2,
        ESR_ELx_EC_SOFTSTP_LOW => *vcpu_cpsr(vcpu) |= DBG_SPSR_SS,
        _ => {}
    }
    0
}

unsafe fn kvm_handle_unknown_ec(vcpu: *mut kvm_vcpu) -> i32 {
    let esr = kvm_vcpu_get_esr(vcpu);
    kvm_pr_unimpl!("Unknown exception class: esr: %#016llx -- %s\n", esr, esr_get_class_string(esr));
    kvm_inject_undefined(vcpu); 1
}

unsafe fn handle_sve(vcpu: *mut kvm_vcpu) -> i32 {
    if guest_hyp_sve_traps_enabled(vcpu) { return kvm_inject_nested_sync(vcpu, kvm_vcpu_get_esr(vcpu)); }
    kvm_inject_undefined(vcpu); 1
}

unsafe fn kvm_handle_ptrauth(vcpu: *mut kvm_vcpu) -> i32 {
    if !vcpu_has_ptrauth(vcpu) { kvm_inject_undefined(vcpu); return 1; }
    if is_nested_ctxt(vcpu) { kvm_inject_nested_sync(vcpu, kvm_vcpu_get_esr(vcpu)); return 1; }
    WARN_ON_ONCE!(1); kvm_inject_undefined(vcpu); 1
}

unsafe fn kvm_handle_eret(vcpu: *mut kvm_vcpu) -> i32 {
    if esr_iss_is_eretax(kvm_vcpu_get_esr(vcpu)) && !vcpu_has_ptrauth(vcpu) { return kvm_handle_ptrauth(vcpu); }
    if is_hyp_ctxt(vcpu) { kvm_emulate_nested_eret(vcpu); }
    else { kvm_inject_nested_sync(vcpu, kvm_vcpu_get_esr(vcpu)); }
    1
}

unsafe fn handle_svc(vcpu: *mut kvm_vcpu) -> i32 { kvm_inject_nested_sync(vcpu, kvm_vcpu_get_esr(vcpu)); 1 }

unsafe fn kvm_handle_gcs(vcpu: *mut kvm_vcpu) -> i32 {
    if kvm_has_feat((*vcpu).kvm, ID_AA64PFR1_EL1, GCS, IMP) { WARN_ON_ONCE!(1); }
    kvm_inject_undefined(vcpu); 1
}

unsafe fn handle_other(vcpu: *mut kvm_vcpu) -> i32 {
    let mut allowed: bool; let mut fwd = is_nested_ctxt(vcpu);
    let hcrx = __vcpu_sys_reg(vcpu, HCRX_EL2); let esr = kvm_vcpu_get_esr(vcpu); let iss = ESR_ELx_ISS(esr); let kvm = (*vcpu).kvm;
    match iss {
        ESR_ELx_ISS_OTHER_ST64BV => { allowed = kvm_has_feat(kvm, ID_AA64ISAR1_EL1, LS64, LS64_V); fwd &= hcrx & HCRX_EL2_EnASR == 0; }
        ESR_ELx_ISS_OTHER_ST64BV0 => { allowed = kvm_has_feat(kvm, ID_AA64ISAR1_EL1, LS64, LS64_ACCDATA); fwd &= hcrx & HCRX_EL2_EnAS0 == 0; }
        ESR_ELx_ISS_OTHER_LDST64B => { allowed = kvm_has_feat(kvm, ID_AA64ISAR1_EL1, LS64, LS64); fwd &= hcrx & HCRX_EL2_EnALS == 0; }
        ESR_ELx_ISS_OTHER_TSBCSYNC => { allowed = kvm_has_feat(kvm, ID_AA64DFR0_EL1, TraceBuffer, TRBE_V1P1); fwd &= __vcpu_sys_reg(vcpu, HFGITR2_EL2) & HFGITR2_EL2_TSBCSYNC != 0; }
        ESR_ELx_ISS_OTHER_PSBCSYNC => { allowed = kvm_has_feat(kvm, ID_AA64DFR0_EL1, PMSVer, V1P5); fwd &= __vcpu_sys_reg(vcpu, HFGITR_EL2) & HFGITR_EL2_PSBCSYNC != 0; }
        _ => { WARN_ON_ONCE!(1); allowed = false; }
    }
    WARN_ON_ONCE!(allowed && !fwd);
    if allowed && fwd { kvm_inject_nested_sync(vcpu, esr); } else { kvm_inject_undefined(vcpu); } 1
}

// The C designated initializer table is represented with the same handler mapping.
static mut arm_exit_handlers: [ExitHandleFn; ESR_ELx_EC_MAX + 1] = [kvm_handle_unknown_ec; ESR_ELx_EC_MAX + 1];

unsafe fn kvm_get_exit_handler(vcpu: *mut kvm_vcpu) -> ExitHandleFn { arm_exit_handlers[ESR_ELx_EC(kvm_vcpu_get_esr(vcpu)) as usize] }

unsafe fn handle_trap_exceptions(vcpu: *mut kvm_vcpu) -> i32 {
    if !kvm_condition_valid(vcpu) { kvm_incr_pc(vcpu); 1 } else { kvm_get_exit_handler(vcpu)(vcpu) }
}

unsafe fn handle_exit(vcpu: *mut kvm_vcpu, mut exception_index: i32) -> i32 {
    let run = (*vcpu).run;
    if ARM_SERROR_PENDING(exception_index) { return 1; }
    exception_index = ARM_EXCEPTION_CODE(exception_index);
    match exception_index {
        ARM_EXCEPTION_IRQ | ARM_EXCEPTION_EL1_SERROR => 1,
        ARM_EXCEPTION_TRAP => handle_trap_exceptions(vcpu),
        ARM_EXCEPTION_HYP_GONE => { (*run).exit_reason = KVM_EXIT_FAIL_ENTRY; 0 }
        ARM_EXCEPTION_IL => { (*run).exit_reason = KVM_EXIT_FAIL_ENTRY; -EINVAL }
        _ => { kvm_pr_unimpl!("Unsupported exception type: %d", exception_index); (*run).exit_reason = KVM_EXIT_INTERNAL_ERROR; 0 }
    }
}

unsafe fn handle_exit_pkvm_state(vcpu: *mut kvm_vcpu, exception_index: i32) {
    let exception_code = ARM_EXCEPTION_CODE(exception_index);
    if !is_protected_kvm_enabled() || kvm_vm_is_protected((*vcpu).kvm) { return; }
    if exception_code == ARM_EXCEPTION_TRAP || exception_code == ARM_EXCEPTION_EL1_SERROR || ARM_SERROR_PENDING(exception_index) {
        kvm_call_hyp_nvhe(__pkvm_vcpu_sync_state); vcpu_set_flag(vcpu, PKVM_HOST_STATE_DIRTY);
    } else { vcpu_clear_flag(vcpu, PKVM_HOST_STATE_DIRTY); }
}

pub unsafe fn handle_exit_early(vcpu: *mut kvm_vcpu, mut exception_index: i32) {
    handle_exit_pkvm_state(vcpu, exception_index);
    if ARM_SERROR_PENDING(exception_index) {
        if this_cpu_has_cap(ARM64_HAS_RAS_EXTN) { let disr = kvm_vcpu_get_disr(vcpu); kvm_handle_guest_serror(vcpu, disr_to_esr(disr)); }
        else { kvm_inject_serror(vcpu); }
        return;
    }
    exception_index = ARM_EXCEPTION_CODE(exception_index);
    if exception_index == ARM_EXCEPTION_EL1_SERROR { kvm_handle_guest_serror(vcpu, kvm_vcpu_get_esr(vcpu)); }
}

unsafe fn nvhe_hyp_panic_host_s2_disabled() -> bool { !is_protected_kvm_enabled() || IS_ENABLED!(CONFIG_PKVM_DISABLE_STAGE2_ON_PANIC) }

unsafe fn print_nvhe_hyp_panic(name: *const i8, panic_addr: u64) {
    if nvhe_hyp_panic_host_s2_disabled() { kvm_err!("nVHE hyp %s at: [<%016llx>] %pB!\n", name, panic_addr, (panic_addr + kaslr_offset()) as *mut core::ffi::c_void); }
    else { kvm_err!("nVHE hyp %s at: %016llx!\n", name, panic_addr); }
}

unsafe fn kvm_nvhe_report_cfi_failure(panic_addr: u64) { print_nvhe_hyp_panic(c"CFI failure".as_ptr(), panic_addr); if IS_ENABLED!(CONFIG_CFI_PERMISSIVE) { kvm_err!(" (CONFIG_CFI_PERMISSIVE ignored for hyp failures)\n"); } }

pub unsafe extern "C" fn nvhe_hyp_panic_handler(esr: u64, spsr: u64, elr_virt: u64, elr_phys: u64, par: u64, vcpu: usize, far: u64, hpfar: u64) -> ! {
    let elr_in_kimg = __phys_to_kimg(elr_phys); let hyp_offset = elr_in_kimg - kaslr_offset() - elr_virt; let mode = spsr & PSR_MODE_MASK; let panic_addr = elr_virt + hyp_offset;
    if mode != PSR_MODE_EL2t && mode != PSR_MODE_EL2h { kvm_err!("Invalid host exception to nVHE hyp!\n"); }
    else if ESR_ELx_EC(esr) == ESR_ELx_EC_BRK64 && esr_brk_comment(esr) == BUG_BRK_IMM { print_nvhe_hyp_panic(c"BUG".as_ptr(), panic_addr); }
    else if IS_ENABLED!(CONFIG_CFI) && esr_is_cfi_brk(esr) { kvm_nvhe_report_cfi_failure(panic_addr); }
    else if IS_ENABLED!(CONFIG_UBSAN_KVM_EL2) && ESR_ELx_EC(esr) == ESR_ELx_EC_BRK64 && esr_is_ubsan_brk(esr) { print_nvhe_hyp_panic(report_ubsan_failure(esr & UBSAN_BRK_MASK), panic_addr); }
    else { print_nvhe_hyp_panic(c"panic".as_ptr(), panic_addr); }
    kvm_nvhe_dump_backtrace(hyp_offset); dump_kernel_instr(panic_addr + kaslr_offset());
    kvm_err!("Hyp Offset: 0x%llx\n", hyp_offset);
    panic!("HYP panic: PS:{:08x} PC:{:016x} ESR:{:016x} FAR:{:016x} HPFAR:{:016x} PAR:{:016x} VCPU:{:016x}", spsr, elr_virt, esr, far, hpfar, par, vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
