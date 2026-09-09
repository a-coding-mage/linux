/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Driver for the Synopsys DesignWare DMA Controller
 *
 * Copyright (C) 2007 Atmel Corporation
 * Copyright (C) 2010-2011 ST Microelectronics
 */

// Dependencies supplied by the surrounding kernel translation.

pub const DW_DMA_MAX_NR_MASTERS: usize = 4;
pub const DW_DMA_MAX_NR_CHANNELS: usize = 8;
pub const DW_DMA_MIN_BURST: u32 = 1;
pub const DW_DMA_MAX_BURST: u32 = 256;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * struct dw_dma_slave - Controller-specific information about a slave
 *
 * @dma_dev:	required DMA master device
 * @src_id:	src request line
 * @dst_id:	dst request line
 * @m_master:	memory master for transfers on allocated channel
 * @p_master:	peripheral master for transfers on allocated channel
 * @channels:	mask of the channels permitted for allocation (zero value means any)
 * @hs_polarity:set active low polarity of handshake interface
 */
#[repr(C)]
pub struct dw_dma_slave {
    pub dma_dev: *mut device,
    pub src_id: u8,
    pub dst_id: u8,
    pub m_master: u8,
    pub p_master: u8,
    pub channels: u8,
    pub hs_polarity: bool,
}

/**
 * struct dw_dma_platform_data - Controller configuration parameters
 * @nr_masters: Number of AHB masters supported by the controller
 * @nr_channels: Number of channels supported by hardware (max 8)
 * @chan_allocation_order: Allocate channels starting from 0 or 7
 * @chan_priority: Set channel priority increasing from 0 to 7 or 7 to 0.
 * @block_size: Maximum block size supported by the controller
 * @data_width: Maximum data width supported by hardware per AHB master
 *		(in bytes, power of 2)
 * @multi_block: Multi block transfers supported by hardware per channel.
 * @max_burst: Maximum value of burst transaction size supported by hardware
 *	       per channel (in units of CTL.SRC_TR_WIDTH/CTL.DST_TR_WIDTH).
 * @protctl: Protection control signals setting per channel.
 * @quirks: Optional platform quirks.
 */
#[repr(C)]
pub struct dw_dma_platform_data {
    pub nr_masters: u32,
    pub nr_channels: u32,
    pub chan_allocation_order: u32,
    pub chan_priority: u32,
    pub block_size: u32,
    pub data_width: [u32; DW_DMA_MAX_NR_MASTERS],
    pub multi_block: [u32; DW_DMA_MAX_NR_CHANNELS],
    pub max_burst: [u32; DW_DMA_MAX_NR_CHANNELS],
    pub protctl: u32,
    pub quirks: u32,
}

pub const CHAN_ALLOCATION_ASCENDING: u32 = 0; // zero to seven
pub const CHAN_ALLOCATION_DESCENDING: u32 = 1; // seven to zero
pub const CHAN_PRIORITY_ASCENDING: u32 = 0; // chan0 highest
pub const CHAN_PRIORITY_DESCENDING: u32 = 1; // chan7 highest
pub const CHAN_PROTCTL_PRIVILEGED: u32 = 1u32 << 0;
pub const CHAN_PROTCTL_BUFFERABLE: u32 = 1u32 << 1;
pub const CHAN_PROTCTL_CACHEABLE: u32 = 1u32 << 2;
pub const CHAN_PROTCTL_MASK: u32 = (1u32 << (2 + 1)) - 1;
pub const DW_DMA_QUIRK_XBAR_PRESENT: u32 = 1u32 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
