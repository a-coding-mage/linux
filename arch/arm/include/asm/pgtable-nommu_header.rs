/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/pgtable-nommu.h
 *
 *  Copyright (C) 1995-2002 Russell King
 *  Copyright (C) 2004  Hyok S. Choi
 */

/* The C header guard is not needed for a Rust source file. */

/*
 * The following declarations correspond to the non-assembly portion of the
 * original header.  Types and symbols supplied by included headers remain
 * external dependencies.
 */

/* Trivial page table functions. */
macro_rules! pgd_present {
    ($pgd:expr) => {{ let _ = &$pgd; 1 }};
}
macro_rules! pgd_none {
    ($pgd:expr) => {{ let _ = &$pgd; 0 }};
}
macro_rules! pgd_bad {
    ($pgd:expr) => {{ let _ = &$pgd; 0 }};
}
macro_rules! pgd_clear {
    ($pgdp:expr) => {{ let _ = &$pgdp; }};
}

/*
 * PMD_SHIFT determines the size of the area a second-level page table can map
 * PGDIR_SHIFT determines what a third-level page table entry can map
 */
pub const PGDIR_SHIFT: u32 = 21;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
/* FIXME */

/* __pgprot is supplied by the page-table dependencies. */
macro_rules! PAGE_NONE { () => { __pgprot(0) }; }
macro_rules! PAGE_SHARED { () => { __pgprot(0) }; }
macro_rules! PAGE_COPY { () => { __pgprot(0) }; }
macro_rules! PAGE_READONLY { () => { __pgprot(0) }; }
macro_rules! PAGE_KERNEL { () => { __pgprot(0) }; }

pub const swapper_pg_dir: *mut pgd_t = core::ptr::null_mut();

pub type pte_addr_t = *mut pte_t;

/* Mark the prot value as uncacheable and unbufferable. */
macro_rules! pgprot_noncached { ($prot:expr) => { $prot }; }
macro_rules! pgprot_writecombine { ($prot:expr) => { $prot }; }
macro_rules! pgprot_device { ($prot:expr) => { $prot }; }

/* These would be in other places but having them here reduces the diffs. */
unsafe extern "C" {
    pub fn kobjsize(objp: *const core::ffi::c_void) -> core::ffi::c_uint;
}

/*
 * All 32bit addresses are effectively valid for vmalloc...
 * Sort of meaningless for non-VM targets.
 */
pub const VMALLOC_START: usize = 0;
pub const VMALLOC_END: usize = 0xffff_ffff;
pub const FIRST_USER_ADDRESS: usize = 0;

/*
 * The original __ASSEMBLY__ branch provides dummy TLB and user structures.
 * They are represented as zero-valued macros here for assembly-equivalent use.
 */
macro_rules! v3_tlb_fns { () => { 0 }; }
macro_rules! v4_tlb_fns { () => { 0 }; }
macro_rules! v4wb_tlb_fns { () => { 0 }; }
macro_rules! v4wbi_tlb_fns { () => { 0 }; }
macro_rules! v6wbi_tlb_fns { () => { 0 }; }
macro_rules! v7wbi_tlb_fns { () => { 0 }; }

macro_rules! v3_user_fns { () => { 0 }; }
macro_rules! v4_user_fns { () => { 0 }; }
macro_rules! v4_mc_user_fns { () => { 0 }; }
macro_rules! v4wb_user_fns { () => { 0 }; }
macro_rules! v4wt_user_fns { () => { 0 }; }
macro_rules! v6_user_fns { () => { 0 }; }
macro_rules! xscale_mc_user_fns { () => { 0 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
