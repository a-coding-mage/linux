/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Board-specific MMC configuration
 */

// C dependencies: linux/types.h, linux/mmc/host.h

#[repr(C)]
pub struct davinci_mmc_config {
    /* get_cd()/get_wp() may sleep */
    pub get_cd: Option<unsafe extern "C" fn(module: i32) -> i32>,
    pub get_ro: Option<unsafe extern "C" fn(module: i32) -> i32>,

    pub set_power: Option<unsafe extern "C" fn(module: i32, on: bool)>,

    /* wires == 0 is equivalent to wires == 4 (4-bit parallel) */
    pub wires: u8,

    pub max_freq: u32,

    /* any additional host capabilities: OR'd in to mmc->f_caps */
    pub caps: u32,

    /* Number of sg segments */
    pub nr_sg: u8,
}

extern "C" {
    pub fn davinci_setup_mmc(module: i32, config: *mut davinci_mmc_config);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum davinci_mmc_controller_version {
    MMC_CTLR_VERSION_1 = 0, /* DM644x and DM355 */
    MMC_CTLR_VERSION_2,     /* DA830 */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
