/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/amba/pl08x.h - ARM PrimeCell DMA Controller driver
 *
 * Copyright (C) 2005 ARM Ltd
 * Copyright (C) 2010 ST-Ericsson SA
 *
 * pl08x information required by platform code
 *
 * Please credit ARM.com
 * Documentation: ARM DDI 0196D
 */

/* C header guard: AMBA_PL08X_H */
/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::{c_char, c_void};

pub struct pl08x_driver_data;
pub struct pl08x_phy_chan;
pub struct pl08x_txd;
pub struct dma_chan;
pub struct dma_slave_map;

/* Bitmasks for selecting AHB ports for DMA transfers */
pub const PL08X_AHB1: u32 = 1 << 0;
pub const PL08X_AHB2: u32 = 1 << 1;

/**
 * struct pl08x_channel_data - data structure to pass info between
 * platform and PL08x driver regarding channel configuration
 * @bus_id: name of this device channel, not just a device name since
 * devices may have more than one channel e.g. "foo_tx"
 * @min_signal: the minimum DMA signal number to be muxed in for this
 * channel (for platforms supporting muxed signals). If you have
 * static assignments, make sure this is set to the assigned signal
 * number, PL08x have 16 possible signals in number 0 thru 15 so
 * when these are not enough they often get muxed (in hardware)
 * disabling simultaneous use of the same channel for two devices.
 * @max_signal: the maximum DMA signal number to be muxed in for
 * the channel. Set to the same as min_signal for
 * devices with static assignments
 * @muxval: a number usually used to poke into some mux regiser to
 * mux in the signal to this channel
 * @addr: source/target address in physical memory for this DMA channel,
 * can be the address of a FIFO register for burst requests for example.
 * This can be left undefined if the PrimeCell API is used for configuring
 * this.
 * @single: the device connected to this channel will request single DMA
 * transfers, not bursts. (Bursts are default.)
 * @periph_buses: the device connected to this channel is accessible via
 * these buses (use PL08X_AHB1 | PL08X_AHB2).
 */
#[repr(C)]
pub struct pl08x_channel_data {
    pub bus_id: *const c_char,
    pub min_signal: i32,
    pub max_signal: i32,
    pub muxval: u32,
    pub addr: dma_addr_t,
    pub single: bool,
    pub periph_buses: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pl08x_burst_size {
    PL08X_BURST_SZ_1,
    PL08X_BURST_SZ_4,
    PL08X_BURST_SZ_8,
    PL08X_BURST_SZ_16,
    PL08X_BURST_SZ_32,
    PL08X_BURST_SZ_64,
    PL08X_BURST_SZ_128,
    PL08X_BURST_SZ_256,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pl08x_bus_width {
    PL08X_BUS_WIDTH_8_BITS,
    PL08X_BUS_WIDTH_16_BITS,
    PL08X_BUS_WIDTH_32_BITS,
}

/**
 * struct pl08x_platform_data - the platform configuration for the PL08x
 * PrimeCells.
 * @slave_channels: the channels defined for the different devices on the
 * platform, all inclusive, including multiplexed channels. The available
 * physical channels will be multiplexed around these signals as they are
 * requested, just enumerate all possible channels.
 * @num_slave_channels: number of elements in the slave channel array
 * @memcpy_burst_size: the appropriate burst size for memcpy operations
 * @memcpy_bus_width: memory bus width
 * @memcpy_prot_buff: whether memcpy DMA is bufferable
 * @memcpy_prot_cache: whether memcpy DMA is cacheable
 * @get_xfer_signal: request a physical signal to be used for a DMA transfer
 * immediately: if there is some multiplexing or similar blocking the use
 * of the channel the transfer can be denied by returning less than zero,
 * else it returns the allocated signal number
 * @put_xfer_signal: indicate to the platform that this physical signal is not
 * running any DMA transfer and multiplexing can be recycled
 * @lli_buses: buses which LLIs can be fetched from: PL08X_AHB1 | PL08X_AHB2
 * @mem_buses: buses which memory can be accessed from: PL08X_AHB1 | PL08X_AHB2
 * @slave_map: DMA slave matching table
 * @slave_map_len: number of elements in @slave_map
 */
#[repr(C)]
pub struct pl08x_platform_data {
    pub slave_channels: *mut pl08x_channel_data,
    pub num_slave_channels: u32,
    pub memcpy_burst_size: pl08x_burst_size,
    pub memcpy_bus_width: pl08x_bus_width,
    pub memcpy_prot_buff: bool,
    pub memcpy_prot_cache: bool,
    pub get_xfer_signal: Option<unsafe extern "C" fn(*const pl08x_channel_data) -> i32>,
    pub put_xfer_signal: Option<unsafe extern "C" fn(*const pl08x_channel_data, i32)>,
    pub lli_buses: u8,
    pub mem_buses: u8,
    pub slave_map: *const dma_slave_map,
    pub slave_map_len: i32,
}

/* CONFIG_AMBA_PL08X is a build-time condition from the original header. */
#[cfg(feature = "CONFIG_AMBA_PL08X")]
unsafe extern "C" {
    pub fn pl08x_filter_id(chan: *mut dma_chan, chan_id: *mut c_void) -> bool;
}

#[cfg(not(feature = "CONFIG_AMBA_PL08X"))]
#[inline]
pub unsafe fn pl08x_filter_id(_chan: *mut dma_chan, _chan_id: *mut c_void) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
