/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from include/asm-xtensa/pgtable.h. */

// External architecture/build symbols supplied by surrounding kernel code.

#[cfg(feature = "mmu")]
pub const USER_RING: usize = 1;
#[cfg(not(feature = "mmu"))]
pub const USER_RING: usize = 0;
pub const KERNEL_RING: usize = 0;

pub const PGDIR_SHIFT: usize = 22;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const PTRS_PER_PTE: usize = 1024;
pub const PTRS_PER_PTE_SHIFT: usize = 10;
pub const PTRS_PER_PGD: usize = 1024;

// USER_PTRS_PER_PGD is TASK_SIZE / PGDIR_SIZE.

#[cfg(feature = "mmu")]
pub const _PAGE_ATTRIB_MASK: usize = 0xf;
pub const _PAGE_HW_EXEC: usize = 1 << 0;
pub const _PAGE_HW_WRITE: usize = 1 << 1;
pub const _PAGE_CA_BYPASS: usize = 0 << 2;
pub const _PAGE_CA_WB: usize = 1 << 2;
pub const _PAGE_CA_WT: usize = 2 << 2;
pub const _PAGE_CA_MASK: usize = 3 << 2;
pub const _PAGE_CA_INVALID: usize = 3 << 2;

// The original condition is XCHAL_HW_VERSION_MAJOR < 2000.
#[cfg(feature = "xtensa_hw_before_2000")]
pub const _PAGE_HW_VALID: usize = 0x01;
#[cfg(feature = "xtensa_hw_before_2000")]
pub const _PAGE_NONE: usize = 0x04;
#[cfg(not(feature = "xtensa_hw_before_2000"))]
pub const _PAGE_HW_VALID: usize = 0x00;
#[cfg(not(feature = "xtensa_hw_before_2000"))]
pub const _PAGE_NONE: usize = 0x0f;

pub const _PAGE_USER: usize = 1 << 4;
pub const _PAGE_WRITABLE_BIT: usize = 6;
pub const _PAGE_WRITABLE: usize = 1 << 6;
pub const _PAGE_DIRTY: usize = 1 << 7;
pub const _PAGE_ACCESSED: usize = 1 << 8;
pub const _PAGE_SWP_EXCLUSIVE: usize = 1 << 1;

#[cfg(feature = "mmu")]
pub const _PAGE_CHG_MASK: usize = PAGE_MASK | _PAGE_ACCESSED | _PAGE_DIRTY;
#[cfg(feature = "mmu")]
pub const _PAGE_PRESENT: usize = _PAGE_HW_VALID | _PAGE_CA_WB | _PAGE_ACCESSED;

#[cfg(feature = "mmu")]
pub const VMALLOC_START: usize = XCHAL_KSEG_CACHED_VADDR - 0x10000000;
#[cfg(feature = "mmu")]
pub const VMALLOC_END: usize = VMALLOC_START + 0x07feffff;
#[cfg(not(feature = "mmu"))]
pub const VMALLOC_START: usize = 0;
#[cfg(not(feature = "mmu"))]
pub const VMALLOC_END: usize = 0xffffffff;

#[cfg(feature = "mmu")]
pub const PAGE_NONE: pgprot_t = __pgprot(_PAGE_NONE | _PAGE_USER);
#[cfg(feature = "mmu")]
pub const PAGE_COPY: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_USER);
#[cfg(feature = "mmu")]
pub const PAGE_COPY_EXEC: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_HW_EXEC);
#[cfg(feature = "mmu")]
pub const PAGE_READONLY: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_USER);
#[cfg(feature = "mmu")]
pub const PAGE_READONLY_EXEC: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_HW_EXEC);
#[cfg(feature = "mmu")]
pub const PAGE_SHARED: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_WRITABLE);
#[cfg(feature = "mmu")]
pub const PAGE_SHARED_EXEC: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_WRITABLE | _PAGE_HW_EXEC);
#[cfg(feature = "mmu")]
pub const PAGE_KERNEL: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_HW_WRITE);
#[cfg(feature = "mmu")]
pub const PAGE_KERNEL_RO: pgprot_t = __pgprot(_PAGE_PRESENT);
#[cfg(feature = "mmu")]
pub const PAGE_KERNEL_EXEC: pgprot_t = __pgprot(_PAGE_PRESENT | _PAGE_HW_WRITE | _PAGE_HW_EXEC);

pub const PFN_PTE_SHIFT: usize = PAGE_SHIFT;

pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> usize { (pmd_val(pmd) & PAGE_MASK) as usize }
pub unsafe fn pmd_pfn(pmd: pmd_t) -> usize { (__pa(pmd_val(pmd)) >> PAGE_SHIFT) as usize }
pub unsafe fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == 0 }
pub unsafe fn pmd_present(pmd: pmd_t) -> usize { pmd_val(pmd) & PAGE_MASK }
pub unsafe fn pmd_bad(pmd: pmd_t) -> usize { pmd_val(pmd) & !PAGE_MASK }
pub unsafe fn pte_none(pte: pte_t) -> bool { pte_val(pte) == (_PAGE_CA_INVALID | _PAGE_USER) }

pub unsafe fn pte_present(pte: pte_t) -> bool {
    (pte_val(pte) & _PAGE_CA_MASK) != _PAGE_CA_INVALID ||
        (pte_val(pte) & _PAGE_ATTRIB_MASK) == _PAGE_NONE
}
pub unsafe fn pte_write(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_WRITABLE) as i32 }
pub unsafe fn pte_dirty(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_DIRTY) as i32 }
pub unsafe fn pte_young(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_ACCESSED) as i32 }

pub unsafe fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) &= !(_PAGE_WRITABLE | _PAGE_HW_WRITE); pte }
pub unsafe fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) &= !(_PAGE_DIRTY | _PAGE_HW_WRITE); pte }
pub unsafe fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) &= !_PAGE_ACCESSED; pte }
pub unsafe fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_DIRTY; pte }
pub unsafe fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_ACCESSED; pte }
pub unsafe fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_WRITABLE; pte }

pub unsafe fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t {
    __pte((pte_val(pte) & _PAGE_CHG_MASK) | pgprot_val(newprot))
}
pub unsafe fn update_pte(ptep: *mut pte_t, pteval: pte_t) { *ptep = pteval; }
pub unsafe fn set_pte(ptep: *mut pte_t, pte: pte_t) { update_pte(ptep, pte); }
pub unsafe fn set_pmd(pmdp: *mut pmd_t, pmdval: pmd_t) { *pmdp = pmdval; }

pub unsafe fn ptep_test_and_clear_young(_vma: *mut vm_area_struct, _addr: usize, ptep: *mut pte_t) -> bool {
    let pte = *ptep;
    if pte_young(pte) == 0 { return false; }
    update_pte(ptep, pte_mkold(pte)); true
}
pub unsafe fn ptep_get_and_clear(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) -> pte_t {
    let pte = *ptep; update_pte(ptep, __pte(_PAGE_CA_INVALID | _PAGE_USER)); pte
}
pub unsafe fn ptep_set_wrprotect(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) {
    let pte = *ptep; update_pte(ptep, pte_wrprotect(pte));
}

pub unsafe fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE != 0 }
pub unsafe fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) |= _PAGE_SWP_EXCLUSIVE; pte }
pub unsafe fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte) &= !_PAGE_SWP_EXCLUSIVE; pte }

// Architecture feature declarations from the original header.
pub const __HAVE_ARCH_PTEP_TEST_AND_CLEAR_YOUNG: bool = true;
pub const __HAVE_ARCH_PTEP_GET_AND_CLEAR: bool = true;
pub const __HAVE_ARCH_PTEP_SET_WRPROTECT: bool = true;
pub const __HAVE_ARCH_PTEP_MKDIRTY: bool = true;
pub const __HAVE_ARCH_PTE_SAME: bool = true;
pub const HAVE_ARCH_UNMAPPED_AREA: bool = true;

// External types and functions referenced by this header are provided by the kernel translation.
extern "C" {
    static mut swapper_pg_dir: pgd_t;
    fn paging_init();
    fn pte_val(pte: pte_t) -> usize;
    fn pte_val_mut(pte: *mut pte_t) -> *mut usize;
    fn pgprot_val(prot: pgprot_t) -> usize;
    fn __pte(value: usize) -> pte_t;
    fn __pgprot(value: usize) -> pgprot_t;
    fn pmd_val(pmd: pmd_t) -> usize;
    fn __pa(value: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
