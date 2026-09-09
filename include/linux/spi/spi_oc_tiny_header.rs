/* SPDX-License-Identifier: GPL-2.0 */

/**
 * struct tiny_spi_platform_data - platform data of the OpenCores tiny SPI
 * @freq:\tinput clock freq to the core.
 * @baudwidth:\tbaud rate divider width of the core.
 *
 * freq and baudwidth are used only if the divider is programmable.
 */
#[repr(C)]
pub struct tiny_spi_platform_data {
    pub freq: u32,
    pub baudwidth: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
