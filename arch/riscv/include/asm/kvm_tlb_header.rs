/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2025 Ventana Micro Systems Inc.
 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kvm_riscv_hfence_type {
    KVM_RISCV_HFENCE_UNKNOWN = 0,
    KVM_RISCV_HFENCE_GVMA_VMID_GPA,
    KVM_RISCV_HFENCE_GVMA_VMID_ALL,
    KVM_RISCV_HFENCE_VVMA_ASID_GVA,
    KVM_RISCV_HFENCE_VVMA_ASID_ALL,
    KVM_RISCV_HFENCE_VVMA_GVA,
    KVM_RISCV_HFENCE_VVMA_ALL,
}

#[repr(C)]
pub struct kvm_riscv_hfence {
    pub type_: kvm_riscv_hfence_type,
    pub asid: ::core::ffi::c_ulong,
    pub vmid: ::core::ffi::c_ulong,
    pub order: ::core::ffi::c_ulong,
    pub addr: gpa_t,
    pub size: gpa_t,
}

pub const KVM_RISCV_VCPU_MAX_HFENCE: u32 = 64;
pub const KVM_RISCV_GSTAGE_TLB_MIN_ORDER: u32 = 12;

extern "C" {
    pub fn kvm_riscv_local_hfence_gvma_vmid_gpa(
        vmid: ::core::ffi::c_ulong,
        gpa: gpa_t,
        gpsz: gpa_t,
        order: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_local_hfence_gvma_vmid_all(vmid: ::core::ffi::c_ulong);
    pub fn kvm_riscv_local_hfence_gvma_gpa(
        gpa: gpa_t,
        gpsz: gpa_t,
        order: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_local_hfence_gvma_all();
    pub fn kvm_riscv_local_hfence_vvma_asid_gva(
        vmid: ::core::ffi::c_ulong,
        asid: ::core::ffi::c_ulong,
        gva: ::core::ffi::c_ulong,
        gvsz: ::core::ffi::c_ulong,
        order: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_local_hfence_vvma_asid_all(
        vmid: ::core::ffi::c_ulong,
        asid: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_local_hfence_vvma_gva(
        vmid: ::core::ffi::c_ulong,
        gva: ::core::ffi::c_ulong,
        gvsz: ::core::ffi::c_ulong,
        order: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_local_hfence_vvma_all(vmid: ::core::ffi::c_ulong);
    pub fn kvm_riscv_local_tlb_sanitize(vcpu: *mut kvm_vcpu);

    pub fn kvm_riscv_tlb_flush_process(vcpu: *mut kvm_vcpu);

    pub fn kvm_riscv_fence_i_process(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_hfence_vvma_all_process(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_hfence_process(vcpu: *mut kvm_vcpu);

    pub fn kvm_riscv_fence_i(
        kvm: *mut kvm,
        hbase: ::core::ffi::c_ulong,
        hmask: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_hfence_gvma_vmid_gpa(
        kvm: *mut kvm,
        hbase: ::core::ffi::c_ulong,
        hmask: ::core::ffi::c_ulong,
        gpa: gpa_t,
        gpsz: gpa_t,
        order: ::core::ffi::c_ulong,
        vmid: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_hfence_gvma_vmid_all(
        kvm: *mut kvm,
        hbase: ::core::ffi::c_ulong,
        hmask: ::core::ffi::c_ulong,
        vmid: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_hfence_vvma_asid_gva(
        kvm: *mut kvm,
        hbase: ::core::ffi::c_ulong,
        hmask: ::core::ffi::c_ulong,
        gva: ::core::ffi::c_ulong,
        gvsz: ::core::ffi::c_ulong,
        order: ::core::ffi::c_ulong,
        asid: ::core::ffi::c_ulong,
        vmid: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_hfence_vvma_asid_all(
        kvm: *mut kvm,
        hbase: ::core::ffi::c_ulong,
        hmask: ::core::ffi::c_ulong,
        asid: ::core::ffi::c_ulong,
        vmid: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_hfence_vvma_gva(
        kvm: *mut kvm,
        hbase: ::core::ffi::c_ulong,
        hmask: ::core::ffi::c_ulong,
        gva: ::core::ffi::c_ulong,
        gvsz: ::core::ffi::c_ulong,
        order: ::core::ffi::c_ulong,
        vmid: ::core::ffi::c_ulong,
    );
    pub fn kvm_riscv_hfence_vvma_all(
        kvm: *mut kvm,
        hbase: ::core::ffi::c_ulong,
        hmask: ::core::ffi::c_ulong,
        vmid: ::core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
