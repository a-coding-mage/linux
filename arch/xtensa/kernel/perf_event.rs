// SPDX-License-Identifier: GPL-2.0-only
/*
 * Xtensa Performance Monitor Module driver
 * See Tensilica Debug User's Guide for PMU registers documentation.
 *
 * Copyright (C) 2015 Cadence Design Systems Inc.
 */

// Linux and Xtensa headers supply the external types, constants, and functions
// referenced below. Build-time XCHAL_HW_MIN_VERSION selects the PMU base.

const XTENSA_HWVERSION_RG_2015_0: u32 = 260000;
#[cfg(any())]
const XTENSA_PMU_ERI_BASE: u32 = 0x00101000;
#[cfg(not(any()))]
const XTENSA_PMU_ERI_BASE: u32 = 0x00001000;

const XTENSA_PMU_PMG: u32 = XTENSA_PMU_ERI_BASE;
const XTENSA_PMU_PMG_PMEN: u32 = 0x1;
const XTENSA_PMU_COUNTER_MASK: u64 = 0xffff_ffff;
const XTENSA_PMU_COUNTER_MAX: i64 = 0x7fff_ffff;
const XTENSA_PMU_PMCTRL_INTEN: u32 = 0x00000001;
const XTENSA_PMU_PMCTRL_KRNLCNT: u32 = 0x00000008;
const XTENSA_PMU_PMCTRL_TRACELEVEL: u32 = 0x000000f0;
const XTENSA_PMU_PMCTRL_SELECT_SHIFT: u32 = 8;
const XTENSA_PMU_PMCTRL_SELECT: u32 = 0x00001f00;
const XTENSA_PMU_PMCTRL_MASK_SHIFT: u32 = 16;
const XTENSA_PMU_PMCTRL_MASK: u32 = 0xffff0000;
const XTENSA_PMU_PMSTAT_OVFL: u32 = 0x00000001;
const XTENSA_PMU_PMSTAT_INTASRT: u32 = 0x00000010;

#[inline]
const fn xtensa_pmu_pm(i: u32) -> u32 { XTENSA_PMU_ERI_BASE + 0x80 + i * 4 }
#[inline]
const fn xtensa_pmu_pmctrl(i: u32) -> u32 { XTENSA_PMU_ERI_BASE + 0x100 + i * 4 }
#[inline]
const fn xtensa_pmu_pmstat(i: u32) -> u32 { XTENSA_PMU_ERI_BASE + 0x180 + i * 4 }
#[inline]
const fn xtensa_pmu_mask(select: u32, mask: u32) -> u32 {
    (select << XTENSA_PMU_PMCTRL_SELECT_SHIFT) |
        (mask << XTENSA_PMU_PMCTRL_MASK_SHIFT) |
        XTENSA_PMU_PMCTRL_TRACELEVEL | XTENSA_PMU_PMCTRL_INTEN
}

#[repr(C)]
struct XtensaPmuEvents {
    event: [*mut PerfEvent; XCHAL_NUM_PERF_COUNTERS],
    used_mask: [c_ulong; BITS_TO_LONGS(XCHAL_NUM_PERF_COUNTERS)],
}

static mut XTENSA_PMU_EVENTS: PerCpu<XtensaPmuEvents> = DEFINE_PER_CPU!();

static XTENSA_HW_CTL: [u32; 7] = [
    xtensa_pmu_mask(0, 0x1), xtensa_pmu_mask(2, 0xffff),
    xtensa_pmu_mask(10, 0x1), xtensa_pmu_mask(12, 0x1),
    xtensa_pmu_mask(2, 0x490), xtensa_pmu_mask(4, 0x1ff),
    xtensa_pmu_mask(3, 0x1ff),
];

static XTENSA_CACHE_CTL: [[[u32; RESULT_MAX]; OP_MAX]; CACHE_MAX] = [[[0; RESULT_MAX]; OP_MAX]; CACHE_MAX];

unsafe fn xtensa_pmu_cache_event(config: u64) -> c_int {
    let cache_type = ((config >> 0) & 0xff) as usize;
    let cache_op = ((config >> 8) & 0xff) as usize;
    let cache_result = ((config >> 16) & 0xff) as usize;
    if cache_type >= XTENSA_CACHE_CTL.len() || cache_op >= OP_MAX || cache_result >= RESULT_MAX { return -EINVAL; }
    let ret = XTENSA_CACHE_CTL[cache_type][cache_op][cache_result] as c_int;
    if ret == 0 { return -EINVAL; }
    ret
}

unsafe fn xtensa_pmu_read_counter(idx: c_int) -> u32 { get_er(xtensa_pmu_pm(idx as u32)) }
unsafe fn xtensa_pmu_write_counter(idx: c_int, v: u32) { set_er(v, xtensa_pmu_pm(idx as u32)); }

unsafe fn xtensa_perf_event_update(event: *mut PerfEvent, hwc: *mut HwPerfEvent, idx: c_int) {
    let (prev_raw_count, new_raw_count);
    loop {
        prev_raw_count = local64_read(&(*hwc).prev_count);
        new_raw_count = xtensa_pmu_read_counter((*event).hw.idx);
        if local64_cmpxchg(&(*hwc).prev_count, prev_raw_count, new_raw_count as i64) == prev_raw_count { break; }
    }
    let delta = ((new_raw_count as u64).wrapping_sub(prev_raw_count as u64) & XTENSA_PMU_COUNTER_MASK) as i64;
    local64_add(delta, &(*event).count);
    local64_sub(delta, &(*hwc).period_left);
}

unsafe fn xtensa_perf_event_set_period(event: *mut PerfEvent, hwc: *mut HwPerfEvent, idx: c_int) -> bool {
    let mut rc = false;
    let mut left: i64;
    if !is_sampling_event(event) { left = XTENSA_PMU_COUNTER_MAX; }
    else {
        let period = (*hwc).sample_period;
        left = local64_read(&(*hwc).period_left);
        if left <= -period || left <= 0 {
            if left <= -period { left = period; } else { left += period; }
            local64_set(&(*hwc).period_left, left); (*hwc).last_period = period; rc = true;
        }
        if left > XTENSA_PMU_COUNTER_MAX { left = XTENSA_PMU_COUNTER_MAX; }
    }
    local64_set(&(*hwc).prev_count, -left);
    xtensa_pmu_write_counter(idx, (-left) as u32);
    perf_event_update_userpage(event); rc
}

unsafe fn xtensa_pmu_enable(_pmu: *mut Pmu) { set_er(get_er(XTENSA_PMU_PMG) | XTENSA_PMU_PMG_PMEN, XTENSA_PMU_PMG); }
unsafe fn xtensa_pmu_disable(_pmu: *mut Pmu) { set_er(get_er(XTENSA_PMU_PMG) & !XTENSA_PMU_PMG_PMEN, XTENSA_PMU_PMG); }

unsafe fn xtensa_pmu_event_init(event: *mut PerfEvent) -> c_int {
    match (*event).attr.type_ {
        PERF_TYPE_HARDWARE => { if (*event).attr.config as usize >= XTENSA_HW_CTL.len() || XTENSA_HW_CTL[(*event).attr.config as usize] == 0 { return -EINVAL; } (*event).hw.config = XTENSA_HW_CTL[(*event).attr.config as usize] as u64; 0 }
        PERF_TYPE_HW_CACHE => { let ret = xtensa_pmu_cache_event((*event).attr.config); if ret < 0 { return ret; } (*event).hw.config = ret as u64; 0 }
        PERF_TYPE_RAW => { if ((*event).attr.config as u32 & XTENSA_PMU_PMCTRL_SELECT) == (1 << XTENSA_PMU_PMCTRL_SELECT_SHIFT) { return -EINVAL; } (*event).hw.config = ((*event).attr.config as u32 & (XTENSA_PMU_PMCTRL_KRNLCNT | XTENSA_PMU_PMCTRL_TRACELEVEL | XTENSA_PMU_PMCTRL_SELECT | XTENSA_PMU_PMCTRL_MASK) | XTENSA_PMU_PMCTRL_INTEN) as u64; 0 }
        _ => -ENOENT,
    }
}

// The remaining PMU callbacks mirror the C driver and depend on kernel APIs/types.
unsafe fn xtensa_pmu_start(event: *mut PerfEvent, flags: c_int) { let hwc = &mut (*event).hw; let idx = hwc.idx; if idx == -1 { return; } if flags & PERF_EF_RELOAD != 0 { xtensa_perf_event_set_period(event, hwc, idx); } hwc.state = 0; set_er(hwc.config as u32, xtensa_pmu_pmctrl(idx as u32)); }
unsafe fn xtensa_pmu_stop(event: *mut PerfEvent, flags: c_int) { let hwc = &mut (*event).hw; let idx = hwc.idx; if hwc.state & PERF_HES_STOPPED == 0 { set_er(0, xtensa_pmu_pmctrl(idx as u32)); set_er(get_er(xtensa_pmu_pmstat(idx as u32)), xtensa_pmu_pmstat(idx as u32)); hwc.state |= PERF_HES_STOPPED; } if flags & PERF_EF_UPDATE != 0 && hwc.state & PERF_HES_UPTODATE == 0 { xtensa_perf_event_update(event, hwc, idx); hwc.state |= PERF_HES_UPTODATE; } }

unsafe fn xtensa_pmu_add(event: *mut PerfEvent, flags: c_int) -> c_int {
    let ev = this_cpu_ptr(&mut XTENSA_PMU_EVENTS);
    let hwc = &mut (*event).hw;
    let mut idx = hwc.idx;
    if test_and_set_bit(idx as usize, (*ev).used_mask.as_mut_ptr()) {
        idx = find_first_zero_bit((*ev).used_mask.as_ptr(), XCHAL_NUM_PERF_COUNTERS) as c_int;
        if idx == XCHAL_NUM_PERF_COUNTERS as c_int { return -EAGAIN; }
        set_bit(idx as usize, (*ev).used_mask.as_mut_ptr()); hwc.idx = idx;
    }
    (*ev).event[idx as usize] = event;
    hwc.state = PERF_HES_UPTODATE | PERF_HES_STOPPED;
    if flags & PERF_EF_START != 0 { xtensa_pmu_start(event, PERF_EF_RELOAD); }
    perf_event_update_userpage(event); 0
}

unsafe fn xtensa_pmu_del(event: *mut PerfEvent, _flags: c_int) {
    let ev = this_cpu_ptr(&mut XTENSA_PMU_EVENTS);
    xtensa_pmu_stop(event, PERF_EF_UPDATE);
    clear_bit((*event).hw.idx as usize, (*ev).used_mask.as_mut_ptr());
    perf_event_update_userpage(event);
}
unsafe fn xtensa_pmu_read(event: *mut PerfEvent) { xtensa_perf_event_update(event, &mut (*event).hw, (*event).hw.idx); }

unsafe fn callchain_trace(frame: *mut Stackframe, data: *mut c_void) -> c_int {
    perf_callchain_store(data as *mut PerfCallchainEntryCtx, (*frame).pc); 0
}
pub unsafe fn perf_callchain_kernel(entry: *mut PerfCallchainEntryCtx, regs: *mut PtRegs) {
    xtensa_backtrace_kernel(regs, (*entry).max_stack, callchain_trace, core::ptr::null_mut(), entry as *mut c_void);
}
pub unsafe fn perf_callchain_user(entry: *mut PerfCallchainEntryCtx, regs: *mut PtRegs) {
    xtensa_backtrace_user(regs, (*entry).max_stack, callchain_trace, entry as *mut c_void);
}

pub unsafe fn perf_event_print_debug() {
    let flags: c_ulong = 0; local_irq_save(&flags);
    pr_info!("CPU#{}: PMG: 0x{:08lx}\n", smp_processor_id(), get_er(XTENSA_PMU_PMG));
    for i in 0..XCHAL_NUM_PERF_COUNTERS { pr_info!("PM{}: 0x{:08lx}, PMCTRL{}: 0x{:08lx}, PMSTAT{}: 0x{:08lx}\n", i, get_er(xtensa_pmu_pm(i as u32)), i, get_er(xtensa_pmu_pmctrl(i as u32)), i, get_er(xtensa_pmu_pmstat(i as u32))); }
    local_irq_restore(flags);
}

pub unsafe fn xtensa_pmu_irq_handler(_irq: c_int, _dev_id: *mut c_void) -> Irqreturn {
    let mut rc = IRQ_NONE; let ev = this_cpu_ptr(&mut XTENSA_PMU_EVENTS);
    for i in for_each_set_bit((*ev).used_mask.as_ptr(), XCHAL_NUM_PERF_COUNTERS) {
        let v = get_er(xtensa_pmu_pmstat(i as u32)); if v & XTENSA_PMU_PMSTAT_OVFL == 0 { continue; }
        set_er(v, xtensa_pmu_pmstat(i as u32)); let event = (*ev).event[i]; let hwc = &mut (*event).hw;
        xtensa_perf_event_update(event, hwc, i as c_int); let last_period = hwc.last_period;
        if xtensa_perf_event_set_period(event, hwc, i as c_int) { let mut data = PerfSampleData::default(); perf_sample_data_init(&mut data, 0, last_period); perf_event_overflow(event, &mut data, get_irq_regs()); }
        rc = IRQ_HANDLED;
    } rc
}

unsafe fn xtensa_pmu_setup(_cpu: c_uint) -> c_int { set_er(0, XTENSA_PMU_PMG); for i in 0..XCHAL_NUM_PERF_COUNTERS { set_er(0, xtensa_pmu_pmctrl(i as u32)); set_er(get_er(xtensa_pmu_pmstat(i as u32)), xtensa_pmu_pmstat(i as u32)); } 0 }
static mut xtensa_pmu: Pmu = Pmu { pmu_enable: Some(xtensa_pmu_enable), pmu_disable: Some(xtensa_pmu_disable), event_init: Some(xtensa_pmu_event_init), add: Some(xtensa_pmu_add), del: Some(xtensa_pmu_del), start: Some(xtensa_pmu_start), stop: Some(xtensa_pmu_stop), read: Some(xtensa_pmu_read) };
unsafe fn xtensa_pmu_init() -> c_int {
    let irq = irq_create_mapping(core::ptr::null_mut(), XCHAL_PROFILING_INTERRUPT);
    let mut ret = cpuhp_setup_state(CPUHP_AP_PERF_XTENSA_STARTING, "perf/xtensa:starting", xtensa_pmu_setup, None);
    if ret != 0 { pr_err!("xtensa_pmu: failed to register CPU-hotplug.\n"); return ret; }
    ret = request_irq(irq, xtensa_pmu_irq_handler, IRQF_PERCPU, "pmu", core::ptr::null_mut());
    if ret < 0 { return ret; }
    ret = perf_pmu_register(&mut xtensa_pmu, "cpu", PERF_TYPE_RAW); if ret != 0 { free_irq(irq, core::ptr::null_mut()); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
