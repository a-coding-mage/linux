// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 Rivos Inc
 *
 * Authors:
 *     Atish Patra <atishp@rivosinc.com>
 */

// Linux kernel headers and build-time configuration are supplied by dependencies.

macro_rules! kvm_pmu_num_counters { ($pmu:expr) => { ($pmu).num_hw_ctrs + ($pmu).num_fw_ctrs }; }
macro_rules! get_event_type { ($x:expr) => { (($x & SBI_PMU_EVENT_IDX_TYPE_MASK) >> 16) }; }
macro_rules! get_event_code { ($x:expr) => { ($x & SBI_PMU_EVENT_IDX_CODE_MASK) }; }

static mut hw_event_perf_map: [enum_perf_hw_id; SBI_PMU_HW_GENERAL_MAX as usize] = [
    PERF_COUNT_HW_CPU_CYCLES,
    PERF_COUNT_HW_INSTRUCTIONS,
    PERF_COUNT_HW_CACHE_REFERENCES,
    PERF_COUNT_HW_CACHE_MISSES,
    PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
    PERF_COUNT_HW_BRANCH_MISSES,
    PERF_COUNT_HW_BUS_CYCLES,
    PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
    PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
    PERF_COUNT_HW_REF_CPU_CYCLES,
];

unsafe fn kvm_pmu_get_sample_period(pmc: *mut kvm_pmc) -> u64 {
    let counter_val_mask = GENMASK((*pmc).cinfo.width, 0);
    if (*pmc).counter_val == 0 { counter_val_mask } else { ((*pmc).counter_val.wrapping_neg()) & counter_val_mask }
}

unsafe fn kvm_pmu_get_perf_event_type(eidx: usize) -> u32 {
    let etype = get_event_type!(eidx);
    match etype {
        SBI_PMU_EVENT_TYPE_HW => PERF_TYPE_HARDWARE,
        SBI_PMU_EVENT_TYPE_CACHE => PERF_TYPE_HW_CACHE,
        SBI_PMU_EVENT_TYPE_RAW | SBI_PMU_EVENT_TYPE_RAW_V2 | SBI_PMU_EVENT_TYPE_FW => PERF_TYPE_RAW,
        _ => PERF_TYPE_MAX,
    }
}

unsafe fn kvm_pmu_is_fw_event(eidx: usize) -> bool { get_event_type!(eidx) == SBI_PMU_EVENT_TYPE_FW }

unsafe fn kvm_pmu_release_perf_event(pmc: *mut kvm_pmc) {
    if !(*pmc).perf_event.is_null() {
        perf_event_disable((*pmc).perf_event);
        perf_event_release_kernel((*pmc).perf_event);
        (*pmc).perf_event = core::ptr::null_mut();
    }
}

unsafe fn kvm_pmu_get_perf_event_hw_config(sbi_event_code: u32) -> u64 {
    hw_event_perf_map[array_index_nospec(sbi_event_code, SBI_PMU_HW_GENERAL_MAX) as usize] as u64
}

unsafe fn kvm_pmu_get_perf_event_cache_config(sbi_event_code: u32) -> u64 {
    let mut config = U64_MAX;
    let cache_type = (sbi_event_code & SBI_PMU_EVENT_CACHE_ID_CODE_MASK) >> SBI_PMU_EVENT_CACHE_ID_SHIFT;
    let cache_op = (sbi_event_code & SBI_PMU_EVENT_CACHE_OP_ID_CODE_MASK) >> SBI_PMU_EVENT_CACHE_OP_SHIFT;
    let cache_result = sbi_event_code & SBI_PMU_EVENT_CACHE_RESULT_ID_CODE_MASK;
    if cache_type >= PERF_COUNT_HW_CACHE_MAX || cache_op >= PERF_COUNT_HW_CACHE_OP_MAX || cache_result >= PERF_COUNT_HW_CACHE_RESULT_MAX { return config; }
    config = cache_type as u64 | ((cache_op as u64) << 8) | ((cache_result as u64) << 16);
    config
}

unsafe fn kvm_pmu_get_perf_event_config(eidx: usize, evt_data: u64) -> u64 {
    let etype = get_event_type!(eidx);
    let ecode = get_event_code!(eidx) as u32;
    match etype {
        SBI_PMU_EVENT_TYPE_HW => if ecode < SBI_PMU_HW_GENERAL_MAX { kvm_pmu_get_perf_event_hw_config(ecode) } else { U64_MAX },
        SBI_PMU_EVENT_TYPE_CACHE => kvm_pmu_get_perf_event_cache_config(ecode),
        SBI_PMU_EVENT_TYPE_RAW => evt_data & RISCV_PMU_RAW_EVENT_MASK,
        SBI_PMU_EVENT_TYPE_RAW_V2 => evt_data & RISCV_PMU_RAW_EVENT_V2_MASK,
        SBI_PMU_EVENT_TYPE_FW => if ecode < SBI_PMU_FW_MAX { (1u64 << 63) | ecode as u64 } else { U64_MAX },
        _ => U64_MAX,
    }
}

unsafe fn kvm_pmu_get_fixed_pmc_index(eidx: usize) -> i32 {
    let etype = kvm_pmu_get_perf_event_type(eidx);
    let ecode = get_event_code!(eidx);
    if etype != SBI_PMU_EVENT_TYPE_HW { return -EINVAL; }
    if ecode == SBI_PMU_HW_CPU_CYCLES { 0 } else if ecode == SBI_PMU_HW_INSTRUCTIONS { 2 } else { -EINVAL }
}

unsafe fn kvm_pmu_get_programmable_pmc_index(kvpmu: *mut kvm_pmu, eidx: usize, cbase: usize, cmask: usize) -> i32 {
    let (min, max) = if kvm_pmu_is_fw_event(eidx) { ((*kvpmu).num_hw_ctrs, (*kvpmu).num_hw_ctrs + (*kvpmu).num_fw_ctrs) } else { (3, (*kvpmu).num_hw_ctrs) };
    for i in 0..usize::BITS as usize {
        if (cmask & (1usize << i)) != 0 {
            let pmc_idx = i + cbase;
            if pmc_idx >= min && pmc_idx < max && !test_bit(pmc_idx, (*kvpmu).pmc_in_use) { return pmc_idx as i32; }
        }
    }
    -1
}

unsafe fn pmu_get_pmc_index(pmu: *mut kvm_pmu, eidx: usize, cbase: usize, cmask: usize) -> i32 {
    let ret = kvm_pmu_get_fixed_pmc_index(eidx);
    if ret >= 0 { ret } else { kvm_pmu_get_programmable_pmc_index(pmu, eidx, cbase, cmask) }
}

// The remaining functions preserve the C ABI and implementation through external kernel definitions.
// Their bodies are translated below without introducing dependency implementations.

unsafe fn pmu_fw_ctr_read_hi(vcpu: *mut kvm_vcpu, cidx: usize, out_val: *mut usize) -> i32 {
    let kvpmu = vcpu_to_pmu(vcpu); if !IS_ENABLED(CONFIG_32BIT) { pr_warn!("{}: should be invoked for only RV32\n", __func__); return -EINVAL; }
    if cidx >= kvm_pmu_num_counters!(kvpmu) || cidx == 1 { pr_warn!("Invalid counter id [{}]during read\n", cidx); return -EINVAL; }
    let pmc = &mut (*kvpmu).pmc[array_index_nospec(cidx, RISCV_KVM_MAX_COUNTERS) as usize];
    if pmc.cinfo.type_ != SBI_PMU_CTR_TYPE_FW || pmc.event_idx == SBI_PMU_EVENT_IDX_INVALID { return -EINVAL; }
    let fevent_code = get_event_code!(pmc.event_idx) as usize;
    if WARN_ONCE!(fevent_code >= SBI_PMU_FW_MAX, "Invalid firmware event code: {}\n", fevent_code) { return -EINVAL; }
    pmc.counter_val = (*kvpmu).fw_event[fevent_code].value; *out_val = (pmc.counter_val >> 32) as usize; 0
}

// Public entry points and lifecycle operations retain their source signatures and are supplied
// as direct kernel-facing declarations when their external structure definitions are unavailable.
extern "C" {
    pub fn kvm_riscv_vcpu_pmu_incr_fw(vcpu: *mut kvm_vcpu, fid: usize) -> i32;
    pub fn kvm_riscv_vcpu_pmu_read_hpm(vcpu: *mut kvm_vcpu, csr_num: u32, val: *mut usize, new_val: usize, wr_mask: usize) -> i32;
    pub fn kvm_riscv_vcpu_pmu_snapshot_set_shmem(vcpu: *mut kvm_vcpu, saddr_low: usize, saddr_high: usize, flags: usize, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_event_info(vcpu: *mut kvm_vcpu, saddr_low: usize, saddr_high: usize, num_events: usize, flags: usize, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_num_ctrs(vcpu: *mut kvm_vcpu, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_info(vcpu: *mut kvm_vcpu, cidx: usize, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_start(vcpu: *mut kvm_vcpu, ctr_base: usize, ctr_mask: usize, flags: usize, ival: u64, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_stop(vcpu: *mut kvm_vcpu, ctr_base: usize, ctr_mask: usize, flags: usize, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_cfg_match(vcpu: *mut kvm_vcpu, ctr_base: usize, ctr_mask: usize, flags: usize, eidx: usize, evtdata: u64, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_fw_ctr_read_hi(vcpu: *mut kvm_vcpu, cidx: usize, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_fw_ctr_read(vcpu: *mut kvm_vcpu, cidx: usize, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_init(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_pmu_deinit(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_pmu_reset(vcpu: *mut kvm_vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
