/* SPDX-License-Identifier: GPL-2.0 */
/*
 * au1550_spi.h - Au1550 PSC SPI controller driver - platform data structure
 */

#[repr(C)]
pub struct au1550_spi_info {
    pub mainclk_hz: u32, // main input clock frequency of PSC
    pub num_chipselect: u16, // number of chipselects supported
    pub activate_cs:
        Option<unsafe extern "C" fn(spi: *mut au1550_spi_info, cs: i32, polarity: i32)>,
    pub deactivate_cs:
        Option<unsafe extern "C" fn(spi: *mut au1550_spi_info, cs: i32, polarity: i32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
