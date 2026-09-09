// SPDX-License-Identifier: GPL-2.0-only
/*
 * Debug and Guest Debug support
 *
 * Copyright (C) 2015 - Linaro Ltd
 * Authors: Alex Bennée <alex.bennee@linaro.org>
 *          Oliver Upton <oliver.upton@linux.dev>
 */

// The declarations used here are supplied by the kernel/KVM environment.

unsafe fn cpu_has_spe(dfr0: u64) -> bool {
    cpuid_feature_extract_unsigned_field(dfr0, ID_AA64DFR0_EL1_PMSVer_SHIFT) != 0
        && (read_sysreg_s(SYS_PMBIDR_EL1) & PMBIDR_EL1_P) == 0
}

/// kvm_arm_setup_mdcr_el2 - configure vcpu mdcr_el2 value
unsafe fn kvm_arm_setup_mdcr_el2(vcpu: *mut kvm_vcpu) {
    preempt_disable();

    (*vcpu).arch.mdcr_el2 = FIELD_PREP(
        MDCR_EL2_HPMN,
        *host_data_ptr(nr_event_counters),
    );
    (*vcpu).arch.mdcr_el2 |= MDCR_EL2_TPM
        | MDCR_EL2_TPMS
        | MDCR_EL2_TTRF
        | MDCR_EL2_TPMCR
        | MDCR_EL2_TDRA
        | MDCR_EL2_TDOSA;

    /* Is the VM being debugged by userspace? */
    if (*vcpu).guest_debug != 0 {
        (*vcpu).arch.mdcr_el2 |= MDCR_EL2_TDE;
    }

    /* Trap debug registers if the guest doesn't have ownership of them. */
    if !kvm_guest_owns_debug_regs(vcpu) {
        (*vcpu).arch.mdcr_el2 |= MDCR_EL2_TDA;
    }

    if vcpu_has_nv(vcpu) {
        kvm_nested_setup_mdcr_el2(vcpu);
    }

    if has_vhe() {
        write_sysreg((*vcpu).arch.mdcr_el2, mdcr_el2);
    }
    preempt_enable();
}

pub unsafe fn kvm_init_host_debug_data() {
    let dfr0 = read_sysreg(id_aa64dfr0_el1);
    let pmuver = cpuid_feature_extract_unsigned_field(dfr0, ID_AA64DFR0_EL1_PMUVer_SHIFT);

    if pmuv3_implemented(pmuver) {
        *host_data_ptr(nr_event_counters) =
            FIELD_GET(ARMV8_PMU_PMCR_N, read_sysreg(pmcr_el0));
    }
    *host_data_ptr(debug_brps) = SYS_FIELD_GET(ID_AA64DFR0_EL1, BRPs, dfr0);
    *host_data_ptr(debug_wrps) = SYS_FIELD_GET(ID_AA64DFR0_EL1, WRPs, dfr0);

    if cpu_has_spe(dfr0) {
        host_data_set_flag(HAS_SPE);
    }
    if has_vhe() {
        return;
    }
    if cpuid_feature_extract_unsigned_field(dfr0, ID_AA64DFR0_EL1_BRBE_SHIFT) != 0 {
        host_data_set_flag(HAS_BRBE);
    }
    if cpuid_feature_extract_unsigned_field(dfr0, ID_AA64DFR0_EL1_TraceFilt_SHIFT) != 0 {
        if is_protected_kvm_enabled() {
            host_data_set_flag(EL1_TRACING_CONFIGURED);
        }
        if cpuid_feature_extract_unsigned_field(dfr0, ID_AA64DFR0_EL1_TraceBuffer_SHIFT) != 0
            && (read_sysreg_s(SYS_TRBIDR_EL1) & TRBIDR_EL1_P) == 0
        {
            host_data_set_flag(HAS_TRBE);
        }
    }
}

pub unsafe fn kvm_debug_init_vhe() {
    if host_data_test_flag(HAS_SPE) {
        write_sysreg_el1(0, SYS_PMSCR);
    }
}

unsafe fn setup_external_mdscr(vcpu: *mut kvm_vcpu) {
    let mut mdscr = vcpu_read_sys_reg(vcpu, MDSCR_EL1) & !(MDSCR_EL1_SS | MDSCR_EL1_MDE | MDSCR_EL1_KDE);
    if (*vcpu).guest_debug & KVM_GUESTDBG_SINGLESTEP != 0 {
        mdscr |= MDSCR_EL1_SS;
    }
    if (*vcpu).guest_debug & KVM_GUESTDBG_USE_HW != 0 {
        mdscr |= MDSCR_EL1_MDE | MDSCR_EL1_KDE;
    }
    (*vcpu).arch.external_mdscr_el1 = mdscr;
}

pub unsafe fn kvm_vcpu_load_debug(vcpu: *mut kvm_vcpu) {
    let mdscr;
    KVM_BUG_ON(vcpu_get_flag(vcpu, SYSREGS_ON_CPU), (*vcpu).kvm);
    if has_vhe() {
        *host_data_ptr(host_debug_state.mdcr_el2) = read_sysreg(mdcr_el2);
    }
    if (*vcpu).guest_debug != 0 || kvm_vcpu_os_lock_enabled(vcpu) {
        (*vcpu).arch.debug_owner = VCPU_DEBUG_HOST_OWNED;
        setup_external_mdscr(vcpu);
        if (*vcpu).guest_debug & KVM_GUESTDBG_SINGLESTEP != 0 {
            if *vcpu_cpsr(vcpu) & DBG_SPSR_SS != 0 { vcpu_clear_flag(vcpu, GUEST_SS_ACTIVE_PENDING); }
            else { vcpu_set_flag(vcpu, GUEST_SS_ACTIVE_PENDING); }
            if !vcpu_get_flag(vcpu, HOST_SS_ACTIVE_PENDING) { *vcpu_cpsr(vcpu) |= DBG_SPSR_SS; }
            else { *vcpu_cpsr(vcpu) &= !DBG_SPSR_SS; }
        }
    } else {
        mdscr = vcpu_read_sys_reg(vcpu, MDSCR_EL1);
        if mdscr & (MDSCR_EL1_KDE | MDSCR_EL1_MDE) != 0 { (*vcpu).arch.debug_owner = VCPU_DEBUG_GUEST_OWNED; }
        else { (*vcpu).arch.debug_owner = VCPU_DEBUG_FREE; }
    }
    kvm_arm_setup_mdcr_el2(vcpu);
}

pub unsafe fn kvm_vcpu_put_debug(vcpu: *mut kvm_vcpu) {
    if has_vhe() { write_sysreg(*host_data_ptr(host_debug_state.mdcr_el2), mdcr_el2); }
    if (*vcpu).guest_debug & KVM_GUESTDBG_SINGLESTEP == 0 { return; }
    if *vcpu_cpsr(vcpu) & DBG_SPSR_SS == 0 { vcpu_set_flag(vcpu, HOST_SS_ACTIVE_PENDING); }
    else { vcpu_clear_flag(vcpu, HOST_SS_ACTIVE_PENDING); }
    if vcpu_get_flag(vcpu, GUEST_SS_ACTIVE_PENDING) { *vcpu_cpsr(vcpu) &= !DBG_SPSR_SS; }
    else { *vcpu_cpsr(vcpu) |= DBG_SPSR_SS; }
}

pub unsafe fn kvm_debug_set_guest_ownership(vcpu: *mut kvm_vcpu) {
    if kvm_host_owns_debug_regs(vcpu) { return; }
    (*vcpu).arch.debug_owner = VCPU_DEBUG_GUEST_OWNED;
    kvm_arm_setup_mdcr_el2(vcpu);
}

pub unsafe fn kvm_debug_handle_oslar(vcpu: *mut kvm_vcpu, val: u64) {
    if val & OSLAR_EL1_OSLK != 0 { __vcpu_rmw_sys_reg(vcpu, OSLSR_EL1, |=, OSLSR_EL1_OSLK); }
    else { __vcpu_rmw_sys_reg(vcpu, OSLSR_EL1, &=, !OSLSR_EL1_OSLK); }
    preempt_disable();
    kvm_arch_vcpu_put(vcpu);
    kvm_arch_vcpu_load(vcpu, smp_processor_id());
    preempt_enable();
}

unsafe fn skip_trbe_access(skip_condition: bool) -> bool {
    WARN_ON_ONCE(preemptible()) || skip_condition || is_protected_kvm_enabled() || !is_kvm_arm_initialised()
}

pub unsafe fn kvm_enable_trbe() { if !skip_trbe_access(has_vhe()) { host_data_set_flag(TRBE_ENABLED); } }
pub unsafe fn kvm_disable_trbe() { if !skip_trbe_access(has_vhe()) { host_data_clear_flag(TRBE_ENABLED); } }

pub unsafe fn kvm_tracing_set_el1_configuration(trfcr_while_in_guest: u64) {
    if skip_trbe_access(false) { return; }
    if has_vhe() { write_sysreg_s(trfcr_while_in_guest, SYS_TRFCR_EL12); return; }
    *host_data_ptr(trfcr_while_in_guest) = trfcr_while_in_guest;
    if read_sysreg_s(SYS_TRFCR_EL1) != trfcr_while_in_guest { host_data_set_flag(EL1_TRACING_CONFIGURED); }
    else { host_data_clear_flag(EL1_TRACING_CONFIGURED); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
