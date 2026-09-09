/* SPDX-License-Identifier: GPL-2.0
 *
 * fixmap.h: compile-time virtual memory allocation
 *
 * Copyright (C) 1998 Ingo Molnar
 *
 * Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
 */

/* Translated from the C header; Linux dependencies are supplied externally. */

/*
 * Here we define all the compile-time 'special' virtual
 * addresses. The point is to have a constant address at
 * compile time, but to set the physical address only
 * in the boot process. We allocate these special addresses
 * from the end of P3 backwards.
 * Also this lets us do fail-safe vmalloc(), we
 * can guarantee that these special addresses and
 * vmalloc()-ed addresses never overlap.
 *
 * these 'compile-time allocated' memory buffers are
 * fixed-size 4k pages. (or larger if used with an increment
 * highger than 1) use fixmap_set(idx,phys) to associate
 * physical memory with fixmap indices.
 *
 * TLB entries of such buffers will not be flushed across
 * task switches.
 */

/*
 * on UP currently we will have no trace of the fixmap mechanizm,
 * no page table allocations, etc. This might change in the
 * future, say framebuffers for the console driver(s) could be
 * fix-mapped?
 */
pub const FIX_N_COLOURS: usize = 8;

#[repr(isize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fixed_addresses {
    /*
     * The FIX_CMAP entries are used by kmap_coherent() to get virtual
     * addresses which are of a known color, and so their values are
     * important. __fix_to_virt(FIX_CMAP_END - n) must give an address
     * which is the same color as a page (n<<PAGE_SHIFT).
     */
    FIX_CMAP_BEGIN = 0,
    FIX_CMAP_END = FIX_CMAP_BEGIN as isize + (FIX_N_COLOURS * NR_CPUS) as isize - 1,

    /* The CONFIG_IOREMAP_FIXED conditional is preserved here. */
    #[cfg(CONFIG_IOREMAP_FIXED)]
    FIX_IOREMAP_BEGIN = FIX_CMAP_END as isize + 1,
    #[cfg(CONFIG_IOREMAP_FIXED)]
    FIX_IOREMAP_END = FIX_IOREMAP_BEGIN as isize + FIX_N_IOREMAPS as isize - 1,

    __end_of_fixed_addresses,
}

#[cfg(CONFIG_IOREMAP_FIXED)]
pub const FIX_N_IOREMAPS: usize = 32;

pub unsafe extern "C" {
    pub fn __set_fixmap(idx: fixed_addresses, phys: ::core::ffi::c_ulong, flags: pgprot_t);
    pub fn __clear_fixmap(idx: fixed_addresses, flags: pgprot_t);
}

/*
 * used by vmalloc.c.
 *
 * Leave one empty page between vmalloc'ed areas and
 * the start of the fixmap, and leave one page empty
 * at the top of mem..
 */
pub const FIXADDR_TOP: usize = P4SEG - PAGE_SIZE;
pub const FIXADDR_SIZE: usize = (__end_of_fixed_addresses as usize) << PAGE_SHIFT;
pub const FIXADDR_START: usize = FIXADDR_TOP - FIXADDR_SIZE;

pub const FIXMAP_PAGE_NOCACHE: pgprot_t = PAGE_KERNEL_NOCACHE;

/* Contents supplied by <asm-generic/fixmap.h>. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
