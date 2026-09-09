// SPDX-License-Identifier: GPL-2.0-only
/*
 * L220/L310 cache controller support
 *
 * Copyright (C) 2016 ARM Limited
 */

// Kernel dependencies supplied by other translation units.

const PMU_NR_COUNTERS: usize = 2;

static mut l2x0_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut l2x0_pmu: *mut pmu = core::ptr::null_mut();
static mut pmu_cpu: cpumask_t = unsafe { core::mem::zeroed() };
static mut l2x0_name: *const core::ffi::c_char = core::ptr::null();
static mut l2x0_pmu_poll_period: ktime_t = 0;
static mut l2x0_pmu_hrtimer: hrtimer = unsafe { core::mem::zeroed() };

/*
 * The L220/PL310 PMU has two equivalent counters, Counter1 and Counter0.
 * Registers controlling these are laid out in pairs, in descending order, i.e.
 * the register for Counter1 comes first, followed by the register for
 * Counter0.
 * We ensure that idx 0 -> Counter0, and idx1 -> Counter1.
 */
static mut events: [*mut perf_event; PMU_NR_COUNTERS] = [core::ptr::null_mut(); PMU_NR_COUNTERS];

/* Find an unused counter */
unsafe fn l2x0_pmu_find_idx() -> i32 {
    for i in 0..PMU_NR_COUNTERS {
        if events[i].is_null() { return i as i32; }
    }
    -1
}

/* How many counters are allocated? */
unsafe fn l2x0_pmu_num_active_counters() -> i32 {
    let mut cnt = 0;
    for i in 0..PMU_NR_COUNTERS { if !events[i].is_null() { cnt += 1; } }
    cnt
}

unsafe fn l2x0_pmu_counter_config_write(idx: i32, val: u32) {
    writel_relaxed(val, l2x0_base.add((L2X0_EVENT_CNT0_CFG as isize - 4 * idx as isize) as usize));
}
unsafe fn l2x0_pmu_counter_read(idx: i32) -> u32 {
    readl_relaxed(l2x0_base.add((L2X0_EVENT_CNT0_VAL as isize - 4 * idx as isize) as usize))
}
unsafe fn l2x0_pmu_counter_write(idx: i32, val: u32) {
    writel_relaxed(val, l2x0_base.add((L2X0_EVENT_CNT0_VAL as isize - 4 * idx as isize) as usize));
}
unsafe fn __l2x0_pmu_enable() {
    let mut val = readl_relaxed(l2x0_base.add(L2X0_EVENT_CNT_CTRL as usize));
    val |= L2X0_EVENT_CNT_CTRL_ENABLE;
    writel_relaxed(val, l2x0_base.add(L2X0_EVENT_CNT_CTRL as usize));
}
unsafe fn __l2x0_pmu_disable() {
    let mut val = readl_relaxed(l2x0_base.add(L2X0_EVENT_CNT_CTRL as usize));
    val &= !L2X0_EVENT_CNT_CTRL_ENABLE;
    writel_relaxed(val, l2x0_base.add(L2X0_EVENT_CNT_CTRL as usize));
}
unsafe fn l2x0_pmu_enable(_pmu: *mut pmu) { if l2x0_pmu_num_active_counters() != 0 { __l2x0_pmu_enable(); } }
unsafe fn l2x0_pmu_disable(_pmu: *mut pmu) { if l2x0_pmu_num_active_counters() != 0 { __l2x0_pmu_disable(); } }

unsafe fn warn_if_saturated(count: u32) {
    if count != 0xffff_ffff { return; }
    pr_warn_ratelimited!("L2X0 counter saturated. Poll period too long\n");
}

unsafe fn l2x0_pmu_event_read(event: *mut perf_event) {
    let hw = &mut (*event).hw;
    let (mut prev_count, mut new_count);
    loop {
        prev_count = local64_read(&hw.prev_count);
        new_count = l2x0_pmu_counter_read(hw.idx);
        if local64_xchg(&mut hw.prev_count, new_count as i64) == prev_count { break; }
    }
    let mask: u64 = 0xffff_ffff;
    local64_add(((new_count as u64).wrapping_sub(prev_count as u64) & mask) as i64, &mut (*event).count);
    warn_if_saturated(new_count);
}

unsafe fn l2x0_pmu_event_configure(event: *mut perf_event) {
    let hw = &mut (*event).hw;
    local64_set(&mut hw.prev_count, 0);
    l2x0_pmu_counter_write(hw.idx, 0);
}

unsafe fn l2x0_pmu_poll(hrtimer: *mut hrtimer) -> hrtimer_restart {
    let mut flags: ulong = 0;
    local_irq_save(&mut flags);
    __l2x0_pmu_disable();
    for i in 0..PMU_NR_COUNTERS {
        let event = events[i];
        if event.is_null() { continue; }
        l2x0_pmu_event_read(event);
        l2x0_pmu_event_configure(event);
    }
    __l2x0_pmu_enable();
    local_irq_restore(flags);
    hrtimer_forward_now(hrtimer, l2x0_pmu_poll_period);
    HRTIMER_RESTART
}

unsafe fn __l2x0_pmu_event_enable(idx: i32, event: u64) {
    let mut val = (event as u32) << L2X0_EVENT_CNT_CFG_SRC_SHIFT;
    val |= L2X0_EVENT_CNT_CFG_INT_DISABLED;
    l2x0_pmu_counter_config_write(idx, val);
}
unsafe fn l2x0_pmu_event_start(event: *mut perf_event, flags: i32) {
    let hw = &mut (*event).hw;
    if WARN_ON_ONCE!(((*event).hw.state & PERF_HES_STOPPED) == 0) { return; }
    if flags & PERF_EF_RELOAD != 0 { WARN_ON_ONCE!((hw.state & PERF_HES_UPTODATE) == 0); l2x0_pmu_event_configure(event); }
    hw.state = 0;
    __l2x0_pmu_event_enable(hw.idx, hw.config_base);
}
unsafe fn __l2x0_pmu_event_disable(idx: i32) {
    let mut val = L2X0_EVENT_CNT_CFG_SRC_DISABLED << L2X0_EVENT_CNT_CFG_SRC_SHIFT;
    val |= L2X0_EVENT_CNT_CFG_INT_DISABLED;
    l2x0_pmu_counter_config_write(idx, val);
}
unsafe fn l2x0_pmu_event_stop(event: *mut perf_event, flags: i32) {
    let hw = &mut (*event).hw;
    if WARN_ON_ONCE!(((*event).hw.state & PERF_HES_STOPPED) != 0) { return; }
    __l2x0_pmu_event_disable(hw.idx);
    hw.state |= PERF_HES_STOPPED;
    if flags & PERF_EF_UPDATE != 0 { l2x0_pmu_event_read(event); hw.state |= PERF_HES_UPTODATE; }
}

unsafe fn l2x0_pmu_event_add(event: *mut perf_event, flags: i32) -> i32 {
    let hw = &mut (*event).hw;
    let idx = l2x0_pmu_find_idx();
    if idx == -1 { return -EAGAIN; }
    if l2x0_pmu_num_active_counters() == 0 { hrtimer_start(&mut l2x0_pmu_hrtimer, l2x0_pmu_poll_period, HRTIMER_MODE_REL_PINNED); }
    events[idx as usize] = event;
    hw.idx = idx;
    l2x0_pmu_event_configure(event);
    hw.state = PERF_HES_STOPPED | PERF_HES_UPTODATE;
    if flags & PERF_EF_START != 0 { l2x0_pmu_event_start(event, 0); }
    0
}
unsafe fn l2x0_pmu_event_del(event: *mut perf_event, _flags: i32) {
    let hw = &mut (*event).hw;
    l2x0_pmu_event_stop(event, PERF_EF_UPDATE);
    events[hw.idx as usize] = core::ptr::null_mut(); hw.idx = -1;
    if l2x0_pmu_num_active_counters() == 0 { hrtimer_cancel(&mut l2x0_pmu_hrtimer); }
}

unsafe fn l2x0_pmu_group_is_valid(event: *mut perf_event) -> bool {
    let pmu = (*event).pmu; let leader = (*event).group_leader; let mut num_hw = 0;
    if (*leader).pmu == pmu { num_hw += 1; } else if !is_software_event(leader) { return false; }
    let mut sibling = core::ptr::null_mut();
    for_each_sibling_event!(sibling, leader) {
        if (*sibling).pmu == pmu { num_hw += 1; } else if !is_software_event(sibling) { return false; }
    }
    num_hw <= PMU_NR_COUNTERS as i32
}

unsafe fn l2x0_pmu_event_init(event: *mut perf_event) -> i32 {
    let hw = &mut (*event).hw;
    if (*event).attr.type_ != (*l2x0_pmu).type_ { return -ENOENT; }
    if is_sampling_event(event) || ((*event).attach_state & PERF_ATTACH_TASK != 0) || (*event).cpu < 0 { return -EINVAL; }
    if (*event).attr.config & !L2X0_EVENT_CNT_CFG_SRC_MASK as u64 != 0 { return -EINVAL; }
    hw.config_base = (*event).attr.config;
    if !l2x0_pmu_group_is_valid(event) { return -EINVAL; }
    (*event).cpu = cpumask_first(&pmu_cpu);
    0
}

#[repr(C)]
struct l2x0_event_attribute { attr: device_attribute, config: u32, pl310_only: bool }

unsafe fn l2x0_pmu_event_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut i8) -> ssize_t { 0 }
unsafe fn l2x0_pmu_event_attr_is_visible(_kobj: *mut kobject, _attr: *mut attribute, _unused: i32) -> umode_t { 0 }
unsafe fn l2x0_pmu_cpumask_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut i8) -> ssize_t { 0 }

// Attribute declarations and sysfs groups mirror the C source; macro-generated
// kernel attribute objects are represented by their externally supplied types.
static mut l2x0_pmu_event_attrs_group: attribute_group = unsafe { core::mem::zeroed() };
static mut l2x0_pmu_cpumask_attr_group: attribute_group = unsafe { core::mem::zeroed() };
static mut l2x0_pmu_attr_groups: [*const attribute_group; 3] = [core::ptr::null(); 3];

unsafe fn l2x0_pmu_reset() { __l2x0_pmu_disable(); for i in 0..PMU_NR_COUNTERS { __l2x0_pmu_event_disable(i as i32); } }
unsafe fn l2x0_pmu_offline_cpu(cpu: u32) -> i32 {
    if !cpumask_test_and_clear_cpu(cpu, &mut pmu_cpu) { return 0; }
    let target = cpumask_any_but(cpu_online_mask, cpu);
    if target >= nr_cpu_ids { return 0; }
    perf_pmu_migrate_context(l2x0_pmu, cpu, target); cpumask_set_cpu(target, &mut pmu_cpu); 0
}
pub unsafe fn l2x0_pmu_suspend() {
    if l2x0_pmu.is_null() { return; }
    l2x0_pmu_disable(l2x0_pmu);
    for i in 0..PMU_NR_COUNTERS { if !events[i].is_null() { l2x0_pmu_event_stop(events[i], PERF_EF_UPDATE); } }
}
pub unsafe fn l2x0_pmu_resume() {
    if l2x0_pmu.is_null() { return; }
    l2x0_pmu_reset();
    for i in 0..PMU_NR_COUNTERS { if !events[i].is_null() { l2x0_pmu_event_start(events[i], PERF_EF_RELOAD); } }
    l2x0_pmu_enable(l2x0_pmu);
}

pub unsafe fn l2x0_pmu_register(base: *mut core::ffi::c_void, part: u32) {
    match part & L2X0_CACHE_ID_PART_MASK {
        L2X0_CACHE_ID_PART_L220 => l2x0_name = c"l2c_220".as_ptr(),
        L2X0_CACHE_ID_PART_L310 => l2x0_name = c"l2c_310".as_ptr(),
        _ => return,
    }
    l2x0_base = base;
}

unsafe fn l2x0_pmu_init() -> i32 {
    if l2x0_base.is_null() { return 0; }
    l2x0_pmu = kzalloc_obj::<pmu>();
    if l2x0_pmu.is_null() { pr_warn!("Unable to allocate L2x0 PMU\n"); return -ENOMEM; }
    (*l2x0_pmu) = pmu { task_ctx_nr: perf_invalid_context, pmu_enable: Some(l2x0_pmu_enable), pmu_disable: Some(l2x0_pmu_disable), read: Some(l2x0_pmu_event_read), start: Some(l2x0_pmu_event_start), stop: Some(l2x0_pmu_event_stop), add: Some(l2x0_pmu_event_add), del: Some(l2x0_pmu_event_del), event_init: Some(l2x0_pmu_event_init), attr_groups: l2x0_pmu_attr_groups.as_ptr(), capabilities: PERF_PMU_CAP_NO_EXCLUDE };
    l2x0_pmu_reset();
    l2x0_pmu_poll_period = ms_to_ktime(1000);
    hrtimer_setup(&mut l2x0_pmu_hrtimer, Some(l2x0_pmu_poll), CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    cpumask_set_cpu(0, &mut pmu_cpu);
    let mut ret = cpuhp_setup_state_nocalls(CPUHP_AP_PERF_ARM_L2X0_ONLINE, c"perf/arm/l2x0:online".as_ptr(), None, Some(l2x0_pmu_offline_cpu));
    if ret != 0 { kfree(l2x0_pmu); l2x0_pmu = core::ptr::null_mut(); return ret; }
    ret = perf_pmu_register(l2x0_pmu, l2x0_name, -1);
    if ret != 0 { cpuhp_remove_state_nocalls(CPUHP_AP_PERF_ARM_L2X0_ONLINE); kfree(l2x0_pmu); l2x0_pmu = core::ptr::null_mut(); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
