/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/* C header dependencies are supplied by other translated files. */

#[allow(non_upper_case_globals)]
extern "C" {
    pub static mut mem_init_done: ::kernel::ffi::c_int;
    pub static mut ioremap_bot: ::kernel::ffi::c_ulong;
    pub static mut ioremap_base: ::kernel::ffi::c_ulong;
}

extern "C" {
    pub fn va_to_phys(address: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn va_to_pte(address: ::kernel::ffi::c_ulong) -> *mut pte_t;
    pub fn iopa(addr: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn raw_local_irq_save(flags: *mut ::kernel::ffi::c_ulong);
    pub fn raw_local_irq_restore(flags: ::kernel::ffi::c_ulong);
    pub fn printk(fmt: *const ::kernel::ffi::c_char, ...) -> ::kernel::ffi::c_int;
    pub fn set_pte_at(mm: *mut mm_struct, addr: ::kernel::ffi::c_ulong,
                      ptep: *mut pte_t, pte: pte_t);
    pub fn __pa(value: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pfn_to_page(pfn: ::kernel::ffi::c_ulong) -> *mut page;
}

/* Build-time configuration and architecture types are supplied externally. */
pub const VMALLOC_START: ::kernel::ffi::c_ulong = CONFIG_KERNEL_START + CONFIG_LOWMEM_SIZE;

pub const PGDIR_SHIFT: ::kernel::ffi::c_uint = PAGE_SHIFT + PTE_SHIFT;
pub const PGDIR_SIZE: ::kernel::ffi::c_ulong = 1u64 << PGDIR_SHIFT;
pub const PGDIR_MASK: ::kernel::ffi::c_ulong = !(PGDIR_SIZE - 1);
pub const PTRS_PER_PTE: ::kernel::ffi::c_uint = 1 << PTE_SHIFT;
pub const PTRS_PER_PMD: ::kernel::ffi::c_uint = 1;
pub const PTRS_PER_PGD: ::kernel::ffi::c_uint = 1 << (32 - PGDIR_SHIFT);
pub const USER_PTRS_PER_PGD: ::kernel::ffi::c_ulong = TASK_SIZE / PGDIR_SIZE;
pub const USER_PGD_PTRS: ::kernel::ffi::c_ulong = PAGE_OFFSET >> PGDIR_SHIFT;
pub const KERNEL_PGD_PTRS: ::kernel::ffi::c_ulong = PTRS_PER_PGD as ::kernel::ffi::c_ulong - USER_PGD_PTRS;

pub const _PAGE_GUARDED: ::kernel::ffi::c_ulong = 0x001;
pub const _PAGE_PRESENT: ::kernel::ffi::c_ulong = 0x002;
pub const _PAGE_NO_CACHE: ::kernel::ffi::c_ulong = 0x004;
pub const _PAGE_WRITETHRU: ::kernel::ffi::c_ulong = 0x008;
pub const _PAGE_USER: ::kernel::ffi::c_ulong = 0x010;
pub const _PAGE_RW: ::kernel::ffi::c_ulong = 0x040;
pub const _PAGE_DIRTY: ::kernel::ffi::c_ulong = 0x080;
pub const _PAGE_HWWRITE: ::kernel::ffi::c_ulong = 0x100;
pub const _PAGE_HWEXEC: ::kernel::ffi::c_ulong = 0x200;
pub const _PAGE_ACCESSED: ::kernel::ffi::c_ulong = 0x400;
pub const _PMD_PRESENT: ::kernel::ffi::c_ulong = PAGE_MASK;
pub const _PAGE_SWP_EXCLUSIVE: ::kernel::ffi::c_ulong = _PAGE_DIRTY;
pub const _PAGE_HASHPTE: ::kernel::ffi::c_ulong = 0;
pub const _PTE_NONE_MASK: ::kernel::ffi::c_ulong = 0;
pub const _PAGE_SHARED: ::kernel::ffi::c_ulong = 0;
pub const _PAGE_EXEC: ::kernel::ffi::c_ulong = 0;
pub const _PAGE_CACHE_CTL: ::kernel::ffi::c_ulong = _PAGE_GUARDED | _PAGE_NO_CACHE | _PAGE_WRITETHRU;
pub const _PAGE_CHG_MASK: ::kernel::ffi::c_ulong = PAGE_MASK | _PAGE_ACCESSED | _PAGE_DIRTY;
pub const _PAGE_BASE: ::kernel::ffi::c_ulong = _PAGE_PRESENT | _PAGE_ACCESSED;
pub const _PAGE_WRENABLE: ::kernel::ffi::c_ulong = _PAGE_RW | _PAGE_DIRTY | _PAGE_HWWRITE;
pub const _PAGE_KERNEL: ::kernel::ffi::c_ulong = _PAGE_BASE | _PAGE_WRENABLE | _PAGE_SHARED | _PAGE_HWEXEC;
pub const _PAGE_IO: ::kernel::ffi::c_ulong = _PAGE_KERNEL | _PAGE_NO_CACHE | _PAGE_GUARDED;

#[macro_export] macro_rules! pgprot_noncached { ($prot:expr) => { __pgprot((pgprot_val($prot) & !_PAGE_CACHE_CTL) | _PAGE_NO_CACHE | _PAGE_GUARDED) }; }
#[macro_export] macro_rules! pgprot_noncached_wc { ($prot:expr) => { __pgprot((pgprot_val($prot) & !_PAGE_CACHE_CTL) | _PAGE_NO_CACHE) }; }
#[macro_export] macro_rules! PAGE_NONE { () => { __pgprot(_PAGE_BASE) }; }
#[macro_export] macro_rules! PAGE_READONLY { () => { __pgprot(_PAGE_BASE | _PAGE_USER) }; }
#[macro_export] macro_rules! PAGE_READONLY_X { () => { __pgprot(_PAGE_BASE | _PAGE_USER | _PAGE_EXEC) }; }
#[macro_export] macro_rules! PAGE_SHARED { () => { __pgprot(_PAGE_BASE | _PAGE_USER | _PAGE_RW) }; }
#[macro_export] macro_rules! PAGE_SHARED_X { () => { __pgprot(_PAGE_BASE | _PAGE_USER | _PAGE_RW | _PAGE_EXEC) }; }
#[macro_export] macro_rules! PAGE_COPY { () => { __pgprot(_PAGE_BASE | _PAGE_USER) }; }
#[macro_export] macro_rules! PAGE_COPY_X { () => { __pgprot(_PAGE_BASE | _PAGE_USER | _PAGE_EXEC) }; }
#[macro_export] macro_rules! PAGE_KERNEL { () => { __pgprot(_PAGE_KERNEL) }; }
#[macro_export] macro_rules! PAGE_KERNEL_RO { () => { __pgprot(_PAGE_BASE | _PAGE_SHARED) }; }
#[macro_export] macro_rules! PAGE_KERNEL_CI { () => { __pgprot(_PAGE_IO) }; }

#[macro_export] macro_rules! pte_none { ($pte:expr) => { (pte_val($pte) & !_PTE_NONE_MASK) == 0 }; }
#[macro_export] macro_rules! pte_present { ($pte:expr) => { pte_val($pte) & _PAGE_PRESENT }; }
#[macro_export] macro_rules! pte_clear { ($mm:expr, $addr:expr, $ptep:expr) => { set_pte_at($mm, $addr, $ptep, __pte(0)); }; }
#[macro_export] macro_rules! pmd_none { ($pmd:expr) => { pmd_val($pmd) == 0 }; }
#[macro_export] macro_rules! pmd_bad { ($pmd:expr) => { (pmd_val($pmd) & _PMD_PRESENT) == 0 }; }
#[macro_export] macro_rules! pmd_present { ($pmd:expr) => { (pmd_val($pmd) & _PMD_PRESENT) != 0 }; }
#[macro_export] macro_rules! pmd_clear { ($pmdp:expr) => { pmd_val(*$pmdp) = 0; }; }
#[macro_export] macro_rules! pte_page { ($x:expr) => { mem_map + ((pte_val($x) - memory_start) >> PAGE_SHIFT) }; }
pub const PFN_PTE_SHIFT: ::kernel::ffi::c_uint = PAGE_SHIFT;
#[macro_export] macro_rules! pte_pfn { ($x:expr) => { pte_val($x) >> PFN_PTE_SHIFT }; }
#[macro_export] macro_rules! pfn_pte { ($pfn:expr, $prot:expr) => { __pte((($pfn as pte_basic_t) << PFN_PTE_SHIFT) | pgprot_val($prot)) }; }

pub unsafe fn pte_read(pte: pte_t) -> ::kernel::ffi::c_int { (pte_val(pte) & _PAGE_USER) as _ }
pub unsafe fn pte_write(pte: pte_t) -> ::kernel::ffi::c_int { (pte_val(pte) & _PAGE_RW) as _ }
pub unsafe fn pte_exec(pte: pte_t) -> ::kernel::ffi::c_int { (pte_val(pte) & _PAGE_EXEC) as _ }
pub unsafe fn pte_dirty(pte: pte_t) -> ::kernel::ffi::c_int { (pte_val(pte) & _PAGE_DIRTY) as _ }
pub unsafe fn pte_young(pte: pte_t) -> ::kernel::ffi::c_int { (pte_val(pte) & _PAGE_ACCESSED) as _ }

pub unsafe fn pte_uncache(mut pte: pte_t) { pte_val(pte) |= _PAGE_NO_CACHE; }
pub unsafe fn pte_cache(mut pte: pte_t) { pte_val(pte) &= !_PAGE_NO_CACHE; }
pub unsafe fn pte_rdprotect(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_USER; pte }
pub unsafe fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_val(pte) &= !(_PAGE_RW | _PAGE_HWWRITE); pte }
pub unsafe fn pte_exprotect(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_EXEC; pte }
pub unsafe fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val(pte) &= !(_PAGE_DIRTY | _PAGE_HWWRITE); pte }
pub unsafe fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_ACCESSED; pte }
pub unsafe fn pte_mkread(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_USER; pte }
pub unsafe fn pte_mkexec(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_USER | _PAGE_EXEC; pte }
pub unsafe fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_RW; pte }
pub unsafe fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_DIRTY; pte }
pub unsafe fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_ACCESSED; pte }

pub unsafe fn mk_pte_phys(physpage: phys_addr_t, pgprot: pgprot_t) -> pte_t {
    let mut pte: pte_t = core::mem::zeroed(); pte_val(pte) = physpage | pgprot_val(pgprot); pte
}
pub unsafe fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte_val(pte) = (pte_val(pte) & _PAGE_CHG_MASK) | pgprot_val(newprot); pte
}

pub unsafe fn set_pte(ptep: *mut pte_t, pte: pte_t) { *ptep = pte; }
pub unsafe fn pte_update(_p: *mut pte_t, _clr: ::kernel::ffi::c_ulong, _set: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong {
    /* MicroBlaze inline assembly atomically clears and sets PTE bits. */
    todo!("MicroBlaze atomic pte_update requires the target assembly implementation")
}

pub struct vm_area_struct;
pub struct mm_struct;
pub struct page;
pub type phys_addr_t = ::kernel::ffi::c_ulong;
pub type pte_basic_t = ::kernel::ffi::c_ulong;
pub const __HAVE_ARCH_PTEP_TEST_AND_CLEAR_YOUNG: bool = true;
pub const __HAVE_ARCH_PTEP_GET_AND_CLEAR: bool = true;

pub unsafe fn ptep_test_and_clear_young(_vma: *mut vm_area_struct, _address: ::kernel::ffi::c_ulong, ptep: *mut pte_t) -> bool { (pte_update(ptep, _PAGE_ACCESSED, 0) & _PAGE_ACCESSED) != 0 }
pub unsafe fn ptep_test_and_clear_dirty(_mm: *mut mm_struct, _addr: ::kernel::ffi::c_ulong, ptep: *mut pte_t) -> ::kernel::ffi::c_int { (pte_update(ptep, _PAGE_DIRTY | _PAGE_HWWRITE, 0) & _PAGE_DIRTY != 0) as _ }
pub unsafe fn ptep_get_and_clear(_mm: *mut mm_struct, _addr: ::kernel::ffi::c_ulong, ptep: *mut pte_t) -> pte_t { __pte(pte_update(ptep, !_PAGE_HASHPTE, 0)) }
pub unsafe fn ptep_mkdirty(_mm: *mut mm_struct, _addr: ::kernel::ffi::c_ulong, ptep: *mut pte_t) { pte_update(ptep, 0, _PAGE_DIRTY); }
pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> ::kernel::ffi::c_ulong { pmd_val(pmd) & PAGE_MASK }
#[macro_export] macro_rules! pmd_pfn { ($pmd:expr) => { __pa(pmd_val($pmd)) >> PAGE_SHIFT }; }
#[macro_export] macro_rules! pmd_page { ($pmd:expr) => { pfn_to_page(__pa(pmd_val($pmd)) >> PAGE_SHIFT) }; }

extern "C" { pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD as usize]; }
#[macro_export] macro_rules! __swp_type { ($entry:expr) => { ($entry).val & 0x1f }; }
#[macro_export] macro_rules! __swp_offset { ($entry:expr) => { ($entry).val >> 6 }; }
#[macro_export] macro_rules! __swp_entry { ($type:expr, $offset:expr) => { swp_entry_t { val: (($type & 0x1f) | ($offset << 6)) } }; }
#[macro_export] macro_rules! __pte_to_swp_entry { ($pte:expr) => { swp_entry_t { val: pte_val($pte) >> 2 } }; }
#[macro_export] macro_rules! __swp_entry_to_pte { ($x:expr) => { pte_t { val: ($x).val << 2 } }; }
pub unsafe fn pte_swp_exclusive(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_SWP_EXCLUSIVE) != 0 }
pub unsafe fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_SWP_EXCLUSIVE; pte }
pub unsafe fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_SWP_EXCLUSIVE; pte }

pub const IOMAP_FULL_CACHING: ::kernel::ffi::c_int = 0;
pub const IOMAP_NOCACHE_SER: ::kernel::ffi::c_int = 1;
pub const IOMAP_NOCACHE_NONSER: ::kernel::ffi::c_int = 2;
pub const IOMAP_NO_COPYBACK: ::kernel::ffi::c_int = 3;
extern "C" {
    pub fn do_page_fault(regs: *mut pt_regs, address: ::kernel::ffi::c_ulong, error_code: ::kernel::ffi::c_ulong);
    pub fn mapin_ram();
    pub fn map_page(va: ::kernel::ffi::c_ulong, pa: phys_addr_t, flags: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn mmu_init();
    pub fn setup_memory();
}
pub struct pt_regs;
pub struct pte_t { pub val: ::kernel::ffi::c_ulong }
pub struct pgd_t { pub val: ::kernel::ffi::c_ulong }
pub struct pmd_t { pub val: ::kernel::ffi::c_ulong }
pub struct swp_entry_t { pub val: ::kernel::ffi::c_ulong }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
