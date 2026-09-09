/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for OMAP-UART controller.
 * Based on drivers/serial/8250.c
 *
 * Copyright (C) 2010 Texas Instruments.
 *
 * Authors:
 *	Govindraj R	<govindraj.raja@ti.com>
 *	Thara Gopinath	<thara@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined in this header translation.

pub const OMAP_SERIAL_DRIVER_NAME: &str = "omap_uart";

/*
 * Use tty device name as ttyO, [O -> OMAP]
 * in bootargs we specify as console=ttyO0 if uart1
 * is used as console uart.
 */
pub const OMAP_SERIAL_NAME: &str = "ttyO";

#[repr(C)]
pub struct omap_uart_port_info {
    pub dma_enabled: bool, /* To specify DMA Mode */
    pub uartclk: ::core::ffi::c_uint, /* UART clock rate */
    pub flags: upf_t, /* UPF_* flags */
    pub dma_rx_buf_size: ::core::ffi::c_uint,
    pub dma_rx_timeout: ::core::ffi::c_uint,
    pub autosuspend_timeout: ::core::ffi::c_uint,
    pub dma_rx_poll_rate: ::core::ffi::c_uint,

    pub get_context_loss_count:
        Option<unsafe extern "C" fn(*mut device) -> ::core::ffi::c_int>,
    pub enable_wakeup: Option<unsafe extern "C" fn(*mut device, bool)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
