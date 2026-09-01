/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2026 Baylibre SAS.
 * Author: Valerio Setti <vsetti@baylibre.com>
 */

/* Dependencies in the original C header:
 * <linux/clk.h>
 * <linux/regmap.h>
 * <sound/pcm.h>
 * <sound/soc.h>
 * <sound/soc-dai.h>
 */

#[repr(C)]
pub struct gx_iface {
    pub mclk: *mut clk,
    pub mclk_rate: core::ffi::c_ulong,

    /* format is common to all the DAIs of the iface */
    pub fmt: core::ffi::c_uint,
}

#[repr(C)]
pub struct gx_stream {
    pub iface: *mut gx_iface,
    pub formatter_list: list_head,
    pub lock: mutex,
    pub channels: core::ffi::c_uint,
    pub width: core::ffi::c_uint,
    pub physical_width: core::ffi::c_uint,
    pub ready: bool,

    /* For continuous clock tracking */
    pub clk_enabled: bool,
}

unsafe extern "C" {
    pub fn gx_stream_alloc(iface: *mut gx_iface) -> *mut gx_stream;
    pub fn gx_stream_free(ts: *mut gx_stream);
    pub fn gx_stream_start(ts: *mut gx_stream) -> core::ffi::c_int;
    pub fn gx_stream_stop(ts: *mut gx_stream);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
