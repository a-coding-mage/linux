/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

pub const PGDIR_SHIFT: u32 = 22;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

pub const USER_PTRS_PER_PGD: usize = PAGE_OFFSET / PGDIR_SIZE;
pub const PTRS_PER_PGD: usize = PAGE_SIZE / core::mem::size_of::<pgd_t>();
pub const PTRS_PER_PMD: usize = 1;
pub const PTRS_PER_PTE: usize = PAGE_SIZE / core::mem::size_of::<pte_t>();
pub const PFN_PTE_SHIFT: u32 = PAGE_SHIFT;

#[inline]
pub fn pmd_pfn(pmd: pmd_t) -> usize { pmd_phys(pmd) >> PAGE_SHIFT }
#[inline]
pub fn pmd_page(pmd: pmd_t) -> Page { pfn_to_page(pmd_phys(pmd) >> PAGE_SHIFT) }
#[inline]
pub unsafe fn pte_clear(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) {
    set_pte(ptep, if addr as u32 >= PAGE_OFFSET as u32 { __pte(_PAGE_GLOBAL) } else { __pte(0) });
}
#[inline]
pub fn pte_none(pte: pte_t) -> bool { (pte_val(pte) & !_PAGE_GLOBAL) == 0 }
#[inline]
pub fn pte_present(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_PRESENT) != 0 }
#[inline]
pub fn pte_pfn(x: pte_t) -> usize { (x.pte_low as usize) >> PAGE_SHIFT }
#[inline]
pub fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t {
    __pte(((pfn as u64) << PAGE_SHIFT) | pgprot_val(prot) as u64)
}
#[inline]
pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline]
pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { pte_t { pte_low: x.val } }
#[inline]
pub fn pte_page(x: pte_t) -> Page { pfn_to_page(pte_pfn(x)) }
#[inline]
pub fn __mk_pte(page_nr: usize, pgprot: pgprot_t) -> pte_t { __pte((page_nr << PAGE_SHIFT) | pgprot_val(pgprot) as usize) }

pub const _PAGE_BASE: usize = _PAGE_PRESENT | _PAGE_ACCESSED;
pub const PAGE_NONE: pgprot_t = __pgprot(_PAGE_PROT_NONE);
pub const PAGE_READ: pgprot_t = __pgprot(_PAGE_BASE | _PAGE_READ | _CACHE_CACHED);
pub const PAGE_WRITE: pgprot_t = __pgprot(_PAGE_BASE | _PAGE_READ | _PAGE_WRITE | _CACHE_CACHED);
pub const PAGE_SHARED: pgprot_t = PAGE_WRITE;
pub const PAGE_KERNEL: pgprot_t = __pgprot(_PAGE_BASE | _PAGE_READ | _PAGE_VALID | _PAGE_WRITE | _PAGE_DIRTY | _PAGE_MODIFIED | _PAGE_GLOBAL | _CACHE_CACHED);
pub const _PAGE_IOREMAP: usize = _PAGE_BASE | _PAGE_READ | _PAGE_VALID | _PAGE_WRITE | _PAGE_DIRTY | _PAGE_MODIFIED | _PAGE_GLOBAL | _CACHE_UNCACHED | _PAGE_SO;
pub const _PAGE_CHG_MASK: usize = !(_PAGE_PRESENT | _PAGE_READ | _PAGE_WRITE | _CACHE_MASK | _PAGE_GLOBAL);

pub unsafe extern "C" { pub fn load_pgd(pg_dir: usize); pub static mut invalid_pte_table: [pte_t; PTRS_PER_PTE]; }

#[inline]
pub unsafe fn set_pte(p: *mut pte_t, pte: pte_t) { *p = pte; #[cfg(CONFIG_CPU_NEED_TLBSYNC)] dcache_wb_line(p as u32); smp_mb(); }
#[inline]
pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> *mut core::ffi::c_void { __va(pmd_val(pmd)) }
#[inline] pub fn pmd_phys(pmd: pmd_t) -> usize { pmd_val(pmd) }
#[inline]
pub unsafe fn set_pmd(p: *mut pmd_t, pmd: pmd_t) { *p = pmd; #[cfg(CONFIG_CPU_NEED_TLBSYNC)] dcache_wb_line(p as u32); smp_mb(); }
#[inline] pub unsafe fn pmd_none(pmd: pmd_t) -> bool { pmd_val(pmd) == __pa(invalid_pte_table.as_ptr()) }
#[inline] pub fn pmd_bad(pmd: pmd_t) -> bool { (pmd_val(pmd) & !PAGE_MASK) != 0 }
#[inline] pub unsafe fn pmd_present(pmd: pmd_t) -> bool { pmd_val(pmd) != __pa(invalid_pte_table.as_ptr()) }
#[inline] pub unsafe fn pmd_clear(p: *mut pmd_t) { pmd_val_mut(&mut *p, __pa(invalid_pte_table.as_ptr())); #[cfg(CONFIG_CPU_NEED_TLBSYNC)] dcache_wb_line(p as u32); }

#[inline] pub fn pte_read(pte: pte_t) -> bool { (pte.pte_low & _PAGE_READ) != 0 }
#[inline] pub fn pte_write(pte: pte_t) -> bool { (pte.pte_low & _PAGE_WRITE) != 0 }
#[inline] pub fn pte_dirty(pte: pte_t) -> bool { (pte.pte_low & _PAGE_MODIFIED) != 0 }
#[inline] pub fn pte_young(pte: pte_t) -> bool { (pte.pte_low & _PAGE_ACCESSED) != 0 }
#[inline] pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte.pte_low &= !(_PAGE_WRITE | _PAGE_DIRTY); pte }
#[inline] pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte.pte_low &= !(_PAGE_MODIFIED | _PAGE_DIRTY); pte }
#[inline] pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte.pte_low &= !(_PAGE_ACCESSED | _PAGE_VALID); pte }
#[inline] pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_WRITE; if pte.pte_low & _PAGE_MODIFIED != 0 { pte.pte_low |= _PAGE_DIRTY; } pte }
#[inline] pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_MODIFIED; if pte.pte_low & _PAGE_WRITE != 0 { pte.pte_low |= _PAGE_DIRTY; } pte }
#[inline] pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_ACCESSED; if pte.pte_low & _PAGE_READ != 0 { pte.pte_low |= _PAGE_VALID; } pte }
#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE != 0 }
#[inline] pub fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte.pte_low |= _PAGE_SWP_EXCLUSIVE; pte }
#[inline] pub fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte.pte_low &= !_PAGE_SWP_EXCLUSIVE; pte }

pub struct file;
pub const __HAVE_PHYS_MEM_ACCESS_PROT: bool = true;
pub unsafe extern "C" { pub fn phys_mem_access_prot(file: *mut file, pfn: usize, size: usize, vma_prot: pgprot_t) -> pgprot_t; }
#[inline] pub fn pgprot_noncached(mut prot: pgprot_t) -> pgprot_t { let mut v = pgprot_val(prot); v = (v & !_CACHE_MASK) | _CACHE_UNCACHED | _PAGE_SO; __pgprot(v) }
#[inline] pub fn pgprot_writecombine(mut prot: pgprot_t) -> pgprot_t { let mut v = pgprot_val(prot); v = (v & !_CACHE_MASK) | _CACHE_UNCACHED; __pgprot(v) }
#[inline] pub fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t { __pte((pte_val(pte) & _PAGE_CHG_MASK) | pgprot_val(newprot)) }

pub unsafe extern "C" { pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD]; pub fn paging_init(); pub fn update_mmu_cache_range(vmf: *mut vm_fault, vma: *mut vm_area_struct, address: usize, pte: *mut pte_t, nr: u32); }
#[inline] pub unsafe fn update_mmu_cache(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t) { update_mmu_cache_range(core::ptr::null_mut(), vma, addr, ptep, 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
