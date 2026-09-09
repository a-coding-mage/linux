/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2011
 *
 * Author: Dmitry Tarnyagin <dmitry.tarnyagin@stericsson.com>
 */

#[repr(C)]
pub struct cw1200_platform_data_spi {
    pub spi_bits_per_word: u8, // REQUIRED
    pub ref_clk: u16,          // REQUIRED (in KHz)

    // All others are optional
    pub have_5ghz: bool,
    pub power_ctrl: Option<unsafe extern "C" fn(
        pdata: *const cw1200_platform_data_spi,
        enable: bool,
    ) -> i32>, // Control 3v3 / 1v8 supply
    pub clk_ctrl: Option<unsafe extern "C" fn(
        pdata: *const cw1200_platform_data_spi,
        enable: bool,
    ) -> i32>, // Control CLK32K
    pub macaddr: *const u8, // if NULL, use cw1200_mac_template module parameter
    pub sdd_file: *const core::ffi::c_char, // if NULL, will use default for detected hw type
}

#[repr(C)]
pub struct cw1200_platform_data_sdio {
    pub ref_clk: u16, // REQUIRED (in KHz)

    // All others are optional
    pub have_5ghz: bool,
    pub no_nptb: bool, // SDIO hardware does not support non-power-of-2-blocksizes
    pub irq: i32,      // IRQ line or 0 to use SDIO IRQ
    pub power_ctrl: Option<unsafe extern "C" fn(
        pdata: *const cw1200_platform_data_sdio,
        enable: bool,
    ) -> i32>, // Control 3v3 / 1v8 supply
    pub clk_ctrl: Option<unsafe extern "C" fn(
        pdata: *const cw1200_platform_data_sdio,
        enable: bool,
    ) -> i32>, // Control CLK32K
    pub macaddr: *const u8, // if NULL, use cw1200_mac_template module parameter
    pub sdd_file: *const core::ffi::c_char, // if NULL, will use default for detected hw type
}

/* An example of SPI support in your board setup file:

   static struct cw1200_platform_data_spi cw1200_platform_data = {
       .ref_clk = 38400,
       .spi_bits_per_word = 16,
       .reset = GPIO_RF_RESET,
       .powerup = GPIO_RF_POWERUP,
       .macaddr = wifi_mac_addr,
       .sdd_file = "sdd_sagrad_1091_1098.bin",
  };
  static struct spi_board_info myboard_spi_devices[] __initdata = {
       {
               .modalias = "cw1200_wlan_spi",
               .max_speed_hz = 52000000,
               .bus_num = 0,
               .irq = WIFI_IRQ,
               .platform_data = &cw1200_platform_data,
               .chip_select = 0,
       },
  };

 */

/* An example of SDIO support in your board setup file:

  static struct cw1200_platform_data_sdio my_cw1200_platform_data = {
	.ref_clk = 38400,
	.have_5ghz = false,
	.sdd_file = "sdd_myplatform.bin",
  };
  cw1200_sdio_set_platform_data(&my_cw1200_platform_data);

 */

unsafe extern "C" {
    pub fn cw1200_sdio_set_platform_data(pdata: *mut cw1200_platform_data_sdio);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
