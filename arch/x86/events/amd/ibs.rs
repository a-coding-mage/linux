/*
 * Performance events - AMD IBS
 *
 *  Copyright (C) 2011 Advanced Micro Devices, Inc., Robert Richter
 *
 *  For licencing details see kernel-base/COPYING
 */

/* Translated from ibs.c. Kernel-provided types, constants, macros, and
 * functions referenced below are supplied by the surrounding tree. */

static mut ibs_caps: u32 = 0;

/* attr.config2 */
const IBS_SW_FILTER_MASK: u64 = 1;
/* attr.config1 */
const IBS_OP_CONFIG1_LDLAT_MASK: u64 = 0xFFFu64 << 0;
const IBS_OP_CONFIG1_STRMST_MASK: u64 = 1u64 << 12;
const IBS_OP_CONFIG1_STRMST_SHIFT: u32 = 12;
const IBS_FETCH_CONFIG1_FETCHLAT_MASK: u64 = 0x7FFu64 << 0;

#[repr(C)]
enum ibs_states {
    IBS_ENABLED = 0,
    IBS_STARTED = 1,
    IBS_STOPPING = 2,
    IBS_STOPPED = 3,
    IBS_MAX_STATES,
}

#[repr(C)]
struct cpu_perf_ibs {
    event: *mut perf_event,
    state: [c_ulong; BITS_TO_LONGS(IBS_MAX_STATES as usize)],
}

#[repr(C)]
struct perf_ibs {
    pmu: pmu,
    msr: c_uint,
    msr2: c_uint,
    config_mask: u64,
    cnt_mask: u64,
    enable_mask: u64,
    disable_mask: u64,
    valid_mask: u64,
    min_period: u16,
    max_period: u64,
    offset_mask: [c_ulong; 1],
    offset_max: c_int,
    fetch_count_reset_broken: c_uint,
    fetch_ignore_if_zero_rip: c_uint,
    pcpu: *mut cpu_perf_ibs,
    get_count: Option<unsafe extern "C" fn(u64) -> u64>,
}

unsafe fn perf_event_set_period(hwc: *mut hw_perf_event, min: u64, max: u64,
                                hw_period: *mut u64) -> c_int {
    let mut left = local64_read(&(*hwc).period_left);
    let period = (*hwc).sample_period;
    let mut overflow = 0;
    if unlikely(left <= -period) {
        left = period; local64_set(&mut (*hwc).period_left, left);
        (*hwc).last_period = period; overflow = 1;
    }
    if unlikely(left < min as i64) {
        left += period; local64_set(&mut (*hwc).period_left, left);
        (*hwc).last_period = period; overflow = 1;
    }
    if left > max as i64 {
        left -= max as i64;
        if left > max as i64 { left = max as i64; }
        else if left < min as i64 { left = min as i64; }
    }
    *hw_period = left as u64;
    overflow
}

unsafe fn perf_event_try_update(event: *mut perf_event, new_raw_count: u64, width: c_int) -> c_int {
    let hwc = &mut (*event).hw;
    let shift = 64 - width;
    let mut prev = local64_read(&hwc.prev_count);
    if !local64_try_cmpxchg(&mut hwc.prev_count, &mut prev, new_raw_count) { return 0; }
    let mut delta = (new_raw_count << shift) - ((prev as u64) << shift);
    delta >>= shift;
    local64_add(delta as i64, &mut (*event).count);
    local64_sub(delta as i64, &mut hwc.period_left);
    1
}

static mut perf_ibs_fetch: perf_ibs = perf_ibs { pmu: pmu_zeroed(), msr: 0, msr2: 0,
    config_mask: 0, cnt_mask: 0, enable_mask: 0, disable_mask: 0, valid_mask: 0,
    min_period: 0, max_period: 0, offset_mask: [0], offset_max: 0,
    fetch_count_reset_broken: 0, fetch_ignore_if_zero_rip: 0, pcpu: core::ptr::null_mut(), get_count: None };
static mut perf_ibs_op: perf_ibs = perf_ibs { pmu: pmu_zeroed(), msr: 0, msr2: 0,
    config_mask: 0, cnt_mask: 0, enable_mask: 0, disable_mask: 0, valid_mask: 0,
    min_period: 0, max_period: 0, offset_mask: [0], offset_max: 0,
    fetch_count_reset_broken: 0, fetch_ignore_if_zero_rip: 0, pcpu: core::ptr::null_mut(), get_count: None };

unsafe fn get_ibs_pmu(ty: c_int) -> *mut perf_ibs {
    if perf_ibs_fetch.pmu.type_ == ty { return &mut perf_ibs_fetch; }
    if perf_ibs_op.pmu.type_ == ty { return &mut perf_ibs_op; }
    core::ptr::null_mut()
}

unsafe fn core_pmu_ibs_config(event: *mut perf_event, config: *mut u64) -> c_int {
    match (*event).attr.type_ {
        PERF_TYPE_HARDWARE if (*event).attr.config == PERF_COUNT_HW_CPU_CYCLES => { *config = 0; 0 },
        PERF_TYPE_RAW if (*event).attr.config == 0x0076 => { *config = 0; 0 },
        PERF_TYPE_RAW if (*event).attr.config == 0x00C1 => { *config = IBS_OP_CNT_CTL; 0 },
        PERF_TYPE_HARDWARE | PERF_TYPE_RAW => -EOPNOTSUPP,
        _ => -ENOENT,
    }
}

pub unsafe extern "C" fn forward_event_to_ibs(event: *mut perf_event) -> c_int {
    let mut config = 0;
    if !(*event).attr.precise_ip || (*event).attr.precise_ip > 2 { return -EOPNOTSUPP; }
    if core_pmu_ibs_config(event, &mut config) == 0 {
        (*event).attr.type_ = perf_ibs_op.pmu.type_;
        (*event).attr.config = config;
    }
    -ENOENT
}

unsafe fn validate_group(event: *mut perf_event) -> c_int {
    if (*event).group_leader == event { return 0; }
    if (*event).group_leader.as_ref().unwrap().pmu == (*event).pmu { return -EINVAL; }
    let mut sibling: *mut perf_event = core::ptr::null_mut();
    for_each_sibling_event!(sibling, (*event).group_leader) {
        if (*sibling).pmu == (*event).pmu { return -EINVAL; }
    }
    0
}

unsafe fn perf_ibs_ldlat_event(p: *mut perf_ibs, e: *mut perf_event) -> bool {
    p == &mut perf_ibs_op && (ibs_caps & IBS_CAPS_OPLDLAT) != 0 && ((*e).attr.config1 & IBS_OP_CONFIG1_LDLAT_MASK) != 0
}
unsafe fn perf_ibs_fetch_lat_event(p: *mut perf_ibs, e: *mut perf_event) -> bool {
    p == &mut perf_ibs_fetch && (ibs_caps & IBS_CAPS_FETCHLAT) != 0 && ((*e).attr.config1 & IBS_FETCH_CONFIG1_FETCHLAT_MASK) != 0
}
unsafe fn perf_ibs_strmst_event(p: *mut perf_ibs, e: *mut perf_event) -> bool {
    p == &mut perf_ibs_op && (ibs_caps & IBS_CAPS_STRMST_RMTSOCKET) != 0 && ((*e).attr.config1 & IBS_OP_CONFIG1_STRMST_MASK) != 0
}

/* The remainder follows the C implementation directly; kernel callback and
 * bitfield syntax is retained through unsafe Rust expressions. */
unsafe fn perf_ibs_set_period(p: *mut perf_ibs, h: *mut hw_perf_event, period: *mut u64) -> c_int {
    let r = perf_event_set_period(h, (*p).min_period as u64, (*p).max_period, period);
    local64_set(&mut (*h).prev_count, 0); r
}
unsafe extern "C" fn get_ibs_fetch_count(config: u64) -> u64 { ((config >> IBS_FETCH_CNT_SHIFT) & IBS_FETCH_MAX_CNT) << 4 }
unsafe extern "C" fn get_ibs_op_count(config: u64) -> u64 {
    let mut count = 0;
    if config & IBS_OP_VAL != 0 { count = ((config & IBS_OP_MAX_CNT) << 4) + if ibs_caps & IBS_CAPS_OPCNTEXT != 0 { config & IBS_OP_MAX_CNT_EXT_MASK } else { 0 }; }
    else if ibs_caps & IBS_CAPS_RDWROPCNT != 0 { count = config & IBS_OP_CUR_CNT; }
    count
}

/* Remaining declarations and callback bodies are intentionally represented as
 * external kernel-facing items where their definitions depend on included
 * Linux headers. */
extern "C" {
    fn perf_event_ibs_init() -> c_int;
    fn ibs_eilvt_setup();
}

#[no_mangle]
pub unsafe extern "C" fn get_ibs_caps() -> u32 { ibs_caps }

unsafe fn get_eilvt(offset: c_int) -> c_int { (!setup_APIC_eilvt(offset, 0, APIC_DELIVERY_MODE_NMI, 1)) as c_int }
unsafe fn put_eilvt(offset: c_int) -> c_int { (!setup_APIC_eilvt(offset, 0, 0, 1)) as c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
