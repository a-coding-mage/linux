// SPDX-License-Identifier: GPL-2.0 OR MIT

/*
 * Xen para-virtual sound device
 *
 * Copyright (C) 2016-2018 EPAM Systems Inc.
 *
 * Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
 */

// External dependencies from linux kernel and ALSA subsystem:
// - linux/platform_device.h
// - sound/core.h
// - sound/pcm.h
// - sound/pcm_params.h
// - xen/xenbus.h
// - xen/xen-front-pgdir-shbuf.h
// Local dependencies:
// - xen_snd_front.h
// - xen_snd_front_alsa.h
// - xen_snd_front_cfg.h
// - xen_snd_front_evtchnl.h

use std::sync::atomic::{AtomicU32, Ordering};
use std::ptr;

// Type definitions from external headers
type Snd_pcm_uframes_t = u64;
type Snd_pcm_format_t = u32;

#[repr(C)]
pub struct xen_snd_front_pcm_stream_info {
    pub front_info: *mut xen_snd_front_info,
    pub evt_pair: *mut xen_snd_front_evtchnl_pair,

    // This is the shared buffer with its backing storage.
    pub shbuf: xen_front_pgdir_shbuf,
    pub buffer: *mut u8,
    pub buffer_sz: usize,
    pub num_pages: i32,
    pub pages: *mut *mut core::ffi::c_void, // struct page **

    pub index: i32,

    pub is_open: bool,
    pub pcm_hw: snd_pcm_hardware,

    // Number of processed frames as reported by the backend.
    pub be_cur_frame: Snd_pcm_uframes_t,
    // Current HW pointer to be reported via .period callback.
    pub hw_ptr: AtomicU32,
    // Modulo of the number of processed frames - for period detection.
    pub out_frames: u32,
}

#[repr(C)]
pub struct xen_snd_front_pcm_instance_info {
    pub card_info: *mut xen_snd_front_card_info,
    pub pcm: *mut snd_pcm,
    pub pcm_hw: snd_pcm_hardware,
    pub num_pcm_streams_pb: i32,
    pub streams_pb: *mut xen_snd_front_pcm_stream_info,
    pub num_pcm_streams_cap: i32,
    pub streams_cap: *mut xen_snd_front_pcm_stream_info,
}

#[repr(C)]
pub struct xen_snd_front_card_info {
    pub front_info: *mut xen_snd_front_info,
    pub card: *mut snd_card,
    pub pcm_hw: snd_pcm_hardware,
    pub num_pcm_instances: i32,
    pub pcm_instances: *mut xen_snd_front_pcm_instance_info,
}

#[repr(C)]
struct alsa_sndif_sample_format {
    sndif: u8,
    alsa: Snd_pcm_format_t,
}

// Placeholder types for external dependencies
#[repr(C)]
pub struct xen_snd_front_info;

#[repr(C)]
pub struct xen_snd_front_evtchnl_pair {
    pub req: xen_snd_front_evtchnl,
    pub evt: xen_snd_front_evtchnl,
}

#[repr(C)]
pub struct xen_snd_front_evtchnl;

#[repr(C)]
pub struct xen_front_pgdir_shbuf;

#[repr(C)]
pub struct snd_pcm_hardware;

#[repr(C)]
pub struct snd_pcm;

#[repr(C)]
pub struct snd_card;

#[repr(C)]
pub struct snd_pcm_substream;

#[repr(C)]
pub struct snd_pcm_runtime;

#[repr(C)]
pub struct snd_pcm_hw_params;

#[repr(C)]
pub struct snd_pcm_hw_rule;

#[repr(C)]
pub struct snd_mask;

#[repr(C)]
pub struct snd_interval;

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct xen_front_cfg_pcm_instance;

#[repr(C)]
pub struct xen_front_cfg_card;

#[repr(C)]
pub struct xensnd_query_hw_param;

#[repr(C)]
pub struct iov_iter;

// External function declarations
extern "C" {
    fn xen_snd_front_evtchnl_pair_clear(pair: *mut xen_snd_front_evtchnl_pair);
    fn xen_front_pgdir_shbuf_unmap(shbuf: *mut xen_front_pgdir_shbuf) -> i32;
    fn xen_front_pgdir_shbuf_free(shbuf: *mut xen_front_pgdir_shbuf);
    fn free_pages_exact(addr: *mut u8, size: usize);
    fn kfree(addr: *mut core::ffi::c_void);
    fn xen_snd_front_evtchnl_set_connected(evtchnl: *mut xen_snd_front_evtchnl, connected: bool);
    fn xen_snd_front_stream_query_hw_param(
        evtchnl: *mut xen_snd_front_evtchnl,
        req: *mut xensnd_query_hw_param,
        resp: *mut xensnd_query_hw_param,
    ) -> i32;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_refine(mask: *mut snd_mask, refine: *const snd_mask) -> i32;
    fn snd_interval_refine(interval: *mut snd_interval, refine: *const snd_interval) -> i32;
    fn hw_param_mask(
        params: *mut snd_pcm_hw_params,
        var: i32,
    ) -> *mut snd_mask;
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: i32,
    ) -> *mut snd_interval;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut core::ffi::c_void;
    fn pcm_format_to_bits(format: Snd_pcm_format_t) -> u64;
    fn dev_err(dev: *const device, fmt: *const i8, ...);
    fn dev_dbg(dev: *const device, fmt: *const i8, ...);
    fn xen_snd_front_evtchnl_pair_set_connected(pair: *mut xen_snd_front_evtchnl_pair, connected: bool);
    fn alloc_pages_exact(size: usize, gfp_mask: u32) -> *mut u8;
    fn virt_to_page(addr: *mut u8) -> *mut core::ffi::c_void;
    fn kzalloc_objs(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: u32,
        var: i32,
        func: *const core::ffi::c_void,
        private: *mut core::ffi::c_void,
        ...,
    ) -> i32;
    fn xen_front_pgdir_shbuf_alloc(cfg: *mut xen_front_pgdir_shbuf_cfg) -> i32;
    fn xen_front_pgdir_shbuf_map(shbuf: *mut xen_front_pgdir_shbuf) -> i32;
    fn xen_snd_front_stream_prepare(
        evtchnl: *mut xen_snd_front_evtchnl,
        shbuf: *mut xen_front_pgdir_shbuf,
        format: u8,
        channels: u32,
        rate: u32,
        buffer_bytes: u32,
        period_bytes: u32,
    ) -> i32;
    fn xen_snd_front_stream_close(evtchnl: *mut xen_snd_front_evtchnl) -> i32;
    fn xen_snd_front_stream_trigger(evtchnl: *mut xen_snd_front_evtchnl, type_: i32) -> i32;
    fn xen_snd_front_stream_write(evtchnl: *mut xen_snd_front_evtchnl, pos: usize, count: usize) -> i32;
    fn xen_snd_front_stream_read(evtchnl: *mut xen_snd_front_evtchnl, pos: usize, count: usize) -> i32;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_card_new(
        dev: *mut device,
        idx: i32,
        xid: *const i8,
        module: *mut core::ffi::c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> i32;
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn snd_card_free(card: *mut snd_card);
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const i8,
        device: i32,
        playback_count: i32,
        capture_count: i32,
        rpcm: *mut *mut snd_pcm,
    ) -> i32;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: i32, ops: *const snd_pcm_ops);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn copy_from_iter(addr: *mut u8, bytes: usize, iter: *mut iov_iter) -> usize;
    fn copy_to_iter(addr: *const u8, bytes: usize, iter: *mut iov_iter) -> usize;
    fn strscpy(dest: *mut i8, src: *const i8, size: usize) -> isize;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn bytes_to_frames(runtime: *const snd_pcm_runtime, bytes: u64) -> Snd_pcm_uframes_t;
}

#[repr(C)]
pub struct xen_front_pgdir_shbuf_cfg {
    pub xb_dev: *mut core::ffi::c_void,
    pub pgdir: *mut xen_front_pgdir_shbuf,
    pub num_pages: i32,
    pub pages: *mut *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32) -> i32>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> Snd_pcm_uframes_t>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32, u32, *mut iov_iter, u32) -> i32>,
    pub fill_silence: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32, u32, u32) -> i32>,
}

static ALSA_SNDIF_FORMATS: &[alsa_sndif_sample_format] = &[
    alsa_sndif_sample_format { sndif: 0, alsa: 0 },
    alsa_sndif_sample_format { sndif: 1, alsa: 1 },
    alsa_sndif_sample_format { sndif: 4, alsa: 4 },
    alsa_sndif_sample_format { sndif: 5, alsa: 5 },
    alsa_sndif_sample_format { sndif: 2, alsa: 2 },
    alsa_sndif_sample_format { sndif: 3, alsa: 3 },
    alsa_sndif_sample_format { sndif: 8, alsa: 8 },
    alsa_sndif_sample_format { sndif: 9, alsa: 9 },
    alsa_sndif_sample_format { sndif: 6, alsa: 6 },
    alsa_sndif_sample_format { sndif: 7, alsa: 7 },
    alsa_sndif_sample_format { sndif: 12, alsa: 12 },
    alsa_sndif_sample_format { sndif: 13, alsa: 13 },
    alsa_sndif_sample_format { sndif: 10, alsa: 10 },
    alsa_sndif_sample_format { sndif: 11, alsa: 11 },
    alsa_sndif_sample_format { sndif: 14, alsa: 14 },
    alsa_sndif_sample_format { sndif: 15, alsa: 15 },
    alsa_sndif_sample_format { sndif: 16, alsa: 16 },
    alsa_sndif_sample_format { sndif: 17, alsa: 17 },
    alsa_sndif_sample_format { sndif: 18, alsa: 18 },
    alsa_sndif_sample_format { sndif: 19, alsa: 19 },
    alsa_sndif_sample_format { sndif: 20, alsa: 20 },
    alsa_sndif_sample_format { sndif: 21, alsa: 21 },
    alsa_sndif_sample_format { sndif: 22, alsa: 22 },
    alsa_sndif_sample_format { sndif: 23, alsa: 23 },
    alsa_sndif_sample_format { sndif: 24, alsa: 24 },
];

fn to_sndif_format(format: Snd_pcm_format_t) -> i32 {
    for i in 0..ALSA_SNDIF_FORMATS.len() {
        if ALSA_SNDIF_FORMATS[i].alsa == format {
            return ALSA_SNDIF_FORMATS[i].sndif as i32;
        }
    }
    -22 // -EINVAL
}

fn to_sndif_formats_mask(alsa_formats: u64) -> u64 {
    let mut mask: u64 = 0;

    for i in 0..ALSA_SNDIF_FORMATS.len() {
        unsafe {
            if (pcm_format_to_bits(ALSA_SNDIF_FORMATS[i].alsa) & alsa_formats) != 0 {
                mask |= 1u64 << ALSA_SNDIF_FORMATS[i].sndif;
            }
        }
    }

    mask
}

fn to_alsa_formats_mask(sndif_formats: u64) -> u64 {
    let mut mask: u64 = 0;

    for i in 0..ALSA_SNDIF_FORMATS.len() {
        unsafe {
            if ((1u64 << ALSA_SNDIF_FORMATS[i].sndif) & sndif_formats) != 0 {
                mask |= pcm_format_to_bits(ALSA_SNDIF_FORMATS[i].alsa);
            }
        }
    }

    mask
}

unsafe fn stream_clear(stream: *mut xen_snd_front_pcm_stream_info) {
    (*stream).is_open = false;
    (*stream).be_cur_frame = 0;
    (*stream).out_frames = 0;
    (*stream).hw_ptr.store(0, Ordering::Relaxed);
    xen_snd_front_evtchnl_pair_clear((*stream).evt_pair);
    memset(&mut (*stream).shbuf as *mut _ as *mut core::ffi::c_void, 0, std::mem::size_of::<xen_front_pgdir_shbuf>());
    (*stream).buffer = ptr::null_mut();
    (*stream).buffer_sz = 0;
    (*stream).pages = ptr::null_mut();
    (*stream).num_pages = 0;
}

unsafe fn stream_free(stream: *mut xen_snd_front_pcm_stream_info) {
    xen_front_pgdir_shbuf_unmap(&mut (*stream).shbuf);
    xen_front_pgdir_shbuf_free(&mut (*stream).shbuf);
    if !(*stream).buffer.is_null() {
        free_pages_exact((*stream).buffer, (*stream).buffer_sz);
    }
    if !(*stream).pages.is_null() {
        kfree((*stream).pages as *mut core::ffi::c_void);
    }
    stream_clear(stream);
}

unsafe fn stream_get(substream: *mut snd_pcm_substream) -> *mut xen_snd_front_pcm_stream_info {
    let pcm_instance = snd_pcm_substream_chip(substream) as *mut xen_snd_front_pcm_instance_info;
    let stream: *mut xen_snd_front_pcm_stream_info;

    // Constants for stream type (would be defined in external headers)
    const SNDRV_PCM_STREAM_PLAYBACK: i32 = 0;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        stream = (*pcm_instance).streams_pb.add((*substream).number as usize);
    } else {
        stream = (*pcm_instance).streams_cap.add((*substream).number as usize);
    }

    stream
}

unsafe extern "C" fn alsa_hw_rule(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> i32 {
    let stream = (*rule).private as *mut xen_snd_front_pcm_stream_info;
    let front_info = (*stream).front_info;
    let dev = &(*front_info).xb_dev as *const _ as *const device;

    // Constants from ALSA headers
    const SNDRV_PCM_HW_PARAM_FORMAT: i32 = 0;
    const SNDRV_PCM_HW_PARAM_RATE: i32 = 10;
    const SNDRV_PCM_HW_PARAM_CHANNELS: i32 = 9;
    const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: i32 = 12;
    const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: i32 = 13;

    let formats = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let rates = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let period = hw_param_interval(params, SNDRV_PCM_HW_PARAM_PERIOD_SIZE);
    let buffer = hw_param_interval(params, SNDRV_PCM_HW_PARAM_BUFFER_SIZE);

    let mut req = std::mem::zeroed::<xensnd_query_hw_param>();
    let mut resp = std::mem::zeroed::<xensnd_query_hw_param>();

    // Collect all the values we need for the query.
    req.formats = to_sndif_formats_mask((*formats).bits[0] as u64 | (((*formats).bits[1] as u64) << 32));

    req.rates.min = (*rates).min;
    req.rates.max = (*rates).max;

    req.channels.min = (*channels).min;
    req.channels.max = (*channels).max;

    req.buffer.min = (*buffer).min;
    req.buffer.max = (*buffer).max;

    req.period.min = (*period).min;
    req.period.max = (*period).max;

    let ret = xen_snd_front_stream_query_hw_param(&mut (*stream).evt_pair.as_mut().unwrap().req, &mut req, &mut resp);
    if ret < 0 {
        // Check if this is due to backend communication error.
        const EIO: i32 = -5;
        const ETIMEDOUT: i32 = -110;
        if ret == EIO || ret == ETIMEDOUT {
            dev_err(dev, "Failed to query ALSA HW parameters\0".as_ptr() as *const i8);
        }
        return ret;
    }

    // Refine HW parameters after the query.
    let mut changed = 0;

    let sndif_formats = to_alsa_formats_mask(resp.formats);
    let mut mask = std::mem::zeroed::<snd_mask>();
    snd_mask_none(&mut mask);
    mask.bits[0] = sndif_formats as u32;
    mask.bits[1] = (sndif_formats >> 32) as u32;
    let ret = snd_mask_refine(formats, &mask);
    if ret < 0 {
        return ret;
    }
    changed |= ret;

    let mut interval = std::mem::zeroed::<snd_interval>();
    interval.openmin = 0;
    interval.openmax = 0;
    interval.integer = 1;

    interval.min = resp.rates.min;
    interval.max = resp.rates.max;
    let ret = snd_interval_refine(rates, &interval);
    if ret < 0 {
        return ret;
    }
    changed |= ret;

    interval.min = resp.channels.min;
    interval.max = resp.channels.max;
    let ret = snd_interval_refine(channels, &interval);
    if ret < 0 {
        return ret;
    }
    changed |= ret;

    interval.min = resp.buffer.min;
    interval.max = resp.buffer.max;
    let ret = snd_interval_refine(buffer, &interval);
    if ret < 0 {
        return ret;
    }
    changed |= ret;

    interval.min = resp.period.min;
    interval.max = resp.period.max;
    let ret = snd_interval_refine(period, &interval);
    if ret < 0 {
        return ret;
    }
    changed |= ret;

    changed
}

unsafe extern "C" fn alsa_open(substream: *mut snd_pcm_substream) -> i32 {
    let pcm_instance = snd_pcm_substream_chip(substream) as *mut xen_snd_front_pcm_instance_info;
    let stream = stream_get(substream);
    let runtime = (*substream).runtime;
    let front_info = (*(*pcm_instance).card_info).front_info;
    let dev = &(*front_info).xb_dev as *const _ as *const device;

    // Constants from ALSA headers
    const SNDRV_PCM_STREAM_PLAYBACK: i32 = 0;
    const SNDRV_PCM_HW_PARAM_FORMAT: i32 = 0;
    const SNDRV_PCM_HW_PARAM_RATE: i32 = 10;
    const SNDRV_PCM_HW_PARAM_CHANNELS: i32 = 9;
    const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: i32 = 12;
    const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: i32 = 13;
    const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
    const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
    const SNDRV_PCM_INFO_DOUBLE: u32 = 1 << 2;
    const SNDRV_PCM_INFO_BATCH: u32 = 1 << 3;
    const SNDRV_PCM_INFO_NONINTERLEAVED: u32 = 1 << 4;
    const SNDRV_PCM_INFO_RESUME: u32 = 1 << 5;
    const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 6;
    const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 7;

    // Return our HW properties: override defaults with those configured via XenStore.
    (*runtime).hw = (*stream).pcm_hw;
    (*runtime).hw.info &= !(SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_DOUBLE
        | SNDRV_PCM_INFO_BATCH
        | SNDRV_PCM_INFO_NONINTERLEAVED
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_PAUSE);
    (*runtime).hw.info |= SNDRV_PCM_INFO_INTERLEAVED;

    (*stream).evt_pair = (*front_info).evt_pairs.add((*stream).index as usize);

    (*stream).front_info = front_info;

    (*(*stream).evt_pair).evt.u.evt.substream = substream;

    stream_clear(stream);

    xen_snd_front_evtchnl_set_connected(&mut (*stream).evt_pair.as_mut().unwrap().req, true);

    let ret = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_FORMAT,
        alsa_hw_rule as *const core::ffi::c_void,
        stream as *mut core::ffi::c_void,
        SNDRV_PCM_HW_PARAM_FORMAT,
        -1,
    );
    if ret != 0 {
        dev_err(
            dev,
            "Failed to add HW rule for SNDRV_PCM_HW_PARAM_FORMAT\0".as_ptr() as *const i8,
        );
        return ret;
    }

    let ret = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        alsa_hw_rule as *const core::ffi::c_void,
        stream as *mut core::ffi::c_void,
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    );
    if ret != 0 {
        dev_err(
            dev,
            "Failed to add HW rule for SNDRV_PCM_HW_PARAM_RATE\0".as_ptr() as *const i8,
        );
        return ret;
    }

    let ret = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        alsa_hw_rule as *const core::ffi::c_void,
        stream as *mut core::ffi::c_void,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if ret != 0 {
        dev_err(
            dev,
            "Failed to add HW rule for SNDRV_PCM_HW_PARAM_CHANNELS\0".as_ptr() as *const i8,
        );
        return ret;
    }

    let ret = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
        alsa_hw_rule as *const core::ffi::c_void,
        stream as *mut core::ffi::c_void,
        SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
        -1,
    );
    if ret != 0 {
        dev_err(
            dev,
            "Failed to add HW rule for SNDRV_PCM_HW_PARAM_PERIOD_SIZE\0".as_ptr() as *const i8,
        );
        return ret;
    }

    let ret = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
        alsa_hw_rule as *const core::ffi::c_void,
        stream as *mut core::ffi::c_void,
        SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
        -1,
    );
    if ret != 0 {
        dev_err(
            dev,
            "Failed to add HW rule for SNDRV_PCM_HW_PARAM_BUFFER_SIZE\0".as_ptr() as *const i8,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn alsa_close(substream: *mut snd_pcm_substream) -> i32 {
    let stream = stream_get(substream);

    xen_snd_front_evtchnl_pair_set_connected((*stream).evt_pair, false);
    0
}

unsafe fn shbuf_setup_backstore(
    stream: *mut xen_snd_front_pcm_stream_info,
    buffer_sz: usize,
) -> i32 {
    const GFP_KERNEL: u32 = 0x0;

    (*stream).buffer = alloc_pages_exact(buffer_sz, GFP_KERNEL);
    if (*stream).buffer.is_null() {
        return -12; // -ENOMEM
    }

    (*stream).buffer_sz = buffer_sz;
    (*stream).num_pages = ((buffer_sz + 4096 - 1) / 4096) as i32; // DIV_ROUND_UP with PAGE_SIZE=4096
    (*stream).pages = kzalloc_objs(
        (*stream).num_pages as usize * std::mem::size_of::<*mut core::ffi::c_void>(),
        GFP_KERNEL,
    ) as *mut *mut core::ffi::c_void;
    if (*stream).pages.is_null() {
        return -12; // -ENOMEM
    }

    for i in 0..(*stream).num_pages {
        *(*stream).pages.add(i as usize) = virt_to_page((*stream).buffer.add(i as usize * 4096));
    }

    0
}

unsafe extern "C" fn alsa_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> i32 {
    let stream = stream_get(substream);
    let front_info = (*stream).front_info;
    let mut buf_cfg = std::mem::zeroed::<xen_front_pgdir_shbuf_cfg>();

    // This callback may be called multiple times,
    // so free the previously allocated shared buffer if any.
    stream_free(stream);

    // Get buffer size from params - placeholder implementation
    let buffer_bytes = 0u32; // params_buffer_bytes(params)

    let ret = shbuf_setup_backstore(stream, buffer_bytes as usize);
    if ret < 0 {
        goto_fail(stream, front_info);
        return ret;
    }

    memset(&mut buf_cfg as *mut _ as *mut core::ffi::c_void, 0, std::mem::size_of::<xen_front_pgdir_shbuf_cfg>());
    buf_cfg.xb_dev = (*front_info).xb_dev as *mut core::ffi::c_void;
    buf_cfg.pgdir = &mut (*stream).shbuf;
    buf_cfg.num_pages = (*stream).num_pages;
    buf_cfg.pages = (*stream).pages;

    let ret = xen_front_pgdir_shbuf_alloc(&mut buf_cfg);
    if ret < 0 {
        goto_fail(stream, front_info);
        return ret;
    }

    let ret = xen_front_pgdir_shbuf_map(&mut (*stream).shbuf);
    if ret < 0 {
        goto_fail(stream, front_info);
        return ret;
    }

    0
}

unsafe fn goto_fail(stream: *mut xen_snd_front_pcm_stream_info, front_info: *mut xen_snd_front_info) {
    stream_free(stream);
    dev_err(
        &(*front_info).xb_dev as *const _ as *const device,
        "Failed to allocate buffers for stream with index %d\0".as_ptr() as *const i8,
        (*stream).index,
    );
}

unsafe extern "C" fn alsa_hw_free(substream: *mut snd_pcm_substream) -> i32 {
    let stream = stream_get(substream);

    xen_snd_front_evtchnl_set_connected(&mut (*stream).evt_pair.as_mut().unwrap().evt, false);

    let ret = xen_snd_front_stream_close(&mut (*stream).evt_pair.as_mut().unwrap().req);
    stream_free(stream);
    ret
}

unsafe extern "C" fn alsa_prepare(substream: *mut snd_pcm_substream) -> i32 {
    let stream = stream_get(substream);

    if !(*stream).is_open {
        let runtime = (*substream).runtime;
        let ret = to_sndif_format((*runtime).format);
        if ret < 0 {
            dev_err(
                &(*stream).front_info.as_ref().unwrap().xb_dev as *const _ as *const device,
                "Unsupported sample format: %d\0".as_ptr() as *const i8,
                (*runtime).format,
            );
            return ret;
        }
        let sndif_format = ret as u8;

        let ret = xen_snd_front_stream_prepare(
            &mut (*stream).evt_pair.as_mut().unwrap().req,
            &mut (*stream).shbuf,
            sndif_format,
            (*runtime).channels,
            (*runtime).rate,
            0, // snd_pcm_lib_buffer_bytes(substream)
            0, // snd_pcm_lib_period_bytes(substream)
        );
        if ret < 0 {
            return ret;
        }

        (*stream).is_open = true;
        xen_snd_front_evtchnl_set_connected(&mut (*stream).evt_pair.as_mut().unwrap().evt, true);
    }

    0
}

unsafe extern "C" fn alsa_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    const SNDRV_PCM_TRIGGER_START: i32 = 0;
    const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
    const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 4;
    const SNDRV_PCM_TRIGGER_RESUME: i32 = 3;
    const XENSND_OP_TRIGGER_START: i32 = 0;
    const XENSND_OP_TRIGGER_STOP: i32 = 1;
    const XENSND_OP_TRIGGER_PAUSE: i32 = 2;
    const XENSND_OP_TRIGGER_RESUME: i32 = 3;

    let stream = stream_get(substream);
    let type_ = match cmd {
        SNDRV_PCM_TRIGGER_START => XENSND_OP_TRIGGER_START,
        SNDRV_PCM_TRIGGER_RESUME => XENSND_OP_TRIGGER_RESUME,
        SNDRV_PCM_TRIGGER_STOP => XENSND_OP_TRIGGER_STOP,
        SNDRV_PCM_TRIGGER_SUSPEND => XENSND_OP_TRIGGER_PAUSE,
        _ => return -22, // -EINVAL
    };

    xen_snd_front_stream_trigger(&mut (*stream).evt_pair.as_mut().unwrap().req, type_)
}

#[no_mangle]
pub unsafe extern "C" fn xen_snd_front_alsa_handle_cur_pos(evtchnl: *mut xen_snd_front_evtchnl, pos_bytes: u64) {
    let substream = (*evtchnl).u.evt.substream;
    let stream = stream_get(substream);
    let runtime = (*substream).runtime;

    if (*runtime).buffer_size == 0 || (*runtime).period_size == 0 {
        return;
    }

    let cur_frame = bytes_to_frames(runtime, pos_bytes);

    let delta = cur_frame.wrapping_sub((*stream).be_cur_frame);
    (*stream).be_cur_frame = cur_frame;

    let mut new_hw_ptr = (*stream).hw_ptr.load(Ordering::Relaxed) as Snd_pcm_uframes_t;
    new_hw_ptr = (new_hw_ptr.wrapping_add(delta)) % (*runtime).buffer_size;
    (*stream).hw_ptr.store(new_hw_ptr as u32, Ordering::Relaxed);

    (*stream).out_frames = (*stream).out_frames.wrapping_add(delta as u32);
    if (*stream).out_frames > (*runtime).period_size as u32 {
        (*stream).out_frames %= (*runtime).period_size as u32;
        snd_pcm_period_elapsed(substream);
    }
}

unsafe extern "C" fn alsa_pointer(substream: *mut snd_pcm_substream) -> Snd_pcm_uframes_t {
    let stream = stream_get(substream);
    (*stream).hw_ptr.load(Ordering::Relaxed) as Snd_pcm_uframes_t
}

unsafe extern "C" fn alsa_pb_copy(
    substream: *mut snd_pcm_substream,
    _channel: i32,
    pos: u32,
    src: *mut iov_iter,
    count: u32,
) -> i32 {
    let stream = stream_get(substream);

    if pos as usize + count as usize > (*stream).buffer_sz {
        return -22; // -EINVAL
    }

    if copy_from_iter((*stream).buffer.add(pos as usize), count as usize, src) != count as usize {
        return -14; // -EFAULT
    }

    xen_snd_front_stream_write(&mut (*stream).evt_pair.as_mut().unwrap().req, pos as usize, count as usize)
}

unsafe extern "C" fn alsa_cap_copy(
    substream: *mut snd_pcm_substream,
    _channel: i32,
    pos: u32,
    dst: *mut iov_iter,
    count: u32,
) -> i32 {
    let stream = stream_get(substream);

    if pos as usize + count as usize > (*stream).buffer_sz {
        return -22; // -EINVAL
    }

    let ret = xen_snd_front_stream_read(&mut (*stream).evt_pair.as_mut().unwrap().req, pos as usize, count as usize);
    if ret < 0 {
        return ret;
    }

    if copy_to_iter((*stream).buffer.add(pos as usize), count as usize, dst) != count as usize {
        return -14; // -EFAULT
    }
    0
}

unsafe extern "C" fn alsa_pb_fill_silence(
    substream: *mut snd_pcm_substream,
    _channel: i32,
    pos: u32,
    count: u32,
) -> i32 {
    let stream = stream_get(substream);

    if pos as usize + count as usize > (*stream).buffer_sz {
        return -22; // -EINVAL
    }

    memset((*stream).buffer.add(pos as usize) as *mut core::ffi::c_void, 0, count as usize);

    xen_snd_front_stream_write(&mut (*stream).evt_pair.as_mut().unwrap().req, pos as usize, count as usize)
}

static SND_DRV_ALSA_PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(alsa_open),
    close: Some(alsa_close),
    hw_params: Some(alsa_hw_params),
    hw_free: Some(alsa_hw_free),
    prepare: Some(alsa_prepare),
    trigger: Some(alsa_trigger),
    pointer: Some(alsa_pointer),
    copy: Some(alsa_pb_copy),
    fill_silence: Some(alsa_pb_fill_silence),
};

static SND_DRV_ALSA_CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(alsa_open),
    close: Some(alsa_close),
    hw_params: Some(alsa_hw_params),
    hw_free: Some(alsa_hw_free),
    prepare: Some(alsa_prepare),
    trigger: Some(alsa_trigger),
    pointer: Some(alsa_pointer),
    copy: Some(alsa_cap_copy),
    fill_silence: None,
};

unsafe fn new_pcm_instance(
    card_info: *mut xen_snd_front_card_info,
    instance_cfg: *mut xen_front_cfg_pcm_instance,
    pcm_instance_info: *mut xen_snd_front_pcm_instance_info,
) -> i32 {
    let mut pcm: *mut snd_pcm = ptr::null_mut();

    dev_dbg(
        &(*(*card_info).front_info).xb_dev as *const _ as *const device,
        "New PCM device \"%s\" with id %d playback %d capture %d\0".as_ptr() as *const i8,
    );

    (*pcm_instance_info).card_info = card_info;
    (*pcm_instance_info).pcm_hw = (*instance_cfg).pcm_hw;

    if (*instance_cfg).num_streams_pb != 0 {
        (*pcm_instance_info).streams_pb = devm_kcalloc(
            &(*(*card_info).card).card_dev as *const _ as *mut device,
            (*instance_cfg).num_streams_pb as usize,
            std::mem::size_of::<xen_snd_front_pcm_stream_info>(),
            0x0, // GFP_KERNEL
        ) as *mut xen_snd_front_pcm_stream_info;
        if (*pcm_instance_info).streams_pb.is_null() {
            return -12; // -ENOMEM
        }
    }

    if (*instance_cfg).num_streams_cap != 0 {
        (*pcm_instance_info).streams_cap = devm_kcalloc(
            &(*(*card_info).card).card_dev as *const _ as *mut device,
            (*instance_cfg).num_streams_cap as usize,
            std::mem::size_of::<xen_snd_front_pcm_stream_info>(),
            0x0, // GFP_KERNEL
        ) as *mut xen_snd_front_pcm_stream_info;
        if (*pcm_instance_info).streams_cap.is_null() {
            return -12; // -ENOMEM
        }
    }

    (*pcm_instance_info).num_pcm_streams_pb = (*instance_cfg).num_streams_pb;
    (*pcm_instance_info).num_pcm_streams_cap = (*instance_cfg).num_streams_cap;

    for i in 0..(*pcm_instance_info).num_pcm_streams_pb as usize {
        (*(*pcm_instance_info).streams_pb.add(i)).pcm_hw = (*(*instance_cfg).streams_pb.add(i)).pcm_hw;
        (*(*pcm_instance_info).streams_pb.add(i)).index = (*(*instance_cfg).streams_pb.add(i)).index;
    }

    for i in 0..(*pcm_instance_info).num_pcm_streams_cap as usize {
        (*(*pcm_instance_info).streams_cap.add(i)).pcm_hw = (*(*instance_cfg).streams_cap.add(i)).pcm_hw;
        (*(*pcm_instance_info).streams_cap.add(i)).index = (*(*instance_cfg).streams_cap.add(i)).index;
    }

    const SNDRV_PCM_STREAM_PLAYBACK: i32 = 0;
    const SNDRV_PCM_STREAM_CAPTURE: i32 = 1;

    let ret = snd_pcm_new(
        (*card_info).card,
        ptr::null(), // instance_cfg->name
        (*instance_cfg).device_id,
        (*instance_cfg).num_streams_pb,
        (*instance_cfg).num_streams_cap,
        &mut pcm,
    );
    if ret < 0 {
        return ret;
    }

    (*pcm).private_data = pcm_instance_info as *mut core::ffi::c_void;
    (*pcm).info_flags = 0;
    (*pcm).nonatomic = true;
    strscpy(
        (*pcm).name.as_mut_ptr(),
        "Virtual card PCM\0".as_ptr() as *const i8,
        std::mem::size_of_val(&(*pcm).name),
    );

    if (*instance_cfg).num_streams_pb != 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &SND_DRV_ALSA_PLAYBACK_OPS);
    }

    if (*instance_cfg).num_streams_cap != 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &SND_DRV_ALSA_CAPTURE_OPS);
    }

    (*pcm_instance_info).pcm = pcm;
    0
}

#[no_mangle]
pub unsafe extern "C" fn xen_snd_front_alsa_init(front_info: *mut xen_snd_front_info) -> i32 {
    let dev = &(*front_info).xb_dev as *const _ as *const device;
    let cfg = &(*front_info).cfg;
    let mut card: *mut snd_card = ptr::null_mut();

    dev_dbg(dev, "Creating virtual sound card\n\0".as_ptr() as *const i8);

    let ret = snd_card_new(
        dev as *mut device,
        0,
        "XENSND\0".as_ptr() as *const i8, // XENSND_DRIVER_NAME
        ptr::null_mut(), // THIS_MODULE
        std::mem::size_of::<xen_snd_front_card_info>(),
        &mut card,
    );
    if ret < 0 {
        return ret;
    }

    let card_info = (*card).private_data as *mut xen_snd_front_card_info;
    (*card_info).front_info = front_info;
    (*front_info).card_info = card_info;
    (*card_info).card = card;
    (*card_info).pcm_instances = devm_kcalloc(
        dev as *mut device,
        (*cfg).num_pcm_instances as usize,
        std::mem::size_of::<xen_snd_front_pcm_instance_info>(),
        0x0, // GFP_KERNEL
    ) as *mut xen_snd_front_pcm_instance_info;
    if (*card_info).pcm_instances.is_null() {
        let ret = -12; // -ENOMEM
        goto_fail_init(card);
        return ret;
    }

    (*card_info).num_pcm_instances = (*cfg).num_pcm_instances;
    (*card_info).pcm_hw = (*cfg).pcm_hw;

    for i in 0..(*cfg).num_pcm_instances as usize {
        let ret = new_pcm_instance(
            card_info,
            (*cfg).pcm_instances.add(i),
            (*card_info).pcm_instances.add(i),
        );
        if ret < 0 {
            goto_fail_init(card);
            return ret;
        }
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        "XENSND\0".as_ptr() as *const i8,
        std::mem::size_of_val(&(*card).driver),
    );
    strscpy(
        (*card).shortname.as_mut_ptr(),
        ptr::null(), // cfg->name_short
        std::mem::size_of_val(&(*card).shortname),
    );
    strscpy(
        (*card).longname.as_mut_ptr(),
        ptr::null(), // cfg->name_long
        std::mem::size_of_val(&(*card).longname),
    );

    let ret = snd_card_register(card);
    if ret < 0 {
        goto_fail_init(card);
        return ret;
    }

    0
}

unsafe fn goto_fail_init(card: *mut snd_card) {
    snd_card_free(card);
}

#[no_mangle]
pub unsafe extern "C" fn xen_snd_front_alsa_fini(front_info: *mut xen_snd_front_info) {
    let card_info = (*front_info).card_info;
    if card_info.is_null() {
        return;
    }

    let card = (*card_info).card;
    if card.is_null() {
        return;
    }

    dev_dbg(
        &(*front_info).xb_dev as *const _ as *const device,
        "Removing virtual sound card %d\n\0".as_ptr() as *const i8,
        (*card).number,
    );
    snd_card_free(card);

    // Card_info will be freed when destroying front_info->xb_dev->dev.
    (*card_info).card = ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
