/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of OpenRISC asm/pgtable.h. */

/* Dependencies supplied by the surrounding kernel translation are intentionally external. */
extern "C" {
    pub fn paging_init();
    pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD];
    pub fn update_cache(vma: *mut vm_area_struct, address: usize, pte: *mut pte_t);
}

pub type pte_addr_t = *mut pte_t;

pub const PGDIR_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - 2);
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const PTRS_PER_PTE: usize = 1usize << (PAGE_SHIFT - 2);
pub const PTRS_PER_PGD: usize = 1usize << (32 - PGDIR_SHIFT);
pub const USER_PTRS_PER_PGD: usize = TASK_SIZE / PGDIR_SIZE;
pub const VMALLOC_START: usize = PAGE_OFFSET - 0x04000000usize;
pub const VMALLOC_END: usize = PAGE_OFFSET;

pub const _PAGE_CC: usize = 0x001;
pub const _PAGE_CI: usize = 0x002;
pub const _PAGE_WBC: usize = 0x004;
pub const _PAGE_WOM: usize = 0x008;
pub const _PAGE_A: usize = 0x010;
pub const _PAGE_D: usize = 0x020;
pub const _PAGE_URE: usize = 0x040;
pub const _PAGE_UWE: usize = 0x080;
pub const _PAGE_SRE: usize = 0x100;
pub const _PAGE_SWE: usize = 0x200;
pub const _PAGE_EXEC: usize = 0x400;
pub const _PAGE_U_SHARED: usize = 0x800;
pub const _PAGE_PRESENT: usize = _PAGE_CC;
pub const _PAGE_USER: usize = _PAGE_URE;
pub const _PAGE_WRITE: usize = _PAGE_UWE | _PAGE_SWE;
pub const _PAGE_DIRTY: usize = _PAGE_D;
pub const _PAGE_ACCESSED: usize = _PAGE_A;
pub const _PAGE_NO_CACHE: usize = _PAGE_CI;
pub const _PAGE_SHARED: usize = _PAGE_U_SHARED;
pub const _PAGE_READ: usize = _PAGE_URE | _PAGE_SRE;
pub const _PAGE_CHG_MASK: usize = PAGE_MASK | _PAGE_ACCESSED | _PAGE_DIRTY;
pub const _PAGE_BASE: usize = _PAGE_PRESENT | _PAGE_ACCESSED;
pub const _PAGE_ALL: usize = _PAGE_PRESENT | _PAGE_ACCESSED;
pub const _KERNPG_TABLE: usize = _PAGE_BASE | _PAGE_SRE | _PAGE_SWE | _PAGE_ACCESSED | _PAGE_DIRTY;
pub const _PAGE_SWP_EXCLUSIVE: usize = _PAGE_U_SHARED;

#[inline] pub unsafe fn set_pte(pteptr: *mut pte_t, pteval: pte_t) { *pteptr = pteval; }
#[inline] pub unsafe fn set_pmd(pmdptr: *mut pmd_t, pmdval: pmd_t) { *pmdptr = pmdval; }

pub const fn VMALLOC_VMADDR(x: usize) -> usize { x }

/* __pgprot, pte_val, pgprot_val, pmd_val and related representations are external. */
pub const PAGE_NONE: pgprot_t = __pgprot(_PAGE_ALL);
pub const PAGE_READONLY: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_URE | _PAGE_SRE);
pub const PAGE_READONLY_X: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_URE | _PAGE_SRE | _PAGE_EXEC);
pub const PAGE_SHARED: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_URE | _PAGE_SRE | _PAGE_UWE | _PAGE_SWE | _PAGE_SHARED);
pub const PAGE_SHARED_X: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_URE | _PAGE_SRE | _PAGE_UWE | _PAGE_SWE | _PAGE_SHARED | _PAGE_EXEC);
pub const PAGE_COPY: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_URE | _PAGE_SRE);
pub const PAGE_COPY_X: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_URE | _PAGE_SRE | _PAGE_EXEC);
pub const PAGE_KERNEL: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_SRE | _PAGE_SWE | _PAGE_SHARED | _PAGE_DIRTY | _PAGE_EXEC);
pub const PAGE_KERNEL_RO: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_SRE | _PAGE_SHARED | _PAGE_DIRTY | _PAGE_EXEC);
pub const PAGE_KERNEL_NOCACHE: pgprot_t = __pgprot(_PAGE_ALL | _PAGE_SRE | _PAGE_SWE | _PAGE_SHARED | _PAGE_DIRTY | _PAGE_EXEC | _PAGE_CI);

#[inline] pub fn pte_read(pte: pte_t) -> usize { pte_val(pte) & _PAGE_READ }
#[inline] pub fn pte_write(pte: pte_t) -> usize { pte_val(pte) & _PAGE_WRITE }
#[inline] pub fn pte_exec(pte: pte_t) -> usize { pte_val(pte) & _PAGE_EXEC }
#[inline] pub fn pte_dirty(pte: pte_t) -> usize { pte_val(pte) & _PAGE_DIRTY }
#[inline] pub fn pte_young(pte: pte_t) -> usize { pte_val(pte) & _PAGE_ACCESSED }

#[inline] pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte); pte_val_set(&mut pte, pte_val(pte) & !_PAGE_WRITE); pte }
#[inline] pub fn pte_rdprotect(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) & !_PAGE_READ); pte }
#[inline] pub fn pte_exprotect(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) & !_PAGE_EXEC); pte }
#[inline] pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) & !_PAGE_DIRTY); pte }
#[inline] pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) & !_PAGE_ACCESSED); pte }
#[inline] pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) | _PAGE_WRITE); pte }
#[inline] pub fn pte_mkread(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) | _PAGE_READ); pte }
#[inline] pub fn pte_mkexec(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) | _PAGE_EXEC); pte }
#[inline] pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) | _PAGE_DIRTY); pte }
#[inline] pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) | _PAGE_ACCESSED); pte }

#[inline] pub unsafe fn __mk_pte(page: *mut core::ffi::c_void, pgprot: pgprot_t) -> pte_t { __pte(__pa(page) | pgprot_val(pgprot)) }
#[inline] pub fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t { pte_val_set(&mut pte, (pte_val(pte) & _PAGE_CHG_MASK) | pgprot_val(newprot)); pte }
#[inline] pub unsafe fn __pte_page(pte: pte_t) -> usize { __va(pte_val(pte) & PAGE_MASK) as usize }
#[inline] pub unsafe fn pmd_set(pmdp: *mut pmd_t, ptep: *mut pte_t) { pmd_val_set(&mut *pmdp, _KERNPG_TABLE | ptep as usize); }
#[inline] pub fn pmd_page_vaddr(pmd: pmd_t) -> usize { unsafe { __va(pmd_val(pmd) & PAGE_MASK) as usize } }
#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE != 0 }
#[inline] pub fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) | _PAGE_SWP_EXCLUSIVE); pte }
#[inline] pub fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_val_set(&mut pte, pte_val(pte) & !_PAGE_SWP_EXCLUSIVE); pte }

pub struct vm_area_struct;
pub struct vm_fault;

#[inline] pub unsafe fn update_tlb(_vma: *mut vm_area_struct, _address: usize, _pte: *mut pte_t) {}
#[inline] pub unsafe fn update_mmu_cache_range(vmf: *mut vm_fault, vma: *mut vm_area_struct, address: usize, ptep: *mut pte_t, _nr: u32) { update_tlb(vma, address, ptep); update_cache(vma, address, ptep); }

/* Swap encoding: type occupies bits 5..10, offset begins at bit 12. */
#[inline] pub fn __swp_type(x: swp_entry_t) -> usize { (x.val >> 5) & 0x3f }
#[inline] pub fn __swp_offset(x: swp_entry_t) -> usize { x.val >> 12 }
#[inline] pub fn __swp_entry(ty: usize, offset: usize) -> swp_entry_t { swp_entry_t { val: ((ty & 0x3f) << 5) | (offset << 12) } }
#[inline] pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline] pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
