/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP GPMC Platform data
 *
 * Copyright (C) 2014 Texas Instruments, Inc. - https://www.ti.com
 *	Roger Quadros <rogerq@ti.com>
 */

/* Maximum Number of Chip Selects */
pub const GPMC_CS_NUM: usize = 8;

/* bool type time settings */
#[repr(C)]
pub struct gpmc_bool_timings {
    pub cycle2cyclediffcsen: bool,
    pub cycle2cyclesamecsen: bool,
    pub we_extra_delay: bool,
    pub oe_extra_delay: bool,
    pub adv_extra_delay: bool,
    pub cs_extra_delay: bool,
    pub time_para_granularity: bool,
}

/*
 * Note that all values in this struct are in nanoseconds except sync_clk
 * (which is in picoseconds), while the register values are in gpmc_fck cycles.
 */
#[repr(C)]
pub struct gpmc_timings {
    /* Minimum clock period for synchronous mode (in picoseconds) */
    pub sync_clk: u32,
    /* Chip-select signal timings corresponding to GPMC_CS_CONFIG2 */
    pub cs_on: u32,
    pub cs_rd_off: u32,
    pub cs_wr_off: u32,
    /* ADV signal timings corresponding to GPMC_CONFIG3 */
    pub adv_on: u32,
    pub adv_rd_off: u32,
    pub adv_wr_off: u32,
    pub adv_aad_mux_on: u32,
    pub adv_aad_mux_rd_off: u32,
    pub adv_aad_mux_wr_off: u32,
    /* WE signals timings corresponding to GPMC_CONFIG4 */
    pub we_on: u32,
    pub we_off: u32,
    /* OE signals timings corresponding to GPMC_CONFIG4 */
    pub oe_on: u32,
    pub oe_off: u32,
    pub oe_aad_mux_on: u32,
    pub oe_aad_mux_off: u32,
    /* Access time and cycle time timings corresponding to GPMC_CONFIG5 */
    pub page_burst_access: u32,
    pub access: u32,
    pub rd_cycle: u32,
    pub wr_cycle: u32,
    pub bus_turnaround: u32,
    pub cycle2cycle_delay: u32,
    pub wait_monitoring: u32,
    pub clk_activation: u32,
    /* The following are only on OMAP3430 */
    pub wr_access: u32,
    pub wr_data_mux_bus: u32,
    pub bool_timings: gpmc_bool_timings,
}

/* Device timings in picoseconds */
#[repr(C)]
pub struct gpmc_device_timings {
    pub t_ceasu: u32,
    pub t_avdasu: u32,
    /* XXX: try to combine t_avdp_r & t_avdp_w. Issue is
     * of tusb using these timings even for sync whilst
     * ideally for adv_rd/(wr)_off it should have considered
     * t_avdh instead. This indirectly necessitates r/w
     * variations of t_avdp as it is possible to have one
     * sync & other async
     */
    pub t_avdp_r: u32,
    pub t_avdp_w: u32,
    pub t_aavdh: u32,
    pub t_oeasu: u32,
    pub t_aa: u32,
    pub t_iaa: u32,
    pub t_oe: u32,
    pub t_ce: u32,
    pub t_rd_cycle: u32,
    pub t_cez_r: u32,
    pub t_cez_w: u32,
    pub t_oez: u32,
    pub t_weasu: u32,
    pub t_wpl: u32,
    pub t_wph: u32,
    pub t_wr_cycle: u32,
    pub clk: u32,
    pub t_bacc: u32,
    pub t_ces: u32,
    pub t_avds: u32,
    pub t_avdh: u32,
    pub t_ach: u32,
    pub t_rdyo: u32,
    pub t_ce_rdyz: u32,
    pub t_ce_avd: u32,
    /* XXX: check the possibility of combining
     * cyc_aavhd_oe & cyc_aavdh_we
     */
    pub cyc_aavdh_oe: u8,
    pub cyc_aavdh_we: u8,
    pub cyc_oe: u8,
    pub cyc_wpl: u8,
    pub cyc_iaa: u32,
    /* extra delays */
    pub ce_xdelay: bool,
    pub avd_xdelay: bool,
    pub oe_xdelay: bool,
    pub we_xdelay: bool,
}

pub const GPMC_BURST_4: u32 = 4;
pub const GPMC_BURST_8: u32 = 8;
pub const GPMC_BURST_16: u32 = 16;
pub const GPMC_DEVWIDTH_8BIT: u32 = 1;
pub const GPMC_DEVWIDTH_16BIT: u32 = 2;
pub const GPMC_MUX_AAD: u32 = 1;
pub const GPMC_MUX_AD: u32 = 2;

/* Wait pin polarity values */
pub const GPMC_WAITPINPOLARITY_INVALID: u32 = u32::MAX;
pub const GPMC_WAITPINPOLARITY_ACTIVE_LOW: u32 = 0;
pub const GPMC_WAITPINPOLARITY_ACTIVE_HIGH: u32 = 1;
pub const GPMC_WAITPIN_INVALID: u32 = u32::MAX;

#[repr(C)]
pub struct gpmc_settings {
    pub burst_wrap: bool,
    pub burst_read: bool,
    pub burst_write: bool,
    pub device_nand: bool,
    pub sync_read: bool,
    pub sync_write: bool,
    pub wait_on_read: bool,
    pub wait_on_write: bool,
    pub burst_len: u32,
    pub device_width: u32,
    pub mux_add_data: u32,
    pub wait_pin: u32,
    pub wait_pin_polarity: u32,
}

/* Data for each chip select */
#[repr(C)]
pub struct gpmc_omap_cs_data {
    pub valid: bool,
    pub is_nand: bool,
    pub settings: *mut gpmc_settings,
    pub device_timings: *mut gpmc_device_timings,
    pub gpmc_timings: *mut gpmc_timings,
    pub pdev: *mut platform_device,
    pub pdata_size: u32,
}

#[repr(C)]
pub struct gpmc_omap_platform_data {
    pub cs: [gpmc_omap_cs_data; GPMC_CS_NUM],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
