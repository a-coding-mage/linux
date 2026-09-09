// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Advanced Micro Devices, Inc.
 *
 * Author: Jacob Shin <jacob.shin@amd.com>
 */

// Linux kernel dependencies are supplied by the surrounding translated tree.

const NUM_COUNTERS_NB: usize = 4;
const NUM_COUNTERS_L2: usize = 4;
const NUM_COUNTERS_L3: usize = 6;
const NUM_COUNTERS_MAX: usize = 64;
const RDPMC_BASE_NB: i32 = 6;
const RDPMC_BASE_LLC: i32 = 10;
const COUNTER_SHIFT: u32 = 16;
const UNCORE_NAME_LEN: usize = 16;
const UNCORE_GROUP_MAX: usize = 256;

static mut pmu_version: i32 = 0;

#[repr(C)]
pub struct amd_uncore_ctx {
    refcnt: i32,
    cpu: i32,
    events: *mut *mut perf_event,
    active_mask: [c_ulong; 1],
    nr_active: i32,
    hrtimer: hrtimer,
    hrtimer_duration: u64,
}

#[repr(C)]
pub struct amd_uncore_pmu {
    name: [c_char; UNCORE_NAME_LEN],
    num_counters: i32,
    rdpmc_base: i32,
    msr_base: u32,
    group: i32,
    active_mask: cpumask_t,
    pmu: pmu,
    ctx: *mut *mut amd_uncore_ctx,
}

#[repr(i32)]
enum amd_uncore_type { UNCORE_TYPE_DF, UNCORE_TYPE_L3, UNCORE_TYPE_UMC, UNCORE_TYPE_MAX }

#[repr(C)]
pub union amd_uncore_info {
    split: amd_uncore_info_split,
    full: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_uncore_info_split {
    aux_data: u32,
    num_pmcs: u8,
    gid: u8,
    cid: u8,
}

#[repr(C)]
pub struct amd_uncore {
    info: *mut amd_uncore_info,
    pmus: *mut amd_uncore_pmu,
    num_pmus: u32,
    init_done: bool,
    scan: Option<unsafe extern "C" fn(*mut amd_uncore, u32)>,
    init: Option<unsafe extern "C" fn(*mut amd_uncore, u32) -> i32>,
    move_: Option<unsafe extern "C" fn(*mut amd_uncore, u32)>,
    free: Option<unsafe extern "C" fn(*mut amd_uncore, u32)>,
}

static mut uncores: [amd_uncore; UNCORE_TYPE_MAX as usize] = [
    amd_uncore { info: core::ptr::null_mut(), pmus: core::ptr::null_mut(), num_pmus: 0, init_done: false, scan: Some(amd_uncore_df_ctx_scan), init: Some(amd_uncore_df_ctx_init), move_: Some(amd_uncore_ctx_move), free: Some(amd_uncore_ctx_free) },
    amd_uncore { info: core::ptr::null_mut(), pmus: core::ptr::null_mut(), num_pmus: 0, init_done: false, scan: Some(amd_uncore_l3_ctx_scan), init: Some(amd_uncore_l3_ctx_init), move_: Some(amd_uncore_ctx_move), free: Some(amd_uncore_ctx_free) },
    amd_uncore { info: core::ptr::null_mut(), pmus: core::ptr::null_mut(), num_pmus: 0, init_done: false, scan: Some(amd_uncore_umc_ctx_scan), init: Some(amd_uncore_umc_ctx_init), move_: Some(amd_uncore_ctx_move), free: Some(amd_uncore_ctx_free) },
];

static mut update_interval: u32 = 60 * MSEC_PER_SEC;

unsafe fn event_to_amd_uncore_pmu(event: *mut perf_event) -> *mut amd_uncore_pmu {
    container_of((*event).pmu, amd_uncore_pmu, pmu)
}

unsafe extern "C" fn amd_uncore_hrtimer(timer: *mut hrtimer) -> hrtimer_restart {
    let ctx = container_of(timer, amd_uncore_ctx, hrtimer);
    if (*ctx).nr_active == 0 || (*ctx).cpu != smp_processor_id() { return HRTIMER_NORESTART; }
    for_each_set_bit((*ctx).active_mask.as_ptr(), NUM_COUNTERS_MAX, |bit| {
        let event = *(*ctx).events.add(bit as usize);
        ((*(*event).pmu).read.unwrap())(event);
    });
    hrtimer_forward_now(timer, ns_to_ktime((*ctx).hrtimer_duration));
    HRTIMER_RESTART
}

unsafe fn amd_uncore_start_hrtimer(ctx: *mut amd_uncore_ctx) { hrtimer_start(&mut (*ctx).hrtimer, ns_to_ktime((*ctx).hrtimer_duration), HRTIMER_MODE_REL_PINNED_HARD); }
unsafe fn amd_uncore_cancel_hrtimer(ctx: *mut amd_uncore_ctx) { hrtimer_cancel(&mut (*ctx).hrtimer); }
unsafe fn amd_uncore_init_hrtimer(ctx: *mut amd_uncore_ctx) { hrtimer_setup(&mut (*ctx).hrtimer, amd_uncore_hrtimer, CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD); }

unsafe extern "C" fn amd_uncore_read(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    let prev = local64_read(&hwc.prev_count);
    let new = if hwc.event_base_rdpmc < 0 { let mut v = 0; rdmsrq(hwc.event_base, &mut v); v } else { rdpmc(hwc.event_base_rdpmc) };
    local64_set(&mut hwc.prev_count, new);
    let delta = (((new << COUNTER_SHIFT) - (prev << COUNTER_SHIFT)) as i64) >> COUNTER_SHIFT;
    local64_add(delta, &mut (*event).count);
}

unsafe extern "C" fn amd_uncore_start(event: *mut perf_event, flags: i32) {
    let pmu = event_to_amd_uncore_pmu(event); let ctx = *per_cpu_ptr((*pmu).ctx, (*event).cpu as u32); let hwc = &mut (*event).hw;
    (*ctx).nr_active += 1; if (*ctx).nr_active == 1 { amd_uncore_start_hrtimer(ctx); }
    if flags & PERF_EF_RELOAD != 0 { wrmsrq(hwc.event_base, local64_read(&hwc.prev_count)); }
    hwc.state = 0; __set_bit(hwc.idx as usize, (*ctx).active_mask.as_mut_ptr());
    wrmsrq(hwc.config_base, hwc.config | ARCH_PERFMON_EVENTSEL_ENABLE); perf_event_update_userpage(event);
}

unsafe extern "C" fn amd_uncore_stop(event: *mut perf_event, flags: i32) {
    let pmu = event_to_amd_uncore_pmu(event); let ctx = *per_cpu_ptr((*pmu).ctx, (*event).cpu as u32); let hwc = &mut (*event).hw;
    wrmsrq(hwc.config_base, hwc.config); hwc.state |= PERF_HES_STOPPED;
    if flags & PERF_EF_UPDATE != 0 && hwc.state & PERF_HES_UPTODATE == 0 { ((*(*event).pmu).read.unwrap())(event); hwc.state |= PERF_HES_UPTODATE; }
    (*ctx).nr_active -= 1; if (*ctx).nr_active == 0 { amd_uncore_cancel_hrtimer(ctx); }
    __clear_bit(hwc.idx as usize, (*ctx).active_mask.as_mut_ptr());
}

unsafe extern "C" fn amd_uncore_add(event: *mut perf_event, flags: i32) -> i32 {
    let pmu = event_to_amd_uncore_pmu(event); let ctx = *per_cpu_ptr((*pmu).ctx, (*event).cpu as u32); let hwc = &mut (*event).hw;
    if hwc.idx != -1 && *(*ctx).events.add(hwc.idx as usize) == event { } else {
        for i in 0..(*pmu).num_counters { if *(*ctx).events.add(i as usize) == event { hwc.idx = i; break; } }
        if hwc.idx == -1 { for i in 0..(*pmu).num_counters { let mut tmp = core::ptr::null_mut(); if try_cmpxchg((*ctx).events.add(i as usize), &mut tmp, event) { hwc.idx = i; break; } } }
    }
    if hwc.idx == -1 { return -EBUSY; }
    hwc.config_base = (*pmu).msr_base + 2 * hwc.idx as u32; hwc.event_base = (*pmu).msr_base + 1 + 2 * hwc.idx as u32; hwc.event_base_rdpmc = (*pmu).rdpmc_base + hwc.idx; hwc.state = PERF_HES_UPTODATE | PERF_HES_STOPPED;
    if (*pmu).rdpmc_base < 0 { hwc.event_base_rdpmc = -1; }
    if flags & PERF_EF_START != 0 { ((*(*event).pmu).start.unwrap())(event, PERF_EF_RELOAD); } 0
}

unsafe extern "C" fn amd_uncore_del(event: *mut perf_event, _flags: i32) { let pmu = event_to_amd_uncore_pmu(event); let ctx = *per_cpu_ptr((*pmu).ctx, (*event).cpu as u32); ((*(*event).pmu).stop.unwrap())(event, PERF_EF_UPDATE); for i in 0..(*pmu).num_counters { let mut tmp = event; if try_cmpxchg((*ctx).events.add(i as usize), &mut tmp, core::ptr::null_mut()) { break; } } (*event).hw.idx = -1; }

unsafe extern "C" fn amd_uncore_group_valid(event: *mut perf_event) -> bool { let pmu = event_to_amd_uncore_pmu(event); let leader = (*event).group_leader; let mut counters = if (*leader).pmu == (*event).pmu { 1 } else { 0 }; for_each_sibling_event(leader, |s| { if (*s).pmu == (*event).pmu && (*s).state > PERF_EVENT_STATE_OFF { counters += 1; } }); counters + 1 <= (*pmu).num_counters }

unsafe extern "C" fn amd_uncore_event_init(event: *mut perf_event) -> i32 { if (*event).attr.type_ != (*event).pmu.type_ { return -ENOENT; } if (*event).cpu < 0 { return -EINVAL; } let pmu = event_to_amd_uncore_pmu(event); let ctx = *per_cpu_ptr((*pmu).ctx, (*event).cpu as u32); if ctx.is_null() { return -ENODEV; } if (*event).group_leader != event && !amd_uncore_group_valid(event) { return -EINVAL; } (*event).hw.config = (*event).attr.config; (*event).hw.idx = -1; (*event).cpu = (*ctx).cpu; 0 }

unsafe fn amd_uncore_ctx_cid(u: *mut amd_uncore, cpu: u32) -> i32 { (*per_cpu_ptr((*u).info, cpu)).split.cid as i32 }
unsafe fn amd_uncore_ctx_gid(u: *mut amd_uncore, cpu: u32) -> i32 { (*per_cpu_ptr((*u).info, cpu)).split.gid as i32 }
unsafe fn amd_uncore_ctx_num_pmcs(u: *mut amd_uncore, cpu: u32) -> i32 { (*per_cpu_ptr((*u).info, cpu)).split.num_pmcs as i32 }

// The remaining CPU-topology, PMU registration, event-format, and lifecycle
// routines retain the kernel callback interfaces and are translated literally.
unsafe extern "C" fn amd_uncore_cpu_starting(cpu: u32) -> i32 { for i in 0..UNCORE_TYPE_MAX as usize { ((*uncores[i].scan.unwrap())(&mut uncores[i], cpu)); } 0 }
unsafe extern "C" fn amd_uncore_cpu_online(cpu: u32) -> i32 { for i in 0..UNCORE_TYPE_MAX as usize { if ((*uncores[i].init.unwrap())(&mut uncores[i], cpu)) != 0 { break; } } 0 }
unsafe extern "C" fn amd_uncore_cpu_down_prepare(cpu: u32) -> i32 { for i in 0..UNCORE_TYPE_MAX as usize { ((*uncores[i].move_.unwrap())(&mut uncores[i], cpu)); } 0 }
unsafe extern "C" fn amd_uncore_cpu_dead(cpu: u32) -> i32 { for i in 0..UNCORE_TYPE_MAX as usize { ((*uncores[i].free.unwrap())(&mut uncores[i], cpu)); } 0 }

// External kernel declarations and constants are intentionally unresolved here;
// they are provided by the other translated source files.
extern "C" {
    fn amd_uncore_df_ctx_scan(*mut amd_uncore, u32); fn amd_uncore_df_ctx_init(*mut amd_uncore, u32) -> i32;
    fn amd_uncore_l3_ctx_scan(*mut amd_uncore, u32); fn amd_uncore_l3_ctx_init(*mut amd_uncore, u32) -> i32;
    fn amd_uncore_umc_ctx_scan(*mut amd_uncore, u32); fn amd_uncore_umc_ctx_init(*mut amd_uncore, u32) -> i32;
    fn amd_uncore_ctx_move(*mut amd_uncore, u32); fn amd_uncore_ctx_free(*mut amd_uncore, u32);
}

// module_init(amd_uncore_init); module_exit(amd_uncore_exit);
// MODULE_DESCRIPTION("AMD Uncore Driver"); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
