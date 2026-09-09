/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Copyright (C) 2023 OpenSynergy GmbH
 * Copyright (C) 2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

/* Sample data on trailing clock edge */
pub const VIRTIO_SPI_CPHA: usize = 1usize << 0;
/* Clock is high when IDLE */
pub const VIRTIO_SPI_CPOL: usize = 1usize << 1;
/* Chip Select is active high */
pub const VIRTIO_SPI_CS_HIGH: usize = 1usize << 2;
/* Transmit LSB first */
pub const VIRTIO_SPI_MODE_LSB_FIRST: usize = 1usize << 3;
/* Loopback mode */
pub const VIRTIO_SPI_MODE_LOOP: usize = 1usize << 4;

#[repr(C)]
pub struct virtio_spi_config {
    pub cs_max_number: u8,
    pub cs_change_supported: u8,
    pub tx_nbits_supported: u8,
    pub rx_nbits_supported: u8,
    pub bits_per_word_mask: u32,
    pub mode_func_supported: u32,
    pub max_freq_hz: u32,
    pub max_word_delay_ns: u32,
    pub max_cs_setup_ns: u32,
    pub max_cs_hold_ns: u32,
    pub max_cs_inactive_ns: u32,
}

pub const VIRTIO_SPI_RX_TX_SUPPORT_DUAL: usize = 1usize << 0;
pub const VIRTIO_SPI_RX_TX_SUPPORT_QUAD: usize = 1usize << 1;
pub const VIRTIO_SPI_RX_TX_SUPPORT_OCTAL: usize = 1usize << 2;

pub const VIRTIO_SPI_MF_SUPPORT_CPHA_0: usize = 1usize << 0;
pub const VIRTIO_SPI_MF_SUPPORT_CPHA_1: usize = 1usize << 1;
pub const VIRTIO_SPI_MF_SUPPORT_CPOL_0: usize = 1usize << 2;
pub const VIRTIO_SPI_MF_SUPPORT_CPOL_1: usize = 1usize << 3;
pub const VIRTIO_SPI_MF_SUPPORT_CS_HIGH: usize = 1usize << 4;
pub const VIRTIO_SPI_MF_SUPPORT_LSB_FIRST: usize = 1usize << 5;
pub const VIRTIO_SPI_MF_SUPPORT_LOOPBACK: usize = 1usize << 6;

#[repr(C)]
pub struct spi_transfer_head {
    pub chip_select_id: u8,
    pub bits_per_word: u8,
    pub cs_change: u8,
    pub tx_nbits: u8,
    pub rx_nbits: u8,
    pub reserved: [u8; 3],
    pub mode: u32,
    pub freq: u32,
    pub word_delay_ns: u32,
    pub cs_setup_ns: u32,
    pub cs_delay_hold_ns: u32,
    pub cs_change_delay_inactive_ns: u32,
}

#[repr(C)]
pub struct spi_transfer_result {
    pub result: u8,
}

pub const VIRTIO_SPI_TRANS_OK: u8 = 0;
pub const VIRTIO_SPI_PARAM_ERR: u8 = 1;
pub const VIRTIO_SPI_TRANS_ERR: u8 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
