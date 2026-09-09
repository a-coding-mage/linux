/* SPDX-License-Identifier: GPL-2.0-only */
/* include/linux/platform_data/s3c-hsotg.h
 *
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      Ben Dooks <ben@simtec.co.uk>
 *      http://armlinux.simtec.co.uk/
 *
 * S3C USB2.0 High-speed / OtG platform information
 */

// C header guard: __LINUX_USB_S3C_HSOTG_H

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dwc2_hsotg_dmamode {
    S3C_HSOTG_DMA_NONE,
    S3C_HSOTG_DMA_ONLY,
    S3C_HSOTG_DMA_DRV,
}

/**
 * struct dwc2_hsotg_plat - platform data for high-speed otg/udc
 * @dma: Whether to use DMA or not.
 * @is_osc: The clock source is an oscillator, not a crystal
 */
#[repr(C)]
pub struct dwc2_hsotg_plat {
    pub dma: dwc2_hsotg_dmamode,
    // C bit-field: unsigned int is_osc:1;
    pub is_osc: u32,
    pub phy_type: core::ffi::c_int,

    pub phy_init: Option<unsafe extern "C" fn(
        pdev: *mut platform_device,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int>,
    pub phy_exit: Option<unsafe extern "C" fn(
        pdev: *mut platform_device,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int>,
}

unsafe extern "C" {
    pub fn dwc2_hsotg_set_platdata(pd: *mut dwc2_hsotg_plat);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
