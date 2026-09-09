/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm/dma-mapping.h. */

/* Dependency supplied by linux/swiotlb.h. */

extern "C" {
    pub static jazz_dma_ops: dma_map_ops;
}

#[inline]
pub unsafe fn get_arch_dma_ops() -> *const dma_map_ops {
    /* Preserves the source build-time CONFIG_MACH_JAZZ condition. */
    #[cfg(CONFIG_MACH_JAZZ)]
    {
        &jazz_dma_ops as *const dma_map_ops
    }

    #[cfg(not(CONFIG_MACH_JAZZ))]
    {
        core::ptr::null()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
