/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 Google LLC
 * Author: Fuad Tabba <tabba@google.com>
 */

// Translated from the C header. Dependencies are supplied by other headers.

#[repr(C)]
pub struct pkvm_hyp_vcpu {
    pub vcpu: kvm_vcpu,
    /* Backpointer to the host's (untrusted) vCPU instance. */
    pub host_vcpu: *mut kvm_vcpu,
    /*
     * If this hyp VCPU is loaded, this points back to the per-CPU pointer
     * tracking us. Otherwise it is NULL if not loaded.
     */
    pub loaded_hyp_vcpu: *mut *mut pkvm_hyp_vcpu,
}

#[repr(C)]
pub struct pkvm_hyp_vm {
    pub kvm: kvm,
    /* Backpointer to the host's (untrusted) KVM instance. */
    pub host_kvm: *mut kvm,
    /* The guest's stage-2 page-table managed by the hypervisor. */
    pub pgt: kvm_pgtable,
    pub mm_ops: kvm_pgtable_mm_ops,
    pub pool: hyp_pool,
    pub lock: hyp_spinlock_t,
    /* Array of the hyp vCPU structures for this VM. */
    pub vcpus: [*mut pkvm_hyp_vcpu; 0],
}

extern "C" {
    pub static mut vm_table_lock: hyp_spinlock_t;

    pub fn pkvm_hyp_vm_table_init(tbl: *mut core::ffi::c_void);

    pub fn __pkvm_reserve_vm() -> core::ffi::c_int;
    pub fn __pkvm_unreserve_vm(handle: pkvm_handle_t);
    pub fn __pkvm_init_vm(
        host_kvm: *mut kvm,
        vm_hva: core::ffi::c_ulong,
        pgd_hva: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn __pkvm_init_vcpu(
        handle: pkvm_handle_t,
        host_vcpu: *mut kvm_vcpu,
        vcpu_hva: core::ffi::c_ulong,
    ) -> core::ffi::c_int;

    pub fn __pkvm_reclaim_dying_guest_page(
        handle: pkvm_handle_t,
        gfn: u64,
    ) -> core::ffi::c_int;
    pub fn __pkvm_start_teardown_vm(handle: pkvm_handle_t) -> core::ffi::c_int;
    pub fn __pkvm_finalize_teardown_vm(handle: pkvm_handle_t) -> core::ffi::c_int;

    pub fn get_vm_by_handle(handle: pkvm_handle_t) -> *mut pkvm_hyp_vm;
    pub fn pkvm_load_hyp_vcpu(
        handle: pkvm_handle_t,
        vcpu_idx: core::ffi::c_uint,
    ) -> *mut pkvm_hyp_vcpu;
    pub fn pkvm_put_hyp_vcpu(hyp_vcpu: *mut pkvm_hyp_vcpu);
    pub fn pkvm_get_loaded_hyp_vcpu() -> *mut pkvm_hyp_vcpu;

    pub fn get_pkvm_hyp_vm(handle: pkvm_handle_t) -> *mut pkvm_hyp_vm;
    pub fn get_np_pkvm_hyp_vm(handle: pkvm_handle_t) -> *mut pkvm_hyp_vm;
    pub fn put_pkvm_hyp_vm(hyp_vm: *mut pkvm_hyp_vm);

    pub fn kvm_handle_pvm_hvc64(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool;
    pub fn kvm_handle_pvm_sysreg(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool;
    pub fn kvm_handle_pvm_restricted(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool;
    pub fn kvm_init_pvm_id_regs(vcpu: *mut kvm_vcpu);
    pub fn kvm_check_pvm_sysreg_table() -> core::ffi::c_int;
}

#[inline]
pub unsafe fn pkvm_hyp_vcpu_to_hyp_vm(
    hyp_vcpu: *mut pkvm_hyp_vcpu,
) -> *mut pkvm_hyp_vm {
    // container_of(hyp_vcpu->vcpu.kvm, struct pkvm_hyp_vm, kvm): kvm is first.
    (*hyp_vcpu).vcpu.kvm as *mut pkvm_hyp_vm
}

#[inline]
pub unsafe fn pkvm_hyp_vcpu_is_protected(hyp_vcpu: *mut pkvm_hyp_vcpu) -> bool {
    vcpu_is_protected(&(*hyp_vcpu).vcpu)
}

#[inline]
pub unsafe fn pkvm_hyp_vm_is_protected(hyp_vm: *mut pkvm_hyp_vm) -> bool {
    kvm_vm_is_protected(&(*hyp_vm).kvm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
