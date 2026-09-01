// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2012, Analog Devices Inc.
 *	Author: Lars-Peter Clausen <lars@metafoo.de>
 *
 *  Based on:
 *	imx-pcm-dma-mx2.c, Copyright 2009 Sascha Hauer <s.hauer@pengutronix.de>
 *	mxs-pcm.c, Copyright (C) 2011 Freescale Semiconductor, Inc.
 *	ep93xx-pcm.c, Copyright (C) 2006 Lennert Buytenhek <buytenh@wantstofly.org>
 *		      Copyright (C) 2006 Applied Data Systems
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

pub type dma_cookie_t = i32;
pub type snd_pcm_uframes_t = libc::c_ulong;
pub type snd_pcm_format_t = i32;
pub type dma_filter_fn = Option<unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool>;
pub type dma_cap_mask_t = [libc::c_ulong; 1];

#[repr(C)]
pub struct dmaengine_pcm_runtime_data {
    pub dma_chan: *mut dma_chan,
    pub cookie: dma_cookie_t,
    pub pos: libc::c_uint,
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: libc::c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub no_period_wakeup: bool,
    pub dma_addr: dma_addr_t,
    pub delay: snd_pcm_sframes_t,
    pub info: libc::c_uint,
}

pub type dma_addr_t = libc::c_ulong;
pub type snd_pcm_sframes_t = libc::c_long;

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_slave_config {
    pub direction: dma_transfer_direction,
    pub src_addr: dma_addr_t,
    pub dst_addr: dma_addr_t,
    pub src_addr_width: dma_slave_buswidth,
    pub dst_addr_width: dma_slave_buswidth,
    pub src_maxburst: libc::c_uint,
    pub dst_maxburst: libc::c_uint,
    pub src_port_window_size: libc::c_uint,
    pub dst_port_window_size: libc::c_uint,
    pub device_fc: bool,
    pub peripheral_config: *mut c_void,
    pub peripheral_size: libc::size_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_slave_buswidth {
    DMA_SLAVE_BUSWIDTH_UNDEFINED = 0,
    DMA_SLAVE_BUSWIDTH_1_BYTE = 1,
    DMA_SLAVE_BUSWIDTH_2_BYTES = 2,
    DMA_SLAVE_BUSWIDTH_3_BYTES = 3,
    DMA_SLAVE_BUSWIDTH_4_BYTES = 4,
    DMA_SLAVE_BUSWIDTH_8_BYTES = 8,
}

pub use dma_slave_buswidth::{
    DMA_SLAVE_BUSWIDTH_1_BYTE, DMA_SLAVE_BUSWIDTH_2_BYTES, DMA_SLAVE_BUSWIDTH_3_BYTES,
    DMA_SLAVE_BUSWIDTH_4_BYTES, DMA_SLAVE_BUSWIDTH_8_BYTES, DMA_SLAVE_BUSWIDTH_UNDEFINED,
};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_transfer_direction {
    DMA_MEM_TO_DEV = 1,
    DMA_DEV_TO_MEM = 2,
}

pub use dma_transfer_direction::{DMA_DEV_TO_MEM, DMA_MEM_TO_DEV};

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: dma_slave_buswidth,
    pub maxburst: libc::c_uint,
    pub port_window_size: libc::c_uint,
    pub flags: libc::c_uint,
    pub peripheral_config: *mut c_void,
    pub peripheral_size: libc::size_t,
}

#[repr(C)]
pub struct dma_async_tx_descriptor {
    pub callback: Option<unsafe extern "C" fn(*mut c_void)>,
    pub callback_param: *mut c_void,
}

#[repr(C)]
pub struct dma_tx_state {
    pub residue: libc::c_uint,
    pub in_flight_bytes: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_status {
    DMA_IN_PROGRESS = 0,
    DMA_PAUSED = 1,
}

pub use dma_status::{DMA_IN_PROGRESS, DMA_PAUSED};

#[repr(C)]
pub struct dma_slave_caps {
    pub cmd_pause: bool,
    pub cmd_resume: bool,
    pub residue_granularity: dma_residue_granularity,
    pub src_addr_widths: u32,
    pub dst_addr_widths: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum dma_residue_granularity {
    DMA_RESIDUE_GRANULARITY_DESCRIPTOR = 0,
    DMA_RESIDUE_GRANULARITY_SEGMENT = 1,
    DMA_RESIDUE_GRANULARITY_BURST = 2,
}

pub use dma_residue_granularity::DMA_RESIDUE_GRANULARITY_SEGMENT;

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: libc::c_uint,
    pub formats: u64,
}

pub const EINVAL: libc::c_int = 22;
pub const ENOMEM: libc::c_int = 12;
pub const ENXIO: libc::c_int = 6;

pub const SNDRV_PCM_STREAM_PLAYBACK: libc::c_int = 0;
pub const SNDRV_PCM_TRIGGER_START: libc::c_int = 0;
pub const SNDRV_PCM_TRIGGER_STOP: libc::c_int = 1;
pub const SNDRV_PCM_TRIGGER_PAUSE_PUSH: libc::c_int = 3;
pub const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: libc::c_int = 4;
pub const SNDRV_PCM_TRIGGER_SUSPEND: libc::c_int = 5;
pub const SNDRV_PCM_TRIGGER_RESUME: libc::c_int = 6;
pub const SNDRV_PCM_INFO_PAUSE: libc::c_uint = 1 << 11;
pub const SNDRV_PCM_INFO_RESUME: libc::c_uint = 1 << 12;
pub const SNDRV_PCM_INFO_BATCH: libc::c_uint = 1 << 16;
pub const SNDRV_PCM_HW_PARAM_PERIODS: libc::c_int = 11;
pub const SND_DMAENGINE_PCM_DAI_FLAG_PACK: libc::c_uint = 1 << 0;
pub const DMA_CTRL_ACK: libc::c_ulong = 1 << 0;
pub const DMA_PREP_INTERRUPT: libc::c_ulong = 1 << 1;
pub const DMA_SLAVE: libc::c_int = 0;
pub const DMA_CYCLIC: libc::c_int = 1;

unsafe extern "C" {
    fn params_physical_width(params: *const snd_pcm_hw_params) -> libc::c_int;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> libc::c_uint;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> libc::c_uint;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_substream_to_dma_direction(
        substream: *mut snd_pcm_substream,
    ) -> dma_transfer_direction;
    fn dmaengine_prep_dma_cyclic(
        chan: *mut dma_chan,
        buf_addr: dma_addr_t,
        buf_len: libc::c_uint,
        period_len: libc::c_uint,
        dir: dma_transfer_direction,
        flags: libc::c_ulong,
    ) -> *mut dma_async_tx_descriptor;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn dma_async_issue_pending(chan: *mut dma_chan);
    fn dmaengine_resume(chan: *mut dma_chan) -> libc::c_int;
    fn dmaengine_pause(chan: *mut dma_chan) -> libc::c_int;
    fn dmaengine_terminate_async(chan: *mut dma_chan) -> libc::c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: libc::c_uint) -> snd_pcm_uframes_t;
    fn dmaengine_tx_status(
        chan: *mut dma_chan,
        cookie: dma_cookie_t,
        state: *mut dma_tx_state,
    ) -> dma_status;
    fn dma_cap_zero(mask: *mut dma_cap_mask_t);
    fn dma_cap_set(tx_type: libc::c_int, mask: *mut dma_cap_mask_t);
    fn dma_request_channel(
        mask: dma_cap_mask_t,
        fn_: dma_filter_fn,
        fn_param: *mut c_void,
    ) -> *mut dma_chan;
    fn snd_pcm_hw_constraint_integer(
        runtime: *mut snd_pcm_runtime,
        var: libc::c_int,
    ) -> libc::c_int;
    fn kzalloc(size: libc::size_t, flags: libc::c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn dmaengine_synchronize(chan: *mut dma_chan);
    fn dma_release_channel(chan: *mut dma_chan);
    fn dma_get_slave_caps(chan: *mut dma_chan, caps: *mut dma_slave_caps) -> libc::c_int;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> libc::c_int;
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> u64;
}

#[inline]
const fn BIT(nr: libc::c_int) -> u32 {
    1u32 << nr
}

#[inline]
unsafe fn substream_to_prtd(
    substream: *const snd_pcm_substream,
) -> *mut dmaengine_pcm_runtime_data {
    unsafe { (*(*substream).runtime).private_data as *mut dmaengine_pcm_runtime_data }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_get_chan(
    substream: *mut snd_pcm_substream,
) -> *mut dma_chan {
    let prtd = unsafe { substream_to_prtd(substream) };

    unsafe { (*prtd).dma_chan }
}

/**
 * snd_hwparams_to_dma_slave_config - Convert hw_params to dma_slave_config
 * @substream: PCM substream
 * @params: hw_params
 * @slave_config: DMA slave config
 *
 * This function can be used to initialize a dma_slave_config from a substream
 * and hw_params in a dmaengine based PCM driver implementation.
 *
 * Return: zero if successful, or a negative error code
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hwparams_to_dma_slave_config(
    substream: *const snd_pcm_substream,
    params: *const snd_pcm_hw_params,
    slave_config: *mut dma_slave_config,
) -> libc::c_int {
    let buswidth: dma_slave_buswidth;
    let bits: libc::c_int;

    bits = unsafe { params_physical_width(params) };
    if bits < 8 || bits > 64 {
        return -EINVAL;
    } else if bits == 8 {
        buswidth = DMA_SLAVE_BUSWIDTH_1_BYTE;
    } else if bits == 16 {
        buswidth = DMA_SLAVE_BUSWIDTH_2_BYTES;
    } else if bits == 24 {
        buswidth = DMA_SLAVE_BUSWIDTH_3_BYTES;
    } else if bits <= 32 {
        buswidth = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else {
        buswidth = DMA_SLAVE_BUSWIDTH_8_BYTES;
    }

    if unsafe { (*substream).stream } == SNDRV_PCM_STREAM_PLAYBACK {
        unsafe {
            (*slave_config).direction = DMA_MEM_TO_DEV;
            (*slave_config).dst_addr_width = buswidth;
        }
    } else {
        unsafe {
            (*slave_config).direction = DMA_DEV_TO_MEM;
            (*slave_config).src_addr_width = buswidth;
        }
    }

    unsafe {
        (*slave_config).device_fc = false;
    }

    0
}

/**
 * snd_dmaengine_pcm_set_config_from_dai_data() - Initializes a dma slave config
 *  using DAI DMA data.
 * @substream: PCM substream
 * @dma_data: DAI DMA data
 * @slave_config: DMA slave configuration
 *
 * Initializes the {dst,src}_addr, {dst,src}_maxburst, {dst,src}_addr_width
 * fields of the DMA slave config from the same fields of the DAI DMA
 * data struct. The src and dst fields will be initialized depending on the
 * direction of the substream. If the substream is a playback stream the dst
 * fields will be initialized, if it is a capture stream the src fields will be
 * initialized. The {dst,src}_addr_width field will only be initialized if the
 * SND_DMAENGINE_PCM_DAI_FLAG_PACK flag is set or if the addr_width field of
 * the DAI DMA data struct is not equal to DMA_SLAVE_BUSWIDTH_UNDEFINED. If
 * both conditions are met the latter takes priority.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_set_config_from_dai_data(
    substream: *const snd_pcm_substream,
    dma_data: *const snd_dmaengine_dai_dma_data,
    slave_config: *mut dma_slave_config,
) {
    if unsafe { (*substream).stream } == SNDRV_PCM_STREAM_PLAYBACK {
        unsafe {
            (*slave_config).dst_addr = (*dma_data).addr;
            (*slave_config).dst_maxburst = (*dma_data).maxburst;
            (*slave_config).dst_port_window_size = (*dma_data).port_window_size;
        }
        if unsafe { (*dma_data).flags & SND_DMAENGINE_PCM_DAI_FLAG_PACK } != 0 {
            unsafe {
                (*slave_config).dst_addr_width = DMA_SLAVE_BUSWIDTH_UNDEFINED;
            }
        }
        if unsafe { (*dma_data).addr_width } != DMA_SLAVE_BUSWIDTH_UNDEFINED {
            unsafe {
                (*slave_config).dst_addr_width = (*dma_data).addr_width;
            }
        }
    } else {
        unsafe {
            (*slave_config).src_addr = (*dma_data).addr;
            (*slave_config).src_maxburst = (*dma_data).maxburst;
            (*slave_config).src_port_window_size = (*dma_data).port_window_size;
        }
        if unsafe { (*dma_data).flags & SND_DMAENGINE_PCM_DAI_FLAG_PACK } != 0 {
            unsafe {
                (*slave_config).src_addr_width = DMA_SLAVE_BUSWIDTH_UNDEFINED;
            }
        }
        if unsafe { (*dma_data).addr_width } != DMA_SLAVE_BUSWIDTH_UNDEFINED {
            unsafe {
                (*slave_config).src_addr_width = (*dma_data).addr_width;
            }
        }
    }

    unsafe {
        (*slave_config).peripheral_config = (*dma_data).peripheral_config;
        (*slave_config).peripheral_size = (*dma_data).peripheral_size;
    }
}

unsafe extern "C" fn dmaengine_pcm_dma_complete(arg: *mut c_void) {
    let new_pos: libc::c_uint;
    let substream = arg as *mut snd_pcm_substream;
    let prtd = unsafe { substream_to_prtd(substream) };

    let mut tmp_new_pos = unsafe { (*prtd).pos.wrapping_add(snd_pcm_lib_period_bytes(substream)) };
    if tmp_new_pos >= unsafe { snd_pcm_lib_buffer_bytes(substream) } {
        tmp_new_pos = 0;
    }
    new_pos = tmp_new_pos;
    unsafe {
        (*prtd).pos = new_pos;

        snd_pcm_period_elapsed(substream);
    }
}

unsafe fn dmaengine_pcm_prepare_and_submit(
    substream: *mut snd_pcm_substream,
) -> libc::c_int {
    let prtd = unsafe { substream_to_prtd(substream) };
    let chan = unsafe { (*prtd).dma_chan };
    let desc: *mut dma_async_tx_descriptor;
    let direction: dma_transfer_direction;
    let mut flags: libc::c_ulong = DMA_CTRL_ACK;

    direction = unsafe { snd_pcm_substream_to_dma_direction(substream) };

    if !unsafe { (*(*substream).runtime).no_period_wakeup } {
        flags |= DMA_PREP_INTERRUPT;
    }

    unsafe {
        (*prtd).pos = 0;
    }
    desc = unsafe {
        dmaengine_prep_dma_cyclic(
            chan,
            (*(*substream).runtime).dma_addr,
            snd_pcm_lib_buffer_bytes(substream),
            snd_pcm_lib_period_bytes(substream),
            direction,
            flags,
        )
    };

    if desc.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*desc).callback = Some(dmaengine_pcm_dma_complete);
        (*desc).callback_param = substream as *mut c_void;
        (*prtd).cookie = dmaengine_submit(desc);
    }

    0
}

/**
 * snd_dmaengine_pcm_trigger - dmaengine based PCM trigger implementation
 * @substream: PCM substream
 * @cmd: Trigger command
 *
 * This function can be used as the PCM trigger callback for dmaengine based PCM
 * driver implementations.
 *
 * Return: 0 on success, a negative error code otherwise
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: libc::c_int,
) -> libc::c_int {
    let prtd = unsafe { substream_to_prtd(substream) };
    let runtime = unsafe { (*substream).runtime };
    let ret: libc::c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            ret = unsafe { dmaengine_pcm_prepare_and_submit(substream) };
            if ret != 0 {
                return ret;
            }
            unsafe {
                dma_async_issue_pending((*prtd).dma_chan);
            }
        }
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => unsafe {
            dmaengine_resume((*prtd).dma_chan);
        },
        SNDRV_PCM_TRIGGER_SUSPEND => {
            if unsafe { (*runtime).info & SNDRV_PCM_INFO_PAUSE } != 0 {
                unsafe {
                    dmaengine_pause((*prtd).dma_chan);
                }
            } else {
                unsafe {
                    dmaengine_terminate_async((*prtd).dma_chan);
                }
            }
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => unsafe {
            dmaengine_pause((*prtd).dma_chan);
        },
        SNDRV_PCM_TRIGGER_STOP => unsafe {
            dmaengine_terminate_async((*prtd).dma_chan);
        },
        _ => return -EINVAL,
    }

    0
}

/**
 * snd_dmaengine_pcm_pointer_no_residue - dmaengine based PCM pointer implementation
 * @substream: PCM substream
 *
 * This function is deprecated and should not be used by new drivers, as its
 * results may be unreliable.
 *
 * Return: PCM position in frames
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_pointer_no_residue(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let prtd = unsafe { substream_to_prtd(substream) };
    unsafe { bytes_to_frames((*substream).runtime, (*prtd).pos) }
}

/**
 * snd_dmaengine_pcm_pointer - dmaengine based PCM pointer implementation
 * @substream: PCM substream
 *
 * This function can be used as the PCM pointer callback for dmaengine based PCM
 * driver implementations.
 *
 * Return: PCM position in frames
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let prtd = unsafe { substream_to_prtd(substream) };
    let runtime = unsafe { (*substream).runtime };
    let mut state = dma_tx_state {
        residue: 0,
        in_flight_bytes: 0,
    };
    let status: dma_status;
    let buf_size: libc::c_uint;
    let mut pos: libc::c_uint = 0;

    status = unsafe { dmaengine_tx_status((*prtd).dma_chan, (*prtd).cookie, &mut state) };
    if status == DMA_IN_PROGRESS || status == DMA_PAUSED {
        buf_size = unsafe { snd_pcm_lib_buffer_bytes(substream) };
        if state.residue > 0 && state.residue <= buf_size {
            pos = buf_size.wrapping_sub(state.residue);
        }

        unsafe {
            (*runtime).delay = bytes_to_frames(runtime, state.in_flight_bytes) as snd_pcm_sframes_t;
        }
    }

    unsafe { bytes_to_frames(runtime, pos) }
}

/**
 * snd_dmaengine_pcm_request_channel - Request channel for the dmaengine PCM
 * @filter_fn: Filter function used to request the DMA channel
 * @filter_data: Data passed to the DMA filter function
 *
 * This function request a DMA channel for usage with dmaengine PCM.
 *
 * Return: NULL or the requested DMA channel
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_request_channel(
    filter_fn: dma_filter_fn,
    filter_data: *mut c_void,
) -> *mut dma_chan {
    let mut mask: dma_cap_mask_t = [0; 1];

    unsafe {
        dma_cap_zero(&mut mask);
        dma_cap_set(DMA_SLAVE, &mut mask);
        dma_cap_set(DMA_CYCLIC, &mut mask);

        dma_request_channel(mask, filter_fn, filter_data)
    }
}

/**
 * snd_dmaengine_pcm_open - Open a dmaengine based PCM substream
 * @substream: PCM substream
 * @chan: DMA channel to use for data transfers
 *
 * The function should usually be called from the pcm open callback. Note that
 * this function will use private_data field of the substream's runtime. So it
 * is not available to your pcm driver implementation.
 *
 * Return: 0 on success, a negative error code otherwise
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_open(
    substream: *mut snd_pcm_substream,
    chan: *mut dma_chan,
) -> libc::c_int {
    let prtd: *mut dmaengine_pcm_runtime_data;
    let ret: libc::c_int;

    if chan.is_null() {
        return -ENXIO;
    }

    ret = unsafe { snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS) };
    if ret < 0 {
        return ret;
    }

    prtd = unsafe {
        kzalloc(
            core::mem::size_of::<dmaengine_pcm_runtime_data>(),
            0,
        ) as *mut dmaengine_pcm_runtime_data
    };
    if prtd.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*prtd).dma_chan = chan;

        (*(*substream).runtime).private_data = prtd as *mut c_void;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_sync_stop(
    substream: *mut snd_pcm_substream,
) -> libc::c_int {
    let prtd = unsafe { substream_to_prtd(substream) };
    let mut state = dma_tx_state {
        residue: 0,
        in_flight_bytes: 0,
    };
    let status: dma_status;

    status = unsafe { dmaengine_tx_status((*prtd).dma_chan, (*prtd).cookie, &mut state) };
    if status != DMA_PAUSED {
        unsafe {
            dmaengine_synchronize((*prtd).dma_chan);
        }
    }

    0
}

unsafe fn __snd_dmaengine_pcm_close(
    substream: *mut snd_pcm_substream,
    release_channel: bool,
) {
    let prtd = unsafe { substream_to_prtd(substream) };
    let mut state = dma_tx_state {
        residue: 0,
        in_flight_bytes: 0,
    };
    let status: dma_status;

    status = unsafe { dmaengine_tx_status((*prtd).dma_chan, (*prtd).cookie, &mut state) };
    if status == DMA_PAUSED {
        unsafe {
            dmaengine_terminate_async((*prtd).dma_chan);
        }
    }

    unsafe {
        dmaengine_synchronize((*prtd).dma_chan);
    }
    if release_channel {
        unsafe {
            dma_release_channel((*prtd).dma_chan);
        }
    }
    unsafe {
        kfree(prtd as *mut c_void);
    }
}

/**
 * snd_dmaengine_pcm_close - Close a dmaengine based PCM substream
 * @substream: PCM substream
 *
 * Return: 0 on success, a negative error code otherwise
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_close(
    substream: *mut snd_pcm_substream,
) -> libc::c_int {
    unsafe {
        __snd_dmaengine_pcm_close(substream, false);
    }
    0
}

/**
 * snd_dmaengine_pcm_close_release_chan - Close a dmaengine based PCM
 *					  substream and release channel
 * @substream: PCM substream
 *
 * Releases the DMA channel associated with the PCM substream.
 *
 * Return: zero if successful, or a negative error code
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_close_release_chan(
    substream: *mut snd_pcm_substream,
) -> libc::c_int {
    unsafe {
        __snd_dmaengine_pcm_close(substream, true);
    }
    0
}

/**
 * snd_dmaengine_pcm_refine_runtime_hwparams - Refine runtime hw params
 * @substream: PCM substream
 * @dma_data: DAI DMA data
 * @hw: PCM hw params
 * @chan: DMA channel to use for data transfers
 *
 * This function will query DMA capability, then refine the pcm hardware
 * parameters.
 *
 * Return: 0 on success, a negative error code otherwise
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dmaengine_pcm_refine_runtime_hwparams(
    substream: *mut snd_pcm_substream,
    dma_data: *mut snd_dmaengine_dai_dma_data,
    hw: *mut snd_pcm_hardware,
    chan: *mut dma_chan,
) -> libc::c_int {
    let mut dma_caps = dma_slave_caps {
        cmd_pause: false,
        cmd_resume: false,
        residue_granularity: dma_residue_granularity::DMA_RESIDUE_GRANULARITY_DESCRIPTOR,
        src_addr_widths: 0,
        dst_addr_widths: 0,
    };
    let mut addr_widths: u32 = BIT(DMA_SLAVE_BUSWIDTH_1_BYTE as libc::c_int)
        | BIT(DMA_SLAVE_BUSWIDTH_2_BYTES as libc::c_int)
        | BIT(DMA_SLAVE_BUSWIDTH_4_BYTES as libc::c_int);
    let mut i: snd_pcm_format_t;
    let ret: libc::c_int;

    if hw.is_null() || chan.is_null() || dma_data.is_null() {
        return -EINVAL;
    }

    ret = unsafe { dma_get_slave_caps(chan, &mut dma_caps) };
    if ret == 0 {
        if dma_caps.cmd_pause && dma_caps.cmd_resume {
            unsafe {
                (*hw).info |= SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME;
            }
        }
        if dma_caps.residue_granularity <= DMA_RESIDUE_GRANULARITY_SEGMENT {
            unsafe {
                (*hw).info |= SNDRV_PCM_INFO_BATCH;
            }
        }

        if unsafe { (*substream).stream } == SNDRV_PCM_STREAM_PLAYBACK {
            addr_widths = dma_caps.dst_addr_widths;
        } else {
            addr_widths = dma_caps.src_addr_widths;
        }
    }

    /*
     * If SND_DMAENGINE_PCM_DAI_FLAG_PACK is set keep
     * hw.formats set to 0, meaning no restrictions are in place.
     * In this case it's the responsibility of the DAI driver to
     * provide the supported format information.
     */
    if unsafe { (*dma_data).flags & SND_DMAENGINE_PCM_DAI_FLAG_PACK } == 0 {
        /*
         * Prepare formats mask for valid/allowed sample types. If the
         * dma does not have support for the given physical word size,
         * it needs to be masked out so user space can not use the
         * format which produces corrupted audio.
         * In case the dma driver does not implement the slave_caps the
         * default assumption is that it supports 1, 2 and 4 bytes
         * widths.
         */
        i = 0;
        while i <= 63 {
            let bits = unsafe { snd_pcm_format_physical_width(i) };

            /*
             * Enable only samples with DMA supported physical
             * widths
             */
            match bits {
                8 | 16 | 24 | 32 | 64 => {
                    if (addr_widths & (1u32 << (bits / 8))) != 0 {
                        unsafe {
                            (*hw).formats |= pcm_format_to_bits(i);
                        }
                    }
                }
                _ => {
                    /* Unsupported types */
                }
            }

            i += 1;
        }
    }

    ret
}

/* MODULE_DESCRIPTION("PCM dmaengine helper APIs"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
