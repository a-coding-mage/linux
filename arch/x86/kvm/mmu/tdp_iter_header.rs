// SPDX-License-Identifier: GPL-2.0

// Translated from tdp_iter.h.  Dependencies supplied by the surrounding KVM
// translation are intentionally referenced but not redefined here.

#[inline]
pub unsafe fn kvm_tdp_mmu_read_spte(sptep: tdp_ptep_t) -> u64 {
    READ_ONCE(*rcu_dereference(sptep))
}

// mmu_lock must be held for write when using the atomic APIs, except when KVM
// is freezing a leaf SPTE for removal.
#[inline]
pub unsafe fn kvm_tdp_mmu_write_spte_atomic(sptep: tdp_ptep_t, new_spte: u64) -> u64 {
    KVM_MMU_WARN_ON(is_ept_ve_possible(new_spte));
    xchg(rcu_dereference(sptep), new_spte)
}

#[inline]
pub unsafe fn tdp_mmu_clear_spte_bits_atomic(sptep: tdp_ptep_t, mask: u64) -> u64 {
    let sptep_atomic: *mut atomic64_t = rcu_dereference(sptep) as *mut atomic64_t;
    atomic64_fetch_and(!mask, sptep_atomic) as u64
}

#[inline]
pub unsafe fn __kvm_tdp_mmu_write_spte(sptep: tdp_ptep_t, new_spte: u64) {
    KVM_MMU_WARN_ON(is_ept_ve_possible(new_spte));
    WRITE_ONCE(*rcu_dereference(sptep), new_spte);
}

// SPTEs must be modified atomically if they are shadow-present, leaf SPTEs,
// and have volatile bits that must not be clobbered.
#[inline]
pub fn kvm_tdp_mmu_spte_need_atomic_update(old_spte: u64, level: i32) -> bool {
    is_shadow_present_pte(old_spte)
        && is_last_spte(old_spte, level)
        && spte_needs_atomic_update(old_spte)
}

#[inline]
pub unsafe fn kvm_tdp_mmu_write_spte(
    sptep: tdp_ptep_t,
    old_spte: u64,
    new_spte: u64,
    level: i32,
) -> u64 {
    if kvm_tdp_mmu_spte_need_atomic_update(old_spte, level) {
        return kvm_tdp_mmu_write_spte_atomic(sptep, new_spte);
    }

    __kvm_tdp_mmu_write_spte(sptep, new_spte);
    old_spte
}

#[inline]
pub unsafe fn tdp_mmu_clear_spte_bits(
    sptep: tdp_ptep_t,
    old_spte: u64,
    mask: u64,
    level: i32,
) -> u64 {
    if kvm_tdp_mmu_spte_need_atomic_update(old_spte, level) {
        return tdp_mmu_clear_spte_bits_atomic(sptep, mask);
    }

    __kvm_tdp_mmu_write_spte(sptep, old_spte & !mask);
    old_spte
}

// A TDP iterator performs a pre-order walk over a TDP paging structure.
#[repr(C)]
pub struct tdp_iter {
    // The iterator will traverse the paging structure towards this GFN.
    pub next_last_level_gfn: gfn_t,
    // The next_last_level_gfn when the thread last yielded.
    pub yielded_gfn: gfn_t,
    // Pointers to the page tables traversed to reach the current SPTE.
    pub pt_path: [tdp_ptep_t; PT64_ROOT_MAX_LEVEL as usize],
    // A pointer to the current SPTE.
    pub sptep: tdp_ptep_t,
    // The lowest GFN mapped by the current SPTE.
    pub gfn: gfn_t,
    // Mask applied to convert the GFN to the mapping GPA.
    pub gfn_bits: gfn_t,
    pub root_level: i32,
    pub min_level: i32,
    pub level: i32,
    // The address space ID, i.e. SMM vs. regular.
    pub as_id: i32,
    // A snapshot of the value at sptep.
    pub old_spte: u64,
    // False if the iterator walks off the end of the paging structure.
    pub valid: bool,
    // True if KVM dropped mmu_lock and yielded in the middle of a walk.
    pub yielded: bool,
}

// Iterates over every SPTE mapping the GFN range [start, end) in preorder.
#[macro_export]
macro_rules! for_each_tdp_pte_min_level {
    ($iter:expr, $kvm:expr, $root:expr, $min_level:expr, $start:expr, $end:expr) => {
        for tdp_iter_start(&mut $iter, $root, $min_level, $start, kvm_gfn_root_bits($kvm, $root));
            $iter.valid && $iter.gfn < $end;
            tdp_iter_next(&mut $iter)
    };
}

#[macro_export]
macro_rules! for_each_tdp_pte_min_level_all {
    ($iter:expr, $root:expr, $min_level:expr) => {
        for tdp_iter_start(&mut $iter, $root, $min_level, 0, 0);
            $iter.valid && $iter.gfn < tdp_mmu_max_gfn_exclusive();
            tdp_iter_next(&mut $iter)
    };
}

#[macro_export]
macro_rules! for_each_tdp_pte {
    ($iter:expr, $kvm:expr, $root:expr, $start:expr, $end:expr) => {
        for_each_tdp_pte_min_level!($iter, $kvm, $root, PG_LEVEL_4K, $start, $end)
    };
}

pub fn spte_to_child_pt(pte: u64, level: i32) -> tdp_ptep_t;
pub fn tdp_iter_start(
    iter: *mut tdp_iter,
    root: *mut kvm_mmu_page,
    min_level: i32,
    next_last_level_gfn: gfn_t,
    gfn_bits: gfn_t,
);
pub fn tdp_iter_next(iter: *mut tdp_iter);
pub fn tdp_iter_restart(iter: *mut tdp_iter);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
