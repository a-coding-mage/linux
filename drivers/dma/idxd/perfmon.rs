// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2020 Intel Corporation. All rights rsvd. */

// Kernel dependencies: idxd.h, perfmon.h, Linux I/O, scheduler, and perf APIs.

/* These attributes specify the bits in the config word used by perfmon. */
DEFINE_PERFMON_FORMAT_ATTR!(event_category, "config:0-3");
DEFINE_PERFMON_FORMAT_ATTR!(event, "config:4-31");

/* These attributes specify the bits in the config1 word used by perfmon. */
DEFINE_PERFMON_FORMAT_ATTR!(filter_wq, "config1:0-31");
DEFINE_PERFMON_FORMAT_ATTR!(filter_tc, "config1:32-39");
DEFINE_PERFMON_FORMAT_ATTR!(filter_pgsz, "config1:40-43");
DEFINE_PERFMON_FORMAT_ATTR!(filter_sz, "config1:44-51");
DEFINE_PERFMON_FORMAT_ATTR!(filter_eng, "config1:52-59");

const PERFMON_FILTERS_START: usize = 2;
const PERFMON_FILTERS_MAX: usize = 5;

static mut PERFMON_FORMAT_ATTRS: [*mut attribute; 8] = [
    &raw mut format_attr_idxd_event_category.attr,
    &raw mut format_attr_idxd_event.attr,
    &raw mut format_attr_idxd_filter_wq.attr,
    &raw mut format_attr_idxd_filter_tc.attr,
    &raw mut format_attr_idxd_filter_pgsz.attr,
    &raw mut format_attr_idxd_filter_sz.attr,
    &raw mut format_attr_idxd_filter_eng.attr,
    core::ptr::null_mut(),
];

static mut PERFMON_FORMAT_ATTR_GROUP: attribute_group = attribute_group {
    name: c"format".as_ptr(),
    attrs: PERFMON_FORMAT_ATTRS.as_ptr() as *mut *mut attribute,
};

static PERFMON_ATTR_GROUPS: [*const attribute_group; 2] = [
    &raw const PERFMON_FORMAT_ATTR_GROUP,
    core::ptr::null(),
];

unsafe fn is_idxd_event(idxd_pmu: *mut idxd_pmu, event: *mut perf_event) -> bool {
    (*idxd_pmu).pmu as *mut pmu == (*event).pmu
}

unsafe fn perfmon_collect_events(idxd_pmu: *mut idxd_pmu, leader: *mut perf_event, do_grp: bool) -> i32 {
    let max_count = (*idxd_pmu).n_counters;
    let mut n = (*idxd_pmu).n_events;
    if n >= max_count { return -EINVAL; }

    if is_idxd_event(idxd_pmu, leader) {
        (*idxd_pmu).event_list[n as usize] = leader;
        (*leader).hw.idx = n;
        n += 1;
    }
    if !do_grp { return n; }

    for_each_sibling_event!(event, leader) {
        if !is_idxd_event(idxd_pmu, event) || (*event).state <= PERF_EVENT_STATE_OFF { continue; }
        if n >= max_count { return -EINVAL; }
        (*idxd_pmu).event_list[n as usize] = event;
        (*event).hw.idx = n;
        n += 1;
    }
    n
}

unsafe fn perfmon_assign_hw_event(idxd_pmu: *mut idxd_pmu, event: *mut perf_event, idx: i32) {
    let idxd = (*idxd_pmu).idxd;
    let hwc = &mut (*event).hw;
    hwc.idx = idx;
    hwc.config_base = ioread64(CNTRCFG_REG!(idxd, idx));
    hwc.event_base = ioread64(CNTRCFG_REG!(idxd, idx));
}

unsafe fn perfmon_assign_event(idxd_pmu: *mut idxd_pmu, _event: *mut perf_event) -> i32 {
    for i in 0..IDXD_PMU_EVENT_MAX {
        if !test_and_set_bit(i, &mut (*idxd_pmu).used_mask) { return i as i32; }
    }
    -EINVAL
}

unsafe fn perfmon_validate_group(pmu: *mut idxd_pmu, event: *mut perf_event) -> i32 {
    let leader = (*event).group_leader;
    let fake_pmu = kzalloc_obj::<idxd_pmu>();
    if fake_pmu.is_null() { return -ENOMEM; }
    (*fake_pmu).pmu.name = (*pmu).pmu.name;
    (*fake_pmu).n_counters = (*pmu).n_counters;
    let mut n = perfmon_collect_events(fake_pmu, leader, true);
    if n < 0 { kfree(fake_pmu); return n; }
    (*fake_pmu).n_events = n;
    n = perfmon_collect_events(fake_pmu, event, false);
    if n < 0 { kfree(fake_pmu); return n; }
    (*fake_pmu).n_events = n;
    for i in 0..n {
        let ev = (*fake_pmu).event_list[i as usize];
        let idx = perfmon_assign_event(fake_pmu, ev);
        if idx < 0 { kfree(fake_pmu); return idx; }
    }
    kfree(fake_pmu);
    0
}

unsafe fn perfmon_pmu_event_init(event: *mut perf_event) -> i32 {
    let idxd = event_to_idxd(event);
    (*event).hw.idx = -1;
    if (*event).attr.type_ != (*(*event).pmu).type_ { return -ENOENT; }
    if (*event).attr.sample_period != 0 { return -EINVAL; }
    if (*event).cpu < 0 { return -EINVAL; }
    if (*event).pmu != &raw mut (*idxd).idxd_pmu.as_mut().unwrap().pmu { return -EINVAL; }
    (*event).hw.event_base = ioread64(PERFMON_TABLE_OFFSET!(idxd));
    (*event).hw.config = (*event).attr.config;
    if (*event).group_leader != event { return perfmon_validate_group((*idxd).idxd_pmu, event); }
    0
}

unsafe fn perfmon_pmu_read_counter(event: *mut perf_event) -> u64 {
    let idxd = event_to_idxd(event);
    ioread64(CNTRDATA_REG!(idxd, (*event).hw.idx))
}

unsafe fn perfmon_pmu_event_update(event: *mut perf_event) {
    let idxd = event_to_idxd(event);
    let shift = 64 - (*idxd).idxd_pmu.counter_width;
    let hwc = &mut (*event).hw;
    let mut prev = local64_read(&hwc.prev_count);
    let mut new_raw_count;
    loop {
        new_raw_count = perfmon_pmu_read_counter(event);
        if local64_try_cmpxchg(&mut hwc.prev_count, &mut prev, new_raw_count) { break; }
    }
    let n = new_raw_count << shift;
    let p = prev << shift;
    local64_add((n.wrapping_sub(p)) >> shift, &mut (*event).count);
}

pub unsafe fn perfmon_counter_overflow(idxd: *mut idxd_device) {
    let n_counters = core::cmp::min((*idxd).idxd_pmu.n_counters, OVERFLOW_SIZE);
    let mut ovfstatus = ioread32(OVFSTATUS_REG!(idxd));
    let mut max_loop = OVERFLOW_SIZE;
    while ovfstatus != 0 && max_loop != 0 {
        max_loop -= 1;
        for_each_set_bit!(i, &mut ovfstatus, n_counters) {
            let mut clear = 0;
            let event = (*idxd).idxd_pmu.event_list[i as usize];
            perfmon_pmu_event_update(event);
            set_bit(i, &mut clear);
            iowrite32(clear, OVFSTATUS_REG!(idxd));
        }
        ovfstatus = ioread32(OVFSTATUS_REG!(idxd));
    }
    WARN_ON_ONCE!(ovfstatus != 0);
}

unsafe fn perfmon_reset_config(idxd: *mut idxd_device) {
    iowrite32(CONFIG_RESET, PERFRST_REG!(idxd));
    iowrite32(0, OVFSTATUS_REG!(idxd));
    iowrite32(0, PERFFRZ_REG!(idxd));
}
unsafe fn perfmon_reset_counters(idxd: *mut idxd_device) { iowrite32(CNTR_RESET, PERFRST_REG!(idxd)); }
unsafe fn perfmon_reset(idxd: *mut idxd_device) { perfmon_reset_config(idxd); perfmon_reset_counters(idxd); }

unsafe fn perfmon_pmu_event_start(event: *mut perf_event, _mode: i32) {
    let idxd = event_to_idxd(event);
    let hwc = &mut (*event).hw;
    (*event).hw.idx = hwc.idx;
    let cntr = hwc.idx;
    let mut event_cfg: event_cfg = core::mem::zeroed();
    let mut flt_cfg: filter_cfg = core::mem::zeroed();
    event_cfg.val = (*event).attr.config;
    flt_cfg.val = (*event).attr.config1;
    let flt_wq = flt_cfg.wq; let flt_tc = flt_cfg.tc; let flt_pg_sz = flt_cfg.pg_sz;
    let flt_xfer_sz = flt_cfg.xfer_sz; let flt_eng = flt_cfg.eng;
    if flt_wq != 0 && test_bit(FLT_WQ, &(*idxd).idxd_pmu.supported_filters) { iowrite32(flt_wq, FLTCFG_REG!(idxd, cntr, FLT_WQ)); }
    if flt_tc != 0 && test_bit(FLT_TC, &(*idxd).idxd_pmu.supported_filters) { iowrite32(flt_tc, FLTCFG_REG!(idxd, cntr, FLT_TC)); }
    if flt_pg_sz != 0 && test_bit(FLT_PG_SZ, &(*idxd).idxd_pmu.supported_filters) { iowrite32(flt_pg_sz, FLTCFG_REG!(idxd, cntr, FLT_PG_SZ)); }
    if flt_xfer_sz != 0 && test_bit(FLT_XFER_SZ, &(*idxd).idxd_pmu.supported_filters) { iowrite32(flt_xfer_sz, FLTCFG_REG!(idxd, cntr, FLT_XFER_SZ)); }
    if flt_eng != 0 && test_bit(FLT_ENG, &(*idxd).idxd_pmu.supported_filters) { iowrite32(flt_eng, FLTCFG_REG!(idxd, cntr, FLT_ENG)); }
    let cntrdata = ioread64(CNTRDATA_REG!(idxd, cntr));
    local64_set(&mut (*event).hw.prev_count, cntrdata);
    let mut cntr_cfg = (event_cfg.event_cat << CNTRCFG_CATEGORY_SHIFT) | (event_cfg.event_enc << CNTRCFG_EVENT_SHIFT);
    cntr_cfg |= CNTRCFG_IRQ_OVERFLOW | CNTRCFG_ENABLE;
    iowrite64(cntr_cfg, CNTRCFG_REG!(idxd, cntr));
}

unsafe fn perfmon_pmu_event_stop(event: *mut perf_event, mode: i32) {
    let idxd = event_to_idxd(event); let cntr = (*event).hw.idx;
    let mut i = 0;
    while i < (*idxd).idxd_pmu.n_events { if (*idxd).idxd_pmu.event_list[i as usize] == event { i += 1; while i < (*idxd).idxd_pmu.n_events { (*idxd).idxd_pmu.event_list[(i - 1) as usize] = (*idxd).idxd_pmu.event_list[i as usize]; i += 1; } (*idxd).idxd_pmu.n_events -= 1; break; } i += 1; }
    let mut cfg = ioread64(CNTRCFG_REG!(idxd, cntr)); cfg &= !CNTRCFG_ENABLE; iowrite64(cfg, CNTRCFG_REG!(idxd, cntr));
    if mode == PERF_EF_UPDATE { perfmon_pmu_event_update(event); }
    (*event).hw.idx = -1; clear_bit(cntr, &mut (*idxd).idxd_pmu.used_mask);
}
unsafe fn perfmon_pmu_event_del(event: *mut perf_event, _mode: i32) { perfmon_pmu_event_stop(event, PERF_EF_UPDATE); }

unsafe fn perfmon_pmu_event_add(event: *mut perf_event, flags: i32) -> i32 {
    let idxd = event_to_idxd(event); let pmu = (*idxd).idxd_pmu; let hwc = &mut (*event).hw;
    let n = perfmon_collect_events(pmu, event, false); if n < 0 { return n; }
    hwc.state = PERF_HES_UPTODATE | PERF_HES_STOPPED; if flags & PERF_EF_START == 0 { hwc.state |= PERF_HES_ARCH; }
    let idx = perfmon_assign_event(pmu, event); if idx < 0 { return idx; }
    perfmon_assign_hw_event(pmu, event, idx); if flags & PERF_EF_START != 0 { perfmon_pmu_event_start(event, 0); }
    (*pmu).n_events = n; 0
}

unsafe fn enable_perfmon_pmu(idxd: *mut idxd_device) { iowrite32(COUNTER_UNFREEZE, PERFFRZ_REG!(idxd)); }
unsafe fn disable_perfmon_pmu(idxd: *mut idxd_device) { iowrite32(COUNTER_FREEZE, PERFFRZ_REG!(idxd)); }
unsafe fn perfmon_pmu_enable(pmu: *mut pmu) { enable_perfmon_pmu(pmu_to_idxd(pmu)); }
unsafe fn perfmon_pmu_disable(pmu: *mut pmu) { disable_perfmon_pmu(pmu_to_idxd(pmu)); }

unsafe fn skip_filter(i: usize) { for j in i..PERFMON_FILTERS_MAX { PERFMON_FORMAT_ATTRS[PERFMON_FILTERS_START + j] = PERFMON_FORMAT_ATTRS[PERFMON_FILTERS_START + j + 1]; } }

unsafe fn idxd_pmu_init(idxd_pmu: *mut idxd_pmu) {
    for i in 0..PERFMON_FILTERS_MAX { if !test_bit(i, &(*idxd_pmu).supported_filters) { skip_filter(i); } }
    (*idxd_pmu).pmu.name = (*idxd_pmu).name;
    (*idxd_pmu).pmu.attr_groups = PERFMON_ATTR_GROUPS.as_ptr();
    (*idxd_pmu).pmu.task_ctx_nr = perf_invalid_context;
    (*idxd_pmu).pmu.event_init = perfmon_pmu_event_init; (*idxd_pmu).pmu.pmu_enable = perfmon_pmu_enable; (*idxd_pmu).pmu.pmu_disable = perfmon_pmu_disable;
    (*idxd_pmu).pmu.add = perfmon_pmu_event_add; (*idxd_pmu).pmu.del = perfmon_pmu_event_del; (*idxd_pmu).pmu.start = perfmon_pmu_event_start; (*idxd_pmu).pmu.stop = perfmon_pmu_event_stop; (*idxd_pmu).pmu.read = perfmon_pmu_event_update;
    (*idxd_pmu).pmu.capabilities = PERF_PMU_CAP_NO_EXCLUDE; (*idxd_pmu).pmu.scope = PERF_PMU_SCOPE_SYS_WIDE; (*idxd_pmu).pmu.module = THIS_MODULE;
}

pub unsafe fn perfmon_pmu_remove(idxd: *mut idxd_device) {
    if (*idxd).idxd_pmu.is_null() { return; }
    perf_pmu_unregister(&mut (*(*idxd).idxd_pmu).pmu); kfree((*idxd).idxd_pmu); (*idxd).idxd_pmu = core::ptr::null_mut();
}

pub unsafe fn perfmon_pmu_init(idxd: *mut idxd_device) -> i32 {
    if (*idxd).perfmon_offset == 0 { return -ENODEV; }
    let pmu = kzalloc_obj::<idxd_pmu>(); if pmu.is_null() { return -ENOMEM; }
    (*pmu).idxd = idxd; (*idxd).idxd_pmu = pmu;
    let rc = if (*idxd).data.type_ == IDXD_TYPE_DSA { sprintf((*pmu).name, c"dsa%d", (*idxd).id) } else if (*idxd).data.type_ == IDXD_TYPE_IAX { sprintf((*pmu).name, c"iax%d", (*idxd).id) } else { kfree(pmu); (*idxd).idxd_pmu = core::ptr::null_mut(); return -ENODEV; };
    if rc < 0 { kfree(pmu); (*idxd).idxd_pmu = core::ptr::null_mut(); return rc; }
    perfmon_reset(idxd);
    let perfcap = ioread64(PERFCAP_REG!(idxd));
    if perfcap.num_perf_counter == 0 || perfcap.counter_width == 0 || !perfcap.overflow_interrupt || !perfcap.counter_freeze || perfcap.num_event_category == 0 || perfcap.cap_per_counter { kfree(pmu); (*idxd).idxd_pmu = core::ptr::null_mut(); return rc; }
    (*pmu).n_event_categories = perfcap.num_event_category; (*pmu).supported_event_categories = perfcap.global_event_category; (*pmu).per_counter_caps_supported = perfcap.cap_per_counter;
    (*pmu).supported_filters = perfcap.filter; if perfcap.filter != 0 { (*pmu).n_filters = hweight8(perfcap.filter); }
    (*pmu).n_counters = perfcap.num_perf_counter; (*pmu).counter_width = perfcap.counter_width;
    idxd_pmu_init(pmu);
    let rc = perf_pmu_register(&mut (*pmu).pmu, (*pmu).name, -1); if rc != 0 { kfree(pmu); (*idxd).idxd_pmu = core::ptr::null_mut(); }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
