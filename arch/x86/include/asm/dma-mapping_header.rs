/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_X86_DMA_MAPPING_H

// Supplied by an external dependency in the translated source.
#[repr(C)]
pub struct dma_map_ops {
    _private: [u8; 0],
}

extern "C" {
    pub static dma_ops: *const dma_map_ops;
}

pub unsafe fn get_arch_dma_ops() -> *const dma_map_ops {
    dma_ops
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
