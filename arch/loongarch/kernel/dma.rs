// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

use core::ffi::c_int;

#[repr(C)]
pub struct bus_dma_region {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub bus_dma_limit: u64,
    pub dma_range_map: *const bus_dma_region,
    pub coherent_dma_mask: u64,
    pub dma_mask: *mut u64,
}

extern "C" {
    pub fn acpi_dma_get_range(
        dev: *mut device,
        map: *mut *const bus_dma_region,
    ) -> c_int;
    pub fn dma_range_map_max(map: *const bus_dma_region) -> u64;
    pub fn ilog2(value: u64) -> u32;
}

#[inline]
unsafe fn dma_bit_mask(bits: u32) -> u64 {
    if bits >= 64 {
        u64::MAX
    } else {
        (1u64 << bits).wrapping_sub(1)
    }
}

pub unsafe fn acpi_arch_dma_setup(dev: *mut device) {
    let ret: c_int;
    let mask: u64;
    let end: u64;
    let mut map: *const bus_dma_region = core::ptr::null();

    ret = acpi_dma_get_range(dev, &mut map);
    if ret == 0 && !map.is_null() {
        end = dma_range_map_max(map);

        mask = dma_bit_mask(ilog2(end).wrapping_add(1));
        (*dev).bus_dma_limit = end;
        (*dev).dma_range_map = map;
        (*dev).coherent_dma_mask = (*dev).coherent_dma_mask.min(mask);
        *(*dev).dma_mask = (*(*dev).dma_mask).min(mask);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
