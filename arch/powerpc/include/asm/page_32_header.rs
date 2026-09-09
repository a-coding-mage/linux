/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the C header `asm/page_32.h`.
//
// The following build-time checks and configuration-dependent definitions are
// retained as comments because their configuration symbols are supplied by
// the surrounding build.
// #if defined(CONFIG_PHYSICAL_ALIGN) && (CONFIG_PHYSICAL_START != 0)
// #if (CONFIG_PHYSICAL_START % CONFIG_PHYSICAL_ALIGN) != 0
// #error "CONFIG_PHYSICAL_START must be a multiple of CONFIG_PHYSICAL_ALIGN"
// #endif
// #endif

pub const VMA_DATA_DEFAULT_FLAGS: _ = VMA_DATA_DEFAULT_FLAGS32;

// #if defined(CONFIG_PPC_256K_PAGES) || \
//     (defined(CONFIG_PPC_8xx) && defined(CONFIG_PPC_16K_PAGES))
// #define PTE_SHIFT (PAGE_SHIFT - PTE_T_LOG2 - 2) /* 1/4 of a page */
// #else
// #define PTE_SHIFT (PAGE_SHIFT - PTE_T_LOG2) /* full page */
// #endif
#[cfg(any(CONFIG_PPC_256K_PAGES, all(CONFIG_PPC_8xx, CONFIG_PPC_16K_PAGES)))]
pub const PTE_SHIFT: _ = PAGE_SHIFT - PTE_T_LOG2 - 2;
#[cfg(not(any(CONFIG_PPC_256K_PAGES, all(CONFIG_PPC_8xx, CONFIG_PPC_16K_PAGES))))]
pub const PTE_SHIFT: _ = PAGE_SHIFT - PTE_T_LOG2;

/*
 * The basic type of a PTE - 64 bits for those CPUs with > 32 bit
 * physical addressing.
 */
#[cfg(CONFIG_PTE_64BIT)]
pub type pte_basic_t = u64;
#[cfg(not(CONFIG_PTE_64BIT))]
pub type pte_basic_t = usize;

/*
 * Clear page using the dcbz instruction, which doesn't cause any
 * memory traffic (except to write out any cache lines which get
 * displaced).  This only works on cacheable memory.
 */
pub unsafe fn clear_page(mut addr: *mut core::ffi::c_void) {
    let mut i: u32;

    // WARN_ON((unsigned long)addr & (L1_CACHE_BYTES - 1));
    if (addr as usize) & (L1_CACHE_BYTES - 1) != 0 {
        // Dependency supplied by the surrounding kernel translation.
        unsafe { WARN_ON(true); }
    }

    i = 0;
    while i < PAGE_SIZE / L1_CACHE_BYTES {
        unsafe { dcbz(addr); }
        addr = (addr as *mut u8).add(L1_CACHE_BYTES) as *mut core::ffi::c_void;
        i += 1;
    }
}

unsafe extern "C" {
    fn WARN_ON(condition: bool) -> bool;
    fn dcbz(addr: *mut core::ffi::c_void);
    fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
}

// #include <asm-generic/getorder.h>

pub const PGD_T_LOG2: _ = (core::mem::size_of::<pgd_t>() as usize).trailing_zeros() - 1;
pub const PTE_T_LOG2: _ = (core::mem::size_of::<pte_t>() as usize).trailing_zeros() - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
