// SPDX-License-Identifier: GPL-2.0
// Dependency provided by <linux/dma-direct.h> in the original source.

pub unsafe extern "C" fn phys_to_dma(
    _dev: *mut device,
    paddr: phys_addr_t,
) -> dma_addr_t {
    paddr | 0x8000_0000
}

pub unsafe extern "C" fn dma_to_phys(
    _dev: *mut device,
    dma_addr: dma_addr_t,
) -> phys_addr_t {
    if dma_addr > 0x8fff_ffff {
        return dma_addr;
    }
    dma_addr & 0x0fff_ffff
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
