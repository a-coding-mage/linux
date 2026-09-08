/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declaration supplied by the DMA mapping interface.
pub enum dma_map_ops {}

unsafe extern "C" {
    pub static alpha_pci_ops: dma_map_ops;
}

#[inline]
pub unsafe fn get_arch_dma_ops() -> *const dma_map_ops {
    &raw const alpha_pci_ops
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
