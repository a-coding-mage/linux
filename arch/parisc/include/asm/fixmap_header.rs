/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file defines the locations of the fixed mappings on parisc.
 *
 * All of the values in this file are machine virtual addresses.
 *
 * All of the values in this file must be <4GB (because of assembly
 * loading restrictions).  If you place this region anywhere above
 * __PAGE_OFFSET, you must adjust the memory map accordingly
 */

/*
 * The tmpalias region is used in kernel space to copy/clear/flush data
 * from pages congruently mapped with user space. It is comprised of a
 * pair regions. The size of these regions is determined by the largest
 * cache aliasing boundary for machines that support equivalent aliasing.
 *
 * The c3750 with PA8700 processor returns an alias value of 11. This
 * indicates that it has an alias boundary of 4 MB. It also supports
 * non-equivalent aliasing without a performance penalty.
 *
 * Machines with PA8800/PA8900 processors return an alias value of 0.
 * This indicates the alias boundary is unknown and may be larger than
 * 16 MB. Non-equivalent aliasing is not supported.
 *
 * Here we assume the maximum alias boundary is 4 MB.
 */
pub const TMPALIAS_SIZE_BITS: usize = 22; /* 4 MB */
pub const TMPALIAS_MAP_START: usize = __PAGE_OFFSET - (2usize << TMPALIAS_SIZE_BITS);

pub const FIXMAP_SIZE: usize = FIX_BITMAP_COUNT as usize * PAGE_SHIFT;
pub const FIXMAP_START: usize = TMPALIAS_MAP_START - FIXMAP_SIZE;
/* This is the kernel area for all maps (vmalloc, dma etc.)  most
 * usually, it extends up to TMPALIAS_MAP_START.  Virtual addresses
 * 0..GATEWAY_PAGE_SIZE are reserved for the gateway page */
pub const KERNEL_MAP_START: usize = GATEWAY_PAGE_SIZE;
pub const KERNEL_MAP_END: usize = FIXMAP_START;

/* The declarations below are omitted when translating for the assembler. */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fixed_addresses {
    /* Support writing RO kernel text via kprobes, jump labels, etc. */
    FIX_TEXT_POKE0,
    FIX_TEXT_KEXEC,
    FIX_BITMAP_COUNT,
}

extern "C" {
    pub static mut parisc_vmalloc_start: *mut core::ffi::c_void;
}

pub const PCXL_DMA_MAP_SIZE: usize = 8 * 1024 * 1024;
pub const VMALLOC_START: usize = parisc_vmalloc_start as usize;
pub const VMALLOC_END: usize = KERNEL_MAP_END;

#[inline]
pub const fn __fix_to_virt(_x: usize) -> usize {
    FIXMAP_START + (_x << PAGE_SHIFT)
}

extern "C" {
    pub fn set_fixmap(idx: fixed_addresses, phys: phys_addr_t);
    pub fn clear_fixmap(idx: fixed_addresses);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
