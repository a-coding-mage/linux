/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2009 Samsung Electronics Ltd.
 *	Jaswinder Singh <jassi.brar@samsung.com>
 */

// C dependency: linux/dmaengine.h

// struct platform_device;

/**
 * struct s3c64xx_spi_csinfo - ChipSelect description
 * @fb_delay: Slave specific feedback delay.
 *            Refer to FB_CLK_SEL register definition in SPI chapter.
 *
 * This is per SPI-Slave Chipselect information.
 * Allocate and initialize one in machine init code and make the
 * spi_board_info.controller_data point to it.
 */
#[repr(C)]
pub struct s3c64xx_spi_csinfo {
    pub fb_delay: u8,
}

/**
 * struct s3c64xx_spi_info - SPI Controller defining structure
 * @src_clk_nr: Clock source index for the CLK_CFG[SPI_CLKSEL] field.
 * @num_cs: Number of CS this controller emulates.
 * @no_cs: Used when CS line is not connected.
 * @polling: Using polling mode when %true (no 'dmas' property in devicetree)
 * @cfg_gpio: Configure pins for this SPI controller.
 */
#[repr(C)]
pub struct s3c64xx_spi_info {
    pub src_clk_nr: core::ffi::c_int,
    pub num_cs: core::ffi::c_int,
    pub no_cs: bool,
    pub polling: bool,
    pub cfg_gpio: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
}

/**
 * s3c64xx_spi0_set_platdata - SPI Controller configure callback by the board
 *				initialization code.
 * @src_clk_nr: Clock the SPI controller is to use to generate SPI clocks.
 * @num_cs: Number of elements in the 'cs' array.
 *
 * Call this from machine init code for each SPI Controller that
 * has some chips attached to it.
 */
unsafe extern "C" {
    pub fn s3c64xx_spi0_set_platdata(
        src_clk_nr: core::ffi::c_int,
        num_cs: core::ffi::c_int,
    );

    /* defined by architecture to configure gpio */
    pub fn s3c64xx_spi0_cfg_gpio() -> core::ffi::c_int;

    pub static mut s3c64xx_spi0_pdata: s3c64xx_spi_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
