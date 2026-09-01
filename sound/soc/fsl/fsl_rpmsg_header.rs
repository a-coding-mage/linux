/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2017-2021 NXP
 */

/*
 * External C types from included kernel headers.
 */
pub enum clk {}
pub enum platform_device {}

/*
 * struct fsl_rpmsg_soc_data
 * @rates: supported rates
 * @formats: supported formats
 */
#[repr(C)]
pub struct fsl_rpmsg_soc_data {
    pub rates: ::core::ffi::c_int,
    pub formats: u64,
}

/*
 * struct fsl_rpmsg - rpmsg private data
 *
 * @ipg: ipg clock for cpu dai (SAI)
 * @mclk: master clock for cpu dai (SAI)
 * @dma: clock for dma device
 * @pll8k: parent clock for multiple of 8kHz frequency
 * @pll11k: parent clock for multiple of 11kHz frequency
 * @card_pdev: Platform_device pointer to register a sound card
 * @soc_data: soc specific data
 * @mclk_streams: Active streams that are using baudclk
 * @force_lpa: force enable low power audio routine if condition satisfy
 * @enable_lpa: enable low power audio routine according to dts setting
 * @buffer_size: pre allocated dma buffer size
 */
#[repr(C)]
pub struct fsl_rpmsg {
    pub ipg: *mut clk,
    pub mclk: *mut clk,
    pub dma: *mut clk,
    pub pll8k: *mut clk,
    pub pll11k: *mut clk,
    pub card_pdev: *mut platform_device,
    pub soc_data: *const fsl_rpmsg_soc_data,
    pub mclk_streams: ::core::ffi::c_uint,
    pub force_lpa: ::core::ffi::c_int,
    pub enable_lpa: ::core::ffi::c_int,
    pub buffer_size: [::core::ffi::c_int; 2],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
