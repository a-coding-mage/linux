/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Google LLC
 * Author: Quentin Perret <qperret@google.com>
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust types and symbols.

#[repr(C)]
pub struct host_mmu {
    pub arch: kvm_arch,
    pub pgt: kvm_pgtable,
    pub mm_ops: kvm_pgtable_mm_ops,
    pub lock: hyp_spinlock_t,
}

extern "C" {
    pub static mut host_mmu: host_mmu;
}

/* This corresponds to page-table locking order */
#[repr(C)]
pub enum pkvm_component_id {
    PKVM_ID_HOST,
    PKVM_ID_HYP,
    PKVM_ID_GUEST,
}

extern "C" {
    pub fn __pkvm_prot_finalize() -> c_int;
    pub fn __pkvm_host_share_hyp(pfn: u64) -> c_int;
    pub fn __pkvm_guest_share_host(vcpu: *mut pkvm_hyp_vcpu, gfn: u64) -> c_int;
    pub fn __pkvm_guest_unshare_host(vcpu: *mut pkvm_hyp_vcpu, gfn: u64) -> c_int;
    pub fn __pkvm_host_unshare_hyp(pfn: u64) -> c_int;
    pub fn __pkvm_host_donate_hyp(pfn: u64, nr_pages: u64) -> c_int;
    pub fn __pkvm_hyp_donate_host(pfn: u64, nr_pages: u64) -> c_int;
    pub fn __pkvm_host_share_ffa(pfn: u64, nr_pages: u64) -> c_int;
    pub fn __pkvm_host_unshare_ffa(pfn: u64, nr_pages: u64) -> c_int;
    pub fn __pkvm_host_donate_guest(pfn: u64, gfn: u64, vcpu: *mut pkvm_hyp_vcpu) -> c_int;
    pub fn __pkvm_vcpu_in_poison_fault(hyp_vcpu: *mut pkvm_hyp_vcpu) -> c_int;
    pub fn __pkvm_host_force_reclaim_page_guest(phys: phys_addr_t) -> c_int;
    pub fn __pkvm_host_reclaim_page_guest(gfn: u64, vm: *mut pkvm_hyp_vm) -> c_int;
    pub fn __pkvm_host_share_guest(
        pfn: u64, gfn: u64, nr_pages: u64, vcpu: *mut pkvm_hyp_vcpu,
        prot: kvm_pgtable_prot,
    ) -> c_int;
    pub fn __pkvm_host_unshare_guest(gfn: u64, nr_pages: u64, hyp_vm: *mut pkvm_hyp_vm) -> c_int;
    pub fn __pkvm_host_relax_perms_guest(
        gfn: u64, vcpu: *mut pkvm_hyp_vcpu, prot: kvm_pgtable_prot,
    ) -> c_int;
    pub fn __pkvm_host_wrprotect_guest(gfn: u64, nr_pages: u64, hyp_vm: *mut pkvm_hyp_vm) -> c_int;
    pub fn __pkvm_host_test_clear_young_guest(
        gfn: u64, nr_pages: u64, mkold: bool, vm: *mut pkvm_hyp_vm,
    ) -> c_int;
    pub fn __pkvm_host_mkyoung_guest(gfn: u64, vcpu: *mut pkvm_hyp_vcpu) -> c_int;

    pub fn addr_is_memory(phys: phys_addr_t) -> bool;
    pub fn host_stage2_idmap_locked(addr: phys_addr_t, size: u64, prot: kvm_pgtable_prot) -> c_int;
    pub fn host_stage2_set_owner_locked(addr: phys_addr_t, size: u64, owner_id: u8) -> c_int;
    pub fn kvm_host_prepare_stage2(pgt_pool_base: *mut core::ffi::c_void) -> c_int;
    pub fn kvm_guest_prepare_stage2(vm: *mut pkvm_hyp_vm, pgd: *mut core::ffi::c_void) -> c_int;
    pub fn kvm_guest_destroy_stage2(vm: *mut pkvm_hyp_vm);
    pub fn handle_host_mem_abort(host_ctxt: *mut kvm_cpu_context);

    pub fn hyp_pin_shared_mem(from: *mut core::ffi::c_void, to: *mut core::ffi::c_void) -> c_int;
    pub fn hyp_unpin_shared_mem(from: *mut core::ffi::c_void, to: *mut core::ffi::c_void);
    pub fn reclaim_pgtable_pages(vm: *mut pkvm_hyp_vm, mc: *mut kvm_hyp_memcache);
    pub fn refill_memcache(mc: *mut kvm_hyp_memcache, min_pages: c_ulong,
                           host_mc: *mut kvm_hyp_memcache) -> c_int;
}

#[inline(always)]
pub unsafe fn __load_host_stage2() {
    if static_branch_likely(&kvm_protected_mode_initialized) {
        __load_stage2(&mut host_mmu.arch.mmu);
    } else {
        write_sysreg(0, vttbr_el2);
    }
}

#[cfg(feature = "CONFIG_NVHE_EL2_DEBUG")]
extern "C" {
    pub fn pkvm_ownership_selftest(base: *mut core::ffi::c_void);
    pub fn init_selftest_vm(virt: *mut core::ffi::c_void) -> *mut pkvm_hyp_vcpu;
    pub fn teardown_selftest_vm();
}

#[cfg(not(feature = "CONFIG_NVHE_EL2_DEBUG"))]
#[inline]
pub fn pkvm_ownership_selftest(_base: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
