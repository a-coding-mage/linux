/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Author: Kevin Wells <kevin.wells@nxp.com>
 *
 * Copyright (C) 2010 NXP Semiconductors
 */

// Dependencies corresponding to <linux/types.h> and <linux/phy.h> are
// supplied by the surrounding translation unit.

#[cfg(CONFIG_ARCH_LPC32XX)]
extern "C" {
    pub fn lpc32xx_return_iram(
        mapbase: *mut *mut core::ffi::c_void,
        dmaaddr: *mut dma_addr_t,
    ) -> u32;
    pub fn lpc32xx_set_phy_interface_mode(mode: phy_interface_t);
    pub fn lpc32xx_loopback_set(mapbase: resource_size_t, state: i32);
}

#[cfg(not(CONFIG_ARCH_LPC32XX))]
pub unsafe fn lpc32xx_return_iram(
    mapbase: *mut *mut core::ffi::c_void,
    dmaaddr: *mut dma_addr_t,
) -> u32 {
    *mapbase = core::ptr::null_mut();
    *dmaaddr = 0;
    0
}

#[cfg(not(CONFIG_ARCH_LPC32XX))]
pub fn lpc32xx_set_phy_interface_mode(_mode: phy_interface_t) {}

#[cfg(not(CONFIG_ARCH_LPC32XX))]
pub fn lpc32xx_loopback_set(_mapbase: resource_size_t, _state: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
