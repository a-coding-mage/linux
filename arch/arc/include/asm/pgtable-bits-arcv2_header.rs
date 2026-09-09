/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/*
 * Page table flags for software walked/managed MMUv3 (ARC700) and MMUv4 (HS).
 * These correspond to the corresponding bits in the TLB.
 *
 * Build-time C configuration conditions are retained below as Rust cfg
 * conditions; the supplied dependencies define the surrounding ABI and types.
 */

#[cfg(CONFIG_ARC_CACHE_PAGES)]
pub const _PAGE_CACHEABLE: usize = 1 << 0; /* Cached (H) */
#[cfg(not(CONFIG_ARC_CACHE_PAGES))]
pub const _PAGE_CACHEABLE: usize = 0;

pub const _PAGE_EXECUTE: usize = 1 << 1; /* User Execute (H) */
pub const _PAGE_WRITE: usize = 1 << 2;   /* User Write (H) */
pub const _PAGE_READ: usize = 1 << 3;    /* User Read (H) */
pub const _PAGE_ACCESSED: usize = 1 << 4; /* Accessed (s) */
pub const _PAGE_DIRTY: usize = 1 << 5;   /* Modified (s) */
pub const _PAGE_SPECIAL: usize = 1 << 6;
pub const _PAGE_GLOBAL: usize = 1 << 8;  /* ASID agnostic (H) */
pub const _PAGE_PRESENT: usize = 1 << 9; /* PTE/TLB Valid (H) */

/* We borrow bit 5 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: usize = _PAGE_DIRTY;

#[cfg(CONFIG_ARC_MMU_V4)]
pub const _PAGE_HW_SZ: usize = 1 << 10; /* Normal/super (H) */
#[cfg(not(CONFIG_ARC_MMU_V4))]
pub const _PAGE_HW_SZ: usize = 0;

/* Defaults for every user page. */
pub const ___DEF: usize = _PAGE_PRESENT | _PAGE_CACHEABLE;

/* Set of bits not changed in pte_modify. */
pub const _PAGE_CHG_MASK: usize = PAGE_MASK_PHYS | _PAGE_ACCESSED | _PAGE_DIRTY | _PAGE_SPECIAL;

/* More abbreviated helpers. */
pub const PAGE_U_NONE: pgprot_t = __pgprot(___DEF);
pub const PAGE_U_R: pgprot_t = __pgprot(___DEF | _PAGE_READ);
pub const PAGE_U_W_R: pgprot_t = __pgprot(___DEF | _PAGE_READ | _PAGE_WRITE);
pub const PAGE_U_X_R: pgprot_t = __pgprot(___DEF | _PAGE_READ | _PAGE_EXECUTE);
pub const PAGE_U_X_W_R: pgprot_t = __pgprot(___DEF | _PAGE_READ | _PAGE_WRITE | _PAGE_EXECUTE);
pub const PAGE_KERNEL: pgprot_t = __pgprot(___DEF | _PAGE_GLOBAL | _PAGE_READ | _PAGE_WRITE | _PAGE_EXECUTE);
pub const PAGE_SHARED: pgprot_t = PAGE_U_W_R;

#[inline]
pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t {
    __pgprot(pgprot_val(prot) & !_PAGE_CACHEABLE)
}

#[inline]
pub fn pte_write(pte: pte_t) -> usize { pte_val(pte) & _PAGE_WRITE }
#[inline]
pub fn pte_dirty(pte: pte_t) -> usize { pte_val(pte) & _PAGE_DIRTY }
#[inline]
pub fn pte_young(pte: pte_t) -> usize { pte_val(pte) & _PAGE_ACCESSED }
#[inline]
pub fn pte_special(pte: pte_t) -> usize { pte_val(pte) & _PAGE_SPECIAL }

#[inline]
pub fn pte_mknotpresent(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_PRESENT; pte }
#[inline]
pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_WRITE; pte }
#[inline]
pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_WRITE; pte }
#[inline]
pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_DIRTY; pte }
#[inline]
pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_DIRTY; pte }
#[inline]
pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_ACCESSED; pte }
#[inline]
pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_ACCESSED; pte }
#[inline]
pub fn pte_mkspecial(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_SPECIAL; pte }
#[inline]
pub fn pte_mkhuge(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_HW_SZ; pte }

#[inline]
pub fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t {
    __pte((pte_val(pte) & _PAGE_CHG_MASK) | pgprot_val(newprot))
}

pub struct vm_fault;
extern "C" {
    pub fn update_mmu_cache_range(
        vmf: *mut vm_fault, vma: *mut vm_area_struct, address: usize,
        ptep: *mut pte_t, nr: u32,
    );
}

#[inline]
pub unsafe fn update_mmu_cache(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t) {
    update_mmu_cache_range(core::ptr::null_mut(), vma, addr, ptep, 1)
}

/* Encode/decode swap entries and swap PTEs. */
#[inline]
pub fn __swp_entry(type_: usize, off: usize) -> swp_entry_t {
    swp_entry_t { val: (type_ & 0x1f) | (off << 13) }
}

#[inline]
pub fn __swp_type(pte_lookalike: swp_entry_t) -> usize { pte_lookalike.val & 0x1f }
#[inline]
pub fn __swp_offset(pte_lookalike: swp_entry_t) -> usize { pte_lookalike.val >> 13 }
#[inline]
pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline]
pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { pte_t { val: x.val } }

#[inline]
pub fn pte_swp_exclusive(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_SWP_EXCLUSIVE) != 0 }
#[inline]
pub fn swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_SWP_EXCLUSIVE; pte }
#[inline]
pub fn swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_SWP_EXCLUSIVE; pte }

/* CONFIG_TRANSPARENT_HUGEPAGE includes <asm/hugepage.h> in the C header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
