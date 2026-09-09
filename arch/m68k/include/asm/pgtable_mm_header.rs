/* SPDX-License-Identifier: GPL-2.0 */

// Conditional dependencies from the C header are supplied by other Rust units.
// CONFIG_SUN3 / CONFIG_COLDFIRE select the corresponding architecture setup.

/*
 * This file contains the functions and defines necessary to modify and use
 * the m68k page table tree.
 */

// C: #define set_pte(pteptr, pteval) do { *(pteptr) = (pteval); } while (0)
#[inline]
pub unsafe fn set_pte<T>(pteptr: *mut T, pteval: T) {
    *pteptr = pteval;
}

/* PMD_SHIFT determines the size of the area a second-level page table can map. */
#[cfg(CONFIG_PGTABLE_LEVELS_3)]
pub const PMD_SHIFT: usize = 18;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);

/* PGDIR_SHIFT determines what a third-level page table entry can map. */
#[cfg(CONFIG_SUN3)]
pub const PGDIR_SHIFT: usize = 17;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const PGDIR_SHIFT: usize = 22;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE)))]
pub const PGDIR_SHIFT: usize = 25;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

/*
 * entries per page directory level: the m68k is configured as three-level,
 * so we do have PMD level physically.
 */
#[cfg(CONFIG_SUN3)]
pub const PTRS_PER_PTE: usize = 16;
#[cfg(CONFIG_SUN3)]
pub const __PAGETABLE_PMD_FOLDED: usize = 1;
#[cfg(CONFIG_SUN3)]
pub const PTRS_PER_PMD: usize = 1;
#[cfg(CONFIG_SUN3)]
pub const PTRS_PER_PGD: usize = 2048;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const PTRS_PER_PTE: usize = 512;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const __PAGETABLE_PMD_FOLDED: usize = 1;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const PTRS_PER_PMD: usize = 1;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const PTRS_PER_PGD: usize = 1024;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE)))]
pub const PTRS_PER_PTE: usize = 64;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE)))]
pub const PTRS_PER_PMD: usize = 128;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE)))]
pub const PTRS_PER_PGD: usize = 128;
pub const USER_PTRS_PER_PGD: usize = TASK_SIZE / PGDIR_SIZE;

/* Virtual address region for use by kernel_map(). */
#[cfg(CONFIG_SUN3)]
pub const KMAP_START: usize = 0x0dc00000;
#[cfg(CONFIG_SUN3)]
pub const KMAP_END: usize = 0x0e000000;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const KMAP_START: usize = 0xe0000000;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const KMAP_END: usize = 0xf0000000;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), CONFIG_VIRT))]
pub const KMAP_START: usize = 0xdf000000;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), CONFIG_VIRT))]
pub const KMAP_END: usize = 0xff000000;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), not(CONFIG_VIRT)))]
pub const KMAP_START: usize = 0xd0000000;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), not(CONFIG_VIRT)))]
pub const KMAP_END: usize = 0xf0000000;

#[cfg(CONFIG_SUN3)]
unsafe extern "C" {
    pub static mut m68k_vmalloc_end: usize;
}

#[cfg(CONFIG_SUN3)]
pub const VMALLOC_START: usize = 0x0f800000;
#[cfg(CONFIG_SUN3)]
pub const VMALLOC_END: usize = unsafe { m68k_vmalloc_end };
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const VMALLOC_START: usize = 0xd0000000;
#[cfg(all(not(CONFIG_SUN3), CONFIG_COLDFIRE))]
pub const VMALLOC_END: usize = 0xe0000000;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), CONFIG_VIRT))]
pub const VMALLOC_OFFSET: usize = PAGE_SIZE;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), CONFIG_VIRT))]
pub const VMALLOC_START: usize = ((high_memory as usize + VMALLOC_OFFSET) & !(VMALLOC_OFFSET - 1));
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), CONFIG_VIRT))]
pub const VMALLOC_END: usize = KMAP_START;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), not(CONFIG_VIRT)))]
pub const VMALLOC_OFFSET: usize = 8 * 1024 * 1024;
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), not(CONFIG_VIRT)))]
pub const VMALLOC_START: usize = ((high_memory as usize + VMALLOC_OFFSET) & !(VMALLOC_OFFSET - 1));
#[cfg(all(not(CONFIG_SUN3), not(CONFIG_COLDFIRE), not(CONFIG_VIRT)))]
pub const VMALLOC_END: usize = KMAP_START;

pub unsafe extern "C" {
    pub fn kernel_set_cachemode(addr: *mut core::ffi::c_void, size: usize, cmode: i32);
}

pub unsafe fn update_mmu_cache_range(
    _vmf: *mut vm_fault,
    _vma: *mut vm_area_struct,
    _address: usize,
    _ptep: *mut pte_t,
    _nr: u32,
) {
}

#[inline]
pub unsafe fn update_mmu_cache(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t) {
    update_mmu_cache_range(core::ptr::null_mut(), vma, addr, ptep, 1);
}

/* MMU-specific headers are supplied by the selected architecture dependency. */

/* Macro to mark a page protection value as "uncacheable". */
#[cfg(CONFIG_COLDFIRE)]
#[inline]
pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t {
    __pgprot(pgprot_val(prot) | CF_PAGE_NOCACHE)
}

#[cfg(not(CONFIG_COLDFIRE))]
pub const __SUN3_PAGE_NOCACHE: usize = 0;

#[cfg(not(CONFIG_COLDFIRE))]
pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t {
    if MMU_IS_SUN3 {
        __pgprot(pgprot_val(prot) | __SUN3_PAGE_NOCACHE)
    } else if MMU_IS_851 || MMU_IS_030 {
        __pgprot(pgprot_val(prot) | _PAGE_NOCACHE030)
    } else if MMU_IS_040 || MMU_IS_060 {
        __pgprot((pgprot_val(prot) & _CACHEMASK040) | _PAGE_NOCACHE_S)
    } else {
        prot
    }
}

#[cfg(not(CONFIG_COLDFIRE))]
unsafe extern "C" {
    pub fn pgprot_dmacoherent(prot: pgprot_t) -> pgprot_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
