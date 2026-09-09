/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * 10G controller driver for Samsung Exynos SoCs
 *
 * Copyright (C) 2013 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Author: Siva Reddy Kallam <siva.kallam@samsung.com>
 */

// Dependency supplied by the Linux PHY interface headers:
// use the externally defined `phy_interface_t` type here.

use core::ffi::{c_char, c_int, c_uint};

/* MDC Clock Selection define */
pub const SXGBE_CSR_100_150M: c_uint = 0x0; /* MDC = clk_scr_i/62 */
pub const SXGBE_CSR_150_250M: c_uint = 0x1; /* MDC = clk_scr_i/102 */
pub const SXGBE_CSR_250_300M: c_uint = 0x2; /* MDC = clk_scr_i/122 */
pub const SXGBE_CSR_300_350M: c_uint = 0x3; /* MDC = clk_scr_i/142 */
pub const SXGBE_CSR_350_400M: c_uint = 0x4; /* MDC = clk_scr_i/162 */
pub const SXGBE_CSR_400_500M: c_uint = 0x5; /* MDC = clk_scr_i/202 */

/* Platfrom data for platform device structure's
 * platform_data field
 */
#[repr(C)]
pub struct sxgbe_mdio_bus_data {
    pub phy_mask: c_uint,
    pub irqs: *mut c_int,
    pub probed_phy_irq: c_int,
}

#[repr(C)]
pub struct sxgbe_dma_cfg {
    pub pbl: c_int,
    pub fixed_burst: c_int,
    pub burst_map: c_int,
    pub adv_addr_mode: c_int,
}

#[repr(C)]
pub struct sxgbe_plat_data {
    pub phy_bus_name: *mut c_char,
    pub bus_id: c_int,
    pub phy_addr: c_int,
    pub interface: phy_interface_t,
    pub mdio_bus_data: *mut sxgbe_mdio_bus_data,
    pub dma_cfg: *mut sxgbe_dma_cfg,
    pub clk_csr: c_int,
    pub pmt: c_int,
    pub force_sf_dma_mode: c_int,
    pub force_thresh_dma_mode: c_int,
    pub riwt_off: c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
