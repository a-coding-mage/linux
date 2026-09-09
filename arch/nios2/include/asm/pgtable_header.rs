/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 *
 * Based on asm/pgtable-32.h from mips.
 * This file is subject to the terms and conditions of the GNU General Public
 * License.
 */

// C dependencies: linux/io.h, linux/bug.h, asm/page.h,
// asm/cacheflush.h, asm/tlbflush.h, asm/pgtable-bits.h, and
// asm-generic/pgtable-nopmd.h.

pub const VMALLOC_START: usize = CONFIG_NIOS2_KERNEL_MMU_REGION_BASE;
pub const VMALLOC_END: usize = CONFIG_NIOS2_KERNEL_REGION_BASE - SZ_32M - 1;
pub const MODULES_VADDR: usize = CONFIG_NIOS2_KERNEL_REGION_BASE - SZ_32M;
pub const MODULES_END: usize = CONFIG_NIOS2_KERNEL_REGION_BASE - 1;

pub struct mm_struct;

#[inline]
pub const fn mkp(x: bool, w: bool, r: bool) -> pgprot_t {
    __pgprot(_PAGE_PRESENT | _PAGE_CACHED |
        if x { _PAGE_EXEC } else { 0 } |
        if r { _PAGE_READ } else { 0 } |
        if w { _PAGE_WRITE } else { 0 })
}

pub const PAGE_KERNEL: pgprot_t = __pgprot(
    _PAGE_PRESENT | _PAGE_CACHED | _PAGE_READ | _PAGE_WRITE | _PAGE_EXEC | _PAGE_GLOBAL);
pub const PAGE_SHARED: pgprot_t = __pgprot(
    _PAGE_PRESENT | _PAGE_CACHED | _PAGE_READ | _PAGE_WRITE | _PAGE_ACCESSED);
pub const PAGE_COPY: pgprot_t = mkp(false, false, true);

pub const PTRS_PER_PGD: usize = PAGE_SIZE / core::mem::size_of::<pgd_t>();
pub const PTRS_PER_PTE: usize = PAGE_SIZE / core::mem::size_of::<pte_t>();
pub const USER_PTRS_PER_PGD: usize = CONFIG_NIOS2_KERNEL_MMU_REGION_BASE / PGDIR_SIZE;
pub const PGDIR_SHIFT: usize = 22;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

unsafe extern "C" {
    pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD];
    pub static mut invalid_pte_table: [pte_t; PAGE_SIZE / core::mem::size_of::<pte_t>()];
    pub fn paging_init();
    pub fn mmu_init();
    pub fn update_mmu_cache_range(vmf: *mut vm_fault, vma: *mut vm_area_struct,
        address: usize, ptep: *mut pte_t, nr: u32);
}

#[inline]
pub unsafe fn set_pmd(pmdptr: *mut pmd_t, pmdval: pmd_t) { *pmdptr = pmdval; }

#[inline] pub fn pte_write(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_WRITE) as i32 }
#[inline] pub fn pte_dirty(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_DIRTY) as i32 }
#[inline] pub fn pte_young(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_ACCESSED) as i32 }

#[inline]
pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t {
    __pgprot(pgprot_val(prot) & !_PAGE_CACHED)
}

#[inline] pub fn pte_none(pte: pte_t) -> bool { (pte_val(pte) & !(_PAGE_GLOBAL | 0xf)) == 0 }
#[inline] pub fn pte_present(pte: pte_t) -> i32 { (pte_val(pte) & _PAGE_PRESENT) as i32 }

#[inline]
pub unsafe fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte).clone_from(&(pte_val(pte) & !_PAGE_WRITE)); pte }
#[inline]
pub unsafe fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte).clone_from(&(pte_val(pte) & !_PAGE_DIRTY)); pte }
#[inline]
pub unsafe fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte).clone_from(&(pte_val(pte) & !_PAGE_ACCESSED)); pte }
#[inline]
pub unsafe fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { *pte_val_mut(&mut pte) |= _PAGE_WRITE; pte }
#[inline]
pub unsafe fn pte_mkdirty(mut pte: pte_t) -> pte_t { *pte_val_mut(&mut pte) |= _PAGE_DIRTY; pte }
#[inline]
pub unsafe fn pte_mkyoung(mut pte: pte_t) -> pte_t { *pte_val_mut(&mut pte) |= _PAGE_ACCESSED; pte }

#[inline]
pub unsafe fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    let mask = _PAGE_READ | _PAGE_WRITE | _PAGE_EXEC;
    *pte_val_mut(&mut pte) = (pte_val(pte) & !mask) | (pgprot_val(newprot) & mask);
    pte
}

#[inline]
pub fn pmd_present(pmd: pmd_t) -> bool {
    pmd_val(pmd) != invalid_pte_table.as_ptr() as usize && pmd_val(pmd) != 0
}
#[inline]
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { *pmd_val_mut(&mut *pmdp) = invalid_pte_table.as_ptr() as usize; }

#[inline] pub fn pte_pfn(pte: pte_t) -> usize { pte_val(pte) & 0xfffff }
#[inline] pub fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t { __pte(pfn | pgprot_val(prot)) }
#[inline] pub fn pte_page(pte: pte_t) -> *mut page { pfn_to_page(pte_pfn(pte)) }

#[inline] pub unsafe fn set_pte(ptep: *mut pte_t, pteval: pte_t) { *ptep = pteval; }
pub const PFN_PTE_SHIFT: usize = 0;

pub unsafe fn set_ptes(_mm: *mut mm_struct, _addr: usize, mut ptep: *mut pte_t,
    mut pte: pte_t, mut nr: u32) {
    let paddr = page_to_virt(pte_page(pte)) as usize;
    flush_dcache_range(paddr, paddr + nr as usize * PAGE_SIZE);
    loop {
        set_pte(ptep, pte);
        nr -= 1;
        if nr == 0 { break; }
        ptep = ptep.add(1);
        *pte_val_mut(&mut pte) += 1;
    }
}

#[inline] pub fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == invalid_pte_table.as_ptr() as usize || pmd_val(pmd) == 0 }
#[inline] pub fn pmd_bad(pmd: pmd_t) -> usize { pmd_val(pmd) & !PAGE_MASK }

pub unsafe fn pte_clear(_mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) {
    let mut null = core::mem::zeroed::<pte_t>();
    *pte_val_mut(&mut null) = (addr >> PAGE_SHIFT) & 0xf;
    set_pte(ptep, null);
}

#[inline] pub fn pmd_phys(pmd: pmd_t) -> usize { virt_to_phys(pmd_val(pmd) as *mut core::ffi::c_void) }
#[inline] pub fn pmd_pfn(pmd: pmd_t) -> usize { pmd_phys(pmd) >> PAGE_SHIFT }
#[inline] pub fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(pmd_phys(pmd) >> PAGE_SHIFT) }
#[inline] pub fn pmd_page_vaddr(pmd: pmd_t) -> usize { pmd_val(pmd) }

#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_SWP_EXCLUSIVE) != 0 }
#[inline] pub unsafe fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { *pte_val_mut(&mut pte) |= _PAGE_SWP_EXCLUSIVE; pte }
#[inline] pub unsafe fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { *pte_val_mut(&mut pte) &= !_PAGE_SWP_EXCLUSIVE; pte }

#[inline]
pub unsafe fn ptep_set_access_flags(vma: *mut vm_area_struct, address: usize,
    ptep: *mut pte_t, entry: pte_t, _dirty: i32) -> bool {
    if !pte_same(*ptep, entry) { set_ptes((*vma).vm_mm, address, ptep, entry, 1); }
    true
}

// The following C macros are represented by the corresponding Rust operations:
// __swp_type(swp) = ((swp.val >> 26) & 0x1f)
// __swp_offset(swp) = (swp.val & 0xfffff)
// __swp_entry(type, off) = swap_entry_t { val: (((type & 0x1f) << 26) | (off & 0xfffff)) }
// __swp_entry_to_pte(swp) = pte_t { val: swp.val }
// __pte_to_swp_entry(pte) = swap_entry_t { val: pte_val(pte) }
// update_mmu_cache(vma, addr, ptep) = update_mmu_cache_range(null_mut(), vma, addr, ptep, 1)

unsafe extern "C" {
    fn pte_same(a: pte_t, b: pte_t) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
