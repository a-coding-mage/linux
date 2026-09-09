/* SPDX-License-Identifier: GPL-2.0 */

/* Linux PTEL encoding; configuration-specific branches mirror the C header. */

pub const _PAGE_WT: u64 = 0x001;
pub const _PAGE_HW_SHARED: u64 = 0x002;
pub const _PAGE_DIRTY: u64 = 0x004;
pub const _PAGE_CACHABLE: u64 = 0x008;
pub const _PAGE_SZ0: u64 = 0x010;
pub const _PAGE_RW: u64 = 0x020;
pub const _PAGE_USER: u64 = 0x040;
pub const _PAGE_SZ1: u64 = 0x080;
pub const _PAGE_PRESENT: u64 = 0x100;
pub const _PAGE_PROTNONE: u64 = 0x200;
pub const _PAGE_ACCESSED: u64 = 0x400;
pub const _PAGE_SPECIAL: u64 = 0x800;
pub const _PAGE_SZ_MASK: u64 = _PAGE_SZ0 | _PAGE_SZ1;
pub const _PAGE_PR_MASK: u64 = _PAGE_RW | _PAGE_USER;

pub const _PAGE_EXT_ESZ0: u64 = 0x0010;
pub const _PAGE_EXT_ESZ1: u64 = 0x0020;
pub const _PAGE_EXT_ESZ2: u64 = 0x0040;
pub const _PAGE_EXT_ESZ3: u64 = 0x0080;
pub const _PAGE_EXT_USER_EXEC: u64 = 0x0100;
pub const _PAGE_EXT_USER_WRITE: u64 = 0x0200;
pub const _PAGE_EXT_USER_READ: u64 = 0x0400;
pub const _PAGE_EXT_KERN_EXEC: u64 = 0x0800;
pub const _PAGE_EXT_KERN_WRITE: u64 = 0x1000;
pub const _PAGE_EXT_KERN_READ: u64 = 0x2000;
pub const _PAGE_EXT_WIRED: u64 = 0x4000;

#[inline]
pub const fn _PAGE_EXT(x: u64) -> u64 { x << 32 }

#[cfg(feature = "CONFIG_X2TLB")]
pub const _PAGE_PCC_MASK: u64 = 0x00000000;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_AREA5: u64 = 0x00000000;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_AREA6: u64 = 0x80000000;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_IODYN: u64 = 0x00000001;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_IO8: u64 = 0x20000000;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_IO16: u64 = 0x20000001;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_COM8: u64 = 0x40000000;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_COM16: u64 = 0x40000001;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_ATR8: u64 = 0x60000000;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_ATR16: u64 = 0x60000001;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_PCC_MASK: u64 = 0xe0000001;
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline]
pub fn copy_ptea_attributes(x: u64) -> u64 { ((x >> 28) & 0xe) | (x & 0x1) }

#[cfg(feature = "CONFIG_CPU_SH3")]
pub const _PAGE_CLEAR_FLAGS: u64 = _PAGE_PROTNONE | _PAGE_ACCESSED | _PAGE_SZ1 | _PAGE_HW_SHARED;
#[cfg(all(not(feature = "CONFIG_CPU_SH3"), feature = "CONFIG_X2TLB"))]
pub const _PAGE_CLEAR_FLAGS: u64 = _PAGE_PROTNONE | _PAGE_ACCESSED | _PAGE_PR_MASK | _PAGE_SZ_MASK;
#[cfg(all(not(feature = "CONFIG_CPU_SH3"), not(feature = "CONFIG_X2TLB")))]
pub const _PAGE_CLEAR_FLAGS: u64 = _PAGE_PROTNONE | _PAGE_ACCESSED;

/* phys_addr_mask() is supplied by the architecture implementation. */
#[inline]
pub fn _PAGE_FLAGS_HARDWARE_MASK() -> u64 { phys_addr_mask() & !_PAGE_CLEAR_FLAGS }

#[cfg(not(feature = "CONFIG_MMU"))]
pub const _PAGE_FLAGS_HARD: u64 = 0;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_X2TLB", feature = "CONFIG_PAGE_SIZE_4KB"))]
pub const _PAGE_FLAGS_HARD: u64 = _PAGE_EXT(_PAGE_EXT_ESZ0);
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_X2TLB", feature = "CONFIG_PAGE_SIZE_8KB"))]
pub const _PAGE_FLAGS_HARD: u64 = _PAGE_EXT(_PAGE_EXT_ESZ1);
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_X2TLB", feature = "CONFIG_PAGE_SIZE_64KB"))]
pub const _PAGE_FLAGS_HARD: u64 = _PAGE_EXT(_PAGE_EXT_ESZ2);
#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_X2TLB"), feature = "CONFIG_PAGE_SIZE_4KB"))]
pub const _PAGE_FLAGS_HARD: u64 = _PAGE_SZ0;
#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_X2TLB"), feature = "CONFIG_PAGE_SIZE_64KB"))]
pub const _PAGE_FLAGS_HARD: u64 = _PAGE_SZ1;

#[cfg(all(feature = "CONFIG_X2TLB", feature = "CONFIG_HUGETLB_PAGE_SIZE_64K"))]
pub const _PAGE_SZHUGE: u64 = _PAGE_EXT_ESZ2;
#[cfg(all(feature = "CONFIG_X2TLB", feature = "CONFIG_HUGETLB_PAGE_SIZE_256K"))]
pub const _PAGE_SZHUGE: u64 = _PAGE_EXT_ESZ0 | _PAGE_EXT_ESZ2;
#[cfg(all(feature = "CONFIG_X2TLB", feature = "CONFIG_HUGETLB_PAGE_SIZE_1MB"))]
pub const _PAGE_SZHUGE: u64 = _PAGE_EXT_ESZ0 | _PAGE_EXT_ESZ1 | _PAGE_EXT_ESZ2;
#[cfg(all(feature = "CONFIG_X2TLB", feature = "CONFIG_HUGETLB_PAGE_SIZE_4MB"))]
pub const _PAGE_SZHUGE: u64 = _PAGE_EXT_ESZ3;
#[cfg(all(feature = "CONFIG_X2TLB", feature = "CONFIG_HUGETLB_PAGE_SIZE_64MB"))]
pub const _PAGE_SZHUGE: u64 = _PAGE_EXT_ESZ2 | _PAGE_EXT_ESZ3;
#[cfg(feature = "CONFIG_X2TLB")]
pub const _PAGE_WIRED: u64 = _PAGE_EXT(_PAGE_EXT_WIRED);
#[cfg(all(not(feature = "CONFIG_X2TLB"), feature = "CONFIG_HUGETLB_PAGE_SIZE_64K"))]
pub const _PAGE_SZHUGE: u64 = _PAGE_SZ1;
#[cfg(all(not(feature = "CONFIG_X2TLB"), feature = "CONFIG_HUGETLB_PAGE_SIZE_1MB"))]
pub const _PAGE_SZHUGE: u64 = _PAGE_SZ0 | _PAGE_SZ1;
#[cfg(not(feature = "CONFIG_X2TLB"))]
pub const _PAGE_WIRED: u64 = 0;

/* Stub out _PAGE_SZHUGE when no configuration supplied a definition. */
#[cfg(not(any(feature = "CONFIG_HUGETLB_PAGE_SIZE_64K", feature = "CONFIG_HUGETLB_PAGE_SIZE_256K", feature = "CONFIG_HUGETLB_PAGE_SIZE_1MB", feature = "CONFIG_HUGETLB_PAGE_SIZE_4MB", feature = "CONFIG_HUGETLB_PAGE_SIZE_64MB")))]
pub const _PAGE_SZHUGE: u64 = _PAGE_FLAGS_HARD;

pub const _PAGE_CHG_MASK: u64 = PTE_MASK | _PAGE_ACCESSED | _PAGE_CACHABLE | _PAGE_DIRTY | _PAGE_SPECIAL;

/* __ASSEMBLER__-excluded declarations are represented as Rust items. */
#[inline]
pub unsafe fn set_pte(ptep: *mut pte_t, pte: pte_t) {
    #[cfg(feature = "CONFIG_X2TLB")]
    {
        (*ptep).pte_high = pte.pte_high;
        smp_wmb();
        (*ptep).pte_low = pte.pte_low;
    }
    #[cfg(not(feature = "CONFIG_X2TLB"))]
    { *ptep = pte; }
}

#[inline]
pub unsafe fn set_pmd(pmdptr: *mut pmd_t, pmdval: pmd_t) { *pmdptr = pmdval; }
pub const PFN_PTE_SHIFT: u64 = PAGE_SHIFT;
#[inline] pub fn pfn_pte(pfn: u64, prot: pgprot_t) -> pte_t { __pte((pfn << PAGE_SHIFT) | pgprot_val(prot)) }
#[inline] pub fn pfn_pmd(pfn: u64, prot: pgprot_t) -> pmd_t { __pmd((pfn << PAGE_SHIFT) | pgprot_val(prot)) }
#[inline] pub fn pte_none(x: pte_t) -> bool { pte_val(x) == 0 }
#[inline] pub fn pte_present(x: pte_t) -> bool { (x.pte_low & (_PAGE_PRESENT | _PAGE_PROTNONE)) != 0 }
#[inline] pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) { set_pte(ptep, __pte(0)); }
#[inline] pub fn pmd_none(x: pmd_t) -> bool { pmd_val(x) == 0 }
#[inline] pub fn pmd_present(x: pmd_t) -> bool { pmd_val(x) != 0 }
#[inline] pub unsafe fn pmd_clear(xp: *mut pmd_t) { set_pmd(xp, __pmd(0)); }
#[inline] pub fn pmd_bad(x: pmd_t) -> u64 { pmd_val(x) & !PAGE_MASK }
#[inline] pub fn pages_to_mb(x: u64) -> u64 { x >> (20 - PAGE_SHIFT) }
#[inline] pub fn pte_page(x: pte_t) -> *mut page { pfn_to_page(pte_pfn(x)) }
#[inline] pub fn pte_not_present(pte: pte_t) -> bool { (pte.pte_low & _PAGE_PRESENT) == 0 }
#[inline] pub fn pte_dirty(pte: pte_t) -> bool { (pte.pte_low & _PAGE_DIRTY) != 0 }
#[inline] pub fn pte_young(pte: pte_t) -> bool { (pte.pte_low & _PAGE_ACCESSED) != 0 }
#[inline] pub fn pte_special(pte: pte_t) -> bool { (pte.pte_low & _PAGE_SPECIAL) != 0 }
#[inline] pub fn pte_write(pte: pte_t) -> bool {
    #[cfg(feature = "CONFIG_X2TLB")] { (pte.pte_high & (_PAGE_EXT_USER_WRITE | _PAGE_EXT_KERN_WRITE)) != 0 }
    #[cfg(not(feature = "CONFIG_X2TLB"))] { (pte.pte_low & _PAGE_RW) != 0 }
}

#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte.pte_high &= !(_PAGE_EXT_USER_WRITE | _PAGE_EXT_KERN_WRITE); pte }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte.pte_low &= !_PAGE_RW; pte }
#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte.pte_high |= _PAGE_EXT_USER_WRITE | _PAGE_EXT_KERN_WRITE; pte }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_RW; pte }
#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn pte_mkhuge(mut pte: pte_t) -> pte_t { pte.pte_high |= _PAGE_SZHUGE; pte }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn pte_mkhuge(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_SZHUGE; pte }
#[inline] pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte.pte_low &= !_PAGE_DIRTY; pte }
#[inline] pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_DIRTY; pte }
#[inline] pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte.pte_low &= !_PAGE_ACCESSED; pte }
#[inline] pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_ACCESSED; pte }
#[inline] pub fn pte_mkspecial(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_SPECIAL; pte }

#[inline] pub fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t { __pgprot(pgprot_val(prot) & !_PAGE_CACHABLE) }
#[inline] pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t { pgprot_writecombine(prot) }
#[inline]
pub fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte.pte_low &= _PAGE_CHG_MASK;
    pte.pte_low |= pgprot_val(newprot);
    #[cfg(feature = "CONFIG_X2TLB")]
    { pte.pte_high |= pgprot_val(newprot) >> 32; }
    pte
}
#[inline] pub fn pmd_page_vaddr(pmd: pmd_t) -> u64 { pmd_val(pmd) }
#[inline] pub fn pmd_pfn(pmd: pmd_t) -> u64 { __pa(pmd_val(pmd)) >> PAGE_SHIFT }
#[inline] pub fn pmd_page(pmd: pmd_t) -> *mut page { virt_to_page(pmd_val(pmd)) }

/* Swap entry encoding. */
#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn __swp_type(x: swp_entry_t) -> u64 { x.val & 0x1f }
#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn __swp_offset(x: swp_entry_t) -> u64 { x.val >> 5 }
#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn __swp_entry(ty: u64, offset: u64) -> swp_entry_t { swp_entry_t { val: (ty & 0x1f) | (offset << 5) } }
#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte.pte_high } }
#[cfg(feature = "CONFIG_X2TLB")]
#[inline] pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { pte_t { pte_high: 0, pte_low: x.val } }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn __swp_type(x: swp_entry_t) -> u64 { x.val & 0x1f }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn __swp_offset(x: swp_entry_t) -> u64 { x.val >> 10 }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn __swp_entry(ty: u64, offset: u64) -> swp_entry_t { swp_entry_t { val: (ty & 0x1f) | (offset << 10) } }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) >> 1 } }
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline] pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val << 1) }

pub const _PAGE_SWP_EXCLUSIVE: u64 = _PAGE_USER;
#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { (pte.pte_low & _PAGE_SWP_EXCLUSIVE) != 0 }
#[inline] pub fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_SWP_EXCLUSIVE; pte }
#[inline] pub fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte.pte_low &= !_PAGE_SWP_EXCLUSIVE; pte }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
