/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 SiFive
 * Copyright (C) 2018 Andes Technology Corporation
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 */

/* C dependencies: linux/perf_event.h, linux/ptrace.h, linux/interrupt.h */
/* This header's declarations are conditional on CONFIG_RISCV_PMU. */

pub const RISCV_MAX_COUNTERS: usize = 64;
pub const RISCV_OP_UNSUPP: i32 = -EOPNOTSUPP;
pub const RISCV_PMU_SBI_PDEV_NAME: &[u8] = b"riscv-pmu-sbi\0";
pub const RISCV_PMU_LEGACY_PDEV_NAME: &[u8] = b"riscv-pmu-legacy\0";

pub const RISCV_PMU_STOP_FLAG_RESET: u32 = 1;
pub const RISCV_PMU_CONFIG1_GUEST_EVENTS: u32 = 0x1;

#[repr(C)]
pub struct cpu_hw_events {
    /* currently enabled events */
    pub n_events: i32,
    /* Counter overflow interrupt */
    pub irq: i32,
    /* currently enabled events */
    pub events: [*mut perf_event; RISCV_MAX_COUNTERS],
    /* currently enabled hardware counters */
    pub used_hw_ctrs: [c_ulong; RISCV_MAX_COUNTERS],
    /* currently enabled firmware counters */
    pub used_fw_ctrs: [c_ulong; RISCV_MAX_COUNTERS],
    /* The virtual address of the shared memory where counter snapshot will be taken */
    pub snapshot_addr: *mut core::ffi::c_void,
    /* The physical address of the shared memory where counter snapshot will be taken */
    pub snapshot_addr_phys: phys_addr_t,
    /* Boolean flag to indicate setup is already done */
    pub snapshot_set_done: bool,
    /* A shadow copy of the counter values to avoid clobbering during multiple SBI calls */
    pub snapshot_cval_shcopy: [u64; RISCV_MAX_COUNTERS],
}

#[repr(C)]
pub struct riscv_pmu {
    pub pmu: pmu,
    pub name: *mut core::ffi::c_char,

    pub handle_irq: Option<unsafe extern "C" fn(irq_num: i32, dev: *mut core::ffi::c_void) -> irqreturn_t>,

    pub cmask: c_ulong,
    pub ctr_read: Option<unsafe extern "C" fn(event: *mut perf_event) -> u64>,
    pub ctr_get_idx: Option<unsafe extern "C" fn(event: *mut perf_event) -> i32>,
    pub ctr_get_width: Option<unsafe extern "C" fn(idx: i32) -> i32>,
    pub ctr_clear_idx: Option<unsafe extern "C" fn(event: *mut perf_event)>,
    pub ctr_start: Option<unsafe extern "C" fn(event: *mut perf_event, init_val: u64)>,
    pub ctr_stop: Option<unsafe extern "C" fn(event: *mut perf_event, flag: c_ulong)>,
    pub event_map: Option<unsafe extern "C" fn(event: *mut perf_event, config: *mut u64) -> i32>,
    pub event_init: Option<unsafe extern "C" fn(event: *mut perf_event)>,
    pub event_mapped: Option<unsafe extern "C" fn(event: *mut perf_event, mm: *mut mm_struct)>,
    pub event_unmapped: Option<unsafe extern "C" fn(event: *mut perf_event, mm: *mut mm_struct)>,
    pub csr_index: Option<unsafe extern "C" fn(event: *mut perf_event) -> u8>,

    pub hw_events: *mut cpu_hw_events,
    pub node: hlist_node,
    pub riscv_pm_nb: notifier_block,
}

/* Equivalent to container_of(p, struct riscv_pmu, pmu). */
#[inline]
pub unsafe fn to_riscv_pmu(p: *mut pmu) -> *mut riscv_pmu {
    (p as *mut u8).sub(core::mem::offset_of!(riscv_pmu, pmu)) as *mut riscv_pmu
}

extern "C" {
    pub fn riscv_pmu_start(event: *mut perf_event, flags: i32);
    pub fn riscv_pmu_stop(event: *mut perf_event, flags: i32);
    pub fn riscv_pmu_ctr_read_csr(csr: c_ulong) -> c_ulong;
    pub fn riscv_pmu_event_set_period(event: *mut perf_event) -> i32;
    pub fn riscv_pmu_ctr_get_width_mask(event: *mut perf_event) -> u64;
    pub fn riscv_pmu_event_update(event: *mut perf_event) -> u64;
    pub fn riscv_pmu_legacy_skip_init();
    pub fn riscv_pmu_alloc() -> *mut riscv_pmu;
    pub fn riscv_pmu_get_hpm_info(hw_ctr_width: *mut u32, num_hw_ctr: *mut u32) -> i32;
    pub fn riscv_pmu_get_event_info(ty: u32, config: u64, econfig: *mut u64) -> i32;
}

/* External types/constants supplied by the included Linux headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
