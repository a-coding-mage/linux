/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019 Arm Ltd. */

// Dependency supplied by asm/kvm_emulate.h.

use core::ffi::c_ulong;

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_device_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_one_reg {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn kvm_smccc_call_handler(vcpu: *mut kvm_vcpu) -> i32;

    fn vcpu_get_reg(vcpu: *mut kvm_vcpu, reg: u8) -> c_ulong;
    fn vcpu_set_reg(vcpu: *mut kvm_vcpu, reg: u8, value: c_ulong);

    pub fn kvm_arm_init_hypercalls(kvm: *mut kvm);
    pub fn kvm_arm_teardown_hypercalls(kvm: *mut kvm);
    pub fn kvm_arm_get_fw_num_regs(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_arm_copy_fw_reg_indices(vcpu: *mut kvm_vcpu, uindices: *mut u64) -> i32;
    pub fn kvm_arm_get_fw_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    pub fn kvm_arm_set_fw_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;

    pub fn kvm_vm_smccc_has_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> i32;
    pub fn kvm_vm_smccc_set_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> i32;
}

#[inline]
pub unsafe fn smccc_get_function(vcpu: *mut kvm_vcpu) -> u32 {
    vcpu_get_reg(vcpu, 0) as u32
}

#[inline]
pub unsafe fn smccc_get_arg1(vcpu: *mut kvm_vcpu) -> c_ulong {
    vcpu_get_reg(vcpu, 1)
}

#[inline]
pub unsafe fn smccc_get_arg2(vcpu: *mut kvm_vcpu) -> c_ulong {
    vcpu_get_reg(vcpu, 2)
}

#[inline]
pub unsafe fn smccc_get_arg3(vcpu: *mut kvm_vcpu) -> c_ulong {
    vcpu_get_reg(vcpu, 3)
}

#[inline]
pub unsafe fn smccc_set_retval(
    vcpu: *mut kvm_vcpu,
    a0: c_ulong,
    a1: c_ulong,
    a2: c_ulong,
    a3: c_ulong,
) {
    vcpu_set_reg(vcpu, 0, a0);
    vcpu_set_reg(vcpu, 1, a1);
    vcpu_set_reg(vcpu, 2, a2);
    vcpu_set_reg(vcpu, 3, a3);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
