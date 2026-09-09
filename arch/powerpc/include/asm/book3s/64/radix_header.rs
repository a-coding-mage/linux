/* SPDX-License-Identifier: GPL-2.0 */
// Translated from radix.h. C preprocessor configuration and included
// architecture definitions are supplied by the surrounding translation unit.

pub const RADIX_PTE_NONE_MASK: usize = _PAGE_DIRTY | _PAGE_ACCESSED;
pub const RADIX_PMD_VAL_BITS: usize = 0x8000_0000_0000_0000usize | RADIX_PTE_INDEX_SIZE;
pub const RADIX_PUD_VAL_BITS: usize = 0x8000_0000_0000_0000usize | RADIX_PMD_INDEX_SIZE;
pub const RADIX_PGD_VAL_BITS: usize = 0x8000_0000_0000_0000usize | RADIX_PUD_INDEX_SIZE;
pub const RADIX_PMD_BAD_BITS: usize = 0x6000_0000_0000_00e0usize;
pub const RADIX_PUD_BAD_BITS: usize = 0x6000_0000_0000_00e0usize;
pub const RADIX_P4D_BAD_BITS: usize = 0x6000_0000_0000_00e0usize;
pub const RADIX_PMD_SHIFT: usize = PAGE_SHIFT + RADIX_PTE_INDEX_SIZE;
pub const RADIX_PUD_SHIFT: usize = RADIX_PMD_SHIFT + RADIX_PMD_INDEX_SIZE;
pub const RADIX_PGD_SHIFT: usize = RADIX_PUD_SHIFT + RADIX_PUD_INDEX_SIZE;
pub const R_PTRS_PER_PTE: usize = 1usize << RADIX_PTE_INDEX_SIZE;
pub const R_PTRS_PER_PMD: usize = 1usize << RADIX_PMD_INDEX_SIZE;
pub const R_PTRS_PER_PUD: usize = 1usize << RADIX_PUD_INDEX_SIZE;
pub const RADIX_PGTABLE_EADDR_SIZE: usize = RADIX_PTE_INDEX_SIZE + RADIX_PMD_INDEX_SIZE + RADIX_PUD_INDEX_SIZE + RADIX_PGD_INDEX_SIZE + PAGE_SHIFT;
pub const RADIX_PGTABLE_RANGE: usize = 1usize << RADIX_PGTABLE_EADDR_SIZE;

#[cfg(all(CONFIG_SPARSEMEM_VMEMMAP, CONFIG_SPARSEMEM_EXTREME))]
pub const R_MAX_PHYSMEM_BITS: usize = 51;
#[cfg(not(all(CONFIG_SPARSEMEM_VMEMMAP, CONFIG_SPARSEMEM_EXTREME)))]
pub const R_MAX_PHYSMEM_BITS: usize = 46;

pub const RADIX_KERN_VIRT_START: usize = 0xc008_0000_0000_0000usize;
pub const RADIX_KERN_MAP_SIZE: usize = 1usize << 49;
pub const RADIX_VMALLOC_START: usize = RADIX_KERN_VIRT_START;
pub const RADIX_VMALLOC_SIZE: usize = RADIX_KERN_MAP_SIZE;
pub const RADIX_VMALLOC_END: usize = RADIX_VMALLOC_START + RADIX_VMALLOC_SIZE;
pub const RADIX_KERN_IO_START: usize = RADIX_VMALLOC_END;
pub const RADIX_KERN_IO_SIZE: usize = RADIX_KERN_MAP_SIZE;
pub const RADIX_KERN_IO_END: usize = RADIX_KERN_IO_START + RADIX_KERN_IO_SIZE;
pub const RADIX_VMEMMAP_START: usize = RADIX_KERN_IO_END;
pub const RADIX_VMEMMAP_SIZE: usize = RADIX_KERN_MAP_SIZE;
pub const RADIX_VMEMMAP_END: usize = RADIX_VMEMMAP_START + RADIX_VMEMMAP_SIZE;

extern "C" {
    #[cfg(CONFIG_STRICT_KERNEL_RWX)] pub fn radix__mark_rodata_ro();
    #[cfg(CONFIG_STRICT_KERNEL_RWX)] pub fn radix__mark_initmem_nx();
    pub fn radix__ptep_set_access_flags(vma: *mut vm_area_struct, ptep: *mut pte_t, entry: pte_t, address: usize, psize: i32);
    pub fn radix__ptep_modify_prot_commit(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t, old_pte: pte_t, pte: pte_t);
    pub fn radix__vmemmap_create_mapping(start: usize, page_size: usize, phys: usize) -> i32;
    pub fn radix__vmemmap_populate(start: usize, end: usize, node: i32, altmap: *mut vmem_altmap) -> i32;
    pub fn radix__vmemmap_free(start: usize, end: usize, altmap: *mut vmem_altmap);
    pub fn radix__vmemmap_remove_mapping(start: usize, page_size: usize);
    pub fn radix__map_kernel_page(ea: usize, pa: usize, flags: pgprot_t, psz: u32) -> i32;
}

#[inline]
pub unsafe fn __radix_pte_update(ptep: *mut pte_t, clr: usize, set: usize) -> usize {
    // The original uses a PowerPC ldarx/stdcx. retry loop with big-endian PTEs.
    let mut old_be: u64;
    let mut tmp_be: u64;
    core::arch::asm!(
        "1: ldarx {old}, 0, {ptr}\n andc {tmp}, {old}, {clr}\n or {tmp}, {tmp}, {set}\n stdcx. {tmp}, 0, {ptr}\n bne- 1b",
        old = out(reg) old_be, tmp = out(reg) tmp_be, ptr = in(reg) ptep,
        clr = in(reg) clr.to_be(), set = in(reg) set.to_be(), options(nostack)
    );
    usize::from_be(old_be)
}

#[inline]
pub unsafe fn radix__pte_update(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, clr: usize, set: usize, huge: i32) -> usize {
    let old_pte = __radix_pte_update(ptep, clr, set);
    if huge == 0 { assert_pte_locked(mm, addr); }
    old_pte
}

#[inline]
pub unsafe fn radix__ptep_get_and_clear_full(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, full: i32) -> pte_t {
    let old_pte = if full != 0 { let old = pte_val(*ptep); *ptep = __pte(0); old } else { radix__pte_update(mm, addr, ptep, !0usize, 0, 0) };
    __pte(old_pte)
}

#[inline] pub fn radix__pte_same(a: pte_t, b: pte_t) -> i32 { (pte_raw(a) ^ pte_raw(b) == 0) as i32 }
#[inline] pub fn radix__pte_none(pte: pte_t) -> i32 { ((pte_val(pte) & !RADIX_PTE_NONE_MASK) == 0) as i32 }
#[inline] pub fn radix__pmd_bad(pmd: pmd_t) -> i32 { ((pmd_val(pmd) & RADIX_PMD_BAD_BITS) != 0) as i32 }
#[inline] pub fn radix__pmd_same(a: pmd_t, b: pmd_t) -> i32 { (pmd_raw(a) ^ pmd_raw(b) == 0) as i32 }
#[inline] pub fn radix__pud_bad(pud: pud_t) -> i32 { ((pud_val(pud) & RADIX_PUD_BAD_BITS) != 0) as i32 }
#[inline] pub fn radix__pud_same(a: pud_t, b: pud_t) -> i32 { (pud_raw(a) ^ pud_raw(b) == 0) as i32 }
#[inline] pub fn radix__p4d_bad(p4d: p4d_t) -> i32 { ((p4d_val(p4d) & RADIX_P4D_BAD_BITS) != 0) as i32 }

#[inline]
pub fn radix__get_tree_size() -> usize { (0x5usize << 5) | (0x2usize << 61) }

#[inline]
pub fn radix__set_pte_at(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t, pte: pte_t, _percpu: i32) {
    // The architecture's optional ptesync is intentionally omitted, as in the C inline function.
    unsafe { *ptep = pte; }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn radix__pmd_trans_huge(pmd: pmd_t) -> i32 { ((pmd_val(pmd) & _PAGE_PTE) == _PAGE_PTE) as i32 }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn radix__pmd_mkhuge(pmd: pmd_t) -> pmd_t { __pmd(pmd_val(pmd) | _PAGE_PTE) }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn radix__pud_trans_huge(pud: pud_t) -> i32 { ((pud_val(pud) & _PAGE_PTE) == _PAGE_PTE) as i32 }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[inline] pub fn radix__pud_mkhuge(pud: pud_t) -> pud_t { __pud(pud_val(pud) | _PAGE_PTE) }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
extern "C" {
    pub fn radix__pmd_hugepage_update(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, clr: usize, set: usize) -> usize;
    pub fn radix__pud_hugepage_update(mm: *mut mm_struct, addr: usize, pudp: *mut pud_t, clr: usize, set: usize) -> usize;
    pub fn radix__pmdp_collapse_flush(vma: *mut vm_area_struct, address: usize, pmdp: *mut pmd_t) -> pmd_t;
    pub fn radix__pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t, pgtable: pgtable_t);
    pub fn radix__pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
    pub fn radix__pmdp_huge_get_and_clear(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t) -> pmd_t;
    pub fn radix__pudp_huge_get_and_clear(mm: *mut mm_struct, addr: usize, pudp: *mut pud_t) -> pud_t;
}

#[repr(C)] pub struct vmem_altmap { _private: [u8; 0] }
#[repr(C)] pub struct dev_pagemap { _private: [u8; 0] }

#[cfg(CONFIG_MEMORY_HOTPLUG)]
extern "C" {
    pub fn radix__create_section_mapping(start: usize, end: usize, nid: i32, prot: pgprot_t) -> i32;
    pub fn radix__remove_section_mapping(start: usize, end: usize) -> i32;
}

#[cfg(CONFIG_ARCH_WANT_OPTIMIZE_DAX_VMEMMAP)]
extern "C" { pub fn vmemmap_can_optimize(altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap) -> bool; }
extern "C" { pub fn vmemmap_populate_compound_pages(start_pfn: usize, start: usize, end: usize, node: i32, pgmap: *mut dev_pagemap) -> i32; }

// External types and helpers referenced above are provided by other translated headers.
extern "C" { fn assert_pte_locked(mm: *mut mm_struct, addr: usize); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
