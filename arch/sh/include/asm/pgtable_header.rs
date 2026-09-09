/* SPDX-License-Identifier: GPL-2.0
 *
 * This file contains the functions and defines necessary to modify and
 * use the SuperH page table tree.
 *
 * Copyright (C) 1999 Niibe Yutaka
 * Copyright (C) 2002 - 2007 Paul Mundt
 */

// Dependencies supplied by the corresponding architecture headers:
// pgtable-3level.h / pgtable-2level.h, page.h, mmu.h, addrspace.h,
// fixmap.h, and pgtable_32.h.

pub const NEFF: u32 = 32;
pub const NEFF_SIGN: u64 = 1u64 << (NEFF - 1);
pub const NEFF_MASK: u64 = (!0u64) << NEFF;

pub unsafe fn neff_sign_extend(val: usize) -> u64 {
    let extended = val as u64;
    if (extended & NEFF_SIGN) != 0 {
        extended | NEFF_MASK
    } else {
        extended
    }
}

// CONFIG_29BIT selects the physical address width at build time.
#[cfg(CONFIG_29BIT)]
pub const NPHYS: u32 = 29;
#[cfg(not(CONFIG_29BIT))]
pub const NPHYS: u32 = 32;

pub const NPHYS_SIGN: u64 = 1u64 << (NPHYS - 1);
pub const NPHYS_MASK: u64 = (!0u64) << NPHYS;

pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

/* Entries per level */
pub const PTRS_PER_PTE: usize = PAGE_SIZE / (1usize << PTE_MAGNITUDE);

pub const PHYS_ADDR_MASK29: usize = 0x1fffffff;
pub const PHYS_ADDR_MASK32: usize = 0xffffffff;

pub unsafe fn phys_addr_mask() -> usize {
    /* Is the MMU in 29bit mode? */
    if __in_29bit_mode() {
        PHYS_ADDR_MASK29
    } else {
        PHYS_ADDR_MASK32
    }
}

// These macros depend on the externally supplied PAGE_MASK and phys_addr_mask.
pub const VMALLOC_START: usize = P3SEG;
pub const VMALLOC_END: usize = FIXADDR_START - 2 * PAGE_SIZE;

/*
 * SH-X and lower (legacy) SuperH parts (SH-3, SH-4, some SH-4A) can't do page
 * protection for execute, and considers it the same as a read. Also, write
 * permission implies read permission. This is the closest we can get..
 *
 * SH-X2 (SH7785) and later parts take this to the opposite end of the extreme,
 * not only supporting separate execute, read, and write bits, but having
 * completely separate permission bits for user and kernel space.
 */
/*xwr*/

pub type pte_addr_t = *mut pte_t;

pub unsafe fn pte_pfn(x: &pte_t) -> usize {
    (x.pte_low >> PAGE_SHIFT) as usize
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn __update_cache(vma: *mut vm_area_struct, address: usize, pte: pte_t);
    pub fn __update_tlb(vma: *mut vm_area_struct, address: usize, pte: pte_t);
}

pub unsafe fn update_mmu_cache_range(
    vmf: *mut vm_fault,
    vma: *mut vm_area_struct,
    address: usize,
    ptep: *mut pte_t,
    nr: u32,
) {
    let pte = *ptep;
    __update_cache(vma, address, pte);
    __update_tlb(vma, address, pte);
    let _ = (vmf, nr);
}

#[macro_export]
macro_rules! update_mmu_cache {
    ($vma:expr, $addr:expr, $ptep:expr) => {
        update_mmu_cache_range(core::ptr::null_mut(), $vma, $addr, $ptep, 1)
    };
}

extern "C" {
    pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD];
    pub fn paging_init();
    pub fn page_table_range_init(start: usize, end: usize, pgd: *mut pgd_t);
}

pub unsafe fn __pte_access_permitted(pte: pte_t, prot: u64) -> bool {
    (pte_val(pte) & (prot | _PAGE_SPECIAL)) == prot
}

// CONFIG_X2TLB selects the protection-bit layout at build time.
#[cfg(CONFIG_X2TLB)]
pub unsafe fn pte_access_permitted(pte: pte_t, write: bool) -> bool {
    let mut prot = _PAGE_PRESENT;
    prot |= _PAGE_EXT(_PAGE_EXT_KERN_READ | _PAGE_EXT_USER_READ);
    if write {
        prot |= _PAGE_EXT(_PAGE_EXT_KERN_WRITE | _PAGE_EXT_USER_WRITE);
    }
    __pte_access_permitted(pte, prot)
}

#[cfg(not(CONFIG_X2TLB))]
pub unsafe fn pte_access_permitted(pte: pte_t, write: bool) -> bool {
    let mut prot = _PAGE_PRESENT | _PAGE_USER;
    if write {
        prot |= _PAGE_RW;
    }
    __pte_access_permitted(pte, prot)
}

pub const HAVE_ARCH_UNMAPPED_AREA: bool = true;
pub const HAVE_ARCH_UNMAPPED_AREA_TOPDOWN: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
