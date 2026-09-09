/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2022 Amarula Solutions(India)
 * Author: Jagan Teki <jagan@amarulasolutions.com>
 */

// C header dependencies are supplied by other translated units.

#[repr(C)]
pub struct platform_device {
    _unused: [u8; 0],
}

pub const DSIM_STATE_ENABLED: u32 = 1u32 << 0;
pub const DSIM_STATE_INITIALIZED: u32 = 1u32 << 1;
pub const DSIM_STATE_CMD_LPM: u32 = 1u32 << 2;
pub const DSIM_STATE_VIDOUT_AVAILABLE: u32 = 1u32 << 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum samsung_dsim_type {
    DSIM_TYPE_EXYNOS3250,
    DSIM_TYPE_EXYNOS4210,
    DSIM_TYPE_EXYNOS5410,
    DSIM_TYPE_EXYNOS5422,
    DSIM_TYPE_EXYNOS5433,
    DSIM_TYPE_EXYNOS7870,
    DSIM_TYPE_IMX8MM,
    DSIM_TYPE_IMX8MP,
    DSIM_TYPE_COUNT,
}

#[inline]
pub const fn samsung_dsim_hw_is_exynos(hw: samsung_dsim_type) -> bool {
    (hw as isize) >= (samsung_dsim_type::DSIM_TYPE_EXYNOS3250 as isize)
        && (hw as isize) <= (samsung_dsim_type::DSIM_TYPE_EXYNOS5433 as isize)
}

#[repr(C)]
pub struct samsung_dsim_transfer {
    pub list: list_head,
    pub completed: completion,
    pub result: ::core::ffi::c_int,
    pub packet: mipi_dsi_packet,
    pub flags: u16,
    pub tx_done: u16,
    pub rx_payload: *mut u8,
    pub rx_len: u16,
    pub rx_done: u16,
}

#[repr(C)]
pub struct samsung_dsim_driver_data {
    pub reg_ofs: *const u32,
    pub plltmr_reg: u32,
    // C unsigned int bit-fields, each one bit wide.
    pub has_legacy_status_reg: u32,
    pub has_freqband: u32,
    pub has_clklane_stop: u32,
    pub has_broken_fifoctrl_emptyhdr: u32,
    pub has_sfrctrl: u32,
    pub clk_data: *mut clk_bulk_data,
    pub num_clks: u32,
    pub min_freq: u32,
    pub max_freq: u32,
    pub wait_for_hdr_fifo: u32,
    pub wait_for_reset: u32,
    pub num_bits_resol: u32,
    pub video_mode_bit: u32,
    pub pll_stable_bit: u32,
    pub esc_clken_bit: u32,
    pub byte_clken_bit: u32,
    pub tx_req_hsclk_bit: u32,
    pub lane_esc_clk_bit: u32,
    pub lane_esc_data_offset: u32,
    pub pll_p_offset: u32,
    pub pll_m_offset: u32,
    pub pll_s_offset: u32,
    pub main_vsa_offset: u32,
    pub reg_values: *const u32,
    pub pll_fin_min: u32,
    pub pll_fin_max: u32,
    pub m_min: u16,
    pub m_max: u16,
}

#[repr(C)]
pub struct samsung_dsim_host_ops {
    pub register_host: Option<unsafe extern "C" fn(*mut samsung_dsim) -> ::core::ffi::c_int>,
    pub unregister_host: Option<unsafe extern "C" fn(*mut samsung_dsim)>,
    pub attach: Option<unsafe extern "C" fn(*mut samsung_dsim, *mut mipi_dsi_device) -> ::core::ffi::c_int>,
    pub detach: Option<unsafe extern "C" fn(*mut samsung_dsim, *mut mipi_dsi_device)>,
    pub te_irq_handler: Option<unsafe extern "C" fn(*mut samsung_dsim) -> irqreturn_t>,
}

#[repr(C)]
pub struct samsung_dsim_plat_data {
    pub hw_type: samsung_dsim_type,
    pub host_ops: *const samsung_dsim_host_ops,
}

#[repr(C)]
pub struct samsung_dsim {
    pub dsi_host: mipi_dsi_host,
    pub bridge: drm_bridge,
    pub dev: *mut device,
    pub mode: drm_display_mode,
    pub reg_base: *mut ::core::ffi::c_void,
    pub phy: *mut phy,
    pub pll_clk: *mut clk,
    pub supplies: [regulator_bulk_data; 2],
    pub irq: ::core::ffi::c_int,
    pub te_gpio: *mut gpio_desc,
    pub pll_clk_rate: u32,
    pub burst_clk_rate: u32,
    pub hs_clock: u32,
    pub esc_clk_rate: u32,
    pub lanes: u32,
    pub mode_flags: u32,
    pub format: u32,
    pub swap_dn_dp_clk: bool,
    pub swap_dn_dp_data: bool,
    pub state: ::core::ffi::c_int,
    pub brightness: *mut drm_property,
    pub completed: completion,
    // protects transfer_list
    pub transfer_lock: spinlock_t,
    pub transfer_list: list_head,
    pub driver_data: *const samsung_dsim_driver_data,
    pub plat_data: *const samsung_dsim_plat_data,
    pub priv_: *mut ::core::ffi::c_void,
}

extern "C" {
    pub fn samsung_dsim_probe(pdev: *mut platform_device) -> ::core::ffi::c_int;
    pub fn samsung_dsim_remove(pdev: *mut platform_device);
    pub static samsung_dsim_pm_ops: dev_pm_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
