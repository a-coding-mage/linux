/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Linaro Ltd.
 * Author: Shannon Zhao <shannon.zhao@linaro.org>
 */

// Translated from the C header. External types and functions are supplied by
// the surrounding kernel translation.

pub const KVM_ARMV8_PMU_MAX_COUNTERS: usize = 32;
pub const KVM_ARMV8_PMU_GICV5_IRQ: u32 = 0x20000017;

// C condition: IS_ENABLED(CONFIG_HW_PERF_EVENTS) && IS_ENABLED(CONFIG_KVM)
#[cfg(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM"))]
#[repr(C)]
pub struct kvm_pmc {
    pub idx: u8,
    pub perf_event: *mut perf_event,
}

#[cfg(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM"))]
#[repr(C)]
pub struct kvm_pmu_events {
    pub events_host: u64,
    pub events_guest: u64,
}

#[cfg(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM"))]
#[repr(C)]
pub struct kvm_pmu {
    pub overflow_work: irq_work,
    pub events: kvm_pmu_events,
    pub pmc: [kvm_pmc; KVM_ARMV8_PMU_MAX_COUNTERS],
    pub irq_num: i32,
    pub created: bool,
}

#[cfg(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM"))]
#[repr(C)]
pub struct arm_pmu_entry {
    pub entry: list_head,
    pub arm_pmu: *mut arm_pmu,
}

#[cfg(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM"))]
extern "C" {
    pub fn kvm_supports_guest_pmuv3() -> bool;
    pub fn kvm_pmu_get_counter_value(vcpu: *mut kvm_vcpu, select_idx: u64) -> u64;
    pub fn kvm_pmu_set_counter_value(vcpu: *mut kvm_vcpu, select_idx: u64, val: u64);
    pub fn kvm_pmu_set_counter_value_user(vcpu: *mut kvm_vcpu, select_idx: u64, val: u64);
    pub fn kvm_pmu_implemented_counter_mask(vcpu: *mut kvm_vcpu) -> u64;
    pub fn kvm_pmu_accessible_counter_mask(vcpu: *mut kvm_vcpu) -> u64;
    pub fn kvm_pmu_get_pmceid(vcpu: *mut kvm_vcpu, pmceid1: bool) -> u64;
    pub fn kvm_pmu_vcpu_init(vcpu: *mut kvm_vcpu);
    pub fn kvm_pmu_vcpu_destroy(vcpu: *mut kvm_vcpu);
    pub fn kvm_pmu_reprogram_counter_mask(vcpu: *mut kvm_vcpu, val: u64);
    pub fn kvm_pmu_flush_hwstate(vcpu: *mut kvm_vcpu);
    pub fn kvm_pmu_sync_hwstate(vcpu: *mut kvm_vcpu);
    pub fn kvm_pmu_should_notify_user(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_pmu_update_run(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_pmu_software_increment(vcpu: *mut kvm_vcpu, val: u64);
    pub fn kvm_pmu_handle_pmcr(vcpu: *mut kvm_vcpu, val: u64);
    pub fn kvm_pmu_set_counter_event_type(vcpu: *mut kvm_vcpu, data: u64, select_idx: u64);
    pub fn kvm_vcpu_reload_pmu(vcpu: *mut kvm_vcpu);
    pub fn kvm_arm_pmu_v3_set_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> i32;
    pub fn kvm_arm_pmu_v3_get_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> i32;
    pub fn kvm_arm_pmu_v3_has_attr(vcpu: *mut kvm_vcpu, attr: *mut kvm_device_attr) -> i32;
    pub fn kvm_arm_pmu_v3_enable(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_get_pmu_events() -> *mut kvm_pmu_events;
    pub fn kvm_vcpu_pmu_restore_guest(vcpu: *mut kvm_vcpu);
    pub fn kvm_vcpu_pmu_restore_host(vcpu: *mut kvm_vcpu);
    pub fn kvm_vcpu_pmu_resync_el0();
    pub fn kvm_arm_pmu_get_pmuver_limit() -> u8;
    pub fn kvm_pmu_evtyper_mask(kvm: *mut kvm) -> u64;
    pub fn kvm_arm_set_default_pmu(kvm: *mut kvm) -> i32;
    pub fn kvm_arm_pmu_get_max_counters(kvm: *mut kvm) -> u8;
    pub fn kvm_vcpu_read_pmcr(vcpu: *mut kvm_vcpu) -> u64;
    pub fn kvm_pmu_counter_is_hyp(vcpu: *mut kvm_vcpu, idx: u32) -> bool;
    pub fn kvm_pmu_nested_transition(vcpu: *mut kvm_vcpu);
}

#[cfg(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM"))]
#[inline]
pub unsafe fn kvm_arm_pmu_irq_initialized(v: *const kvm_vcpu) -> bool {
    (*v).arch.pmu.irq_num != 0
}

#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[repr(C)]
pub struct kvm_pmu {}

#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_supports_guest_pmuv3() -> bool { false }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_arm_pmu_irq_initialized<T>(_v: *const T) -> bool { false }

#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_get_counter_value(_vcpu: *mut kvm_vcpu, _select_idx: u64) -> u64 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_set_counter_value(_vcpu: *mut kvm_vcpu, _select_idx: u64, _val: u64) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_set_counter_value_user(_vcpu: *mut kvm_vcpu, _select_idx: u64, _val: u64) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_implemented_counter_mask(_vcpu: *mut kvm_vcpu) -> u64 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_accessible_counter_mask(_vcpu: *mut kvm_vcpu) -> u64 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_get_pmceid(_vcpu: *mut kvm_vcpu, _pmceid1: bool) -> u64 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_vcpu_init(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_vcpu_destroy(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_reprogram_counter_mask(_vcpu: *mut kvm_vcpu, _val: u64) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_flush_hwstate(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_sync_hwstate(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_pmu_should_notify_user(_vcpu: *mut kvm_vcpu) -> bool { false }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_pmu_update_run(_vcpu: *mut kvm_vcpu) -> bool { false }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_software_increment(_vcpu: *mut kvm_vcpu, _val: u64) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_handle_pmcr(_vcpu: *mut kvm_vcpu, _val: u64) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_set_counter_event_type(_vcpu: *mut kvm_vcpu, _data: u64, _select_idx: u64) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_arm_pmu_v3_set_attr(_vcpu: *mut kvm_vcpu, _attr: *mut kvm_device_attr) -> i32 { -6 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_arm_pmu_v3_get_attr(_vcpu: *mut kvm_vcpu, _attr: *mut kvm_device_attr) -> i32 { -6 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_arm_pmu_v3_has_attr(_vcpu: *mut kvm_vcpu, _attr: *mut kvm_device_attr) -> i32 { -6 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_arm_pmu_v3_enable(_vcpu: *mut kvm_vcpu) -> i32 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_vcpu_reload_pmu(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_vcpu_has_pmu<T>(_vcpu: *const T) -> bool { false }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_vcpu_has_pmuv3_strict<T>(_vcpu: *const T) -> bool { false }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_update_vcpu_events(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_vcpu_pmu_restore_guest(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_vcpu_pmu_restore_host(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub fn kvm_arm_pmu_get_pmuver_limit() -> u8 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_evtyper_mask(_kvm: *mut kvm) -> u64 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_vcpu_pmu_resync_el0() {}
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_arm_set_default_pmu(_kvm: *mut kvm) -> i32 { -19 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_arm_pmu_get_max_counters(_kvm: *mut kvm) -> u8 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_vcpu_read_pmcr(_vcpu: *mut kvm_vcpu) -> u64 { 0 }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_counter_is_hyp(_vcpu: *mut kvm_vcpu, _idx: u32) -> bool { false }
#[cfg(not(all(feature = "CONFIG_HW_PERF_EVENTS", feature = "CONFIG_KVM")))]
#[inline] pub unsafe fn kvm_pmu_nested_transition(_vcpu: *mut kvm_vcpu) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
