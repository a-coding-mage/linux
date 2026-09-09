/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */
/* CONFIG_PPC_64K_PAGES selects hash-64k.h; otherwise hash-4k.h. */

pub const H_PTE_NONE_MASK: usize = _PAGE_HPTEFLAGS;

pub const H_PTRS_PER_PTE: usize = 1usize << H_PTE_INDEX_SIZE;
pub const H_PTRS_PER_PMD: usize = 1usize << H_PMD_INDEX_SIZE;
pub const H_PTRS_PER_PUD: usize = 1usize << H_PUD_INDEX_SIZE;

pub const HASH_PMD_VAL_BITS: u64 = 0x8000_0000_0000_0000;
pub const HASH_PUD_VAL_BITS: u64 = 0x8000_0000_0000_0000;
pub const HASH_PGD_VAL_BITS: u64 = 0x8000_0000_0000_0000;

pub const H_PGTABLE_EADDR_SIZE: usize =
    H_PTE_INDEX_SIZE + H_PMD_INDEX_SIZE + H_PUD_INDEX_SIZE + H_PGD_INDEX_SIZE + PAGE_SHIFT;
pub const H_PGTABLE_RANGE: usize = 1usize << H_PGTABLE_EADDR_SIZE;
pub const EA_MASK: usize = !(0xcusize << 60);

/* CONFIG_HUGETLB_PAGE && CONFIG_PPC_64K_PAGES adds one PUD cache index bit. */
#[cfg(all(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_PPC_64K_PAGES"))]
pub const H_PUD_CACHE_INDEX: usize = H_PUD_INDEX_SIZE + 1;
#[cfg(not(all(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_PPC_64K_PAGES")))]
pub const H_PUD_CACHE_INDEX: usize = H_PUD_INDEX_SIZE;

pub const H_VMALLOC_START: usize = H_KERN_VIRT_START;
pub const H_VMALLOC_SIZE: usize = H_KERN_MAP_SIZE;
pub const H_VMALLOC_END: usize = H_VMALLOC_START + H_VMALLOC_SIZE;
pub const H_KERN_IO_START: usize = H_VMALLOC_END;
pub const H_KERN_IO_SIZE: usize = H_KERN_MAP_SIZE;
pub const H_KERN_IO_END: usize = H_KERN_IO_START + H_KERN_IO_SIZE;
pub const H_VMEMMAP_START: usize = H_KERN_IO_END;
pub const H_VMEMMAP_SIZE: usize = H_KERN_MAP_SIZE;
pub const H_VMEMMAP_END: usize = H_VMEMMAP_START + H_VMEMMAP_SIZE;

#[inline]
pub const fn non_linear_region_id(ea: usize) -> usize {
    ((ea.wrapping_sub(H_KERN_VIRT_START)) >> REGION_SHIFT) + 2
}

pub const USER_REGION_ID: usize = 0;
pub const LINEAR_MAP_REGION_ID: usize = 1;
pub const VMALLOC_REGION_ID: usize = non_linear_region_id(H_VMALLOC_START);
pub const IO_REGION_ID: usize = non_linear_region_id(H_KERN_IO_START);
pub const VMEMMAP_REGION_ID: usize = non_linear_region_id(H_VMEMMAP_START);
pub const INVALID_REGION_ID: usize = VMEMMAP_REGION_ID + 1;

pub const _PTEIDX_SECONDARY: usize = 0x8;
pub const _PTEIDX_GROUP_IX: usize = 0x7;
pub const H_PMD_BAD_BITS: usize = PTE_TABLE_SIZE - 1;
pub const H_PUD_BAD_BITS: usize = PMD_TABLE_SIZE - 1;

#[inline]
pub fn get_region_id(ea: usize) -> i32 {
    let id = ea >> 60;
    if id == 0 { return USER_REGION_ID as i32; }
    if id != (PAGE_OFFSET >> 60) { return INVALID_REGION_ID as i32; }
    if ea < H_KERN_VIRT_START { return LINEAR_MAP_REGION_ID as i32; }
    non_linear_region_id(ea) as i32
}

#[inline]
pub fn hash__pmd_same(pmd_a: pmd_t, pmd_b: pmd_t) -> i32 {
    (((pmd_raw(pmd_a) ^ pmd_raw(pmd_b)) & !cpu_to_be64(_PAGE_HPTEFLAGS)) == 0) as i32
}
#[inline]
pub fn hash__pmd_bad(pmd: pmd_t) -> usize { pmd_val(pmd) & H_PMD_BAD_BITS }

#[inline]
pub fn hash__pud_same(pud_a: pud_t, pud_b: pud_t) -> i32 {
    (((pud_raw(pud_a) ^ pud_raw(pud_b)) & !cpu_to_be64(_PAGE_HPTEFLAGS)) == 0) as i32
}
#[inline]
pub fn hash__pud_bad(pud: pud_t) -> usize { pud_val(pud) & H_PUD_BAD_BITS }

#[inline]
pub fn hash__p4d_bad(p4d: p4d_t) -> i32 { (p4d_val(p4d) == 0) as i32 }

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
extern "C" { pub fn hash__mark_rodata_ro(); pub fn hash__mark_initmem_nx(); }
extern "C" {
    pub fn hpte_need_flush(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: usize, huge: i32);
    pub fn htab_convert_pte_flags(pteflags: usize, flags: usize) -> usize;
}

/* The original PowerPC ldarx/stdcx. loop is retained as an architecture TODO. */
#[inline]
pub unsafe fn hash__pte_update_one(_ptep: *mut pte_t, _clr: usize, _set: usize) -> usize {
    todo!("translate PowerPC ldarx/stdcx. PTE update sequence")
}

#[inline]
pub unsafe fn hash__pte_update(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t,
                               clr: usize, set: usize, huge: i32) -> usize {
    let old = hash__pte_update_one(ptep, clr, set);
    /* CONFIG_PPC_4K_PAGES huge-page replication and locking are supplied externally. */
    if huge == 0 { assert_pte_locked(mm, addr); }
    if old & H_PAGE_HASHPTE != 0 { hpte_need_flush(mm, addr, ptep, old, huge); }
    old
}

#[inline]
pub unsafe fn hash__ptep_set_access_flags(_ptep: *mut pte_t, _entry: pte_t) {
    todo!("translate PowerPC atomic access-flag update sequence")
}

#[inline]
pub fn hash__pte_same(pte_a: pte_t, pte_b: pte_t) -> i32 {
    (((pte_raw(pte_a) ^ pte_raw(pte_b)) & !cpu_to_be64(_PAGE_HPTEFLAGS)) == 0) as i32
}
#[inline]
pub fn hash__pte_none(pte: pte_t) -> i32 { ((pte_val(pte) & !H_PTE_NONE_MASK) == 0) as i32 }

extern "C" {
    pub fn pte_get_hash_gslot(vpn: usize, shift: usize, ssize: i32,
                              rpte: real_pte_t, subpg_index: u32) -> usize;
    pub fn hash__map_kernel_page(ea: usize, pa: usize, prot: pgprot_t) -> i32;
    pub fn hash__vmemmap_create_mapping(start: usize, page_size: usize, phys: usize) -> i32;
    pub fn hash__vmemmap_remove_mapping(start: usize, page_size: usize);
    pub fn hash__create_section_mapping(start: usize, end: usize, nid: i32, prot: pgprot_t) -> i32;
    pub fn hash__remove_section_mapping(start: usize, end: usize) -> i32;
}

#[inline]
pub unsafe fn hash__set_pte_at(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t,
                               pte: pte_t, _percpu: i32) { *ptep = pte; }

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
extern "C" { pub fn hpte_do_hugepage_flush(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, old_pmd: usize); }
#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub unsafe fn hpte_do_hugepage_flush(_mm: *mut mm_struct, _addr: usize, _pmdp: *mut pmd_t, _old_pmd: usize) {
    WARN(1, "%s called with THP disabled\n", "hpte_do_hugepage_flush");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
