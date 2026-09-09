/* SPDX-License-Identifier: GPL-2.0 */

/*
 * For each bitbanged SPI bus, set up a platform_device node with:
 *   - name "spi_gpio"
 *   - id the same as the SPI bus number it implements
 *   - dev.platform data pointing to a struct spi_gpio_platform_data
 *
 * Use spi_board_info with these busses in the usual way.
 *
 * If the bitbanged bus is later switched to a "native" controller,
 * that platform_device and controller_data should be removed.
 */

/**
 * struct spi_gpio_platform_data - parameter for bitbanged SPI host controller
 * @num_chipselect: how many target devices to allow
 */
#[repr(C)]
pub struct spi_gpio_platform_data {
    pub num_chipselect: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
