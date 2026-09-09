// SPDX-License-Identifier: GPL-1.0+
/*
 * Renesas USB
 *
 * Copyright (C) 2011 Renesas Solutions Corp.
 * Copyright (C) 2019 Renesas Electronics Corporation
 * Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */

/* Dependencies: linux/notifier.h, linux/platform_device.h, linux/usb/ch9.h. */

/*
 * module type
 *
 * it will be return value from get_id
 */
pub const USBHS_HOST: i32 = 0;
pub const USBHS_GADGET: i32 = 1;
pub const USBHS_MAX: i32 = 2;

/*
 * callback functions for platform
 *
 * These functions are called from driver for platform
 */
#[repr(C)]
pub struct renesas_usbhs_platform_callback {
    /* Hardware init function for platform; called when driver was probed. */
    pub hardware_init: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
    /* Hardware exit function for platform; called when driver was removed. */
    pub hardware_exit: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
    /* Board-specific clock control. */
    pub power_ctrl: Option<unsafe extern "C" fn(pdev: *mut platform_device, base: *mut core::ffi::c_void, enable: i32) -> i32>,
    /* Phy reset for platform. */
    pub phy_reset: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
    /* Get USB ID function: USBHS_HOST or USBHS_GADGET. */
    pub get_id: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
    /* Get VBUS status function. */
    pub get_vbus: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
    /* VBUS control is needed for Host. */
    pub set_vbus: Option<unsafe extern "C" fn(pdev: *mut platform_device, enable: i32) -> i32>,
    /* Extcon notifier to set host/peripheral mode. */
    pub notifier: Option<unsafe extern "C" fn(nb: *mut notifier_block, event: c_ulong, data: *mut core::ffi::c_void) -> i32>,
}

/* Parameters for renesas usbhs; some registers need USB chip-specific parameters. */
#[repr(C)]
pub struct renesas_usbhs_driver_pipe_config {
    pub type_: u8, /* USB_ENDPOINT_XFER_xxx */
    pub bufsize: u16,
    pub bufnum: u8,
    pub double_buf: bool,
}

#[macro_export]
macro_rules! RENESAS_USBHS_PIPE {
    ($type:expr, $size:expr, $num:expr, $double_buf:expr) => {
        renesas_usbhs_driver_pipe_config {
            type_: $type,
            bufsize: $size,
            bufnum: $num,
            double_buf: $double_buf,
        }
    };
}

#[repr(C)]
pub struct renesas_usbhs_driver_param {
    /* pipe settings */
    pub pipe_configs: *mut renesas_usbhs_driver_pipe_config,
    pub pipe_size: i32, /* pipe_configs array size */

    /* for BUSWAIT :: BWAIT; see renesas_usbhs/common.c :: usbhsc_set_buswait() */
    pub buswait_bwait: i32,
    /* Delay time from notify_hotplug callback, in msec. */
    pub detection_delay: i32,

    /* DMA IDs for dmaengine. */
    pub d0_tx_id: i32,
    pub d0_rx_id: i32,
    pub d1_tx_id: i32,
    pub d1_rx_id: i32,
    pub d2_tx_id: i32,
    pub d2_rx_id: i32,
    pub d3_tx_id: i32,
    pub d3_rx_id: i32,

    /* PIO <--> DMA border; default is 64 byte. */
    pub pio_dma_border: i32,

    /* C bit-fields occupy one u32 storage unit. */
    pub feature_flags: u32,
}

pub const USBHS_USB_DMAC_XFER_SIZE: u32 = 32; /* hardcode the xfer size */

#[repr(C)]
pub struct renesas_usbhs_platform_info {
    /* Platform callback functions. */
    pub platform_callback: renesas_usbhs_platform_callback,
    /* Driver parameters for some registers. */
    pub driver_param: renesas_usbhs_driver_param,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
