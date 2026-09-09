/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/include/asm/pmu.h
 *
 *  Copyright (C) 2009 picoChip Designs Ltd, Jamie Iles
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARMPMU_MAX_HWEVENTS: usize = if cfg!(feature = "CONFIG_ARM") { 32 } else { 33 };

#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARMPMU_EVT_64BIT: u32 = 0x00001;
#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARMPMU_EVT_47BIT: u32 = 0x00002;
#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARMPMU_EVT_63BIT: u32 = 0x00004;

#[cfg(feature = "CONFIG_ARM_PMU")]
pub const HW_OP_UNSUPPORTED: u32 = 0xFFFF;
#[cfg(feature = "CONFIG_ARM_PMU")]
pub const CACHE_OP_UNSUPPORTED: u32 = 0xFFFF;

/* C macros PERF_MAP_ALL_UNSUPPORTED and PERF_CACHE_MAP_ALL_UNSUPPORTED use
 * designated range initializers; their equivalent initializers depend on the
 * imported PERF_COUNT_HW_* constants and are retained by this declaration. */

#[cfg(feature = "CONFIG_ARM_PMU")]
#[repr(C)]
pub struct pmu_hw_events {
    pub events: [*mut perf_event; ARMPMU_MAX_HWEVENTS],
    pub used_mask: [usize; (ARMPMU_MAX_HWEVENTS + usize::BITS as usize - 1) / usize::BITS as usize],
    pub percpu_pmu: *mut arm_pmu,
    pub irq: i32,
    pub branch_stack: *mut perf_branch_stack,
    pub branch_users: u32,
}

#[cfg(feature = "CONFIG_ARM_PMU")]
#[repr(C)]
pub enum armpmu_attr_groups {
    ARMPMU_ATTR_GROUP_COMMON,
    ARMPMU_ATTR_GROUP_EVENTS,
    ARMPMU_ATTR_GROUP_FORMATS,
    ARMPMU_ATTR_GROUP_CAPS,
    ARMPMU_NR_ATTR_GROUPS,
}

#[cfg(feature = "CONFIG_ARM_PMU")]
#[repr(C)]
pub struct arm_pmu {
    pub pmu: pmu,
    pub supported_cpus: cpumask_t,
    pub name: *mut i8,
    pub handle_irq: Option<unsafe extern "C" fn(*mut arm_pmu) -> irqreturn_t>,
    pub enable: Option<unsafe extern "C" fn(*mut perf_event)>,
    pub disable: Option<unsafe extern "C" fn(*mut perf_event)>,
    pub get_event_idx: Option<unsafe extern "C" fn(*mut pmu_hw_events, *mut perf_event) -> i32>,
    pub clear_event_idx: Option<unsafe extern "C" fn(*mut pmu_hw_events, *mut perf_event)>,
    pub set_event_filter: Option<unsafe extern "C" fn(*mut hw_perf_event, *mut perf_event_attr) -> i32>,
    pub read_counter: Option<unsafe extern "C" fn(*mut perf_event) -> u64>,
    pub write_counter: Option<unsafe extern "C" fn(*mut perf_event, u64)>,
    pub start: Option<unsafe extern "C" fn(*mut arm_pmu)>,
    pub stop: Option<unsafe extern "C" fn(*mut arm_pmu)>,
    pub reset: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub map_event: Option<unsafe extern "C" fn(*mut perf_event) -> i32>,
    pub map_pmuv3_event: Option<unsafe extern "C" fn(u32) -> i32>,
    pub cntr_mask: [usize; (ARMPMU_MAX_HWEVENTS + usize::BITS as usize - 1) / usize::BITS as usize],
    pub secure_access: bool,
    pub plat_device: *mut platform_device,
    pub hw_events: *mut pmu_hw_events,
    pub node: hlist_node,
    pub cpu_pm_nb: notifier_block,
    pub attr_groups: [*const attribute_group; armpmu_attr_groups::ARMPMU_NR_ATTR_GROUPS as usize + 1],
    pub pmuver: i32,
    pub avoid_pmccntr: bool,
    pub reg_pmmir: u64,
    pub reg_brbidr: u64,
    pub pmceid_bitmap: [usize; (0x40 + usize::BITS as usize - 1) / usize::BITS as usize],
    pub pmceid_ext_bitmap: [usize; (0x40 + usize::BITS as usize - 1) / usize::BITS as usize],
    pub acpi_cpuid: usize,
}

#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARMV8_PMUV3_MAX_COMMON_EVENTS: usize = 0x40;
#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARMV8_PMUV3_EXT_COMMON_EVENT_BASE: u32 = 0x4000;

#[cfg(feature = "CONFIG_ARM_PMU")]
pub type armpmu_init_fn = unsafe extern "C" fn(*mut arm_pmu) -> i32;

#[cfg(feature = "CONFIG_ARM_PMU")]
#[inline]
pub unsafe fn to_arm_pmu(p: *mut pmu) -> *mut arm_pmu {
    /* Equivalent to container_of(p, struct arm_pmu, pmu). */
    (p as *mut u8).sub(core::mem::offset_of!(arm_pmu, pmu)) as *mut arm_pmu
}

#[cfg(feature = "CONFIG_ARM_PMU")]
#[repr(C)]
pub struct pmu_probe_info {
    pub cpuid: u32,
    pub mask: u32,
    pub init: Option<armpmu_init_fn>,
}

#[cfg(feature = "CONFIG_ARM_PMU")]
#[macro_export]
macro_rules! PMU_PROBE {
    ($cpuid:expr, $mask:expr, $fn_:expr) => {
        $crate::pmu_probe_info { cpuid: $cpuid, mask: $mask, init: Some($fn_) }
    };
}

#[cfg(feature = "CONFIG_ARM_PMU")]
#[macro_export]
macro_rules! ARM_PMU_PROBE {
    ($cpuid:expr, $fn_:expr) => { $crate::PMU_PROBE!($cpuid, ARM_CPU_PART_MASK, $fn_) };
}

#[cfg(feature = "CONFIG_ARM_PMU")]
#[macro_export]
macro_rules! XSCALE_PMU_PROBE {
    ($version:expr, $fn_:expr) => {
        $crate::PMU_PROBE!((ARM_CPU_IMP_INTEL << 24) | $version, $crate::ARM_PMU_XSCALE_MASK, $fn_)
    };
}

#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARM_PMU_XSCALE_MASK: u32 = (0xff << 24) | ARM_CPU_XSCALE_ARCH_MASK;

#[cfg(feature = "CONFIG_ARM_PMU")]
pub const ARMV8_PMU_PDEV_NAME: &str = "armv8-pmu";
pub const ARMV8_SPE_PDEV_NAME: &str = "arm,spe-v1";
pub const ARMV8_TRBE_PDEV_NAME: &str = "arm,trbe";

#[cfg(feature = "CONFIG_ARM_PMU")]
extern "C" {
    pub fn armpmu_event_update(event: *mut perf_event) -> u64;
    pub fn armpmu_event_set_period(event: *mut perf_event) -> i32;
    pub fn armpmu_map_event(
        event: *mut perf_event,
        event_map: *const [u32; PERF_COUNT_HW_MAX as usize],
        cache_map: *const [[[u32; PERF_COUNT_HW_CACHE_RESULT_MAX as usize]; PERF_COUNT_HW_CACHE_OP_MAX as usize]; PERF_COUNT_HW_CACHE_MAX as usize],
        raw_event_mask: u32,
    ) -> i32;
    pub fn arm_pmu_device_probe(pdev: *mut platform_device, of_table: *const of_device_id, probe_table: *const pmu_probe_info) -> i32;
    pub fn arm_pmu_irq_is_nmi() -> bool;
    pub fn armpmu_alloc() -> *mut arm_pmu;
    pub fn armpmu_free(pmu: *mut arm_pmu);
    pub fn armpmu_register(pmu: *mut arm_pmu) -> i32;
    pub fn armpmu_request_irq(armpmu: *mut pmu_hw_events, irq: i32, cpu: i32) -> i32;
    pub fn armpmu_free_irq(armpmu: *mut pmu_hw_events, irq: i32, cpu: i32);
}

#[cfg(all(feature = "CONFIG_ARM_PMU", feature = "CONFIG_ACPI"))]
extern "C" { pub fn arm_pmu_acpi_probe(init_fn: armpmu_init_fn) -> i32; }
#[cfg(all(feature = "CONFIG_ARM_PMU", not(feature = "CONFIG_ACPI")))]
pub unsafe fn arm_pmu_acpi_probe(_init_fn: armpmu_init_fn) -> i32 { 0 }

#[cfg(all(feature = "CONFIG_ARM_PMU", feature = "CONFIG_KVM"))]
extern "C" { pub fn kvm_host_pmu_init(pmu: *mut arm_pmu); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
