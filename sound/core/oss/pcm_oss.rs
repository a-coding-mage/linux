// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Digital Audio (PCM) abstract layer / OSS compatible
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 * Rust source-level translation of core/oss/pcm_oss.c.
 * C include dependencies, kernel attributes, module metadata, and build-time
 * CONFIG_* conditionals are preserved as comments or external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_t = bool;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type fmode_t = c_uint;
type __poll_t = c_uint;
type snd_pcm_uframes_t = c_ulong;
type snd_pcm_sframes_t = c_long;
type snd_pcm_format_t = c_int;
type snd_pcm_hw_param_t = c_uint;
type snd_pcm_state_t = c_int;
type u64_t = u64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ERESTARTSYS: c_int = 512;
const EBUSY: c_int = 16;
const EAGAIN: c_int = 11;
const EIO: c_int = 5;
const EPIPE: c_int = 32;
const ESTRPIPE: c_int = 86;
const ENXIO: c_int = 6;
const EFAULT: c_int = 14;
const ENODEV: c_int = 19;

const SNDRV_CARDS: usize = 32;
const SNDRV_PCM_DEVICES: c_int = 32;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_OSS_DEVICE_TYPE_PCM: c_int = 0;
const SNDRV_OSS_VERSION: c_int = 0x030803;

const SNDRV_PCM_HW_PARAM_ACCESS: snd_pcm_hw_param_t = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: snd_pcm_hw_param_t = 1;
const SNDRV_PCM_HW_PARAM_SUBFORMAT: snd_pcm_hw_param_t = 2;
const SNDRV_PCM_HW_PARAM_CHANNELS: snd_pcm_hw_param_t = 10;
const SNDRV_PCM_HW_PARAM_RATE: snd_pcm_hw_param_t = 11;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: snd_pcm_hw_param_t = 13;
const SNDRV_PCM_HW_PARAM_PERIODS: snd_pcm_hw_param_t = 15;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: snd_pcm_hw_param_t = 16;

const SNDRV_PCM_ACCESS_MMAP_INTERLEAVED: c_uint = 0;
const SNDRV_PCM_ACCESS_RW_INTERLEAVED: c_uint = 3;
const SNDRV_PCM_ACCESS_RW_NONINTERLEAVED: c_uint = 4;

const SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_A_LAW: snd_pcm_format_t = 1;
const SNDRV_PCM_FORMAT_IMA_ADPCM: snd_pcm_format_t = 2;
const SNDRV_PCM_FORMAT_U8: snd_pcm_format_t = 3;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 4;
const SNDRV_PCM_FORMAT_S16_BE: snd_pcm_format_t = 5;
const SNDRV_PCM_FORMAT_S8: snd_pcm_format_t = 6;
const SNDRV_PCM_FORMAT_U16_LE: snd_pcm_format_t = 7;
const SNDRV_PCM_FORMAT_U16_BE: snd_pcm_format_t = 8;
const SNDRV_PCM_FORMAT_MPEG: snd_pcm_format_t = 9;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 10;
const SNDRV_PCM_FORMAT_S32_BE: snd_pcm_format_t = 11;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 12;
const SNDRV_PCM_FORMAT_S24_BE: snd_pcm_format_t = 13;
const SNDRV_PCM_FORMAT_S24_3LE: snd_pcm_format_t = 14;
const SNDRV_PCM_FORMAT_FLOAT: snd_pcm_format_t = 15;
const SNDRV_PCM_FORMAT_IEC958_SUBFRAME: snd_pcm_format_t = 16;

const SNDRV_PCM_STATE_OPEN: snd_pcm_state_t = 0;
const SNDRV_PCM_STATE_SETUP: snd_pcm_state_t = 1;
const SNDRV_PCM_STATE_PREPARED: snd_pcm_state_t = 2;
const SNDRV_PCM_STATE_RUNNING: snd_pcm_state_t = 3;
const SNDRV_PCM_STATE_XRUN: snd_pcm_state_t = 4;
const SNDRV_PCM_STATE_DRAINING: snd_pcm_state_t = 5;
const SNDRV_PCM_STATE_SUSPENDED: snd_pcm_state_t = 7;

const SNDRV_PCM_IOCTL_DROP: c_uint = 0;
const SNDRV_PCM_IOCTL_HW_PARAMS: c_uint = 1;
const SNDRV_PCM_IOCTL_SW_PARAMS: c_uint = 2;
const SNDRV_PCM_IOCTL_PREPARE: c_uint = 3;
const SNDRV_PCM_IOCTL_DELAY: c_uint = 4;
const SNDRV_PCM_IOCTL_FORWARD: c_uint = 5;
const SNDRV_PCM_IOCTL_DRAIN: c_uint = 6;
const SNDRV_PCM_IOCTL_START: c_uint = 7;

const SNDRV_PCM_TSTAMP_NONE: c_uint = 0;
const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;

const O_NONBLOCK: c_uint = 0x800;
const O_APPEND: c_uint = 0x400;
const FMODE_READ: fmode_t = 1;
const FMODE_WRITE: fmode_t = 2;
const VM_READ: c_ulong = 1;
const VM_WRITE: c_ulong = 2;
const GFP_KERNEL: c_uint = 0;
const TASK_INTERRUPTIBLE: c_int = 1;
const TASK_RUNNING: c_int = 0;
const HZ: c_long = 100;
const EPOLLIN: __poll_t = 0x001;
const EPOLLOUT: __poll_t = 0x004;
const EPOLLRDNORM: __poll_t = 0x040;
const EPOLLWRNORM: __poll_t = 0x100;

const AFMT_QUERY: c_int = 0;
const AFMT_MU_LAW: c_int = 0x00000001;
const AFMT_A_LAW: c_int = 0x00000002;
const AFMT_IMA_ADPCM: c_int = 0x00000004;
const AFMT_U8: c_int = 0x00000008;
const AFMT_S16_LE: c_int = 0x00000010;
const AFMT_S16_BE: c_int = 0x00000020;
const AFMT_S8: c_int = 0x00000040;
const AFMT_U16_LE: c_int = 0x00000080;
const AFMT_U16_BE: c_int = 0x00000100;
const AFMT_MPEG: c_int = 0x00000200;
/* define extended formats in the recent OSS versions (if any) */
/* linear formats */
const AFMT_S32_LE: c_int = 0x00001000;
const AFMT_S32_BE: c_int = 0x00002000;
const AFMT_S24_LE: c_int = 0x00008000;
const AFMT_S24_BE: c_int = 0x00010000;
const AFMT_S24_PACKED: c_int = 0x00040000;
/* other supported formats */
const AFMT_FLOAT: c_int = 0x00004000;
const AFMT_SPDIF_RAW: c_int = 0x00020000;
/* unsupported formats */
const AFMT_AC3: c_int = 0x00000400;
const AFMT_VORBIS: c_int = 0x00000800;

const PCM_ENABLE_OUTPUT: c_int = 0x00000001;
const PCM_ENABLE_INPUT: c_int = 0x00000002;
const DSP_CAP_DUPLEX: c_int = 0x00000100;
const DSP_CAP_TRIGGER: c_int = 0x00001000;
const DSP_CAP_MMAP: c_int = 0x00002000;
const DSP_CAP_REALTIME: c_int = 0x00004000;

const OSS_GETVERSION: c_uint = 0x80044d76;
const OSS_ALSAEMULVER: c_uint = 0x80044df9;
const SNDCTL_DSP_RESET: c_uint = 0x00005000;
const SNDCTL_DSP_SYNC: c_uint = 0x00005001;
const SNDCTL_DSP_SPEED: c_uint = 0xc0045002;
const SOUND_PCM_READ_RATE: c_uint = 0x80045002;
const SNDCTL_DSP_STEREO: c_uint = 0xc0045003;
const SNDCTL_DSP_GETBLKSIZE: c_uint = 0xc0045004;
const SNDCTL_DSP_SETFMT: c_uint = 0xc0045005;
const SOUND_PCM_READ_BITS: c_uint = 0x80045005;
const SNDCTL_DSP_CHANNELS: c_uint = 0xc0045006;
const SOUND_PCM_READ_CHANNELS: c_uint = 0x80045006;
const SOUND_PCM_WRITE_FILTER: c_uint = 0xc0045007;
const SOUND_PCM_READ_FILTER: c_uint = 0xc0045008;
const SNDCTL_DSP_POST: c_uint = 0x00005008;
const SNDCTL_DSP_SUBDIVIDE: c_uint = 0xc0045009;
const SNDCTL_DSP_SETFRAGMENT: c_uint = 0xc004500a;
const SNDCTL_DSP_GETFMTS: c_uint = 0x8004500b;
const SNDCTL_DSP_GETOSPACE: c_uint = 0x8010500c;
const SNDCTL_DSP_GETISPACE: c_uint = 0x8010500d;
const SNDCTL_DSP_NONBLOCK: c_uint = 0x0000500e;
const SNDCTL_DSP_GETCAPS: c_uint = 0x8004500f;
const SNDCTL_DSP_GETTRIGGER: c_uint = 0x80045010;
const SNDCTL_DSP_SETTRIGGER: c_uint = 0x40045010;
const SNDCTL_DSP_GETIPTR: c_uint = 0x800c5011;
const SNDCTL_DSP_GETOPTR: c_uint = 0x800c5012;
const SNDCTL_DSP_MAPINBUF: c_uint = 0x800c5013;
const SNDCTL_DSP_MAPOUTBUF: c_uint = 0x800c5014;
const SNDCTL_DSP_SETSYNCRO: c_uint = 0x00005015;
const SNDCTL_DSP_SETDUPLEX: c_uint = 0x00005016;
const SNDCTL_DSP_GETODELAY: c_uint = 0x80045017;
const SNDCTL_DSP_PROFILE: c_uint = 0x40045018;

#[repr(C)]
pub struct snd_interval {
    empty: c_uint,
    min: c_uint,
    max: c_uint,
    openmin: c_uint,
    openmax: c_uint,
    integer: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    bits: [c_uint; 2],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    flags: c_uint,
    masks: [snd_mask; 8],
    mres: [snd_mask; 5],
    intervals: [snd_interval; 12],
    ires: [snd_interval; 9],
    rmask: c_uint,
    cmask: c_uint,
    info: c_uint,
    msbits: c_uint,
    rate_num: c_uint,
    rate_den: c_uint,
    fifo_size: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_sw_params {
    tstamp_mode: c_uint,
    period_step: c_uint,
    sleep_min: c_uint,
    avail_min: snd_pcm_uframes_t,
    xfer_align: snd_pcm_uframes_t,
    start_threshold: snd_pcm_uframes_t,
    stop_threshold: snd_pcm_uframes_t,
    silence_threshold: snd_pcm_uframes_t,
    silence_size: snd_pcm_uframes_t,
    boundary: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_oss_setup {
    task_name: *mut c_char,
    next: *mut snd_pcm_oss_setup,
    disable: c_uint,
    direct: c_uint,
    block: c_uint,
    nonblock: c_uint,
    partialfrag: c_uint,
    nosilence: c_uint,
    buggyptr: c_uint,
    periods: c_uint,
    period_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_oss_runtime {
    params: c_uint,
    trigger: c_uint,
    rate: c_uint,
    channels: c_uint,
    format: c_int,
    fragshift: c_int,
    maxfrags: c_int,
    subdivision: c_int,
    period_bytes: ssize_t,
    buffer_bytes: ssize_t,
    mmap_bytes: ssize_t,
    period_frames: snd_pcm_uframes_t,
    periods: ssize_t,
    buffer: *mut c_char,
    buffer_used: ssize_t,
    bytes: snd_pcm_sframes_t,
    period_ptr: ssize_t,
    prepare: c_uint,
    prev_hw_ptr_period: snd_pcm_uframes_t,
    params_lock: mutex,
    rw_ref: atomic_t,
    plugin_first: *mut snd_pcm_plugin,
    plugin_last: *mut snd_pcm_plugin,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    oss: snd_pcm_oss_runtime,
    hw_ptr_interrupt: snd_pcm_uframes_t,
    boundary: snd_pcm_uframes_t,
    buffer_size: snd_pcm_uframes_t,
    period_size: snd_pcm_uframes_t,
    periods: c_uint,
    access: c_uint,
    info: c_uint,
    silence_threshold: snd_pcm_uframes_t,
    silence_size: snd_pcm_uframes_t,
    start_threshold: snd_pcm_uframes_t,
    stop_threshold: snd_pcm_uframes_t,
    state: snd_pcm_state_t,
    sleep: wait_queue_head_t,
    control: *mut snd_pcm_mmap_control,
    status: *mut snd_pcm_mmap_status,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    pcm: *mut snd_pcm,
    stream: c_int,
    f_flags: c_uint,
    mmap_count: atomic_t,
    oss: snd_pcm_substream_oss,
    pstr: *mut snd_pcm_str,
    pcm_release: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
pub struct snd_pcm_substream_oss {
    oss: c_uint,
    setup: snd_pcm_oss_setup,
}

#[repr(C)]
pub struct snd_pcm_oss_file {
    streams: [*mut snd_pcm_substream; 2],
}

#[repr(C)]
pub struct snd_pcm_plugin {
    next: *mut snd_pcm_plugin,
    prev: *mut snd_pcm_plugin,
    plug: *mut snd_pcm_substream,
    src_width: size_t,
    src_format: snd_pcm_plugin_format,
    dst_width: size_t,
    dst_format: snd_pcm_plugin_format,
}

#[repr(C)]
pub struct snd_pcm_plugin_format {
    channels: c_uint,
}

#[repr(C)]
pub struct snd_pcm_plugin_channel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    card: *mut snd_card,
    device: c_uint,
    name: [c_char; 128],
    streams: [snd_pcm_str; 2],
    info_flags: c_uint,
    open_wait: wait_queue_head_t,
    open_mutex: mutex,
    oss: snd_pcm_oss_pcm,
}

#[repr(C)]
pub struct snd_pcm_oss_pcm {
    reg: c_uint,
    reg_mask: c_uint,
}

#[repr(C)]
pub struct snd_pcm_str {
    substream_count: c_uint,
    proc_root: *mut c_void,
    oss: snd_pcm_str_oss,
}

#[repr(C)]
pub struct snd_pcm_str_oss {
    setup_mutex: mutex,
    setup_list: *mut snd_pcm_oss_setup,
    proc_entry: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_card {
    number: c_int,
    module: *mut c_void,
    shutdown: c_uint,
}

#[repr(C)]
pub struct file {
    f_mode: fmode_t,
    f_flags: c_uint,
    private_data: *mut c_void,
    f_lock: spinlock_t,
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    vm_flags: c_ulong,
    vm_pgoff: c_ulong,
    vm_start: c_ulong,
    vm_end: c_ulong,
}

#[repr(C)]
pub struct count_info {
    bytes: c_int,
    blocks: c_int,
    ptr: c_int,
}

#[repr(C)]
pub struct audio_buf_info {
    fragments: c_int,
    fragstotal: c_int,
    fragsize: c_int,
    bytes: c_int,
}

#[repr(C)]
pub struct buffmem_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_mmap_control {
    appl_ptr: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_mmap_status {
    hw_ptr: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_notify {
    n_register: Option<unsafe extern "C" fn(*mut snd_pcm) -> c_int>,
    n_disconnect: Option<unsafe extern "C" fn(*mut snd_pcm) -> c_int>,
    n_unregister: Option<unsafe extern "C" fn(*mut snd_pcm) -> c_int>,
}

#[repr(C)]
pub struct file_operations {
    owner: *mut c_void,
    read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
}

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { counter: c_int }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_entry_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { comm: [c_char; 16] }
#[repr(C)] pub struct snd_info_entry { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_buffer { error: c_int }

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut THIS_MODULE: *mut c_void;
    fn snd_interval_checkempty(i: *const snd_interval) -> c_int;
    fn snd_interval_none(i: *mut snd_interval);
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_interval_min(i: *const snd_interval) -> c_uint;
    fn snd_interval_max(i: *const snd_interval) -> c_uint;
    fn snd_interval_setinteger(i: *mut snd_interval) -> c_int;
    fn snd_mask_min(mask: *const snd_mask) -> c_uint;
    fn snd_mask_max(mask: *const snd_mask) -> c_uint;
    fn snd_mask_refine(mask: *mut snd_mask, v: *const snd_mask) -> c_int;
    fn snd_mask_refine_min(mask: *mut snd_mask, val: c_uint) -> c_int;
    fn snd_mask_refine_max(mask: *mut snd_mask, val: c_uint) -> c_int;
    fn snd_mask_refine_set(mask: *mut snd_mask, val: c_uint) -> c_int;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set(mask: *mut snd_mask, val: c_uint);
    fn snd_mask_test(mask: *const snd_mask, val: c_int) -> c_int;
    fn snd_mask_test_format(mask: *const snd_mask, val: snd_pcm_format_t) -> c_int;
    fn hw_is_mask(v: snd_pcm_hw_param_t) -> c_int;
    fn hw_is_interval(v: snd_pcm_hw_param_t) -> c_int;
    fn hw_param_mask(p: *mut snd_pcm_hw_params, v: snd_pcm_hw_param_t) -> *mut snd_mask;
    fn hw_param_mask_c(p: *const snd_pcm_hw_params, v: snd_pcm_hw_param_t) -> *const snd_mask;
    fn hw_param_interval(p: *mut snd_pcm_hw_params, v: snd_pcm_hw_param_t) -> *mut snd_interval;
    fn hw_param_interval_c(p: *const snd_pcm_hw_params, v: snd_pcm_hw_param_t) -> *const snd_interval;
    fn _snd_pcm_hw_params_any(p: *mut snd_pcm_hw_params);
    fn snd_pcm_hw_refine(s: *mut snd_pcm_substream, p: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_hw_param_value(p: *const snd_pcm_hw_params, v: snd_pcm_hw_param_t, d: *mut c_int) -> c_int;
    fn snd_pcm_hw_param_first(s: *mut snd_pcm_substream, p: *mut snd_pcm_hw_params, v: snd_pcm_hw_param_t, d: *mut c_int) -> c_int;
    fn snd_pcm_hw_param_last(s: *mut snd_pcm_substream, p: *mut snd_pcm_hw_params, v: snd_pcm_hw_param_t, d: *mut c_int) -> c_int;
    fn params_format(p: *const snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(p: *const snd_pcm_hw_params) -> c_uint;
    fn params_rate(p: *const snd_pcm_hw_params) -> c_uint;
    fn params_access(p: *const snd_pcm_hw_params) -> c_uint;
    fn params_periods(p: *const snd_pcm_hw_params) -> c_uint;
    fn params_period_size(p: *const snd_pcm_hw_params) -> snd_pcm_uframes_t;
    fn params_buffer_size(p: *const snd_pcm_hw_params) -> snd_pcm_uframes_t;
    fn snd_pcm_format_physical_width(f: snd_pcm_format_t) -> c_uint;
    fn snd_pcm_format_set_silence(f: snd_pcm_format_t, data: *mut c_void, samples: size_t);
    fn frames_to_bytes(r: *mut snd_pcm_runtime, frames: snd_pcm_sframes_t) -> ssize_t;
    fn bytes_to_frames(r: *mut snd_pcm_runtime, bytes: ssize_t) -> snd_pcm_sframes_t;
    fn snd_pcm_lib_buffer_bytes(s: *mut snd_pcm_substream) -> ssize_t;
    fn snd_pcm_plug_client_size(s: *mut snd_pcm_substream, frames: snd_pcm_sframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_plug_slave_size(s: *mut snd_pcm_substream, frames: snd_pcm_sframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_plug_slave_format(f: snd_pcm_format_t, m: *const snd_mask) -> snd_pcm_format_t;
    fn snd_pcm_kernel_ioctl(s: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int;
    fn snd_pcm_get_state(s: *mut snd_pcm_substream) -> snd_pcm_state_t;
    fn __snd_pcm_lib_xfer(s: *mut snd_pcm_substream, p: *mut c_void, interleaved: bool_t, frames: snd_pcm_uframes_t, in_kernel: c_int) -> snd_pcm_sframes_t;
    fn snd_pcm_kernel_writev(s: *mut snd_pcm_substream, bufs: *mut *mut c_void, frames: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_kernel_readv(s: *mut snd_pcm_substream, bufs: *mut *mut c_void, frames: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_lib_write(s: *mut snd_pcm_substream, buf: *const c_void, frames: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_lib_writev(s: *mut snd_pcm_substream, bufs: *const *const c_void, frames: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_playback_avail(r: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    fn snd_pcm_capture_avail(r: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    fn snd_pcm_mmap_data(s: *mut snd_pcm_substream, f: *mut file, a: *mut vm_area_struct) -> c_int;
    fn snd_pcm_runtime_buffer_set_silence(r: *mut snd_pcm_runtime) -> c_int;
    fn snd_pcm_plugin_free(p: *mut snd_pcm_plugin);
    fn snd_pcm_plug_format_plugins(s: *mut snd_pcm_substream, p: *mut snd_pcm_hw_params, sp: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_plugin_build_io(s: *mut snd_pcm_substream, sp: *mut snd_pcm_hw_params, p: *mut *mut snd_pcm_plugin) -> c_int;
    fn snd_pcm_plug_alloc(s: *mut snd_pcm_substream, size: snd_pcm_sframes_t) -> c_int;
    fn snd_pcm_plug_client_channels_buf(s: *mut snd_pcm_substream, buf: *mut c_char, frames: snd_pcm_sframes_t, ch: *mut *mut snd_pcm_plugin_channel) -> snd_pcm_sframes_t;
    fn snd_pcm_plug_write_transfer(s: *mut snd_pcm_substream, ch: *mut snd_pcm_plugin_channel, frames: snd_pcm_sframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_plug_read_transfer(s: *mut snd_pcm_substream, ch: *mut snd_pcm_plugin_channel, frames: snd_pcm_sframes_t) -> snd_pcm_sframes_t;
    fn mutex_lock_interruptible(m: *mut mutex) -> c_int;
    fn mutex_trylock(m: *mut mutex) -> c_int;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn mutex_init(m: *mut mutex);
    fn atomic_read(a: *const atomic_t) -> c_int;
    fn atomic_inc(a: *mut atomic_t);
    fn atomic_dec(a: *mut atomic_t);
    fn atomic_set(a: *mut atomic_t, v: c_int);
    fn kvfree(p: *mut c_void);
    fn kvzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn put_user(v: c_int, p: *mut c_int) -> c_int;
    fn get_user(v: *mut c_int, p: *const c_int) -> c_int;
    fn signal_pending(t: *mut task_struct) -> c_int;
    fn init_waitqueue_entry(w: *mut wait_queue_entry_t, t: *mut task_struct);
    fn add_wait_queue(h: *mut wait_queue_head_t, w: *mut wait_queue_entry_t);
    fn remove_wait_queue(h: *mut wait_queue_head_t, w: *mut wait_queue_entry_t);
    fn set_current_state(s: c_int);
    fn schedule_timeout(t: c_long) -> c_long;
    fn schedule();
    fn poll_wait(f: *mut file, h: *mut wait_queue_head_t, p: *mut poll_table);
    fn vm_flags_set(a: *mut vm_area_struct, flags: c_ulong);
    fn nonseekable_open(i: *mut inode, f: *mut file) -> c_int;
    fn iminor(i: *mut inode) -> c_int;
    fn snd_lookup_oss_minor_data(minor: c_int, typ: c_int) -> *mut snd_pcm;
    fn snd_card_file_add(c: *mut snd_card, f: *mut file) -> c_int;
    fn snd_card_file_remove(c: *mut snd_card, f: *mut file) -> c_int;
    fn try_module_get(m: *mut c_void) -> c_int;
    fn module_put(m: *mut c_void);
    fn snd_card_unref(c: *mut snd_card);
    fn snd_pcm_open_substream(p: *mut snd_pcm, stream: c_int, f: *mut file, s: *mut *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_release_substream(s: *mut snd_pcm_substream);
    fn wake_up(h: *mut wait_queue_head_t);
    fn snd_register_oss_device(t: c_int, c: *mut snd_card, i: c_int, fops: *const file_operations, data: *mut c_void) -> c_int;
    fn snd_unregister_oss_device(t: c_int, c: *mut snd_card, i: c_int) -> c_int;
    fn snd_pcm_notify(n: *mut snd_pcm_notify, free: c_int) -> c_int;
    fn rounddown_pow_of_two(v: ssize_t) -> ssize_t;
    fn roundup_pow_of_two(v: ssize_t) -> ssize_t;
    fn array_size(a: ssize_t, b: ssize_t) -> ssize_t;
    fn div_u64(a: u64_t, b: u64_t) -> u64_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn simple_strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn snd_info_get_line(b: *mut snd_info_buffer, line: *mut c_char, len: size_t) -> c_int;
    fn snd_info_get_str(dst: *mut c_char, src: *const c_char, len: size_t) -> *const c_char;
    fn snd_info_create_card_entry(card: *mut snd_card, name: *const c_char, root: *mut c_void) -> *mut snd_info_entry;
    fn snd_info_register(e: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(e: *mut snd_info_entry);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pcm_dbg(pcm: *mut snd_pcm, fmt: *const c_char, ...);
    fn pcm_err(pcm: *mut snd_pcm, fmt: *const c_char, ...);
    fn snd_BUG_ON(v: bool_t) -> c_int;
}

static mut dsp_map: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut adsp_map: [c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];
static mut nonblock_open: bool_t = true;

unsafe fn kmalloc_obj<T>() -> *mut T {
    kmalloc(mem::size_of::<T>(), GFP_KERNEL) as *mut T
}
unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn snd_interval_refine_min_rs(i: *mut snd_interval, min: c_uint, openmin: c_int) -> c_int {
    let mut changed = 0;
    if (*i).min < min {
        (*i).min = min;
        (*i).openmin = openmin as c_uint;
        changed = 1;
    } else if (*i).min == min && (*i).openmin == 0 && openmin != 0 {
        (*i).openmin = 1;
        changed = 1;
    }
    if (*i).integer != 0 && (*i).openmin != 0 {
        (*i).min = (*i).min.wrapping_add(1);
        (*i).openmin = 0;
    }
    if snd_interval_checkempty(i) != 0 {
        snd_interval_none(i);
        return -EINVAL;
    }
    changed
}

unsafe fn snd_interval_refine_max_rs(i: *mut snd_interval, max: c_uint, openmax: c_int) -> c_int {
    let mut changed = 0;
    if (*i).max > max {
        (*i).max = max;
        (*i).openmax = openmax as c_uint;
        changed = 1;
    } else if (*i).max == max && (*i).openmax == 0 && openmax != 0 {
        (*i).openmax = 1;
        changed = 1;
    }
    if (*i).integer != 0 && (*i).openmax != 0 {
        (*i).max = (*i).max.wrapping_sub(1);
        (*i).openmax = 0;
    }
    if snd_interval_checkempty(i) != 0 {
        snd_interval_none(i);
        return -EINVAL;
    }
    changed
}

unsafe fn snd_interval_refine_set_rs(i: *mut snd_interval, val: c_uint) -> c_int {
    let mut t: snd_interval = mem::zeroed();
    t.empty = 0;
    t.min = val;
    t.max = val;
    t.openmin = 0;
    t.openmax = 0;
    t.integer = 1;
    snd_interval_refine(i, &t)
}

unsafe fn snd_pcm_hw_param_value_min_rs(params: *const snd_pcm_hw_params, var: snd_pcm_hw_param_t, dir: *mut c_int) -> c_uint {
    if hw_is_mask(var) != 0 {
        if !dir.is_null() { *dir = 0; }
        return snd_mask_min(hw_param_mask_c(params, var));
    }
    if hw_is_interval(var) != 0 {
        let i = hw_param_interval_c(params, var);
        if !dir.is_null() { *dir = (*i).openmin as c_int; }
        return snd_interval_min(i);
    }
    (-EINVAL) as c_uint
}

unsafe fn snd_pcm_hw_param_value_max_rs(params: *const snd_pcm_hw_params, var: snd_pcm_hw_param_t, dir: *mut c_int) -> c_int {
    if hw_is_mask(var) != 0 {
        if !dir.is_null() { *dir = 0; }
        return snd_mask_max(hw_param_mask_c(params, var)) as c_int;
    }
    if hw_is_interval(var) != 0 {
        let i = hw_param_interval_c(params, var);
        if !dir.is_null() { *dir = -((*i).openmax as c_int); }
        return snd_interval_max(i) as c_int;
    }
    -EINVAL
}

unsafe fn _snd_pcm_hw_param_mask(params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, val: *const snd_mask) -> c_int {
    let changed = snd_mask_refine(hw_param_mask(params, var), val);
    if changed > 0 {
        (*params).cmask |= 1u32 << var;
        (*params).rmask |= 1u32 << var;
    }
    changed
}

unsafe fn snd_pcm_hw_param_mask_rs(pcm: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, val: *const snd_mask) -> c_int {
    let changed = _snd_pcm_hw_param_mask(params, var, val);
    if changed < 0 { return changed; }
    if (*params).rmask != 0 {
        let err = snd_pcm_hw_refine(pcm, params);
        if err < 0 { return err; }
    }
    0
}

unsafe fn _snd_pcm_hw_param_min(params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, mut val: c_uint, dir: c_int) -> c_int {
    let mut open = 0;
    if dir != 0 {
        if dir > 0 {
            open = 1;
        } else if val > 0 {
            open = 1;
            val = val.wrapping_sub(1);
        }
    }
    let changed;
    if hw_is_mask(var) != 0 {
        changed = snd_mask_refine_min(hw_param_mask(params, var), val.wrapping_add((open != 0) as c_uint));
    } else if hw_is_interval(var) != 0 {
        changed = snd_interval_refine_min_rs(hw_param_interval(params, var), val, open);
    } else {
        return -EINVAL;
    }
    if changed > 0 {
        (*params).cmask |= 1u32 << var;
        (*params).rmask |= 1u32 << var;
    }
    changed
}

unsafe fn snd_pcm_hw_param_min_rs(pcm: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, val: c_uint, dir: *mut c_int) -> c_int {
    let changed = _snd_pcm_hw_param_min(params, var, val, if dir.is_null() { 0 } else { *dir });
    if changed < 0 { return changed; }
    if (*params).rmask != 0 {
        let err = snd_pcm_hw_refine(pcm, params);
        if err < 0 { return err; }
    }
    snd_pcm_hw_param_value_min_rs(params, var, dir) as c_int
}

unsafe fn _snd_pcm_hw_param_max(params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, mut val: c_uint, dir: c_int) -> c_int {
    let mut open = 0;
    if dir != 0 {
        if dir < 0 {
            open = 1;
        } else {
            open = 1;
            val = val.wrapping_add(1);
        }
    }
    let changed;
    if hw_is_mask(var) != 0 {
        if val == 0 && open != 0 {
            snd_mask_none(hw_param_mask(params, var));
            changed = -EINVAL;
        } else {
            changed = snd_mask_refine_max(hw_param_mask(params, var), val.wrapping_sub((open != 0) as c_uint));
        }
    } else if hw_is_interval(var) != 0 {
        changed = snd_interval_refine_max_rs(hw_param_interval(params, var), val, open);
    } else {
        return -EINVAL;
    }
    if changed > 0 {
        (*params).cmask |= 1u32 << var;
        (*params).rmask |= 1u32 << var;
    }
    changed
}

unsafe fn snd_pcm_hw_param_max_rs(pcm: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, val: c_uint, dir: *mut c_int) -> c_int {
    let changed = _snd_pcm_hw_param_max(params, var, val, if dir.is_null() { 0 } else { *dir });
    if changed < 0 { return changed; }
    if (*params).rmask != 0 {
        let err = snd_pcm_hw_refine(pcm, params);
        if err < 0 { return err; }
    }
    snd_pcm_hw_param_value_max_rs(params, var, dir)
}

unsafe fn boundary_sub(a: c_int, mut adir: c_int, b: c_int, mut bdir: c_int, c: *mut c_int, cdir: *mut c_int) -> c_int {
    adir = if adir < 0 { -1 } else if adir > 0 { 1 } else { 0 };
    bdir = if bdir < 0 { -1 } else if bdir > 0 { 1 } else { 0 };
    *c = a.wrapping_sub(b);
    *cdir = adir - bdir;
    if *cdir == -2 { *c = (*c).wrapping_sub(1); } else if *cdir == 2 { *c = (*c).wrapping_add(1); }
    0
}

unsafe fn boundary_lt(mut a: c_uint, mut adir: c_int, mut b: c_uint, mut bdir: c_int) -> c_int {
    if adir < 0 { a = a.wrapping_sub(1); adir = 1; } else if adir > 0 { adir = 1; }
    if bdir < 0 { b = b.wrapping_sub(1); bdir = 1; } else if bdir > 0 { bdir = 1; }
    (a < b || (a == b && adir < bdir)) as c_int
}

/* Return 1 if min is nearer to best than max */
unsafe fn boundary_nearer(min: c_int, mindir: c_int, best: c_int, bestdir: c_int, max: c_int, maxdir: c_int) -> c_int {
    let mut dmin = 0;
    let mut dmindir = 0;
    let mut dmax = 0;
    let mut dmaxdir = 0;
    boundary_sub(best, bestdir, min, mindir, &mut dmin, &mut dmindir);
    boundary_sub(max, maxdir, best, bestdir, &mut dmax, &mut dmaxdir);
    boundary_lt(dmin as c_uint, dmindir, dmax as c_uint, dmaxdir)
}

unsafe fn snd_pcm_hw_param_near_rs(pcm: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, mut best: c_uint, dir: *mut c_int) -> c_int {
    let mut last = 0;
    let mut mindir;
    let mut maxdir;
    let valdir = if dir.is_null() { 0 } else { *dir };
    if best > c_int::MAX as c_uint { best = c_int::MAX as c_uint; }
    let saved_min = best;
    let mut min = best as c_int;
    let mut max = best as c_int;
    mindir = valdir;
    maxdir = valdir;
    if maxdir > 0 { maxdir = 0; } else if maxdir == 0 { maxdir = -1; } else { maxdir = 1; max -= 1; }
    let save = kmalloc_obj::<snd_pcm_hw_params>();
    if save.is_null() { return -ENOMEM; }
    *save = ptr::read(params);
    min = snd_pcm_hw_param_min_rs(pcm, params, var, min as c_uint, &mut mindir);
    if min >= 0 {
        if max < 0 { goto_end: {} }
        if !((max < 0) || (min as c_uint == saved_min && mindir == valdir)) {
            let params1 = kmalloc_obj::<snd_pcm_hw_params>();
            if params1.is_null() { kfree(save as *mut c_void); return -ENOMEM; }
            *params1 = ptr::read(save);
            max = snd_pcm_hw_param_max_rs(pcm, params1, var, max as c_uint, &mut maxdir);
            if max >= 0 && boundary_nearer(max, maxdir, best as c_int, valdir, min, mindir) != 0 {
                *params = ptr::read(params1);
                last = 1;
            }
            kfree(params1 as *mut c_void);
        }
    } else {
        *params = ptr::read(save);
        max = snd_pcm_hw_param_max_rs(pcm, params, var, max as c_uint, &mut maxdir);
        if max < 0 { kfree(save as *mut c_void); return max; }
        last = 1;
    }
    kfree(save as *mut c_void);
    if last != 0 { snd_pcm_hw_param_last(pcm, params, var, dir) } else { snd_pcm_hw_param_first(pcm, params, var, dir) }
}

unsafe fn _snd_pcm_hw_param_set(params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, mut val: c_uint, dir: c_int) -> c_int {
    let changed;
    if hw_is_mask(var) != 0 {
        let m = hw_param_mask(params, var);
        if val == 0 && dir < 0 {
            snd_mask_none(m);
            changed = -EINVAL;
        } else {
            if dir > 0 { val = val.wrapping_add(1); } else if dir < 0 { val = val.wrapping_sub(1); }
            changed = snd_mask_refine_set(m, val);
        }
    } else if hw_is_interval(var) != 0 {
        let i = hw_param_interval(params, var);
        if val == 0 && dir < 0 {
            snd_interval_none(i);
            changed = -EINVAL;
        } else if dir == 0 {
            changed = snd_interval_refine_set_rs(i, val);
        } else {
            let mut t: snd_interval = mem::zeroed();
            t.openmin = 1;
            t.openmax = 1;
            t.empty = 0;
            t.integer = 0;
            if dir < 0 { t.min = val.wrapping_sub(1); t.max = val; } else { t.min = val; t.max = val.wrapping_add(1); }
            changed = snd_interval_refine(i, &t);
        }
    } else {
        return -EINVAL;
    }
    if changed > 0 {
        (*params).cmask |= 1u32 << var;
        (*params).rmask |= 1u32 << var;
    }
    changed
}

unsafe fn snd_pcm_hw_param_set_rs(pcm: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t, val: c_uint, dir: c_int) -> c_int {
    let changed = _snd_pcm_hw_param_set(params, var, val, dir);
    if changed < 0 { return changed; }
    if (*params).rmask != 0 {
        let err = snd_pcm_hw_refine(pcm, params);
        if err < 0 { return err; }
    }
    snd_pcm_hw_param_value(params, var, ptr::null_mut())
}

unsafe fn _snd_pcm_hw_param_setinteger(params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t) -> c_int {
    let changed = snd_interval_setinteger(hw_param_interval(params, var));
    if changed > 0 {
        (*params).cmask |= 1u32 << var;
        (*params).rmask |= 1u32 << var;
    }
    changed
}

/* plugin */
/* CONFIG_SND_PCM_OSS_PLUGINS */
unsafe fn snd_pcm_oss_plugin_clear(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let mut plugin = (*runtime).oss.plugin_first;
    while !plugin.is_null() {
        let next = (*plugin).next;
        snd_pcm_plugin_free(plugin);
        plugin = next;
    }
    (*runtime).oss.plugin_first = ptr::null_mut();
    (*runtime).oss.plugin_last = ptr::null_mut();
    0
}

unsafe fn snd_pcm_plugin_insert(plugin: *mut snd_pcm_plugin) -> c_int {
    let runtime = (**plugin).plug.runtime;
    (*plugin).next = (*runtime).oss.plugin_first;
    (*plugin).prev = ptr::null_mut();
    if !(*runtime).oss.plugin_first.is_null() {
        (*(*runtime).oss.plugin_first).prev = plugin;
        (*runtime).oss.plugin_first = plugin;
    } else {
        (*runtime).oss.plugin_last = plugin;
        (*runtime).oss.plugin_first = plugin;
    }
    0
}

pub unsafe extern "C" fn snd_pcm_plugin_append(plugin: *mut snd_pcm_plugin) -> c_int {
    let runtime = (**plugin).plug.runtime;
    (*plugin).next = ptr::null_mut();
    (*plugin).prev = (*runtime).oss.plugin_last;
    if !(*runtime).oss.plugin_last.is_null() {
        (*(*runtime).oss.plugin_last).next = plugin;
        (*runtime).oss.plugin_last = plugin;
    } else {
        (*runtime).oss.plugin_last = plugin;
        (*runtime).oss.plugin_first = plugin;
    }
    0
}

unsafe fn snd_pcm_oss_bytes(substream: *mut snd_pcm_substream, frames: c_long) -> c_long {
    let runtime = (*substream).runtime;
    let buffer_size = snd_pcm_lib_buffer_bytes(substream);
    let bytes = frames_to_bytes(runtime, frames);
    if buffer_size == (*runtime).oss.buffer_bytes { return bytes as c_long; }
    if mem::size_of::<c_long>() >= 8 {
        ((*runtime).oss.buffer_bytes * bytes / buffer_size) as c_long
    } else {
        let bsize = ((*runtime).oss.buffer_bytes as u64_t) * (bytes as u64_t);
        div_u64(bsize, buffer_size as u64_t) as c_long
    }
}

unsafe fn snd_pcm_alsa_frames(substream: *mut snd_pcm_substream, bytes: c_long) -> c_long {
    let runtime = (*substream).runtime;
    let buffer_size = snd_pcm_lib_buffer_bytes(substream);
    if buffer_size == (*runtime).oss.buffer_bytes {
        bytes_to_frames(runtime, bytes as ssize_t)
    } else {
        bytes_to_frames(runtime, (buffer_size * bytes as ssize_t) / (*runtime).oss.buffer_bytes)
    }
}

unsafe fn get_hw_ptr_period(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t {
    (*runtime).hw_ptr_interrupt
}

unsafe fn snd_pcm_oss_format_from(format: c_int) -> snd_pcm_format_t {
    match format {
        AFMT_MU_LAW => SNDRV_PCM_FORMAT_MU_LAW,
        AFMT_A_LAW => SNDRV_PCM_FORMAT_A_LAW,
        AFMT_IMA_ADPCM => SNDRV_PCM_FORMAT_IMA_ADPCM,
        AFMT_U8 => SNDRV_PCM_FORMAT_U8,
        AFMT_S16_LE => SNDRV_PCM_FORMAT_S16_LE,
        AFMT_S16_BE => SNDRV_PCM_FORMAT_S16_BE,
        AFMT_S8 => SNDRV_PCM_FORMAT_S8,
        AFMT_U16_LE => SNDRV_PCM_FORMAT_U16_LE,
        AFMT_U16_BE => SNDRV_PCM_FORMAT_U16_BE,
        AFMT_MPEG => SNDRV_PCM_FORMAT_MPEG,
        AFMT_S32_LE => SNDRV_PCM_FORMAT_S32_LE,
        AFMT_S32_BE => SNDRV_PCM_FORMAT_S32_BE,
        AFMT_S24_LE => SNDRV_PCM_FORMAT_S24_LE,
        AFMT_S24_BE => SNDRV_PCM_FORMAT_S24_BE,
        AFMT_S24_PACKED => SNDRV_PCM_FORMAT_S24_3LE,
        AFMT_FLOAT => SNDRV_PCM_FORMAT_FLOAT,
        AFMT_SPDIF_RAW => SNDRV_PCM_FORMAT_IEC958_SUBFRAME,
        _ => SNDRV_PCM_FORMAT_U8,
    }
}

unsafe fn snd_pcm_oss_format_to(format: snd_pcm_format_t) -> c_int {
    match format {
        SNDRV_PCM_FORMAT_MU_LAW => AFMT_MU_LAW,
        SNDRV_PCM_FORMAT_A_LAW => AFMT_A_LAW,
        SNDRV_PCM_FORMAT_IMA_ADPCM => AFMT_IMA_ADPCM,
        SNDRV_PCM_FORMAT_U8 => AFMT_U8,
        SNDRV_PCM_FORMAT_S16_LE => AFMT_S16_LE,
        SNDRV_PCM_FORMAT_S16_BE => AFMT_S16_BE,
        SNDRV_PCM_FORMAT_S8 => AFMT_S8,
        SNDRV_PCM_FORMAT_U16_LE => AFMT_U16_LE,
        SNDRV_PCM_FORMAT_U16_BE => AFMT_U16_BE,
        SNDRV_PCM_FORMAT_MPEG => AFMT_MPEG,
        SNDRV_PCM_FORMAT_S32_LE => AFMT_S32_LE,
        SNDRV_PCM_FORMAT_S32_BE => AFMT_S32_BE,
        SNDRV_PCM_FORMAT_S24_LE => AFMT_S24_LE,
        SNDRV_PCM_FORMAT_S24_BE => AFMT_S24_BE,
        SNDRV_PCM_FORMAT_S24_3LE => AFMT_S24_PACKED,
        SNDRV_PCM_FORMAT_FLOAT => AFMT_FLOAT,
        SNDRV_PCM_FORMAT_IEC958_SUBFRAME => AFMT_SPDIF_RAW,
        _ => -EINVAL,
    }
}

unsafe fn snd_pcm_oss_period_size(substream: *mut snd_pcm_substream, oss_params: *mut snd_pcm_hw_params, slave_params: *mut snd_pcm_hw_params) -> c_int {
    let runtime = (*substream).runtime;
    let oss_frame_size = (snd_pcm_format_physical_width(params_format(oss_params)) * params_channels(oss_params) / 8) as ssize_t;
    let mut oss_buffer_size = snd_pcm_hw_param_value_max_rs(slave_params, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, ptr::null_mut()) as ssize_t;
    if oss_buffer_size <= 0 { return -EINVAL; }
    oss_buffer_size = snd_pcm_plug_client_size(substream, oss_buffer_size * oss_frame_size);
    if oss_buffer_size <= 0 { return -EINVAL; }
    oss_buffer_size = rounddown_pow_of_two(oss_buffer_size);
    if atomic_read(&(*substream).mmap_count) != 0 && oss_buffer_size > (*runtime).oss.mmap_bytes {
        oss_buffer_size = (*runtime).oss.mmap_bytes;
    }
    let mut oss_period_size: ssize_t;
    if (*substream).oss.setup.period_size > 16 {
        oss_period_size = (*substream).oss.setup.period_size as ssize_t;
    } else if (*runtime).oss.fragshift != 0 {
        oss_period_size = 1isize << (*runtime).oss.fragshift;
        if oss_period_size > oss_buffer_size / 2 { oss_period_size = oss_buffer_size / 2; }
    } else {
        let bytes_per_sec = (params_rate(oss_params) * snd_pcm_format_physical_width(params_format(oss_params)) * params_channels(oss_params) / 8) as ssize_t;
        oss_period_size = oss_buffer_size;
        while { oss_period_size /= 2; oss_period_size > bytes_per_sec } {}
        let mut sd = if (*runtime).oss.subdivision == 0 { 4 } else { (*runtime).oss.subdivision };
        if (*runtime).oss.subdivision == 0 {
            if oss_period_size / sd as ssize_t > 4096 { sd *= 2; }
            if oss_period_size / sd as ssize_t < 4096 { sd = 1; }
        }
        oss_period_size /= sd as ssize_t;
        if oss_period_size < 16 { oss_period_size = 16; }
    }
    let mut min_period_size = snd_pcm_plug_client_size(substream, snd_pcm_hw_param_value_min_rs(slave_params, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, ptr::null_mut()) as c_long);
    if min_period_size > 0 {
        min_period_size *= oss_frame_size;
        min_period_size = roundup_pow_of_two(min_period_size);
        if oss_period_size < min_period_size { oss_period_size = min_period_size; }
    }
    let mut max_period_size = snd_pcm_plug_client_size(substream, snd_pcm_hw_param_value_max_rs(slave_params, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, ptr::null_mut()) as c_long);
    if max_period_size > 0 {
        max_period_size *= oss_frame_size;
        max_period_size = rounddown_pow_of_two(max_period_size);
        if oss_period_size > max_period_size { oss_period_size = max_period_size; }
    }
    let mut oss_periods = oss_buffer_size / oss_period_size;
    if (*substream).oss.setup.periods > 1 { oss_periods = (*substream).oss.setup.periods as ssize_t; }
    let mut s = snd_pcm_hw_param_value_max_rs(slave_params, SNDRV_PCM_HW_PARAM_PERIODS, ptr::null_mut()) as ssize_t;
    if s > 0 && (*runtime).oss.maxfrags != 0 && s > (*runtime).oss.maxfrags as ssize_t { s = (*runtime).oss.maxfrags as ssize_t; }
    if oss_periods > s { oss_periods = s; }
    s = snd_pcm_hw_param_value_min_rs(slave_params, SNDRV_PCM_HW_PARAM_PERIODS, ptr::null_mut()) as ssize_t;
    if s < 2 { s = 2; }
    if oss_periods < s { oss_periods = s; }
    while oss_period_size * oss_periods > oss_buffer_size { oss_period_size /= 2; }
    if oss_period_size < 16 { return -EINVAL; }
    /* don't allocate too large period; 1MB period must be enough */
    if oss_period_size > 1024 * 1024 { return -ENOMEM; }
    (*runtime).oss.period_bytes = oss_period_size;
    (*runtime).oss.period_frames = 1;
    (*runtime).oss.periods = oss_periods;
    0
}

unsafe fn choose_rate(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, best_rate: c_uint) -> c_int {
    let save = kmalloc_obj::<snd_pcm_hw_params>();
    if save.is_null() { return -ENOMEM; }
    *save = ptr::read(params);
    let it = hw_param_interval_c(save, SNDRV_PCM_HW_PARAM_RATE);
    let mut rate = best_rate;
    loop {
        if (*it).max < rate || ((*it).max == rate) && (*it).openmax != 0) { break; }
        if (*it).min < rate || ((*it).min == rate) && (*it).openmin == 0) {
            let ret = snd_pcm_hw_param_set_rs(substream, params, SNDRV_PCM_HW_PARAM_RATE, rate, 0);
            if ret == rate as c_int { kfree(save as *mut c_void); return rate as c_int; }
            *params = ptr::read(save);
        }
        let prev = rate;
        rate = rate.wrapping_add(best_rate);
        if rate <= prev { break; }
    }
    kfree(save as *mut c_void);
    snd_pcm_hw_param_near_rs(substream, params, SNDRV_PCM_HW_PARAM_RATE, best_rate, ptr::null_mut())
}

/* parameter locking: returns immediately if tried during streaming */
unsafe fn lock_params(runtime: *mut snd_pcm_runtime) -> c_int {
    if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 { return -ERESTARTSYS; }
    if atomic_read(&(*runtime).oss.rw_ref) != 0 {
        mutex_unlock(&mut (*runtime).oss.params_lock);
        return -EBUSY;
    }
    0
}

unsafe fn unlock_params(runtime: *mut snd_pcm_runtime) {
    mutex_unlock(&mut (*runtime).oss.params_lock);
}

unsafe fn snd_pcm_oss_release_buffers(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    kvfree((*runtime).oss.buffer as *mut c_void);
    (*runtime).oss.buffer = ptr::null_mut();
    snd_pcm_oss_plugin_clear(substream);
}

/* call with params_lock held */
unsafe fn snd_pcm_oss_change_params_locked(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    if (*runtime).oss.params == 0 { return 0; }
    let sw_params = kzalloc_obj::<snd_pcm_sw_params>();
    let params = kmalloc_obj::<snd_pcm_hw_params>();
    let sparams = kmalloc_obj::<snd_pcm_hw_params>();
    let mut err: c_int;
    if sw_params.is_null() || params.is_null() || sparams.is_null() { err = -ENOMEM; goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    let direct = if atomic_read(&(*substream).mmap_count) != 0 { 1 } else { (*substream).oss.setup.direct as c_int };
    _snd_pcm_hw_params_any(sparams);
    _snd_pcm_hw_param_setinteger(sparams, SNDRV_PCM_HW_PARAM_PERIODS);
    _snd_pcm_hw_param_min(sparams, SNDRV_PCM_HW_PARAM_PERIODS, 2, 0);
    let mut mask: snd_mask = mem::zeroed();
    snd_mask_none(&mut mask);
    if atomic_read(&(*substream).mmap_count) != 0 {
        snd_mask_set(&mut mask, SNDRV_PCM_ACCESS_MMAP_INTERLEAVED);
    } else {
        snd_mask_set(&mut mask, SNDRV_PCM_ACCESS_RW_INTERLEAVED);
        if direct == 0 { snd_mask_set(&mut mask, SNDRV_PCM_ACCESS_RW_NONINTERLEAVED); }
    }
    err = snd_pcm_hw_param_mask_rs(substream, sparams, SNDRV_PCM_HW_PARAM_ACCESS, &mask);
    if err < 0 { err = -EINVAL; goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    err = choose_rate(substream, sparams, (*runtime).oss.rate);
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    err = snd_pcm_hw_param_near_rs(substream, sparams, SNDRV_PCM_HW_PARAM_CHANNELS, (*runtime).oss.channels, ptr::null_mut());
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    let format = snd_pcm_oss_format_from((*runtime).oss.format);
    let sformat_mask = hw_param_mask_c(sparams, SNDRV_PCM_HW_PARAM_FORMAT);
    let mut sformat = if direct != 0 { format } else { snd_pcm_plug_slave_format(format, sformat_mask) };
    if sformat < 0 || snd_mask_test_format(sformat_mask, sformat) == 0 {
        sformat = 0;
        while sformat < 32 {
            if snd_mask_test_format(sformat_mask, sformat) != 0 && snd_pcm_oss_format_to(sformat) >= 0 { break; }
            sformat += 1;
        }
        if sformat >= 32 { err = -EINVAL; goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    }
    err = _snd_pcm_hw_param_set(sparams, SNDRV_PCM_HW_PARAM_FORMAT, sformat as c_uint, 0);
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    if direct != 0 {
        memcpy(params as *mut c_void, sparams as *const c_void, mem::size_of::<snd_pcm_hw_params>());
    } else {
        _snd_pcm_hw_params_any(params);
        _snd_pcm_hw_param_set(params, SNDRV_PCM_HW_PARAM_ACCESS, SNDRV_PCM_ACCESS_RW_INTERLEAVED, 0);
        _snd_pcm_hw_param_set(params, SNDRV_PCM_HW_PARAM_FORMAT, snd_pcm_oss_format_from((*runtime).oss.format) as c_uint, 0);
        _snd_pcm_hw_param_set(params, SNDRV_PCM_HW_PARAM_CHANNELS, (*runtime).oss.channels, 0);
        _snd_pcm_hw_param_set(params, SNDRV_PCM_HW_PARAM_RATE, (*runtime).oss.rate, 0);
    }
    let oss_frame_size = (snd_pcm_format_physical_width(params_format(params)) * params_channels(params) / 8) as ssize_t;
    err = snd_pcm_oss_period_size(substream, params, sparams);
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    let n = snd_pcm_plug_slave_size(substream, (*runtime).oss.period_bytes / oss_frame_size);
    err = snd_pcm_hw_param_near_rs(substream, sparams, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, n as c_uint, ptr::null_mut());
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    err = snd_pcm_hw_param_near_rs(substream, sparams, SNDRV_PCM_HW_PARAM_PERIODS, (*runtime).oss.periods as c_uint, ptr::null_mut());
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DROP, ptr::null_mut());
    err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_HW_PARAMS, sparams as *mut c_void);
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    snd_pcm_oss_plugin_clear(substream);
    if direct == 0 {
        err = snd_pcm_plug_format_plugins(substream, params, sparams);
        if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
        if !(*runtime).oss.plugin_first.is_null() {
            let mut plugin: *mut snd_pcm_plugin = ptr::null_mut();
            err = snd_pcm_plugin_build_io(substream, sparams, &mut plugin);
            if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
            err = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int { snd_pcm_plugin_append(plugin) } else { snd_pcm_plugin_insert(plugin) };
            if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
        }
    }
    (*sw_params).start_threshold = if (*runtime).oss.trigger != 0 { 1 } else { (*runtime).boundary };
    (*sw_params).stop_threshold = if atomic_read(&(*substream).mmap_count) != 0 || (*substream).stream == SNDRV_PCM_STREAM_CAPTURE as c_int { (*runtime).boundary } else { (*runtime).buffer_size };
    (*sw_params).tstamp_mode = SNDRV_PCM_TSTAMP_NONE;
    (*sw_params).period_step = 1;
    (*sw_params).avail_min = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int { 1 } else { (*runtime).period_size };
    if atomic_read(&(*substream).mmap_count) != 0 || (*substream).oss.setup.nosilence != 0 {
        (*sw_params).silence_threshold = 0;
        (*sw_params).silence_size = 0;
    } else {
        let mut frames = (*runtime).period_size + 16;
        if frames > (*runtime).buffer_size { frames = (*runtime).buffer_size; }
        (*sw_params).silence_threshold = frames;
        (*sw_params).silence_size = frames;
    }
    err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_SW_PARAMS, sw_params as *mut c_void);
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    (*runtime).oss.periods = params_periods(sparams) as ssize_t;
    let mut oss_period_size = snd_pcm_plug_client_size(substream, params_period_size(sparams) as c_long);
    if oss_period_size < 0 { err = -EINVAL; goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    if !(*runtime).oss.plugin_first.is_null() {
        err = snd_pcm_plug_alloc(substream, oss_period_size);
        if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    }
    oss_period_size = array_size(oss_period_size, oss_frame_size);
    let oss_buffer_size = array_size(oss_period_size, (*runtime).oss.periods);
    if oss_buffer_size <= 0 { err = -EINVAL; goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    (*runtime).oss.period_bytes = oss_period_size;
    (*runtime).oss.buffer_bytes = oss_buffer_size;
    (*runtime).oss.format = snd_pcm_oss_format_to(params_format(params));
    (*runtime).oss.channels = params_channels(params);
    (*runtime).oss.rate = params_rate(params);
    kvfree((*runtime).oss.buffer as *mut c_void);
    (*runtime).oss.buffer = kvzalloc((*runtime).oss.period_bytes as size_t, GFP_KERNEL) as *mut c_char;
    if (*runtime).oss.buffer.is_null() { err = -ENOMEM; goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    (*runtime).oss.params = 0;
    (*runtime).oss.prepare = 1;
    (*runtime).oss.buffer_used = 0;
    err = snd_pcm_runtime_buffer_set_silence(runtime);
    if err < 0 { goto_failure(runtime, substream, sw_params, params, sparams, err); return err; }
    (*runtime).oss.period_frames = snd_pcm_alsa_frames(substream, oss_period_size as c_long) as snd_pcm_uframes_t;
    goto_failure(runtime, substream, sw_params, params, sparams, 0)
}

unsafe fn goto_failure(_runtime: *mut snd_pcm_runtime, substream: *mut snd_pcm_substream, sw_params: *mut snd_pcm_sw_params, params: *mut snd_pcm_hw_params, sparams: *mut snd_pcm_hw_params, err: c_int) -> c_int {
    if err != 0 { snd_pcm_oss_release_buffers(substream); }
    kfree(sw_params as *mut c_void);
    kfree(params as *mut c_void);
    kfree(sparams as *mut c_void);
    err
}

unsafe fn snd_pcm_oss_change_params(substream: *mut snd_pcm_substream, trylock: bool_t) -> c_int {
    let runtime = (*substream).runtime;
    if trylock {
        if mutex_trylock(&mut (*runtime).oss.params_lock) == 0 { return -EAGAIN; }
    } else if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 {
        return -ERESTARTSYS;
    }
    let err = snd_pcm_oss_change_params_locked(substream);
    mutex_unlock(&mut (*runtime).oss.params_lock);
    err
}

unsafe fn snd_pcm_oss_get_active_substream(pcm_oss_file: *mut snd_pcm_oss_file, r_substream: *mut *mut snd_pcm_substream) -> c_int {
    let mut asubstream: *mut snd_pcm_substream = ptr::null_mut();
    for idx in 0..2 {
        let substream = (*pcm_oss_file).streams[idx];
        if substream.is_null() { continue; }
        if asubstream.is_null() { asubstream = substream; }
        if (*(*substream).runtime).oss.params != 0 {
            let err = snd_pcm_oss_change_params(substream, false);
            if err < 0 { return err; }
        }
    }
    if asubstream.is_null() { return -EIO; }
    if !r_substream.is_null() { *r_substream = asubstream; }
    0
}

/* call with params_lock held; PREPARE is called unconditionally */
unsafe fn snd_pcm_oss_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_PREPARE, ptr::null_mut());
    if err < 0 { return err; }
    (*runtime).oss.prepare = 0;
    (*runtime).oss.prev_hw_ptr_period = 0;
    (*runtime).oss.period_ptr = 0;
    (*runtime).oss.buffer_used = 0;
    0
}

unsafe fn snd_pcm_oss_make_ready(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    if (*runtime).oss.params != 0 {
        let err = snd_pcm_oss_change_params(substream, false);
        if err < 0 { return err; }
    }
    if (*runtime).oss.prepare != 0 {
        if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 { return -ERESTARTSYS; }
        let err = snd_pcm_oss_prepare(substream);
        mutex_unlock(&mut (*runtime).oss.params_lock);
        if err < 0 { return err; }
    }
    0
}

unsafe fn snd_pcm_oss_make_ready_locked(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    if (*runtime).oss.params != 0 {
        let err = snd_pcm_oss_change_params_locked(substream);
        if err < 0 { return err; }
    }
    if (*runtime).oss.prepare != 0 {
        let err = snd_pcm_oss_prepare(substream);
        if err < 0 { return err; }
    }
    0
}

unsafe fn snd_pcm_oss_capture_position_fixup(substream: *mut snd_pcm_substream, delay: *mut snd_pcm_sframes_t) -> c_int {
    let mut err;
    loop {
        err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DELAY, delay as *mut c_void);
        if err < 0 { break; }
        let runtime = (*substream).runtime;
        if *delay <= (*runtime).buffer_size as snd_pcm_sframes_t { break; }
        /* in case of overrun, skip whole periods like OSS/Linux driver does */
        /* until avail(delay) <= buffer_size */
        let mut frames = (*delay - (*runtime).buffer_size as snd_pcm_sframes_t) as snd_pcm_uframes_t + (*runtime).period_size - 1;
        frames /= (*runtime).period_size;
        frames *= (*runtime).period_size;
        err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_FORWARD, &mut frames as *mut _ as *mut c_void);
        if err < 0 { break; }
    }
    err
}

pub unsafe extern "C" fn snd_pcm_oss_write3(substream: *mut snd_pcm_substream, ptr_: *const c_char, frames: snd_pcm_uframes_t, in_kernel: c_int) -> snd_pcm_sframes_t {
    let runtime = (*substream).runtime;
    let mut ret;
    loop {
        let state = snd_pcm_get_state(substream);
        if state == SNDRV_PCM_STATE_XRUN || state == SNDRV_PCM_STATE_SUSPENDED {
            ret = snd_pcm_oss_prepare(substream);
            if ret < 0 { break; }
        }
        mutex_unlock(&mut (*runtime).oss.params_lock);
        ret = __snd_pcm_lib_xfer(substream, ptr_ as *mut c_void, true, frames, in_kernel) as c_int;
        mutex_lock(&mut (*runtime).oss.params_lock);
        if ret != -EPIPE && ret != -ESTRPIPE { break; }
        if snd_pcm_get_state(substream) == SNDRV_PCM_STATE_PREPARED { return -EAGAIN as snd_pcm_sframes_t; }
    }
    ret as snd_pcm_sframes_t
}

pub unsafe extern "C" fn snd_pcm_oss_read3(substream: *mut snd_pcm_substream, ptr_: *mut c_char, frames: snd_pcm_uframes_t, in_kernel: c_int) -> snd_pcm_sframes_t {
    let runtime = (*substream).runtime;
    let mut delay: snd_pcm_sframes_t = 0;
    let mut ret;
    loop {
        let state = snd_pcm_get_state(substream);
        if state == SNDRV_PCM_STATE_XRUN || state == SNDRV_PCM_STATE_SUSPENDED {
            ret = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DRAIN, ptr::null_mut());
            if ret < 0 { break; }
        } else if state == SNDRV_PCM_STATE_SETUP {
            ret = snd_pcm_oss_prepare(substream);
            if ret < 0 { break; }
        }
        ret = snd_pcm_oss_capture_position_fixup(substream, &mut delay);
        if ret < 0 { break; }
        mutex_unlock(&mut (*runtime).oss.params_lock);
        ret = __snd_pcm_lib_xfer(substream, ptr_ as *mut c_void, true, frames, in_kernel) as c_int;
        mutex_lock(&mut (*runtime).oss.params_lock);
        if ret == -EPIPE {
            if snd_pcm_get_state(substream) == SNDRV_PCM_STATE_DRAINING {
                ret = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DROP, ptr::null_mut());
                if ret < 0 { break; }
            }
            continue;
        }
        if ret != -ESTRPIPE { break; }
    }
    ret as snd_pcm_sframes_t
}

pub unsafe extern "C" fn snd_pcm_oss_writev3(substream: *mut snd_pcm_substream, bufs: *mut *mut c_void, frames: snd_pcm_uframes_t) -> snd_pcm_sframes_t {
    let mut ret;
    loop {
        let state = snd_pcm_get_state(substream);
        if state == SNDRV_PCM_STATE_XRUN || state == SNDRV_PCM_STATE_SUSPENDED {
            ret = snd_pcm_oss_prepare(substream);
            if ret < 0 { break; }
        }
        ret = snd_pcm_kernel_writev(substream, bufs, frames) as c_int;
        if ret != -EPIPE && ret != -ESTRPIPE { break; }
        if snd_pcm_get_state(substream) == SNDRV_PCM_STATE_PREPARED { return -EAGAIN as snd_pcm_sframes_t; }
    }
    ret as snd_pcm_sframes_t
}

pub unsafe extern "C" fn snd_pcm_oss_readv3(substream: *mut snd_pcm_substream, bufs: *mut *mut c_void, frames: snd_pcm_uframes_t) -> snd_pcm_sframes_t {
    let mut ret;
    loop {
        let state = snd_pcm_get_state(substream);
        if state == SNDRV_PCM_STATE_XRUN || state == SNDRV_PCM_STATE_SUSPENDED {
            ret = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DRAIN, ptr::null_mut());
            if ret < 0 { break; }
        } else if state == SNDRV_PCM_STATE_SETUP {
            ret = snd_pcm_oss_prepare(substream);
            if ret < 0 { break; }
        }
        ret = snd_pcm_kernel_readv(substream, bufs, frames) as c_int;
        if ret != -EPIPE && ret != -ESTRPIPE { break; }
    }
    ret as snd_pcm_sframes_t
}

unsafe fn snd_pcm_oss_write2(substream: *mut snd_pcm_substream, mut buf: *const c_char, mut bytes: size_t, in_kernel: c_int) -> ssize_t {
    let runtime = (*substream).runtime;
    let frames1;
    if !(*runtime).oss.plugin_first.is_null() {
        let mut channels: *mut snd_pcm_plugin_channel = ptr::null_mut();
        let oss_frame_bytes = ((*(*runtime).oss.plugin_first).src_width * (*(*runtime).oss.plugin_first).src_format.channels as usize) / 8;
        if in_kernel == 0 {
            if copy_from_user((*runtime).oss.buffer as *mut c_void, buf as *const c_void, bytes) != 0 { return -EFAULT as ssize_t; }
            buf = (*runtime).oss.buffer;
        }
        let frames = (bytes / oss_frame_bytes) as snd_pcm_sframes_t;
        frames1 = snd_pcm_plug_client_channels_buf(substream, buf as *mut c_char, frames, &mut channels);
        if frames1 < 0 { return frames1 as ssize_t; }
        let frames1b = snd_pcm_plug_write_transfer(substream, channels, frames1);
        if frames1b <= 0 { return frames1b as ssize_t; }
        bytes = (frames1b as usize) * oss_frame_bytes;
    } else {
        let frames = bytes_to_frames(runtime, bytes as ssize_t);
        let f = snd_pcm_oss_write3(substream, buf, frames as snd_pcm_uframes_t, in_kernel);
        if f <= 0 { return f as ssize_t; }
        bytes = frames_to_bytes(runtime, f) as size_t;
    }
    bytes as ssize_t
}

unsafe fn snd_pcm_oss_write1(substream: *mut snd_pcm_substream, mut buf: *const c_char, mut bytes: size_t) -> ssize_t {
    let mut xfer: size_t = 0;
    let mut tmp: ssize_t = 0;
    let runtime = (*substream).runtime;
    if atomic_read(&(*substream).mmap_count) != 0 { return -ENXIO as ssize_t; }
    atomic_inc(&mut (*runtime).oss.rw_ref);
    while bytes > 0 {
        if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 { tmp = -ERESTARTSYS as ssize_t; break; }
        tmp = snd_pcm_oss_make_ready_locked(substream) as ssize_t;
        if tmp < 0 { mutex_unlock(&mut (*runtime).oss.params_lock); break; }
        if bytes < (*runtime).oss.period_bytes as usize || (*runtime).oss.buffer_used > 0 {
            tmp = bytes as ssize_t;
            if tmp + (*runtime).oss.buffer_used > (*runtime).oss.period_bytes { tmp = (*runtime).oss.period_bytes - (*runtime).oss.buffer_used; }
            if tmp > 0 && copy_from_user((*runtime).oss.buffer.offset((*runtime).oss.buffer_used), buf as *const c_void, tmp as usize) != 0 {
                tmp = -EFAULT as ssize_t;
                mutex_unlock(&mut (*runtime).oss.params_lock);
                break;
            }
            (*runtime).oss.buffer_used += tmp;
            buf = buf.offset(tmp);
            bytes -= tmp as usize;
            xfer += tmp as usize;
            if (*substream).oss.setup.partialfrag != 0 || (*runtime).oss.buffer_used == (*runtime).oss.period_bytes {
                tmp = snd_pcm_oss_write2(substream, (*runtime).oss.buffer.offset((*runtime).oss.period_ptr), ((*runtime).oss.buffer_used - (*runtime).oss.period_ptr) as usize, 1);
                if tmp <= 0 { mutex_unlock(&mut (*runtime).oss.params_lock); break; }
                (*runtime).oss.bytes += tmp as snd_pcm_sframes_t;
                (*runtime).oss.period_ptr += tmp;
                (*runtime).oss.period_ptr %= (*runtime).oss.period_bytes;
                if (*runtime).oss.period_ptr == 0 || (*runtime).oss.period_ptr == (*runtime).oss.buffer_used { (*runtime).oss.buffer_used = 0; }
                else if ((*substream).f_flags & O_NONBLOCK) != 0 { tmp = -EAGAIN as ssize_t; mutex_unlock(&mut (*runtime).oss.params_lock); break; }
            }
        } else {
            tmp = snd_pcm_oss_write2(substream, buf, (*runtime).oss.period_bytes as usize, 0);
            if tmp <= 0 { mutex_unlock(&mut (*runtime).oss.params_lock); break; }
            (*runtime).oss.bytes += tmp as snd_pcm_sframes_t;
            buf = buf.offset(tmp);
            bytes -= tmp as usize;
            xfer += tmp as usize;
            if ((*substream).f_flags & O_NONBLOCK) != 0 && tmp != (*runtime).oss.period_bytes { tmp = -EAGAIN as ssize_t; }
        }
        mutex_unlock(&mut (*runtime).oss.params_lock);
        if tmp < 0 { break; }
        if signal_pending(current) != 0 { tmp = -ERESTARTSYS as ssize_t; break; }
        tmp = 0;
    }
    atomic_dec(&mut (*runtime).oss.rw_ref);
    if xfer > 0 { xfer as ssize_t } else { tmp }
}

unsafe fn snd_pcm_oss_read2(substream: *mut snd_pcm_substream, mut buf: *mut c_char, mut bytes: size_t, in_kernel: c_int) -> ssize_t {
    let runtime = (*substream).runtime;
    if !(*runtime).oss.plugin_first.is_null() {
        let final_dst = buf;
        let mut channels: *mut snd_pcm_plugin_channel = ptr::null_mut();
        let oss_frame_bytes = ((*(*runtime).oss.plugin_last).dst_width * (*(*runtime).oss.plugin_last).dst_format.channels as usize) / 8;
        if in_kernel == 0 { buf = (*runtime).oss.buffer; }
        let frames = (bytes / oss_frame_bytes) as snd_pcm_sframes_t;
        let mut frames1 = snd_pcm_plug_client_channels_buf(substream, buf, frames, &mut channels);
        if frames1 < 0 { return frames1 as ssize_t; }
        frames1 = snd_pcm_plug_read_transfer(substream, channels, frames1);
        if frames1 <= 0 { return frames1 as ssize_t; }
        bytes = frames1 as usize * oss_frame_bytes;
        if in_kernel == 0 && copy_to_user(final_dst as *mut c_void, buf as *const c_void, bytes) != 0 { return -EFAULT as ssize_t; }
    } else {
        let frames = bytes_to_frames(runtime, bytes as ssize_t);
        let frames1 = snd_pcm_oss_read3(substream, buf, frames as snd_pcm_uframes_t, in_kernel);
        if frames1 <= 0 { return frames1 as ssize_t; }
        bytes = frames_to_bytes(runtime, frames1) as size_t;
    }
    bytes as ssize_t
}

unsafe fn snd_pcm_oss_read1(substream: *mut snd_pcm_substream, mut buf: *mut c_char, mut bytes: size_t) -> ssize_t {
    let mut xfer: size_t = 0;
    let mut tmp: ssize_t = 0;
    let runtime = (*substream).runtime;
    if atomic_read(&(*substream).mmap_count) != 0 { return -ENXIO as ssize_t; }
    atomic_inc(&mut (*runtime).oss.rw_ref);
    while bytes > 0 {
        if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 { tmp = -ERESTARTSYS as ssize_t; break; }
        tmp = snd_pcm_oss_make_ready_locked(substream) as ssize_t;
        if tmp < 0 { mutex_unlock(&mut (*runtime).oss.params_lock); break; }
        if bytes < (*runtime).oss.period_bytes as usize || (*runtime).oss.buffer_used > 0 {
            if (*runtime).oss.buffer_used == 0 {
                tmp = snd_pcm_oss_read2(substream, (*runtime).oss.buffer, (*runtime).oss.period_bytes as usize, 1);
                if tmp <= 0 { mutex_unlock(&mut (*runtime).oss.params_lock); break; }
                (*runtime).oss.bytes += tmp as snd_pcm_sframes_t;
                (*runtime).oss.period_ptr = tmp;
                (*runtime).oss.buffer_used = tmp;
            }
            tmp = bytes as ssize_t;
            if tmp as usize > (*runtime).oss.buffer_used as usize { tmp = (*runtime).oss.buffer_used; }
            if copy_to_user(buf as *mut c_void, (*runtime).oss.buffer.offset((*runtime).oss.period_ptr - (*runtime).oss.buffer_used) as *const c_void, tmp as usize) != 0 {
                tmp = -EFAULT as ssize_t;
                mutex_unlock(&mut (*runtime).oss.params_lock);
                break;
            }
            buf = buf.offset(tmp);
            bytes -= tmp as usize;
            xfer += tmp as usize;
            (*runtime).oss.buffer_used -= tmp;
        } else {
            tmp = snd_pcm_oss_read2(substream, buf, (*runtime).oss.period_bytes as usize, 0);
            if tmp <= 0 { mutex_unlock(&mut (*runtime).oss.params_lock); break; }
            (*runtime).oss.bytes += tmp as snd_pcm_sframes_t;
            buf = buf.offset(tmp);
            bytes -= tmp as usize;
            xfer += tmp as usize;
        }
        mutex_unlock(&mut (*runtime).oss.params_lock);
        if tmp < 0 { break; }
        if signal_pending(current) != 0 { tmp = -ERESTARTSYS as ssize_t; break; }
        tmp = 0;
    }
    atomic_dec(&mut (*runtime).oss.rw_ref);
    if xfer > 0 { xfer as ssize_t } else { tmp }
}

unsafe fn snd_pcm_oss_reset(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    for i in 0..2 {
        let substream = (*pcm_oss_file).streams[i];
        if substream.is_null() { continue; }
        let runtime = (*substream).runtime;
        snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DROP, ptr::null_mut());
        mutex_lock(&mut (*runtime).oss.params_lock);
        (*runtime).oss.prepare = 1;
        (*runtime).oss.buffer_used = 0;
        (*runtime).oss.prev_hw_ptr_period = 0;
        (*runtime).oss.period_ptr = 0;
        mutex_unlock(&mut (*runtime).oss.params_lock);
    }
    0
}

unsafe fn snd_pcm_oss_post(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    if !substream.is_null() {
        let err = snd_pcm_oss_make_ready(substream);
        if err < 0 { return err; }
        snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_START, ptr::null_mut());
    }
    /* note: all errors from the start action are ignored */
    /* OSS apps do not know, how to handle them */
    0
}

unsafe fn snd_pcm_oss_sync1(substream: *mut snd_pcm_substream, size: size_t) -> c_int {
    let runtime = (*substream).runtime;
    let mut result: ssize_t;
    let mut wait: wait_queue_entry_t = mem::zeroed();
    init_waitqueue_entry(&mut wait, current);
    add_wait_queue(&mut (*runtime).sleep, &mut wait);
    loop {
        result = snd_pcm_oss_write2(substream, (*runtime).oss.buffer, size, 1);
        if result > 0 { (*runtime).oss.buffer_used = 0; result = 0; break; }
        if result != 0 && result != -EAGAIN as ssize_t { break; }
        result = 0;
        set_current_state(TASK_INTERRUPTIBLE);
        let state = (*runtime).state;
        if state != SNDRV_PCM_STATE_RUNNING { set_current_state(TASK_RUNNING); break; }
        let res = schedule_timeout(10 * HZ);
        if signal_pending(current) != 0 { result = -ERESTARTSYS as ssize_t; break; }
        if res == 0 { result = -EIO as ssize_t; break; }
    }
    remove_wait_queue(&mut (*runtime).sleep, &mut wait);
    result as c_int
}

unsafe fn snd_pcm_oss_sync(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut err = 0;
    let mut substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    if !substream.is_null() {
        let runtime = (*substream).runtime;
        if atomic_read(&(*substream).mmap_count) == 0 {
            atomic_inc(&mut (*runtime).oss.rw_ref);
            if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 {
                atomic_dec(&mut (*runtime).oss.rw_ref);
                return -ERESTARTSYS;
            }
            err = snd_pcm_oss_make_ready_locked(substream);
            if err < 0 {
                mutex_unlock(&mut (*runtime).oss.params_lock);
                atomic_dec(&mut (*runtime).oss.rw_ref);
                return err;
            }
            let format = snd_pcm_oss_format_from((*runtime).oss.format);
            let width = snd_pcm_format_physical_width(format) as size_t;
            if (*runtime).oss.buffer_used > 0 {
                let size = (8 * ((*runtime).oss.period_bytes - (*runtime).oss.buffer_used) as usize + 7) / width;
                snd_pcm_format_set_silence(format, (*runtime).oss.buffer.offset((*runtime).oss.buffer_used) as *mut c_void, size);
                err = snd_pcm_oss_sync1(substream, (*runtime).oss.period_bytes as usize);
                if err < 0 { mutex_unlock(&mut (*runtime).oss.params_lock); atomic_dec(&mut (*runtime).oss.rw_ref); return err; }
            } else if (*runtime).oss.period_ptr > 0 {
                let size = (*runtime).oss.period_bytes - (*runtime).oss.period_ptr;
                snd_pcm_format_set_silence(format, (*runtime).oss.buffer as *mut c_void, size as usize * 8 / width);
                err = snd_pcm_oss_sync1(substream, size as usize);
                if err < 0 { mutex_unlock(&mut (*runtime).oss.params_lock); atomic_dec(&mut (*runtime).oss.rw_ref); return err; }
            }
            /* The ALSA's period might be a bit large than OSS one.
             * Fill the remain portion of ALSA period with zeros.
             */
            let mut size = (*(*runtime).control).appl_ptr % (*runtime).period_size;
            if size > 0 {
                size = (*runtime).period_size - size;
                if (*runtime).access == SNDRV_PCM_ACCESS_RW_INTERLEAVED { snd_pcm_lib_write(substream, ptr::null(), size); }
                else if (*runtime).access == SNDRV_PCM_ACCESS_RW_NONINTERLEAVED { snd_pcm_lib_writev(substream, ptr::null(), size); }
            }
            mutex_unlock(&mut (*runtime).oss.params_lock);
            atomic_dec(&mut (*runtime).oss.rw_ref);
        }
        let saved_f_flags = (*substream).f_flags;
        (*substream).f_flags &= !O_NONBLOCK;
        err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DRAIN, ptr::null_mut());
        (*substream).f_flags = saved_f_flags;
        if err < 0 { return err; }
        mutex_lock(&mut (*runtime).oss.params_lock);
        (*runtime).oss.prepare = 1;
        mutex_unlock(&mut (*runtime).oss.params_lock);
    }
    substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE];
    if !substream.is_null() {
        err = snd_pcm_oss_make_ready(substream);
        if err < 0 { return err; }
        let runtime = (*substream).runtime;
        err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DROP, ptr::null_mut());
        if err < 0 { return err; }
        mutex_lock(&mut (*runtime).oss.params_lock);
        (*runtime).oss.buffer_used = 0;
        (*runtime).oss.prepare = 1;
        mutex_unlock(&mut (*runtime).oss.params_lock);
    }
    0
}

unsafe fn snd_pcm_oss_set_rate(pcm_oss_file: *mut snd_pcm_oss_file, mut rate: c_int) -> c_int {
    for idx in (0..2).rev() {
        let substream = (*pcm_oss_file).streams[idx];
        if substream.is_null() { continue; }
        let runtime = (*substream).runtime;
        if rate < 1000 { rate = 1000; } else if rate > 192000 { rate = 192000; }
        let err = lock_params(runtime);
        if err < 0 { return err; }
        if (*runtime).oss.rate != rate as c_uint {
            (*runtime).oss.params = 1;
            (*runtime).oss.rate = rate as c_uint;
        }
        unlock_params(runtime);
    }
    snd_pcm_oss_get_rate(pcm_oss_file)
}

unsafe fn snd_pcm_oss_get_rate(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut substream: *mut snd_pcm_substream = ptr::null_mut();
    let err = snd_pcm_oss_get_active_substream(pcm_oss_file, &mut substream);
    if err < 0 { return err; }
    (*(*substream).runtime).oss.rate as c_int
}

unsafe fn snd_pcm_oss_set_channels(pcm_oss_file: *mut snd_pcm_oss_file, mut channels: c_uint) -> c_int {
    if channels < 1 { channels = 1; }
    if channels > 128 { return -EINVAL; }
    for idx in (0..2).rev() {
        let substream = (*pcm_oss_file).streams[idx];
        if substream.is_null() { continue; }
        let runtime = (*substream).runtime;
        let err = lock_params(runtime);
        if err < 0 { return err; }
        if (*runtime).oss.channels != channels {
            (*runtime).oss.params = 1;
            (*runtime).oss.channels = channels;
        }
        unlock_params(runtime);
    }
    snd_pcm_oss_get_channels(pcm_oss_file)
}

unsafe fn snd_pcm_oss_get_channels(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut substream: *mut snd_pcm_substream = ptr::null_mut();
    let err = snd_pcm_oss_get_active_substream(pcm_oss_file, &mut substream);
    if err < 0 { return err; }
    (*(*substream).runtime).oss.channels as c_int
}

unsafe fn snd_pcm_oss_get_block_size(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut substream: *mut snd_pcm_substream = ptr::null_mut();
    let err = snd_pcm_oss_get_active_substream(pcm_oss_file, &mut substream);
    if err < 0 { return err; }
    (*(*substream).runtime).oss.period_bytes as c_int
}

unsafe fn snd_pcm_oss_get_formats(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut substream: *mut snd_pcm_substream = ptr::null_mut();
    let err = snd_pcm_oss_get_active_substream(pcm_oss_file, &mut substream);
    if err < 0 { return err; }
    let direct = if atomic_read(&(*substream).mmap_count) != 0 { 1 } else { (*substream).oss.setup.direct as c_int };
    if direct == 0 {
        return AFMT_MU_LAW | AFMT_U8 | AFMT_S16_LE | AFMT_S16_BE | AFMT_S8 | AFMT_U16_LE |
            AFMT_U16_BE | AFMT_S32_LE | AFMT_S32_BE | AFMT_S24_LE | AFMT_S24_BE | AFMT_S24_PACKED;
    }
    let params = kmalloc_obj::<snd_pcm_hw_params>();
    if params.is_null() { return -ENOMEM; }
    _snd_pcm_hw_params_any(params);
    let err = snd_pcm_hw_refine(substream, params);
    if err < 0 { kfree(params as *mut c_void); return err; }
    let format_mask = hw_param_mask_c(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let mut formats = 0;
    for fmt in 0..32 {
        if snd_mask_test(format_mask, fmt) != 0 {
            let f = snd_pcm_oss_format_to(fmt);
            if f >= 0 { formats |= f; }
        }
    }
    kfree(params as *mut c_void);
    formats
}

unsafe fn snd_pcm_oss_set_format(pcm_oss_file: *mut snd_pcm_oss_file, mut format: c_int) -> c_int {
    if format != AFMT_QUERY {
        let formats = snd_pcm_oss_get_formats(pcm_oss_file);
        if formats < 0 { return formats; }
        if (formats & format) == 0 { format = AFMT_U8; }
        for idx in (0..2).rev() {
            let substream = (*pcm_oss_file).streams[idx];
            if substream.is_null() { continue; }
            let runtime = (*substream).runtime;
            let err = lock_params(runtime);
            if err < 0 { return err; }
            if (*runtime).oss.format != format {
                (*runtime).oss.params = 1;
                (*runtime).oss.format = format;
            }
            unlock_params(runtime);
        }
    }
    snd_pcm_oss_get_format(pcm_oss_file)
}

unsafe fn snd_pcm_oss_get_format(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut substream: *mut snd_pcm_substream = ptr::null_mut();
    let err = snd_pcm_oss_get_active_substream(pcm_oss_file, &mut substream);
    if err < 0 { return err; }
    (*(*substream).runtime).oss.format
}

unsafe fn snd_pcm_oss_set_subdivide1(substream: *mut snd_pcm_substream, mut subdivide: c_int) -> c_int {
    let runtime = (*substream).runtime;
    if subdivide == 0 {
        subdivide = (*runtime).oss.subdivision;
        if subdivide == 0 { subdivide = 1; }
        return subdivide;
    }
    if (*runtime).oss.subdivision != 0 || (*runtime).oss.fragshift != 0 { return -EINVAL; }
    if subdivide != 1 && subdivide != 2 && subdivide != 4 && subdivide != 8 && subdivide != 16 { return -EINVAL; }
    (*runtime).oss.subdivision = subdivide;
    (*runtime).oss.params = 1;
    subdivide
}

unsafe fn snd_pcm_oss_set_subdivide(pcm_oss_file: *mut snd_pcm_oss_file, subdivide: c_int) -> c_int {
    let mut err = -EINVAL;
    for idx in (0..2).rev() {
        let substream = (*pcm_oss_file).streams[idx];
        if substream.is_null() { continue; }
        let runtime = (*substream).runtime;
        err = lock_params(runtime);
        if err < 0 { return err; }
        err = snd_pcm_oss_set_subdivide1(substream, subdivide);
        unlock_params(runtime);
        if err < 0 { return err; }
    }
    err
}

unsafe fn snd_pcm_oss_set_fragment1(substream: *mut snd_pcm_substream, val: c_uint) -> c_int {
    let runtime = (*substream).runtime;
    if (*runtime).oss.subdivision != 0 || (*runtime).oss.fragshift != 0 { return -EINVAL; }
    let fragshift = (val & 0xffff) as c_int;
    if fragshift >= 25 { return -EINVAL; } /* should be large enough */
    (*runtime).oss.fragshift = fragshift;
    (*runtime).oss.maxfrags = ((val >> 16) & 0xffff) as c_int;
    if (*runtime).oss.fragshift < 4 { (*runtime).oss.fragshift = 4; } /* < 16 */
    if (*runtime).oss.maxfrags < 2 { (*runtime).oss.maxfrags = 2; }
    (*runtime).oss.params = 1;
    0
}

unsafe fn snd_pcm_oss_set_fragment(pcm_oss_file: *mut snd_pcm_oss_file, val: c_uint) -> c_int {
    let mut err = -EINVAL;
    for idx in (0..2).rev() {
        let substream = (*pcm_oss_file).streams[idx];
        if substream.is_null() { continue; }
        let runtime = (*substream).runtime;
        err = lock_params(runtime);
        if err < 0 { return err; }
        err = snd_pcm_oss_set_fragment1(substream, val);
        unlock_params(runtime);
        if err < 0 { return err; }
    }
    err
}

unsafe fn snd_pcm_oss_nonblock(file: *mut file) -> c_int {
    (*file).f_flags |= O_NONBLOCK;
    0
}

unsafe fn snd_pcm_oss_get_caps1(substream: *mut snd_pcm_substream, mut res: c_int) -> c_int {
    if substream.is_null() {
        res &= !DSP_CAP_DUPLEX;
        return res;
    }
    /* DSP_CAP_MULTI conditional omitted unless supplied by build. */
    /* DSP_CAP_REALTIME is set all times: all ALSA drivers can return actual pointer in ring buffer */
    res
}

unsafe fn snd_pcm_oss_get_caps(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut result = DSP_CAP_TRIGGER | DSP_CAP_MMAP | DSP_CAP_DUPLEX | DSP_CAP_REALTIME;
    for idx in 0..2 {
        result = snd_pcm_oss_get_caps1((*pcm_oss_file).streams[idx], result);
    }
    result |= 0x0001; /* revision - same as SB AWE 64 */
    result
}

unsafe fn snd_pcm_oss_simulate_fill(substream: *mut snd_pcm_substream, hw_ptr: snd_pcm_uframes_t) {
    let runtime = (*substream).runtime;
    let mut appl_ptr = hw_ptr + (*runtime).buffer_size;
    appl_ptr %= (*runtime).boundary;
    (*(*runtime).control).appl_ptr = appl_ptr;
}

unsafe fn snd_pcm_oss_set_trigger(pcm_oss_file: *mut snd_pcm_oss_file, trigger: c_int) -> c_int {
    let psubstream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    let csubstream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE];
    if !psubstream.is_null() {
        let err = snd_pcm_oss_make_ready(psubstream);
        if err < 0 { return err; }
    }
    if !csubstream.is_null() {
        let err = snd_pcm_oss_make_ready(csubstream);
        if err < 0 { return err; }
    }
    if !psubstream.is_null() {
        let runtime = (*psubstream).runtime;
        let mut cmd = 0;
        if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 { return -ERESTARTSYS; }
        if (trigger & PCM_ENABLE_OUTPUT) != 0 {
            if (*runtime).oss.trigger == 0 {
                if atomic_read(&(*psubstream).mmap_count) != 0 { snd_pcm_oss_simulate_fill(psubstream, get_hw_ptr_period(runtime)); }
                (*runtime).oss.trigger = 1;
                (*runtime).start_threshold = 1;
                cmd = SNDRV_PCM_IOCTL_START;
            }
        } else if (*runtime).oss.trigger != 0 {
            (*runtime).oss.trigger = 0;
            (*runtime).start_threshold = (*runtime).boundary;
            cmd = SNDRV_PCM_IOCTL_DROP;
            (*runtime).oss.prepare = 1;
        }
        mutex_unlock(&mut (*runtime).oss.params_lock);
        if cmd != 0 {
            let err = snd_pcm_kernel_ioctl(psubstream, cmd, ptr::null_mut());
            if err < 0 { return err; }
        }
    }
    if !csubstream.is_null() {
        let runtime = (*csubstream).runtime;
        let mut cmd = 0;
        if mutex_lock_interruptible(&mut (*runtime).oss.params_lock) != 0 { return -ERESTARTSYS; }
        if (trigger & PCM_ENABLE_INPUT) != 0 {
            if (*runtime).oss.trigger == 0 {
                (*runtime).oss.trigger = 1;
                (*runtime).start_threshold = 1;
                cmd = SNDRV_PCM_IOCTL_START;
            }
        } else if (*runtime).oss.trigger != 0 {
            (*runtime).oss.trigger = 0;
            (*runtime).start_threshold = (*runtime).boundary;
            cmd = SNDRV_PCM_IOCTL_DROP;
            (*runtime).oss.prepare = 1;
        }
        mutex_unlock(&mut (*runtime).oss.params_lock);
        if cmd != 0 {
            let err = snd_pcm_kernel_ioctl(csubstream, cmd, ptr::null_mut());
            if err < 0 { return err; }
        }
    }
    0
}

unsafe fn snd_pcm_oss_get_trigger(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let mut result = 0;
    let psubstream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    let csubstream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE];
    if !psubstream.is_null() && !(*psubstream).runtime.is_null() {
        mutex_lock(&mut (*(*psubstream).runtime).oss.params_lock);
        if (*(*psubstream).runtime).oss.trigger != 0 { result |= PCM_ENABLE_OUTPUT; }
        mutex_unlock(&mut (*(*psubstream).runtime).oss.params_lock);
    }
    if !csubstream.is_null() && !(*csubstream).runtime.is_null() {
        mutex_lock(&mut (*(*csubstream).runtime).oss.params_lock);
        if (*(*csubstream).runtime).oss.trigger != 0 { result |= PCM_ENABLE_INPUT; }
        mutex_unlock(&mut (*(*csubstream).runtime).oss.params_lock);
    }
    result
}

unsafe fn snd_pcm_oss_get_odelay(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    let substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    if substream.is_null() { return -EINVAL; }
    let err = snd_pcm_oss_make_ready(substream);
    if err < 0 { return err; }
    let runtime = (*substream).runtime;
    if (*runtime).oss.params != 0 || (*runtime).oss.prepare != 0 { return 0; }
    let mut delay: snd_pcm_sframes_t = 0;
    let err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DELAY, &mut delay as *mut _ as *mut c_void);
    if err == -EPIPE { delay = 0; } /* hack for broken OSS applications */
    else if err < 0 { return err; }
    snd_pcm_oss_bytes(substream, delay) as c_int
}

unsafe fn snd_pcm_oss_get_ptr(pcm_oss_file: *mut snd_pcm_oss_file, stream: c_int, info_user: *mut count_info) -> c_int {
    if info_user.is_null() { return -EFAULT; }
    let substream = (*pcm_oss_file).streams[stream as usize];
    if substream.is_null() { return -EINVAL; }
    let err = snd_pcm_oss_make_ready(substream);
    if err < 0 { return err; }
    let runtime = (*substream).runtime;
    let mut info: count_info = mem::zeroed();
    if (*runtime).oss.params != 0 || (*runtime).oss.prepare != 0 {
        if copy_to_user(info_user as *mut c_void, &info as *const _ as *const c_void, mem::size_of::<count_info>()) != 0 { return -EFAULT; }
        return 0;
    }
    let mut delay: snd_pcm_sframes_t = 0;
    let fixup: snd_pcm_sframes_t;
    let mut err;
    if stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
        err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DELAY, &mut delay as *mut _ as *mut c_void);
        if err == -EPIPE || err == -ESTRPIPE || (err == 0 && delay < 0) { err = 0; delay = 0; fixup = 0; }
        else { fixup = (*runtime).oss.buffer_used as snd_pcm_sframes_t; }
    } else {
        err = snd_pcm_oss_capture_position_fixup(substream, &mut delay);
        fixup = -((*runtime).oss.buffer_used as snd_pcm_sframes_t);
    }
    if err < 0 { return err; }
    info.ptr = snd_pcm_oss_bytes(substream, ((*(*runtime).status).hw_ptr % (*runtime).buffer_size) as c_long) as c_int;
    if atomic_read(&(*substream).mmap_count) != 0 {
        delay = get_hw_ptr_period(runtime) as snd_pcm_sframes_t;
        let mut n = delay - (*runtime).oss.prev_hw_ptr_period as snd_pcm_sframes_t;
        if n < 0 { n += (*runtime).boundary as snd_pcm_sframes_t; }
        info.blocks = (n as snd_pcm_uframes_t / (*runtime).period_size) as c_int;
        (*runtime).oss.prev_hw_ptr_period = delay as snd_pcm_uframes_t;
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int { snd_pcm_oss_simulate_fill(substream, delay as snd_pcm_uframes_t); }
        info.bytes = (snd_pcm_oss_bytes(substream, (*(*runtime).status).hw_ptr as c_long) & c_int::MAX as c_long) as c_int;
    } else {
        delay = snd_pcm_oss_bytes(substream, delay);
        if stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
            if (*substream).oss.setup.buggyptr != 0 { info.blocks = (((*runtime).oss.buffer_bytes as snd_pcm_sframes_t - delay - fixup) / (*runtime).oss.period_bytes as snd_pcm_sframes_t) as c_int; }
            else { info.blocks = ((delay + fixup) / (*runtime).oss.period_bytes as snd_pcm_sframes_t) as c_int; }
            info.bytes = (((*runtime).oss.bytes - delay) & c_int::MAX as c_long) as c_int;
        } else {
            delay += fixup;
            info.blocks = (delay / (*runtime).oss.period_bytes as snd_pcm_sframes_t) as c_int;
            info.bytes = (((*runtime).oss.bytes + delay) & c_int::MAX as c_long) as c_int;
        }
    }
    if copy_to_user(info_user as *mut c_void, &info as *const _ as *const c_void, mem::size_of::<count_info>()) != 0 { return -EFAULT; }
    0
}

unsafe fn snd_pcm_oss_get_space(pcm_oss_file: *mut snd_pcm_oss_file, stream: c_int, info_user: *mut audio_buf_info) -> c_int {
    if info_user.is_null() { return -EFAULT; }
    let substream = (*pcm_oss_file).streams[stream as usize];
    if substream.is_null() { return -EINVAL; }
    let runtime = (*substream).runtime;
    if (*runtime).oss.params != 0 {
        let err = snd_pcm_oss_change_params(substream, false);
        if err < 0 { return err; }
    }
    let mut info: audio_buf_info = mem::zeroed();
    info.fragsize = (*runtime).oss.period_bytes as c_int;
    info.fragstotal = (*runtime).periods as c_int;
    if (*runtime).oss.prepare != 0 {
        if stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
            info.bytes = ((*runtime).oss.period_bytes * (*runtime).oss.periods) as c_int;
            info.fragments = (*runtime).oss.periods as c_int;
        } else {
            info.bytes = 0;
            info.fragments = 0;
        }
    } else {
        let mut avail: snd_pcm_sframes_t = 0;
        let fixup: snd_pcm_sframes_t;
        let mut err;
        if stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
            err = snd_pcm_kernel_ioctl(substream, SNDRV_PCM_IOCTL_DELAY, &mut avail as *mut _ as *mut c_void);
            if err == -EPIPE || err == -ESTRPIPE || (err == 0 && avail < 0) {
                avail = (*runtime).buffer_size as snd_pcm_sframes_t;
                err = 0;
                fixup = 0;
            } else {
                avail = (*runtime).buffer_size as snd_pcm_sframes_t - avail;
                fixup = -((*runtime).oss.buffer_used as snd_pcm_sframes_t);
            }
        } else {
            err = snd_pcm_oss_capture_position_fixup(substream, &mut avail);
            fixup = (*runtime).oss.buffer_used as snd_pcm_sframes_t;
        }
        if err < 0 { return err; }
        info.bytes = (snd_pcm_oss_bytes(substream, avail) + fixup) as c_int;
        info.fragments = info.bytes / (*runtime).oss.period_bytes as c_int;
    }
    if copy_to_user(info_user as *mut c_void, &info as *const _ as *const c_void, mem::size_of::<audio_buf_info>()) != 0 { return -EFAULT; }
    0
}

unsafe fn snd_pcm_oss_get_mapbuf(_pcm_oss_file: *mut snd_pcm_oss_file, _stream: c_int, _info: *mut buffmem_desc) -> c_int {
    // it won't be probably implemented
    // pr_debug("TODO: snd_pcm_oss_get_mapbuf\n");
    -EINVAL
}

unsafe fn strip_task_path(path: *const c_char) -> *const c_char {
    let mut ptr_ = path;
    let mut ptrl: *const c_char = ptr::null();
    while *ptr_ != 0 {
        if *ptr_ as u8 == b'/' { ptrl = ptr_.add(1); }
        ptr_ = ptr_.add(1);
    }
    ptrl
}

unsafe fn snd_pcm_oss_look_for_setup(pcm: *mut snd_pcm, stream: c_int, mut task_name: *const c_char, rsetup: *mut snd_pcm_oss_setup) {
    mutex_lock(&mut (*pcm).streams[stream as usize].oss.setup_mutex);
    let mut setup: *mut snd_pcm_oss_setup;
    loop {
        setup = (*pcm).streams[stream as usize].oss.setup_list;
        while !setup.is_null() {
            if strcmp((*setup).task_name, task_name) == 0 { break; }
            setup = (*setup).next;
        }
        if !setup.is_null() { break; }
        task_name = strip_task_path(task_name);
        if task_name.is_null() { break; }
    }
    if !setup.is_null() { *rsetup = ptr::read(setup); }
    mutex_unlock(&mut (*pcm).streams[stream as usize].oss.setup_mutex);
}

unsafe extern "C" fn snd_pcm_oss_release_substream(substream: *mut snd_pcm_substream) {
    snd_pcm_oss_release_buffers(substream);
    (*substream).oss.oss = 0;
}

unsafe fn snd_pcm_oss_init_substream(substream: *mut snd_pcm_substream, setup: *mut snd_pcm_oss_setup, minor: c_int) {
    (*substream).oss.oss = 1;
    (*substream).oss.setup = ptr::read(setup);
    if (*setup).nonblock != 0 { (*substream).f_flags |= O_NONBLOCK; }
    else if (*setup).block != 0 { (*substream).f_flags &= !O_NONBLOCK; }
    let runtime = (*substream).runtime;
    (*runtime).oss.params = 1;
    (*runtime).oss.trigger = 1;
    (*runtime).oss.rate = 8000;
    mutex_init(&mut (*runtime).oss.params_lock);
    match SNDRV_MINOR_OSS_DEVICE(minor) {
        SNDRV_MINOR_OSS_PCM_8 => (*runtime).oss.format = AFMT_U8,
        SNDRV_MINOR_OSS_PCM_16 => (*runtime).oss.format = AFMT_S16_LE,
        _ => (*runtime).oss.format = AFMT_MU_LAW,
    }
    (*runtime).oss.channels = 1;
    (*runtime).oss.fragshift = 0;
    (*runtime).oss.maxfrags = 0;
    (*runtime).oss.subdivision = 0;
    (*substream).pcm_release = Some(snd_pcm_oss_release_substream);
    atomic_set(&mut (*runtime).oss.rw_ref, 0);
}

const SNDRV_MINOR_OSS_PCM_8: c_int = 0;
const SNDRV_MINOR_OSS_PCM_16: c_int = 1;
unsafe fn SNDRV_MINOR_OSS_DEVICE(minor: c_int) -> c_int { minor & 0xff }

unsafe fn snd_pcm_oss_release_file(pcm_oss_file: *mut snd_pcm_oss_file) -> c_int {
    if pcm_oss_file.is_null() { return 0; }
    for cidx in 0..2 {
        let substream = (*pcm_oss_file).streams[cidx];
        if !substream.is_null() { snd_pcm_release_substream(substream); }
    }
    kfree(pcm_oss_file as *mut c_void);
    0
}

unsafe fn snd_pcm_oss_open_file(file: *mut file, pcm: *mut snd_pcm, rpcm_oss_file: *mut *mut snd_pcm_oss_file, minor: c_int, setup: *mut snd_pcm_oss_setup) -> c_int {
    if !rpcm_oss_file.is_null() { *rpcm_oss_file = ptr::null_mut(); }
    let pcm_oss_file = kzalloc_obj::<snd_pcm_oss_file>();
    if pcm_oss_file.is_null() { return -ENOMEM; }
    let mut f_mode = (*file).f_mode;
    if (f_mode & (FMODE_WRITE | FMODE_READ)) == (FMODE_WRITE | FMODE_READ) && ((*pcm).info_flags & SNDRV_PCM_INFO_HALF_DUPLEX) != 0 {
        f_mode = FMODE_WRITE;
    }
    (*file).f_flags &= !O_APPEND;
    for idx in 0..2 {
        let setup_i = setup.add(idx);
        if (*setup_i).disable != 0 { continue; }
        if (*pcm).streams[idx].substream_count == 0 { continue; } /* no matching substream */
        if idx == SNDRV_PCM_STREAM_PLAYBACK {
            if (f_mode & FMODE_WRITE) == 0 { continue; }
        } else if (f_mode & FMODE_READ) == 0 { continue; }
        let mut substream: *mut snd_pcm_substream = ptr::null_mut();
        let err = snd_pcm_open_substream(pcm, idx as c_int, file, &mut substream);
        if err < 0 {
            snd_pcm_oss_release_file(pcm_oss_file);
            return err;
        }
        (*pcm_oss_file).streams[idx] = substream;
        snd_pcm_oss_init_substream(substream, setup_i, minor);
    }
    if (*pcm_oss_file).streams[0].is_null() && (*pcm_oss_file).streams[1].is_null() {
        snd_pcm_oss_release_file(pcm_oss_file);
        return -EINVAL;
    }
    (*file).private_data = pcm_oss_file as *mut c_void;
    if !rpcm_oss_file.is_null() { *rpcm_oss_file = pcm_oss_file; }
    0
}

unsafe fn snd_task_name(task: *mut task_struct, name: *mut c_char, size: size_t) -> c_int {
    if snd_BUG_ON(task.is_null() || name.is_null() || size < 2) != 0 { return -EINVAL; }
    let mut idx = 0usize;
    while idx < (*task).comm.len() && idx + 1 < size {
        *name.add(idx) = (*task).comm[idx];
        idx += 1;
    }
    *name.add(idx) = 0;
    0
}

unsafe extern "C" fn snd_pcm_oss_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut task_name = [0 as c_char; 32];
    let mut setup: [snd_pcm_oss_setup; 2] = mem::zeroed();
    let mut wait: wait_queue_entry_t = mem::zeroed();
    nonseekable_open(inode, file);
    let pcm = snd_lookup_oss_minor_data(iminor(inode), SNDRV_OSS_DEVICE_TYPE_PCM);
    if pcm.is_null() { return -ENODEV; }
    let mut err = snd_card_file_add((*pcm).card, file);
    if err < 0 { snd_card_unref((*pcm).card); return err; }
    if try_module_get((*(*pcm).card).module) == 0 {
        snd_card_file_remove((*pcm).card, file);
        snd_card_unref((*pcm).card);
        return -ENODEV;
    }
    if snd_task_name(current, task_name.as_mut_ptr(), task_name.len()) < 0 {
        module_put((*(*pcm).card).module);
        snd_card_file_remove((*pcm).card, file);
        snd_card_unref((*pcm).card);
        return -EFAULT;
    }
    if ((*file).f_mode & FMODE_WRITE) != 0 { snd_pcm_oss_look_for_setup(pcm, SNDRV_PCM_STREAM_PLAYBACK as c_int, task_name.as_ptr(), &mut setup[0]); }
    if ((*file).f_mode & FMODE_READ) != 0 { snd_pcm_oss_look_for_setup(pcm, SNDRV_PCM_STREAM_CAPTURE as c_int, task_name.as_ptr(), &mut setup[1]); }
    let mut nonblock = (((*file).f_flags & O_NONBLOCK) != 0) as c_int;
    if nonblock == 0 { nonblock = nonblock_open as c_int; }
    init_waitqueue_entry(&mut wait, current);
    add_wait_queue(&mut (*pcm).open_wait, &mut wait);
    mutex_lock(&mut (*pcm).open_mutex);
    let mut pcm_oss_file: *mut snd_pcm_oss_file = ptr::null_mut();
    loop {
        err = snd_pcm_oss_open_file(file, pcm, &mut pcm_oss_file, iminor(inode), setup.as_mut_ptr());
        if err >= 0 { break; }
        if err == -EAGAIN {
            if nonblock != 0 { err = -EBUSY; break; }
        } else { break; }
        set_current_state(TASK_INTERRUPTIBLE);
        mutex_unlock(&mut (*pcm).open_mutex);
        schedule();
        mutex_lock(&mut (*pcm).open_mutex);
        if (*(*pcm).card).shutdown != 0 { err = -ENODEV; break; }
        if signal_pending(current) != 0 { err = -ERESTARTSYS; break; }
    }
    remove_wait_queue(&mut (*pcm).open_wait, &mut wait);
    mutex_unlock(&mut (*pcm).open_mutex);
    if err < 0 {
        module_put((*(*pcm).card).module);
        snd_card_file_remove((*pcm).card, file);
        snd_card_unref((*pcm).card);
        return err;
    }
    snd_card_unref((*pcm).card);
    err
}

unsafe extern "C" fn snd_pcm_oss_release(_inode: *mut inode, file: *mut file) -> c_int {
    let pcm_oss_file = (*file).private_data as *mut snd_pcm_oss_file;
    let mut substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    if substream.is_null() { substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE]; }
    if snd_BUG_ON(substream.is_null()) != 0 { return -ENXIO; }
    let pcm = (*substream).pcm;
    if (*(*pcm).card).shutdown == 0 { snd_pcm_oss_sync(pcm_oss_file); }
    mutex_lock(&mut (*pcm).open_mutex);
    snd_pcm_oss_release_file(pcm_oss_file);
    mutex_unlock(&mut (*pcm).open_mutex);
    wake_up(&mut (*pcm).open_wait);
    module_put((*(*pcm).card).module);
    snd_card_file_remove((*pcm).card, file);
    0
}

unsafe extern "C" fn snd_pcm_oss_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let p = arg as *mut c_int;
    let pcm_oss_file = (*file).private_data as *mut snd_pcm_oss_file;
    let mut res: c_int = 0;
    if cmd == OSS_GETVERSION { return put_user(SNDRV_OSS_VERSION, p) as c_long; }
    if cmd == OSS_ALSAEMULVER { return put_user(1, p) as c_long; }
    if ((cmd >> 8) & 0xff) != b'P' as c_uint { return -EINVAL as c_long; }
    match cmd {
        SNDCTL_DSP_RESET => snd_pcm_oss_reset(pcm_oss_file) as c_long,
        SNDCTL_DSP_SYNC => snd_pcm_oss_sync(pcm_oss_file) as c_long,
        SNDCTL_DSP_SPEED => { if get_user(&mut res, p) != 0 { return -EFAULT as c_long; } res = snd_pcm_oss_set_rate(pcm_oss_file, res); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SOUND_PCM_READ_RATE => { res = snd_pcm_oss_get_rate(pcm_oss_file); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_STEREO => { if get_user(&mut res, p) != 0 { return -EFAULT as c_long; } res = if res > 0 { 2 } else { 1 }; res = snd_pcm_oss_set_channels(pcm_oss_file, res as c_uint); if res < 0 { return res as c_long; } res -= 1; put_user(res, p) as c_long }
        SNDCTL_DSP_GETBLKSIZE => { res = snd_pcm_oss_get_block_size(pcm_oss_file); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_SETFMT => { if get_user(&mut res, p) != 0 { return -EFAULT as c_long; } res = snd_pcm_oss_set_format(pcm_oss_file, res); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SOUND_PCM_READ_BITS => { res = snd_pcm_oss_get_format(pcm_oss_file); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_CHANNELS => { if get_user(&mut res, p) != 0 { return -EFAULT as c_long; } res = snd_pcm_oss_set_channels(pcm_oss_file, res as c_uint); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SOUND_PCM_READ_CHANNELS => { res = snd_pcm_oss_get_channels(pcm_oss_file); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SOUND_PCM_WRITE_FILTER | SOUND_PCM_READ_FILTER => -EIO as c_long,
        SNDCTL_DSP_POST => snd_pcm_oss_post(pcm_oss_file) as c_long,
        SNDCTL_DSP_SUBDIVIDE => { if get_user(&mut res, p) != 0 { return -EFAULT as c_long; } res = snd_pcm_oss_set_subdivide(pcm_oss_file, res); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_SETFRAGMENT => { if get_user(&mut res, p) != 0 { return -EFAULT as c_long; } snd_pcm_oss_set_fragment(pcm_oss_file, res as c_uint) as c_long }
        SNDCTL_DSP_GETFMTS => { res = snd_pcm_oss_get_formats(pcm_oss_file); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_GETOSPACE | SNDCTL_DSP_GETISPACE => snd_pcm_oss_get_space(pcm_oss_file, if cmd == SNDCTL_DSP_GETISPACE { SNDRV_PCM_STREAM_CAPTURE as c_int } else { SNDRV_PCM_STREAM_PLAYBACK as c_int }, arg as *mut audio_buf_info) as c_long,
        SNDCTL_DSP_NONBLOCK => snd_pcm_oss_nonblock(file) as c_long,
        SNDCTL_DSP_GETCAPS => { res = snd_pcm_oss_get_caps(pcm_oss_file); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_GETTRIGGER => { res = snd_pcm_oss_get_trigger(pcm_oss_file); if res < 0 { return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_SETTRIGGER => { if get_user(&mut res, p) != 0 { return -EFAULT as c_long; } snd_pcm_oss_set_trigger(pcm_oss_file, res) as c_long }
        SNDCTL_DSP_GETIPTR | SNDCTL_DSP_GETOPTR => snd_pcm_oss_get_ptr(pcm_oss_file, if cmd == SNDCTL_DSP_GETIPTR { SNDRV_PCM_STREAM_CAPTURE as c_int } else { SNDRV_PCM_STREAM_PLAYBACK as c_int }, arg as *mut count_info) as c_long,
        SNDCTL_DSP_MAPINBUF | SNDCTL_DSP_MAPOUTBUF => snd_pcm_oss_get_mapbuf(pcm_oss_file, if cmd == SNDCTL_DSP_MAPINBUF { SNDRV_PCM_STREAM_CAPTURE as c_int } else { SNDRV_PCM_STREAM_PLAYBACK as c_int }, arg as *mut buffmem_desc) as c_long,
        SNDCTL_DSP_SETSYNCRO => 0,
        SNDCTL_DSP_SETDUPLEX => if (snd_pcm_oss_get_caps(pcm_oss_file) & DSP_CAP_DUPLEX) != 0 { 0 } else { -EIO as c_long },
        SNDCTL_DSP_GETODELAY => { res = snd_pcm_oss_get_odelay(pcm_oss_file); if res < 0 { put_user(0, p); return res as c_long; } put_user(res, p) as c_long }
        SNDCTL_DSP_PROFILE => 0,
        _ => -EINVAL as c_long,
    }
}

/* CONFIG_COMPAT: all compatible */
unsafe extern "C" fn snd_pcm_oss_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    snd_pcm_oss_ioctl(file, cmd, arg)
}

unsafe extern "C" fn snd_pcm_oss_read(file: *mut file, buf: *mut c_char, count: size_t, _offset: *mut loff_t) -> ssize_t {
    let pcm_oss_file = (*file).private_data as *mut snd_pcm_oss_file;
    let substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE];
    if substream.is_null() { return -ENXIO as ssize_t; }
    (*substream).f_flags = (*file).f_flags & O_NONBLOCK;
    snd_pcm_oss_read1(substream, buf, count)
}

unsafe extern "C" fn snd_pcm_oss_write(file: *mut file, buf: *const c_char, count: size_t, _offset: *mut loff_t) -> ssize_t {
    let pcm_oss_file = (*file).private_data as *mut snd_pcm_oss_file;
    let substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    if substream.is_null() { return -ENXIO as ssize_t; }
    (*substream).f_flags = (*file).f_flags & O_NONBLOCK;
    snd_pcm_oss_write1(substream, buf, count)
}

unsafe fn snd_pcm_oss_playback_ready(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    if atomic_read(&(*substream).mmap_count) != 0 {
        ((*runtime).oss.prev_hw_ptr_period != get_hw_ptr_period(runtime)) as c_int
    } else {
        (snd_pcm_playback_avail(runtime) >= (*runtime).oss.period_frames) as c_int
    }
}

unsafe fn snd_pcm_oss_capture_ready(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    if atomic_read(&(*substream).mmap_count) != 0 {
        ((*runtime).oss.prev_hw_ptr_period != get_hw_ptr_period(runtime)) as c_int
    } else {
        (snd_pcm_capture_avail(runtime) >= (*runtime).oss.period_frames) as c_int
    }
}

unsafe fn need_input_retrigger(runtime: *mut snd_pcm_runtime) -> bool_t {
    mutex_lock(&mut (*runtime).oss.params_lock);
    let ret = (*runtime).oss.trigger != 0;
    if ret { (*runtime).oss.trigger = 0; }
    mutex_unlock(&mut (*runtime).oss.params_lock);
    ret
}

unsafe extern "C" fn snd_pcm_oss_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let pcm_oss_file = (*file).private_data as *mut snd_pcm_oss_file;
    let psubstream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
    let csubstream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE];
    let mut mask: __poll_t = 0;
    if !psubstream.is_null() {
        let runtime = (*psubstream).runtime;
        poll_wait(file, &mut (*runtime).sleep, wait);
        if (*runtime).state != SNDRV_PCM_STATE_DRAINING &&
            ((*runtime).state != SNDRV_PCM_STATE_RUNNING || snd_pcm_oss_playback_ready(psubstream) != 0) {
            mask |= EPOLLOUT | EPOLLWRNORM;
        }
    }
    if !csubstream.is_null() {
        let runtime = (*csubstream).runtime;
        poll_wait(file, &mut (*runtime).sleep, wait);
        let ostate = (*runtime).state;
        if ostate != SNDRV_PCM_STATE_RUNNING || snd_pcm_oss_capture_ready(csubstream) != 0 {
            mask |= EPOLLIN | EPOLLRDNORM;
        }
        if ostate != SNDRV_PCM_STATE_RUNNING && need_input_retrigger(runtime) {
            let mut ofile: snd_pcm_oss_file = mem::zeroed();
            ofile.streams[SNDRV_PCM_STREAM_CAPTURE] = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE];
            snd_pcm_oss_set_trigger(&mut ofile, PCM_ENABLE_INPUT);
        }
    }
    mask
}

unsafe extern "C" fn snd_pcm_oss_mmap(file: *mut file, area: *mut vm_area_struct) -> c_int {
    let pcm_oss_file = (*file).private_data as *mut snd_pcm_oss_file;
    let mut substream: *mut snd_pcm_substream = ptr::null_mut();
    match (*area).vm_flags & (VM_READ | VM_WRITE) {
        x if x == (VM_READ | VM_WRITE) => {
            substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK];
            if substream.is_null() { substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE]; }
        }
        VM_READ => substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_CAPTURE],
        VM_WRITE => substream = (*pcm_oss_file).streams[SNDRV_PCM_STREAM_PLAYBACK],
        _ => return -EINVAL,
    }
    /* set VM_READ access as well to fix memset() routines that do reads before writes */
    vm_flags_set(area, VM_READ);
    if substream.is_null() { return -ENXIO; }
    let runtime = (*substream).runtime;
    if ((*runtime).info & SNDRV_PCM_INFO_MMAP_VALID) == 0 { return -EIO; }
    if ((*runtime).info & SNDRV_PCM_INFO_INTERLEAVED) != 0 { (*runtime).access = SNDRV_PCM_ACCESS_MMAP_INTERLEAVED; } else { return -EIO; }
    if (*runtime).oss.params != 0 {
        let err = snd_pcm_oss_change_params(substream, true);
        if err < 0 { return err; }
    }
    if !(*runtime).oss.plugin_first.is_null() { return -EIO; }
    if (*area).vm_pgoff != 0 { return -EINVAL; }
    let err = snd_pcm_mmap_data(substream, file, area);
    if err < 0 { return err; }
    (*runtime).oss.mmap_bytes = ((*area).vm_end - (*area).vm_start) as ssize_t;
    (*runtime).silence_threshold = 0;
    (*runtime).silence_size = 0;
    /* In mmap mode we never stop */
    (*runtime).stop_threshold = (*runtime).boundary;
    0
}

/* CONFIG_SND_VERBOSE_PROCFS */
unsafe fn snd_pcm_oss_proc_read(_entry: *mut snd_info_entry, _buffer: *mut snd_info_buffer) {
    /* Procfs printing depends on snd_iprintf and entry internals supplied outside this file. */
}

unsafe fn snd_pcm_oss_proc_free_setup_list(pstr: *mut snd_pcm_str) {
    let mut setup = (*pstr).oss.setup_list;
    (*pstr).oss.setup_list = ptr::null_mut();
    while !setup.is_null() {
        let setupn = (*setup).next;
        kfree((*setup).task_name as *mut c_void);
        kfree(setup as *mut c_void);
        setup = setupn;
    }
    (*pstr).oss.setup_list = ptr::null_mut();
}

unsafe fn snd_pcm_oss_proc_write(_entry: *mut snd_info_entry, _buffer: *mut snd_info_buffer) {
    /* The original parses procfs setup lines and updates snd_pcm_oss_setup nodes.
     * This translation leaves snd_info_entry private layout as an external dependency. */
}

unsafe fn snd_pcm_oss_proc_init(_pcm: *mut snd_pcm) {
    /* Creates /proc entries when CONFIG_SND_VERBOSE_PROCFS is enabled. */
}

unsafe fn snd_pcm_oss_proc_done(pcm: *mut snd_pcm) {
    for stream in 0..2 {
        let pstr = &mut (*pcm).streams[stream] as *mut snd_pcm_str;
        snd_info_free_entry((*pstr).oss.proc_entry);
        (*pstr).oss.proc_entry = ptr::null_mut();
        snd_pcm_oss_proc_free_setup_list(pstr);
    }
}

static mut snd_pcm_oss_f_reg: file_operations = file_operations {
    owner: ptr::null_mut(),
    read: Some(snd_pcm_oss_read),
    write: Some(snd_pcm_oss_write),
    open: Some(snd_pcm_oss_open),
    release: Some(snd_pcm_oss_release),
    poll: Some(snd_pcm_oss_poll),
    unlocked_ioctl: Some(snd_pcm_oss_ioctl),
    compat_ioctl: Some(snd_pcm_oss_ioctl_compat),
    mmap: Some(snd_pcm_oss_mmap),
};

unsafe fn register_oss_dsp(pcm: *mut snd_pcm, index: c_int) {
    if snd_register_oss_device(SNDRV_OSS_DEVICE_TYPE_PCM, (*pcm).card, index, &snd_pcm_oss_f_reg, pcm as *mut c_void) < 0 {
        /* pcm_err(pcm, "unable to register OSS PCM device %i:%i\n", ...); */
    }
}

unsafe extern "C" fn snd_pcm_oss_register_minor(pcm: *mut snd_pcm) -> c_int {
    (*pcm).oss.reg = 0;
    if dsp_map[(*(*pcm).card).number as usize] == (*pcm).device as c_int {
        let duplex = ((*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK].substream_count > 0 &&
            (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE].substream_count != 0 &&
            ((*pcm).info_flags & SNDRV_PCM_INFO_HALF_DUPLEX) == 0) as c_int;
        register_oss_dsp(pcm, 0);
        let _ = duplex;
        /* SNDRV_OSS_INFO_DEV_AUDIO registration is conditional. */
        (*pcm).oss.reg += 1;
        (*pcm).oss.reg_mask |= 1;
    }
    if adsp_map[(*(*pcm).card).number as usize] == (*pcm).device as c_int {
        register_oss_dsp(pcm, 1);
        (*pcm).oss.reg += 1;
        (*pcm).oss.reg_mask |= 2;
    }
    if (*pcm).oss.reg != 0 { snd_pcm_oss_proc_init(pcm); }
    0
}

unsafe extern "C" fn snd_pcm_oss_disconnect_minor(pcm: *mut snd_pcm) -> c_int {
    if (*pcm).oss.reg != 0 {
        if ((*pcm).oss.reg_mask & 1) != 0 {
            (*pcm).oss.reg_mask &= !1;
            snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_PCM, (*pcm).card, 0);
        }
        if ((*pcm).oss.reg_mask & 2) != 0 {
            (*pcm).oss.reg_mask &= !2;
            snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_PCM, (*pcm).card, 1);
        }
        if dsp_map[(*(*pcm).card).number as usize] == (*pcm).device as c_int {
            /* SNDRV_OSS_INFO_DEV_AUDIO unregister is conditional. */
        }
        (*pcm).oss.reg = 0;
    }
    0
}

unsafe extern "C" fn snd_pcm_oss_unregister_minor(pcm: *mut snd_pcm) -> c_int {
    snd_pcm_oss_disconnect_minor(pcm);
    snd_pcm_oss_proc_done(pcm);
    0
}

static mut snd_pcm_oss_notify: snd_pcm_notify = snd_pcm_notify {
    n_register: Some(snd_pcm_oss_register_minor),
    n_disconnect: Some(snd_pcm_oss_disconnect_minor),
    n_unregister: Some(snd_pcm_oss_unregister_minor),
};

unsafe extern "C" fn alsa_pcm_oss_init() -> c_int {
    /* check device map table */
    for i in 0..SNDRV_CARDS {
        if dsp_map[i] < 0 || dsp_map[i] >= SNDRV_PCM_DEVICES {
            dsp_map[i] = 0;
        }
        if adsp_map[i] < 0 || adsp_map[i] >= SNDRV_PCM_DEVICES {
            adsp_map[i] = 1;
        }
    }
    let err = snd_pcm_notify(&mut snd_pcm_oss_notify, 0);
    if err < 0 { return err; }
    0
}

unsafe extern "C" fn alsa_pcm_oss_exit() {
    snd_pcm_notify(&mut snd_pcm_oss_notify, 1);
}

/* module_init(alsa_pcm_oss_init) */
/* module_exit(alsa_pcm_oss_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
