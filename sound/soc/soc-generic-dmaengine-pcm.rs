// SPDX-License-Identifier: GPL-2.0+
//
//  Copyright (C) 2013, Analog Devices Inc.
//	Author: Lars-Peter Clausen <lars@metafoo.de>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;

const NULL: *mut c_void = ptr::null_mut();
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EPROBE_DEFER: c_int = 517;
const UINT_MAX: c_uint = c_uint::MAX;
const SIZE_MAX: size_t = size_t::MAX;

const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_STREAM_LAST: usize = 1;

const SNDRV_PCM_INFO_MMAP: c_uint = 0x00000001;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0x00000002;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0x00000100;
const SNDRV_PCM_INFO_BATCH: c_uint = 0x00010000;
const SNDRV_DMA_TYPE_DEV_IRAM: c_int = 4;
const DMA_SLAVE_BUSWIDTH_8_BYTES: c_uint = 8;
const DMA_RESIDUE_GRANULARITY_DESCRIPTOR: c_int = 1;
const SND_SOC_COMP_ORDER_LATE: c_int = 10;

const SND_DMAENGINE_PCM_DRV_NAME: *const c_char = b"dmic-codec\0".as_ptr() as *const c_char;

/*
 * The platforms dmaengine driver does not support reporting the amount of
 * bytes that are still left to transfer.
 */
const SND_DMAENGINE_PCM_FLAG_NO_RESIDUE: c_uint = 1u32 << 31;

extern "C" {
    static mut prealloc_buffer_size_kbytes_param: c_uint;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_dmaengine_dai_dma_data;
    fn snd_hwparams_to_dma_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> c_int;
    fn snd_dmaengine_pcm_set_config_from_dai_data(
        substream: *mut snd_pcm_substream,
        dma_data: *mut snd_dmaengine_dai_dma_data,
        slave_config: *mut dma_slave_config,
    );
    fn snd_soc_component_to_priv(component: *mut snd_soc_component) -> *mut dmaengine_pcm;
    fn snd_dmaengine_pcm_get_chan(substream: *mut snd_pcm_substream) -> *mut dma_chan;
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dma_get_max_seg_size(dev: *mut device) -> size_t;
    fn snd_dmaengine_pcm_refine_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        dma_data: *mut snd_dmaengine_dai_dma_data,
        hw: *mut snd_pcm_hardware,
        chan: *mut dma_chan,
    ) -> c_int;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    ) -> c_int;
    fn snd_dmaengine_pcm_open(substream: *mut snd_pcm_substream, chan: *mut dma_chan) -> c_int;
    fn snd_dmaengine_pcm_close(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_dmaengine_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn snd_dmaengine_pcm_request_channel(
        filter_fn: Option<unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool_>,
        filter_data: *mut c_void,
    ) -> *mut dma_chan;
    fn dma_get_slave_caps(chan: *mut dma_chan, caps: *mut dma_slave_caps) -> c_int;
    fn dma_request_slave_channel(dev: *mut device, name: *const c_char) -> *mut dma_chan;
    fn snd_pcm_set_managed_buffer(
        substream: *mut snd_pcm_substream,
        ty: c_int,
        dev: *mut device,
        size: size_t,
        max: size_t,
    );
    fn strscpy_pad(dest: *mut c_char, src: *const c_char, count: size_t) -> ssize_t;
    fn snd_dmaengine_pcm_pointer_no_residue(
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;
    fn snd_dmaengine_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    fn copy_from_iter(addr: *mut c_void, bytes: c_ulong, iter: *mut iov_iter) -> c_ulong;
    fn copy_to_iter(addr: *mut c_void, bytes: c_ulong, iter: *mut iov_iter) -> c_ulong;
    fn snd_dmaengine_pcm_sync_stop(substream: *mut snd_pcm_substream) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dma_request_chan(dev: *mut device, name: *const c_char) -> *mut dma_chan;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn device_property_match_string(
        dev: *mut device,
        propname: *const c_char,
        string: *const c_char,
    ) -> c_int;
    fn dma_release_channel(chan: *mut dma_chan);
    fn snd_soc_component_alloc(dev: *mut device) -> *mut snd_soc_component;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_component_set_name(component: *mut snd_soc_component, name: *const c_char);
    fn snd_soc_component_set_priv(component: *mut snd_soc_component, priv_: *mut c_void);
    fn snd_soc_register_component(
        component: *mut snd_soc_component,
        driver: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_lookup_component(dev: *mut device, name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_unregister_component_by_driver(
        dev: *mut device,
        driver: *const snd_soc_component_driver,
    );
}

type ssize_t = isize;

#[repr(C)]
pub struct device {
    pub of_node: *mut c_void,
}

#[repr(C)]
pub struct dma_device {
    pub dev: *mut device,
}

#[repr(C)]
pub struct dma_chan {
    pub device: *mut dma_device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut c_void,
    pub dma_bytes: c_ulong,
    pub channels: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params;

#[repr(C)]
pub struct dma_slave_config;

#[repr(C)]
pub struct snd_soc_dai;

#[repr(C)]
pub struct snd_soc_dai_link {
    pub num_cpus: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
    pub dev: *mut device,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_str {
    pub substream: *mut snd_pcm_substream,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_str; 2],
    pub name: [c_char; 80],
    pub id: [c_char; 64],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: c_ulong,
    pub addr_width: c_uint,
    pub maxburst: c_uint,
    pub filter_data: *mut c_void,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct dma_slave_caps {
    pub src_addr_widths: u32,
    pub dstn_addr_widths: u32,
    pub directions: u32,
    pub cmd_pause: bool_,
    pub cmd_resume: bool_,
    pub cmd_terminate: bool_,
    pub residue_granularity: c_int,
}

#[repr(C)]
pub struct iov_iter;

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub driver: *const snd_soc_component_driver,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe_order: c_int,
    pub open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t,
    >,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            c_int,
            c_ulong,
            *mut iov_iter,
            c_ulong,
        ) -> c_int,
    >,
    pub pcm_new:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub sync_stop:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub debugfs_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut dma_slave_config,
        ) -> c_int,
    >,
    pub pcm_hardware: *const snd_pcm_hardware,
    pub compat_request_channel: Option<
        unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_substream) -> *mut dma_chan,
    >,
    pub compat_filter_fn: Option<unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool_>,
    pub process: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int,
    >,
    pub prealloc_buffer_size: size_t,
    pub chan_names: [*const c_char; 2],
    pub dma_dev: *mut device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct dmaengine_pcm {
    pub config: *const snd_dmaengine_pcm_config,
    pub flags: c_uint,
    pub chan: [*mut dma_chan; 2],
}

const SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX: c_uint = 1 << 0;
const SND_DMAENGINE_PCM_FLAG_COMPAT: c_uint = 1 << 1;
const SND_DMAENGINE_PCM_FLAG_NO_DT: c_uint = 1 << 2;

static mut prealloc_buffer_size_kbytes: c_uint = 512;
// module_param(prealloc_buffer_size_kbytes, uint, 0444);
// MODULE_PARM_DESC(prealloc_buffer_size_kbytes, "Preallocate DMA buffer size (KB).");

unsafe extern "C" fn dmaengine_dma_dev(
    pcm: *mut dmaengine_pcm,
    substream: *mut snd_pcm_substream,
) -> *mut device {
    if (*pcm).chan[(*substream).stream as usize].is_null() {
        return ptr::null_mut();
    }

    (*(*(*pcm).chan[(*substream).stream as usize]).device).dev
}

/**
 * snd_dmaengine_pcm_prepare_slave_config() - Generic prepare_slave_config callback
 * @substream: PCM substream
 * @params: hw_params
 * @slave_config: DMA slave config to prepare
 *
 * This function can be used as a generic prepare_slave_config callback for
 * platforms which make use of the snd_dmaengine_dai_dma_data struct for their
 * DAI DMA data. Internally the function will first call
 * snd_hwparams_to_dma_slave_config to fill in the slave config based on the
 * hw_params, followed by snd_dmaengine_pcm_set_config_from_dai_data to fill in
 * the remaining fields based on the DAI DMA data.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dmaengine_pcm_prepare_slave_config(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    slave_config: *mut dma_slave_config,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let ret: c_int;

    if (*(*rtd).dai_link).num_cpus > 1 {
        dev_err(
            (*rtd).dev,
            b"%s doesn't support Multi CPU yet\n\0".as_ptr() as *const c_char,
            b"snd_dmaengine_pcm_prepare_slave_config\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);

    ret = snd_hwparams_to_dma_slave_config(substream, params, slave_config);
    if ret != 0 {
        return ret;
    }

    snd_dmaengine_pcm_set_config_from_dai_data(substream, dma_data, slave_config);

    0
}
// EXPORT_SYMBOL_GPL(snd_dmaengine_pcm_prepare_slave_config);

unsafe extern "C" fn dmaengine_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let pcm = snd_soc_component_to_priv(component);
    let chan = snd_dmaengine_pcm_get_chan(substream);
    let mut slave_config: dma_slave_config = core::mem::zeroed();
    let ret: c_int;

    if (*(*pcm).config).prepare_slave_config.is_none() {
        return 0;
    }

    ret = ((*(*pcm).config).prepare_slave_config.unwrap())(substream, params, &mut slave_config);
    if ret != 0 {
        return ret;
    }

    dmaengine_slave_config(chan, &mut slave_config)
}

unsafe extern "C" fn dmaengine_pcm_set_runtime_hwparams(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let pcm = snd_soc_component_to_priv(component);
    let dma_dev = dmaengine_dma_dev(pcm, substream);
    let chan = (*pcm).chan[(*substream).stream as usize];
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let mut hw: snd_pcm_hardware = core::mem::zeroed();

    if (*(*rtd).dai_link).num_cpus > 1 {
        dev_err(
            (*rtd).dev,
            b"%s doesn't support Multi CPU yet\n\0".as_ptr() as *const c_char,
            b"dmaengine_pcm_set_runtime_hwparams\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    if !(*(*pcm).config).pcm_hardware.is_null() {
        return snd_soc_set_runtime_hwparams(substream, (*(*pcm).config).pcm_hardware);
    }

    dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);

    hw.info = SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED;
    hw.periods_min = 2;
    hw.periods_max = UINT_MAX;
    hw.period_bytes_min = ((*dma_data).maxburst * DMA_SLAVE_BUSWIDTH_8_BYTES) as size_t;
    if hw.period_bytes_min == 0 {
        hw.period_bytes_min = 256;
    }
    hw.period_bytes_max = dma_get_max_seg_size(dma_dev);
    hw.buffer_bytes_max = SIZE_MAX;
    hw.fifo_size = (*dma_data).fifo_size;

    if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_NO_RESIDUE) != 0 {
        hw.info |= SNDRV_PCM_INFO_BATCH;
    }

    /**
     * FIXME: Remove the return value check to align with the code
     * before adding snd_dmaengine_pcm_refine_runtime_hwparams
     * function.
     */
    snd_dmaengine_pcm_refine_runtime_hwparams(substream, dma_data, &mut hw, chan);

    snd_soc_set_runtime_hwparams(substream, &hw)
}

unsafe extern "C" fn dmaengine_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let pcm = snd_soc_component_to_priv(component);
    let chan = (*pcm).chan[(*substream).stream as usize];
    let ret: c_int;

    ret = dmaengine_pcm_set_runtime_hwparams(component, substream);
    if ret != 0 {
        return ret;
    }

    snd_dmaengine_pcm_open(substream, chan)
}

unsafe extern "C" fn dmaengine_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    snd_dmaengine_pcm_close(substream)
}

unsafe extern "C" fn dmaengine_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    snd_dmaengine_pcm_trigger(substream, cmd)
}

unsafe extern "C" fn dmaengine_pcm_compat_request_channel(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
    substream: *mut snd_pcm_substream,
) -> *mut dma_chan {
    let pcm = snd_soc_component_to_priv(component);
    let dma_data: *mut snd_dmaengine_dai_dma_data;

    if (*(*rtd).dai_link).num_cpus > 1 {
        dev_err(
            (*rtd).dev,
            b"%s doesn't support Multi CPU yet\n\0".as_ptr() as *const c_char,
            b"dmaengine_pcm_compat_request_channel\0".as_ptr() as *const c_char,
        );
        return ptr::null_mut();
    }

    dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);

    if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX) != 0 && !(*pcm).chan[0].is_null() {
        return (*pcm).chan[0];
    }

    if let Some(compat_request_channel) = (*(*pcm).config).compat_request_channel {
        return compat_request_channel(rtd, substream);
    }

    snd_dmaengine_pcm_request_channel((*(*pcm).config).compat_filter_fn, (*dma_data).filter_data)
}

unsafe extern "C" fn dmaengine_pcm_can_report_residue(
    dev: *mut device,
    chan: *mut dma_chan,
) -> bool_ {
    let mut dma_caps: dma_slave_caps = core::mem::zeroed();
    let ret: c_int;

    ret = dma_get_slave_caps(chan, &mut dma_caps);
    if ret != 0 {
        dev_warn(
            dev,
            b"Failed to get DMA channel capabilities, falling back to period counting: %d\n\0"
                .as_ptr() as *const c_char,
            ret,
        );
        return false;
    }

    if dma_caps.residue_granularity == DMA_RESIDUE_GRANULARITY_DESCRIPTOR {
        return false;
    }

    true
}

unsafe extern "C" fn dmaengine_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let pcm = snd_soc_component_to_priv(component);
    let config = (*pcm).config;
    let dev = (*component).dev;
    let prealloc_buffer_size: size_t;
    let max_buffer_size: size_t;
    let mut i: c_uint;

    if (*config).prealloc_buffer_size != 0 {
        prealloc_buffer_size = (*config).prealloc_buffer_size;
    } else {
        prealloc_buffer_size = (prealloc_buffer_size_kbytes as size_t) * 1024;
    }

    if !(*config).pcm_hardware.is_null() && (*(*config).pcm_hardware).buffer_bytes_max != 0 {
        max_buffer_size = (*(*config).pcm_hardware).buffer_bytes_max;
    } else {
        max_buffer_size = SIZE_MAX;
    }

    i = 0;
    while i <= SNDRV_PCM_STREAM_LAST as c_uint {
        let substream = (*(*rtd).pcm).streams[i as usize].substream;
        if substream.is_null() {
            i += 1;
            continue;
        }

        if (*pcm).chan[i as usize].is_null() && !(*config).chan_names[i as usize].is_null() {
            (*pcm).chan[i as usize] = dma_request_slave_channel(dev, (*config).chan_names[i as usize]);
        }

        if (*pcm).chan[i as usize].is_null()
            && ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_COMPAT) != 0
        {
            (*pcm).chan[i as usize] =
                dmaengine_pcm_compat_request_channel(component, rtd, substream);
        }

        if (*pcm).chan[i as usize].is_null() {
            dev_err(
                (*component).dev,
                b"Missing dma channel for stream: %d\n\0".as_ptr() as *const c_char,
                i,
            );
            return -EINVAL;
        }

        snd_pcm_set_managed_buffer(
            substream,
            SNDRV_DMA_TYPE_DEV_IRAM,
            dmaengine_dma_dev(pcm, substream),
            prealloc_buffer_size,
            max_buffer_size,
        );

        if !dmaengine_pcm_can_report_residue(dev, (*pcm).chan[i as usize]) {
            (*pcm).flags |= SND_DMAENGINE_PCM_FLAG_NO_RESIDUE;
        }

        if (*(*(*rtd).pcm).streams[i as usize].pcm).name[0] == 0 {
            strscpy_pad(
                (*(*(*rtd).pcm).streams[i as usize].pcm).name.as_mut_ptr(),
                (*(*(*rtd).pcm).streams[i as usize].pcm).id.as_ptr(),
                size_of::<[c_char; 80]>(),
            );
        }

        i += 1;
    }

    0
}

unsafe extern "C" fn dmaengine_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let pcm = snd_soc_component_to_priv(component);

    if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_NO_RESIDUE) != 0 {
        snd_dmaengine_pcm_pointer_no_residue(substream)
    } else {
        snd_dmaengine_pcm_pointer(substream)
    }
}

unsafe extern "C" fn dmaengine_copy(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    channel: c_int,
    hwoff: c_ulong,
    iter: *mut iov_iter,
    bytes: c_ulong,
) -> c_int {
    let runtime = (*substream).runtime;
    let pcm = snd_soc_component_to_priv(component);
    let process = (*(*pcm).config).process;
    let is_playback = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int;
    let dma_ptr = ((*runtime).dma_area as *mut u8)
        .add(hwoff as usize)
        .add(channel as usize * ((*runtime).dma_bytes / (*runtime).channels as c_ulong) as usize)
        as *mut c_void;

    if is_playback {
        if copy_from_iter(dma_ptr, bytes, iter) != bytes {
            return -EFAULT;
        }
    }

    if let Some(process_fn) = process {
        let ret = process_fn(substream, channel, hwoff, bytes);
        if ret < 0 {
            return ret;
        }
    }

    if !is_playback {
        if copy_to_iter(dma_ptr, bytes, iter) != bytes {
            return -EFAULT;
        }
    }

    0
}

unsafe extern "C" fn dmaengine_pcm_sync_stop(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    snd_dmaengine_pcm_sync_stop(substream)
}

static dmaengine_pcm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: SND_DMAENGINE_PCM_DRV_NAME,
    probe_order: SND_SOC_COMP_ORDER_LATE,
    open: Some(dmaengine_pcm_open),
    close: Some(dmaengine_pcm_close),
    hw_params: Some(dmaengine_pcm_hw_params),
    trigger: Some(dmaengine_pcm_trigger),
    pointer: Some(dmaengine_pcm_pointer),
    copy: None,
    pcm_new: Some(dmaengine_pcm_new),
    sync_stop: Some(dmaengine_pcm_sync_stop),
    debugfs_prefix: b"dma\0".as_ptr() as *const c_char,
};

static dmaengine_pcm_component_process: snd_soc_component_driver = snd_soc_component_driver {
    name: SND_DMAENGINE_PCM_DRV_NAME,
    probe_order: SND_SOC_COMP_ORDER_LATE,
    open: Some(dmaengine_pcm_open),
    close: Some(dmaengine_pcm_close),
    hw_params: Some(dmaengine_pcm_hw_params),
    trigger: Some(dmaengine_pcm_trigger),
    pointer: Some(dmaengine_pcm_pointer),
    copy: Some(dmaengine_copy),
    pcm_new: Some(dmaengine_pcm_new),
    sync_stop: Some(dmaengine_pcm_sync_stop),
    debugfs_prefix: b"dma\0".as_ptr() as *const c_char,
};

static dmaengine_pcm_dma_channel_names: [*const c_char; 2] = [
    b"tx\0".as_ptr() as *const c_char,
    b"rx\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn dmaengine_pcm_request_chan_of(
    pcm: *mut dmaengine_pcm,
    mut dev: *mut device,
    config: *const snd_dmaengine_pcm_config,
) -> c_int {
    let mut i: c_uint;
    let mut name: *const c_char;
    let chan: *mut dma_chan;

    if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_NO_DT) != 0
        || ((*dev).of_node.is_null()
            && ((*config).dma_dev.is_null() || (*(*config).dma_dev).of_node.is_null()))
    {
        return 0;
    }

    if !(*config).dma_dev.is_null() {
        /*
         * If this warning is seen, it probably means that your Linux
         * device structure does not match your HW device structure.
         * It would be best to refactor the Linux device structure to
         * correctly match the HW structure.
         */
        dev_warn(
            dev,
            b"DMA channels sourced from device %s\0".as_ptr() as *const c_char,
            dev_name((*config).dma_dev),
        );
        dev = (*config).dma_dev;
    }

    i = 0;
    while i <= SNDRV_PCM_STREAM_LAST as c_uint {
        if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX) != 0 {
            name = b"rx-tx\0".as_ptr() as *const c_char;
        } else {
            name = dmaengine_pcm_dma_channel_names[i as usize];
        }
        if !(*config).chan_names[i as usize].is_null() {
            name = (*config).chan_names[i as usize];
        }
        let requested = dma_request_chan(dev, name);
        if IS_ERR(requested as *const c_void) {
            /*
             * Only report probe deferral errors, channels
             * might not be present for devices that
             * support only TX or only RX.
             */
            if PTR_ERR(requested as *const c_void) == -(EPROBE_DEFER as c_long) {
                return -EPROBE_DEFER;
            }

            if device_property_match_string(
                dev,
                b"dma-names\0".as_ptr() as *const c_char,
                name,
            ) >= 0
            {
                dev_warn(
                    dev,
                    b"dma-names has '%s' but request failed (%ld)\n\0".as_ptr() as *const c_char,
                    name,
                    PTR_ERR(requested as *const c_void),
                );
            }

            (*pcm).chan[i as usize] = ptr::null_mut();
        } else {
            chan = requested;
            (*pcm).chan[i as usize] = chan;
        }
        if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX) != 0 {
            break;
        }
        i += 1;
    }

    if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX) != 0 {
        (*pcm).chan[1] = (*pcm).chan[0];
    }

    if (*pcm).chan[0].is_null() && (*pcm).chan[1].is_null() {
        dev_err(
            dev,
            b"no DMA channel found for either playback or capture\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn dmaengine_pcm_release_chan(pcm: *mut dmaengine_pcm) {
    let mut i: c_uint = 0;

    while i <= SNDRV_PCM_STREAM_LAST as c_uint {
        if (*pcm).chan[i as usize].is_null() {
            i += 1;
            continue;
        }
        dma_release_channel((*pcm).chan[i as usize]);
        if ((*pcm).flags & SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX) != 0 {
            break;
        }
        i += 1;
    }
}

static snd_dmaengine_pcm_default_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    pcm_hardware: ptr::null(),
    compat_request_channel: None,
    compat_filter_fn: None,
    process: None,
    prealloc_buffer_size: 0,
    chan_names: [ptr::null(), ptr::null()],
    dma_dev: ptr::null_mut(),
    name: ptr::null(),
};

/**
 * snd_dmaengine_pcm_register - Register a dmaengine based PCM device
 * @dev: The parent device for the PCM device
 * @config: Platform specific PCM configuration
 * @flags: Platform specific quirks
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dmaengine_pcm_register(
    dev: *mut device,
    mut config: *const snd_dmaengine_pcm_config,
    flags: c_uint,
) -> c_int {
    let component: *mut snd_soc_component;
    let driver: *const snd_soc_component_driver;
    let pcm: *mut dmaengine_pcm;
    let mut ret: c_int;

    component = snd_soc_component_alloc(dev);
    if component.is_null() {
        return -ENOMEM;
    }

    pcm = kzalloc(size_of::<dmaengine_pcm>(), 0) as *mut dmaengine_pcm;
    if pcm.is_null() {
        return -ENOMEM;
    }

    if config.is_null() {
        config = &snd_dmaengine_pcm_default_config;
    }
    (*pcm).config = config;
    (*pcm).flags = flags;

    if !(*config).name.is_null() {
        snd_soc_component_set_name(component, (*config).name);
    }
    snd_soc_component_set_priv(component, pcm as *mut c_void);

    ret = dmaengine_pcm_request_chan_of(pcm, dev, config);
    if ret != 0 {
        dmaengine_pcm_release_chan(pcm);
        kfree(pcm as *mut c_void);
        return ret;
    }

    if (*config).process.is_some() {
        driver = &dmaengine_pcm_component_process;
    } else {
        driver = &dmaengine_pcm_component;
    }

    ret = snd_soc_register_component(component, driver, ptr::null_mut(), 0);
    if ret != 0 {
        dmaengine_pcm_release_chan(pcm);
        kfree(pcm as *mut c_void);
        return ret;
    }

    0
}
// EXPORT_SYMBOL_GPL(snd_dmaengine_pcm_register);

/**
 * snd_dmaengine_pcm_unregister - Removes a dmaengine based PCM device
 * @dev: Parent device the PCM was register with
 *
 * Removes a dmaengine based PCM device previously registered with
 * snd_dmaengine_pcm_register.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dmaengine_pcm_unregister(dev: *mut device) {
    let component: *mut snd_soc_component;
    let pcm: *mut dmaengine_pcm;

    component = snd_soc_lookup_component(dev, SND_DMAENGINE_PCM_DRV_NAME);
    if component.is_null() {
        return;
    }

    pcm = snd_soc_component_to_priv(component);

    snd_soc_unregister_component_by_driver(dev, (*component).driver);
    dmaengine_pcm_release_chan(pcm);
    kfree(pcm as *mut c_void);
}
// EXPORT_SYMBOL_GPL(snd_dmaengine_pcm_unregister);

// MODULE_DESCRIPTION("ASoC helpers for generic PCM dmaengine API");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
