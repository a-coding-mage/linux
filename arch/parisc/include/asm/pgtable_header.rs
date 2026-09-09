/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from parisc/include/asm/pgtable.h. Includes and build-time
 * configuration are supplied by the surrounding kernel translation. */

/* we simulate an x86-style page table for the linux mm code */

extern "C" {
    pub static mut pa_tlb_flush_lock: spinlock_t;
    #[cfg(all(feature = "CONFIG_64BIT", feature = "CONFIG_SMP"))]
    pub static mut pa_serialize_tlb_flushes: ::core::ffi::c_int;
    pub fn __update_cache(pte: pte_t);
    pub fn paging_init();
    pub static mut swapper_pg_dir: [pgd_t; 0];
    pub static mut pg0: [pte_t; 0];
    pub fn ptep_clear_flush_young(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t) -> bool;
    pub fn ptep_clear_flush(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t;
}

/* External kernel dependencies. */
#[allow(non_camel_case_types)] pub type c_ulong = usize;
pub type spinlock_t = u8;
pub type mm_struct = ::core::ffi::c_void;
pub type vm_area_struct = ::core::ffi::c_void;
pub type pgd_t = u64;
pub type pmd_t = u64;
pub type pud_t = u64;
pub type pte_t = u64;
pub type pgprot_t = u64;
#[repr(C)] pub struct swp_entry_t { pub val: c_ulong }

extern "C" {
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn mtsp(space: c_ulong, reg: c_ulong);
    fn pdtlb(reg: c_ulong, addr: c_ulong);
    fn pitlb(reg: c_ulong, addr: c_ulong);
    fn set_pmd(pmd: *mut pmd_t, val: pmd_t);
    fn set_pud(pud: *mut pud_t, val: pud_t);
    fn set_pte_at(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, pte: pte_t);
    fn __va(addr: c_ulong) -> *mut ::core::ffi::c_void;
    fn virt_to_page(addr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn pfn_to_page(pfn: c_ulong) -> *mut ::core::ffi::c_void;
    fn parisc_requires_coherency() -> bool;
}

pub const KERNEL_INITIAL_ORDER: usize = 26;
pub const KERNEL_INITIAL_SIZE: usize = 1usize << KERNEL_INITIAL_ORDER;
pub const PLD_SHIFT: usize = PAGE_SHIFT;
pub const PLD_SIZE: usize = PAGE_SIZE;
pub const BITS_PER_PTE: usize = PAGE_SHIFT - BITS_PER_PTE_ENTRY;
pub const PTRS_PER_PTE: usize = 1usize << BITS_PER_PTE;
pub const BITS_PER_PMD: usize = 0;
pub const PGDIR_SHIFT: usize = PLD_SHIFT + BITS_PER_PTE + BITS_PER_PMD;
pub const BITS_PER_PGD: usize = PAGE_SHIFT + PGD_TABLE_ORDER - BITS_PER_PGD_ENTRY;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const PTRS_PER_PGD: usize = 1usize << BITS_PER_PGD;
pub const USER_PTRS_PER_PGD: usize = PTRS_PER_PGD;
pub const MAX_ADDRBITS: usize = BITS_PER_LONG;
pub const MAX_ADDRESS: u64 = 1u64 << MAX_ADDRBITS;
pub const SPACEID_SHIFT: usize = 0;
pub const PT_INITIAL: usize = 1;

pub const _PAGE_READ_BIT: usize = 31;
pub const _PAGE_WRITE_BIT: usize = 30;
pub const _PAGE_EXEC_BIT: usize = 29;
pub const _PAGE_GATEWAY_BIT: usize = 28;
pub const _PAGE_DMB_BIT: usize = 27;
pub const _PAGE_DIRTY_BIT: usize = 26;
pub const _PAGE_REFTRAP_BIT: usize = 25;
pub const _PAGE_NO_CACHE_BIT: usize = 24;
pub const _PAGE_ACCESSED_BIT: usize = 23;
pub const _PAGE_PRESENT_BIT: usize = 22;
pub const _PAGE_HPAGE_BIT: usize = 21;
pub const _PAGE_USER_BIT: usize = 20;
pub const _PAGE_SPECIAL_BIT: usize = _PAGE_HPAGE_BIT;
pub const fn xlate_pabit(x: usize) -> usize { 31 - x }
pub const PTE_SHIFT: usize = xlate_pabit(_PAGE_USER_BIT);
pub const PFN_PTE_SHIFT: usize = 12;
pub const _PAGE_READ: u64 = 1u64 << xlate_pabit(_PAGE_READ_BIT);
pub const _PAGE_WRITE: u64 = 1u64 << xlate_pabit(_PAGE_WRITE_BIT);
pub const _PAGE_RW: u64 = _PAGE_READ | _PAGE_WRITE;
pub const _PAGE_EXEC: u64 = 1u64 << xlate_pabit(_PAGE_EXEC_BIT);
pub const _PAGE_GATEWAY: u64 = 1u64 << xlate_pabit(_PAGE_GATEWAY_BIT);
pub const _PAGE_DMB: u64 = 1u64 << xlate_pabit(_PAGE_DMB_BIT);
pub const _PAGE_DIRTY: u64 = 1u64 << xlate_pabit(_PAGE_DIRTY_BIT);
pub const _PAGE_REFTRAP: u64 = 1u64 << xlate_pabit(_PAGE_REFTRAP_BIT);
pub const _PAGE_NO_CACHE: u64 = 1u64 << xlate_pabit(_PAGE_NO_CACHE_BIT);
pub const _PAGE_ACCESSED: u64 = 1u64 << xlate_pabit(_PAGE_ACCESSED_BIT);
pub const _PAGE_PRESENT: u64 = 1u64 << xlate_pabit(_PAGE_PRESENT_BIT);
pub const _PAGE_HUGE: u64 = 1u64 << xlate_pabit(_PAGE_HPAGE_BIT);
pub const _PAGE_USER: u64 = 1u64 << xlate_pabit(_PAGE_USER_BIT);
pub const _PAGE_SPECIAL: u64 = 1u64 << xlate_pabit(_PAGE_SPECIAL_BIT);
pub const _PAGE_TABLE: u64 = _PAGE_PRESENT | _PAGE_READ | _PAGE_WRITE | _PAGE_DIRTY | _PAGE_ACCESSED;
pub const _PAGE_CHG_MASK: u64 = PAGE_MASK | _PAGE_ACCESSED | _PAGE_DIRTY | _PAGE_SPECIAL;
pub const _PAGE_KERNEL_RO: u64 = _PAGE_PRESENT | _PAGE_READ | _PAGE_DIRTY | _PAGE_ACCESSED;
pub const _PAGE_KERNEL_EXEC: u64 = _PAGE_KERNEL_RO | _PAGE_EXEC;
pub const _PAGE_KERNEL_RWX: u64 = _PAGE_KERNEL_EXEC | _PAGE_WRITE;
pub const _PAGE_KERNEL: u64 = _PAGE_KERNEL_RO | _PAGE_WRITE;
pub const _PAGE_SWP_EXCLUSIVE: u64 = _PAGE_ACCESSED;
pub const _PxD_PRESENT_BIT: usize = 31;
pub const _PxD_VALID_BIT: usize = 30;
pub const PxD_FLAG_PRESENT: u64 = 1u64 << xlate_pabit(_PxD_PRESENT_BIT);
pub const PxD_FLAG_VALID: u64 = 1u64 << xlate_pabit(_PxD_VALID_BIT);
pub const PxD_FLAG_MASK: u64 = 0xf;
pub const PxD_FLAG_SHIFT: usize = 4;
pub const PxD_VALUE_SHIFT: usize = PFN_PTE_SHIFT - PxD_FLAG_SHIFT;

#[inline] pub unsafe fn pte_none(x: pte_t) -> bool { x == 0 }
#[inline] pub unsafe fn pte_present(x: pte_t) -> bool { x & _PAGE_PRESENT != 0 }
#[inline] pub unsafe fn pte_user(x: pte_t) -> bool { x & _PAGE_USER != 0 }
#[inline] pub unsafe fn pmd_flag(x: pmd_t) -> u64 { x & PxD_FLAG_MASK }
#[inline] pub unsafe fn pmd_address(x: pmd_t) -> c_ulong { ((x & !PxD_FLAG_MASK) << PxD_VALUE_SHIFT) as c_ulong }
#[inline] pub unsafe fn pud_flag(x: pud_t) -> u64 { x & PxD_FLAG_MASK }
#[inline] pub unsafe fn pud_address(x: pud_t) -> c_ulong { ((x & !PxD_FLAG_MASK) << PxD_VALUE_SHIFT) as c_ulong }
#[inline] pub unsafe fn pgd_flag(x: pgd_t) -> u64 { x & PxD_FLAG_MASK }
#[inline] pub unsafe fn pgd_address(x: pgd_t) -> c_ulong { ((x & !PxD_FLAG_MASK) << PxD_VALUE_SHIFT) as c_ulong }
#[inline] pub unsafe fn pmd_none(x: pmd_t) -> bool { x == 0 }
#[inline] pub unsafe fn pmd_bad(x: pmd_t) -> bool { pmd_flag(x) & PxD_FLAG_VALID == 0 }
#[inline] pub unsafe fn pmd_present(x: pmd_t) -> bool { pmd_flag(x) & PxD_FLAG_PRESENT != 0 }
#[inline] pub unsafe fn pmd_clear(pmd: *mut pmd_t) { set_pmd(pmd, 0) }

#[inline] pub unsafe fn pte_dirty(pte: pte_t) -> bool { pte & _PAGE_DIRTY != 0 }
#[inline] pub unsafe fn pte_young(pte: pte_t) -> bool { pte & _PAGE_ACCESSED != 0 }
#[inline] pub unsafe fn pte_write(pte: pte_t) -> bool { pte & _PAGE_WRITE != 0 }
#[inline] pub unsafe fn pte_special(pte: pte_t) -> bool { pte & _PAGE_SPECIAL != 0 }
#[inline] pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte &= !_PAGE_DIRTY; pte }
#[inline] pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte &= !_PAGE_ACCESSED; pte }
#[inline] pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte &= !_PAGE_WRITE; pte }
#[inline] pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte |= _PAGE_DIRTY; pte }
#[inline] pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte |= _PAGE_ACCESSED; pte }
#[inline] pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte |= _PAGE_WRITE; pte }
#[inline] pub fn pte_mkspecial(mut pte: pte_t) -> pte_t { pte |= _PAGE_SPECIAL; pte }

#[inline] pub fn pfn_pte(pfn: c_ulong, pgprot: pgprot_t) -> pte_t { ((pfn << PFN_PTE_SHIFT) as u64) | pgprot }
#[inline] pub fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t { (pte & _PAGE_CHG_MASK) | newprot }
#[inline] pub fn pte_pfn(x: pte_t) -> c_ulong { (x >> PFN_PTE_SHIFT) as c_ulong }
#[inline] pub unsafe fn pte_page(pte: pte_t) -> *mut ::core::ffi::c_void { pfn_to_page(pte_pfn(pte)) }
#[inline] pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> c_ulong { __va(pmd_address(pmd)) as c_ulong }
#[inline] pub unsafe fn pmd_pfn(pmd: pmd_t) -> c_ulong { pmd_address(pmd) >> PAGE_SHIFT }
#[inline] pub unsafe fn __pmd_page(pmd: pmd_t) -> c_ulong { __va(pmd_address(pmd)) as c_ulong }
#[inline] pub unsafe fn pmd_page(pmd: pmd_t) -> *mut ::core::ffi::c_void { virt_to_page(__pmd_page(pmd) as *mut _) }

#[inline] pub unsafe fn set_ptes(mm: *mut mm_struct, mut addr: c_ulong, mut ptep: *mut pte_t, mut pte: pte_t, mut nr: u32) {
    if pte_present(pte) && pte_user(pte) { __update_cache(pte); }
    loop {
        *ptep = pte;
        /* purge_tlb_entries(mm, addr); */
        nr -= 1;
        if nr == 0 { break; }
        ptep = ptep.add(1);
        pte = pte.wrapping_add(1u64 << PFN_PTE_SHIFT);
        addr += PAGE_SIZE;
    }
}
pub const PG_dcache_dirty: usize = PG_arch_1;
pub const _PAGE_SIZE_ENCODING_4K: usize = 0;
pub const _PAGE_SIZE_ENCODING_16K: usize = 1;
pub const _PAGE_SIZE_ENCODING_64K: usize = 2;
pub const _PAGE_SIZE_ENCODING_256K: usize = 3;
pub const _PAGE_SIZE_ENCODING_1M: usize = 4;
pub const _PAGE_SIZE_ENCODING_4M: usize = 5;
pub const _PAGE_SIZE_ENCODING_16M: usize = 6;
pub const _PAGE_SIZE_ENCODING_64M: usize = 7;
pub const _PAGE_SIZE_ENCODING_DEFAULT: usize = _PAGE_SIZE_ENCODING_4K;
pub const HAVE_ARCH_UNMAPPED_AREA: bool = true;
pub const HAVE_ARCH_UNMAPPED_AREA_TOPDOWN: bool = true;
pub const __HAVE_ARCH_PTEP_TEST_AND_CLEAR_YOUNG: bool = true;
pub const __HAVE_ARCH_PTEP_CLEAR_YOUNG_FLUSH: bool = true;
pub const __HAVE_ARCH_PTEP_CLEAR_FLUSH: bool = true;
pub const __HAVE_ARCH_PTEP_SET_WRPROTECT: bool = true;
pub const __HAVE_ARCH_PTE_SAME: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
