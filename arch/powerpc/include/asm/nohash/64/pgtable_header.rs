/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file contains the functions and defines necessary to modify and use
 * the ppc64 non-hashed page table.
 *
 * C includes are intentionally omitted; the referenced types and symbols are
 * supplied by the corresponding translated dependencies.
 */

/* Size of EA range mapped by our pagetables. */
pub const PGTABLE_EADDR_SIZE: usize = PTE_INDEX_SIZE + PMD_INDEX_SIZE +
    PUD_INDEX_SIZE + PGD_INDEX_SIZE + PAGE_SHIFT;
pub const PGTABLE_RANGE: u64 = 1u64 << PGTABLE_EADDR_SIZE;

pub const PMD_CACHE_INDEX: usize = PMD_INDEX_SIZE;
pub const PUD_CACHE_INDEX: usize = PUD_INDEX_SIZE;

/* Define the address range of the kernel non-linear virtual area. */
pub const KERN_VIRT_START: u64 = 0xc000_1000_0000_0000;
pub const KERN_VIRT_SIZE: u64 = 0x0000_1000_0000_0000;

/* The vmalloc space starts at the beginning of that region. */
pub const VMALLOC_START: u64 = KERN_VIRT_START;
pub const VMALLOC_SIZE: u64 = KERN_VIRT_SIZE >> 2;
pub const VMALLOC_END: u64 = VMALLOC_START + VMALLOC_SIZE;

/* The third quarter of the kernel virtual space is used for IO mappings. */
pub const KERN_IO_START: u64 = KERN_VIRT_START + (KERN_VIRT_SIZE >> 1);
pub const KERN_IO_SIZE: u64 = KERN_VIRT_SIZE >> 2;
pub const FULL_IO_SIZE: u64 = 0x8000_0000;
pub const ISA_IO_BASE: u64 = KERN_IO_START;
pub const ISA_IO_END: u64 = KERN_IO_START + 0x10000;
pub const PHB_IO_BASE: u64 = ISA_IO_END;
pub const PHB_IO_END: u64 = KERN_IO_START + FULL_IO_SIZE;
pub const IOREMAP_BASE: u64 = PHB_IO_END;
pub const IOREMAP_START: u64 = ioremap_bot;
pub const IOREMAP_END: u64 = KERN_IO_START + KERN_IO_SIZE - FIXADDR_SIZE;
pub const FIXADDR_SIZE: u64 = SZ_32M;
pub const FIXADDR_TOP: u64 = IOREMAP_END + FIXADDR_SIZE;

/* Address of the vmemap area, in its own region after vmalloc on Book3E. */
pub const VMEMMAP_BASE: u64 = VMALLOC_END;
pub const VMEMMAP_END: u64 = KERN_IO_START;
pub const vmemmap: *mut page = VMEMMAP_BASE as *mut page;

pub const PTE_RPN_MASK: usize = !((1usize << PTE_RPN_SHIFT) - 1);
pub const H_PAGE_4K_PFN: usize = 0;

pub const PMD_BAD_BITS: usize = PTE_TABLE_SIZE - 1;
pub const PUD_BAD_BITS: usize = PMD_TABLE_SIZE - 1;

pub unsafe fn pmd_set(pmdp: *mut pmd_t, val: c_ulong) {
    *pmdp = __pmd(val);
}

pub unsafe fn pmd_clear(pmdp: *mut pmd_t) {
    *pmdp = __pmd(0);
}

pub fn pmd_pte(pmd: pmd_t) -> pte_t {
    __pte(pmd_val(pmd))
}

pub fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == 0 }
pub fn pmd_bad(pmd: pmd_t) -> bool {
    !is_kernel_addr(pmd_val(pmd)) || (pmd_val(pmd) & PMD_BAD_BITS) != 0
}
pub fn pmd_present(pmd: pmd_t) -> bool { !pmd_none(pmd) }
pub fn pmd_page_vaddr(pmd: pmd_t) -> *const c_void {
    (pmd_val(pmd) & !PMD_MASKED_BITS) as *const c_void
}
extern "C" { pub fn pmd_page(pmd: pmd_t) -> *mut page; }
pub fn pmd_pfn(pmd: pmd_t) -> c_ulong { page_to_pfn(unsafe { pmd_page(pmd) }) }

pub unsafe fn pud_set(pudp: *mut pud_t, val: c_ulong) {
    *pudp = __pud(val);
}

pub unsafe fn pud_clear(pudp: *mut pud_t) {
    *pudp = __pud(0);
}

pub fn pud_none(pud: pud_t) -> bool { pud_val(pud) == 0 }
pub fn pud_bad(pud: pud_t) -> bool {
    !is_kernel_addr(pud_val(pud)) || (pud_val(pud) & PUD_BAD_BITS) != 0
}
pub fn pud_present(pud: pud_t) -> bool { pud_val(pud) != 0 }

pub fn pud_pgtable(pud: pud_t) -> *mut pmd_t {
    (pud_val(pud) & !PUD_MASKED_BITS) as *mut pmd_t
}

extern "C" { pub fn pud_page(pud: pud_t) -> *mut page; }

pub fn pud_pte(pud: pud_t) -> pte_t { __pte(pud_val(pud)) }
pub fn pte_pud(pte: pte_t) -> pud_t { __pud(pte_val(pte)) }
pub fn pud_write(pud: pud_t) -> bool { pte_write(pud_pte(pud)) }
pub fn p4d_write(pgd: p4d_t) -> bool { pte_write(p4d_pte(p4d)) }

pub unsafe fn p4d_set(p4dp: *mut p4d_t, val: c_ulong) {
    *p4dp = __p4d(val);
}

pub unsafe fn huge_ptep_set_wrprotect(mm: *mut mm_struct, addr: c_ulong,
                                      ptep: *mut pte_t) {
    pte_update(mm, addr, ptep, _PAGE_WRITE, 0, 1);
}

pub unsafe fn ptep_clear_flush_young(vma: *mut vm_area_struct,
                                     address: c_ulong, ptep: *mut pte_t) -> bool {
    ptep_test_and_clear_young(vma, address, ptep)
}

pub unsafe fn pmd_ERROR(e: pmd_t) {
    pr_err("%s:%d: bad pmd %08lx.\n", file!(), line!(), pmd_val(e));
}
pub unsafe fn pgd_ERROR(e: pgd_t) {
    pr_err("%s:%d: bad pgd %08lx.\n", file!(), line!(), pgd_val(e));
}

pub const SWP_TYPE_BITS: usize = 5;
pub fn __swp_type(x: swp_entry_t) -> c_ulong {
    (x.val >> 2) & ((1 << SWP_TYPE_BITS) - 1)
}
pub fn __swp_offset(x: swp_entry_t) -> c_ulong { x.val >> PTE_RPN_SHIFT }
pub fn __swp_entry(type_: c_ulong, offset: c_ulong) -> swp_entry_t {
    swp_entry_t { val: ((type_ & 0x1f) << 2) | (offset << PTE_RPN_SHIFT) }
}
pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t {
    swp_entry_t { val: pte_val(pte) }
}
pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }

/* We borrow MSB 56 (LSB 7) to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: usize = 0x80;

extern "C" {
    pub fn vmemmap_create_mapping(start: c_ulong, page_size: c_ulong,
                                  phys: c_ulong) -> c_int;
    pub fn vmemmap_remove_mapping(start: c_ulong, page_size: c_ulong);
    pub fn __patch_exception(exc: c_int, addr: c_ulong);
}

/* Build-time C macro patch_exception(exc, name): address of the external symbol. */
pub unsafe fn patch_exception(exc: c_int, name: *const c_uint) {
    __patch_exception(exc, name as c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
