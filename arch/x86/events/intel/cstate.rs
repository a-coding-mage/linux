/*
 * Support cstate residency counters
 *
 * Copyright (C) 2015, Intel Corp.
 * Author: Kan Liang (kan.liang@intel.com)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 */

// The original C file includes kernel headers supplying the types, constants,
// macros, and external functions referenced below.

#[repr(C)]
struct CstateModel {
    core_events: libc::c_ulong,
    pkg_events: libc::c_ulong,
    module_events: libc::c_ulong,
    quirks: libc::c_ulong,
}

const SLM_PKG_C6_USE_C7_MSR: libc::c_ulong = 1 << 0;
const KNL_CORE_C6_MSR: libc::c_ulong = 1 << 1;

static mut cstate_core_pmu: pmu = unsafe { core::mem::zeroed() };
static mut has_cstate_core: bool = false;

#[repr(C)]
#[derive(Copy, Clone)]
enum PerfCstateCoreEvents {
    PERF_CSTATE_CORE_C1_RES = 0,
    PERF_CSTATE_CORE_C3_RES,
    PERF_CSTATE_CORE_C6_RES,
    PERF_CSTATE_CORE_C7_RES,
    PERF_CSTATE_CORE_EVENT_MAX,
}

PMU_EVENT_ATTR_STRING!(c1-residency, attr_cstate_core_c1, "event=0x00");
PMU_EVENT_ATTR_STRING!(c3-residency, attr_cstate_core_c3, "event=0x01");
PMU_EVENT_ATTR_STRING!(c6-residency, attr_cstate_core_c6, "event=0x02");
PMU_EVENT_ATTR_STRING!(c7-residency, attr_cstate_core_c7, "event=0x03");

static mut core_msr_mask: libc::c_ulong = 0;
PMU_EVENT_GROUP!(events, cstate_core_c1);
PMU_EVENT_GROUP!(events, cstate_core_c3);
PMU_EVENT_GROUP!(events, cstate_core_c6);
PMU_EVENT_GROUP!(events, cstate_core_c7);

unsafe fn test_msr(idx: i32, data: *mut libc::c_void) -> bool {
    test_bit(idx, data as *mut libc::c_ulong)
}

static mut core_msr: [perf_msr; 4] = [
    perf_msr { msr: MSR_CORE_C1_RES, group: &group_cstate_core_c1, test: test_msr },
    perf_msr { msr: MSR_CORE_C3_RESIDENCY, group: &group_cstate_core_c3, test: test_msr },
    perf_msr { msr: MSR_CORE_C6_RESIDENCY, group: &group_cstate_core_c6, test: test_msr },
    perf_msr { msr: MSR_CORE_C7_RESIDENCY, group: &group_cstate_core_c7, test: test_msr },
];

static mut attrs_empty: [*mut attribute; 1] = [core::ptr::null_mut()];
static mut cstate_events_attr_group: attribute_group = attribute_group { name: "events", attrs: attrs_empty.as_mut_ptr() };

DEFINE_CSTATE_FORMAT_ATTR!(cstate_event, event, "config:0-63");
static mut cstate_format_attrs: [*mut attribute; 2] = [unsafe { &mut format_attr_cstate_event.attr }, core::ptr::null_mut()];
static mut cstate_format_attr_group: attribute_group = attribute_group { name: "format", attrs: cstate_format_attrs.as_mut_ptr() };
static mut cstate_attr_groups: [*const attribute_group; 3] = [&cstate_events_attr_group, &cstate_format_attr_group, core::ptr::null()];

static mut cstate_pkg_pmu: pmu = unsafe { core::mem::zeroed() };
static mut has_cstate_pkg: bool = false;

#[repr(C)]
enum PerfCstatePkgEvents {
    PERF_CSTATE_PKG_C2_RES = 0,
    PERF_CSTATE_PKG_C3_RES,
    PERF_CSTATE_PKG_C6_RES,
    PERF_CSTATE_PKG_C7_RES,
    PERF_CSTATE_PKG_C8_RES,
    PERF_CSTATE_PKG_C9_RES,
    PERF_CSTATE_PKG_C10_RES,
    PERF_CSTATE_PKG_EVENT_MAX,
}

PMU_EVENT_ATTR_STRING!(c2-residency, attr_cstate_pkg_c2, "event=0x00");
PMU_EVENT_ATTR_STRING!(c3-residency, attr_cstate_pkg_c3, "event=0x01");
PMU_EVENT_ATTR_STRING!(c6-residency, attr_cstate_pkg_c6, "event=0x02");
PMU_EVENT_ATTR_STRING!(c7-residency, attr_cstate_pkg_c7, "event=0x03");
PMU_EVENT_ATTR_STRING!(c8-residency, attr_cstate_pkg_c8, "event=0x04");
PMU_EVENT_ATTR_STRING!(c9-residency, attr_cstate_pkg_c9, "event=0x05");
PMU_EVENT_ATTR_STRING!(c10-residency, attr_cstate_pkg_c10, "event=0x06");
static mut pkg_msr_mask: libc::c_ulong = 0;
PMU_EVENT_GROUP!(events, cstate_pkg_c2);
PMU_EVENT_GROUP!(events, cstate_pkg_c3);
PMU_EVENT_GROUP!(events, cstate_pkg_c6);
PMU_EVENT_GROUP!(events, cstate_pkg_c7);
PMU_EVENT_GROUP!(events, cstate_pkg_c8);
PMU_EVENT_GROUP!(events, cstate_pkg_c9);
PMU_EVENT_GROUP!(events, cstate_pkg_c10);
static mut pkg_msr: [perf_msr; 7] = [
    perf_msr { msr: MSR_PKG_C2_RESIDENCY, group: &group_cstate_pkg_c2, test: test_msr },
    perf_msr { msr: MSR_PKG_C3_RESIDENCY, group: &group_cstate_pkg_c3, test: test_msr },
    perf_msr { msr: MSR_PKG_C6_RESIDENCY, group: &group_cstate_pkg_c6, test: test_msr },
    perf_msr { msr: MSR_PKG_C7_RESIDENCY, group: &group_cstate_pkg_c7, test: test_msr },
    perf_msr { msr: MSR_PKG_C8_RESIDENCY, group: &group_cstate_pkg_c8, test: test_msr },
    perf_msr { msr: MSR_PKG_C9_RESIDENCY, group: &group_cstate_pkg_c9, test: test_msr },
    perf_msr { msr: MSR_PKG_C10_RESIDENCY, group: &group_cstate_pkg_c10, test: test_msr },
];

static mut cstate_module_pmu: pmu = unsafe { core::mem::zeroed() };
static mut has_cstate_module: bool = false;
const PERF_CSTATE_MODULE_C6_RES: usize = 0;
const PERF_CSTATE_MODULE_EVENT_MAX: usize = 1;
PMU_EVENT_ATTR_STRING!(c6-residency, attr_cstate_module_c6, "event=0x00");
static mut module_msr_mask: libc::c_ulong = 0;
PMU_EVENT_GROUP!(events, cstate_module_c6);
static mut module_msr: [perf_msr; 1] = [perf_msr { msr: MSR_MODULE_C6_RES_MS, group: &group_cstate_module_c6, test: test_msr }];

unsafe fn cstate_pmu_event_init(event: *mut perf_event) -> i32 {
    let mut cfg = (*event).attr.config;
    if (*event).attr.type_ != (*(*event).pmu).type_ { return -ENOENT; }
    if (*event).attr.sample_period != 0 { return -EINVAL; }
    if (*event).cpu < 0 { return -EINVAL; }
    if (*event).pmu == &cstate_core_pmu {
        if cfg >= PERF_CSTATE_CORE_EVENT_MAX as u64 { return -EINVAL; }
        cfg = array_index_nospec(cfg as libc::c_ulong, PERF_CSTATE_CORE_EVENT_MAX);
        if core_msr_mask & (1 << cfg) == 0 { return -EINVAL; }
        (*event).hw.event_base = core_msr[cfg as usize].msr;
    } else if (*event).pmu == &cstate_pkg_pmu {
        if cfg >= PERF_CSTATE_PKG_EVENT_MAX as u64 { return -EINVAL; }
        cfg = array_index_nospec(cfg as libc::c_ulong, PERF_CSTATE_PKG_EVENT_MAX);
        if pkg_msr_mask & (1 << cfg) == 0 { return -EINVAL; }
        (*event).hw.event_base = pkg_msr[cfg as usize].msr;
    } else if (*event).pmu == &cstate_module_pmu {
        if cfg >= PERF_CSTATE_MODULE_EVENT_MAX as u64 { return -EINVAL; }
        cfg = array_index_nospec(cfg as libc::c_ulong, PERF_CSTATE_MODULE_EVENT_MAX);
        if module_msr_mask & (1 << cfg) == 0 { return -EINVAL; }
        (*event).hw.event_base = module_msr[cfg as usize].msr;
    } else { return -ENOENT; }
    (*event).hw.config = cfg;
    (*event).hw.idx = -1;
    0
}

unsafe fn cstate_pmu_read_counter(event: *mut perf_event) -> u64 {
    let mut val = 0u64;
    rdmsrq((*event).hw.event_base, val);
    val
}

unsafe fn cstate_pmu_event_update(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    let mut prev_raw_count = local64_read(&hwc.prev_count);
    let new_raw_count;
    loop {
        new_raw_count = cstate_pmu_read_counter(event);
        if local64_try_cmpxchg(&mut hwc.prev_count, &mut prev_raw_count, new_raw_count) { break; }
    }
    local64_add(new_raw_count.wrapping_sub(prev_raw_count), &mut (*event).count);
}

unsafe fn cstate_pmu_event_start(event: *mut perf_event, _mode: i32) { local64_set(&mut (*event).hw.prev_count, cstate_pmu_read_counter(event)); }
unsafe fn cstate_pmu_event_stop(event: *mut perf_event, _mode: i32) { cstate_pmu_event_update(event); }
unsafe fn cstate_pmu_event_del(event: *mut perf_event, _mode: i32) { cstate_pmu_event_stop(event, PERF_EF_UPDATE); }
unsafe fn cstate_pmu_event_add(event: *mut perf_event, mode: i32) -> i32 {
    if mode & PERF_EF_START != 0 { cstate_pmu_event_start(event, mode); }
    0
}

// Model tables and PMU registrations are represented with the original kernel
// macros so their externally supplied definitions remain dependencies.
static nhm_cstates: CstateModel = CstateModel { core_events: BIT!(1) | BIT!(2), pkg_events: BIT!(1) | BIT!(2) | BIT!(3), module_events: 0, quirks: 0 };
static snb_cstates: CstateModel = CstateModel { core_events: BIT!(1) | BIT!(2) | BIT!(3), pkg_events: BIT!(0) | BIT!(1) | BIT!(2) | BIT!(3), module_events: 0, quirks: 0 };
static hswult_cstates: CstateModel = CstateModel { core_events: BIT!(1) | BIT!(2) | BIT!(3), pkg_events: BIT!(0) | BIT!(1) | BIT!(2) | BIT!(3) | BIT!(4) | BIT!(5) | BIT!(6), module_events: 0, quirks: 0 };
static cnl_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(1) | BIT!(2) | BIT!(3), pkg_events: BIT!(0) | BIT!(1) | BIT!(2) | BIT!(3) | BIT!(4) | BIT!(5) | BIT!(6), module_events: 0, quirks: 0 };
static icl_cstates: CstateModel = CstateModel { core_events: BIT!(2) | BIT!(3), pkg_events: BIT!(0) | BIT!(1) | BIT!(2) | BIT!(3) | BIT!(4) | BIT!(5) | BIT!(6), module_events: 0, quirks: 0 };
static icx_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(2), pkg_events: BIT!(0) | BIT!(2), module_events: 0, quirks: 0 };
static adl_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(2) | BIT!(3), pkg_events: BIT!(0) | BIT!(1) | BIT!(2) | BIT!(4) | BIT!(6), module_events: 0, quirks: 0 };
static lnl_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(2) | BIT!(3), pkg_events: BIT!(0) | BIT!(2) | BIT!(6), module_events: 0, quirks: 0 };
static nvl_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(2) | BIT!(3), pkg_events: BIT!(0) | BIT!(2) | BIT!(6), module_events: BIT!(0), quirks: 0 };
static slm_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(2), pkg_events: BIT!(2), module_events: 0, quirks: SLM_PKG_C6_USE_C7_MSR };
static knl_cstates: CstateModel = CstateModel { core_events: BIT!(2), pkg_events: BIT!(0) | BIT!(1) | BIT!(2), module_events: 0, quirks: KNL_CORE_C6_MSR };
static glm_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(1) | BIT!(2), pkg_events: BIT!(0) | BIT!(1) | BIT!(2) | BIT!(6), module_events: 0, quirks: 0 };
static grr_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(2), pkg_events: 0, module_events: BIT!(0), quirks: 0 };
static srf_cstates: CstateModel = CstateModel { core_events: BIT!(0) | BIT!(2), pkg_events: BIT!(0) | BIT!(2), module_events: BIT!(0), quirks: 0 };

static intel_cstates_match: [x86_cpu_id; 59] = [
    X86_MATCH_VFM!(INTEL_NEHALEM, &nhm_cstates), X86_MATCH_VFM!(INTEL_NEHALEM_EP, &nhm_cstates), X86_MATCH_VFM!(INTEL_NEHALEM_EX, &nhm_cstates),
    X86_MATCH_VFM!(INTEL_WESTMERE, &nhm_cstates), X86_MATCH_VFM!(INTEL_WESTMERE_EP, &nhm_cstates), X86_MATCH_VFM!(INTEL_WESTMERE_EX, &nhm_cstates),
    X86_MATCH_VFM!(INTEL_SANDYBRIDGE, &snb_cstates), X86_MATCH_VFM!(INTEL_SANDYBRIDGE_X, &snb_cstates), X86_MATCH_VFM!(INTEL_IVYBRIDGE, &snb_cstates), X86_MATCH_VFM!(INTEL_IVYBRIDGE_X, &snb_cstates),
    X86_MATCH_VFM!(INTEL_HASWELL, &snb_cstates), X86_MATCH_VFM!(INTEL_HASWELL_X, &snb_cstates), X86_MATCH_VFM!(INTEL_HASWELL_G, &snb_cstates), X86_MATCH_VFM!(INTEL_HASWELL_L, &hswult_cstates),
    X86_MATCH_VFM!(INTEL_ATOM_SILVERMONT, &slm_cstates), X86_MATCH_VFM!(INTEL_ATOM_SILVERMONT_D, &slm_cstates), X86_MATCH_VFM!(INTEL_ATOM_AIRMONT, &slm_cstates), X86_MATCH_VFM!(INTEL_ATOM_AIRMONT_NP, &slm_cstates),
    X86_MATCH_VFM!(INTEL_BROADWELL, &snb_cstates), X86_MATCH_VFM!(INTEL_BROADWELL_D, &snb_cstates), X86_MATCH_VFM!(INTEL_BROADWELL_G, &snb_cstates), X86_MATCH_VFM!(INTEL_BROADWELL_X, &snb_cstates),
    X86_MATCH_VFM!(INTEL_SKYLAKE_L, &snb_cstates), X86_MATCH_VFM!(INTEL_SKYLAKE, &snb_cstates), X86_MATCH_VFM!(INTEL_SKYLAKE_X, &snb_cstates), X86_MATCH_VFM!(INTEL_KABYLAKE_L, &hswult_cstates), X86_MATCH_VFM!(INTEL_KABYLAKE, &hswult_cstates), X86_MATCH_VFM!(INTEL_COMETLAKE_L, &hswult_cstates), X86_MATCH_VFM!(INTEL_COMETLAKE, &hswult_cstates),
    X86_MATCH_VFM!(INTEL_CANNONLAKE_L, &cnl_cstates), X86_MATCH_VFM!(INTEL_XEON_PHI_KNL, &knl_cstates), X86_MATCH_VFM!(INTEL_XEON_PHI_KNM, &knl_cstates),
    X86_MATCH_VFM!(INTEL_ATOM_GOLDMONT, &glm_cstates), X86_MATCH_VFM!(INTEL_ATOM_GOLDMONT_D, &glm_cstates), X86_MATCH_VFM!(INTEL_ATOM_GOLDMONT_PLUS, &glm_cstates), X86_MATCH_VFM!(INTEL_ATOM_TREMONT_D, &glm_cstates), X86_MATCH_VFM!(INTEL_ATOM_TREMONT, &glm_cstates), X86_MATCH_VFM!(INTEL_ATOM_TREMONT_L, &glm_cstates), X86_MATCH_VFM!(INTEL_ATOM_GRACEMONT, &adl_cstates), X86_MATCH_VFM!(INTEL_ATOM_CRESTMONT_X, &srf_cstates), X86_MATCH_VFM!(INTEL_ATOM_CRESTMONT, &grr_cstates), X86_MATCH_VFM!(INTEL_ATOM_DARKMONT_X, &srf_cstates),
    X86_MATCH_VFM!(INTEL_ICELAKE_L, &icl_cstates), X86_MATCH_VFM!(INTEL_ICELAKE, &icl_cstates), X86_MATCH_VFM!(INTEL_ICELAKE_X, &icx_cstates), X86_MATCH_VFM!(INTEL_ICELAKE_D, &icx_cstates), X86_MATCH_VFM!(INTEL_SAPPHIRERAPIDS_X, &icx_cstates), X86_MATCH_VFM!(INTEL_EMERALDRAPIDS_X, &icx_cstates), X86_MATCH_VFM!(INTEL_GRANITERAPIDS_X, &icx_cstates), X86_MATCH_VFM!(INTEL_GRANITERAPIDS_D, &icx_cstates), X86_MATCH_VFM!(INTEL_DIAMONDRAPIDS_X, &srf_cstates),
    X86_MATCH_VFM!(INTEL_TIGERLAKE_L, &icl_cstates), X86_MATCH_VFM!(INTEL_TIGERLAKE, &icl_cstates), X86_MATCH_VFM!(INTEL_ROCKETLAKE, &icl_cstates), X86_MATCH_VFM!(INTEL_ALDERLAKE, &adl_cstates), X86_MATCH_VFM!(INTEL_ALDERLAKE_L, &adl_cstates), X86_MATCH_VFM!(INTEL_RAPTORLAKE, &adl_cstates), X86_MATCH_VFM!(INTEL_RAPTORLAKE_P, &adl_cstates), X86_MATCH_VFM!(INTEL_RAPTORLAKE_S, &adl_cstates), X86_MATCH_VFM!(INTEL_METEORLAKE, &adl_cstates), X86_MATCH_VFM!(INTEL_METEORLAKE_L, &adl_cstates), X86_MATCH_VFM!(INTEL_ARROWLAKE, &adl_cstates), X86_MATCH_VFM!(INTEL_ARROWLAKE_H, &adl_cstates), X86_MATCH_VFM!(INTEL_ARROWLAKE_U, &adl_cstates), X86_MATCH_VFM!(INTEL_LUNARLAKE_M, &lnl_cstates), X86_MATCH_VFM!(INTEL_PANTHERLAKE_L, &lnl_cstates), X86_MATCH_VFM!(INTEL_WILDCATLAKE_L, &lnl_cstates), X86_MATCH_VFM!(INTEL_NOVALAKE, &nvl_cstates), X86_MATCH_VFM!(INTEL_NOVALAKE_L, &nvl_cstates),
    X86_MATCH_VFM!(0, core::ptr::null()),
];

unsafe fn cstate_probe(cm: *const CstateModel) -> i32 {
    if (*cm).quirks & SLM_PKG_C6_USE_C7_MSR != 0 { pkg_msr[2].msr = MSR_PKG_C7_RESIDENCY; }
    if (*cm).quirks & KNL_CORE_C6_MSR != 0 { pkg_msr[2].msr = MSR_KNL_CORE_C6_RESIDENCY; }
    core_msr_mask = perf_msr_probe(core_msr.as_mut_ptr(), 4, true, &(*cm).core_events as *const _ as *mut _);
    pkg_msr_mask = perf_msr_probe(pkg_msr.as_mut_ptr(), 7, true, &(*cm).pkg_events as *const _ as *mut _);
    module_msr_mask = perf_msr_probe(module_msr.as_mut_ptr(), 1, true, &(*cm).module_events as *const _ as *mut _);
    has_cstate_core = core_msr_mask != 0; has_cstate_pkg = pkg_msr_mask != 0; has_cstate_module = module_msr_mask != 0;
    if has_cstate_core || has_cstate_pkg || has_cstate_module { 0 } else { -ENODEV }
}

unsafe fn cstate_cleanup() {
    if has_cstate_core { perf_pmu_unregister(&mut cstate_core_pmu); }
    if has_cstate_pkg { perf_pmu_unregister(&mut cstate_pkg_pmu); }
    if has_cstate_module { perf_pmu_unregister(&mut cstate_module_pmu); }
}

unsafe fn cstate_init() -> i32 {
    let mut err;
    if has_cstate_core { err = perf_pmu_register(&mut cstate_core_pmu, cstate_core_pmu.name, -1); if err != 0 { has_cstate_core = false; pr_info!("Failed to register cstate core pmu\n"); cstate_cleanup(); return err; } }
    if has_cstate_pkg {
        if topology_max_dies_per_package() > 1 { cstate_pkg_pmu.scope = PERF_PMU_SCOPE_DIE; err = perf_pmu_register(&mut cstate_pkg_pmu, "cstate_die", -1); } else { err = perf_pmu_register(&mut cstate_pkg_pmu, cstate_pkg_pmu.name, -1); }
        if err != 0 { has_cstate_pkg = false; pr_info!("Failed to register cstate pkg pmu\n"); cstate_cleanup(); return err; }
    }
    if has_cstate_module { err = perf_pmu_register(&mut cstate_module_pmu, cstate_module_pmu.name, -1); if err != 0 { has_cstate_module = false; pr_info!("Failed to register cstate cluster pmu\n"); cstate_cleanup(); return err; } }
    0
}

unsafe fn cstate_pmu_init() -> i32 {
    if boot_cpu_has(X86_FEATURE_HYPERVISOR) { return -ENODEV; }
    let id = x86_match_cpu(intel_cstates_match.as_ptr());
    if id.is_null() { return -ENODEV; }
    let err = cstate_probe((*id).driver_data as *const CstateModel);
    if err != 0 { return err; }
    cstate_init()
}

unsafe fn cstate_pmu_exit() { cstate_cleanup(); }
module_init!(cstate_pmu_init);
module_exit!(cstate_pmu_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
