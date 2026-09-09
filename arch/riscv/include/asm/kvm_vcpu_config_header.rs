/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2026 Qualcomm Technologies, Inc.
 */

// Dependency intent: `u64` corresponds to Linux `u64`, and `c_ulong` to
// Linux `unsigned long`.

#[repr(C)]
pub struct kvm_vcpu_config {
    pub henvcfg: u64,
    pub hstateen0: u64,
    pub hedeleg: ::core::ffi::c_ulong,
    pub hideleg: ::core::ffi::c_ulong,
}

pub struct kvm_vcpu;

unsafe extern "C" {
    pub fn kvm_riscv_vcpu_config_init(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_config_guest_debug(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_config_ran_once(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_config_load(vcpu: *mut kvm_vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
