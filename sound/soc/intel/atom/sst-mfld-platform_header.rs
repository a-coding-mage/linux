/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  sst_mfld_platform.h - Intel MID Platform driver header file
 *
 *  Copyright (C) 2010 Intel Corp
 *  Author: Vinod Koul <vinod.koul@intel.com>
 *  Author: Harsha Priya <priya.harsha@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

/* C header guard removed: __SST_PLATFORMDRV_H__ */
/* Dependencies from C includes:
 * "sst-mfld-dsp.h"
 * "sst-atom-controls.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;

/* External dependency types supplied by other files. */
pub enum device {}
pub enum platform_device {}
pub enum snd_compress_ops {}
pub enum snd_sst_params {}
pub enum snd_compr_tstamp64 {}
pub enum snd_compr_caps {}
pub enum snd_compr_codec_caps {}
pub enum snd_compr_metadata {}
pub enum snd_sst_bytes_v2 {}
pub enum snd_soc_component {}
pub enum snd_soc_dai {}
pub enum soc_mixer_control {}
pub enum mutex {}
pub enum snd_soc_card {}
pub enum sst_cmd_sba_hw_set_ssp {}
pub enum sst_platform_data {}
pub enum spinlock_t {}

unsafe extern "C" {
    pub static mut sst: *mut sst_device;
    pub static sst_platform_compress_ops: snd_compress_ops;
}

pub const DRV_NAME: &[u8; 4] = b"sst\0";

pub const SST_MONO: c_int = 1;
pub const SST_STEREO: c_int = 2;
pub const SST_MAX_CAP: c_int = 5;

pub const SST_MAX_BUFFER: c_int = 800 * 1024;
pub const SST_MIN_BUFFER: c_int = 800 * 1024;
pub const SST_MIN_PERIOD_BYTES: c_int = 32;
pub const SST_MAX_PERIOD_BYTES: c_int = SST_MAX_BUFFER;
pub const SST_MIN_PERIODS: c_int = 2;
pub const SST_MAX_PERIODS: c_int = 1024 * 2;
pub const SST_FIFO_SIZE: c_int = 0;

#[repr(C)]
pub struct pcm_stream_info {
    pub str_id: c_int,
    pub arg: *mut c_void,
    pub period_elapsed: Option<unsafe extern "C" fn(arg: *mut c_void)>,
    pub buffer_ptr: u64,
    pub pcm_delay: u64,
    pub sfreq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_drv_status {
    SST_PLATFORM_INIT = 1,
    SST_PLATFORM_STARTED = 2,
    SST_PLATFORM_RUNNING = 3,
    SST_PLATFORM_PAUSED = 4,
    SST_PLATFORM_DROPPED = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_stream_ops {
    STREAM_OPS_PLAYBACK = 0,
    STREAM_OPS_CAPTURE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_audio_device_type {
    SND_SST_DEVICE_HEADSET = 1,
    SND_SST_DEVICE_IHF = 2,
    SND_SST_DEVICE_VIBRA = 3,
    SND_SST_DEVICE_HAPTIC = 4,
    SND_SST_DEVICE_CAPTURE = 5,
    SND_SST_DEVICE_COMPRESS = 6,
}

/* PCM Parameters */
#[repr(C)]
pub struct sst_pcm_params {
    pub codec: u16, /* codec type */
    pub num_chan: u8, /* 1=Mono, 2=Stereo */
    pub pcm_wd_sz: u8, /* 16/24 - bit*/
    pub reserved: u32, /* Bitrate in bits per second */
    pub sfreq: u32, /* Sampling rate in Hz */
    pub ring_buffer_size: u32,
    pub period_count: u32, /* period elapsed in samples*/
    pub ring_buffer_addr: u32,
}

#[repr(C)]
pub struct sst_stream_params {
    pub result: u32,
    pub stream_id: u32,
    pub codec: u8,
    pub ops: u8,
    pub stream_type: u8,
    pub device_type: u8,
    pub sparams: sst_pcm_params,
}

#[repr(C)]
pub struct sst_compress_cb {
    pub param: *mut c_void,
    pub compr_cb: Option<unsafe extern "C" fn(param: *mut c_void)>,
    pub drain_cb_param: *mut c_void,
    pub drain_notify: Option<unsafe extern "C" fn(param: *mut c_void)>,
}

#[repr(C)]
pub struct compress_sst_ops {
    pub name: *const c_char,
    pub open: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            str_params: *mut snd_sst_params,
            cb: *mut sst_compress_cb,
        ) -> c_int,
    >,
    pub stream_start: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,
    pub stream_drop: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,
    pub stream_drain: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,
    pub stream_partial_drain:
        Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,
    pub stream_pause: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,
    pub stream_pause_release:
        Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,

    pub tstamp: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            str_id: c_uint,
            tstamp: *mut snd_compr_tstamp64,
        ) -> c_int,
    >,
    pub ack: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint, bytes: c_ulong) -> c_int>,
    pub close: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,
    pub get_caps: Option<unsafe extern "C" fn(caps: *mut snd_compr_caps) -> c_int>,
    pub get_codec_caps:
        Option<unsafe extern "C" fn(codec: *mut snd_compr_codec_caps) -> c_int>,
    pub set_metadata: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            str_id: c_uint,
            mdata: *mut snd_compr_metadata,
        ) -> c_int,
    >,
    pub power: Option<unsafe extern "C" fn(dev: *mut device, state: bool) -> c_int>,
}

#[repr(C)]
pub struct sst_ops {
    pub open:
        Option<unsafe extern "C" fn(dev: *mut device, str_param: *mut snd_sst_params) -> c_int>,
    pub stream_init:
        Option<unsafe extern "C" fn(dev: *mut device, str_info: *mut pcm_stream_info) -> c_int>,
    pub stream_start: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_int) -> c_int>,
    pub stream_drop: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_int) -> c_int>,
    pub stream_pause: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_int) -> c_int>,
    pub stream_pause_release:
        Option<unsafe extern "C" fn(dev: *mut device, str_id: c_int) -> c_int>,
    pub stream_read_tstamp:
        Option<unsafe extern "C" fn(dev: *mut device, str_info: *mut pcm_stream_info) -> c_int>,
    pub send_byte_stream:
        Option<unsafe extern "C" fn(dev: *mut device, bytes: *mut snd_sst_bytes_v2) -> c_int>,
    pub close: Option<unsafe extern "C" fn(dev: *mut device, str_id: c_uint) -> c_int>,
    pub power: Option<unsafe extern "C" fn(dev: *mut device, state: bool) -> c_int>,
}

#[repr(C)]
pub struct sst_runtime_stream {
    pub stream_status: c_int,
    pub id: c_uint,
    pub bytes_written: usize,
    pub stream_info: pcm_stream_info,
    pub ops: *mut sst_ops,
    pub compr_ops: *mut compress_sst_ops,
    pub status_lock: spinlock_t,
}

#[repr(C)]
pub struct sst_device {
    pub name: *mut c_char,
    pub dev: *mut device,
    pub ops: *mut sst_ops,
    pub pdev: *mut platform_device,
    pub compr_ops: *mut compress_sst_ops,
}

unsafe extern "C" {
    pub fn sst_dsp_init_v2_dpcm(component: *mut snd_soc_component) -> c_int;
    pub fn sst_send_pipe_gains(dai: *mut snd_soc_dai, stream: c_int, mute: c_int) -> c_int;
    pub fn send_ssp_cmd(dai: *mut snd_soc_dai, id: *const c_char, enable: bool) -> c_int;
    pub fn sst_handle_vb_timer(dai: *mut snd_soc_dai, enable: bool) -> c_int;

    pub fn sst_set_stream_status(stream: *mut sst_runtime_stream, state: c_int);
    pub fn sst_fill_stream_params(
        substream: *mut c_void,
        ctx: *const sst_data,
        str_params: *mut snd_sst_params,
        is_compress: bool,
    ) -> c_int;
}

#[repr(C)]
pub struct sst_algo_int_control_v2 {
    pub mc: soc_mixer_control,
    pub module_id: u16, /* module identifieer */
    pub pipe_id: u16, /* location info: pipe_id + instance_id */
    pub instance_id: u16,
    pub value: c_uint, /* Value received is stored here */
}

#[repr(C)]
pub struct sst_data {
    pub pdev: *mut platform_device,
    pub pdata: *mut sst_platform_data,
    pub byte_stream: *mut snd_sst_bytes_v2,
    pub lock: mutex,
    pub soc_card: *mut snd_soc_card,
    pub ssp_cmd: sst_cmd_sba_hw_set_ssp,
}

unsafe extern "C" {
    pub fn sst_register_dsp(dev: *mut sst_device) -> c_int;
    pub fn sst_unregister_dsp(dev: *mut sst_device) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
