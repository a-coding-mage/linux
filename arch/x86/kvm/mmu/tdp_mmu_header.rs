// SPDX-License-Identifier: GPL-2.0

// Translated from x86/kvm/mmu/tdp_mmu.h.
// C dependencies supplied by the surrounding translation unit are intentionally
// referenced but not implemented here.

extern "C" {
    pub fn kvm_mmu_init_tdp_mmu(kvm: *mut kvm);
    pub fn kvm_mmu_uninit_tdp_mmu(kvm: *mut kvm);

    pub fn kvm_tdp_mmu_alloc_root(vcpu: *mut kvm_vcpu, private: bool);

    pub fn kvm_tdp_mmu_put_root(kvm: *mut kvm, root: *mut kvm_mmu_page);

    pub fn kvm_tdp_mmu_zap_leafs(kvm: *mut kvm, start: gfn_t, end: gfn_t, flush: bool) -> bool;
    pub fn kvm_tdp_mmu_zap_possible_nx_huge_page(
        kvm: *mut kvm,
        sp: *mut kvm_mmu_page,
    ) -> bool;
    pub fn kvm_tdp_mmu_zap_all(kvm: *mut kvm);
    pub fn kvm_tdp_mmu_invalidate_roots(kvm: *mut kvm, root_types: kvm_tdp_mmu_root_types);
    pub fn kvm_tdp_mmu_zap_invalidated_roots(kvm: *mut kvm, shared: bool);

    pub fn kvm_tdp_mmu_map(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault) -> ::core::ffi::c_int;

    pub fn kvm_tdp_mmu_unmap_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range, flush: bool) -> bool;
    pub fn kvm_tdp_mmu_age_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
    pub fn kvm_tdp_mmu_test_age_gfn(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;

    pub fn kvm_tdp_mmu_wrprot_slot(
        kvm: *mut kvm,
        slot: *const kvm_memory_slot,
        min_level: ::core::ffi::c_int,
    ) -> bool;
    pub fn kvm_tdp_mmu_clear_dirty_slot(kvm: *mut kvm, slot: *const kvm_memory_slot);
    pub fn kvm_tdp_mmu_clear_dirty_pt_masked(
        kvm: *mut kvm,
        slot: *mut kvm_memory_slot,
        gfn: gfn_t,
        mask: ::core::ffi::c_ulong,
        wrprot: bool,
    );
    pub fn kvm_tdp_mmu_recover_huge_pages(kvm: *mut kvm, slot: *const kvm_memory_slot);

    pub fn kvm_tdp_mmu_write_protect_gfn(
        kvm: *mut kvm,
        slot: *mut kvm_memory_slot,
        gfn: gfn_t,
        min_level: ::core::ffi::c_int,
    ) -> bool;

    pub fn kvm_tdp_mmu_try_split_huge_pages(
        kvm: *mut kvm,
        slot: *const kvm_memory_slot,
        start: gfn_t,
        end: gfn_t,
        target_level: ::core::ffi::c_int,
        shared: bool,
    );

    pub fn kvm_tdp_mmu_get_walk(
        vcpu: *mut kvm_vcpu,
        addr: u64,
        sptes: *mut u64,
        root_level: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn kvm_tdp_mmu_fast_pf_get_last_sptep(
        vcpu: *mut kvm_vcpu,
        gfn: gfn_t,
        spte: *mut u64,
    ) -> *mut u64;
}

pub type kvm_tdp_mmu_root_types = u32;
pub const KVM_INVALID_ROOTS: kvm_tdp_mmu_root_types = 1 << 0;
pub const KVM_DIRECT_ROOTS: kvm_tdp_mmu_root_types = 1 << 1;
pub const KVM_MIRROR_ROOTS: kvm_tdp_mmu_root_types = 1 << 2;
pub const KVM_VALID_ROOTS: kvm_tdp_mmu_root_types = KVM_DIRECT_ROOTS | KVM_MIRROR_ROOTS;
pub const KVM_ALL_ROOTS: kvm_tdp_mmu_root_types = KVM_VALID_ROOTS | KVM_INVALID_ROOTS;

#[inline]
pub unsafe fn kvm_tdp_mmu_get_root(root: *mut kvm_mmu_page) -> bool {
    refcount_inc_not_zero(&mut (*root).tdp_mmu_root_count)
}

#[inline]
pub unsafe fn kvm_gfn_range_filter_to_root_types(
    kvm: *mut kvm,
    process: kvm_gfn_range_filter,
) -> kvm_tdp_mmu_root_types {
    let mut ret: kvm_tdp_mmu_root_types = 0;

    if !kvm_has_mirrored_tdp(kvm) {
        return KVM_DIRECT_ROOTS;
    }

    if process & KVM_FILTER_PRIVATE != 0 {
        ret |= KVM_MIRROR_ROOTS;
    }
    if process & KVM_FILTER_SHARED != 0 {
        ret |= KVM_DIRECT_ROOTS;
    }

    WARN_ON_ONCE(ret == 0);
    ret
}

#[inline]
pub unsafe fn tdp_mmu_get_root_for_fault(
    vcpu: *mut kvm_vcpu,
    fault: *mut kvm_page_fault,
) -> *mut kvm_mmu_page {
    if !kvm_is_addr_direct((*vcpu).kvm, (*fault).addr) {
        return root_to_sp((*(*vcpu).arch.mmu).mirror_root_hpa);
    }
    root_to_sp((*(*vcpu).arch.mmu).root.hpa)
}

#[inline]
pub unsafe fn tdp_mmu_get_root(
    vcpu: *mut kvm_vcpu,
    type_: kvm_tdp_mmu_root_types,
) -> *mut kvm_mmu_page {
    if type_ == KVM_MIRROR_ROOTS {
        return root_to_sp((*(*vcpu).arch.mmu).mirror_root_hpa);
    }
    root_to_sp((*(*vcpu).arch.mmu).root.hpa)
}

#[inline]
pub unsafe fn kvm_tdp_mmu_walk_lockless_begin() {
    rcu_read_lock();
}

#[inline]
pub unsafe fn kvm_tdp_mmu_walk_lockless_end() {
    rcu_read_unlock();
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn is_tdp_mmu_page(sp: *mut kvm_mmu_page) -> bool {
    (*sp).tdp_mmu_page
}

#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn is_tdp_mmu_page(_sp: *mut kvm_mmu_page) -> bool {
    false
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
