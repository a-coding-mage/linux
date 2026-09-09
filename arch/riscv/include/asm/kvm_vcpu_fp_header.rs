/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 *     Anup Patel <anup.patel@wdc.com>
 */

// The C header guard and Linux include are not executable Rust constructs.

pub enum kvm_cpu_context {}
pub enum kvm_vcpu {}
pub enum kvm_one_reg {}

// CONFIG_FPU conditionally selects the real declarations versus no-op inline
// implementations in the original header.
#[cfg(feature = "CONFIG_FPU")]
extern "C" {
    pub fn __kvm_riscv_fp_f_save(context: *mut kvm_cpu_context);
    pub fn __kvm_riscv_fp_f_restore(context: *mut kvm_cpu_context);
    pub fn __kvm_riscv_fp_d_save(context: *mut kvm_cpu_context);
    pub fn __kvm_riscv_fp_d_restore(context: *mut kvm_cpu_context);

    pub fn kvm_riscv_vcpu_fp_reset(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_guest_fp_save(cntx: *mut kvm_cpu_context, isa: *const usize);
    pub fn kvm_riscv_vcpu_guest_fp_restore(cntx: *mut kvm_cpu_context, isa: *const usize);
    pub fn kvm_riscv_vcpu_host_fp_save(cntx: *mut kvm_cpu_context);
    pub fn kvm_riscv_vcpu_host_fp_restore(cntx: *mut kvm_cpu_context);
}

#[cfg(not(feature = "CONFIG_FPU"))]
#[inline]
pub unsafe fn kvm_riscv_vcpu_fp_reset(_vcpu: *mut kvm_vcpu) {}

#[cfg(not(feature = "CONFIG_FPU"))]
#[inline]
pub unsafe fn kvm_riscv_vcpu_guest_fp_save(
    _cntx: *mut kvm_cpu_context,
    _isa: *const usize,
) {
}

#[cfg(not(feature = "CONFIG_FPU"))]
#[inline]
pub unsafe fn kvm_riscv_vcpu_guest_fp_restore(
    _cntx: *mut kvm_cpu_context,
    _isa: *const usize,
) {
}

#[cfg(not(feature = "CONFIG_FPU"))]
#[inline]
pub unsafe fn kvm_riscv_vcpu_host_fp_save(_cntx: *mut kvm_cpu_context) {}

#[cfg(not(feature = "CONFIG_FPU"))]
#[inline]
pub unsafe fn kvm_riscv_vcpu_host_fp_restore(_cntx: *mut kvm_cpu_context) {}

extern "C" {
    pub fn kvm_riscv_vcpu_get_reg_fp(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
        rtype: usize,
    ) -> i32;
    pub fn kvm_riscv_vcpu_set_reg_fp(
        vcpu: *mut kvm_vcpu,
        reg: *const kvm_one_reg,
        rtype: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
