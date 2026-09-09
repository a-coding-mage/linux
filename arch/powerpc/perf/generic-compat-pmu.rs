// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019 Madhavan Srinivasan, IBM Corporation.

// #define pr_fmt(fmt) "generic-compat-pmu: " fmt
// #include "isa207-common.h"

/*
 * Raw event encoding:
 *
 *        60        56        52        48        44        40        36        32
 * | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - |
 *
 *        28        24        20        16        12         8         4         0
 * | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - |
 *                                 [ pmc ]                       [    pmcxsel    ]
 */

/* Event codes defined in ISA v3.0B */
pub const PM_CYC_ALT: u32 = 0x100f0;
pub const PM_CYC_INST_CMPL: u32 = 0x100f2;
pub const PM_FLOP_CMPL: u32 = 0x100f4;
pub const PM_L1_ITLB_MISS: u32 = 0x100f6;
pub const PM_NO_INST_AVAIL: u32 = 0x100f8;
pub const PM_LD_CMPL: u32 = 0x100fc;
pub const PM_INST_CMPL_ALT: u32 = 0x100fe;
pub const PM_ST_CMPL: u32 = 0x200f0;
pub const PM_INST_DISP: u32 = 0x200f2;
pub const PM_RUN_CYC: u32 = 0x200f4;
pub const PM_L1_DTLB_RELOAD: u32 = 0x200f6;
pub const PM_BR_TAKEN_CMPL: u32 = 0x200fa;
pub const PM_L1_ICACHE_MISS: u32 = 0x200fc;
pub const PM_L1_RELOAD_FROM_MEM: u32 = 0x200fe;
pub const PM_ST_MISS_L1: u32 = 0x300f0;
pub const PM_INST_DISP_ALT: u32 = 0x300f2;
pub const PM_BR_MISPREDICT: u32 = 0x300f6;
pub const PM_DTLB_MISS: u32 = 0x300fc;
pub const PM_DATA_FROM_L3MISS: u32 = 0x300fe;
pub const PM_LD_MISS_L1: u32 = 0x400f0;
pub const PM_CYC_INST_DISP: u32 = 0x400f2;
pub const PM_BR_MPRED_CMPL: u32 = 0x400f6;
pub const PM_RUN_INST_CMPL: u32 = 0x400fa;
pub const PM_ITLB_MISS: u32 = 0x400fc;
pub const PM_LD_NOT_CACHED: u32 = 0x400fe;
pub const PM_INST_CMPL: u32 = 0x500fa;
pub const PM_CYC: u32 = 0x600f4;

/* Table of alternatives, sorted in increasing order of column 0. */
pub static GENERIC_EVENT_ALTERNATIVES: [[u32; MAX_ALT]; 3] = [
    [PM_CYC_ALT, PM_CYC],
    [PM_INST_CMPL_ALT, PM_INST_CMPL],
    [PM_INST_DISP, PM_INST_DISP_ALT],
];

unsafe fn generic_get_alternatives(event: u64, flags: u32, alt: *mut u64) -> i32 {
    isa207_get_alternatives(
        event,
        alt,
        GENERIC_EVENT_ALTERNATIVES.len(),
        flags,
        GENERIC_EVENT_ALTERNATIVES.as_ptr(),
    )
}

// GENERIC_EVENT_ATTR!(cpu_cycles, PM_CYC);
// GENERIC_EVENT_ATTR!(instructions, PM_INST_CMPL);
// GENERIC_EVENT_ATTR!(stalled_cycles_frontend, PM_NO_INST_AVAIL);
// GENERIC_EVENT_ATTR!(branch_misses, PM_BR_MPRED_CMPL);
// GENERIC_EVENT_ATTR!(cache_misses, PM_LD_MISS_L1);
// CACHE_EVENT_ATTR!(L1_dcache_load_misses, PM_LD_MISS_L1);
// CACHE_EVENT_ATTR!(L1_dcache_store_misses, PM_ST_MISS_L1);
// CACHE_EVENT_ATTR!(L1_icache_load_misses, PM_L1_ICACHE_MISS);
// CACHE_EVENT_ATTR!(LLC_load_misses, PM_DATA_FROM_L3MISS);
// CACHE_EVENT_ATTR!(branch_load_misses, PM_BR_MPRED_CMPL);
// CACHE_EVENT_ATTR!(dTLB_load_misses, PM_DTLB_MISS);
// CACHE_EVENT_ATTR!(iTLB_load_misses, PM_ITLB_MISS);

static mut GENERIC_COMPAT_EVENTS_ATTR: [*mut attribute; 13] = [
    GENERIC_EVENT_PTR!(PM_CYC), GENERIC_EVENT_PTR!(PM_INST_CMPL),
    GENERIC_EVENT_PTR!(PM_NO_INST_AVAIL), GENERIC_EVENT_PTR!(PM_BR_MPRED_CMPL),
    GENERIC_EVENT_PTR!(PM_LD_MISS_L1), CACHE_EVENT_PTR!(PM_LD_MISS_L1),
    CACHE_EVENT_PTR!(PM_ST_MISS_L1), CACHE_EVENT_PTR!(PM_L1_ICACHE_MISS),
    CACHE_EVENT_PTR!(PM_DATA_FROM_L3MISS), CACHE_EVENT_PTR!(PM_BR_MPRED_CMPL),
    CACHE_EVENT_PTR!(PM_DTLB_MISS), CACHE_EVENT_PTR!(PM_ITLB_MISS), core::ptr::null_mut(),
];

static GENERIC_COMPAT_PMU_EVENTS_GROUP: attribute_group = attribute_group {
    name: "events", attrs: GENERIC_COMPAT_EVENTS_ATTR.as_ptr() as *mut *mut attribute,
};

// PMU_FORMAT_ATTR!(event, "config:0-19");
// PMU_FORMAT_ATTR!(pmcxsel, "config:0-7");
// PMU_FORMAT_ATTR!(pmc, "config:16-19");
static GENERIC_COMPAT_PMU_FORMAT_ATTR: [*mut attribute; 4] = [
    &format_attr_event.attr, &format_attr_pmcxsel.attr, &format_attr_pmc.attr, core::ptr::null_mut(),
];
static GENERIC_COMPAT_PMU_FORMAT_GROUP: attribute_group = attribute_group {
    name: "format", attrs: GENERIC_COMPAT_PMU_FORMAT_ATTR.as_ptr() as *mut *mut attribute,
};
static mut GENERIC_COMPAT_PMU_CAPS_ATTRS: [*mut attribute; 1] = [core::ptr::null_mut()];
static mut GENERIC_COMPAT_PMU_CAPS_GROUP: attribute_group = attribute_group {
    name: "caps", attrs: GENERIC_COMPAT_PMU_CAPS_ATTRS.as_ptr() as *mut *mut attribute,
};
static GENERIC_COMPAT_PMU_ATTR_GROUPS: [*const attribute_group; 4] = [
    &GENERIC_COMPAT_PMU_FORMAT_GROUP, &GENERIC_COMPAT_PMU_EVENTS_GROUP,
    &GENERIC_COMPAT_PMU_CAPS_GROUP, core::ptr::null(),
];

static mut COMPAT_GENERIC_EVENTS: [i32; 5] = [
    PM_CYC as i32, PM_INST_CMPL as i32, PM_NO_INST_AVAIL as i32,
    PM_BR_MPRED_CMPL as i32, PM_LD_MISS_L1 as i32,
];

/* Generalized cache-related events; 0 means unsupported, -1 nonsensical. */
static mut GENERIC_COMPAT_CACHE_EVENTS: [[[i64; RESULT_MAX]; OP_MAX]; MAX] = [[[0; RESULT_MAX]; OP_MAX]; MAX];

unsafe fn generic_compute_mmcr(
    event: *mut u64, n_ev: i32, hwc: *mut u32, mmcr: *mut mmcr_regs,
    pevents: *mut *mut perf_event, flags: u32,
) -> i32 {
    let ret = isa207_compute_mmcr(event, n_ev, hwc, mmcr, pevents, flags);
    if ret == 0 { (*mmcr).mmcr0 |= MMCR0_C56RUN; }
    ret
}

static mut GENERIC_COMPAT_PMU: power_pmu = power_pmu {
    name: "ISAv3", n_counter: MAX_PMU_COUNTERS, add_fields: ISA207_ADD_FIELDS,
    test_adder: ISA207_TEST_ADDER, compute_mmcr: generic_compute_mmcr,
    get_constraint: isa207_get_constraint, get_alternatives: generic_get_alternatives,
    disable_pmc: isa207_disable_pmc, flags: PPMU_HAS_SIER | PPMU_ARCH_207S,
    n_generic: COMPAT_GENERIC_EVENTS.len(), generic_events: COMPAT_GENERIC_EVENTS.as_mut_ptr(),
    cache_events: &mut GENERIC_COMPAT_CACHE_EVENTS, attr_groups: GENERIC_COMPAT_PMU_ATTR_GROUPS.as_ptr(),
};

pub unsafe fn init_generic_compat_pmu() -> i32 {
    if !cpu_has_feature(CPU_FTR_ARCH_300) { return -ENODEV; }
    let rc = register_power_pmu(&mut GENERIC_COMPAT_PMU);
    if rc != 0 { return rc; }
    (*cur_cpu_spec).cpu_user_features2 |= PPC_FEATURE2_EBB;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
