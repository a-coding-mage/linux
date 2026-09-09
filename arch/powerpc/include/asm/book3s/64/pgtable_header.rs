/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of asm-powerpc/book3s/64/pgtable.h. External kernel
 * types, constants, functions, and configuration predicates are supplied by
 * the surrounding translation unit. */

pub const _PAGE_EXEC: usize = 0x00001;
pub const _PAGE_WRITE: usize = 0x00002;
pub const _PAGE_READ: usize = 0x00004;
pub const _PAGE_PRIVILEGED: usize = 0x00008;
pub const _PAGE_SAO: usize = 0x00010;
pub const _PAGE_NON_IDEMPOTENT: usize = 0x00020;
pub const _PAGE_TOLERANT: usize = 0x00030;
pub const _PAGE_DIRTY: usize = 0x00080;
pub const _PAGE_ACCESSED: usize = 0x00100;
pub const _RPAGE_SW0: usize = 0x2000000000000000;
pub const _RPAGE_SW1: usize = 0x00800;
pub const _RPAGE_SW2: usize = 0x00400;
pub const _RPAGE_SW3: usize = 0x00200;
pub const _RPAGE_RSV1: usize = 0x00040;
pub const _RPAGE_PKEY_BIT4: usize = 0x1000000000000000;
pub const _RPAGE_PKEY_BIT3: usize = 0x0800000000000000;
pub const _RPAGE_PKEY_BIT2: usize = 0x0400000000000000;
pub const _RPAGE_PKEY_BIT1: usize = 0x0200000000000000;
pub const _RPAGE_PKEY_BIT0: usize = 0x0100000000000000;
pub const _PAGE_PTE: usize = 0x4000000000000000;
pub const _PAGE_PRESENT: usize = 0x8000000000000000;
pub const _PAGE_INVALID: usize = _RPAGE_SW0;
pub const _RPAGE_RPN0: usize = 0x01000;
pub const _RPAGE_RPN1: usize = 0x02000;
pub const _RPAGE_RPN43: usize = 0x0080000000000000;
pub const _RPAGE_RPN42: usize = 0x0040000000000000;
pub const _RPAGE_RPN41: usize = 0x0020000000000000;
pub const _RPAGE_PA_MAX: usize = 56;
pub const _PAGE_PA_MAX: usize = 53;
pub const _PAGE_SOFT_DIRTY: usize = _RPAGE_SW3;
pub const _PAGE_SPECIAL: usize = _RPAGE_SW2;
pub const _PAGE_NO_CACHE: usize = _PAGE_TOLERANT;

pub const PTE_RPN_MASK: usize = (((1usize << _PAGE_PA_MAX) - 1) & PAGE_MASK);
pub const PTE_RPN_SHIFT: usize = PAGE_SHIFT;
pub const _HPAGE_CHG_MASK: usize = PTE_RPN_MASK | _PAGE_HPTEFLAGS | _PAGE_DIRTY |
    _PAGE_ACCESSED | H_PAGE_THP_HUGE | _PAGE_SPECIAL | _PAGE_PTE | _PAGE_SOFT_DIRTY;
pub const _PAGE_KERNEL_RW: usize = _PAGE_PRIVILEGED | _PAGE_RW | _PAGE_DIRTY;
pub const _PAGE_KERNEL_RO: usize = _PAGE_PRIVILEGED | _PAGE_READ;
pub const _PAGE_KERNEL_ROX: usize = _PAGE_PRIVILEGED | _PAGE_READ | _PAGE_EXEC;
pub const _PAGE_KERNEL_RWX: usize = _PAGE_PRIVILEGED | _PAGE_DIRTY | _PAGE_RW | _PAGE_EXEC;
pub const _PAGE_CHG_MASK: usize = PTE_RPN_MASK | _PAGE_HPTEFLAGS | _PAGE_DIRTY |
    _PAGE_ACCESSED | _PAGE_SPECIAL | _PAGE_PTE | _PAGE_SOFT_DIRTY;
pub const _PAGE_BASE_NC: usize = _PAGE_PRESENT | _PAGE_ACCESSED;
pub const _PAGE_BASE: usize = _PAGE_BASE_NC;
pub const _PAGE_CACHE_CTL: usize = _PAGE_SAO | _PAGE_NON_IDEMPOTENT | _PAGE_TOLERANT;

pub const PTE_INDEX_SIZE: usize = unsafe { __pte_index_size };
pub const PMD_INDEX_SIZE: usize = unsafe { __pmd_index_size };
pub const PUD_INDEX_SIZE: usize = unsafe { __pud_index_size };
pub const PGD_INDEX_SIZE: usize = unsafe { __pgd_index_size };
pub const PMD_CACHE_INDEX: usize = 0;
pub const PTE_TABLE_SIZE: usize = unsafe { __pte_table_size };
pub const PMD_TABLE_SIZE: usize = unsafe { __pmd_table_size };
pub const PUD_TABLE_SIZE: usize = unsafe { __pud_table_size };
pub const PGD_TABLE_SIZE: usize = unsafe { __pgd_table_size };
pub const PTE_FRAG_NR: usize = unsafe { __pte_frag_nr };
pub const PTE_FRAG_SIZE_SHIFT: usize = unsafe { __pte_frag_size_shift };
pub const PTE_FRAG_SIZE: usize = 1usize << PTE_FRAG_SIZE_SHIFT;
pub const PMD_FRAG_NR: usize = unsafe { __pmd_frag_nr };
pub const PMD_FRAG_SIZE_SHIFT: usize = unsafe { __pmd_frag_size_shift };
pub const PMD_FRAG_SIZE: usize = 1usize << PMD_FRAG_SIZE_SHIFT;
pub const PTRS_PER_PTE: usize = 1usize << PTE_INDEX_SIZE;
pub const PTRS_PER_PMD: usize = 1usize << PMD_INDEX_SIZE;
pub const PTRS_PER_PUD: usize = 1usize << PUD_INDEX_SIZE;
pub const PTRS_PER_PGD: usize = 1usize << PGD_INDEX_SIZE;
pub const PMD_SHIFT: usize = PAGE_SHIFT + PTE_INDEX_SIZE;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
pub const PUD_SHIFT: usize = PMD_SHIFT + PMD_INDEX_SIZE;
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
pub const PUD_MASK: usize = !(PUD_SIZE - 1);
pub const PGDIR_SHIFT: usize = PUD_SHIFT + PUD_INDEX_SIZE;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const PMD_MASKED_BITS: usize = 0xc0000000000000ff;
pub const PUD_MASKED_BITS: usize = 0xc0000000000000ff;
pub const P4D_MASKED_BITS: usize = 0xc0000000000000ff;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pgtable_index { PTE_INDEX = 0, PMD_INDEX, PUD_INDEX, PGD_INDEX, HTLB_16M_INDEX, HTLB_16G_INDEX }

extern "C" {
    pub static mut __pte_index_size: usize; pub static mut __pmd_index_size: usize;
    pub static mut __pud_index_size: usize; pub static mut __pgd_index_size: usize;
    pub static mut __pud_cache_index: usize; pub static mut __pte_table_size: usize;
    pub static mut __pmd_table_size: usize; pub static mut __pud_table_size: usize;
    pub static mut __pgd_table_size: usize; pub static mut __pmd_val_bits: usize;
    pub static mut __pud_val_bits: usize; pub static mut __pgd_val_bits: usize;
    pub static mut __pte_frag_nr: usize; pub static mut __pte_frag_size_shift: usize;
    pub static mut __pmd_frag_nr: usize; pub static mut __pmd_frag_size_shift: usize;
    pub static mut __vmalloc_start: usize; pub static mut __vmalloc_end: usize;
    pub static mut __kernel_virt_start: usize; pub static mut __kernel_io_start: usize;
    pub static mut __kernel_io_end: usize; pub static mut vmemmap: *mut page;
    pub static mut pci_io_base: usize;
    pub fn radix_enabled() -> bool;
    pub fn pte_update(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, clr: usize, set: usize, huge: i32) -> usize;
}

pub const FULL_IO_SIZE: usize = 0x80000000;
pub const FIXADDR_SIZE: usize = SZ_32M;

#[inline] pub unsafe fn pmd_leaf(pmd: pmd_t) -> bool { (pmd_raw(pmd) & cpu_to_be64(_PAGE_PTE)) != 0 }
#[inline] pub unsafe fn pud_leaf(pud: pud_t) -> bool { (pud_raw(pud) & cpu_to_be64(_PAGE_PTE)) != 0 }
#[inline] pub unsafe fn pmd_leaf_size(_pmd: pmd_t) -> usize { if IS_ENABLED(CONFIG_PPC_4K_PAGES) && !radix_enabled() { SZ_16M } else { PMD_SIZE } }
#[inline] pub unsafe fn pud_leaf_size(_pud: pud_t) -> usize { if IS_ENABLED(CONFIG_PPC_4K_PAGES) && !radix_enabled() { SZ_16G } else { PUD_SIZE } }

#[inline] pub unsafe fn pte_write(pte: pte_t) -> i32 { ((pte_raw(pte) & cpu_to_be64(_PAGE_WRITE)) != 0) as i32 }
#[inline] pub unsafe fn pte_read(pte: pte_t) -> i32 { ((pte_raw(pte) & cpu_to_be64(_PAGE_READ)) != 0) as i32 }
#[inline] pub unsafe fn pte_dirty(pte: pte_t) -> i32 { ((pte_raw(pte) & cpu_to_be64(_PAGE_DIRTY)) != 0) as i32 }
#[inline] pub unsafe fn pte_young(pte: pte_t) -> i32 { ((pte_raw(pte) & cpu_to_be64(_PAGE_ACCESSED)) != 0) as i32 }
#[inline] pub unsafe fn pte_special(pte: pte_t) -> i32 { ((pte_raw(pte) & cpu_to_be64(_PAGE_SPECIAL)) != 0) as i32 }
#[inline] pub unsafe fn pte_exec(pte: pte_t) -> bool { (pte_raw(pte) & cpu_to_be64(_PAGE_EXEC)) != 0 }
#[inline] pub unsafe fn pte_hw_valid(pte: pte_t) -> bool { (pte_raw(pte) & cpu_to_be64(_PAGE_PRESENT|_PAGE_PTE)) == cpu_to_be64(_PAGE_PRESENT|_PAGE_PTE) }
#[inline] pub unsafe fn pte_present(pte: pte_t) -> i32 { (pte_hw_valid(pte) || (pte_raw(pte)&cpu_to_be64(_PAGE_INVALID|_PAGE_PTE)) == cpu_to_be64(_PAGE_INVALID|_PAGE_PTE)) as i32 }
#[inline] pub unsafe fn pte_wrprotect(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) & cpu_to_be64(!_PAGE_WRITE)) }
#[inline] pub unsafe fn pte_exprotect(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) & cpu_to_be64(!_PAGE_EXEC)) }
#[inline] pub unsafe fn pte_mkclean(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) & cpu_to_be64(!_PAGE_DIRTY)) }
#[inline] pub unsafe fn pte_mkold(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) & cpu_to_be64(!_PAGE_ACCESSED)) }
#[inline] pub unsafe fn pte_mkexec(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) | cpu_to_be64(_PAGE_EXEC)) }
#[inline] pub unsafe fn pte_mkwrite_novma(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) | cpu_to_be64(_PAGE_RW)) }
#[inline] pub unsafe fn pte_mkdirty(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) | cpu_to_be64(_PAGE_DIRTY|_PAGE_SOFT_DIRTY)) }
#[inline] pub unsafe fn pte_mkyoung(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) | cpu_to_be64(_PAGE_ACCESSED)) }
#[inline] pub unsafe fn pte_mkspecial(pte: pte_t) -> pte_t { __pte_raw(pte_raw(pte) | cpu_to_be64(_PAGE_SPECIAL)) }
#[inline] pub unsafe fn pte_mkhuge(pte: pte_t) -> pte_t { pte }
#[inline] pub unsafe fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t { __pte_raw((pte_raw(pte)&cpu_to_be64(_PAGE_CHG_MASK))|cpu_to_be64(pgprot_val(newprot))) }

#[inline] pub unsafe fn pte_user(pte: pte_t) -> bool { (pte_raw(pte)&cpu_to_be64(_PAGE_PRIVILEGED)) == 0 }
#[inline] pub unsafe fn pte_access_permitted(pte: pte_t, write: bool) -> bool { pte_present(pte)!=0 && pte_user(pte) && pte_read(pte)!=0 && (!write || pte_write(pte)!=0) && arch_pte_access_permitted(pte_val(pte),write,false) }
#[inline] pub unsafe fn pte_user_accessible_page(_mm: *mut mm_struct, _addr: usize, pte: pte_t) -> bool { pte_present(pte)!=0 && pte_user(pte) }
#[inline] pub unsafe fn pmd_none(pmd: pmd_t) -> i32 { (pmd_raw(pmd)==0) as i32 }
#[inline] pub unsafe fn pud_none(pud: pud_t) -> i32 { (pud_raw(pud)==0) as i32 }
#[inline] pub unsafe fn pmd_present(pmd: pmd_t) -> i32 { ((pmd_raw(pmd)&cpu_to_be64(_PAGE_PRESENT|_PAGE_INVALID))!=0) as i32 }
#[inline] pub unsafe fn pud_present(pud: pud_t) -> i32 { ((pud_raw(pud)&cpu_to_be64(_PAGE_PRESENT))!=0) as i32 }
#[inline] pub unsafe fn pmd_write(pmd: pmd_t) -> i32 { pte_write(pmd_pte(pmd)) }
#[inline] pub unsafe fn pud_write(pud: pud_t) -> i32 { pte_write(pud_pte(pud)) }
#[inline] pub unsafe fn pmd_pte(pmd: pmd_t) -> pte_t { __pte_raw(pmd_raw(pmd)) }
#[inline] pub unsafe fn pte_pmd(pte: pte_t) -> pmd_t { __pmd_raw(pte_raw(pte)) }
#[inline] pub unsafe fn pud_pte(pud: pud_t) -> pte_t { __pte_raw(pud_raw(pud)) }
#[inline] pub unsafe fn pte_pud(pte: pte_t) -> pud_t { __pud_raw(pte_raw(pte)) }
#[inline] pub unsafe fn pmdp_ptep(pmd: *mut pmd_t) -> *mut pte_t { pmd as *mut pte_t }
#[inline] pub unsafe fn pudp_ptep(pud: *mut pud_t) -> *mut pte_t { pud as *mut pte_t }
#[inline] pub unsafe fn pmd_mkhuge(pmd: pmd_t) -> pmd_t { pmd }
#[inline] pub unsafe fn pud_mkhuge(pud: pud_t) -> pud_t { pud }

/* The remaining declarations are intentionally external: they are supplied
 * by the hash/radix page-table implementations and by generic MM headers. */
extern "C" {
    pub fn arch_pte_access_permitted(pte: u64, write: bool, execute: bool) -> bool;
    pub fn pmd_page(pmd: pmd_t) -> *mut page; pub fn pud_page(pud: pud_t) -> *mut page;
    pub fn p4d_page(p4d: p4d_t) -> *mut page;
    pub fn unmap_kernel_page(va: usize);
    pub fn pmdp_set_access_flags(vma: *mut vm_area_struct,address: usize,pmdp:*mut pmd_t,entry:pmd_t,dirty:i32)->i32;
    pub fn pudp_set_access_flags(vma: *mut vm_area_struct,address: usize,pudp:*mut pud_t,entry:pud_t,dirty:i32)->i32;
    pub fn pmdp_test_and_clear_young(vma:*mut vm_area_struct,address:usize,pmdp:*mut pmd_t)->bool;
    pub fn pudp_test_and_clear_young(vma:*mut vm_area_struct,address:usize,pudp:*mut pud_t)->bool;
    pub fn ptep_modify_prot_start(vma:*mut vm_area_struct,address:usize,ptep:*mut pte_t)->pte_t;
    pub fn ptep_modify_prot_commit(vma:*mut vm_area_struct,address:usize,ptep:*mut pte_t,old:pte_t,new:pte_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
