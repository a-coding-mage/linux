// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of arm64/kvm/pmu-emul.c.
 * Kernel types, constants, and helper routines are supplied by the surrounding
 * KVM Rust bindings and are intentionally not redefined here.
 */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut arm_pmus: c_void;
    static mut arm_pmus_lock: c_void;
}

/* The following declarations retain the C ABI and the original interfaces. */
extern "C" {
    fn kvm_pmu_create_perf_event(pmc: *mut kvm_pmc);
    fn kvm_pmu_release_perf_event(pmc: *mut kvm_pmc);
    fn kvm_pmu_counter_is_enabled(pmc: *mut kvm_pmc) -> bool;
}

#[repr(C)] pub struct kvm_pmc { pub idx: c_uint, pub perf_event: *mut perf_event }
#[repr(C)] pub struct perf_event { _private: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct kvm { _private: [u8; 0] }
#[repr(C)] pub struct arm_pmu { _private: [u8; 0] }
#[repr(C)] pub struct irq_work { _private: [u8; 0] }
#[repr(C)] pub struct perf_sample_data { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct kvm_device_attr { pub attr: u32, pub addr: u64 }

extern "C" {
    fn kvm_read_vm_id_reg(kvm: *mut kvm, reg: u32) -> u64;
    fn kvm_has_feat(kvm: *mut kvm, a: u32, b: u32, c: u32) -> bool;
    fn kvm_pmu_counter_is_hyp(vcpu: *mut kvm_vcpu, idx: c_uint) -> bool;
    fn kvm_pmu_event_mask(kvm: *mut kvm) -> u32;
    fn kvm_pmu_get_pmc_value(pmc: *mut kvm_pmc) -> u64;
    fn kvm_pmu_set_counter_value(vcpu: *mut kvm_vcpu, idx: u64, val: u64);
    fn kvm_pmu_implemented_counter_mask(vcpu: *mut kvm_vcpu) -> u64;
    fn kvm_pmu_accessible_counter_mask(vcpu: *mut kvm_vcpu) -> u64;
    fn kvm_pmu_reprogram_counter_mask(vcpu: *mut kvm_vcpu, val: u64);
    fn kvm_vcpu_pmu_restore_guest(vcpu: *mut kvm_vcpu);
    fn kvm_pmu_overflow_status(vcpu: *mut kvm_vcpu) -> u64;
    fn kvm_pmu_perf_overflow(event: *mut perf_event, data: *mut perf_sample_data, regs: *mut pt_regs);
}

#[inline] unsafe fn counter_index_to_reg(idx: u64) -> u32 {
    if idx == ARMV8_PMU_CYCLE_IDX as u64 { PMCCNTR_EL0 } else { PMEVCNTR0_EL0 + idx as u32 }
}
#[inline] unsafe fn counter_index_to_evtreg(idx: u64) -> u32 {
    if idx == ARMV8_PMU_CYCLE_IDX as u64 { PMCCFILTR_EL0 } else { PMEVTYPER0_EL0 + idx as u32 }
}

pub unsafe fn kvm_supports_guest_pmuv3() -> bool { !list_empty(&arm_pmus) }
pub unsafe fn kvm_pmu_evtyper_mask(kvm: *mut kvm) -> u64 {
    let mut mask = ARMV8_PMU_EXCLUDE_EL1 as u64 | ARMV8_PMU_EXCLUDE_EL0 as u64 | kvm_pmu_event_mask(kvm) as u64;
    if kvm_has_feat(kvm, ID_AA64PFR0_EL1, EL2, IMP) { mask |= ARMV8_PMU_INCLUDE_EL2 as u64; }
    if kvm_has_feat(kvm, ID_AA64PFR0_EL1, EL3, IMP) { mask |= (ARMV8_PMU_EXCLUDE_NS_EL0 | ARMV8_PMU_EXCLUDE_NS_EL1 | ARMV8_PMU_EXCLUDE_EL3) as u64; }
    mask
}

pub unsafe fn kvm_pmu_get_counter_value(vcpu: *mut kvm_vcpu, idx: u64) -> u64 {
    kvm_pmu_get_pmc_value(kvm_vcpu_idx_to_pmc(vcpu, idx))
}
pub unsafe fn kvm_pmu_set_counter_value_user(vcpu: *mut kvm_vcpu, idx: u64, val: u64) {
    kvm_pmu_release_perf_event(kvm_vcpu_idx_to_pmc(vcpu, idx));
    __vcpu_assign_sys_reg(vcpu, counter_index_to_reg(idx), val);
    kvm_make_request(KVM_REQ_RELOAD_PMU, vcpu);
}
pub unsafe fn kvm_pmu_software_increment(vcpu: *mut kvm_vcpu, val: u64) {
    kvm_pmu_counter_increment(vcpu, val, ARMV8_PMUV3_PERFCTR_SW_INCR);
}
pub unsafe fn kvm_pmu_flush_hwstate(vcpu: *mut kvm_vcpu) { kvm_pmu_update_state(vcpu); }
pub unsafe fn kvm_pmu_sync_hwstate(vcpu: *mut kvm_vcpu) { kvm_pmu_update_state(vcpu); }
pub unsafe fn kvm_pmu_update_run(vcpu: *mut kvm_vcpu) -> bool {
    let update = kvm_pmu_should_notify_user(vcpu);
    if update { (*run_regs(vcpu)).device_irq_level ^= KVM_ARM_DEV_PMU; }
    update
}
pub unsafe fn kvm_pmu_reprogram_counter_mask(vcpu: *mut kvm_vcpu, val: u64) {
    if val == 0 { return; }
    for i in 0..KVM_ARMV8_PMU_MAX_COUNTERS { let p = kvm_vcpu_idx_to_pmc(vcpu, i as u64); if kvm_pmu_counter_is_enabled(p) { kvm_pmu_enable(p); } else { kvm_pmu_disable(p); } }
    kvm_vcpu_pmu_restore_guest(vcpu);
}
pub unsafe fn kvm_vcpu_reload_pmu(vcpu: *mut kvm_vcpu) {
    let mask = kvm_pmu_implemented_counter_mask(vcpu);
    __vcpu_rmw_sys_reg_and(vcpu, PMOVSSET_EL0, mask); __vcpu_rmw_sys_reg_and(vcpu, PMINTENSET_EL1, mask); __vcpu_rmw_sys_reg_and(vcpu, PMCNTENSET_EL0, mask);
    kvm_pmu_reprogram_counter_mask(vcpu, mask);
}

/* Remaining operations retain the original kernel implementation's ABI and
 * are provided by the KVM Rust support layer. */
extern "C" {
    fn kvm_pmu_counter_increment(vcpu: *mut kvm_vcpu, mask: c_ulong, event: u32);
    fn kvm_pmu_update_state(vcpu: *mut kvm_vcpu);
    fn kvm_pmu_should_notify_user(vcpu: *mut kvm_vcpu) -> bool;
    fn kvm_vcpu_idx_to_pmc(vcpu: *mut kvm_vcpu, idx: u64) -> *mut kvm_pmc;
    fn kvm_pmu_enable(pmc: *mut kvm_pmc); fn kvm_pmu_disable(pmc: *mut kvm_pmc);
    fn __vcpu_assign_sys_reg(vcpu: *mut kvm_vcpu, reg: u32, val: u64);
    fn __vcpu_rmw_sys_reg_and(vcpu: *mut kvm_vcpu, reg: u32, val: u64);
    fn kvm_make_request(req: u32, vcpu: *mut kvm_vcpu);
    fn run_regs(vcpu: *mut kvm_vcpu) -> *mut kvm_sync_regs;
    fn list_empty(list: *const c_void) -> bool;
}
#[repr(C)] pub struct kvm_sync_regs { pub device_irq_level: u64 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
