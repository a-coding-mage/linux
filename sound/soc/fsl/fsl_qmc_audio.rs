// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC using the QUICC Multichannel Controller (QMC)
 *
 * Copyright 2022 CS GROUP France
 *
 * Author: Herve Codina <herve.codina@bootlin.com>
 */

// Translated from the C implementation source. Kernel, ALSA, platform, OF, DMA,
// and QMC symbols are external dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type size_t = usize;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type dma_addr_t = u64;
type snd_pcm_uframes_t = c_ulong;
type snd_pcm_access_t = c_int;
type snd_pcm_format_t = c_int;
type gfp_t = c_uint;
type snd_pcm_hw_rule_func_t =
    Option<unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int>;

const DMA_BIT_MASK_32: u64 = (1u64 << 32) - 1;
const GFP_KERNEL: gfp_t = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_ACCESS_MMAP_INTERLEAVED: c_int = 0;
const SNDRV_PCM_ACCESS_RW_INTERLEAVED: c_int = 3;
const SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED: c_int = 1;
const SNDRV_PCM_ACCESS_RW_NONINTERLEAVED: c_int = 4;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_NONINTERLEAVED: c_uint = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 4;
const SNDRV_PCM_HW_PARAM_ACCESS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 3;
const SNDRV_PCM_HW_PARAM_FRAME_BITS: c_int = 4;
const QMC_TRANSPARENT: c_int = 0;
const QMC_CHAN_WRITE: c_int = 0;
const QMC_CHAN_READ: c_int = 1;

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct device_node {
    name: *const c_char,
    parent: *mut device_node,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_card {
    snd_card: *mut snd_card,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    pcm: *mut snd_pcm,
}

#[repr(C)]
struct snd_pcm_runtime {
    private_data: *mut c_void,
    dma_addr: dma_addr_t,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
struct snd_mask {
    bits: [u32; 8],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_rule {
    private: *mut c_void,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
    buffer_bytes_max: size_t,
}

#[repr(C)]
struct snd_soc_component_driver {
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    trigger:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    of_xlate_dai_name: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *const of_phandle_args, *mut *const c_char)
            -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_dai_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    formats: u64,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    id: c_int,
    name: *const c_char,
    playback: snd_soc_dai_stream,
    capture: snd_soc_dai_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
    driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai)
            -> c_int,
    >,
}

#[repr(C)]
struct of_phandle_args {
    args: [u32; 8],
}

#[repr(C)]
struct qmc_chan {
    _private: [u8; 0],
}

#[repr(C)]
struct qmc_chan_transp_param {
    max_rx_buf_size: size_t,
}

#[repr(C)]
struct qmc_chan_param {
    mode: c_int,
    transp: qmc_chan_transp_param,
}

#[repr(C)]
struct qmc_chan_info {
    mode: c_int,
    nb_tx_ts: c_uint,
    nb_rx_ts: c_uint,
    tx_fs_rate: c_ulong,
    rx_fs_rate: c_ulong,
}

#[repr(C)]
struct qmc_chan_ts_info {
    rx_ts_mask: u64,
    tx_ts_mask: u64,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: driver_inner,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct qmc_dai {
    name: *mut c_char,
    id: c_int,
    dev: *mut device,
    nb_tx_ts: c_uint,
    nb_rx_ts: c_uint,

    nb_chans_avail: c_uint,
    nb_chans_used_tx: c_uint,
    nb_chans_used_rx: c_uint,
    qmc_chans: *mut *mut qmc_chan,
}

#[repr(C)]
struct qmc_audio {
    dev: *mut device,
    num_dais: c_uint,
    dais: *mut qmc_dai,
    dai_drivers: *mut snd_soc_dai_driver,
}

#[repr(C)]
struct qmc_dai_prtd {
    qmc_dai: *mut qmc_dai,

    buffer_ended: snd_pcm_uframes_t,
    buffer_size: snd_pcm_uframes_t,
    period_size: snd_pcm_uframes_t,

    ch_dma_addr_start: dma_addr_t,
    ch_dma_addr_current: dma_addr_t,
    ch_dma_addr_end: dma_addr_t,
    ch_dma_size: size_t,
    ch_dma_offset: size_t,

    channels: c_uint,
    substream: *mut snd_pcm_substream,
}

unsafe extern "C" {
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        min: size_t,
        max: size_t,
    );
    fn params_access(params: *mut snd_pcm_hw_params) -> snd_pcm_access_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> snd_pcm_uframes_t;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> snd_pcm_uframes_t;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn qmc_chan_write_submit(
        chan: *mut qmc_chan,
        addr: dma_addr_t,
        size: size_t,
        complete: Option<unsafe extern "C" fn(*mut c_void)>,
        context: *mut c_void,
    ) -> c_int;
    fn qmc_chan_read_submit(
        chan: *mut qmc_chan,
        addr: dma_addr_t,
        size: size_t,
        complete: Option<unsafe extern "C" fn(*mut c_void, size_t, c_uint)>,
        context: *mut c_void,
    ) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_uint;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_test_format(mask: *const snd_mask, format: snd_pcm_format_t) -> c_int;
    fn snd_mask_set_format(mask: *mut snd_mask, format: snd_pcm_format_t);
    fn snd_mask_refine(old: *mut snd_mask, new: *const snd_mask) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: snd_pcm_hw_rule_func_t,
        private: *mut c_void,
        dep: c_int,
        terminator: c_int,
    ) -> c_int;
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        val: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_mask64(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        mask: u64,
    ) -> c_int;
    fn qmc_chan_set_param(chan: *mut qmc_chan, param: *mut qmc_chan_param) -> c_int;
    fn qmc_chan_start(chan: *mut qmc_chan, direction: c_int) -> c_int;
    fn qmc_chan_stop(chan: *mut qmc_chan, direction: c_int) -> c_int;
    fn qmc_chan_reset(chan: *mut qmc_chan, direction: c_int) -> c_int;
    fn snd_pcm_format_little_endian(format: snd_pcm_format_t) -> c_int;
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> u64;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out: *mut u32) -> c_int;
    fn devm_kasprintf(dev: *mut device, flags: gfp_t, fmt: *const c_char, ...) -> *mut c_char;
    fn qmc_chan_count_phandles(np: *mut device_node, name: *const c_char) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: gfp_t) -> *mut c_void;
    fn devm_qmc_chan_get_byphandles_index(
        dev: *mut device,
        np: *mut device_node,
        name: *const c_char,
        index: c_uint,
    ) -> *mut qmc_chan;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn qmc_chan_get_info(chan: *mut qmc_chan, info: *mut qmc_chan_info) -> c_int;
    fn qmc_chan_get_ts_info(chan: *mut qmc_chan, info: *mut qmc_chan_ts_info) -> c_int;
    fn fls64(x: u64) -> c_int;
    fn snd_pcm_rate_to_rate_bit(rate: c_ulong) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: gfp_t) -> *mut c_void;
    fn of_get_available_child_count(np: *mut device_node) -> c_uint;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_ptr(id: *const of_device_id) -> *const of_device_id;
    fn first_available_child_of_node(np: *mut device_node) -> *mut device_node;
    fn next_available_child_of_node(np: *mut device_node, child: *mut device_node)
        -> *mut device_node;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn qmc_audio_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card = (*(*rtd).card).snd_card;
    let mut ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK_32);
    if ret != 0 {
        return ret;
    }

    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        64 * 1024,
        64 * 1024,
    );
    0
}

unsafe extern "C" fn qmc_audio_access_is_interleaved(access: snd_pcm_access_t) -> bool_t {
    match access {
        SNDRV_PCM_ACCESS_MMAP_INTERLEAVED | SNDRV_PCM_ACCESS_RW_INTERLEAVED => true,
        _ => false,
    }
}

unsafe extern "C" fn qmc_audio_pcm_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = (*substream).runtime;
    let prtd = (*(*substream).runtime).private_data as *mut qmc_dai_prtd;

    /*
     * In interleaved mode, the driver uses one QMC channel for all audio
     * channels whereas in non-interleaved mode, it uses one QMC channel per
     * audio channel.
     */
    (*prtd).channels = if qmc_audio_access_is_interleaved(params_access(params)) {
        1
    } else {
        params_channels(params)
    };

    (*prtd).substream = substream;

    (*prtd).buffer_ended = 0;
    (*prtd).buffer_size = params_buffer_size(params);
    (*prtd).period_size = params_period_size(params);

    (*prtd).ch_dma_addr_start = (*runtime).dma_addr;
    (*prtd).ch_dma_offset = params_buffer_bytes(params) / (*prtd).channels as size_t;
    (*prtd).ch_dma_addr_end = (*runtime).dma_addr + (*prtd).ch_dma_offset as dma_addr_t;
    (*prtd).ch_dma_addr_current = (*prtd).ch_dma_addr_start;
    (*prtd).ch_dma_size = params_period_bytes(params) / (*prtd).channels as size_t;

    0
}

unsafe extern "C" fn qmc_audio_pcm_write_complete(context: *mut c_void);

unsafe extern "C" fn qmc_audio_pcm_write_submit(prtd: *mut qmc_dai_prtd) -> c_int {
    let mut i: c_uint;
    let mut ret: c_int;

    i = 0;
    while i < (*prtd).channels {
        ret = qmc_chan_write_submit(
            *(*(*prtd).qmc_dai).qmc_chans.add(i as usize),
            (*prtd).ch_dma_addr_current + (i as dma_addr_t) * (*prtd).ch_dma_offset as dma_addr_t,
            (*prtd).ch_dma_size,
            if i == (*prtd).channels - 1 {
                Some(qmc_audio_pcm_write_complete)
            } else {
                None
            },
            prtd as *mut c_void,
        );
        if ret != 0 {
            dev_err(
                (*(*prtd).qmc_dai).dev,
                c"write_submit %u failed %d\n".as_ptr(),
                i,
                ret,
            );
            return ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn qmc_audio_pcm_write_complete(context: *mut c_void) {
    let prtd = context as *mut qmc_dai_prtd;

    (*prtd).buffer_ended += (*prtd).period_size;
    if (*prtd).buffer_ended >= (*prtd).buffer_size {
        (*prtd).buffer_ended = 0;
    }

    (*prtd).ch_dma_addr_current += (*prtd).ch_dma_size as dma_addr_t;
    if (*prtd).ch_dma_addr_current >= (*prtd).ch_dma_addr_end {
        (*prtd).ch_dma_addr_current = (*prtd).ch_dma_addr_start;
    }

    qmc_audio_pcm_write_submit(prtd);

    snd_pcm_period_elapsed((*prtd).substream);
}

unsafe extern "C" fn qmc_audio_pcm_read_complete(
    context: *mut c_void,
    length: size_t,
    flags: c_uint,
);

unsafe extern "C" fn qmc_audio_pcm_read_submit(prtd: *mut qmc_dai_prtd) -> c_int {
    let mut i: c_uint;
    let mut ret: c_int;

    i = 0;
    while i < (*prtd).channels {
        ret = qmc_chan_read_submit(
            *(*(*prtd).qmc_dai).qmc_chans.add(i as usize),
            (*prtd).ch_dma_addr_current + (i as dma_addr_t) * (*prtd).ch_dma_offset as dma_addr_t,
            (*prtd).ch_dma_size,
            if i == (*prtd).channels - 1 {
                Some(qmc_audio_pcm_read_complete)
            } else {
                None
            },
            prtd as *mut c_void,
        );
        if ret != 0 {
            dev_err(
                (*(*prtd).qmc_dai).dev,
                c"read_submit %u failed %d\n".as_ptr(),
                i,
                ret,
            );
            return ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn qmc_audio_pcm_read_complete(
    context: *mut c_void,
    length: size_t,
    _flags: c_uint,
) {
    let prtd = context as *mut qmc_dai_prtd;

    if length != (*prtd).ch_dma_size {
        dev_err(
            (*(*prtd).qmc_dai).dev,
            c"read complete length = %zu, exp %zu\n".as_ptr(),
            length,
            (*prtd).ch_dma_size,
        );
    }

    (*prtd).buffer_ended += (*prtd).period_size;
    if (*prtd).buffer_ended >= (*prtd).buffer_size {
        (*prtd).buffer_ended = 0;
    }

    (*prtd).ch_dma_addr_current += (*prtd).ch_dma_size as dma_addr_t;
    if (*prtd).ch_dma_addr_current >= (*prtd).ch_dma_addr_end {
        (*prtd).ch_dma_addr_current = (*prtd).ch_dma_addr_start;
    }

    qmc_audio_pcm_read_submit(prtd);

    snd_pcm_period_elapsed((*prtd).substream);
}

unsafe extern "C" fn qmc_audio_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let prtd = (*(*substream).runtime).private_data as *mut qmc_dai_prtd;
    let mut ret: c_int;

    if (*prtd).qmc_dai.is_null() {
        dev_err((*component).dev, c"qmc_dai is not set\n".as_ptr());
        return -EINVAL;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            (*prtd).buffer_ended = 0;
            (*prtd).ch_dma_addr_current = (*prtd).ch_dma_addr_start;

            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                /* Submit first chunk ... */
                ret = qmc_audio_pcm_write_submit(prtd);
                if ret != 0 {
                    return ret;
                }

                /* ... prepare next one ... */
                (*prtd).ch_dma_addr_current += (*prtd).ch_dma_size as dma_addr_t;
                if (*prtd).ch_dma_addr_current >= (*prtd).ch_dma_addr_end {
                    (*prtd).ch_dma_addr_current = (*prtd).ch_dma_addr_start;
                }

                /* ... and send it */
                ret = qmc_audio_pcm_write_submit(prtd);
                if ret != 0 {
                    return ret;
                }
            } else {
                /* Submit first chunk ... */
                ret = qmc_audio_pcm_read_submit(prtd);
                if ret != 0 {
                    return ret;
                }

                /* ... prepare next one ... */
                (*prtd).ch_dma_addr_current += (*prtd).ch_dma_size as dma_addr_t;
                if (*prtd).ch_dma_addr_current >= (*prtd).ch_dma_addr_end {
                    (*prtd).ch_dma_addr_current = (*prtd).ch_dma_addr_start;
                }

                /* ... and send it */
                ret = qmc_audio_pcm_read_submit(prtd);
                if ret != 0 {
                    return ret;
                }
            }
        }
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {}
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {}
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn qmc_audio_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let prtd = (*(*substream).runtime).private_data as *mut qmc_dai_prtd;

    (*prtd).buffer_ended
}

unsafe extern "C" fn qmc_audio_of_xlate_dai_name(
    component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
) -> c_int {
    let qmc_audio = dev_get_drvdata((*component).dev) as *mut qmc_audio;
    let mut dai_driver: *mut snd_soc_dai_driver;
    let id = (*args).args[0] as c_int;
    let mut i: c_int;

    i = 0;
    while i < (*qmc_audio).num_dais as c_int {
        dai_driver = (*qmc_audio).dai_drivers.add(i as usize);
        if (*dai_driver).id == id {
            *dai_name = (*dai_driver).name;
            return 0;
        }
        i += 1;
    }

    -EINVAL
}

static qmc_audio_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_NONINTERLEAVED
        | SNDRV_PCM_INFO_PAUSE,
    period_bytes_min: 32,
    period_bytes_max: 64 * 1024,
    periods_min: 2,
    periods_max: 2 * 1024,
    buffer_bytes_max: 64 * 1024,
};

unsafe extern "C" fn qmc_audio_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let mut prtd: *mut qmc_dai_prtd;
    let mut ret: c_int;

    snd_soc_set_runtime_hwparams(substream, &qmc_audio_pcm_hardware);

    /* ensure that buffer size is a multiple of period size */
    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    prtd = kzalloc(size_of::<qmc_dai_prtd>(), GFP_KERNEL) as *mut qmc_dai_prtd;
    if prtd.is_null() {
        return -ENOMEM;
    }

    (*runtime).private_data = prtd as *mut c_void;

    0
}

unsafe extern "C" fn qmc_audio_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let prtd = (*(*substream).runtime).private_data as *mut qmc_dai_prtd;

    kfree(prtd as *mut c_void);
    0
}

static qmc_audio_soc_platform: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(qmc_audio_pcm_open),
    close: Some(qmc_audio_pcm_close),
    hw_params: Some(qmc_audio_pcm_hw_params),
    trigger: Some(qmc_audio_pcm_trigger),
    pointer: Some(qmc_audio_pcm_pointer),
    pcm_new: Some(qmc_audio_pcm_new),
    of_xlate_dai_name: Some(qmc_audio_of_xlate_dai_name),
};

unsafe extern "C" fn qmc_dai_get_index(dai: *mut snd_soc_dai) -> c_uint {
    let qmc_audio = snd_soc_dai_get_drvdata(dai) as *mut qmc_audio;

    (*dai).driver.offset_from((*qmc_audio).dai_drivers) as c_uint
}

unsafe extern "C" fn qmc_dai_get_data(dai: *mut snd_soc_dai) -> *mut qmc_dai {
    let qmc_audio = snd_soc_dai_get_drvdata(dai) as *mut qmc_audio;
    let mut index: c_uint;

    index = qmc_dai_get_index(dai);
    if index > (*qmc_audio).num_dais {
        return ptr::null_mut();
    }

    (*qmc_audio).dais.add(index as usize)
}

/*
 * The constraints for format/channel is to match with the number of 8bit
 * time-slots available.
 */
unsafe extern "C" fn qmc_dai_hw_rule_channels_by_format(
    qmc_dai: *mut qmc_dai,
    params: *mut snd_pcm_hw_params,
    nb_ts: c_uint,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let format = params_format(params);
    let mut ch: snd_interval = snd_interval { min: 0, max: 0 };

    match snd_pcm_format_physical_width(format) {
        8 => ch.max = nb_ts,
        16 => ch.max = nb_ts / 2,
        32 => ch.max = nb_ts / 4,
        64 => ch.max = nb_ts / 8,
        _ => {
            dev_err(
                (*qmc_dai).dev,
                c"format physical width %u not supported\n".as_ptr(),
                snd_pcm_format_physical_width(format),
            );
            return -EINVAL;
        }
    }

    ch.min = if ch.max != 0 { 1 } else { 0 };

    snd_interval_refine(c, &ch)
}

unsafe extern "C" fn qmc_dai_hw_rule_playback_channels_by_format(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let qmc_dai = (*rule).private as *mut qmc_dai;

    qmc_dai_hw_rule_channels_by_format(qmc_dai, params, (*qmc_dai).nb_tx_ts)
}

unsafe extern "C" fn qmc_dai_hw_rule_capture_channels_by_format(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let qmc_dai = (*rule).private as *mut qmc_dai;

    qmc_dai_hw_rule_channels_by_format(qmc_dai, params, (*qmc_dai).nb_rx_ts)
}

unsafe extern "C" fn qmc_dai_hw_rule_format_by_channels(
    qmc_dai: *mut qmc_dai,
    params: *mut snd_pcm_hw_params,
    nb_ts: c_uint,
) -> c_int {
    let f_old = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let channels = params_channels(params);
    let mut slot_width: c_uint;
    let mut format: snd_pcm_format_t;
    let mut f_new: snd_mask = snd_mask { bits: [0; 8] };

    if channels == 0 || channels > nb_ts {
        dev_err(
            (*qmc_dai).dev,
            c"channels %u not supported\n".as_ptr(),
            nb_ts,
        );
        return -EINVAL;
    }

    slot_width = (nb_ts / channels) * 8;

    snd_mask_none(&mut f_new);
    // Translation of pcm_for_each_format(format).
    format = 0;
    while format <= 64 {
        if snd_mask_test_format(f_old, format) != 0 {
            if snd_pcm_format_physical_width(format) <= slot_width {
                snd_mask_set_format(&mut f_new, format);
            }
        }
        format += 1;
    }

    snd_mask_refine(f_old, &f_new)
}

unsafe extern "C" fn qmc_dai_hw_rule_playback_format_by_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let qmc_dai = (*rule).private as *mut qmc_dai;

    qmc_dai_hw_rule_format_by_channels(qmc_dai, params, (*qmc_dai).nb_tx_ts)
}

unsafe extern "C" fn qmc_dai_hw_rule_capture_format_by_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let qmc_dai = (*rule).private as *mut qmc_dai;

    qmc_dai_hw_rule_format_by_channels(qmc_dai, params, (*qmc_dai).nb_rx_ts)
}

unsafe extern "C" fn qmc_dai_constraints_interleaved(
    substream: *mut snd_pcm_substream,
    qmc_dai: *mut qmc_dai,
) -> c_int {
    let mut hw_rule_channels_by_format: snd_pcm_hw_rule_func_t;
    let mut hw_rule_format_by_channels: snd_pcm_hw_rule_func_t;
    let mut frame_bits: c_uint;
    let mut access: u64;
    let mut ret: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        hw_rule_channels_by_format = Some(qmc_dai_hw_rule_capture_channels_by_format);
        hw_rule_format_by_channels = Some(qmc_dai_hw_rule_capture_format_by_channels);
        frame_bits = (*qmc_dai).nb_rx_ts * 8;
    } else {
        hw_rule_channels_by_format = Some(qmc_dai_hw_rule_playback_channels_by_format);
        hw_rule_format_by_channels = Some(qmc_dai_hw_rule_playback_format_by_channels);
        frame_bits = (*qmc_dai).nb_tx_ts * 8;
    }

    ret = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        hw_rule_channels_by_format,
        qmc_dai as *mut c_void,
        SNDRV_PCM_HW_PARAM_FORMAT,
        -1,
    );
    if ret != 0 {
        dev_err(
            (*qmc_dai).dev,
            c"Failed to add channels rule (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_FORMAT,
        hw_rule_format_by_channels,
        qmc_dai as *mut c_void,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if ret != 0 {
        dev_err(
            (*qmc_dai).dev,
            c"Failed to add format rule (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_pcm_hw_constraint_single(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_FRAME_BITS,
        frame_bits,
    );
    if ret < 0 {
        dev_err(
            (*qmc_dai).dev,
            c"Failed to add frame_bits constraint (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    access = (1u64 << SNDRV_PCM_ACCESS_MMAP_INTERLEAVED) | (1u64 << SNDRV_PCM_ACCESS_RW_INTERLEAVED);
    ret = snd_pcm_hw_constraint_mask64((*substream).runtime, SNDRV_PCM_HW_PARAM_ACCESS, access);
    if ret != 0 {
        dev_err(
            (*qmc_dai).dev,
            c"Failed to add hw_param_access constraint (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn qmc_dai_constraints_noninterleaved(
    substream: *mut snd_pcm_substream,
    qmc_dai: *mut qmc_dai,
) -> c_int {
    let mut frame_bits: c_uint;
    let mut access: u64;
    let mut ret: c_int;

    frame_bits = if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*qmc_dai).nb_rx_ts * 8
    } else {
        (*qmc_dai).nb_tx_ts * 8
    };
    ret = snd_pcm_hw_constraint_single(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_FRAME_BITS,
        frame_bits,
    );
    if ret < 0 {
        dev_err(
            (*qmc_dai).dev,
            c"Failed to add frame_bits constraint (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    access =
        (1u64 << SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED) | (1u64 << SNDRV_PCM_ACCESS_RW_NONINTERLEAVED);
    ret = snd_pcm_hw_constraint_mask64((*substream).runtime, SNDRV_PCM_HW_PARAM_ACCESS, access);
    if ret != 0 {
        dev_err(
            (*qmc_dai).dev,
            c"Failed to add hw_param_access constraint (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn qmc_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let prtd = (*(*substream).runtime).private_data as *mut qmc_dai_prtd;
    let mut qmc_dai: *mut qmc_dai;

    qmc_dai = qmc_dai_get_data(dai);
    if qmc_dai.is_null() {
        dev_err((*dai).dev, c"Invalid dai\n".as_ptr());
        return -EINVAL;
    }

    (*prtd).qmc_dai = qmc_dai;

    if (*qmc_dai).nb_chans_avail > 1 {
        qmc_dai_constraints_noninterleaved(substream, qmc_dai)
    } else {
        qmc_dai_constraints_interleaved(substream, qmc_dai)
    }
}

unsafe extern "C" fn qmc_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut chan_param: qmc_chan_param = qmc_chan_param {
        mode: 0,
        transp: qmc_chan_transp_param { max_rx_buf_size: 0 },
    };
    let mut nb_chans_used: c_uint;
    let mut qmc_dai: *mut qmc_dai;
    let mut i: c_uint;
    let mut ret: c_int;

    qmc_dai = qmc_dai_get_data(dai);
    if qmc_dai.is_null() {
        dev_err((*dai).dev, c"Invalid dai\n".as_ptr());
        return -EINVAL;
    }

    /*
     * In interleaved mode, the driver uses one QMC channel for all audio
     * channels whereas in non-interleaved mode, it uses one QMC channel per
     * audio channel.
     */
    nb_chans_used = if qmc_audio_access_is_interleaved(params_access(params)) {
        1
    } else {
        params_channels(params)
    };

    if nb_chans_used > (*qmc_dai).nb_chans_avail {
        dev_err(
            (*dai).dev,
            c"Not enough qmc_chans. Need %u, avail %u\n".as_ptr(),
            nb_chans_used,
            (*qmc_dai).nb_chans_avail,
        );
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        chan_param.mode = QMC_TRANSPARENT;
        chan_param.transp.max_rx_buf_size = params_period_bytes(params) / nb_chans_used as size_t;
        i = 0;
        while i < nb_chans_used {
            ret = qmc_chan_set_param(*(*qmc_dai).qmc_chans.add(i as usize), &mut chan_param);
            if ret != 0 {
                dev_err(
                    (*dai).dev,
                    c"qmc_chans[%u], set param failed %d\n".as_ptr(),
                    i,
                    ret,
                );
                return ret;
            }
            i += 1;
        }
        (*qmc_dai).nb_chans_used_rx = nb_chans_used;
    } else {
        (*qmc_dai).nb_chans_used_tx = nb_chans_used;
    }

    0
}

unsafe extern "C" fn qmc_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut nb_chans_used: c_uint;
    let mut qmc_dai: *mut qmc_dai;
    let mut i: c_uint;
    let mut direction: c_int;
    let mut ret: c_int = 0;
    let mut ret_tmp: c_int;

    qmc_dai = qmc_dai_get_data(dai);
    if qmc_dai.is_null() {
        dev_err((*dai).dev, c"Invalid dai\n".as_ptr());
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        direction = QMC_CHAN_WRITE;
        nb_chans_used = (*qmc_dai).nb_chans_used_tx;
    } else {
        direction = QMC_CHAN_READ;
        nb_chans_used = (*qmc_dai).nb_chans_used_rx;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            i = 0;
            while i < nb_chans_used {
                ret = qmc_chan_start(*(*qmc_dai).qmc_chans.add(i as usize), direction);
                if ret != 0 {
                    while i != 0 {
                        i -= 1;
                        qmc_chan_stop(*(*qmc_dai).qmc_chans.add(i as usize), direction);
                        qmc_chan_reset(*(*qmc_dai).qmc_chans.add(i as usize), direction);
                    }
                    return ret;
                }
                i += 1;
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            /* Stop and reset all QMC channels and return the first error encountered */
            i = 0;
            while i < nb_chans_used {
                ret_tmp = qmc_chan_stop(*(*qmc_dai).qmc_chans.add(i as usize), direction);
                if ret == 0 {
                    ret = ret_tmp;
                }
                if ret_tmp != 0 {
                    i += 1;
                    continue;
                }

                ret_tmp = qmc_chan_reset(*(*qmc_dai).qmc_chans.add(i as usize), direction);
                if ret == 0 {
                    ret = ret_tmp;
                }
                i += 1;
            }
            if ret != 0 {
                return ret;
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /* Stop all QMC channels and return the first error encountered */
            i = 0;
            while i < nb_chans_used {
                ret_tmp = qmc_chan_stop(*(*qmc_dai).qmc_chans.add(i as usize), direction);
                if ret == 0 {
                    ret = ret_tmp;
                }
                i += 1;
            }
            if ret != 0 {
                return ret;
            }
        }
        _ => return -EINVAL,
    }

    0
}

static qmc_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(qmc_dai_startup),
    trigger: Some(qmc_dai_trigger),
    hw_params: Some(qmc_dai_hw_params),
};

unsafe extern "C" fn qmc_audio_formats(nb_ts: u8, is_noninterleaved: bool_t) -> u64 {
    let mut format_width: c_uint;
    let mut chan_width: c_uint;
    let mut format: snd_pcm_format_t;
    let mut formats_mask: u64;

    if nb_ts == 0 {
        return 0;
    }

    formats_mask = 0;
    chan_width = nb_ts as c_uint * 8;
    // Translation of pcm_for_each_format(format).
    format = 0;
    while format <= 64 {
        /*
         * Support format other than little-endian (ie big-endian or
         * without endianness such as 8bit formats)
         */
        if snd_pcm_format_little_endian(format) == 1 {
            format += 1;
            continue;
        }

        /* Support physical width multiple of 8bit */
        format_width = snd_pcm_format_physical_width(format);
        if format_width == 0 || format_width % 8 != 0 {
            format += 1;
            continue;
        }

        /*
         * And support physical width that can fit N times in the
         * channel
         */
        if format_width > chan_width || chan_width % format_width != 0 {
            format += 1;
            continue;
        }

        /*
         * In non interleaved mode, we can only support formats that
         * can fit only 1 time in the channel
         */
        if is_noninterleaved && format_width != chan_width {
            format += 1;
            continue;
        }

        formats_mask |= pcm_format_to_bits(format);
        format += 1;
    }
    formats_mask
}

unsafe extern "C" fn qmc_audio_dai_parse(
    qmc_audio: *mut qmc_audio,
    np: *mut device_node,
    qmc_dai: *mut qmc_dai,
    qmc_soc_dai_driver: *mut snd_soc_dai_driver,
) -> c_int {
    let mut ts_info: qmc_chan_ts_info = qmc_chan_ts_info {
        rx_ts_mask: 0,
        tx_ts_mask: 0,
    };
    let mut info: qmc_chan_info = qmc_chan_info {
        mode: 0,
        nb_tx_ts: 0,
        nb_rx_ts: 0,
        tx_fs_rate: 0,
        rx_fs_rate: 0,
    };
    let mut rx_fs_rate: c_ulong = 0;
    let mut tx_fs_rate: c_ulong = 0;
    let mut prev_last_rx_ts: c_int = 0;
    let mut prev_last_tx_ts: c_int = 0;
    let mut nb_tx_ts: c_uint = 0;
    let mut nb_rx_ts: c_uint = 0;
    let mut i: c_uint;
    let mut last_rx_ts: c_int;
    let mut last_tx_ts: c_int;
    let mut count: c_int;
    let mut val: u32 = 0;
    let mut ret: c_int;

    (*qmc_dai).dev = (*qmc_audio).dev;

    ret = of_property_read_u32(np, c"reg".as_ptr(), &mut val);
    if ret != 0 {
        dev_err((*qmc_audio).dev, c"%pOF: failed to read reg\n".as_ptr(), np);
        return ret;
    }
    (*qmc_dai).id = val as c_int;

    (*qmc_dai).name = devm_kasprintf(
        (*qmc_audio).dev,
        GFP_KERNEL,
        c"%s.%d".as_ptr(),
        (*(*np).parent).name,
        (*qmc_dai).id,
    );
    if (*qmc_dai).name.is_null() {
        return -ENOMEM;
    }

    count = qmc_chan_count_phandles(np, c"fsl,qmc-chan".as_ptr());
    if count < 0 {
        return dev_err_probe(
            (*qmc_audio).dev,
            count,
            c"dai %d get number of QMC channel failed\n".as_ptr(),
            (*qmc_dai).id,
        );
    }
    if count == 0 {
        return dev_err_probe(
            (*qmc_audio).dev,
            -EINVAL,
            c"dai %d no QMC channel defined\n".as_ptr(),
            (*qmc_dai).id,
        );
    }

    (*qmc_dai).qmc_chans = devm_kcalloc(
        (*qmc_audio).dev,
        count as size_t,
        size_of::<*mut qmc_chan>(),
        GFP_KERNEL,
    ) as *mut *mut qmc_chan;
    if (*qmc_dai).qmc_chans.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < count as c_uint {
        *(*qmc_dai).qmc_chans.add(i as usize) = devm_qmc_chan_get_byphandles_index(
            (*qmc_audio).dev,
            np,
            c"fsl,qmc-chan".as_ptr(),
            i,
        );
        if IS_ERR(*(*qmc_dai).qmc_chans.add(i as usize) as *const c_void) {
            return dev_err_probe(
                (*qmc_audio).dev,
                PTR_ERR(*(*qmc_dai).qmc_chans.add(i as usize) as *const c_void),
                c"dai %d get QMC channel %d failed\n".as_ptr(),
                (*qmc_dai).id,
                i,
            );
        }

        ret = qmc_chan_get_info(*(*qmc_dai).qmc_chans.add(i as usize), &mut info);
        if ret != 0 {
            dev_err(
                (*qmc_audio).dev,
                c"dai %d get QMC %d channel info failed %d\n".as_ptr(),
                (*qmc_dai).id,
                i,
                ret,
            );
            return ret;
        }

        if info.mode != QMC_TRANSPARENT {
            dev_err(
                (*qmc_audio).dev,
                c"dai %d QMC chan %d mode %d is not QMC_TRANSPARENT\n".as_ptr(),
                (*qmc_dai).id,
                i,
                info.mode,
            );
            return -EINVAL;
        }

        /*
         * All channels must have the same number of Tx slots and the
         * same numbers of Rx slots.
         */
        if i == 0 {
            nb_tx_ts = info.nb_tx_ts;
            nb_rx_ts = info.nb_rx_ts;
            tx_fs_rate = info.tx_fs_rate;
            rx_fs_rate = info.rx_fs_rate;
        } else {
            if nb_tx_ts != info.nb_tx_ts {
                dev_err(
                    (*qmc_audio).dev,
                    c"dai %d QMC chan %d inconsistent number of Tx timeslots (%u instead of %u)\n"
                        .as_ptr(),
                    (*qmc_dai).id,
                    i,
                    info.nb_tx_ts,
                    nb_tx_ts,
                );
                return -EINVAL;
            }
            if nb_rx_ts != info.nb_rx_ts {
                dev_err(
                    (*qmc_audio).dev,
                    c"dai %d QMC chan %d inconsistent number of Rx timeslots (%u instead of %u)\n"
                        .as_ptr(),
                    (*qmc_dai).id,
                    i,
                    info.nb_rx_ts,
                    nb_rx_ts,
                );
                return -EINVAL;
            }
            if tx_fs_rate != info.tx_fs_rate {
                dev_err(
                    (*qmc_audio).dev,
                    c"dai %d QMC chan %d inconsistent Tx frame sample rate (%lu instead of %lu)\n"
                        .as_ptr(),
                    (*qmc_dai).id,
                    i,
                    info.tx_fs_rate,
                    tx_fs_rate,
                );
                return -EINVAL;
            }
            if rx_fs_rate != info.rx_fs_rate {
                dev_err(
                    (*qmc_audio).dev,
                    c"dai %d QMC chan %d inconsistent Rx frame sample rate (%lu instead of %lu)\n"
                        .as_ptr(),
                    (*qmc_dai).id,
                    i,
                    info.rx_fs_rate,
                    rx_fs_rate,
                );
                return -EINVAL;
            }
        }

        ret = qmc_chan_get_ts_info(*(*qmc_dai).qmc_chans.add(i as usize), &mut ts_info);
        if ret != 0 {
            dev_err(
                (*qmc_audio).dev,
                c"dai %d get QMC %d channel TS info failed %d\n".as_ptr(),
                (*qmc_dai).id,
                i,
                ret,
            );
            return ret;
        }

        last_rx_ts = fls64(ts_info.rx_ts_mask);
        last_tx_ts = fls64(ts_info.tx_ts_mask);

        if prev_last_rx_ts > last_rx_ts {
            dev_err(
                (*qmc_audio).dev,
                c"dai %d QMC chan %d unordered channels (RX timeslot %d before %d)\n".as_ptr(),
                (*qmc_dai).id,
                i,
                prev_last_rx_ts,
                last_rx_ts,
            );
            return -EINVAL;
        }
        if prev_last_tx_ts > last_tx_ts {
            dev_err(
                (*qmc_audio).dev,
                c"dai %d QMC chan %d unordered channels (TX timeslot %d before %d)\n".as_ptr(),
                (*qmc_dai).id,
                i,
                prev_last_tx_ts,
                last_tx_ts,
            );
            return -EINVAL;
        }

        prev_last_rx_ts = last_rx_ts;
        prev_last_tx_ts = last_tx_ts;
        i += 1;
    }

    (*qmc_dai).nb_chans_avail = count as c_uint;
    (*qmc_dai).nb_tx_ts = nb_tx_ts * count as c_uint;
    (*qmc_dai).nb_rx_ts = nb_rx_ts * count as c_uint;

    (*qmc_soc_dai_driver).id = (*qmc_dai).id;
    (*qmc_soc_dai_driver).name = (*qmc_dai).name;

    (*qmc_soc_dai_driver).playback.channels_min = 0;
    (*qmc_soc_dai_driver).playback.channels_max = 0;
    if nb_tx_ts != 0 {
        (*qmc_soc_dai_driver).playback.channels_min = 1;
        (*qmc_soc_dai_driver).playback.channels_max =
            if count > 1 { count as c_uint } else { nb_tx_ts };
    }
    (*qmc_soc_dai_driver).playback.formats = qmc_audio_formats(nb_tx_ts as u8, count > 1);

    (*qmc_soc_dai_driver).capture.channels_min = 0;
    (*qmc_soc_dai_driver).capture.channels_max = 0;
    if nb_rx_ts != 0 {
        (*qmc_soc_dai_driver).capture.channels_min = 1;
        (*qmc_soc_dai_driver).capture.channels_max =
            if count > 1 { count as c_uint } else { nb_rx_ts };
    }
    (*qmc_soc_dai_driver).capture.formats = qmc_audio_formats(nb_rx_ts as u8, count > 1);

    (*qmc_soc_dai_driver).playback.rates = snd_pcm_rate_to_rate_bit(tx_fs_rate);
    (*qmc_soc_dai_driver).playback.rate_min = tx_fs_rate as c_uint;
    (*qmc_soc_dai_driver).playback.rate_max = tx_fs_rate as c_uint;
    (*qmc_soc_dai_driver).capture.rates = snd_pcm_rate_to_rate_bit(rx_fs_rate);
    (*qmc_soc_dai_driver).capture.rate_min = rx_fs_rate as c_uint;
    (*qmc_soc_dai_driver).capture.rate_max = rx_fs_rate as c_uint;

    (*qmc_soc_dai_driver).ops = &qmc_dai_ops;

    0
}

unsafe extern "C" fn qmc_audio_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut qmc_audio: *mut qmc_audio;
    let mut i: c_uint;
    let mut ret: c_int;

    qmc_audio = devm_kzalloc(&mut (*pdev).dev, size_of::<qmc_audio>(), GFP_KERNEL) as *mut qmc_audio;
    if qmc_audio.is_null() {
        return -ENOMEM;
    }

    (*qmc_audio).dev = &mut (*pdev).dev;

    (*qmc_audio).num_dais = of_get_available_child_count(np);
    if (*qmc_audio).num_dais != 0 {
        (*qmc_audio).dais = devm_kcalloc(
            &mut (*pdev).dev,
            (*qmc_audio).num_dais as size_t,
            size_of::<qmc_dai>(),
            GFP_KERNEL,
        ) as *mut qmc_dai;
        if (*qmc_audio).dais.is_null() {
            return -ENOMEM;
        }

        (*qmc_audio).dai_drivers = devm_kcalloc(
            &mut (*pdev).dev,
            (*qmc_audio).num_dais as size_t,
            size_of::<snd_soc_dai_driver>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_driver;
        if (*qmc_audio).dai_drivers.is_null() {
            return -ENOMEM;
        }
    }

    i = 0;
    // Translation of for_each_available_child_of_node_scoped(np, child).
    let mut child = first_available_child_of_node(np);
    while !child.is_null() {
        ret = qmc_audio_dai_parse(
            qmc_audio,
            child,
            (*qmc_audio).dais.add(i as usize),
            (*qmc_audio).dai_drivers.add(i as usize),
        );
        if ret != 0 {
            return ret;
        }
        i += 1;
        child = next_available_child_of_node(np, child);
    }

    platform_set_drvdata(pdev, qmc_audio as *mut c_void);

    ret = devm_snd_soc_register_component(
        (*qmc_audio).dev,
        &qmc_audio_soc_platform,
        (*qmc_audio).dai_drivers,
        (*qmc_audio).num_dais as c_int,
    );
    if ret != 0 {
        return ret;
    }

    0
}

static qmc_audio_id_table: [of_device_id; 2] = [
    of_device_id {
        compatible: c"fsl,qmc-audio".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    }, /* sentinel */
];
// MODULE_DEVICE_TABLE(of, qmc_audio_id_table);

static mut qmc_audio_driver: platform_driver = platform_driver {
    driver: driver_inner {
        name: c"fsl-qmc-audio".as_ptr(),
        of_match_table: unsafe { of_match_ptr(qmc_audio_id_table.as_ptr()) },
    },
    probe: Some(qmc_audio_probe),
};
// module_platform_driver(qmc_audio_driver);

// MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
// MODULE_DESCRIPTION("CPM/QE QMC audio driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
