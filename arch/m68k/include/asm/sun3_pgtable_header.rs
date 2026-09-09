/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent: <asm/sun3mmu.h>, <asm/virtconvert.h>, and <linux/linkage.h>. */
/* This file contains the things which change drastically for the sun3 pagetable stuff. */

/* For virtual address to physical address conversion. */
macro_rules! VTOP { ($addr:expr) => { __pa($addr) }; }
macro_rules! PTOV { ($addr:expr) => { __va($addr) }; }

/* These are defined for compatibility although the sun3 does not use them. */
pub const _PAGE_NOCACHE030: u32 = 0x040;
pub const _CACHEMASK040: u32 = !0x060;
pub const _PAGE_NOCACHE_S: u32 = 0x040;

/* Page protection values within PTE. */
pub const SUN3_PAGE_VALID: u32 = 0x80000000;
pub const SUN3_PAGE_WRITEABLE: u32 = 0x40000000;
pub const SUN3_PAGE_SYSTEM: u32 = 0x20000000;
pub const SUN3_PAGE_NOCACHE: u32 = 0x10000000;
pub const SUN3_PAGE_ACCESSED: u32 = 0x02000000;
pub const SUN3_PAGE_MODIFIED: u32 = 0x01000000;

/* Externally used page protection values. */
pub const _PAGE_PRESENT: u32 = SUN3_PAGE_VALID;
pub const _PAGE_ACCESSED: u32 = SUN3_PAGE_ACCESSED;

/* Compound page protection values. */
// TODO: work out which ones should have SUN3_PAGE_NOCACHE and fix.
macro_rules! PAGE_NONE { () => { __pgprot(SUN3_PAGE_VALID | SUN3_PAGE_ACCESSED | SUN3_PAGE_NOCACHE) }; }
macro_rules! PAGE_SHARED { () => { __pgprot(SUN3_PAGE_VALID | SUN3_PAGE_WRITEABLE | SUN3_PAGE_ACCESSED | SUN3_PAGE_NOCACHE) }; }
macro_rules! PAGE_COPY { () => { __pgprot(SUN3_PAGE_VALID | SUN3_PAGE_ACCESSED | SUN3_PAGE_NOCACHE) }; }
macro_rules! PAGE_READONLY { () => { __pgprot(SUN3_PAGE_VALID | SUN3_PAGE_ACCESSED | SUN3_PAGE_NOCACHE) }; }
macro_rules! PAGE_KERNEL { () => { __pgprot(SUN3_PAGE_VALID | SUN3_PAGE_WRITEABLE | SUN3_PAGE_SYSTEM | SUN3_PAGE_NOCACHE | SUN3_PAGE_ACCESSED | SUN3_PAGE_MODIFIED) }; }
macro_rules! PAGE_INIT { () => { __pgprot(SUN3_PAGE_VALID | SUN3_PAGE_WRITEABLE | SUN3_PAGE_SYSTEM | SUN3_PAGE_NOCACHE) }; }

/* Fake page protections used on PMDs. */
pub const SUN3_PMD_VALID: u32 = 0x00000001;
pub const SUN3_PMD_MASK: u32 = 0x0000003F;
pub const SUN3_PMD_MAGIC: u32 = 0x0000002B;
pub const _PAGE_SWP_EXCLUSIVE: u32 = 0x040;

pub unsafe fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte_val_mut(&mut pte, (pte_val(pte) & SUN3_PAGE_CHG_MASK) | pgprot_val(newprot));
    pte
}

macro_rules! pmd_set { ($pmdp:expr, $ptep:expr) => {{}}; }
macro_rules! __pte_page { ($pte:expr) => { __va((pte_val($pte) & SUN3_PAGE_PGNUM_MASK) << PAGE_SHIFT) }; }

pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> usize { __va(pmd_val(pmd) & PAGE_MASK) as usize }
pub unsafe fn pte_none(pte: pte_t) -> i32 { (pte_val(pte) == 0) as i32 }
pub unsafe fn pte_present(pte: pte_t) -> i32 { (pte_val(pte) & SUN3_PAGE_VALID) as i32 }
pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) { pte_val_mut(&mut *ptep, 0); }

pub const PFN_PTE_SHIFT: u32 = 0;
macro_rules! pte_pfn { ($pte:expr) => { pte_val($pte) & SUN3_PAGE_PGNUM_MASK }; }
macro_rules! pfn_pte { ($pfn:expr, $pgprot:expr) => {{ let mut __pte: pte_t = core::mem::zeroed(); pte_val_mut(&mut __pte, ($pfn) | pgprot_val($pgprot)); __pte }}; }
macro_rules! pte_page { ($pte:expr) => { virt_to_page(__pte_page!($pte)) }; }
macro_rules! pmd_pfn { ($pmd:expr) => { pmd_val($pmd) >> PAGE_SHIFT }; }
macro_rules! pmd_page { ($pmd:expr) => { virt_to_page(pmd_page_vaddr($pmd) as *mut core::ffi::c_void) }; }

pub unsafe fn pmd_none2(pmd: *mut pmd_t) -> i32 { (pmd_val(*pmd) == 0) as i32 }
macro_rules! pmd_none { ($pmd:expr) => { pmd_none2(&mut $pmd) }; }
pub unsafe fn pmd_bad2(_pmd: *mut pmd_t) -> i32 { 0 }
macro_rules! pmd_bad { ($pmd:expr) => { pmd_bad2(&mut $pmd) }; }
pub unsafe fn pmd_present2(pmd: *mut pmd_t) -> i32 { (pmd_val(*pmd) & SUN3_PMD_VALID) as i32 }
macro_rules! pmd_present { ($pmd:expr) => { !pmd_none2(&mut $pmd) }; }
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { pmd_val_mut(&mut *pmdp, 0); }

macro_rules! pte_ERROR { ($e:expr) => { pr_err!("%s:%d: bad pte %08lx.\n", file!(), line!(), pte_val($e)) }; }
macro_rules! pgd_ERROR { ($e:expr) => { pr_err!("%s:%d: bad pgd %08lx.\n", file!(), line!(), pgd_val($e)) }; }

pub unsafe fn pte_write(pte: pte_t) -> i32 { (pte_val(pte) & SUN3_PAGE_WRITEABLE) as i32 }
pub unsafe fn pte_dirty(pte: pte_t) -> i32 { (pte_val(pte) & SUN3_PAGE_MODIFIED) as i32 }
pub unsafe fn pte_young(pte: pte_t) -> i32 { (pte_val(pte) & SUN3_PAGE_ACCESSED) as i32 }
pub unsafe fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) & !SUN3_PAGE_WRITEABLE); pte }
pub unsafe fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) & !SUN3_PAGE_MODIFIED); pte }
pub unsafe fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) & !SUN3_PAGE_ACCESSED); pte }
pub unsafe fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) | SUN3_PAGE_WRITEABLE); pte }
pub unsafe fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) | SUN3_PAGE_MODIFIED); pte }
pub unsafe fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) | SUN3_PAGE_ACCESSED); pte }
pub unsafe fn pte_mknocache(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) | SUN3_PAGE_NOCACHE); pte }
pub unsafe fn pte_mkcache(pte: pte_t) -> pte_t { pte }

extern "C" { pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD]; pub static mut kernel_pg_dir: [pgd_t; PTRS_PER_PGD]; }

macro_rules! __swp_type { ($x:expr) => { ($x).val & 0x3f }; }
macro_rules! __swp_offset { ($x:expr) => { ($x).val >> 7 }; }
macro_rules! __swp_entry { ($type:expr, $offset:expr) => { swp_entry_t { val: (($type & 0x3f) | (($offset << 7) & !SUN3_PAGE_VALID)) } }; }
macro_rules! __pte_to_swp_entry { ($pte:expr) => { swp_entry_t { val: pte_val($pte) } }; }
macro_rules! __swp_entry_to_pte { ($x:expr) => { pte_t { val: ($x).val } }; }

pub unsafe fn pte_swp_exclusive(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_SWP_EXCLUSIVE) != 0 }
pub unsafe fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) | _PAGE_SWP_EXCLUSIVE); pte }
pub unsafe fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_val_mut(&mut pte, pte_val(pte) & !_PAGE_SWP_EXCLUSIVE); pte }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
