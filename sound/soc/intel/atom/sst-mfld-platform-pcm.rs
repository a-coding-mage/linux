// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_mfld_platform.c - Intel MID Platform driver
 *
 *  Copyright (C) 2010-2014 Intel Corp
 *  Author: Vinod Koul <vinod.koul@intel.com>
 *  Author: Harsha Priya <priya.harsha@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C includes removed; required kernel, ALSA, ASoC, SST, and atom-control symbols
// are expected to be supplied by surrounding bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static mut sst_platform_compress_ops: snd_compr_ops;

    fn WARN_ON(condition: bool) -> bool;
    fn try_module_get(owner: *mut module) -> bool;
    fn module_put(owner: *mut module);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn samples_to_bytes(runtime: *mut snd_pcm_runtime, size: snd_pcm_uframes_t) -> ssize_t;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> ssize_t;
    fn sst_send_pipe_gains(dai: *mut snd_soc_dai, stream: c_int, mute: c_int) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn sst_handle_vb_timer(dai: *mut snd_soc_dai, enable: bool) -> c_int;
    fn sst_fill_ssp_defaults(dai: *mut snd_soc_dai);
    fn send_ssp_cmd(dai: *mut snd_soc_dai, name: *const c_char, enable: c_int) -> c_int;
    fn sst_fill_ssp_config(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn sst_fill_ssp_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_new_compress(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        min: usize,
        max: usize,
    );
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn sst_dsp_init_v2_dpcm(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_suspend(dev: *mut device) -> c_int;
    fn snd_soc_poweroff(dev: *mut device) -> c_int;
    fn snd_soc_resume(dev: *mut device) -> c_int;
}

type ssize_t = isize;
type snd_pcm_uframes_t = usize;
type snd_pcm_sframes_t = isize;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_driver {
    pub owner: *mut module,
}
#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct snd_compr_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_pcm {
    pub internal: c_int,
    pub id: [c_char; 64],
    pub card: *mut snd_card,
    pub device: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: usize,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub private_data: *mut c_void,
    pub dma_addr: u32,
    pub channels: c_uint,
    pub period_size: snd_pcm_uframes_t,
    pub sample_bits: c_uint,
    pub rate: c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
    pub number: c_int,
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_compr_device {
    pub device: c_int,
}
#[repr(C)]
pub struct snd_compr_stream {
    pub device: *mut snd_compr_device,
    pub direction: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub compress_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub ops: *const snd_soc_dai_ops,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub name: *const c_char,
    pub driver: *mut snd_soc_dai_driver,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub pcm: *mut snd_pcm,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
    pub compress_ops: *const snd_compr_ops,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}
#[repr(C)]
pub struct dev_pm_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub complete: Option<unsafe extern "C" fn(*mut device)>,
}
#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct sst_device {
    pub dev: *mut device,
    pub name: *const c_char,
    pub ops: *mut sst_ops,
}
#[repr(C)]
pub struct sst_ops {
    pub open: Option<unsafe extern "C" fn(*mut device, *mut snd_sst_params) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub power: Option<unsafe extern "C" fn(*mut device, bool) -> c_int>,
    pub stream_init: Option<unsafe extern "C" fn(*mut device, *mut pcm_stream_info) -> c_int>,
    pub stream_drop: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_start: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_pause: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_pause_release: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub stream_read_tstamp: Option<unsafe extern "C" fn(*mut device, *mut pcm_stream_info) -> c_int>,
}
#[repr(C)]
pub struct pcm_stream_info {
    pub str_id: c_int,
    pub period_elapsed: Option<unsafe extern "C" fn(*mut c_void)>,
    pub arg: *mut c_void,
    pub buffer_ptr: snd_pcm_uframes_t,
    pub sfreq: c_uint,
    pub pcm_delay: snd_pcm_sframes_t,
}
#[repr(C)]
pub struct sst_runtime_stream {
    pub status_lock: spinlock_t,
    pub stream_status: c_int,
    pub stream_info: pcm_stream_info,
    pub ops: *mut sst_ops,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_dev_stream_map {
    pub dev_num: c_int,
    pub subdev_num: c_int,
    pub direction: c_int,
    pub device_id: c_int,
    pub task_id: c_int,
    pub status: c_int,
}
#[repr(C)]
pub struct sst_platform_data {
    pub pdev_strm_map: *mut sst_dev_stream_map,
    pub strm_map_size: c_int,
}
#[repr(C)]
pub struct sst_data {
    pub pdata: *mut sst_platform_data,
    pub pdev: *mut platform_device,
    pub lock: mutex,
    pub soc_card: *mut snd_soc_card,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_ring_buf_info {
    pub addr: u32,
    pub size: ssize_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_params_ext {
    pub ring_buf_info: [snd_sst_ring_buf_info; 1],
    pub sg_count: c_int,
    pub reserved: c_int,
    pub frag_size: ssize_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_pcm_params {
    pub num_chan: u8,
    pub pcm_wd_sz: c_uint,
    pub sfreq: c_uint,
    pub use_offload_path: u8,
    pub reserved2: u8,
    pub channel_map: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_sst_stream_params_union {
    pub pcm_params: snd_sst_pcm_params,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_stream_params {
    pub uc: snd_sst_stream_params_union,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_params {
    pub stream_type: c_int,
    pub stream_id: c_int,
    pub device_type: c_int,
    pub task: c_int,
    pub ops: u8,
    pub sparams: snd_sst_stream_params,
    pub aparams: snd_sst_alloc_params_ext,
    pub codec: c_int,
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EEXIST: c_int = 17;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SST_MAX_BUFFER: usize = 0;
const SST_MIN_BUFFER: usize = 0;
const SST_MIN_PERIOD_BYTES: usize = 0;
const SST_MAX_PERIOD_BYTES: usize = 0;
const SST_MIN_PERIODS: c_uint = 0;
const SST_MAX_PERIODS: c_uint = 0;
const SST_FIFO_SIZE: usize = 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_DOUBLE: c_uint = 1 << 1;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 2;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 3;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 4;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 5;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 6;
const SNDRV_PCM_INFO_SYNC_START: c_uint = 1 << 7;
const MERR_DPCM_AUDIO: c_int = 0;
const MERR_DPCM_COMPR: c_int = 0;
const MERR_DPCM_DEEP_BUFFER: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const PIPE_MEDIA1_IN: c_int = 0;
const PIPE_MEDIA0_IN: c_int = 0;
const PIPE_PCM1_OUT: c_int = 0;
const PIPE_MEDIA3_IN: c_int = 0;
const SST_TASK_ID_MEDIA: c_int = 0;
const SST_STREAM_TYPE_MUSIC: c_int = 0;
const SST_CODEC_TYPE_PCM: c_int = 0;
const SST_PLATFORM_RUNNING: c_int = 0;
const SST_PLATFORM_INIT: c_int = 1;
const SST_PLATFORM_DROPPED: c_int = 2;
const SST_PLATFORM_PAUSED: c_int = 3;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_RATE_8000: c_uint = 0;
const SNDRV_PCM_RATE_16000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SST_STEREO: c_uint = 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 5;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const DRV_NAME: *const c_char = b"sst-mfld-platform\0".as_ptr() as *const c_char;

static mut sst: *mut sst_device = ptr::null_mut();
static mut sst_lock: mutex = mutex { _private: [] };

#[no_mangle]
pub unsafe extern "C" fn sst_register_dsp(dev: *mut sst_device) -> c_int {
    if WARN_ON(dev.is_null()) {
        return -EINVAL;
    }
    if !try_module_get((*(*(*dev).dev).driver).owner) {
        return -ENODEV;
    }
    mutex_lock(&raw mut sst_lock);
    if !sst.is_null() {
        dev_err((*dev).dev, b"we already have a device %s\n\0".as_ptr() as *const c_char, (*sst).name);
        module_put((*(*(*dev).dev).driver).owner);
        mutex_unlock(&raw mut sst_lock);
        return -EEXIST;
    }
    dev_dbg((*dev).dev, b"registering device %s\n\0".as_ptr() as *const c_char, (*dev).name);
    sst = dev;
    mutex_unlock(&raw mut sst_lock);
    0
}
// EXPORT_SYMBOL_GPL(sst_register_dsp);

#[no_mangle]
pub unsafe extern "C" fn sst_unregister_dsp(dev: *mut sst_device) -> c_int {
    if WARN_ON(dev.is_null()) {
        return -EINVAL;
    }
    if dev != sst {
        return -EINVAL;
    }
    mutex_lock(&raw mut sst_lock);
    if sst.is_null() {
        mutex_unlock(&raw mut sst_lock);
        return -EIO;
    }
    module_put((*(*(*sst).dev).driver).owner);
    dev_dbg((*dev).dev, b"unreg %s\n\0".as_ptr() as *const c_char, (*sst).name);
    sst = ptr::null_mut();
    mutex_unlock(&raw mut sst_lock);
    0
}
// EXPORT_SYMBOL_GPL(sst_unregister_dsp);

static sst_platform_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_DOUBLE
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_SYNC_START,
    buffer_bytes_max: SST_MAX_BUFFER,
    period_bytes_min: SST_MIN_PERIOD_BYTES,
    period_bytes_max: SST_MAX_PERIOD_BYTES,
    periods_min: SST_MIN_PERIODS,
    periods_max: SST_MAX_PERIODS,
    fifo_size: SST_FIFO_SIZE,
};

static mut dpcm_strm_map: [sst_dev_stream_map; 5] = [
    sst_dev_stream_map { dev_num: 0xFF, subdev_num: 0xFF, direction: 0xFF, device_id: 0xFF, task_id: 0xFF, status: 0xFF }, /* Reserved, not in use */
    sst_dev_stream_map { dev_num: MERR_DPCM_AUDIO, subdev_num: 0, direction: SNDRV_PCM_STREAM_PLAYBACK, device_id: PIPE_MEDIA1_IN, task_id: SST_TASK_ID_MEDIA, status: 0 },
    sst_dev_stream_map { dev_num: MERR_DPCM_COMPR, subdev_num: 0, direction: SNDRV_PCM_STREAM_PLAYBACK, device_id: PIPE_MEDIA0_IN, task_id: SST_TASK_ID_MEDIA, status: 0 },
    sst_dev_stream_map { dev_num: MERR_DPCM_AUDIO, subdev_num: 0, direction: SNDRV_PCM_STREAM_CAPTURE, device_id: PIPE_PCM1_OUT, task_id: SST_TASK_ID_MEDIA, status: 0 },
    sst_dev_stream_map { dev_num: MERR_DPCM_DEEP_BUFFER, subdev_num: 0, direction: SNDRV_PCM_STREAM_PLAYBACK, device_id: PIPE_MEDIA3_IN, task_id: SST_TASK_ID_MEDIA, status: 0 },
];

unsafe extern "C" fn sst_media_digital_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    sst_send_pipe_gains(dai, stream, mute)
}

/* helper functions */
#[no_mangle]
pub unsafe extern "C" fn sst_set_stream_status(stream: *mut sst_runtime_stream, state: c_int) {
    (*stream).stream_status = state;
}

unsafe fn sst_get_stream_status(stream: *mut sst_runtime_stream) -> c_int {
    (*stream).stream_status
}

unsafe fn sst_fill_alloc_params(substream: *mut snd_pcm_substream, alloc_param: *mut snd_sst_alloc_params_ext) {
    let channels: c_uint;
    let period_size: snd_pcm_uframes_t;
    let periodbytes: ssize_t;
    let buffer_bytes: ssize_t = snd_pcm_lib_buffer_bytes(substream);
    let buffer_addr: u32 = (*(*substream).runtime).dma_addr;

    channels = (*(*substream).runtime).channels;
    period_size = (*(*substream).runtime).period_size;
    periodbytes = samples_to_bytes((*substream).runtime, period_size);
    (*alloc_param).ring_buf_info[0].addr = buffer_addr;
    (*alloc_param).ring_buf_info[0].size = buffer_bytes;
    (*alloc_param).sg_count = 1;
    (*alloc_param).reserved = 0;
    (*alloc_param).frag_size = periodbytes * channels as ssize_t;
}

unsafe fn sst_fill_pcm_params(substream: *mut snd_pcm_substream, param: *mut snd_sst_stream_params) {
    (*param).uc.pcm_params.num_chan = (*(*substream).runtime).channels as u8;
    (*param).uc.pcm_params.pcm_wd_sz = (*(*substream).runtime).sample_bits;
    (*param).uc.pcm_params.sfreq = (*(*substream).runtime).rate;

    /* PCM stream via ALSA interface */
    (*param).uc.pcm_params.use_offload_path = 0;
    (*param).uc.pcm_params.reserved2 = 0;
    memset(
        (*param).uc.pcm_params.channel_map.as_mut_ptr() as *mut c_void,
        0,
        size_of::<u8>(),
    );
}

unsafe fn sst_get_stream_mapping(
    dev: c_int,
    _sdev: c_int,
    dir: c_int,
    map: *mut sst_dev_stream_map,
    size: c_int,
) -> c_int {
    let mut i: c_int;

    if map.is_null() {
        return -EINVAL;
    }

    /* index 0 is not used in stream map */
    i = 1;
    while i < size {
        if (*map.offset(i as isize)).dev_num == dev && (*map.offset(i as isize)).direction == dir {
            return i;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sst_fill_stream_params(
    substream: *mut c_void,
    ctx: *const sst_data,
    str_params: *mut snd_sst_params,
    is_compress: bool,
) -> c_int {
    let map_size: c_int;
    let mut index: c_int;
    let map: *mut sst_dev_stream_map;
    let mut pstream: *mut snd_pcm_substream = ptr::null_mut();
    let mut cstream: *mut snd_compr_stream = ptr::null_mut();

    map = (*(*ctx).pdata).pdev_strm_map;
    map_size = (*(*ctx).pdata).strm_map_size;

    if is_compress {
        cstream = substream as *mut snd_compr_stream;
    } else {
        pstream = substream as *mut snd_pcm_substream;
    }

    (*str_params).stream_type = SST_STREAM_TYPE_MUSIC;

    /* For pcm streams */
    if !pstream.is_null() {
        index = sst_get_stream_mapping(
            (*(*pstream).pcm).device,
            (*pstream).number,
            (*pstream).stream,
            map,
            map_size,
        );
        if index <= 0 {
            return -EINVAL;
        }

        (*str_params).stream_id = index;
        (*str_params).device_type = (*map.offset(index as isize)).device_id;
        (*str_params).task = (*map.offset(index as isize)).task_id;
        (*str_params).ops = (*pstream).stream as u8;
    }

    if !cstream.is_null() {
        index = sst_get_stream_mapping((*(*cstream).device).device, 0, (*cstream).direction, map, map_size);
        if index <= 0 {
            return -EINVAL;
        }
        (*str_params).stream_id = index;
        (*str_params).device_type = (*map.offset(index as isize)).device_id;
        (*str_params).task = (*map.offset(index as isize)).task_id;
        (*str_params).ops = (*cstream).direction as u8;
    }
    0
}

unsafe fn sst_platform_alloc_stream(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    let mut param: snd_sst_stream_params = core::mem::zeroed();
    let mut str_params: snd_sst_params = core::mem::zeroed();
    let mut alloc_params: snd_sst_alloc_params_ext = core::mem::zeroed();
    let mut ret_val: c_int;
    let ctx = snd_soc_dai_get_drvdata(dai) as *mut sst_data;

    /* set codec params and inform SST driver the same */
    sst_fill_pcm_params(substream, &mut param);
    sst_fill_alloc_params(substream, &mut alloc_params);
    str_params.sparams = param;
    str_params.aparams = alloc_params;
    str_params.codec = SST_CODEC_TYPE_PCM;

    /* fill the device type and stream id to pass to SST driver */
    ret_val = sst_fill_stream_params(substream as *mut c_void, ctx, &mut str_params, false);
    if ret_val < 0 {
        return ret_val;
    }

    (*stream).stream_info.str_id = str_params.stream_id;
    ((*(*stream).ops).open.unwrap())((*sst).dev, &mut str_params)
}

unsafe extern "C" fn sst_period_elapsed(arg: *mut c_void) {
    let substream = arg as *mut snd_pcm_substream;
    let stream: *mut sst_runtime_stream;
    let status: c_int;

    if substream.is_null() || (*substream).runtime.is_null() {
        return;
    }
    stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    if stream.is_null() {
        return;
    }
    status = sst_get_stream_status(stream);
    if status != SST_PLATFORM_RUNNING {
        return;
    }
    snd_pcm_period_elapsed(substream);
}

unsafe fn sst_platform_init_stream(substream: *mut snd_pcm_substream) -> c_int {
    let stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    let rtd = snd_soc_substream_to_rtd(substream);
    let ret_val: c_int;

    dev_dbg((*rtd).dev, b"setting buffer ptr param\n\0".as_ptr() as *const c_char);
    sst_set_stream_status(stream, SST_PLATFORM_INIT);
    (*stream).stream_info.period_elapsed = Some(sst_period_elapsed);
    (*stream).stream_info.arg = substream as *mut c_void;
    (*stream).stream_info.buffer_ptr = 0;
    (*stream).stream_info.sfreq = (*(*substream).runtime).rate;
    ret_val = ((*(*stream).ops).stream_init.unwrap())((*sst).dev, &mut (*stream).stream_info);
    if ret_val != 0 {
        dev_err((*rtd).dev, b"control_set ret error %d\n\0".as_ptr() as *const c_char, ret_val);
    }
    ret_val
}

unsafe fn power_up_sst(stream: *mut sst_runtime_stream) -> c_int {
    ((*(*stream).ops).power.unwrap())((*sst).dev, true)
}

unsafe fn power_down_sst(stream: *mut sst_runtime_stream) {
    ((*(*stream).ops).power.unwrap())((*sst).dev, false);
}

unsafe extern "C" fn sst_media_open(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let mut ret_val: c_int = 0;
    let runtime = (*substream).runtime;
    let mut stream = kzalloc(size_of::<sst_runtime_stream>(), GFP_KERNEL) as *mut sst_runtime_stream;

    if stream.is_null() {
        return -ENOMEM;
    }
    spin_lock_init(&mut (*stream).status_lock);

    /* get the sst ops */
    mutex_lock(&raw mut sst_lock);
    if sst.is_null() || !try_module_get((*(*(*sst).dev).driver).owner) {
        dev_err((*dai).dev, b"no device available to run\n\0".as_ptr() as *const c_char);
        mutex_unlock(&raw mut sst_lock);
        kfree(stream as *mut c_void);
        return -ENODEV;
    }
    (*stream).ops = (*sst).ops;
    mutex_unlock(&raw mut sst_lock);

    (*stream).stream_info.str_id = 0;
    (*stream).stream_info.arg = substream as *mut c_void;
    /* allocate memory for SST API set */
    (*runtime).private_data = stream as *mut c_void;

    ret_val = power_up_sst(stream);
    if ret_val < 0 {
        kfree(stream as *mut c_void);
        return ret_val;
    }

    /*
     * Make sure the period to be multiple of 1ms to align the
     * design of firmware. Apply same rule to buffer size to make
     * sure alsa could always find a value for period size
     * regardless the buffer size given by user space.
     */
    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 48);
    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 48);

    /* Make sure, that the period size is always even */
    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_PERIODS, 2);

    ret_val = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret_val < 0 {
        kfree(stream as *mut c_void);
        return ret_val;
    }

    stream = ptr::null_mut();
    let _ = stream;
    ret_val
}

unsafe extern "C" fn sst_media_close(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) {
    let stream: *mut sst_runtime_stream;
    let str_id: c_int;

    stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    power_down_sst(stream);

    str_id = (*stream).stream_info.str_id;
    if str_id != 0 {
        ((*(*stream).ops).close.unwrap())((*sst).dev, str_id);
    }
    module_put((*(*(*sst).dev).driver).owner);
    kfree(stream as *mut c_void);
}

unsafe extern "C" fn sst_media_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let stream: *mut sst_runtime_stream;
    let ret_val: c_int;
    let str_id: c_int;

    stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    str_id = (*stream).stream_info.str_id;
    if (*stream).stream_info.str_id != 0 {
        return ((*(*stream).ops).stream_drop.unwrap())((*sst).dev, str_id);
    }

    let alloc_ret = sst_platform_alloc_stream(substream, dai);
    if alloc_ret <= 0 {
        return alloc_ret;
    }
    // snprintf(substream->pcm->id, sizeof(substream->pcm->id), "%d", stream->stream_info.str_id);

    ret_val = sst_platform_init_stream(substream);
    if ret_val != 0 {
        return ret_val;
    }
    (*(*substream).runtime).hw.info = SNDRV_PCM_INFO_BLOCK_TRANSFER;
    0
}

unsafe extern "C" fn sst_enable_ssp(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let mut ret: c_int = 0;

    if snd_soc_dai_active(dai) == 0 {
        ret = sst_handle_vb_timer(dai, true);
        sst_fill_ssp_defaults(dai);
    }
    ret
}

unsafe extern "C" fn sst_be_hw_params(
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int = 0;

    if snd_soc_dai_active(dai) == 1 {
        ret = send_ssp_cmd(dai, (*dai).name, 1);
    }
    ret
}

unsafe extern "C" fn sst_set_format(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let ret: c_int;

    if snd_soc_dai_active(dai) == 0 {
        return 0;
    }

    ret = sst_fill_ssp_config(dai, fmt);
    if ret < 0 {
        dev_err((*dai).dev, b"sst_set_format failed..\n\0".as_ptr() as *const c_char);
    }

    ret
}

unsafe extern "C" fn sst_platform_set_ssp_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let mut ret: c_int = 0;

    if snd_soc_dai_active(dai) == 0 {
        return ret;
    }

    ret = sst_fill_ssp_slot(dai, tx_mask, rx_mask, slots, slot_width);
    if ret < 0 {
        dev_err((*dai).dev, b"sst_fill_ssp_slot failed..%d\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

unsafe extern "C" fn sst_disable_ssp(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    if snd_soc_dai_active(dai) == 0 {
        send_ssp_cmd(dai, (*dai).name, 0);
        sst_handle_vb_timer(dai, false);
    }
}

static sst_media_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(sst_media_open),
    shutdown: Some(sst_media_close),
    prepare: Some(sst_media_prepare),
    hw_params: None,
    set_fmt: None,
    set_tdm_slot: None,
    mute_stream: Some(sst_media_digital_mute),
    compress_new: None,
};

static sst_compr_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: None,
    shutdown: None,
    prepare: None,
    hw_params: None,
    set_fmt: None,
    set_tdm_slot: None,
    compress_new: Some(snd_soc_new_compress),
    mute_stream: Some(sst_media_digital_mute),
};

static sst_be_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(sst_enable_ssp),
    shutdown: Some(sst_disable_ssp),
    prepare: None,
    hw_params: Some(sst_be_hw_params),
    set_fmt: Some(sst_set_format),
    set_tdm_slot: Some(sst_platform_set_ssp_slot),
    mute_stream: None,
    compress_new: None,
};

static mut sst_platform_dai: [snd_soc_dai_driver; 6] = [
    snd_soc_dai_driver {
        name: b"media-cpu-dai\0".as_ptr() as *const c_char,
        ops: &sst_media_dai_ops,
        playback: snd_soc_pcm_stream { stream_name: b"Headset Playback\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        capture: snd_soc_pcm_stream { stream_name: b"Headset Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
    },
    snd_soc_dai_driver {
        name: b"deepbuffer-cpu-dai\0".as_ptr() as *const c_char,
        ops: &sst_media_dai_ops,
        playback: snd_soc_pcm_stream { stream_name: b"Deepbuffer Playback\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
    },
    snd_soc_dai_driver {
        name: b"compress-cpu-dai\0".as_ptr() as *const c_char,
        ops: &sst_compr_dai_ops,
        playback: snd_soc_pcm_stream { stream_name: b"Compress Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
    },
    /* BE CPU  Dais */
    snd_soc_dai_driver {
        name: b"ssp0-port\0".as_ptr() as *const c_char,
        ops: &sst_be_dai_ops,
        playback: snd_soc_pcm_stream { stream_name: b"ssp0 Tx\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        capture: snd_soc_pcm_stream { stream_name: b"ssp0 Rx\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
    },
    snd_soc_dai_driver {
        name: b"ssp1-port\0".as_ptr() as *const c_char,
        ops: &sst_be_dai_ops,
        playback: snd_soc_pcm_stream { stream_name: b"ssp1 Tx\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        capture: snd_soc_pcm_stream { stream_name: b"ssp1 Rx\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
    },
    snd_soc_dai_driver {
        name: b"ssp2-port\0".as_ptr() as *const c_char,
        ops: &sst_be_dai_ops,
        playback: snd_soc_pcm_stream { stream_name: b"ssp2 Tx\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        capture: snd_soc_pcm_stream { stream_name: b"ssp2 Rx\0".as_ptr() as *const c_char, channels_min: SST_STEREO, channels_max: SST_STEREO, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
    },
];

unsafe extern "C" fn sst_soc_open(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime;

    if (*(*substream).pcm).internal != 0 {
        return 0;
    }

    runtime = (*substream).runtime;
    (*runtime).hw = sst_platform_pcm_hw;
    0
}

unsafe extern "C" fn sst_soc_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let mut ret_val: c_int = 0;
    let str_id: c_int;
    let stream: *mut sst_runtime_stream;
    let status: c_int;
    let rtd = snd_soc_substream_to_rtd(substream);

    dev_dbg((*rtd).dev, b"%s called\n\0".as_ptr() as *const c_char, b"sst_soc_trigger\0".as_ptr() as *const c_char);
    if (*(*substream).pcm).internal != 0 {
        return 0;
    }
    stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    str_id = (*stream).stream_info.str_id;
    status = match cmd {
        SNDRV_PCM_TRIGGER_START => {
            dev_dbg((*rtd).dev, b"sst: Trigger Start\n\0".as_ptr() as *const c_char);
            (*stream).stream_info.arg = substream as *mut c_void;
            ret_val = ((*(*stream).ops).stream_start.unwrap())((*sst).dev, str_id);
            SST_PLATFORM_RUNNING
        }
        SNDRV_PCM_TRIGGER_STOP => {
            dev_dbg((*rtd).dev, b"sst: in stop\n\0".as_ptr() as *const c_char);
            ret_val = ((*(*stream).ops).stream_drop.unwrap())((*sst).dev, str_id);
            SST_PLATFORM_DROPPED
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            dev_dbg((*rtd).dev, b"sst: in pause\n\0".as_ptr() as *const c_char);
            ret_val = ((*(*stream).ops).stream_pause.unwrap())((*sst).dev, str_id);
            SST_PLATFORM_PAUSED
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            dev_dbg((*rtd).dev, b"sst: in pause release\n\0".as_ptr() as *const c_char);
            ret_val = ((*(*stream).ops).stream_pause_release.unwrap())((*sst).dev, str_id);
            SST_PLATFORM_RUNNING
        }
        _ => return -EINVAL,
    };

    if ret_val == 0 {
        sst_set_stream_status(stream, status);
    }

    ret_val
}

unsafe extern "C" fn sst_soc_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let stream: *mut sst_runtime_stream;
    let ret_val: c_int;
    let status: c_int;
    let str_info: *mut pcm_stream_info;
    let rtd = snd_soc_substream_to_rtd(substream);

    stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    status = sst_get_stream_status(stream);
    if status == SST_PLATFORM_INIT {
        return 0;
    }
    str_info = &mut (*stream).stream_info;
    ret_val = ((*(*stream).ops).stream_read_tstamp.unwrap())((*sst).dev, str_info);
    if ret_val != 0 {
        dev_err((*rtd).dev, b"sst: error code = %d\n\0".as_ptr() as *const c_char, ret_val);
        return ret_val as snd_pcm_uframes_t;
    }
    (*str_info).buffer_ptr
}

unsafe extern "C" fn sst_soc_delay(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_sframes_t {
    let stream = (*(*substream).runtime).private_data as *mut sst_runtime_stream;
    let str_info = &mut (*stream).stream_info as *mut pcm_stream_info;

    if sst_get_stream_status(stream) == SST_PLATFORM_INIT {
        return 0;
    }

    (*str_info).pcm_delay
}

unsafe extern "C" fn sst_soc_pcm_new(_component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dai = snd_soc_rtd_to_cpu(rtd, 0);
    let pcm = (*rtd).pcm;

    if (*(*dai).driver).playback.channels_min != 0 || (*(*dai).driver).capture.channels_min != 0 {
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV,
            (*(*pcm).card).dev,
            SST_MIN_BUFFER,
            SST_MAX_BUFFER,
        );
    }
    0
}

unsafe extern "C" fn sst_soc_probe(component: *mut snd_soc_component) -> c_int {
    let drv = dev_get_drvdata((*component).dev) as *mut sst_data;

    (*drv).soc_card = (*component).card;
    sst_dsp_init_v2_dpcm(component)
}

unsafe extern "C" fn sst_soc_remove(component: *mut snd_soc_component) {
    let drv = dev_get_drvdata((*component).dev) as *mut sst_data;

    (*drv).soc_card = ptr::null_mut();
}

static sst_soc_platform_drv: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    probe: Some(sst_soc_probe),
    remove: Some(sst_soc_remove),
    open: Some(sst_soc_open),
    trigger: Some(sst_soc_trigger),
    pointer: Some(sst_soc_pointer),
    delay: Some(sst_soc_delay),
    compress_ops: unsafe { &sst_platform_compress_ops },
    pcm_new: Some(sst_soc_pcm_new),
};

unsafe extern "C" fn sst_platform_probe(pdev: *mut platform_device) -> c_int {
    let drv: *mut sst_data;
    let ret: c_int;
    let pdata: *mut sst_platform_data;

    drv = devm_kzalloc(&mut (*pdev).dev, size_of::<sst_data>(), GFP_KERNEL) as *mut sst_data;
    if drv.is_null() {
        return -ENOMEM;
    }

    pdata = devm_kzalloc(&mut (*pdev).dev, size_of::<sst_platform_data>(), GFP_KERNEL) as *mut sst_platform_data;
    if pdata.is_null() {
        return -ENOMEM;
    }

    (*pdata).pdev_strm_map = dpcm_strm_map.as_mut_ptr();
    (*pdata).strm_map_size = dpcm_strm_map.len() as c_int;
    (*drv).pdata = pdata;
    (*drv).pdev = pdev;
    mutex_init(&mut (*drv).lock);
    dev_set_drvdata(&mut (*pdev).dev, drv as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sst_soc_platform_drv,
        sst_platform_dai.as_mut_ptr(),
        sst_platform_dai.len() as c_int,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"registering cpu dais failed\n\0".as_ptr() as *const c_char);
    }

    ret
}

unsafe extern "C" fn sst_platform_remove(pdev: *mut platform_device) {
    dev_dbg(&mut (*pdev).dev, b"sst_platform_remove success\n\0".as_ptr() as *const c_char);
}

// CONFIG_PM_SLEEP conditional code from C follows. In a kernel Rust build this
// should be gated by the matching configuration symbol.

unsafe extern "C" fn sst_soc_prepare(dev: *mut device) -> c_int {
    let drv = dev_get_drvdata(dev) as *mut sst_data;
    let _rtd: *mut snd_soc_pcm_runtime;

    if (*drv).soc_card.is_null() {
        return 0;
    }

    /* suspend all pcms first */
    snd_soc_suspend((*(*drv).soc_card).dev);
    snd_soc_poweroff((*(*drv).soc_card).dev);

    /* set the SSPs to idle */
    // for_each_card_rtds(drv->soc_card, rtd) {
    //     struct snd_soc_dai *dai = snd_soc_rtd_to_cpu(rtd, 0);
    //
    //     if (snd_soc_dai_active(dai)) {
    //         send_ssp_cmd(dai, dai->name, 0);
    //         sst_handle_vb_timer(dai, false);
    //     }
    // }

    0
}

unsafe extern "C" fn sst_soc_complete(dev: *mut device) {
    let drv = dev_get_drvdata(dev) as *mut sst_data;
    let _rtd: *mut snd_soc_pcm_runtime;

    if (*drv).soc_card.is_null() {
        return;
    }

    /* restart SSPs */
    // for_each_card_rtds(drv->soc_card, rtd) {
    //     struct snd_soc_dai *dai = snd_soc_rtd_to_cpu(rtd, 0);
    //
    //     if (snd_soc_dai_active(dai)) {
    //         sst_handle_vb_timer(dai, true);
    //         send_ssp_cmd(dai, dai->name, 1);
    //     }
    // }
    snd_soc_resume((*(*drv).soc_card).dev);
}

// #else
// #define sst_soc_prepare NULL
// #define sst_soc_complete NULL
// #endif

static sst_platform_pm: dev_pm_ops = dev_pm_ops {
    prepare: Some(sst_soc_prepare),
    complete: Some(sst_soc_complete),
};

static mut sst_platform_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"sst-mfld-platform\0".as_ptr() as *const c_char,
        pm: &sst_platform_pm,
    },
    probe: Some(sst_platform_probe),
    remove: Some(sst_platform_remove),
};

// module_platform_driver(sst_platform_driver);

// MODULE_DESCRIPTION("ASoC Intel(R) MID Platform driver");
// MODULE_AUTHOR("Vinod Koul <vinod.koul@intel.com>");
// MODULE_AUTHOR("Harsha Priya <priya.harsha@intel.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:sst-atom-hifi2-platform");
// MODULE_ALIAS("platform:sst-mfld-platform");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
