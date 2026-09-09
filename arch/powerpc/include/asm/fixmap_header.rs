/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Ingo Molnar
 *
 * Copyright 2008 Freescale Semiconductor Inc.
 *   Port to powerpc added by Kumar Gala
 */

// C header guard: _ASM_FIXMAP_H
// C-only includes are supplied by the surrounding translation unit:
// linux/sizes.h, linux/pgtable.h, asm/page.h, linux/threads.h,
// asm/kmap_size.h, and asm-generic/fixmap.h.
// The following declarations are intended for non-assembler builds.

/*
 * Here we define all the compile-time 'special' virtual
 * addresses. The point is to have a constant address at
 * compile time, but to set the physical address only
 * in the boot process. We allocate these special addresses
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

// C enum fixed_addresses. Configuration-dependent members retain their
// original conditional intent; the constants below use the supplied kernel
// constants and macros as external dependencies.
pub const FIX_HOLE: usize = 0;

// CONFIG_PPC32
// reserve the top 128K for early debugging purposes
pub const FIX_EARLY_DEBUG_TOP: usize = FIX_HOLE;
pub const FIX_EARLY_DEBUG_BASE: usize =
    FIX_EARLY_DEBUG_TOP + (ALIGN(SZ_128K, PAGE_SIZE) / PAGE_SIZE) - 1;

// CONFIG_HIGHMEM
// reserved pte's for temporary kernel mappings
pub const FIX_KMAP_BEGIN: usize = FIX_EARLY_DEBUG_BASE + 1;
pub const FIX_KMAP_END: usize = FIX_KMAP_BEGIN + (KM_MAX_IDX * NR_CPUS) - 1;

// CONFIG_PPC_8xx
// For IMMR we need an aligned 512K area
pub const FIX_IMMR_SIZE: usize = 512 * 1024 / PAGE_SIZE;
pub const FIX_IMMR_START: usize = FIX_KMAP_END + 1;
pub const FIX_IMMR_BASE: usize =
    __ALIGN_MASK(FIX_IMMR_START, FIX_IMMR_SIZE - 1) - 1 + FIX_IMMR_SIZE;

// CONFIG_PPC_83xx
// For IMMR we need an aligned 2M area
pub const FIX_IMMR_SIZE_83XX: usize = SZ_2M / PAGE_SIZE;
pub const FIX_IMMR_START_83XX: usize = FIX_IMMR_BASE + 1;
pub const FIX_IMMR_BASE_83XX: usize = __ALIGN_MASK(
    FIX_IMMR_START_83XX,
    FIX_IMMR_SIZE_83XX - 1,
) - 1 + FIX_IMMR_SIZE_83XX;

// FIX_PCIE_MCFG,
pub const __end_of_permanent_fixed_addresses: usize = FIX_IMMR_BASE_83XX + 1;

pub const NR_FIX_BTMAPS: usize = SZ_256K / PAGE_SIZE;
pub const FIX_BTMAPS_SLOTS: usize = 16;
pub const TOTAL_FIX_BTMAPS: usize = NR_FIX_BTMAPS * FIX_BTMAPS_SLOTS;

pub const FIX_BTMAP_END: usize = __end_of_permanent_fixed_addresses;
pub const FIX_BTMAP_BEGIN: usize = FIX_BTMAP_END + TOTAL_FIX_BTMAPS - 1;
pub const __end_of_fixed_addresses: usize = FIX_BTMAP_BEGIN + 1;

pub const __FIXADDR_SIZE: usize = __end_of_fixed_addresses << PAGE_SHIFT;
pub const FIXADDR_START: usize = FIXADDR_TOP - __FIXADDR_SIZE;

pub const FIXMAP_ALIGNED_SIZE: usize =
    ALIGN(FIXADDR_TOP, PGDIR_SIZE) - ALIGN_DOWN(FIXADDR_START, PGDIR_SIZE);
pub const FIXMAP_PTE_SIZE: usize =
    FIXMAP_ALIGNED_SIZE / PGDIR_SIZE * PTE_TABLE_SIZE;

pub const FIXMAP_PAGE_NOCACHE: pgprot_t = PAGE_KERNEL_NCG;
pub const FIXMAP_PAGE_IO: pgprot_t = PAGE_KERNEL_NCG;

extern "C" {
    fn __fix_to_virt(idx: usize) -> usize;
    fn map_kernel_page(virt: usize, phys: phys_addr_t, flags: pgprot_t);
    fn unmap_kernel_page(virt: usize);
}

#[inline]
pub unsafe fn __set_fixmap(idx: usize, phys: phys_addr_t, flags: pgprot_t) {
    // BUILD_BUG_ON(IS_ENABLED(CONFIG_PPC64) && __FIXADDR_SIZE > FIXADDR_SIZE);
    // if (__builtin_constant_p(idx)) BUILD_BUG_ON(idx >= __end_of_fixed_addresses);
    if idx >= __end_of_fixed_addresses {
        // WARN_ON(idx >= __end_of_fixed_addresses)
        return;
    }
    if pgprot_val(flags) != 0 {
        map_kernel_page(__fix_to_virt(idx), phys, flags);
    } else {
        unmap_kernel_page(__fix_to_virt(idx));
    }
}

// #define __early_set_fixmap __set_fixmap
pub use __set_fixmap as __early_set_fixmap;

// CONFIG_PPC_8xx
pub const VIRT_IMMR_BASE: usize = __fix_to_virt(FIX_IMMR_BASE);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
