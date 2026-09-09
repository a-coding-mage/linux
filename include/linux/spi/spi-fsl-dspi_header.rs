/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Freescale DSPI controller driver
 *
 * Copyright (c) 2017 Angelo Dureghello <angelo@sysam.it>
 */

/**
 * struct fsl_dspi_platform_data - platform data for the Freescale DSPI driver
 * @bus_num: board specific identifier for this DSPI driver.
 * @cs_num: number of chip selects supported by this DSPI driver.
 */
#[repr(C)]
pub struct fsl_dspi_platform_data {
    pub cs_num: u32,
    pub bus_num: u32,
    pub sck_cs_delay: u32,
    pub cs_sck_delay: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
