/* SPDX-License-Identifier: GPL-2.0 */

pub const H_PTE_INDEX_SIZE: usize = 9; // size: 8B << 9 = 4KB, maps: 2^9 x 4KB = 2MB
pub const H_PMD_INDEX_SIZE: usize = 7; // size: 8B << 7 = 1KB, maps: 2^7 x 2MB = 256MB
pub const H_PUD_INDEX_SIZE: usize = 9; // size: 8B << 9 = 4KB, maps: 2^9 x 256MB = 128GB
pub const H_PGD_INDEX_SIZE: usize = 9; // size: 8B << 9 = 4KB, maps: 2^9 x 128GB = 64TB

/* Each context is 512TB. On 4k we restrict our max TASK size to 64TB. */
pub const MAX_EA_BITS_PER_CONTEXT: usize = 46;

pub const REGION_SHIFT: usize = 40;
pub const H_KERN_MAP_SIZE: usize = 1usize << REGION_SHIFT;
pub const H_MAX_PHYSMEM_BITS: usize = 46;
pub const H_KERN_VIRT_START: u64 = 0xc0003d0000000000;

/* These items are present only for non-assembler consumers of the C header. */
#[cfg(not(asm))]
pub const H_PTE_TABLE_SIZE: usize = core::mem::size_of::<pte_t>() << H_PTE_INDEX_SIZE;
#[cfg(not(asm))]
pub const H_PMD_TABLE_SIZE: usize = core::mem::size_of::<pmd_t>() << H_PMD_INDEX_SIZE;
#[cfg(not(asm))]
pub const H_PUD_TABLE_SIZE: usize = core::mem::size_of::<pud_t>() << H_PUD_INDEX_SIZE;
#[cfg(not(asm))]
pub const H_PGD_TABLE_SIZE: usize = core::mem::size_of::<pgd_t>() << H_PGD_INDEX_SIZE;

#[cfg(not(asm))]
pub const H_PAGE_F_GIX_SHIFT: usize = _PAGE_PA_MAX;
#[cfg(not(asm))]
pub const H_PAGE_F_SECOND: u64 = _RPAGE_PKEY_BIT0;
#[cfg(not(asm))]
pub const H_PAGE_F_GIX: u64 = _RPAGE_RPN43 | _RPAGE_RPN42 | _RPAGE_RPN41;
#[cfg(not(asm))]
pub const H_PAGE_BUSY: u64 = _RPAGE_RSV1;
#[cfg(not(asm))]
pub const H_PAGE_HASHPTE: u64 = _RPAGE_PKEY_BIT4;
#[cfg(not(asm))]
pub const _PAGE_HPTEFLAGS: u64 = H_PAGE_BUSY | H_PAGE_HASHPTE | H_PAGE_F_SECOND | H_PAGE_F_GIX;

#[cfg(not(asm))]
pub const H_PAGE_4K_PFN: u64 = 0;
#[cfg(not(asm))]
pub const H_PAGE_THP_HUGE: u64 = 0;
#[cfg(not(asm))]
pub const H_PAGE_COMBO: u64 = 0;

#[cfg(not(asm))]
pub const H_PTE_FRAG_SIZE_SHIFT: usize = H_PTE_INDEX_SIZE + 3;
#[cfg(not(asm))]
pub const H_PTE_FRAG_NR: usize = PAGE_SIZE >> H_PTE_FRAG_SIZE_SHIFT;
#[cfg(not(asm))]
pub const H_PMD_FRAG_SIZE_SHIFT: usize = H_PMD_INDEX_SIZE + 3;
#[cfg(not(asm))]
pub const H_PMD_FRAG_NR: usize = PAGE_SIZE >> H_PMD_FRAG_SIZE_SHIFT;

#[cfg(not(asm))]
pub const H_PTE_PKEY_BIT4: u64 = 0;
#[cfg(not(asm))]
pub const H_PTE_PKEY_BIT3: u64 = 0;
#[cfg(not(asm))]
pub const H_PTE_PKEY_BIT2: u64 = _RPAGE_PKEY_BIT3;
#[cfg(not(asm))]
pub const H_PTE_PKEY_BIT1: u64 = _RPAGE_PKEY_BIT2;
#[cfg(not(asm))]
pub const H_PTE_PKEY_BIT0: u64 = _RPAGE_PKEY_BIT1;

#[inline]
pub unsafe fn remap_4k_pfn(vma: *mut vm_area_struct, addr: u64, pfn: u64, prot: u64) -> i32 {
    remap_pfn_range(vma, addr, pfn, PAGE_SIZE, prot)
}

#[inline]
pub unsafe fn __real_pte(pte: pte_t, _ptep: *mut pte_t, _offset: i32) -> real_pte_t {
    real_pte_t { pte }
}

#[inline]
pub unsafe fn __rpte_to_pte(r: real_pte_t) -> pte_t { r.pte }

#[inline]
pub unsafe fn __rpte_to_hidx(rpte: real_pte_t, _index: u64) -> u64 {
    pte_val(__rpte_to_pte(rpte)) >> H_PAGE_F_GIX_SHIFT
}

#[inline]
pub unsafe fn pte_set_hidx(_ptep: *mut pte_t, _rpte: real_pte_t,
                           _subpg_index: u32, hidx: u64, _offset: i32) -> u64 {
    (hidx << H_PAGE_F_GIX_SHIFT) & (H_PAGE_F_SECOND | H_PAGE_F_GIX)
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline]
pub unsafe fn get_hpte_slot_array(_pmdp: *mut pmd_t) -> *mut i8 { BUG(); core::ptr::null_mut() }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline]
pub unsafe fn hpte_valid(_hpte_slot_array: *mut u8, _index: i32) -> u32 { BUG(); 0 }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline]
pub unsafe fn hpte_hash_index(_hpte_slot_array: *mut u8, _index: i32) -> u32 { BUG(); 0 }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline]
pub unsafe fn mark_hpte_slot_valid(_hpte_slot_array: *mut u8, _index: u32, _hidx: u32) { BUG(); }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline]
pub unsafe fn hash__pmd_trans_huge(_pmd: pmd_t) -> i32 { 0 }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline]
pub unsafe fn hash__pmd_mkhuge(pmd: pmd_t) -> pmd_t { BUG(); pmd }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
extern "C" {
    pub fn hash__pmd_hugepage_update(mm: *mut mm_struct, addr: u64, pmdp: *mut pmd_t,
                                     clr: u64, set: u64) -> u64;
    pub fn hash__pmdp_collapse_flush(vma: *mut vm_area_struct, address: u64,
                                     pmdp: *mut pmd_t) -> pmd_t;
    pub fn hash__pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t,
                                           pgtable: pgtable_t);
    pub fn hash__pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
    pub fn hash__pmdp_huge_get_and_clear(mm: *mut mm_struct, addr: u64,
                                        pmdp: *mut pmd_t) -> pmd_t;
    pub fn hash__has_transparent_hugepage() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
