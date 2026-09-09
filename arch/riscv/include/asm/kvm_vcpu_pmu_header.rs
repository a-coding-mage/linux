/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2023 Rivos Inc
 *
 * Authors:
 *     Atish Patra <atishp@rivosinc.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/perf/riscv_pmu.h, asm/kvm_vcpu_insn.h, asm/sbi.h

#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
pub const RISCV_KVM_MAX_FW_CTRS: usize = 32;
#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
pub const RISCV_KVM_MAX_HW_CTRS: usize = 32;
#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
pub const RISCV_KVM_MAX_COUNTERS: usize = RISCV_KVM_MAX_HW_CTRS + RISCV_KVM_MAX_FW_CTRS;

#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
#[repr(C)]
pub struct kvm_fw_event {
    /* Current value of the event */
    pub value: u64,
    /* Event monitoring status */
    pub started: bool,
}

#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
#[repr(C)]
pub struct kvm_pmc {
    pub idx: u8,
    pub perf_event: *mut perf_event,
    pub counter_val: u64,
    pub cinfo: sbi_pmu_ctr_info,
    /* Event monitoring status */
    pub started: bool,
    /* Monitoring event ID */
    pub event_idx: ::core::ffi::c_ulong,
    pub vcpu: *mut kvm_vcpu,
}

#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
#[repr(C)]
pub struct kvm_pmu {
    pub pmc: [kvm_pmc; RISCV_KVM_MAX_COUNTERS],
    pub fw_event: [kvm_fw_event; RISCV_KVM_MAX_FW_CTRS],
    /* Number of the virtual firmware counters available */
    pub num_fw_ctrs: i32,
    /* Number of the virtual hardware counters available */
    pub num_hw_ctrs: i32,
    /* A flag to indicate that pmu initialization is done */
    pub init_done: bool,
    /* Bit map of all the virtual counter used */
    pub pmc_in_use: [u64; (RISCV_KVM_MAX_COUNTERS + 63) / 64],
    /* Bit map of all the virtual counter overflown */
    pub pmc_overflown: [u64; (RISCV_KVM_MAX_COUNTERS + 63) / 64],
    /* The address of the counter snapshot area (guest physical address) */
    pub snapshot_addr: gpa_t,
    /* The actual data of the snapshot */
    pub sdata: *mut riscv_pmu_snapshot_data,
}

#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
#[inline]
pub unsafe fn vcpu_to_pmu(vcpu: *mut kvm_vcpu) -> *mut kvm_pmu {
    &mut (*vcpu).arch.pmu_context
}

#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
#[inline]
pub unsafe fn pmu_to_vcpu(pmu: *mut kvm_pmu) -> *mut kvm_vcpu {
    container_of!(pmu, kvm_vcpu, arch.pmu_context)
}

#[cfg(feature = "CONFIG_32BIT")]
#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
macro_rules! KVM_RISCV_VCPU_HPMCOUNTER_CSR_FUNCS {
    () => { { base: CSR_CYCLEH, count: 32, func: kvm_riscv_vcpu_pmu_read_hpm },
            { base: CSR_CYCLE, count: 32, func: kvm_riscv_vcpu_pmu_read_hpm } };
}

#[cfg(all(not(feature = "CONFIG_32BIT"), feature = "CONFIG_RISCV_PMU_SBI"))]
macro_rules! KVM_RISCV_VCPU_HPMCOUNTER_CSR_FUNCS {
    () => { { base: CSR_CYCLE, count: 32, func: kvm_riscv_vcpu_pmu_read_hpm } };
}

#[cfg(feature = "CONFIG_RISCV_PMU_SBI")]
extern "C" {
    pub fn kvm_riscv_vcpu_pmu_incr_fw(vcpu: *mut kvm_vcpu, fid: ::core::ffi::c_ulong) -> i32;
    pub fn kvm_riscv_vcpu_pmu_read_hpm(vcpu: *mut kvm_vcpu, csr_num: u32, val: *mut ::core::ffi::c_ulong, new_val: ::core::ffi::c_ulong, wr_mask: ::core::ffi::c_ulong) -> i32;
    pub fn kvm_riscv_vcpu_pmu_num_ctrs(vcpu: *mut kvm_vcpu, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_info(vcpu: *mut kvm_vcpu, cidx: ::core::ffi::c_ulong, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_start(vcpu: *mut kvm_vcpu, ctr_base: ::core::ffi::c_ulong, ctr_mask: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong, ival: u64, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_stop(vcpu: *mut kvm_vcpu, ctr_base: ::core::ffi::c_ulong, ctr_mask: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_ctr_cfg_match(vcpu: *mut kvm_vcpu, ctr_base: ::core::ffi::c_ulong, ctr_mask: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong, eidx: ::core::ffi::c_ulong, evtdata: u64, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_fw_ctr_read(vcpu: *mut kvm_vcpu, cidx: ::core::ffi::c_ulong, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_fw_ctr_read_hi(vcpu: *mut kvm_vcpu, cidx: ::core::ffi::c_ulong, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_init(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_pmu_snapshot_set_shmem(vcpu: *mut kvm_vcpu, saddr_low: ::core::ffi::c_ulong, saddr_high: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_event_info(vcpu: *mut kvm_vcpu, saddr_low: ::core::ffi::c_ulong, saddr_high: ::core::ffi::c_ulong, num_events: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong, retdata: *mut kvm_vcpu_sbi_return) -> i32;
    pub fn kvm_riscv_vcpu_pmu_deinit(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_pmu_reset(vcpu: *mut kvm_vcpu);
}

#[cfg(not(feature = "CONFIG_RISCV_PMU_SBI"))]
#[repr(C)]
pub struct kvm_pmu {}

#[cfg(not(feature = "CONFIG_RISCV_PMU_SBI"))]
pub unsafe fn kvm_riscv_vcpu_pmu_read_legacy(vcpu: *mut kvm_vcpu, csr_num: u32, val: *mut ::core::ffi::c_ulong, _new_val: ::core::ffi::c_ulong, _wr_mask: ::core::ffi::c_ulong) -> i32 {
    if csr_num == CSR_CYCLE || csr_num == CSR_INSTRET {
        *val = 0;
        KVM_INSN_CONTINUE_NEXT_SEPC
    } else {
        KVM_INSN_ILLEGAL_TRAP
    }
}

#[cfg(not(feature = "CONFIG_RISCV_PMU_SBI"))]
macro_rules! KVM_RISCV_VCPU_HPMCOUNTER_CSR_FUNCS {
    () => { { base: CSR_CYCLE, count: 3, func: kvm_riscv_vcpu_pmu_read_legacy } };
}

#[cfg(not(feature = "CONFIG_RISCV_PMU_SBI"))]
pub unsafe fn kvm_riscv_vcpu_pmu_init(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(feature = "CONFIG_RISCV_PMU_SBI"))]
pub unsafe fn kvm_riscv_vcpu_pmu_incr_fw(_vcpu: *mut kvm_vcpu, _fid: ::core::ffi::c_ulong) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_RISCV_PMU_SBI"))]
pub unsafe fn kvm_riscv_vcpu_pmu_deinit(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(feature = "CONFIG_RISCV_PMU_SBI"))]
pub unsafe fn kvm_riscv_vcpu_pmu_reset(_vcpu: *mut kvm_vcpu) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
