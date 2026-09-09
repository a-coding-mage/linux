// SPDX-License-Identifier: GPL-2.0
//
// C dependency: <linux/dma-direct.h>

// External types supplied by the surrounding Linux compatibility layer.
pub type dma_addr_t = u32;
pub type phys_addr_t = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub unsafe extern "C" fn phys_to_dma(
    _dev: *mut device,
    paddr: phys_addr_t,
) -> dma_addr_t {
    paddr | 0x80000000
}

pub unsafe extern "C" fn dma_to_phys(
    _dev: *mut device,
    dma_addr: dma_addr_t,
) -> phys_addr_t {
    dma_addr & 0x7fffffff
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
