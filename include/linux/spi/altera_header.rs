/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header File for Altera SPI Driver.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const ALTERA_SPI_MAX_CS: u32 = 32;

/**
 * struct altera_spi_platform_data - Platform data of the Altera SPI driver
 * @mode_bits:          Mode bits of SPI host.
 * @num_chipselect:    Number of chipselects.
 * @bits_per_word_mask: bitmask of supported bits_per_word for transfers.
 * @num_devices:       Number of devices that shall be added when the driver
 *                      is probed.
 * @devices:           The devices to add.
 */
#[repr(C)]
pub struct altera_spi_platform_data {
    pub mode_bits: u16,
    pub num_chipselect: u16,
    pub bits_per_word_mask: u32,
    pub num_devices: u16,
    pub devices: *mut spi_board_info,
}

#[repr(C)]
pub struct altera_spi {
    pub irq: i32,
    pub len: i32,
    pub count: i32,
    pub bytes_per_word: i32,
    pub imr: u32,

    /* data buffers */
    pub tx: *const u8,
    pub rx: *mut u8,

    pub regmap: *mut regmap,
    pub regoff: u32,
    pub dev: *mut device,
}

unsafe extern "C" {
    pub fn altera_spi_irq(irq: i32, dev: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn altera_spi_init_host(host: *mut spi_controller);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
