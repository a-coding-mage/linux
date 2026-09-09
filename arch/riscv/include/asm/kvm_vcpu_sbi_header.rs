/* SPDX-License-Identifier: GPL-2.0-only */
/**
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 */

pub const KVM_SBI_IMPID: u32 = 3;

pub const KVM_SBI_VERSION_MAJOR: u32 = 3;
pub const KVM_SBI_VERSION_MINOR: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum kvm_riscv_sbi_ext_status {
    KVM_RISCV_SBI_EXT_STATUS_UNINITIALIZED,
    KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE,
    KVM_RISCV_SBI_EXT_STATUS_ENABLED,
    KVM_RISCV_SBI_EXT_STATUS_DISABLED,
}

#[repr(C)]
pub struct kvm_vcpu_sbi_context {
    pub return_handled: core::ffi::c_int,
    pub ext_status: [kvm_riscv_sbi_ext_status; KVM_RISCV_SBI_EXT_MAX],
}

#[repr(C)]
pub struct kvm_vcpu_sbi_return {
    pub out_val: core::ffi::c_ulong,
    pub err_val: core::ffi::c_ulong,
    pub utrap: *mut kvm_cpu_trap,
    pub uexit: bool,
}

#[repr(C)]
pub struct kvm_vcpu_sbi_extension {
    pub extid_start: core::ffi::c_ulong,
    pub extid_end: core::ffi::c_ulong,
    pub default_disabled: bool,
    pub handler: Option<unsafe extern "C" fn(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        retdata: *mut kvm_vcpu_sbi_return,
    ) -> core::ffi::c_int>,
    pub probe: Option<unsafe extern "C" fn(vcpu: *mut kvm_vcpu) -> core::ffi::c_ulong>,
    pub init: Option<unsafe extern "C" fn(vcpu: *mut kvm_vcpu) -> core::ffi::c_int>,
    pub deinit: Option<unsafe extern "C" fn(vcpu: *mut kvm_vcpu)>,
    pub reset: Option<unsafe extern "C" fn(vcpu: *mut kvm_vcpu)>,
    pub validate: Option<unsafe extern "C" fn(vcpu: *mut kvm_vcpu)>,
    pub state_reg_subtype: core::ffi::c_ulong,
    pub get_state_reg_count:
        Option<unsafe extern "C" fn(vcpu: *mut kvm_vcpu) -> core::ffi::c_ulong>,
    pub get_state_reg_id: Option<unsafe extern "C" fn(
        vcpu: *mut kvm_vcpu,
        index: core::ffi::c_int,
        reg_id: *mut u64,
    ) -> core::ffi::c_int>,
    pub get_state_reg: Option<unsafe extern "C" fn(
        vcpu: *mut kvm_vcpu,
        reg_num: core::ffi::c_ulong,
        reg_size: core::ffi::c_ulong,
        reg_val: *mut core::ffi::c_void,
    ) -> core::ffi::c_int>,
    pub set_state_reg: Option<unsafe extern "C" fn(
        vcpu: *mut kvm_vcpu,
        reg_num: core::ffi::c_ulong,
        reg_size: core::ffi::c_ulong,
        reg_val: *const core::ffi::c_void,
    ) -> core::ffi::c_int>,
}

extern "C" {
    pub fn kvm_riscv_vcpu_sbi_forward_handler(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        retdata: *mut kvm_vcpu_sbi_return,
    ) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_sbi_system_reset(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        type_: u32,
        flags: u64,
    );
    pub fn kvm_riscv_vcpu_sbi_request_reset(
        vcpu: *mut kvm_vcpu,
        pc: core::ffi::c_ulong,
        a1: core::ffi::c_ulong,
    );
    pub fn kvm_riscv_vcpu_sbi_load_reset_state(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_sbi_return(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_reg_indices_sbi_ext(
        vcpu: *mut kvm_vcpu,
        uindices: *mut u64,
    ) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_set_reg_sbi_ext(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
    ) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_get_reg_sbi_ext(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
    ) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_reg_indices_sbi(
        vcpu: *mut kvm_vcpu,
        uindices: *mut u64,
    ) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_set_reg_sbi(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
    ) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_get_reg_sbi(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
    ) -> core::ffi::c_int;
    pub fn kvm_vcpu_sbi_find_ext(
        vcpu: *mut kvm_vcpu,
        extid: core::ffi::c_ulong,
    ) -> *const kvm_vcpu_sbi_extension;
    pub fn kvm_riscv_vcpu_sbi_ecall(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> core::ffi::c_int;
    pub fn kvm_riscv_vcpu_sbi_init(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_sbi_deinit(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_sbi_reset(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_sbi_validate(vcpu: *mut kvm_vcpu);

    // Preserved from CONFIG_RISCV_SBI_V01.
    #[cfg(CONFIG_RISCV_SBI_V01)]
    pub static vcpu_sbi_ext_v01: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_base: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_time: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_ipi: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_rfence: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_srst: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_hsm: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_dbcn: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_susp: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_sta: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_fwft: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_mpxy: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_experimental: kvm_vcpu_sbi_extension;
    pub static vcpu_sbi_ext_vendor: kvm_vcpu_sbi_extension;
    // Preserved from CONFIG_RISCV_PMU_SBI.
    #[cfg(CONFIG_RISCV_PMU_SBI)]
    pub static vcpu_sbi_ext_pmu: kvm_vcpu_sbi_extension;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
