/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Describes operations that can be performed on software-defined page table
 * leaf entries. These are abstracted from the hardware page table entries
 * themselves by the softleaf_t type, see mm_types.h.
 */

// Dependencies supplied by linux/mm_types.h, linux/swapops.h, and linux/swap.h
// are intentionally referenced but not implemented here.

// Temporary until swp_entry_t eliminated.
pub const LEAF_TYPE_SHIFT: u32 = SWP_TYPE_SHIFT;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum softleaf_type {
    /* Fundamental types. */
    SOFTLEAF_NONE,
    SOFTLEAF_SWAP,
    /* Migration types. */
    SOFTLEAF_MIGRATION_READ,
    SOFTLEAF_MIGRATION_READ_EXCLUSIVE,
    SOFTLEAF_MIGRATION_WRITE,
    /* Device types. */
    SOFTLEAF_DEVICE_PRIVATE_READ,
    SOFTLEAF_DEVICE_PRIVATE_WRITE,
    SOFTLEAF_DEVICE_EXCLUSIVE,
    /* H/W posion types. */
    SOFTLEAF_HWPOISON,
    /* Marker types. */
    SOFTLEAF_MARKER,
}

#[inline]
pub unsafe fn softleaf_mk_none() -> softleaf_t {
    core::mem::zeroed()
}

#[inline]
pub unsafe fn softleaf_from_pte(mut pte: pte_t) -> softleaf_t {
    let arch_entry: softleaf_t;
    if pte_present(pte) || pte_none(pte) { return softleaf_mk_none(); }
    pte = pte_swp_clear_flags(pte);
    arch_entry = __pte_to_swp_entry(pte);
    swp_entry(__swp_type(arch_entry), __swp_offset(arch_entry))
}

#[inline]
pub unsafe fn softleaf_to_pte(entry: softleaf_t) -> pte_t { swp_entry_to_pte(entry) }

#[cfg(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)]
#[inline]
pub unsafe fn softleaf_from_pmd(mut pmd: pmd_t) -> softleaf_t {
    let arch_entry: softleaf_t;
    if pmd_present(pmd) || pmd_none(pmd) { return softleaf_mk_none(); }
    if pmd_swp_soft_dirty(pmd) { pmd = pmd_swp_clear_soft_dirty(pmd); }
    if pmd_swp_uffd(pmd) { pmd = pmd_swp_clear_uffd(pmd); }
    arch_entry = __pmd_to_swp_entry(pmd);
    swp_entry(__swp_type(arch_entry), __swp_offset(arch_entry))
}

#[cfg(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)]
#[inline]
pub unsafe fn softleaf_to_pmd(entry: softleaf_t) -> pmd_t { swp_entry_to_pmd(entry) }

#[cfg(not(CONFIG_ARCH_HAS_PMD_SOFTLEAVES))]
#[inline]
pub unsafe fn softleaf_from_pmd(_pmd: pmd_t) -> softleaf_t { softleaf_mk_none() }

#[cfg(not(CONFIG_ARCH_HAS_PMD_SOFTLEAVES))]
#[inline]
pub unsafe fn softleaf_to_pmd(_entry: softleaf_t) -> pmd_t { __pmd(0) }

#[inline]
pub fn softleaf_is_none(entry: softleaf_t) -> bool { entry.val == 0 }

#[inline]
pub unsafe fn softleaf_type(entry: softleaf_t) -> softleaf_type {
    if softleaf_is_none(entry) { return softleaf_type::SOFTLEAF_NONE; }
    let type_num = entry.val >> LEAF_TYPE_SHIFT;
    if type_num < MAX_SWAPFILES { return softleaf_type::SOFTLEAF_SWAP; }
    match type_num {
        #[cfg(CONFIG_MIGRATION)]
        SWP_MIGRATION_READ => softleaf_type::SOFTLEAF_MIGRATION_READ,
        #[cfg(CONFIG_MIGRATION)]
        SWP_MIGRATION_READ_EXCLUSIVE => softleaf_type::SOFTLEAF_MIGRATION_READ_EXCLUSIVE,
        #[cfg(CONFIG_MIGRATION)]
        SWP_MIGRATION_WRITE => softleaf_type::SOFTLEAF_MIGRATION_WRITE,
        #[cfg(CONFIG_DEVICE_PRIVATE)]
        SWP_DEVICE_WRITE => softleaf_type::SOFTLEAF_DEVICE_PRIVATE_WRITE,
        #[cfg(CONFIG_DEVICE_PRIVATE)]
        SWP_DEVICE_READ => softleaf_type::SOFTLEAF_DEVICE_PRIVATE_READ,
        #[cfg(CONFIG_DEVICE_PRIVATE)]
        SWP_DEVICE_EXCLUSIVE => softleaf_type::SOFTLEAF_DEVICE_EXCLUSIVE,
        #[cfg(CONFIG_MEMORY_FAILURE)]
        SWP_HWPOISON => softleaf_type::SOFTLEAF_HWPOISON,
        SWP_PTE_MARKER => softleaf_type::SOFTLEAF_MARKER,
        _ => { VM_WARN_ON_ONCE(1); softleaf_type::SOFTLEAF_NONE }
    }
}

#[inline] pub unsafe fn softleaf_is_swap(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_SWAP }
#[inline] pub unsafe fn softleaf_is_migration_write(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_MIGRATION_WRITE }
#[inline] pub unsafe fn softleaf_is_migration_read(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_MIGRATION_READ }
#[inline] pub unsafe fn softleaf_is_migration_read_exclusive(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_MIGRATION_READ_EXCLUSIVE }
#[inline] pub unsafe fn softleaf_is_migration(e: softleaf_t) -> bool { matches!(softleaf_type(e), softleaf_type::SOFTLEAF_MIGRATION_READ | softleaf_type::SOFTLEAF_MIGRATION_READ_EXCLUSIVE | softleaf_type::SOFTLEAF_MIGRATION_WRITE) }
#[inline] pub unsafe fn softleaf_is_device_private_write(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_DEVICE_PRIVATE_WRITE }
#[inline] pub unsafe fn softleaf_is_device_private(e: softleaf_t) -> bool { matches!(softleaf_type(e), softleaf_type::SOFTLEAF_DEVICE_PRIVATE_WRITE | softleaf_type::SOFTLEAF_DEVICE_PRIVATE_READ) }
#[inline] pub unsafe fn softleaf_is_device_exclusive(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_DEVICE_EXCLUSIVE }
#[inline] pub unsafe fn softleaf_is_hwpoison(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_HWPOISON }
#[inline] pub unsafe fn softleaf_is_marker(e: softleaf_t) -> bool { softleaf_type(e) == softleaf_type::SOFTLEAF_MARKER }

#[inline]
pub unsafe fn softleaf_to_marker(entry: softleaf_t) -> pte_marker {
    VM_WARN_ON_ONCE(!softleaf_is_marker(entry));
    swp_offset(entry) & PTE_MARKER_MASK
}

#[inline]
pub unsafe fn softleaf_has_pfn(entry: softleaf_t) -> bool {
    BUILD_BUG_ON(SWP_TYPE_SHIFT < SWP_PFN_BITS);
    softleaf_is_migration(entry) || softleaf_is_device_private(entry) || softleaf_is_device_exclusive(entry) || softleaf_is_hwpoison(entry)
}

#[inline]
pub unsafe fn softleaf_to_pfn(entry: softleaf_t) -> c_ulong {
    VM_WARN_ON_ONCE(!softleaf_has_pfn(entry));
    swp_offset(entry) & SWP_PFN_MASK
}

#[inline]
pub unsafe fn softleaf_migration_sync(_entry: softleaf_t, folio: *mut folio) {
    smp_rmb();
    VM_WARN_ON_ONCE(!folio_test_locked(folio));
}

#[inline]
pub unsafe fn softleaf_to_page(entry: softleaf_t) -> *mut page {
    let page = pfn_to_page(softleaf_to_pfn(entry));
    VM_WARN_ON_ONCE(!softleaf_has_pfn(entry));
    if softleaf_is_migration(entry) { softleaf_migration_sync(entry, page_folio(page)); }
    page
}

#[inline]
pub unsafe fn softleaf_to_folio(entry: softleaf_t) -> *mut folio {
    let folio = pfn_folio(softleaf_to_pfn(entry));
    VM_WARN_ON_ONCE(!softleaf_has_pfn(entry));
    if softleaf_is_migration(entry) { softleaf_migration_sync(entry, folio); }
    folio
}

#[inline] pub unsafe fn softleaf_is_poison_marker(e: softleaf_t) -> bool { softleaf_is_marker(e) && softleaf_to_marker(e) & PTE_MARKER_POISONED != 0 }
#[inline] pub unsafe fn softleaf_is_guard_marker(e: softleaf_t) -> bool { softleaf_is_marker(e) && softleaf_to_marker(e) & PTE_MARKER_GUARD != 0 }
#[inline] pub unsafe fn softleaf_is_uffd_wp_marker(e: softleaf_t) -> bool { softleaf_is_marker(e) && softleaf_to_marker(e) & PTE_MARKER_UFFD_WP != 0 }

#[cfg(CONFIG_MIGRATION)]
#[inline] pub unsafe fn softleaf_is_migration_young(e: softleaf_t) -> bool { VM_WARN_ON_ONCE(!softleaf_is_migration(e)); if migration_entry_supports_ad() { swp_offset(e) & SWP_MIG_YOUNG != 0 } else { false } }
#[cfg(not(CONFIG_MIGRATION))]
#[inline] pub unsafe fn softleaf_is_migration_young(_e: softleaf_t) -> bool { false }
#[cfg(CONFIG_MIGRATION)]
#[inline] pub unsafe fn softleaf_is_migration_dirty(e: softleaf_t) -> bool { VM_WARN_ON_ONCE(!softleaf_is_migration(e)); if migration_entry_supports_ad() { swp_offset(e) & SWP_MIG_DIRTY != 0 } else { false } }
#[cfg(not(CONFIG_MIGRATION))]
#[inline] pub unsafe fn softleaf_is_migration_dirty(_e: softleaf_t) -> bool { false }

#[inline] pub unsafe fn pte_is_marker(pte: pte_t) -> bool { softleaf_is_marker(softleaf_from_pte(pte)) }
#[inline] pub unsafe fn pte_is_uffd_wp_marker(pte: pte_t) -> bool { softleaf_is_uffd_wp_marker(softleaf_from_pte(pte)) }
#[inline] pub unsafe fn pte_is_uffd_marker(pte: pte_t) -> bool { let e = softleaf_from_pte(pte); softleaf_is_marker(e) && (softleaf_is_uffd_wp_marker(e) || softleaf_is_poison_marker(e)) }

#[cfg(all(CONFIG_ZONE_DEVICE, CONFIG_ARCH_HAS_PMD_SOFTLEAVES))]
#[inline] pub unsafe fn pmd_is_device_private_entry(pmd: pmd_t) -> bool { softleaf_is_device_private(softleaf_from_pmd(pmd)) }
#[cfg(not(all(CONFIG_ZONE_DEVICE, CONFIG_ARCH_HAS_PMD_SOFTLEAVES)))]
#[inline] pub unsafe fn pmd_is_device_private_entry(_pmd: pmd_t) -> bool { false }

#[inline] pub unsafe fn pmd_is_migration_entry(pmd: pmd_t) -> bool { softleaf_is_migration(softleaf_from_pmd(pmd)) }
#[inline] pub unsafe fn softleaf_is_valid_pmd_entry(e: softleaf_t) -> bool { softleaf_is_device_private(e) || softleaf_is_migration(e) }
#[inline] pub unsafe fn pmd_is_valid_softleaf(pmd: pmd_t) -> bool { softleaf_is_valid_pmd_entry(softleaf_from_pmd(pmd)) }
#[inline] pub unsafe fn pmd_to_softleaf_folio(pmd: pmd_t) -> *mut folio {
    let e = softleaf_from_pmd(pmd);
    if !softleaf_is_valid_pmd_entry(e) { VM_WARN_ON_ONCE(true); return core::ptr::null_mut(); }
    softleaf_to_folio(e)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
