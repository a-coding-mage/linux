/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/amba/pl022.h
 *
 * Copyright (C) 2008-2009 ST-Ericsson AB
 * Copyright (C) 2006 STMicroelectronics Pvt. Ltd.
 *
 * Author: Linus Walleij <linus.walleij@stericsson.com>
 *
 * Initial version inspired by:
 *	linux-2.6.17-rc3-mm1/drivers/spi/pxa2xx_spi.c
 * Initial adoption to PL022 by:
 *      Sachin Verma <sachin.verma@st.com>
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_loopback {
    LOOPBACK_DISABLED,
    LOOPBACK_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_interface {
    SSP_INTERFACE_MOTOROLA_SPI,
    SSP_INTERFACE_TI_SYNC_SERIAL,
    SSP_INTERFACE_NATIONAL_MICROWIRE,
    SSP_INTERFACE_UNIDIRECTIONAL,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_hierarchy {
    SSP_MASTER,
    SSP_SLAVE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ssp_clock_params {
    pub cpsdvsr: u8,
    pub scr: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_rx_endian {
    SSP_RX_MSB,
    SSP_RX_LSB,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_tx_endian {
    SSP_TX_MSB,
    SSP_TX_LSB,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_data_size {
    SSP_DATA_BITS_4 = 0x03,
    SSP_DATA_BITS_5,
    SSP_DATA_BITS_6,
    SSP_DATA_BITS_7,
    SSP_DATA_BITS_8,
    SSP_DATA_BITS_9,
    SSP_DATA_BITS_10,
    SSP_DATA_BITS_11,
    SSP_DATA_BITS_12,
    SSP_DATA_BITS_13,
    SSP_DATA_BITS_14,
    SSP_DATA_BITS_15,
    SSP_DATA_BITS_16,
    SSP_DATA_BITS_17,
    SSP_DATA_BITS_18,
    SSP_DATA_BITS_19,
    SSP_DATA_BITS_20,
    SSP_DATA_BITS_21,
    SSP_DATA_BITS_22,
    SSP_DATA_BITS_23,
    SSP_DATA_BITS_24,
    SSP_DATA_BITS_25,
    SSP_DATA_BITS_26,
    SSP_DATA_BITS_27,
    SSP_DATA_BITS_28,
    SSP_DATA_BITS_29,
    SSP_DATA_BITS_30,
    SSP_DATA_BITS_31,
    SSP_DATA_BITS_32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_mode {
    INTERRUPT_TRANSFER,
    POLLING_TRANSFER,
    DMA_TRANSFER,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_rx_level_trig {
    SSP_RX_1_OR_MORE_ELEM,
    SSP_RX_4_OR_MORE_ELEM,
    SSP_RX_8_OR_MORE_ELEM,
    SSP_RX_16_OR_MORE_ELEM,
    SSP_RX_32_OR_MORE_ELEM,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_tx_level_trig {
    SSP_TX_1_OR_MORE_EMPTY_LOC,
    SSP_TX_4_OR_MORE_EMPTY_LOC,
    SSP_TX_8_OR_MORE_EMPTY_LOC,
    SSP_TX_16_OR_MORE_EMPTY_LOC,
    SSP_TX_32_OR_MORE_EMPTY_LOC,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_spi_clk_phase {
    SSP_CLK_FIRST_EDGE,
    SSP_CLK_SECOND_EDGE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_spi_clk_pol {
    SSP_CLK_POL_IDLE_LOW,
    SSP_CLK_POL_IDLE_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_microwire_ctrl_len {
    SSP_BITS_4 = 0x03,
    SSP_BITS_5,
    SSP_BITS_6,
    SSP_BITS_7,
    SSP_BITS_8,
    SSP_BITS_9,
    SSP_BITS_10,
    SSP_BITS_11,
    SSP_BITS_12,
    SSP_BITS_13,
    SSP_BITS_14,
    SSP_BITS_15,
    SSP_BITS_16,
    SSP_BITS_17,
    SSP_BITS_18,
    SSP_BITS_19,
    SSP_BITS_20,
    SSP_BITS_21,
    SSP_BITS_22,
    SSP_BITS_23,
    SSP_BITS_24,
    SSP_BITS_25,
    SSP_BITS_26,
    SSP_BITS_27,
    SSP_BITS_28,
    SSP_BITS_29,
    SSP_BITS_30,
    SSP_BITS_31,
    SSP_BITS_32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_microwire_wait_state {
    SSP_MWIRE_WAIT_ZERO,
    SSP_MWIRE_WAIT_ONE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_duplex {
    SSP_MICROWIRE_CHANNEL_FULL_DUPLEX,
    SSP_MICROWIRE_CHANNEL_HALF_DUPLEX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_clkdelay {
    SSP_FEEDBACK_CLK_DELAY_NONE,
    SSP_FEEDBACK_CLK_DELAY_1T,
    SSP_FEEDBACK_CLK_DELAY_2T,
    SSP_FEEDBACK_CLK_DELAY_3T,
    SSP_FEEDBACK_CLK_DELAY_4T,
    SSP_FEEDBACK_CLK_DELAY_5T,
    SSP_FEEDBACK_CLK_DELAY_6T,
    SSP_FEEDBACK_CLK_DELAY_7T,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_chip_select {
    SSP_CHIP_SELECT,
    SSP_CHIP_DESELECT,
}

pub struct dma_chan;

#[repr(C)]
pub struct pl022_ssp_controller {
    pub bus_id: u16,
    // C bit-field: u8 enable_dma:1.
    pub enable_dma: u8,
    pub dma_filter: dma_filter_fn,
    pub dma_rx_param: *mut core::ffi::c_void,
    pub dma_tx_param: *mut core::ffi::c_void,
    pub autosuspend_delay: i32,
    pub rt: bool,
}

#[repr(C)]
pub struct pl022_config_chip {
    pub iface: ssp_interface,
    pub hierarchy: ssp_hierarchy,
    pub slave_tx_disable: bool,
    pub clk_freq: ssp_clock_params,
    pub com_mode: ssp_mode,
    pub rx_lev_trig: ssp_rx_level_trig,
    pub tx_lev_trig: ssp_tx_level_trig,
    pub ctrl_len: ssp_microwire_ctrl_len,
    pub wait_state: ssp_microwire_wait_state,
    pub duplex: ssp_duplex,
    pub clkdelay: ssp_clkdelay,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
