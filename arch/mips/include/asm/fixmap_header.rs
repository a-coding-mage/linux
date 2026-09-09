/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Ingo Molnar
 *
 * Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
 */

// C header guard: _ASM_FIXMAP_H

// Dependencies supplied by other translated headers:
// asm/page.h, spaces.h, and, when CONFIG_HIGHMEM is enabled,
// linux/threads.h and asm/kmap_size.h.

/*
 * Here we define all the compile-time 'special' virtual
 * addresses. The point is to have a constant address at
 * compile time, but to set the physical address only
 * in the boot process. We allocate these special  addresses
 * from the end of virtual memory (0xfffff000) backwards.
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
pub const FIX_CMAP_BEGIN: usize = 0;
pub const FIX_CMAP_END: usize = FIX_CMAP_BEGIN + (FIX_N_COLOURS * 2);

// CONFIG_HIGHMEM conditionally reserves PTEs for temporary kernel mappings.
#[cfg(CONFIG_HIGHMEM)]
pub const FIX_KMAP_BEGIN: usize = FIX_CMAP_END + 1;
#[cfg(CONFIG_HIGHMEM)]
pub const FIX_KMAP_END: usize = FIX_KMAP_BEGIN + (KM_MAX_IDX * NR_CPUS) - 1;

#[cfg(CONFIG_HIGHMEM)]
pub const __END_OF_FIXED_ADDRESSES: usize = FIX_KMAP_END;
#[cfg(not(CONFIG_HIGHMEM))]
pub const __END_OF_FIXED_ADDRESSES: usize = FIX_CMAP_END;

/*
 * used by vmalloc.c.
 *
 * Leave one empty page between vmalloc'ed areas and
 * the start of the fixmap, and leave one page empty
 * at the top of mem..
 */
pub const FIXADDR_SIZE: usize = __END_OF_FIXED_ADDRESSES << PAGE_SHIFT;
pub const FIXADDR_START: usize = FIXADDR_TOP - FIXADDR_SIZE;

// Declarations from asm-generic/fixmap.h are supplied by another translation.

/*
 * Called from pagetable_init()
 */
extern "C" {
    pub fn fixrange_init(start: core::ffi::c_ulong,
                         end: core::ffi::c_ulong,
                         pgd_base: *mut pgd_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
