/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  OMAP GPMC (General Purpose Memory Controller) defines
 */

/* Dependency: <linux/platform_data/gpmc-omap.h> */

pub const GPMC_CONFIG_WP: u32 = 0x0000_0005;

/* IRQ numbers in GPMC IRQ domain for legacy boot use */
pub const GPMC_IRQ_FIFOEVENTENABLE: i32 = 0;
pub const GPMC_IRQ_COUNT_EVENT: i32 = 1;

/**
 * gpmc_nand_ops - Interface between NAND and GPMC
 * @nand_write_buffer_empty: get the NAND write buffer empty status.
 */
#[repr(C)]
pub struct gpmc_nand_ops {
    pub nand_writebuffer_empty: Option<unsafe extern "C" fn() -> bool>,
}

#[repr(C)]
pub struct gpmc_nand_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpmc_onenand_info {
    pub sync_read: bool,
    pub sync_write: bool,
    pub burst_len: i32,
}

/* CONFIG_OMAP_GPMC is a build-time condition from the C environment. */
#[cfg(feature = "CONFIG_OMAP_GPMC")]
extern "C" {
    pub fn gpmc_omap_get_nand_ops(regs: *mut gpmc_nand_regs, cs: i32) -> *mut gpmc_nand_ops;

    /**
     * gpmc_omap_onenand_set_timings - set optimized sync timings.
     * @cs:      Chip Select Region
     * @freq:    Chip frequency
     * @latency: Burst latency cycle count
     * @info:    Structure describing parameters used
     *
     * Sets optimized timings for the @cs region based on @freq and @latency.
     * Updates the @info structure based on the GPMC settings.
     */
    pub fn gpmc_omap_onenand_set_timings(
        dev: *mut device,
        cs: i32,
        freq: i32,
        latency: i32,
        info: *mut gpmc_onenand_info,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_OMAP_GPMC"))]
pub unsafe fn gpmc_omap_get_nand_ops(
    _regs: *mut gpmc_nand_regs,
    _cs: i32,
) -> *mut gpmc_nand_ops {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_OMAP_GPMC"))]
pub unsafe fn gpmc_omap_onenand_set_timings(
    _dev: *mut device,
    _cs: i32,
    _freq: i32,
    _latency: i32,
    _info: *mut gpmc_onenand_info,
) -> i32 {
    -EINVAL
}

pub enum device {}
pub enum device_node {}
pub enum gpmc_timings {}
pub enum gpmc_settings {}
pub enum gpmc_device_timings {}
pub enum omap_nand_platform_data {}
pub enum omap_onenand_platform_data {}

extern "C" {
    pub fn gpmc_calc_timings(
        gpmc_t: *mut gpmc_timings,
        gpmc_s: *mut gpmc_settings,
        dev_t: *mut gpmc_device_timings,
    ) -> i32;

    pub fn gpmc_cs_write_reg(cs: i32, idx: i32, val: u32);
    pub fn gpmc_calc_divider(sync_clk: u32) -> i32;
    pub fn gpmc_cs_set_timings(
        cs: i32,
        t: *const gpmc_timings,
        s: *const gpmc_settings,
    ) -> i32;
    pub fn gpmc_cs_program_settings(cs: i32, p: *mut gpmc_settings) -> i32;
    pub fn gpmc_cs_request(cs: i32, size: usize, base: *mut usize) -> i32;
    pub fn gpmc_cs_free(cs: i32);
    pub fn gpmc_configure(cmd: i32, wval: i32) -> i32;
    pub fn gpmc_read_settings_dt(np: *mut device_node, p: *mut gpmc_settings);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
