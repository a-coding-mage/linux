/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Rust translation of dmaengine_pcm.h.
 * Types supplied by the included kernel/ALSA headers are external dependencies.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[inline]
pub unsafe fn snd_pcm_substream_to_dma_direction(
    substream: *const snd_pcm_substream,
) -> dma_transfer_direction {
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        DMA_MEM_TO_DEV
    } else {
        DMA_DEV_TO_MEM
    }
}

extern "C" {
    pub fn snd_hwparams_to_dma_slave_config(
        substream: *const snd_pcm_substream,
        params: *const snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> c_int;
    pub fn snd_dmaengine_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    pub fn snd_dmaengine_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    pub fn snd_dmaengine_pcm_pointer_no_residue(
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;

    pub fn snd_dmaengine_pcm_open(
        substream: *mut snd_pcm_substream,
        chan: *mut dma_chan,
    ) -> c_int;
    pub fn snd_dmaengine_pcm_close(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_dmaengine_pcm_sync_stop(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_dmaengine_pcm_close_release_chan(substream: *mut snd_pcm_substream) -> c_int;

    pub fn snd_dmaengine_pcm_request_channel(
        filter_fn: dma_filter_fn,
        filter_data: *mut c_void,
    ) -> *mut dma_chan;
    pub fn snd_dmaengine_pcm_get_chan(substream: *mut snd_pcm_substream) -> *mut dma_chan;

    pub fn snd_dmaengine_pcm_set_config_from_dai_data(
        substream: *const snd_pcm_substream,
        dma_data: *const snd_dmaengine_dai_dma_data,
        config: *mut dma_slave_config,
    );
    pub fn snd_dmaengine_pcm_refine_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        dma_data: *mut snd_dmaengine_dai_dma_data,
        hw: *mut snd_pcm_hardware,
        chan: *mut dma_chan,
    ) -> c_int;

    pub fn snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    pub fn snd_dmaengine_pcm_unregister(dev: *mut device);
    pub fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    pub fn snd_dmaengine_pcm_prepare_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> c_int;
}

pub const SND_DMAENGINE_PCM_DAI_FLAG_PACK: c_uint = 1 << 0;
pub const SND_DMAENGINE_PCM_FLAG_COMPAT: c_uint = 1 << 0;
pub const SND_DMAENGINE_PCM_FLAG_NO_DT: c_uint = 1 << 1;
pub const SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX: c_uint = 1 << 3;
pub const SND_DMAENGINE_PCM_DRV_NAME: &[u8] = b"snd_dmaengine_pcm\0";

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: dma_slave_buswidth,
    pub maxburst: u32,
    pub filter_data: *mut c_void,
    pub chan_name: *const c_char,
    pub fifo_size: c_uint,
    pub flags: c_uint,
    pub peripheral_config: *mut c_void,
    pub peripheral_size: usize,
    pub port_window_size: u32,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<unsafe extern "C" fn(
        *mut snd_pcm_substream,
        *mut snd_pcm_hw_params,
        *mut dma_slave_config,
    ) -> c_int>,
    pub compat_request_channel: Option<unsafe extern "C" fn(
        *mut snd_soc_pcm_runtime,
        *mut snd_pcm_substream,
    ) -> *mut dma_chan>,
    pub process: Option<unsafe extern "C" fn(
        *mut snd_pcm_substream,
        c_int,
        c_ulong,
        c_ulong,
    ) -> c_int>,
    pub name: *const c_char,
    pub compat_filter_fn: dma_filter_fn,
    pub dma_dev: *mut device,
    pub chan_names: [*const c_char; (SNDRV_PCM_STREAM_LAST + 1) as usize],
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: c_uint,
}

#[repr(C)]
pub struct dmaengine_pcm {
    pub chan: [*mut dma_chan; (SNDRV_PCM_STREAM_LAST + 1) as usize],
    pub config: *const snd_dmaengine_pcm_config,
    pub flags: c_uint,
}

// External declarations supplied by the included kernel and ALSA headers.
extern "C" {
    pub static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    pub static SNDRV_PCM_STREAM_LAST: c_int;
}

// External types and constants: dma_transfer_direction, DMA_MEM_TO_DEV,
// DMA_DEV_TO_MEM, snd_pcm_substream, snd_pcm_hw_params, dma_slave_config,
// snd_pcm_uframes_t, dma_chan, dma_filter_fn, dma_addr_t, dma_slave_buswidth,
// device, snd_soc_pcm_runtime, and snd_pcm_hardware.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
