// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Based on linux/arch/arm/mm/dma-mapping.c
 *
 *  Copyright (C) 2000-2004 Russell King
 */

// The declarations below are supplied by the corresponding kernel headers.
use core::ffi::c_void;

pub type phys_addr_t = usize;
pub type size_t = usize;
pub type dma_data_direction = i32;

pub const DMA_TO_DEVICE: dma_data_direction = 0;
pub const DMA_FROM_DEVICE: dma_data_direction = 1;

unsafe extern "C" {
    fn dmac_map_area(addr: *mut c_void, size: size_t, dir: dma_data_direction);
    fn dmac_unmap_area(addr: *mut c_void, size: size_t, dir: dma_data_direction);
    fn __va(paddr: phys_addr_t) -> *mut c_void;
    fn outer_inv_range(start: phys_addr_t, end: phys_addr_t);
    fn outer_clean_range(start: phys_addr_t, end: phys_addr_t);
    fn dev_assign_dma_coherent(dev: *mut device, coherent: bool);
    fn get_cr() -> u32;

    static cacheid: u32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub const CR_M: u32 = 1 << 0;

pub unsafe fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: size_t,
    dir: dma_data_direction,
) {
    dmac_map_area(__va(paddr), size, dir);

    if dir == DMA_FROM_DEVICE {
        outer_inv_range(paddr, paddr.wrapping_add(size));
    } else {
        outer_clean_range(paddr, paddr.wrapping_add(size));
    }
}

pub unsafe fn arch_sync_dma_for_cpu(
    paddr: phys_addr_t,
    size: size_t,
    dir: dma_data_direction,
) {
    if dir != DMA_TO_DEVICE {
        outer_inv_range(paddr, paddr.wrapping_add(size));
        dmac_unmap_area(__va(paddr), size, dir);
    }
}

pub unsafe fn arch_setup_dma_ops(dev: *mut device, coherent: bool) {
    // CONFIG_CPU_V7M is a build-time kernel configuration condition.
    if cfg!(feature = "CONFIG_CPU_V7M") {
        /*
         * Cache support for v7m is optional, so can be treated as
         * coherent if no cache has been detected. Note that it is not
         * enough to check if MPU is in use or not since in absence of
         * MPU system memory map is used.
         */
        dev_assign_dma_coherent(dev, if cacheid != 0 { coherent } else { true });
    } else {
        /*
         * Assume coherent DMA in case MMU/MPU has not been set up.
         */
        dev_assign_dma_coherent(dev, if (get_cr() & CR_M) != 0 { coherent } else { true });
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
