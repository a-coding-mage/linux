/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2025 Ventana Micro Systems Inc.
 */

// Dependency supplied by the Linux KVM headers.

#[repr(C)]
pub struct kvm_vmid {
    /*
     * Writes to vmid_version and vmid happen with vmid_lock held
     * whereas reads happen without any lock held.
     */
    pub vmid_version: usize,
    pub vmid: usize,
}

// `struct kvm` and `struct kvm_vcpu` are supplied by dependent headers.
#[repr(C)]
pub struct kvm;
#[repr(C)]
pub struct kvm_vcpu;

extern "C" {
    pub fn kvm_riscv_gstage_vmid_detect();
    pub fn kvm_riscv_gstage_vmid_bits() -> usize;
    pub fn kvm_riscv_gstage_vmid_init(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn kvm_riscv_gstage_vmid_ver_changed(vmid: *mut kvm_vmid) -> bool;
    pub fn kvm_riscv_gstage_vmid_update(vcpu: *mut kvm_vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
