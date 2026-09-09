/* SPDX-License-Identifier: GPL-2.0 */

/* C includes and header guards omitted; referenced kernel symbols remain external dependencies. */

pub const PTE_INDEX_SIZE: usize = PTE_SHIFT;
pub const PMD_INDEX_SIZE: usize = 0;
pub const PUD_INDEX_SIZE: usize = 0;
pub const PGD_INDEX_SIZE: usize = 32 - PGDIR_SHIFT;

pub const PMD_CACHE_INDEX: usize = PMD_INDEX_SIZE;
pub const PUD_CACHE_INDEX: usize = PUD_INDEX_SIZE;

/* These sizeof-based declarations retain their C expressions through macros. */
#[macro_export]
macro_rules! PTE_TABLE_SIZE { () => { core::mem::size_of::<pte_t>() << PTE_INDEX_SIZE }; }
pub const PMD_TABLE_SIZE: usize = 0;
pub const PUD_TABLE_SIZE: usize = 0;
#[macro_export]
macro_rules! PGD_TABLE_SIZE { () => { core::mem::size_of::<pgd_t>() << PGD_INDEX_SIZE }; }
#[macro_export]
macro_rules! PMD_MASKED_BITS { () => { PTE_TABLE_SIZE!() - 1 }; }

pub const PTRS_PER_PTE: usize = 1 << PTE_INDEX_SIZE;
pub const PTRS_PER_PGD: usize = 1 << PGD_INDEX_SIZE;

/* PGDIR_SHIFT determines what a top-level page table entry can map. */
pub const PGDIR_SHIFT: usize = PAGE_SHIFT + PTE_INDEX_SIZE;
#[macro_export]
macro_rules! PGDIR_SIZE { () => { 1usize << PGDIR_SHIFT }; }
#[macro_export]
macro_rules! PGDIR_MASK { () => { !(PGDIR_SIZE!() - 1) }; }

/* Bits to mask out from a PGD to get to the PUD page. */
pub const PGD_MASKED_BITS: usize = 0;

#[macro_export]
macro_rules! USER_PTRS_PER_PGD { () => { TASK_SIZE / PGDIR_SIZE!() }; }

#[macro_export]
macro_rules! pgd_ERROR {
    ($e:expr) => { pr_err!("{}:{}: bad pgd {:08x}.\n", file!(), line!(), pgd_val($e) as u64) };
}

pub const FIXADDR_SIZE: usize = 0;

/* CONFIG_KASAN: FIXADDR_TOP is KASAN_SHADOW_START - PAGE_SIZE. */
#[macro_export]
macro_rules! FIXADDR_TOP { () => { (-(PAGE_SIZE as isize)) as usize }; }

/* CONFIG_HIGHMEM: IOREMAP_TOP is PKMAP_BASE; otherwise it is FIXADDR_START. */
#[macro_export]
macro_rules! IOREMAP_TOP { () => { FIXADDR_START }; }

pub const IOREMAP_START: usize = VMALLOC_START;
pub const IOREMAP_END: usize = VMALLOC_END;

pub const VMALLOC_OFFSET: usize = 0x1000000;

/* PPC_PIN_SIZE, when defined, is used as the alignment size. */
#[macro_export]
macro_rules! VMALLOC_START {
    () => { (((high_memory as isize as usize) + VMALLOC_OFFSET) & !(VMALLOC_OFFSET - 1)) };
}

/* CONFIG_KASAN_VMALLOC: align ioremap_bot down by PAGE_SIZE << KASAN_SHADOW_SCALE_SHIFT. */
#[macro_export]
macro_rules! VMALLOC_END { () => { ioremap_bot }; }

/* Platform-specific PTE definitions are supplied by the corresponding C headers. */

/* PTE_RPN_SHIFT defaults to PAGE_SHIFT when not supplied by the platform. */
pub const PTE_RPN_SHIFT: usize = PAGE_SHIFT;

/* CONFIG_PTE_64BIT selects the 64-bit mask and 36 possible physical bits. */
#[macro_export]
macro_rules! PTE_RPN_MASK { () => { !((1usize << PTE_RPN_SHIFT) - 1) }; }
pub const MAX_POSSIBLE_PHYSMEM_BITS: usize = 32;

#[macro_export]
macro_rules! pmd_none { ($pmd:expr) => { pmd_val($pmd) == 0 }; }
#[macro_export]
macro_rules! pmd_bad { ($pmd:expr) => { pmd_val($pmd) & _PMD_BAD }; }
#[macro_export]
macro_rules! pmd_present { ($pmd:expr) => { pmd_val($pmd) & _PMD_PRESENT_MASK }; }

#[inline]
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) {
    *pmdp = __pmd(0);
}

/* CONFIG_BOOKE stores a kernel virtual address in the PMD; other platforms store a physical address. */
#[macro_export]
macro_rules! pmd_pfn { ($pmd:expr) => { pmd_val($pmd) >> PAGE_SHIFT }; }

#[macro_export]
macro_rules! pmd_page {
    ($pmd:expr) => { pfn_to_page(pmd_pfn!($pmd)) };
}

/* Encode/decode swap entries and swap PTEs. */
#[macro_export]
macro_rules! __swp_type { ($entry:expr) => { ($entry).val & 0x1f }; }
#[macro_export]
macro_rules! __swp_offset { ($entry:expr) => { ($entry).val >> 5 }; }
#[macro_export]
macro_rules! __swp_entry {
    ($type:expr, $offset:expr) => { swp_entry_t { val: (($type & 0x1f) | ($offset << 5)) } };
}
#[macro_export]
macro_rules! __pte_to_swp_entry { ($pte:expr) => { swp_entry_t { val: pte_val($pte) >> 3 } }; }
#[macro_export]
macro_rules! __swp_entry_to_pte { ($x:expr) => { pte_t { val: ($x).val << 3 } }; }

/* We borrow LSB 2 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: u64 = 0x000004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
