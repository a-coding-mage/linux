// SPDX-License-Identifier: GPL-2.0-only
// External kernel declarations and macros referenced below are supplied by the surrounding kernel translation.

static mut PERF_NMI_WINDOW: usize = 0;
static mut AMD_PMU_GLOBAL_CNTR_MASK: u64 = 0;

const AMD_MERGE_EVENT: u64 = (0x0f_u64 << 32) | 0xff;
const AMD_MERGE_EVENT_ENABLE: u64 = AMD_MERGE_EVENT | ARCH_PERFMON_EVENTSEL_ENABLE;
const OVERFLOW_WAIT_COUNT: u32 = 50;

static mut PERF_NMI_TSTAMP: usize = 0;

static AMD_HW_CACHE_EVENT_IDS: [[[u64; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX] = [[[0; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX];
static AMD_HW_CACHE_EVENT_IDS_F17H: [[[u64; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX] = [[[0; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX];

static AMD_PERFMON_EVENT_MAP: [u64; PERF_COUNT_HW_MAX] = [
    0x0076, 0x00c0, 0x077d, 0x077e, 0x00c2, 0x00c3, 0x00d0, 0x00d1,
];
static AMD_ZEN1_PERFMON_EVENT_MAP: [u64; PERF_COUNT_HW_MAX] = [
    0x0076, 0x00c0, 0xff60, 0x0964, 0x00c2, 0x00c3, 0x0287, 0x0187,
];
static AMD_ZEN2_PERFMON_EVENT_MAP: [u64; PERF_COUNT_HW_MAX] = [
    0x0076, 0x00c0, 0xff60, 0x0964, 0x00c2, 0x00c3, 0x00a9, 0,
];
static AMD_ZEN4_PERFMON_EVENT_MAP: [u64; PERF_COUNT_HW_MAX] = [
    0x0076, 0x00c0, 0xff60, 0x0964, 0x00c2, 0x00c3, 0x00a9, 0x100000120,
];

static mut EVENT_OFFSETS: [u32; X86_PMC_IDX_MAX] = [0; X86_PMC_IDX_MAX];
static mut COUNT_OFFSETS: [u32; X86_PMC_IDX_MAX] = [0; X86_PMC_IDX_MAX];

unsafe fn amd_pmu_event_map(hw_event: usize) -> u64 {
    if cpu_feature_enabled(X86_FEATURE_ZEN4) || boot_cpu_data.x86 >= 0x1a { return AMD_ZEN4_PERFMON_EVENT_MAP[hw_event]; }
    if cpu_feature_enabled(X86_FEATURE_ZEN2) || boot_cpu_data.x86 >= 0x19 { return AMD_ZEN2_PERFMON_EVENT_MAP[hw_event]; }
    if cpu_feature_enabled(X86_FEATURE_ZEN1) { return AMD_ZEN1_PERFMON_EVENT_MAP[hw_event]; }
    AMD_PERFMON_EVENT_MAP[hw_event]
}

unsafe fn amd_pmu_addr_offset(index: usize, eventsel: bool) -> i32 {
    if index == 0 { return 0; }
    let mut offset = if eventsel { EVENT_OFFSETS[index] } else { COUNT_OFFSETS[index] };
    if offset == 0 { offset = if !boot_cpu_has(X86_FEATURE_PERFCTR_CORE) { index as u32 } else { (index << 1) as u32 }; if eventsel { EVENT_OFFSETS[index] = offset; } else { COUNT_OFFSETS[index] = offset; } }
    offset as i32
}

unsafe fn amd_get_event_code(hwc: *const hw_perf_event) -> u32 { ((*hwc).config >> 24 & 0x0f00 | (*hwc).config & 0x00ff) as u32 }
unsafe fn amd_is_pair_event_code(hwc: *const hw_perf_event) -> bool {
    if x86_pmu.flags & PMU_FL_PAIR == 0 { return false; }
    matches!(amd_get_event_code(hwc), 0x003)
}

unsafe fn amd_core_hw_config(event: *mut perf_event) -> i32 {
    if (*event).attr.exclude_host && (*event).attr.exclude_guest { (*event).hw.config &= !(ARCH_PERFMON_EVENTSEL_USR | ARCH_PERFMON_EVENTSEL_OS); }
    else if (*event).attr.exclude_host { (*event).hw.config |= AMD64_EVENTSEL_GUESTONLY; }
    else if (*event).attr.exclude_guest { (*event).hw.config |= AMD64_EVENTSEL_HOSTONLY; }
    if x86_pmu.flags & PMU_FL_PAIR != 0 && amd_is_pair_event_code(&(*event).hw) { (*event).hw.flags |= PERF_X86_EVENT_PAIR; }
    if has_branch_stack(event) { return static_call(amd_pmu_branch_hw_config)(event); }
    0
}

unsafe fn amd_is_nb_event(hwc: *const hw_perf_event) -> i32 { (((*hwc).config & 0xe0) == 0xe0) as i32 }
unsafe fn amd_has_nb(cpuc: *const cpu_hw_events) -> bool { !(*cpuc).amd_nb.is_null() && (*(*cpuc).amd_nb).nb_id != -1 }

unsafe fn amd_pmu_hw_config(event: *mut perf_event) -> i32 {
    if (*event).attr.precise_ip != 0 && get_ibs_caps() { return forward_event_to_ibs(event); }
    if has_branch_stack(event) && x86_pmu.lbr_nr == 0 { return -EOPNOTSUPP; }
    let ret = x86_pmu_hw_config(event); if ret != 0 { return ret; }
    if (*event).attr.type_ == PERF_TYPE_RAW { (*event).hw.config |= (*event).attr.config & AMD64_RAW_EVENT_MASK; }
    amd_core_hw_config(event)
}

unsafe fn amd_pmu_cpu_reset(_cpu: i32) { if x86_pmu.lbr_nr != 0 { static_call(amd_pmu_branch_reset)(); } if x86_pmu.version < 2 { return; } wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_CTL, 0); wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR, GLOBAL_STATUS_LBRS_FROZEN | AMD_PMU_GLOBAL_CNTR_MASK); }
unsafe fn amd_pmu_set_global_ctl(ctl: u64) { wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_CTL, ctl); }
unsafe fn amd_pmu_get_global_status() -> u64 { let mut status = 0; rdmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS, status); status }
unsafe fn amd_pmu_ack_global_status(status: u64) { wrmsrq(MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR, status); }
unsafe fn amd_pmu_test_overflow_topbit(idx: i32) -> bool { let mut counter = 0; rdmsrq(x86_pmu_event_addr(idx), counter); (counter & BIT_ULL(x86_pmu.cntval_bits - 1)) == 0 }
unsafe fn amd_pmu_test_overflow_status(idx: i32) -> bool { amd_pmu_get_global_status() & BIT_ULL(idx) != 0 }

unsafe fn amd_pmu_wait_on_overflow(idx: i32) { for _ in 0..OVERFLOW_WAIT_COUNT { if !static_call(amd_pmu_test_overflow)(idx) { break; } udelay(1); } }
unsafe fn amd_pmu_check_overflow() { let cpuc = this_cpu_ptr(&cpu_hw_events); if in_nmi() { return; } for_each_set_bit(idx, x86_pmu.cntr_mask, X86_PMC_IDX_MAX) { if test_bit(idx, (*cpuc).active_mask) { amd_pmu_wait_on_overflow(idx); } } }
unsafe fn amd_pmu_enable_event(event: *mut perf_event) { x86_pmu_enable_event(event); }
unsafe fn amd_pmu_enable_all(_added: i32) { amd_brs_enable_all(); let cpuc = this_cpu_ptr(&cpu_hw_events); for_each_set_bit(idx, x86_pmu.cntr_mask, X86_PMC_IDX_MAX) { if test_bit(idx, (*cpuc).active_mask) && !(*cpuc).events[idx].is_null() { amd_pmu_enable_event((*cpuc).events[idx]); } } }
unsafe fn amd_pmu_disable_event(event: *mut perf_event) { x86_pmu_disable_event(event); if !in_nmi() { amd_pmu_wait_on_overflow((*event).hw.idx); } }
unsafe fn amd_pmu_disable_all() { amd_brs_disable_all(); x86_pmu_disable_all(); amd_pmu_check_overflow(); }

unsafe fn amd_pmu_adjust_nmi_window(handled: i32) -> i32 { if handled != 0 { this_cpu_write(PERF_NMI_TSTAMP, jiffies + PERF_NMI_WINDOW); return handled; } if time_after(jiffies, this_cpu_read(PERF_NMI_TSTAMP)) { return NMI_DONE; } NMI_HANDLED }
unsafe fn amd_pmu_handle_irq(regs: *mut pt_regs) -> i32 { let cpuc = this_cpu_ptr(&cpu_hw_events); let enabled = (*cpuc).enabled; (*cpuc).enabled = 0; amd_brs_disable_all(); if (*cpuc).lbr_users != 0 { amd_brs_drain(); } let handled = x86_pmu_handle_irq(regs); (*cpuc).enabled = enabled; if enabled { amd_brs_enable_all(); } amd_pmu_adjust_nmi_window(handled) }

// Family-15h event constraints and Family-17h/19h pair/BRS constraints are represented by the same external kernel structures.
unsafe fn amd_get_event_constraints(cpuc: *mut cpu_hw_events, _idx: i32, event: *mut perf_event) -> *mut event_constraint { if !(amd_has_nb(cpuc) && amd_is_nb_event(&(*event).hw) != 0) { return &mut unconstrained; } __amd_get_nb_event_constraints(cpuc, event, core::ptr::null_mut()) }
unsafe fn amd_put_event_constraints(cpuc: *mut cpu_hw_events, event: *mut perf_event) { if amd_has_nb(cpuc) && amd_is_nb_event(&(*event).hw) != 0 { __amd_put_nb_event_constraints(cpuc, event); } }

unsafe fn amd_event_sysfs_show(page: *mut i8, config: u64) -> isize { let event = (config & ARCH_PERFMON_EVENTSEL_EVENT) | ((config & AMD64_EVENTSEL_EVENT) >> 24); x86_event_sysfs_show(page, config, event) }
unsafe fn amd_pmu_limit_period(event: *mut perf_event, left: *mut i64) { if has_branch_stack(event) && *left > x86_pmu.lbr_nr as i64 { *left -= x86_pmu.lbr_nr as i64; } }

unsafe fn amd_core_pmu_init() -> i32 { PERF_NMI_WINDOW = msecs_to_jiffies(100); if !boot_cpu_has(X86_FEATURE_PERFCTR_CORE) { return 0; } x86_pmu.eventsel = MSR_F15H_PERF_CTL; x86_pmu.perfctr = MSR_F15H_PERF_CTR; x86_pmu.cntr_mask64 = GENMASK_ULL(AMD64_NUM_COUNTERS_CORE - 1, 0); x86_pmu.amd_nb_constraints = 0; 0 }

pub unsafe fn amd_pmu_init() -> i32 { if boot_cpu_data.x86 < 6 { return -ENODEV; } x86_pmu = amd_pmu; let ret = amd_core_pmu_init(); if ret != 0 { return ret; } if num_possible_cpus() == 1 { x86_pmu.amd_nb_constraints = 0; } if boot_cpu_data.x86 >= 0x17 { memcpy(hw_cache_event_ids, AMD_HW_CACHE_EVENT_IDS_F17H.as_ptr(), core::mem::size_of_val(&AMD_HW_CACHE_EVENT_IDS_F17H)); } else { memcpy(hw_cache_event_ids, AMD_HW_CACHE_EVENT_IDS.as_ptr(), core::mem::size_of_val(&AMD_HW_CACHE_EVENT_IDS)); } 0 }
unsafe fn amd_pmu_reload_virt() { if x86_pmu.version >= 2 { amd_pmu_v2_disable_all(); __amd_pmu_enable_all(); amd_pmu_v2_enable_all(0); } else { amd_pmu_disable_all(); amd_pmu_enable_all(0); } }
pub unsafe fn amd_pmu_enable_virt() { let cpuc = this_cpu_ptr(&cpu_hw_events); (*cpuc).perf_ctr_virt_mask = 0; amd_pmu_reload_virt(); }
pub unsafe fn amd_pmu_disable_virt() { let cpuc = this_cpu_ptr(&cpu_hw_events); (*cpuc).perf_ctr_virt_mask = AMD64_EVENTSEL_HOSTONLY; amd_pmu_reload_virt(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
