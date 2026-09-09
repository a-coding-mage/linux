/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 *
 * Rust translation of mips/include/asm/pgtable-32.h.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external. Build-time CONFIG_* conditions from the C header are preserved.

pub static mut temp_tlb_entry: ::core::ffi::c_int = 0;

pub unsafe extern "C" fn add_temporary_entry(
    entrylo0: ::core::ffi::c_ulong,
    entrylo1: ::core::ffi::c_ulong,
    entryhi: ::core::ffi::c_ulong,
    pagemask: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int;

// CONFIG_MIPS_HUGE_TLB_SUPPORT && !CONFIG_PHYS_ADDR_T_64BIT:
// const PGDIR_SHIFT = 2 * PAGE_SHIFT - PTE_T_LOG2 - 1;
// Otherwise: const PGDIR_SHIFT = 2 * PAGE_SHIFT - PTE_T_LOG2;
pub const PGDIR_SIZE: ::core::ffi::c_ulong = 1u64 << PGDIR_SHIFT;
pub const PGDIR_MASK: ::core::ffi::c_ulong = !(PGDIR_SIZE - 1);

// The selected __PGD_TABLE_ORDER follows the same CONFIG_* condition above.
pub const PGD_TABLE_ORDER: usize = if __PGD_TABLE_ORDER >= 0 { __PGD_TABLE_ORDER as usize } else { 0 };
pub const PUD_TABLE_ORDER: usize = aieeee_attempt_to_allocate_pud;
pub const PMD_TABLE_ORDER: usize = aieeee_attempt_to_allocate_pmd;

pub const PTRS_PER_PGD: usize = USER_PTRS_PER_PGD * 2;
// CONFIG_MIPS_HUGE_TLB_SUPPORT && !CONFIG_PHYS_ADDR_T_64BIT uses the / 2 form.
pub const PTRS_PER_PTE: usize = PAGE_SIZE / core::mem::size_of::<pte_t>();
pub const USER_PTRS_PER_PGD: ::core::ffi::c_ulong = 0x80000000u64 / PGDIR_SIZE;
pub const VMALLOC_START: usize = MAP_BASE;
pub const PKMAP_END: usize = FIXADDR_START & !((LAST_PKMAP << PAGE_SHIFT) - 1);
pub const PKMAP_BASE: usize = PKMAP_END - PAGE_SIZE * LAST_PKMAP;
// CONFIG_HIGHMEM: VMALLOC_END = PKMAP_BASE - 2 * PAGE_SIZE; otherwise FIXADDR_START - 2 * PAGE_SIZE.

pub unsafe extern "C" fn load_pgd(pg_dir: ::core::ffi::c_ulong);

pub static mut invalid_pte_table: [pte_t; PTRS_PER_PTE] = [unsafe { core::mem::zeroed() }; PTRS_PER_PTE];

#[inline]
pub unsafe fn pmd_none(pmd: pmd_t) -> ::core::ffi::c_int {
    (pmd_val(pmd) == invalid_pte_table.as_ptr() as ::core::ffi::c_ulong) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn pmd_bad(pmd: pmd_t) -> ::core::ffi::c_int {
    // CONFIG_MIPS_HUGE_TLB_SUPPORT: pmd_leaf(pmd) is inlined here.
    if (pmd_val(pmd) & _PAGE_HUGE) != 0 { return 0; }
    if (pmd_val(pmd) & !PAGE_MASK) != 0 { return 1; }
    0
}

#[inline]
pub unsafe fn pmd_present(pmd: pmd_t) -> ::core::ffi::c_int {
    (pmd_val(pmd) != invalid_pte_table.as_ptr() as ::core::ffi::c_ulong) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) {
    pmd_val(*pmdp) = invalid_pte_table.as_ptr() as ::core::ffi::c_ulong;
}

// CONFIG_XPA
pub const MAX_POSSIBLE_PHYSMEM_BITS: usize = 32;

#[inline]
pub unsafe fn pte_pfn(x: pte_t) -> ::core::ffi::c_ulong {
    (x.pte >> PFN_PTE_SHIFT) as ::core::ffi::c_ulong
}

#[inline]
pub unsafe fn pfn_pte(pfn: ::core::ffi::c_ulong, prot: pgprot_t) -> pte_t {
    __pte(((pfn as u64) << PFN_PTE_SHIFT) | pgprot_val(prot) as u64)
}

#[inline]
pub unsafe fn pfn_pmd(pfn: ::core::ffi::c_ulong, prot: pgprot_t) -> pmd_t {
    __pmd(((pfn as u64) << PFN_PTE_SHIFT) | pgprot_val(prot) as u64)
}

#[inline]
pub unsafe fn pte_page(x: pte_t) -> *mut page {
    pfn_to_page(pte_pfn(x))
}

// Swap-entry encoding. Select exactly one block according to CONFIG_CPU_R3K_TLB,
// CONFIG_XPA, or CONFIG_PHYS_ADDR_T_64BIT && CONFIG_CPU_MIPS32.
pub const _PAGE_SWP_EXCLUSIVE: u32 = 1 << 1;

#[inline]
pub unsafe fn __swp_type(x: swp_entry_t) -> usize { ((x.val >> 8) & 0x1f) as usize }
#[inline]
pub unsafe fn __swp_offset(x: swp_entry_t) -> usize { (x.val >> 13) as usize }
#[inline]
pub unsafe fn __swp_entry(typ: usize, offset: usize) -> swp_entry_t {
    swp_entry_t { val: ((typ as u64) << 8) | ((offset as u64) << 13) }
}
#[inline]
pub unsafe fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline]
pub unsafe fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
