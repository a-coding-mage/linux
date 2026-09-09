// SPDX-License-Identifier: GPL-2.0-only
/*
 * KVM PMU support for AMD
 *
 * Copyright 2015, Red Hat, Inc. and/or its affiliates.
 *
 * Author:
 *   Wei Huang <wei@redhat.com>
 *
 * Implementation is based on pmu_intel.c file
 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum pmu_type {
    PMU_TYPE_COUNTER = 0,
    PMU_TYPE_EVNTSEL,
}

unsafe fn amd_pmu_get_pmc(pmu: *mut kvm_pmu, pmc_idx: i32) -> *mut kvm_pmc {
    let num_counters = (*pmu).nr_arch_gp_counters;
    if pmc_idx >= num_counters as i32 {
        return core::ptr::null_mut();
    }
    &mut (*pmu).gp_counters[array_index_nospec(pmc_idx as usize, num_counters as usize)]
}

unsafe fn get_gp_pmc_amd(pmu: *mut kvm_pmu, msr: u32, ty: pmu_type) -> *mut kvm_pmc {
    let vcpu = pmu_to_vcpu(pmu);
    let idx: u32;
    if (*pmu).version == 0 { return core::ptr::null_mut(); }
    match msr {
        MSR_F15H_PERF_CTL0..=MSR_F15H_PERF_CTR5 => {
            if !guest_cpu_cap_has(vcpu, X86_FEATURE_PERFCTR_CORE) { return core::ptr::null_mut(); }
            idx = (msr - MSR_F15H_PERF_CTL0) / 2;
            if ((msr & 1) != 0) != (ty == pmu_type::PMU_TYPE_EVNTSEL) { return core::ptr::null_mut(); }
        }
        MSR_K7_EVNTSEL0..=MSR_K7_EVNTSEL3 => {
            if ty != pmu_type::PMU_TYPE_EVNTSEL { return core::ptr::null_mut(); }
            idx = msr - MSR_K7_EVNTSEL0;
        }
        MSR_K7_PERFCTR0..=MSR_K7_PERFCTR3 => {
            if ty != pmu_type::PMU_TYPE_COUNTER { return core::ptr::null_mut(); }
            idx = msr - MSR_K7_PERFCTR0;
        }
        _ => return core::ptr::null_mut(),
    }
    amd_pmu_get_pmc(pmu, idx as i32)
}

unsafe fn amd_check_rdpmc_early(vcpu: *mut kvm_vcpu, idx: u32) -> i32 {
    let pmu = vcpu_to_pmu(vcpu);
    if idx >= (*pmu).nr_arch_gp_counters { return -EINVAL; }
    0
}

/* idx is the ECX register of RDPMC instruction */
unsafe fn amd_rdpmc_ecx_to_pmc(vcpu: *mut kvm_vcpu, idx: u32, _mask: *mut u64) -> *mut kvm_pmc {
    amd_pmu_get_pmc(vcpu_to_pmu(vcpu), idx as i32)
}

unsafe fn amd_msr_idx_to_pmc(vcpu: *mut kvm_vcpu, msr: u32) -> *mut kvm_pmc {
    let pmu = vcpu_to_pmu(vcpu);
    let mut pmc = get_gp_pmc_amd(pmu, msr, pmu_type::PMU_TYPE_COUNTER);
    if pmc.is_null() { pmc = get_gp_pmc_amd(pmu, msr, pmu_type::PMU_TYPE_EVNTSEL); }
    pmc
}

unsafe fn amd_is_valid_msr(vcpu: *mut kvm_vcpu, msr: u32) -> bool {
    let pmu = vcpu_to_pmu(vcpu);
    match msr {
        MSR_K7_EVNTSEL0..=MSR_K7_PERFCTR3 => (*pmu).version > 0,
        MSR_F15H_PERF_CTL0..=MSR_F15H_PERF_CTR5 => guest_cpu_cap_has(vcpu, X86_FEATURE_PERFCTR_CORE),
        MSR_AMD64_PERF_CNTR_GLOBAL_STATUS | MSR_AMD64_PERF_CNTR_GLOBAL_CTL |
        MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR | MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_SET => (*pmu).version > 1,
        _ => {
            if msr > MSR_F15H_PERF_CTR5 && msr < MSR_F15H_PERF_CTL0 + 2 * (*pmu).nr_arch_gp_counters {
                return (*pmu).version > 1;
            }
            !amd_msr_idx_to_pmc(vcpu, msr).is_null()
        }
    }
}

unsafe fn amd_pmu_get_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> i32 {
    let pmu = vcpu_to_pmu(vcpu);
    let msr = (*msr_info).index;
    let mut pmc = get_gp_pmc_amd(pmu, msr, pmu_type::PMU_TYPE_COUNTER);
    if !pmc.is_null() { (*msr_info).data = pmc_read_counter(pmc); return 0; }
    pmc = get_gp_pmc_amd(pmu, msr, pmu_type::PMU_TYPE_EVNTSEL);
    if !pmc.is_null() { (*msr_info).data = (*pmc).eventsel; return 0; }
    1
}

unsafe fn amd_pmu_set_msr(vcpu: *mut kvm_vcpu, msr_info: *mut msr_data) -> i32 {
    let pmu = vcpu_to_pmu(vcpu);
    let msr = (*msr_info).index;
    let data = (*msr_info).data;
    let mut pmc = get_gp_pmc_amd(pmu, msr, pmu_type::PMU_TYPE_COUNTER);
    if !pmc.is_null() { pmc_write_counter(pmc, data); return 0; }
    pmc = get_gp_pmc_amd(pmu, msr, pmu_type::PMU_TYPE_EVNTSEL);
    if !pmc.is_null() {
        let data = data & !(*pmu).reserved_bits;
        if data != (*pmc).eventsel {
            (*pmc).eventsel = data;
            (*pmc).eventsel_hw = (data & !AMD64_EVENTSEL_HOSTONLY) | AMD64_EVENTSEL_GUESTONLY;
            if data & AMD64_EVENTSEL_HOST_GUEST_MASK != 0 { __set_bit((*pmc).idx, (*pmu).pmc_has_mode_specific_enables); }
            else { __clear_bit((*pmc).idx, (*pmu).pmc_has_mode_specific_enables); }
            kvm_pmu_request_counter_reprogram(pmc);
        }
        return 0;
    }
    1
}

unsafe fn amd_pmu_refresh(vcpu: *mut kvm_vcpu) {
    let pmu = vcpu_to_pmu(vcpu);
    let mut ebx: union cpuid_0x80000022_ebx = core::mem::zeroed();
    (*pmu).version = 1;
    if guest_cpu_cap_has(vcpu, X86_FEATURE_PERFMON_V2) {
        (*pmu).version = 2;
        // BUILD_BUG_ON: PERFMON_V2 is function 0x80000022, index 0.
        ebx.full = (*kvm_find_cpuid_entry_index(vcpu, 0x80000022, 0)).ebx;
        (*pmu).nr_arch_gp_counters = ebx.split.num_core_pmc;
    } else if guest_cpu_cap_has(vcpu, X86_FEATURE_PERFCTR_CORE) {
        (*pmu).nr_arch_gp_counters = AMD64_NUM_COUNTERS_CORE;
    } else { (*pmu).nr_arch_gp_counters = AMD64_NUM_COUNTERS; }
    (*pmu).nr_arch_gp_counters = min_t((*pmu).nr_arch_gp_counters, kvm_pmu_cap.num_counters_gp);
    if (*pmu).version > 1 {
        (*pmu).global_ctrl_rsvd = !(BIT_ULL((*pmu).nr_arch_gp_counters) - 1);
        (*pmu).global_status_rsvd = (*pmu).global_ctrl_rsvd;
    }
    (*pmu).counter_bitmask[KVM_PMC_GP] = BIT_ULL(48) - 1;
    (*pmu).reserved_bits = 0xfffffff000280000u64;
    if guest_cpu_cap_has(vcpu, X86_FEATURE_SVM) && kvm_vcpu_has_mediated_pmu(vcpu) { (*pmu).reserved_bits &= !AMD64_EVENTSEL_HOST_GUEST_MASK; }
    (*pmu).raw_event_mask = AMD64_RAW_EVENT_MASK;
    (*pmu).counter_bitmask[KVM_PMC_FIXED] = 0;
    (*pmu).nr_arch_fixed_counters = 0;
}

unsafe fn amd_pmu_init(vcpu: *mut kvm_vcpu) {
    let pmu = vcpu_to_pmu(vcpu);
    for i in 0..KVM_MAX_NR_AMD_GP_COUNTERS {
        (*pmu).gp_counters[i].type_ = KVM_PMC_GP;
        (*pmu).gp_counters[i].vcpu = vcpu;
        (*pmu).gp_counters[i].idx = i;
        (*pmu).gp_counters[i].current_config = 0;
    }
}

unsafe fn amd_pmu_is_mediated_pmu_supported(host_pmu: *mut x86_pmu_capability) -> bool { (*host_pmu).version >= 2 }

unsafe fn amd_mediated_pmu_load(vcpu: *mut kvm_vcpu) {
    let pmu = vcpu_to_pmu(vcpu); let mut global_status: u64 = 0;
    rdmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS, global_status);
    if global_status != 0 { wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR, global_status); }
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_SET, (*pmu).global_status);
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_CTL, (*pmu).global_ctrl);
}

unsafe fn amd_mediated_pmu_put(vcpu: *mut kvm_vcpu) {
    let pmu = vcpu_to_pmu(vcpu);
    wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_CTL, 0);
    rdmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS, (*pmu).global_status);
    if (*pmu).global_status != 0 { wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR, (*pmu).global_status); }
}

unsafe fn amd_pmc_is_disabled_in_current_mode(pmc: *mut kvm_pmc) -> bool {
    let vcpu = (*pmc).vcpu;
    if !kvm_vcpu_has_mediated_pmu(vcpu) { return false; }
    if WARN_ON_ONCE((*pmc).eventsel & ARCH_PERFMON_EVENTSEL_ENABLE == 0) { return false; }
    let host_guest_bits = (*pmc).eventsel & AMD64_EVENTSEL_HOST_GUEST_MASK;
    if host_guest_bits == 0 { return false; }
    if (*vcpu).arch.efer & EFER_SVME == 0 { return true; }
    if host_guest_bits == AMD64_EVENTSEL_HOST_GUEST_MASK { return false; }
    (host_guest_bits & AMD64_EVENTSEL_GUESTONLY != 0) != is_guest_mode(vcpu)
}

// The following operation table mirrors the C `amd_pmu_ops` initializer.
static mut amd_pmu_ops: kvm_pmu_ops = kvm_pmu_ops {
    rdpmc_ecx_to_pmc: Some(amd_rdpmc_ecx_to_pmc), msr_idx_to_pmc: Some(amd_msr_idx_to_pmc),
    check_rdpmc_early: Some(amd_check_rdpmc_early), is_valid_msr: Some(amd_is_valid_msr),
    get_msr: Some(amd_pmu_get_msr), set_msr: Some(amd_pmu_set_msr), refresh: Some(amd_pmu_refresh),
    init: Some(amd_pmu_init), pmc_is_disabled_in_current_mode: Some(amd_pmc_is_disabled_in_current_mode),
    is_mediated_pmu_supported: Some(amd_pmu_is_mediated_pmu_supported), mediated_load: Some(amd_mediated_pmu_load),
    mediated_put: Some(amd_mediated_pmu_put), EVENTSEL_EVENT: AMD64_EVENTSEL_EVENT,
    MAX_NR_GP_COUNTERS: KVM_MAX_NR_AMD_GP_COUNTERS, MIN_NR_GP_COUNTERS: AMD64_NUM_COUNTERS,
    PERF_GLOBAL_CTRL: MSR_AMD64_PERF_CNTR_GLOBAL_CTL, GP_EVENTSEL_BASE: MSR_F15H_PERF_CTL0,
    GP_COUNTER_BASE: MSR_F15H_PERF_CTR0, FIXED_COUNTER_BASE: 0, MSR_STRIDE: 2,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
