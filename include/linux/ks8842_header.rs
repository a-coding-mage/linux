/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ks8842.h KS8842 platform data struct definition
 * Copyright (c) 2010 Intel Corporation
 */

// Dependency equivalent of <linux/if_ether.h>.

/**
 * struct ks8842_platform_data - Platform data of the KS8842 network driver
 * @macaddr: The MAC address of the device, set to all 0:s to use the on in
 *           the chip.
 * @rx_dma_channel: The DMA channel to use for RX, -1 for none.
 * @tx_dma_channel: The DMA channel to use for TX, -1 for none.
 */
#[repr(C)]
pub struct ks8842_platform_data {
    pub macaddr: [u8; ETH_ALEN],
    pub rx_dma_channel: i32,
    pub tx_dma_channel: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
