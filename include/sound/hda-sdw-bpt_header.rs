/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2025 Intel Corporation.
 */

use core::ffi::{c_int, c_uint, c_void};

/* Supplied by the Linux kernel headers. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_ext_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

/* CONFIG_SND_SOF_SOF_HDA_SDW_BPT controls whether these symbols are defined. */
#[cfg(feature = "CONFIG_SND_SOF_SOF_HDA_SDW_BPT")]
unsafe extern "C" {
    pub fn hda_sdw_bpt_open(
        dev: *mut device,
        link_id: c_int,
        bpt_tx_stream: *mut *mut hdac_ext_stream,
        dmab_tx_bdl: *mut snd_dma_buffer,
        bpt_tx_num_bytes: c_uint,
        tx_dma_bandwidth: c_uint,
        bpt_rx_stream: *mut *mut hdac_ext_stream,
        dmab_rx_bdl: *mut snd_dma_buffer,
        bpt_rx_num_bytes: c_uint,
        rx_dma_bandwidth: c_uint,
    ) -> c_int;

    pub fn hda_sdw_bpt_send_async(
        dev: *mut device,
        bpt_tx_stream: *mut hdac_ext_stream,
        bpt_rx_stream: *mut hdac_ext_stream,
    ) -> c_int;

    pub fn hda_sdw_bpt_wait(
        dev: *mut device,
        bpt_tx_stream: *mut hdac_ext_stream,
        bpt_rx_stream: *mut hdac_ext_stream,
    ) -> c_int;

    pub fn hda_sdw_bpt_close(
        dev: *mut device,
        link_id: c_int,
        bpt_tx_stream: *mut hdac_ext_stream,
        dmab_tx_bdl: *mut snd_dma_buffer,
        bpt_rx_stream: *mut hdac_ext_stream,
        dmab_rx_bdl: *mut snd_dma_buffer,
    ) -> c_int;

    pub fn hda_sdw_bpt_get_buf_size_alignment(dma_bandwidth: c_uint) -> c_uint;
}

#[cfg(not(feature = "CONFIG_SND_SOF_SOF_HDA_SDW_BPT"))]
pub unsafe fn hda_sdw_bpt_open(
    _dev: *mut device,
    _link_id: c_int,
    _bpt_tx_stream: *mut *mut hdac_ext_stream,
    _dmab_tx_bdl: *mut snd_dma_buffer,
    _bpt_tx_num_bytes: c_uint,
    _tx_dma_bandwidth: c_uint,
    _bpt_rx_stream: *mut *mut hdac_ext_stream,
    _dmab_rx_bdl: *mut snd_dma_buffer,
    _bpt_rx_num_bytes: c_uint,
    _rx_dma_bandwidth: c_uint,
) -> c_int {
    /* WARN_ONCE(1, "SoundWire BPT is disabled"); */
    -95 // -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_SND_SOF_SOF_HDA_SDW_BPT"))]
pub unsafe fn hda_sdw_bpt_send_async(
    _dev: *mut device,
    _bpt_tx_stream: *mut hdac_ext_stream,
    _bpt_rx_stream: *mut hdac_ext_stream,
) -> c_int {
    /* WARN_ONCE(1, "SoundWire BPT is disabled"); */
    -95 // -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_SND_SOF_SOF_HDA_SDW_BPT"))]
pub unsafe fn hda_sdw_bpt_wait(
    _dev: *mut device,
    _bpt_tx_stream: *mut hdac_ext_stream,
    _bpt_rx_stream: *mut hdac_ext_stream,
) -> c_int {
    /* WARN_ONCE(1, "SoundWire BPT is disabled"); */
    -95 // -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_SND_SOF_SOF_HDA_SDW_BPT"))]
pub unsafe fn hda_sdw_bpt_close(
    _dev: *mut device,
    _link_id: c_int,
    _bpt_tx_stream: *mut hdac_ext_stream,
    _dmab_tx_bdl: *mut snd_dma_buffer,
    _bpt_rx_stream: *mut hdac_ext_stream,
    _dmab_rx_bdl: *mut snd_dma_buffer,
) -> c_int {
    /* WARN_ONCE(1, "SoundWire BPT is disabled"); */
    -95 // -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_SND_SOF_SOF_HDA_SDW_BPT"))]
pub unsafe fn hda_sdw_bpt_get_buf_size_alignment(_dma_bandwidth: c_uint) -> c_uint {
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
