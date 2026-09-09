/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from swapops.h. The original declarations are guarded by CONFIG_MMU. */

#[cfg(CONFIG_MMU)]
pub const SWP_TYPE_SHIFT: usize = BITS_PER_XA_VALUE - MAX_SWAPFILES_SHIFT;
#[cfg(CONFIG_MMU)]
pub const SWP_OFFSET_MASK: usize = (1usize << SWP_TYPE_SHIFT) - 1;

#[cfg(CONFIG_MMU)]
pub const SWP_PFN_BITS: usize = if cfg!(MAX_PHYSMEM_BITS) {
    MAX_PHYSMEM_BITS - PAGE_SHIFT
} else {
    core::cmp::min(
        core::mem::size_of::<phys_addr_t>() * 8 - PAGE_SHIFT,
        SWP_TYPE_SHIFT,
    )
};
#[cfg(CONFIG_MMU)]
pub const SWP_PFN_MASK: usize = (1usize << SWP_PFN_BITS) - 1;

#[cfg(CONFIG_MMU)]
pub const SWP_MIG_YOUNG_BIT: usize = SWP_PFN_BITS;
#[cfg(CONFIG_MMU)]
pub const SWP_MIG_DIRTY_BIT: usize = SWP_PFN_BITS + 1;
#[cfg(CONFIG_MMU)]
pub const SWP_MIG_TOTAL_BITS: usize = SWP_PFN_BITS + 2;
#[cfg(CONFIG_MMU)]
pub const SWP_MIG_YOUNG: usize = 1usize << SWP_MIG_YOUNG_BIT;
#[cfg(CONFIG_MMU)]
pub const SWP_MIG_DIRTY: usize = 1usize << SWP_MIG_DIRTY_BIT;

#[cfg(CONFIG_MMU)]
pub unsafe fn pte_swp_clear_flags(mut pte: pte_t) -> pte_t {
    if pte_swp_exclusive(pte) {
        pte = pte_swp_clear_exclusive(pte);
    }
    if pte_swp_soft_dirty(pte) {
        pte = pte_swp_clear_soft_dirty(pte);
    }
    if pte_swp_uffd(pte) {
        pte = pte_swp_clear_uffd(pte);
    }
    pte
}

#[cfg(CONFIG_MMU)]
pub fn swp_entry(type_: c_ulong, offset: pgoff_t) -> swp_entry_t {
    let mut ret: swp_entry_t = unsafe { core::mem::zeroed() };
    ret.val = (type_ << SWP_TYPE_SHIFT) | (offset & SWP_OFFSET_MASK as pgoff_t);
    ret
}

#[cfg(CONFIG_MMU)]
pub fn swp_type(entry: swp_entry_t) -> c_uint {
    entry.val >> SWP_TYPE_SHIFT
}

#[cfg(CONFIG_MMU)]
pub fn swp_offset(entry: swp_entry_t) -> pgoff_t {
    entry.val & SWP_OFFSET_MASK as pgoff_t
}

#[cfg(CONFIG_MMU)]
pub unsafe fn swp_entry_to_pte(entry: swp_entry_t) -> pte_t {
    let arch_entry = __swp_entry(swp_type(entry), swp_offset(entry));
    __swp_entry_to_pte(arch_entry)
}

#[cfg(CONFIG_MMU)]
pub unsafe fn radix_to_swp_entry(arg: *mut core::ffi::c_void) -> swp_entry_t {
    let mut entry: swp_entry_t = core::mem::zeroed();
    entry.val = xa_to_value(arg);
    entry
}

#[cfg(CONFIG_MMU)]
pub unsafe fn swp_to_radix_entry(entry: swp_entry_t) -> *mut core::ffi::c_void {
    xa_mk_value(entry.val)
}

#[cfg(all(CONFIG_MMU, CONFIG_DEVICE_PRIVATE))]
pub fn make_readable_device_private_entry(offset: pgoff_t) -> swp_entry_t { swp_entry(SWP_DEVICE_READ, offset) }
#[cfg(all(CONFIG_MMU, CONFIG_DEVICE_PRIVATE))]
pub fn make_writable_device_private_entry(offset: pgoff_t) -> swp_entry_t { swp_entry(SWP_DEVICE_WRITE, offset) }
#[cfg(all(CONFIG_MMU, CONFIG_DEVICE_PRIVATE))]
pub fn make_device_exclusive_entry(offset: pgoff_t) -> swp_entry_t { swp_entry(SWP_DEVICE_EXCLUSIVE, offset) }

#[cfg(all(CONFIG_MMU, not(CONFIG_DEVICE_PRIVATE)))]
pub fn make_readable_device_private_entry(_offset: pgoff_t) -> swp_entry_t { swp_entry(0, 0) }
#[cfg(all(CONFIG_MMU, not(CONFIG_DEVICE_PRIVATE)))]
pub fn make_writable_device_private_entry(_offset: pgoff_t) -> swp_entry_t { swp_entry(0, 0) }
#[cfg(all(CONFIG_MMU, not(CONFIG_DEVICE_PRIVATE)))]
pub fn make_device_exclusive_entry(_offset: pgoff_t) -> swp_entry_t { swp_entry(0, 0) }

#[cfg(all(CONFIG_MMU, CONFIG_MIGRATION))]
pub fn make_readable_migration_entry(offset: pgoff_t) -> swp_entry_t { swp_entry(SWP_MIGRATION_READ, offset) }
#[cfg(all(CONFIG_MMU, CONFIG_MIGRATION))]
pub fn make_readable_exclusive_migration_entry(offset: pgoff_t) -> swp_entry_t { swp_entry(SWP_MIGRATION_READ_EXCLUSIVE, offset) }
#[cfg(all(CONFIG_MMU, CONFIG_MIGRATION))]
pub fn make_writable_migration_entry(offset: pgoff_t) -> swp_entry_t { swp_entry(SWP_MIGRATION_WRITE, offset) }

#[cfg(all(CONFIG_MMU, CONFIG_MIGRATION))]
pub fn migration_entry_supports_ad() -> bool {
    #[cfg(CONFIG_SWAP)]
    { swap_migration_ad_supported }
    #[cfg(not(CONFIG_SWAP))]
    { false }
}

#[cfg(all(CONFIG_MMU, CONFIG_MIGRATION))]
pub fn make_migration_entry_young(entry: swp_entry_t) -> swp_entry_t {
    if migration_entry_supports_ad() { swp_entry(swp_type(entry), swp_offset(entry) | SWP_MIG_YOUNG as pgoff_t) } else { entry }
}
#[cfg(all(CONFIG_MMU, CONFIG_MIGRATION))]
pub fn make_migration_entry_dirty(entry: swp_entry_t) -> swp_entry_t {
    if migration_entry_supports_ad() { swp_entry(swp_type(entry), swp_offset(entry) | SWP_MIG_DIRTY as pgoff_t) } else { entry }
}

#[cfg(all(CONFIG_MMU, not(CONFIG_MIGRATION)))]
pub fn make_readable_migration_entry(_offset: pgoff_t) -> swp_entry_t { swp_entry(0, 0) }
#[cfg(all(CONFIG_MMU, not(CONFIG_MIGRATION)))]
pub fn make_readable_exclusive_migration_entry(_offset: pgoff_t) -> swp_entry_t { swp_entry(0, 0) }
#[cfg(all(CONFIG_MMU, not(CONFIG_MIGRATION)))]
pub fn make_writable_migration_entry(_offset: pgoff_t) -> swp_entry_t { swp_entry(0, 0) }
#[cfg(all(CONFIG_MMU, not(CONFIG_MIGRATION)))]
pub fn make_migration_entry_young(entry: swp_entry_t) -> swp_entry_t { entry }
#[cfg(all(CONFIG_MMU, not(CONFIG_MIGRATION)))]
pub fn make_migration_entry_dirty(entry: swp_entry_t) -> swp_entry_t { entry }

#[cfg(CONFIG_MMU)]
pub type pte_marker = c_ulong;
#[cfg(CONFIG_MMU)]
pub const PTE_MARKER_UFFD_WP: pte_marker = 1;
#[cfg(CONFIG_MMU)]
pub const PTE_MARKER_POISONED: pte_marker = 1 << 1;
#[cfg(CONFIG_MMU)]
pub const PTE_MARKER_GUARD: pte_marker = 1 << 2;
#[cfg(CONFIG_MMU)]
pub const PTE_MARKER_MASK: pte_marker = (1 << 3) - 1;

#[cfg(CONFIG_MMU)]
pub fn make_pte_marker_entry(marker: pte_marker) -> swp_entry_t { swp_entry(SWP_PTE_MARKER, marker) }
#[cfg(CONFIG_MMU)]
pub unsafe fn make_pte_marker(marker: pte_marker) -> pte_t { swp_entry_to_pte(make_pte_marker_entry(marker)) }
#[cfg(CONFIG_MMU)]
pub fn make_poisoned_swp_entry() -> swp_entry_t { make_pte_marker_entry(PTE_MARKER_POISONED) }
#[cfg(CONFIG_MMU)]
pub fn make_guard_swp_entry() -> swp_entry_t { make_pte_marker_entry(PTE_MARKER_GUARD) }

/* CONFIG_MEMORY_FAILURE and CONFIG_ARCH_HAS_PMD_SOFTLEAVES branches retain their C semantics via cfg. */
#[cfg(all(CONFIG_MMU, CONFIG_MEMORY_FAILURE))]
pub unsafe fn make_hwpoison_entry(page: *mut page) -> swp_entry_t {
    BUG_ON(!PageLocked(page));
    swp_entry(SWP_HWPOISON, page_to_pfn(page))
}
#[cfg(all(CONFIG_MMU, CONFIG_MEMORY_FAILURE))]
pub fn is_hwpoison_entry(entry: swp_entry_t) -> c_int { (swp_type(entry) == SWP_HWPOISON) as c_int }
#[cfg(all(CONFIG_MMU, not(CONFIG_MEMORY_FAILURE)))]
pub fn make_hwpoison_entry(_page: *mut page) -> swp_entry_t { swp_entry(0, 0) }
#[cfg(all(CONFIG_MMU, not(CONFIG_MEMORY_FAILURE)))]
pub fn is_hwpoison_entry(_swp: swp_entry_t) -> c_int { 0 }

#[cfg(all(CONFIG_MMU, CONFIG_MIGRATION))]
extern "C" {
    pub fn migration_entry_wait(mm: *mut mm_struct, pmd: *mut pmd_t, address: c_ulong);
    pub fn migration_entry_wait_huge(vma: *mut vm_area_struct, addr: c_ulong, pte: *mut pte_t);
}

#[cfg(all(CONFIG_MMU, not(CONFIG_MIGRATION)))]
pub unsafe fn migration_entry_wait(_mm: *mut mm_struct, _pmd: *mut pmd_t, _address: c_ulong) {}
#[cfg(all(CONFIG_MMU, not(CONFIG_MIGRATION)))]
pub unsafe fn migration_entry_wait_huge(_vma: *mut vm_area_struct, _addr: c_ulong, _pte: *mut pte_t) {}

#[cfg(CONFIG_MMU)]
pub enum page_vma_mapped_walk {}

#[cfg(all(CONFIG_MMU, CONFIG_ARCH_HAS_PMD_SOFTLEAVES))]
extern "C" {
    pub fn set_pmd_migration_entry(pvmw: *mut page_vma_mapped_walk, page: *mut page) -> c_int;
    pub fn remove_migration_pmd(pvmw: *mut page_vma_mapped_walk, folio: *mut folio);
    pub fn pmd_migration_entry_wait(mm: *mut mm_struct, pmd: *mut pmd_t);
}

#[cfg(all(CONFIG_MMU, CONFIG_ARCH_HAS_PMD_SOFTLEAVES))]
pub unsafe fn swp_entry_to_pmd(entry: swp_entry_t) -> pmd_t {
    let arch_entry = __swp_entry(swp_type(entry), swp_offset(entry));
    __swp_entry_to_pmd(arch_entry)
}

#[cfg(all(CONFIG_MMU, not(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)))]
pub unsafe fn set_pmd_migration_entry(_pvmw: *mut page_vma_mapped_walk, _page: *mut page) -> c_int {
    BUILD_BUG();
}
#[cfg(all(CONFIG_MMU, not(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)))]
pub unsafe fn remove_migration_pmd(_pvmw: *mut page_vma_mapped_walk, _folio: *mut folio) {
    BUILD_BUG();
}
#[cfg(all(CONFIG_MMU, not(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)))]
pub unsafe fn pmd_migration_entry_wait(_m: *mut mm_struct, _p: *mut pmd_t) {}
#[cfg(all(CONFIG_MMU, not(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)))]
pub unsafe fn swp_entry_to_pmd(_entry: swp_entry_t) -> pmd_t { __pmd(0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
