/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the MIPS DMA direct header.

extern "C" {
    pub fn phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t;
    pub fn dma_to_phys(dev: *mut device, daddr: dma_addr_t) -> phys_addr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
