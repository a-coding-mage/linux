/* SPDX-License-Identifier: GPL-2.0 */

/* Linux dependencies and build-time static-call machinery are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct KvmPmuOps {
    pub rdpmc_ecx_to_pmc: Option<unsafe extern "C" fn(*mut KvmVcpu, u32, *mut u64) -> *mut KvmPmc>,
    pub msr_idx_to_pmc: Option<unsafe extern "C" fn(*mut KvmVcpu, u32) -> *mut KvmPmc>,
    pub check_rdpmc_early: Option<unsafe extern "C" fn(*mut KvmVcpu, u32) -> i32>,
    pub is_valid_msr: Option<unsafe extern "C" fn(*mut KvmVcpu, u32) -> bool>,
    pub get_msr: Option<unsafe extern "C" fn(*mut KvmVcpu, *mut MsrData) -> i32>,
    pub set_msr: Option<unsafe extern "C" fn(*mut KvmVcpu, *mut MsrData) -> i32>,
    pub refresh: Option<unsafe extern "C" fn(*mut KvmVcpu)>,
    pub init: Option<unsafe extern "C" fn(*mut KvmVcpu)>,
    pub reset: Option<unsafe extern "C" fn(*mut KvmVcpu)>,
    pub deliver_pmi: Option<unsafe extern "C" fn(*mut KvmVcpu)>,
    pub cleanup: Option<unsafe extern "C" fn(*mut KvmVcpu)>,
    pub pmc_is_disabled_in_current_mode: Option<unsafe extern "C" fn(*mut KvmPmc) -> bool>,
    pub is_mediated_pmu_supported: Option<unsafe extern "C" fn(*mut X86PmuCapability) -> bool>,
    pub mediated_load: Option<unsafe extern "C" fn(*mut KvmVcpu)>,
    pub mediated_put: Option<unsafe extern "C" fn(*mut KvmVcpu)>,
    pub write_global_ctrl: Option<unsafe extern "C" fn(u64)>,
    pub eventsel_event: u64,
    pub max_nr_gp_counters: i32,
    pub min_nr_gp_counters: i32,
    pub perf_global_ctrl: u32,
    pub gp_eventsel_base: u32,
    pub gp_counter_base: u32,
    pub fixed_counter_base: u32,
    pub msr_stride: u32,
}

pub const MSR_IA32_MISC_ENABLE_PMU_RO_MASK: u64 = MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL | MSR_IA32_MISC_ENABLE_BTS_UNAVAIL;
pub const VMWARE_BACKDOOR_PMC_HOST_TSC: u32 = 0x10000;
pub const VMWARE_BACKDOOR_PMC_REAL_TIME: u32 = 0x10001;
pub const VMWARE_BACKDOOR_PMC_APPARENT_TIME: u32 = 0x10002;
pub const KVM_FIXED_PMC_BASE_IDX: i32 = INTEL_PMC_IDX_FIXED;

#[inline]
pub unsafe fn kvm_pmu_has_perf_global_ctrl(pmu: *mut KvmPmu) -> bool { (*pmu).version > 1 }

#[inline]
pub unsafe fn kvm_vcpu_has_mediated_pmu(vcpu: *mut KvmVcpu) -> bool {
    enable_mediated_pmu && (*vcpu).arch.pmu.version != 0
}

#[inline]
pub unsafe fn kvm_pmc_idx_to_pmc(pmu: *mut KvmPmu, mut idx: i32) -> *mut KvmPmc {
    if idx < (*pmu).nr_arch_gp_counters { return (*pmu).gp_counters.as_mut_ptr().add(idx as usize); }
    idx -= KVM_FIXED_PMC_BASE_IDX;
    if idx >= 0 && idx < (*pmu).nr_arch_fixed_counters { return (*pmu).fixed_counters.as_mut_ptr().add(idx as usize); }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn pmc_bitmask(pmc: *mut KvmPmc) -> u64 {
    (*pmc_to_pmu(pmc)).counter_bitmask[(*pmc).type_ as usize]
}

#[inline]
pub unsafe fn pmc_read_counter(pmc: *mut KvmPmc) -> u64 {
    if kvm_vcpu_has_mediated_pmu((*pmc).vcpu) { return (*pmc).counter & pmc_bitmask(pmc); }
    let mut counter = (*pmc).counter.wrapping_add((*pmc).emulated_counter);
    if !(*pmc).perf_event.is_null() && !(*pmc).is_paused {
        let mut enabled = 0u64; let mut running = 0u64;
        counter = counter.wrapping_add(perf_event_read_value((*pmc).perf_event, &mut enabled, &mut running));
    }
    counter & pmc_bitmask(pmc)
}

#[inline]
pub unsafe fn pmc_is_gp(pmc: *mut KvmPmc) -> bool { (*pmc).type_ == KVM_PMC_GP }
#[inline]
pub unsafe fn pmc_is_fixed(pmc: *mut KvmPmc) -> bool { (*pmc).type_ == KVM_PMC_FIXED }
#[inline]
pub unsafe fn kvm_valid_perf_global_ctrl(pmu: *mut KvmPmu, data: u64) -> bool { ((*pmu).global_ctrl_rsvd & data) == 0 }

#[inline]
pub unsafe fn get_gp_pmc(pmu: *mut KvmPmu, msr: u32, base: u32) -> *mut KvmPmc {
    if msr >= base && msr < base + (*pmu).nr_arch_gp_counters as u32 {
        return (*pmu).gp_counters.as_mut_ptr().add((msr - base) as usize);
    }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn get_fixed_pmc(pmu: *mut KvmPmu, msr: u32) -> *mut KvmPmc {
    let base = MSR_CORE_PERF_FIXED_CTR0;
    if msr >= base && msr < base + (*pmu).nr_arch_fixed_counters as u32 { return (*pmu).fixed_counters.as_mut_ptr().add((msr - base) as usize); }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn pmc_is_globally_enabled(pmc: *mut KvmPmc) -> bool {
    let pmu = pmc_to_pmu(pmc); !kvm_pmu_has_perf_global_ctrl(pmu) || test_bit((*pmc).idx, &(*pmu).global_ctrl)
}

#[inline]
pub unsafe fn kvm_pmu_request_counters_reprogram(pmu: *mut KvmPmu, counters: u64) {
    if counters == 0 { return; }
    atomic64_or(counters, &mut (*pmu).__reprogram_pmi);
    kvm_make_request(KVM_REQ_PMU, pmu_to_vcpu(pmu));
}

#[inline]
pub unsafe fn kvm_pmu_request_counter_reprogram_inline(pmc: *mut KvmPmc) {
    kvm_pmu_recalc_pmc_emulation(pmc_to_pmu(pmc), pmc);
    set_bit((*pmc).idx, &mut (*pmc_to_pmu(pmc)).reprogram_pmi);
    kvm_make_request(KVM_REQ_PMU, (*pmc).vcpu);
}

pub const fn kvm_pmu_call(_func: &str) -> usize { 0 }

extern "C" {
    fn set_bit(bit: i32, addr: *mut u64);
    fn kvm_pmu_deliver_pmi(vcpu: *mut KvmVcpu);
    fn kvm_pmu_rdpmc(vcpu: *mut KvmVcpu, pmc: u32, data: *mut u64) -> i32;
    fn kvm_pmu_check_rdpmc_early(vcpu: *mut KvmVcpu, idx: u32) -> i32;
    fn kvm_pmu_is_valid_msr(vcpu: *mut KvmVcpu, msr: u32) -> bool;
    fn kvm_pmu_get_msr(vcpu: *mut KvmVcpu, msr_info: *mut MsrData) -> i32;
    fn kvm_pmu_set_msr(vcpu: *mut KvmVcpu, msr_info: *mut MsrData) -> i32;
    fn kvm_pmu_refresh(vcpu: *mut KvmVcpu);
    fn kvm_pmu_init(vcpu: *mut KvmVcpu);
    fn kvm_pmu_cleanup(vcpu: *mut KvmVcpu);
    fn kvm_pmu_destroy(vcpu: *mut KvmVcpu);
    fn kvm_pmu_instruction_retired(vcpu: *mut KvmVcpu);
    fn kvm_pmu_branch_retired(vcpu: *mut KvmVcpu);
    fn kvm_mediated_pmu_load(vcpu: *mut KvmVcpu);
    fn kvm_mediated_pmu_put(vcpu: *mut KvmVcpu);
}

#[inline] pub unsafe fn vcpu_to_pmu(vcpu: *mut KvmVcpu) -> *mut KvmPmu { &mut (*vcpu).arch.pmu }
#[inline] pub unsafe fn pmu_to_vcpu(pmu: *mut KvmPmu) -> *mut KvmVcpu { container_of_pmu(pmu) }
#[inline] pub unsafe fn pmc_to_pmu(pmc: *mut KvmPmc) -> *mut KvmPmu { &mut (*(*pmc).vcpu).arch.pmu }

#[inline]
pub unsafe fn kvm_pmu_is_fastpath_emulation_allowed(vcpu: *mut KvmVcpu) -> bool {
    let pmu = vcpu_to_pmu(vcpu);
    !kvm_vcpu_has_mediated_pmu(vcpu) || !bitmap_intersects((*pmu).pmc_counting_instructions.as_ptr(), &(*pmu).global_ctrl, X86_PMC_IDX_MAX)
}

#[inline]
pub unsafe fn pmc_is_locally_enabled(pmc: *mut KvmPmc) -> bool {
    let pmu = pmc_to_pmu(pmc);
    if pmc_is_fixed(pmc) {
        return (fixed_ctrl_field((*pmu).fixed_ctr_ctrl, ((*pmc).idx - KVM_FIXED_PMC_BASE_IDX) as u32)
            & (INTEL_FIXED_0_KERNEL | INTEL_FIXED_0_USER)) != 0;
    }
    if ((*pmc).eventsel & ARCH_PERFMON_EVENTSEL_ENABLE) == 0 { return false; }
    if !test_bit((*pmc).idx, &(*pmu).pmc_has_mode_specific_enables) { return true; }
    !pmc_is_disabled_in_current_mode(pmc)
}

#[inline] pub fn fixed_ctrl_field(ctrl_reg: u64, idx: u32) -> u64 { (ctrl_reg >> (idx * INTEL_FIXED_BITS_STRIDE)) & INTEL_FIXED_BITS_MASK }

extern "C" {
    fn container_of_pmu(pmu: *mut KvmPmu) -> *mut KvmVcpu;
    fn perf_event_read_value(event: *mut core::ffi::c_void, enabled: *mut u64, running: *mut u64) -> u64;
    fn test_bit(bit: i32, addr: *const u64) -> bool;
    fn atomic64_or(value: u64, addr: *mut u64);
    fn kvm_make_request(req: i32, vcpu: *mut KvmVcpu);
    fn bitmap_intersects(a: *const u64, b: *const u64, bits: i32) -> bool;
    fn pmc_is_disabled_in_current_mode(pmc: *mut KvmPmc) -> bool;
    fn kvm_pmu_recalc_pmc_emulation(pmu: *mut KvmPmu, pmc: *mut KvmPmc);
    fn kvm_pmu_handle_event(vcpu: *mut KvmVcpu);
    fn kvm_pmu_request_counter_reprogram(pmc: *mut KvmPmc);
    fn kvm_init_pmu_capability(pmu_ops: *mut KvmPmuOps);
    fn kvm_vm_ioctl_set_pmu_event_filter(kvm: *mut Kvm, argp: *mut core::ffi::c_void) -> i32;
    fn is_vmware_backdoor_pmc(pmc_idx: u32) -> bool;
    fn kvm_need_perf_global_ctrl_intercept(vcpu: *mut KvmVcpu) -> bool;
    fn kvm_need_rdpmc_intercept(vcpu: *mut KvmVcpu) -> bool;
}

extern "C" {
    static mut enable_pmu: bool;
    static mut enable_mediated_pmu: bool;
    static mut kvm_pmu_cap: X86PmuCapability;
    static mut intel_pmu_ops: KvmPmuOps;
    static mut amd_pmu_ops: KvmPmuOps;
    fn kvm_pmu_ops_update(pmu_ops: *const KvmPmuOps);
    fn kvm_handle_guest_mediated_pmi();
    fn pmc_write_counter(pmc: *mut KvmPmc, val: u64);
    fn kvm_pmu_deliver_pmi(vcpu: *mut KvmVcpu);
    fn kvm_pmu_rdpmc(vcpu: *mut KvmVcpu, pmc: u32, data: *mut u64) -> i32;
    fn kvm_pmu_check_rdpmc_early(vcpu: *mut KvmVcpu, idx: u32) -> i32;
    fn kvm_pmu_is_valid_msr(vcpu: *mut KvmVcpu, msr: u32) -> bool;
    fn kvm_pmu_get_msr(vcpu: *mut KvmVcpu, msr_info: *mut MsrData) -> i32;
    fn kvm_pmu_set_msr(vcpu: *mut KvmVcpu, msr_info: *mut MsrData) -> i32;
    fn kvm_pmu_refresh(vcpu: *mut KvmVcpu);
    fn kvm_pmu_init(vcpu: *mut KvmVcpu);
    fn kvm_pmu_cleanup(vcpu: *mut KvmVcpu);
    fn kvm_pmu_destroy(vcpu: *mut KvmVcpu);
    fn kvm_pmu_instruction_retired(vcpu: *mut KvmVcpu);
    fn kvm_pmu_branch_retired(vcpu: *mut KvmVcpu);
    fn kvm_mediated_pmu_load(vcpu: *mut KvmVcpu);
    fn kvm_mediated_pmu_put(vcpu: *mut KvmVcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
