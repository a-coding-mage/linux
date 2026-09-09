/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: #include <linux/types.h>

#[repr(C)]
pub struct spi_board_info {
    _private: [u8; 0],
}

/**
 * struct xspi_platform_data - Platform data of the Xilinx SPI driver
 * @devices:        Devices to add when the driver is probed.
 * @num_devices:    Number of devices in the devices array.
 * @num_chipselect: Number of chip select by the IP.
 * @bits_per_word:  Number of bits per word.
 * @force_irq:      If set, forces QSPI transaction requirements.
 */
#[repr(C)]
pub struct xspi_platform_data {
    pub devices: *mut spi_board_info,
    pub num_devices: u8,
    pub num_chipselect: u8,
    pub bits_per_word: u8,
    pub force_irq: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
