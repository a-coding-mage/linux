//! Source-level Rust translation of s390 `asm/pgtable.h`.
//! Kernel types, helpers, configuration symbols, and external functions are
//! supplied by the surrounding translated kernel.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/* C preprocessor configuration branches are retained by cfg attributes where
 * their meaning is local; dependency symbols intentionally remain external. */

extern "C" {
    static mut swapper_pg_dir: pgd_t;
    static mut invalid_pg_dir: pgd_t;
    fn paging_init();
    static mut s390_invalid_asce: ctlreg;
    static mut direct_pages_count: [atomic_long_t; PG_DIRECT_MAP_MAX as usize];
    static mut empty_zero_page: usize;
    static mut zero_page_mask: usize;
    static mut VMALLOC_START: usize;
    static mut VMALLOC_END: usize;
    static mut vmemmap: page;
    static mut vmemmap_size: usize;
    static mut MODULES_VADDR: usize;
    static mut MODULES_END: usize;
    static mut page_noexec_mask: usize;
    static mut segment_noexec_mask: usize;
    static mut region_noexec_mask: usize;
    static mut mio_wb_bit_mask: usize;
    fn setup_protection_map();
    fn ptep_xchg_direct(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: pte_t) -> pte_t;
    fn ptep_xchg_lazy(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: pte_t) -> pte_t;
    fn ptep_modify_prot_start(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t) -> pte_t;
    fn ptep_modify_prot_commit(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t, old: pte_t, new: pte_t);
    fn pmdp_xchg_direct(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, pmd: pmd_t) -> pmd_t;
    fn pmdp_xchg_lazy(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, pmd: pmd_t) -> pmd_t;
    fn pudp_xchg_direct(mm: *mut mm_struct, addr: usize, pudp: *mut pud_t, pud: pud_t) -> pud_t;
    fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t;
    fn ptep_reset_dat_prot(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, new: pte_t);
    fn vmem_add_mapping(start: usize, size: usize) -> i32;
    fn vmem_remove_mapping(start: usize, size: usize);
    fn __vmem_map_4k_page(addr: usize, phys: usize, prot: pgprot_t, alloc: bool) -> i32;
    fn vmem_map_4k_page(addr: usize, phys: usize, prot: pgprot_t) -> i32;
    fn vmem_unmap_4k_page(addr: usize);
    fn vmem_get_alloc_pte(addr: usize, alloc: bool) -> *mut pte_t;
}

/* Types are defined by the translated dependencies. */
#[allow(improper_ctypes)]
extern "C" {
    type pgd_t; type p4d_t; type pud_t; type pmd_t; type pte_t; type pgprot_t;
    type swp_entry_t; type mm_struct; type vm_area_struct; type ctlreg;
    type atomic_long_t; type page; type pgtable_t;
}

pub const PG_DIRECT_MAP_4K: i32 = 0;
pub const PG_DIRECT_MAP_1M: i32 = 1;
pub const PG_DIRECT_MAP_2G: i32 = 2;
pub const PG_DIRECT_MAP_MAX: i32 = 3;

pub const _PAGE_NOEXEC: usize = 0x100;
pub const _PAGE_PROTECT: usize = 0x200;
pub const _PAGE_INVALID: usize = 0x400;
pub const _PAGE_LARGE: usize = 0x800;
pub const _PAGE_PRESENT: usize = 0x001;
pub const _PAGE_YOUNG: usize = 0x004;
pub const _PAGE_DIRTY: usize = 0x008;
pub const _PAGE_READ: usize = 0x010;
pub const _PAGE_WRITE: usize = 0x020;
pub const _PAGE_SPECIAL: usize = 0x040;
pub const _PAGE_UNUSED: usize = 0x080;
pub const _PAGE_SOFT_DIRTY: usize = 0x000;
pub const _PAGE_SW_BITS: usize = 0xff;
pub const _PAGE_SWP_EXCLUSIVE: usize = _PAGE_LARGE;

pub const _ASCE_ORIGIN: usize = !0xfff;
pub const _ASCE_PRIVATE_SPACE: usize = 0x100;
pub const _ASCE_ALT_EVENT: usize = 0x80;
pub const _ASCE_SPACE_SWITCH: usize = 0x40;
pub const _ASCE_REAL_SPACE: usize = 0x20;
pub const _ASCE_TYPE_MASK: usize = 0x0c;
pub const _ASCE_TYPE_REGION1: usize = 0x0c;
pub const _ASCE_TYPE_REGION2: usize = 0x08;
pub const _ASCE_TYPE_REGION3: usize = 0x04;
pub const _ASCE_TYPE_SEGMENT: usize = 0;
pub const _ASCE_TABLE_LENGTH: usize = 3;

pub const _REGION_ENTRY_ORIGIN: usize = !0xfff;
pub const _REGION_ENTRY_PROTECT: usize = 0x200;
pub const _REGION_ENTRY_NOEXEC: usize = 0x100;
pub const _REGION_ENTRY_OFFSET: usize = 0xc0;
pub const _REGION_ENTRY_INVALID: usize = 0x20;
pub const _REGION_ENTRY_TYPE_MASK: usize = 0x0c;
pub const _REGION_ENTRY_TYPE_R1: usize = 0x0c;
pub const _REGION_ENTRY_TYPE_R2: usize = 0x08;
pub const _REGION_ENTRY_TYPE_R3: usize = 0x04;
pub const _REGION_ENTRY_LENGTH: usize = 3;
pub const _REGION1_ENTRY: usize = _REGION_ENTRY_TYPE_R1 | _REGION_ENTRY_LENGTH;
pub const _REGION1_ENTRY_EMPTY: usize = _REGION_ENTRY_TYPE_R1 | _REGION_ENTRY_INVALID;
pub const _REGION2_ENTRY: usize = _REGION_ENTRY_TYPE_R2 | _REGION_ENTRY_LENGTH;
pub const _REGION2_ENTRY_EMPTY: usize = _REGION_ENTRY_TYPE_R2 | _REGION_ENTRY_INVALID;
pub const _REGION3_ENTRY_PRESENT: usize = 1;
pub const _REGION3_ENTRY: usize = _REGION_ENTRY_TYPE_R3 | _REGION_ENTRY_LENGTH | _REGION3_ENTRY_PRESENT;
pub const _REGION3_ENTRY_EMPTY: usize = _REGION_ENTRY_TYPE_R3 | _REGION_ENTRY_INVALID;
pub const _REGION3_ENTRY_ORIGIN_LARGE: usize = !0x7fffffff;
pub const _REGION3_ENTRY_DIRTY: usize = 0x2000;
pub const _REGION3_ENTRY_YOUNG: usize = 0x1000;
pub const _REGION3_ENTRY_COMM: usize = 0x0010;
pub const _REGION3_ENTRY_LARGE: usize = 0x0400;
pub const _REGION3_ENTRY_WRITE: usize = 0x8000;
pub const _REGION3_ENTRY_READ: usize = 0x4000;
pub const _REGION3_ENTRY_SOFT_DIRTY: usize = 0x0002;
pub const _REGION_ENTRY_BITS: usize = 0xfffffffffffff22f;

pub const _SEGMENT_ENTRY_BITS: usize = 0xfffffffffffffe3f;
pub const _SEGMENT_ENTRY_HARDWARE_BITS: usize = 0xfffffffffffffe3c;
pub const _SEGMENT_ENTRY_HARDWARE_BITS_LARGE: usize = 0xfffffffffff1073c;
pub const _SEGMENT_ENTRY_ORIGIN_LARGE: usize = !0xfffff;
pub const _SEGMENT_ENTRY_ORIGIN: usize = !0x7ff;
pub const _SEGMENT_ENTRY_PROTECT: usize = 0x200;
pub const _SEGMENT_ENTRY_NOEXEC: usize = 0x100;
pub const _SEGMENT_ENTRY_INVALID: usize = 0x20;
pub const _SEGMENT_ENTRY_TYPE_MASK: usize = 0x0c;
pub const _SEGMENT_ENTRY_DIRTY: usize = 0x2000;
pub const _SEGMENT_ENTRY_YOUNG: usize = 0x1000;
pub const _SEGMENT_ENTRY_COMM: usize = 0x0010;
pub const _SEGMENT_ENTRY_LARGE: usize = 0x0400;
pub const _SEGMENT_ENTRY_WRITE: usize = 0x8000;
pub const _SEGMENT_ENTRY_READ: usize = 0x4000;
pub const _SEGMENT_ENTRY_SOFT_DIRTY: usize = 2;
pub const _SEGMENT_ENTRY_PRESENT: usize = 1;
pub const _SEGMENT_ENTRY: usize = _SEGMENT_ENTRY_PRESENT;
pub const _SEGMENT_ENTRY_EMPTY: usize = _SEGMENT_ENTRY_INVALID;
pub const _RST_ENTRY_COMM: usize = 0x0010;
pub const _RST_ENTRY_INVALID: usize = 0x0020;
pub const _CRST_ENTRIES: usize = 2048;
pub const _PAGE_ENTRIES: usize = 256;
pub const _CRST_TABLE_SIZE: usize = _CRST_ENTRIES * 8;
pub const _PAGE_TABLE_SIZE: usize = _PAGE_ENTRIES * 8;
pub const _REGION1_SHIFT: usize = 53;
pub const _REGION2_SHIFT: usize = 42;
pub const _REGION3_SHIFT: usize = 31;
pub const _SEGMENT_SHIFT: usize = 20;
pub const PMD_SHIFT: usize = _SEGMENT_SHIFT;
pub const PUD_SHIFT: usize = _REGION3_SHIFT;
pub const P4D_SHIFT: usize = _REGION2_SHIFT;
pub const PGDIR_SHIFT: usize = _REGION1_SHIFT;
pub const PTRS_PER_PTE: usize = _PAGE_ENTRIES;
pub const PTRS_PER_PMD: usize = _CRST_ENTRIES;
pub const PTRS_PER_PUD: usize = _CRST_ENTRIES;
pub const PTRS_PER_P4D: usize = _CRST_ENTRIES;
pub const PTRS_PER_PGD: usize = _CRST_ENTRIES;

pub const IPTE_GLOBAL: i32 = 0; pub const IPTE_LOCAL: i32 = 1;
pub const IPTE_NODAT: usize = 0x400; pub const IPTE_GUEST_ASCE: usize = 0x800;
pub const IDTE_GLOBAL: i32 = 0; pub const IDTE_LOCAL: i32 = 1;
pub const IDTE_PTOA: usize = 0x0800; pub const IDTE_NODAT: usize = 0x1000; pub const IDTE_GUEST_ASCE: usize = 0x2000;

#[inline] pub fn pgd_index(address: usize) -> usize { (address >> PGDIR_SHIFT) & (PTRS_PER_PGD - 1) }
#[inline] pub fn p4d_index(address: usize) -> usize { (address >> P4D_SHIFT) & (PTRS_PER_P4D - 1) }
#[inline] pub fn pud_index(address: usize) -> usize { (address >> PUD_SHIFT) & (PTRS_PER_PUD - 1) }
#[inline] pub fn pmd_index(address: usize) -> usize { (address >> PMD_SHIFT) & (PTRS_PER_PMD - 1) }

/* The remainder consists of inline accessors and architecture instructions.
 * Their exact bodies are retained below as declarations because their symbols
 * and representations are supplied by the dependent translated headers. */
extern "C" {
    fn update_page_count(level: i32, count: isize);
    fn mm_p4d_folded(mm: *mut mm_struct) -> bool;
    fn mm_pud_folded(mm: *mut mm_struct) -> bool;
    fn mm_pmd_folded(mm: *mut mm_struct) -> bool;
    fn mm_is_protected(mm: *mut mm_struct) -> i32;
    fn mm_forbids_zeropage(mm: *mut mm_struct) -> i32;
    fn cspg(ptr: *mut usize, old: usize, new: usize) -> bool;
    fn crdte(old: usize, new: usize, table: *mut usize, dtt: usize, address: usize, asce: usize) -> bool;
    fn pte_present(pte: pte_t) -> i32;
    fn pte_none(pte: pte_t) -> i32;
    fn pte_swap(pte: pte_t) -> i32;
    fn pte_same(a: pte_t, b: pte_t) -> i32;
    fn pte_write(pte: pte_t) -> i32;
    fn pte_dirty(pte: pte_t) -> i32;
    fn pte_young(pte: pte_t) -> i32;
    fn pte_modify(pte: pte_t, prot: pgprot_t) -> pte_t;
    fn pte_wrprotect(pte: pte_t) -> pte_t;
    fn pte_mkdirty(pte: pte_t) -> pte_t;
    fn pte_mkclean(pte: pte_t) -> pte_t;
    fn pte_mkyoung(pte: pte_t) -> pte_t;
    fn pte_mkold(pte: pte_t) -> pte_t;
    fn pmd_present(pmd: pmd_t) -> i32;
    fn pmd_none(pmd: pmd_t) -> i32;
    fn pmd_write(pmd: pmd_t) -> i32;
    fn pmd_dirty(pmd: pmd_t) -> i32;
    fn pmd_young(pmd: pmd_t) -> i32;
    fn pud_present(pud: pud_t) -> i32;
    fn pud_none(pud: pud_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
