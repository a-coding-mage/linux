/* SPDX-License-Identifier: GPL-2.0 */

// The C header declares this externally supplied DMA operations type.
#[repr(C)]
pub struct dma_map_ops {
    _private: [u8; 0],
}

pub unsafe extern "C" {
    pub static dma_ops: *const dma_map_ops;
}

#[inline]
pub fn get_arch_dma_ops() -> *const dma_map_ops {
    /* sparc32 uses per-device dma_ops */
    if cfg!(feature = "CONFIG_SPARC64") {
        // Equivalent to the C IS_ENABLED(CONFIG_SPARC64) branch.
        unsafe { dma_ops }
    } else {
        core::ptr::null()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
