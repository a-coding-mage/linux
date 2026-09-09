// SPDX-License-Identifier: GPL-2.0-only
/*
 * Exported ksyms for the SSI FIQ handler
 *
 * Copyright (C) 2009, Sascha Hauer <s.hauer@pengutronix.de>
 */

// The declarations below are provided by the SSI implementation/header.
// The C EXPORT_SYMBOL declarations make these names available to modules.
extern "C" {
    pub static mut imx_ssi_fiq_tx_buffer: core::ffi::c_void;
    pub static mut imx_ssi_fiq_rx_buffer: core::ffi::c_void;
    pub fn imx_ssi_fiq_start();
    pub fn imx_ssi_fiq_end();
    pub static mut imx_ssi_fiq_base: core::ffi::c_void;
}

// EXPORT_SYMBOL(imx_ssi_fiq_tx_buffer);
// EXPORT_SYMBOL(imx_ssi_fiq_rx_buffer);
// EXPORT_SYMBOL(imx_ssi_fiq_start);
// EXPORT_SYMBOL(imx_ssi_fiq_end);
// EXPORT_SYMBOL(imx_ssi_fiq_base);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
