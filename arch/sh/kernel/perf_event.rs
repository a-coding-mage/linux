// SPDX-License-Identifier: GPL-2.0
/*
 * Performance event support framework for SuperH hardware counters.
 *
 *  Copyright (C) 2009  Paul Mundt
 *
 * Heavily based on the x86 and PowerPC implementations.
 */

// Linux kernel dependencies and build-time configuration are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct CpuHwEvents {
    pub events: [*mut PerfEvent; MAX_HWEVENTS],
    pub used_mask: [c_ulong; BITS_TO_LONGS(MAX_HWEVENTS)],
    pub active_mask: [c_ulong; BITS_TO_LONGS(MAX_HWEVENTS)],
}

extern "C" {
    static mut cpu_hw_events: CpuHwEvents;
    static mut sh_pmu: *mut ShPmu;
    static mut num_events: AtomicT;
    static mut pmc_reserve_mutex: Mutex;

    fn atomic_add_unless(v: *mut AtomicT, a: c_int, u: c_int) -> bool;
    fn mutex_lock(m: *mut Mutex);
    fn mutex_unlock(m: *mut Mutex);
    fn atomic_dec_return(v: *mut AtomicT) -> c_int;
    fn atomic_read(v: *mut AtomicT) -> c_int;
    fn atomic_inc(v: *mut AtomicT);
    fn atomic_inc_not_zero(v: *mut AtomicT) -> bool;
    fn release_pmc_hardware();
    fn reserve_pmc_hardware() -> c_int;
    fn local64_read(v: *const Local64) -> u64;
    fn local64_cmpxchg(v: *mut Local64, old: u64, new: u64) -> u64;
    fn local64_add(v: i64, p: *mut Local64);
    fn this_cpu_ptr<T>(p: *mut T) -> *mut T;
    fn perf_event_update_userpage(e: *mut PerfEvent);
    fn perf_pmu_disable(p: *mut Pmu);
    fn perf_pmu_enable(p: *mut Pmu);
    fn find_first_zero_bit(p: *const c_ulong, n: c_uint) -> c_int;
    fn test_and_set_bit(n: c_int, p: *mut c_ulong) -> bool;
    fn clear_bit(n: c_int, p: *mut c_ulong);
    fn set_bit(n: c_int, p: *mut c_ulong);
    fn has_branch_stack(e: *mut PerfEvent) -> bool;
    fn perf_pmu_register(p: *mut Pmu, name: *const c_char, ty: c_int) -> c_int;
    fn cpuhp_setup_state(state: c_int, name: *const c_char,
                         start: Option<unsafe extern "C" fn(c_uint) -> c_int>,
                         stop: Option<unsafe extern "C" fn(c_uint) -> c_int>) -> c_int;
    fn memset(p: *mut c_void, v: c_int, n: usize) -> *mut c_void;
}

#[inline]
pub unsafe fn sh_pmu_initialized() -> bool { !sh_pmu.is_null() }

pub unsafe extern "C" fn hw_perf_event_destroy(_event: *mut PerfEvent) {
    if !atomic_add_unless(&mut num_events, -1, 1) {
        mutex_lock(&mut pmc_reserve_mutex);
        if atomic_dec_return(&mut num_events) == 0 { release_pmc_hardware(); }
        mutex_unlock(&mut pmc_reserve_mutex);
    }
}

unsafe fn hw_perf_cache_event(config: c_int, evp: *mut c_int) -> c_int {
    if (*sh_pmu).cache_events.is_null() { return -EINVAL; }
    let ty = (config & 0xff) as usize;
    let op = ((config >> 8) & 0xff) as usize;
    let result = ((config >> 16) & 0xff) as usize;
    if ty >= PERF_COUNT_HW_CACHE_MAX || op >= PERF_COUNT_HW_CACHE_OP_MAX ||
       result >= PERF_COUNT_HW_CACHE_RESULT_MAX { return -EINVAL; }
    let ev = (*(*sh_pmu).cache_events)[ty][op][result];
    if ev == 0 { return -EOPNOTSUPP; }
    if ev == -1 { return -EINVAL; }
    *evp = ev; 0
}

unsafe fn __hw_perf_event_init(event: *mut PerfEvent) -> c_int {
    let attr = &mut (*event).attr;
    let hwc = &mut (*event).hw;
    let mut config: c_int = -1;
    let mut err = 0;
    if !sh_pmu_initialized() { return -ENODEV; }
    if !atomic_inc_not_zero(&mut num_events) {
        mutex_lock(&mut pmc_reserve_mutex);
        if atomic_read(&mut num_events) == 0 && reserve_pmc_hardware() != 0 { err = -EBUSY; }
        else { atomic_inc(&mut num_events); }
        mutex_unlock(&mut pmc_reserve_mutex);
    }
    if err != 0 { return err; }
    (*event).destroy = Some(hw_perf_event_destroy);
    match attr.type_ {
        PERF_TYPE_RAW => config = attr.config as c_int & (*sh_pmu).raw_event_mask,
        PERF_TYPE_HW_CACHE => { err = hw_perf_cache_event(attr.config as c_int, &mut config); if err != 0 { return err; } },
        PERF_TYPE_HARDWARE => {
            if attr.config >= (*sh_pmu).max_events as u64 { return -EINVAL; }
            config = ((*sh_pmu).event_map)(attr.config as c_int);
        },
        _ => {}
    }
    if config == -1 { return -EINVAL; }
    hwc.config |= config;
    0
}

unsafe fn sh_perf_event_update(event: *mut PerfEvent, hwc: *mut HwPerfEvent, idx: c_int) {
    let shift = 0;
    let (prev, newv);
    loop {
        prev = local64_read(&(*hwc).prev_count);
        newv = ((*sh_pmu).read)(idx);
        if local64_cmpxchg(&mut (*hwc).prev_count, prev, newv) == prev { break; }
    }
    let delta = (((newv << shift) as i64) - ((prev << shift) as i64)) >> shift;
    local64_add(delta, &mut (*event).count);
}

unsafe extern "C" fn sh_pmu_stop(event: *mut PerfEvent, flags: c_int) {
    let cpuc = this_cpu_ptr(&mut cpu_hw_events); let hwc = &mut (*event).hw; let idx = hwc.idx;
    if event.hw.state & PERF_HES_STOPPED == 0 { ((*sh_pmu).disable)(hwc, idx); (*cpuc).events[idx as usize] = core::ptr::null_mut(); (*event).hw.state |= PERF_HES_STOPPED; }
    if flags & PERF_EF_UPDATE != 0 && event.hw.state & PERF_HES_UPTODATE == 0 { sh_perf_event_update(event, hwc, idx); (*event).hw.state |= PERF_HES_UPTODATE; }
}

unsafe extern "C" fn sh_pmu_start(event: *mut PerfEvent, flags: c_int) {
    let cpuc = this_cpu_ptr(&mut cpu_hw_events); let hwc = &mut (*event).hw; let idx = hwc.idx;
    if idx == -1 { return; }
    (*cpuc).events[idx as usize] = event; (*event).hw.state = 0; ((*sh_pmu).enable)(hwc, idx);
    let _ = flags;
}

unsafe extern "C" fn sh_pmu_del(event: *mut PerfEvent, _flags: c_int) {
    let cpuc = this_cpu_ptr(&mut cpu_hw_events); sh_pmu_stop(event, PERF_EF_UPDATE); clear_bit((*event).hw.idx, (*cpuc).used_mask.as_mut_ptr()); perf_event_update_userpage(event);
}

unsafe extern "C" fn sh_pmu_add(event: *mut PerfEvent, flags: c_int) -> c_int {
    let cpuc = this_cpu_ptr(&mut cpu_hw_events); let hwc = &mut (*event).hw; let mut idx = hwc.idx; let mut ret = -EAGAIN;
    perf_pmu_disable((*event).pmu);
    if test_and_set_bit(idx, (*cpuc).used_mask.as_mut_ptr()) { idx = find_first_zero_bit((*cpuc).used_mask.as_ptr(), (*sh_pmu).num_events as c_uint); if idx == (*sh_pmu).num_events { perf_pmu_enable((*event).pmu); return ret; } set_bit(idx, (*cpuc).used_mask.as_mut_ptr()); hwc.idx = idx; }
    ((*sh_pmu).disable)(hwc, idx); (*event).hw.state = PERF_HES_UPTODATE | PERF_HES_STOPPED; if flags & PERF_EF_START != 0 { sh_pmu_start(event, PERF_EF_RELOAD); } perf_event_update_userpage(event); ret = 0; perf_pmu_enable((*event).pmu); ret
}

unsafe extern "C" fn sh_pmu_read(event: *mut PerfEvent) { sh_perf_event_update(event, &mut (*event).hw, (*event).hw.idx); }

unsafe extern "C" fn sh_pmu_event_init(event: *mut PerfEvent) -> c_int {
    if has_branch_stack(event) { return -EOPNOTSUPP; }
    let err = match (*event).attr.type_ { PERF_TYPE_RAW | PERF_TYPE_HW_CACHE | PERF_TYPE_HARDWARE => __hw_perf_event_init(event), _ => return -ENOENT };
    if err != 0 { if let Some(destroy) = (*event).destroy { destroy(event); } } err
}

unsafe extern "C" fn sh_pmu_enable(_pmu: *mut Pmu) { if sh_pmu_initialized() { ((*sh_pmu).enable_all)(); } }
unsafe extern "C" fn sh_pmu_disable(_pmu: *mut Pmu) { if sh_pmu_initialized() { ((*sh_pmu).disable_all)(); } }

#[repr(C)] pub struct Pmu { pub pmu_enable: Option<unsafe extern "C" fn(*mut Pmu)>, pub pmu_disable: Option<unsafe extern "C" fn(*mut Pmu)>, pub event_init: Option<unsafe extern "C" fn(*mut PerfEvent)->c_int>, pub add: Option<unsafe extern "C" fn(*mut PerfEvent,c_int)->c_int>, pub del: Option<unsafe extern "C" fn(*mut PerfEvent,c_int)>, pub start: Option<unsafe extern "C" fn(*mut PerfEvent,c_int)>, pub stop: Option<unsafe extern "C" fn(*mut PerfEvent,c_int)>, pub read: Option<unsafe extern "C" fn(*mut PerfEvent)> }

static mut pmu: Pmu = Pmu { pmu_enable: Some(sh_pmu_enable), pmu_disable: Some(sh_pmu_disable), event_init: Some(sh_pmu_event_init), add: Some(sh_pmu_add), del: Some(sh_pmu_del), start: Some(sh_pmu_start), stop: Some(sh_pmu_stop), read: Some(sh_pmu_read) };

unsafe extern "C" fn sh_pmu_prepare_cpu(cpu: c_uint) -> c_int { let cpuhw = &mut per_cpu(cpu_hw_events, cpu); memset(cpuhw as *mut _ as *mut c_void, 0, core::mem::size_of::<CpuHwEvents>()); 0 }

pub unsafe extern "C" fn register_sh_pmu(_pmu: *mut ShPmu) -> c_int {
    if !sh_pmu.is_null() { return -EBUSY; } sh_pmu = _pmu;
    pmu.capabilities |= PERF_PMU_CAP_NO_INTERRUPT;
    let _ = perf_pmu_register(&mut pmu, b"cpu\0".as_ptr() as *const c_char, PERF_TYPE_RAW);
    let _ = cpuhp_setup_state(CPUHP_PERF_SUPERH, b"PERF_SUPERH\0".as_ptr() as *const c_char, Some(sh_pmu_prepare_cpu), None); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
