/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020, Linaro Limited
 */

/**
 * enum spi_transfer_cmd - spi transfer commands
 * @SPI_TX: SPI peripheral TX command
 * @SPI_RX: SPI peripheral RX command
 * @SPI_DUPLEX: SPI peripheral Duplex command
 */
#[repr(C)]
pub enum spi_transfer_cmd {
    SPI_TX = 1,
    SPI_RX,
    SPI_DUPLEX,
}

/**
 * struct gpi_spi_config - spi config for peripheral
 *
 * @loopback_en: spi loopback enable when set
 * @clock_pol_high: clock polarity
 * @data_pol_high: data polarity
 * @pack_en: process tx/rx buffers as packed
 * @word_len: spi word length
 * @clk_div: source clock divider
 * @clk_src: serial clock
 * @cmd: spi cmd
 * @fragmentation: keep CS asserted at end of sequence
 * @cs: chip select toggle
 * @set_config: set peripheral config
 * @rx_len: receive length for buffer
 */
#[repr(C)]
pub struct gpi_spi_config {
    pub set_config: u8,
    pub loopback_en: u8,
    pub clock_pol_high: u8,
    pub data_pol_high: u8,
    pub pack_en: u8,
    pub word_len: u8,
    pub fragmentation: u8,
    pub cs: u8,
    pub clk_div: u32,
    pub clk_src: u32,
    pub cmd: spi_transfer_cmd,
    pub rx_len: u32,
}

#[repr(C)]
pub enum i2c_op {
    I2C_WRITE = 1,
    I2C_READ,
}

/**
 * struct gpi_i2c_config - i2c config for peripheral
 *
 * @pack_enable: process tx/rx buffers as packed
 * @cycle_count: clock cycles to be sent
 * @high_count: high period of clock
 * @low_count: low period of clock
 * @clk_div: source clock divider
 * @addr: i2c bus address
 * @stretch: stretch the clock at eot
 * @set_config: set peripheral config
 * @rx_len: receive length for buffer
 * @op: i2c cmd
 * @multi_msg: is part of multi i2c r-w msgs
 */
#[repr(C)]
pub struct gpi_i2c_config {
    pub set_config: u8,
    pub pack_enable: u8,
    pub cycle_count: u8,
    pub high_count: u8,
    pub low_count: u8,
    pub addr: u8,
    pub stretch: u8,
    pub clk_div: u16,
    pub rx_len: u32,
    pub op: i2c_op,
    pub multi_msg: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
