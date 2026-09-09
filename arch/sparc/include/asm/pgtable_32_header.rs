/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of sparc/include/asm/pgtable_32.h. */

pub const PMD_SHIFT: u32 = 18;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
#[inline]
pub const fn PMD_ALIGN(addr: usize) -> usize { (addr + !PMD_MASK) & PMD_MASK }

pub const PGDIR_SHIFT: u32 = 24;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
#[inline]
pub const fn PGDIR_ALIGN(addr: usize) -> usize { (addr + !PGDIR_MASK) & PGDIR_MASK }

extern "C" {
    pub fn load_mmu();
    pub fn calc_highpages() -> c_ulong;
    pub fn bootmem_init(pages_avail: *mut c_ulong) -> c_ulong;
    pub fn paging_init();
    pub static mut ptr_in_current_pgd: c_ulong;
    pub static mut phys_base: c_ulong;
    pub static mut pfn_base: c_ulong;
    pub fn mmu_info(m: *mut seq_file);
    pub fn srmmu_mapiorange(bus: c_uint, xpa: c_ulong, xva: c_ulong, len: c_uint);
    pub fn srmmu_unmapiorange(virt_addr: c_ulong, len: c_uint);
    pub fn srmmu_get_pte(addr: c_ulong) -> c_ulong;
    pub fn flush_tlb_page(vma: *mut vm_area_struct, address: c_ulong);
}

pub type c_ulong = usize;
pub type c_uint = u32;
pub type c_int = i32;

#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }

pub const PTRS_PER_PTE: usize = 64;
pub const PTRS_PER_PMD: usize = 64;
pub const PTRS_PER_PGD: usize = 256;
pub const PTE_SIZE: usize = PTRS_PER_PTE * 4;
pub const FAULT_CODE_PROT: usize = 0x1;
pub const FAULT_CODE_WRITE: usize = 0x2;
pub const FAULT_CODE_USER: usize = 0x4;

/* External types and constants are supplied by the architecture dependencies. */
extern "C" {
    pub fn pfn_to_page(pfn: c_ulong) -> *mut page;
    pub fn __nocache_va(v: c_ulong) -> *mut u8;
    pub fn BUG() -> !;
}

#[inline] pub unsafe fn srmmu_swap(addr: *mut c_ulong, value: c_ulong) -> c_ulong {
    let mut value = value;
    core::arch::asm!("swap [{addr}], {value}", addr = in(reg) addr, value = inout(reg) value, options(nostack));
    value
}

#[inline] pub unsafe fn set_pte(ptep: *mut pte_t, pteval: pte_t) { srmmu_swap(ptep as *mut c_ulong, pte_val(pteval)); }
#[inline] pub fn srmmu_device_memory(x: c_ulong) -> bool { (x & 0xF0000000) != 0 }
#[inline] pub fn pmd_pfn(pmd: pmd_t) -> c_ulong { (pmd_val(pmd) & SRMMU_PTD_PMASK) >> (PAGE_SHIFT - 4) }
#[inline] pub unsafe fn pmd_page(pmd: pmd_t) -> *mut page { if srmmu_device_memory(pmd_val(pmd)) { BUG() } pfn_to_page(pmd_pfn(pmd)) }
#[inline] pub unsafe fn __pmd_page(pmd: pmd_t) -> c_ulong { if srmmu_device_memory(pmd_val(pmd)) { BUG() } (__nocache_va((pmd_val(pmd) & SRMMU_PTD_PMASK) << 4)) as c_ulong }
#[inline] pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> c_ulong { __nocache_va((pmd_val(pmd) & SRMMU_PTD_PMASK) << 4) as c_ulong }
#[inline] pub unsafe fn pud_pgtable(pud: pud_t) -> *mut pmd_t { if srmmu_device_memory(pud_val(pud)) { usize::MAX as *mut pmd_t } else { __nocache_va((pud_val(pud) & SRMMU_PTD_PMASK) << 4) as *mut pmd_t } }
#[inline] pub fn pte_present(pte: pte_t) -> bool { (pte_val(pte) & SRMMU_ET_MASK) == SRMMU_ET_PTE }
#[inline] pub fn pte_none(pte: pte_t) -> bool { pte_val(pte) == 0 }
#[inline] pub unsafe fn __pte_clear(ptep: *mut pte_t) { set_pte(ptep, __pte(0)); }
#[inline] pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: c_ulong, ptep: *mut pte_t) { __pte_clear(ptep); }
#[inline] pub fn pmd_bad(pmd: pmd_t) -> bool { (pmd_val(pmd) & SRMMU_ET_MASK) != SRMMU_ET_PTD }
#[inline] pub fn pmd_present(pmd: pmd_t) -> bool { (pmd_val(pmd) & SRMMU_ET_MASK) == SRMMU_ET_PTD }
#[inline] pub fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == 0 }
#[inline] pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { set_pte(&mut *(pmdp as *mut pte_t), __pte(0)); }
#[inline] pub fn pud_none(pud: pud_t) -> bool { (pud_val(pud) & 0xFFFFFFF) == 0 }
#[inline] pub fn pud_bad(pud: pud_t) -> bool { (pud_val(pud) & SRMMU_ET_MASK) != SRMMU_ET_PTD }
#[inline] pub fn pud_present(pud: pud_t) -> bool { (pud_val(pud) & SRMMU_ET_MASK) == SRMMU_ET_PTD }
#[inline] pub unsafe fn pud_clear(pudp: *mut pud_t) { set_pte(pudp as *mut pte_t, __pte(0)); }

#[inline] pub fn pte_write(pte: pte_t) -> bool { (pte_val(pte) & SRMMU_WRITE) != 0 }
#[inline] pub fn pte_dirty(pte: pte_t) -> bool { (pte_val(pte) & SRMMU_DIRTY) != 0 }
#[inline] pub fn pte_young(pte: pte_t) -> bool { (pte_val(pte) & SRMMU_REF) != 0 }
#[inline] pub fn pte_wrprotect(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !SRMMU_WRITE) }
#[inline] pub fn pte_mkclean(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !SRMMU_DIRTY) }
#[inline] pub fn pte_mkold(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !SRMMU_REF) }
#[inline] pub fn pte_mkwrite_novma(pte: pte_t) -> pte_t { __pte(pte_val(pte) | SRMMU_WRITE) }
#[inline] pub fn pte_mkdirty(pte: pte_t) -> pte_t { __pte(pte_val(pte) | SRMMU_DIRTY) }
#[inline] pub fn pte_mkyoung(pte: pte_t) -> pte_t { __pte(pte_val(pte) | SRMMU_REF) }

pub const PFN_PTE_SHIFT: usize = PAGE_SHIFT - 4;
#[inline] pub fn pfn_pte(pfn: c_ulong, pgprot: pgprot_t) -> pte_t { __pte((pfn << PFN_PTE_SHIFT) | pgprot_val(pgprot)) }
#[inline] pub fn pte_pfn(pte: pte_t) -> c_ulong { if srmmu_device_memory(pte_val(pte)) { usize::MAX } else { (pte_val(pte) & SRMMU_PTE_PMASK) >> PFN_PTE_SHIFT } }
#[inline] pub fn pte_page(pte: pte_t) -> *mut page { unsafe { pfn_to_page(pte_pfn(pte)) } }
#[inline] pub fn mk_pte_phys(page: c_ulong, pgprot: pgprot_t) -> pte_t { __pte((page >> 4) | pgprot_val(pgprot)) }
#[inline] pub fn mk_pte_io(page: c_ulong, pgprot: pgprot_t, space: c_int) -> pte_t { __pte((page >> 4) | ((space as c_ulong) << 28) | pgprot_val(pgprot)) }
#[inline] pub fn pgprot_noncached(mut prot: pgprot_t) -> pgprot_t { pgprot_val_mut(&mut prot, pgprot_val(prot) & !pgprot_val(__pgprot(SRMMU_CACHE))); prot }
#[inline] pub fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t { __pte((pte_val(pte) & SRMMU_CHG_MASK) | pgprot_val(newprot)) }
pub const VMALLOC_START: usize = 0xfe600000;
pub const VMALLOC_END: usize = 0xffc00000;
pub const MODULES_VADDR: usize = VMALLOC_START;
pub const MODULES_END: usize = VMALLOC_END;

#[inline] pub fn __swp_type(entry: swp_entry_t) -> c_ulong { (entry.val >> SRMMU_SWP_TYPE_SHIFT) & SRMMU_SWP_TYPE_MASK }
#[inline] pub fn __swp_offset(entry: swp_entry_t) -> c_ulong { (entry.val >> SRMMU_SWP_OFF_SHIFT) & SRMMU_SWP_OFF_MASK }
#[inline] pub fn __swp_entry(ty: c_ulong, offset: c_ulong) -> swp_entry_t { swp_entry_t { val: ((ty & SRMMU_SWP_TYPE_MASK) << SRMMU_SWP_TYPE_SHIFT) | ((offset & SRMMU_SWP_OFF_MASK) << SRMMU_SWP_OFF_SHIFT) } }
#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { (pte_val(pte) & SRMMU_SWP_EXCLUSIVE) != 0 }
#[inline] pub fn pte_swp_mkexclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) | SRMMU_SWP_EXCLUSIVE) }
#[inline] pub fn pte_swp_clear_exclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !SRMMU_SWP_EXCLUSIVE) }

#[inline] pub unsafe fn __get_phys(addr: c_ulong) -> c_ulong { match sparc_cpu_model { sun4m | sun4d => (srmmu_get_pte(addr) & 0xffffff00) << 4, _ => 0 } }
#[inline] pub unsafe fn __get_iospace(addr: c_ulong) -> c_int { match sparc_cpu_model { sun4m | sun4d => (srmmu_get_pte(addr) >> 28) as c_int, _ => -1 } }
#[inline] pub fn MK_IOSPACE_PFN(space: c_ulong, pfn: c_ulong) -> c_ulong { pfn | (space << (BITS_PER_LONG - 4)) }
#[inline] pub fn GET_IOSPACE(pfn: c_ulong) -> c_ulong { pfn >> (BITS_PER_LONG - 4) }
#[inline] pub fn GET_PFN(pfn: c_ulong) -> c_ulong { pfn & 0x0fffffff }
#[inline] pub fn io_remap_pfn_range_pfn(pfn: c_ulong, _size: c_ulong) -> c_ulong { let offset = (GET_PFN(pfn) as u64) << PAGE_SHIFT; let space = GET_IOSPACE(pfn) as u64; ((offset | (space << 32)) >> PAGE_SHIFT) as c_ulong }

/* ptep_set_access_flags and empty update_mmu_cache hooks retain their C macro intent. */
pub const HAVE_ARCH_PTEP_SET_ACCESS_FLAGS: bool = true;

#[inline] pub fn pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline] pub fn swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }
#[inline] pub fn pud_page(_pud: pud_t) -> *mut u8 { core::ptr::null_mut() }
#[inline] pub unsafe fn pmd_pgtable(pmd: pmd_t) -> *mut u8 { __pmd_page(pmd) as *mut u8 }

/* Empty C hooks. */
#[inline] pub fn update_mmu_cache(_vma: *mut vm_area_struct, _address: c_ulong, _ptep: *mut pte_t) {}
#[inline] pub fn update_mmu_cache_range(_vmf: *mut u8, _vma: *mut vm_area_struct, _address: c_ulong, _ptep: *mut pte_t, _nr: c_ulong) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
