// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance monitoring support for Virtual Processor Area(VPA) based counters
 *
 * Copyright (C) 2024 IBM Corporation
 */

// Kernel includes and symbols are supplied by the surrounding kernel crate.

pub const MODULE_VERS: &str = "1.0";
pub const MODULE_NAME: &str = "pseries_vpa_pmu";

pub const L1_TO_L2_CS_LAT: u32 = 0x1;
pub const L2_TO_L1_CS_LAT: u32 = 0x2;
pub const L2_RUNTIME_AGG: u32 = 0x3;

extern "C" {
    fn kvmhv_get_l1_to_l2_cs_time_vcpu() -> u64;
    fn kvmhv_get_l1_to_l2_cs_time() -> u64;
    fn kvmhv_get_l2_to_l1_cs_time_vcpu() -> u64;
    fn kvmhv_get_l2_to_l1_cs_time() -> u64;
    fn kvmhv_get_l2_runtime_agg_vcpu() -> u64;
    fn kvmhv_get_l2_runtime_agg() -> u64;
    fn kvmhv_set_l2_counters_status(cpu: u32, enabled: bool);
    fn smp_processor_id() -> u32;
    fn firmware_has_feature(feature: u64) -> bool;
    fn is_kvm_guest() -> bool;
    fn perf_pmu_register(pmu: *mut Pmu, name: *const i8, cpu: i32) -> i32;
    fn perf_pmu_unregister(pmu: *mut Pmu);
    fn pr_info(message: *const i8);
}

pub const FW_FEATURE_LPAR: u64 = 0; // supplied by asm/kvm_ppc.h
pub const PERF_ATTACH_TASK: u32 = 1;
pub const PERF_PMU_CAP_NO_EXCLUDE: u64 = 1;
pub const PERF_PMU_CAP_NO_INTERRUPT: u64 = 2;
pub const PERF_SW_CONTEXT: i32 = 0;

#[repr(C)]
pub struct Local64 { pub value: u64 }

#[repr(C)]
pub struct PerfEventAttr { pub type_: u32, pub config: u64 }

#[repr(C)]
pub struct PerfEventHw { pub prev_count: Local64 }

#[repr(C)]
pub struct PerfPmu { pub type_: u32 }

#[repr(C)]
pub struct PerfEvent {
    pub attr: PerfEventAttr,
    pub pmu: *mut PerfPmu,
    pub attach_state: u32,
    pub hw: PerfEventHw,
    pub count: Local64,
}

#[repr(C)]
pub struct Pmu {
    pub module: *mut core::ffi::c_void,
    pub task_ctx_nr: i32,
    pub name: *const i8,
    pub event_init: Option<unsafe extern "C" fn(*mut PerfEvent) -> i32>,
    pub add: Option<unsafe extern "C" fn(*mut PerfEvent, i32) -> i32>,
    pub del: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    pub read: Option<unsafe extern "C" fn(*mut PerfEvent)>,
    pub attr_groups: *const *const core::ffi::c_void,
    pub capabilities: u64,
}

unsafe extern "C" {
    fn is_sampling_event(event: *mut PerfEvent) -> bool;
    fn has_branch_stack(event: *mut PerfEvent) -> bool;
    fn local64_set(ptr: *mut Local64, value: u64);
    fn local64_read(ptr: *const Local64) -> u64;
    fn local64_add(value: u64, ptr: *mut Local64);
}

unsafe fn get_counter_data(event: *mut PerfEvent) -> u64 {
    let config = (*event).attr.config as u32;
    match config {
        L1_TO_L2_CS_LAT => if (*event).attach_state & PERF_ATTACH_TASK != 0 {
            kvmhv_get_l1_to_l2_cs_time_vcpu()
        } else { kvmhv_get_l1_to_l2_cs_time() },
        L2_TO_L1_CS_LAT => if (*event).attach_state & PERF_ATTACH_TASK != 0 {
            kvmhv_get_l2_to_l1_cs_time_vcpu()
        } else { kvmhv_get_l2_to_l1_cs_time() },
        L2_RUNTIME_AGG => if (*event).attach_state & PERF_ATTACH_TASK != 0 {
            kvmhv_get_l2_runtime_agg_vcpu()
        } else { kvmhv_get_l2_runtime_agg() },
        _ => 0,
    }
}

unsafe extern "C" fn vpa_pmu_event_init(event: *mut PerfEvent) -> i32 {
    if (*event).attr.type_ != (*(*event).pmu).type_ { return -2; }
    if is_sampling_event(event) { return -95; }
    if has_branch_stack(event) { return -95; }
    if (*event).attr.config == 0 || (*event).attr.config > 3 { return -22; }
    0
}

unsafe extern "C" fn vpa_pmu_add(event: *mut PerfEvent, _flags: i32) -> i32 {
    kvmhv_set_l2_counters_status(smp_processor_id(), true);
    let data = get_counter_data(event);
    local64_set(&mut (*event).hw.prev_count, data);
    0
}

unsafe extern "C" fn vpa_pmu_read(event: *mut PerfEvent) {
    let prev_data = local64_read(&(*event).hw.prev_count);
    let new_data = get_counter_data(event);
    let final_data = new_data.wrapping_sub(prev_data);
    local64_add(final_data, &mut (*event).count);
}

unsafe extern "C" fn vpa_pmu_del(event: *mut PerfEvent, _flags: i32) {
    vpa_pmu_read(event);
    /* Disable vpa counter accumulation */
    kvmhv_set_l2_counters_status(smp_processor_id(), false);
}

static mut VPA_PMU: Pmu = Pmu {
    module: core::ptr::null_mut(),
    task_ctx_nr: PERF_SW_CONTEXT,
    name: b"vpa_pmu\0".as_ptr() as *const i8,
    event_init: Some(vpa_pmu_event_init),
    add: Some(vpa_pmu_add),
    del: Some(vpa_pmu_del),
    read: Some(vpa_pmu_read),
    attr_groups: core::ptr::null(),
    capabilities: PERF_PMU_CAP_NO_EXCLUDE | PERF_PMU_CAP_NO_INTERRUPT,
};

#[no_mangle]
pub unsafe extern "C" fn pseries_vpa_pmu_init() -> i32 {
    if !firmware_has_feature(FW_FEATURE_LPAR) || is_kvm_guest() { return -19; }
    perf_pmu_register(&mut VPA_PMU, VPA_PMU.name, -1);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pseries_vpa_pmu_cleanup() {
    perf_pmu_unregister(&mut VPA_PMU);
}

// module_init(pseries_vpa_pmu_init);
// module_exit(pseries_vpa_pmu_cleanup);
// MODULE_DESCRIPTION("Perf Driver for pSeries VPA pmu counter");
// MODULE_AUTHOR("Kajol Jain <kjain@linux.ibm.com>");
// MODULE_AUTHOR("Madhavan Srinivasan <maddy@linux.ibm.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
