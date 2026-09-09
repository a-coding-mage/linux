/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * Apple SART device driver
 * Copyright (C) The Asahi Linux Contributors
 *
 * Apple SART is a simple address filter for DMA transactions.
 * Regions of physical memory must be added to the SART's allow
 * list before any DMA can target these. Unlike a proper
 * IOMMU no remapping can be done.
 */

// C dependencies supplied by the surrounding Linux translation:
// linux/device.h, linux/err.h, and linux/types.h.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct apple_sart {
    _private: [u8; 0],
}

// Linux phys_addr_t is an unsigned physical address type.
pub type phys_addr_t = u64;

/*
 * Get a reference to the SART attached to dev.
 *
 * Looks for the phandle reference in apple,sart and returns a pointer
 * to the corresponding apple_sart struct to be used with
 * apple_sart_add_allowed_region and apple_sart_remove_allowed_region.
 */
extern "C" {
    pub fn devm_apple_sart_get(dev: *mut device) -> *mut apple_sart;
}

/*
 * Adds the region [paddr, paddr+size] to the DMA allow list.
 *
 * @sart: SART reference
 * @paddr: Start address of the region to be used for DMA
 * @size: Size of the region to be used for DMA.
 */
extern "C" {
    pub fn apple_sart_add_allowed_region(
        sart: *mut apple_sart,
        paddr: phys_addr_t,
        size: usize,
    ) -> i32;
}

/*
 * Removes the region [paddr, paddr+size] from the DMA allow list.
 *
 * Note that exact same paddr and size used for apple_sart_add_allowed_region
 * have to be passed.
 *
 * @sart: SART reference
 * @paddr: Start address of the region no longer used for DMA
 * @size: Size of the region no longer used for DMA.
 */
extern "C" {
    pub fn apple_sart_remove_allowed_region(
        sart: *mut apple_sart,
        paddr: phys_addr_t,
        size: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
