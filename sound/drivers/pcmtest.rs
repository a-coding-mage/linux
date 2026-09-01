// SPDX-License-Identifier: GPL-2.0
/*
 * Virtual ALSA driver for PCM testing/fuzzing
 *
 * Copyright 2023 Ivan Orlov <ivan.orlov0322@gmail.com>
 *
 * This is a simple virtual ALSA driver, which can be used for audio applications/PCM middle layer
 * testing or fuzzing.
 * It can:
 *	- Simulate 'playback' and 'capture' actions
 *	- Generate random or pattern-based capture data
 *	- Check playback buffer for containing looped template, and notify about the results
 *	through the debugfs entry
 *	- Inject delays into the playback and capturing processes. See 'inject_delay' parameter.
 *	- Inject errors during the PCM callbacks.
 *	- Register custom RESET ioctl and notify when it is called through the debugfs entry
 *	- Work in interleaved and non-interleaved modes
 *	- Support up to 8 substreams
 *	- Support up to 4 channels
 *	- Support framerates from 8 kHz to 48 kHz
 *
 * When driver works in the capture mode with multiple channels, it duplicates the looped
 * pattern to each separate channel. For example, if we have 2 channels, format = U8, interleaved
 * access mode and pattern 'abacaba', the DMA buffer will look like aabbccaabbaaaa..., so buffer for
 * each channel will contain abacabaabacaba... Same for the non-interleaved mode.
 *
 * However, it may break the capturing on the higher framerates with small period size, so it is
 * better to choose larger period sizes.
 *
 * You can find the corresponding selftest in the 'alsa' selftests folder.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type u8 = u8;
type u32 = u32;
type bool_t = bool;
type snd_pcm_uframes_t = c_ulong;

const TIMER_PER_SEC: c_uint = 5;
const TIMER_INTERVAL: c_ulong = HZ / TIMER_PER_SEC as c_ulong;
const DELAY_JIFFIES: c_ulong = HZ;
const PLAYBACK_SUBSTREAM_CNT: c_int = 8;
const CAPTURE_SUBSTREAM_CNT: c_int = 8;
const MAX_CHANNELS_NUM: usize = 4;

const DEFAULT_PATTERN: &[u8; 8] = b"abacaba\0";
const DEFAULT_PATTERN_LEN: u32 = 7;

const FILL_MODE_RAND: i16 = 0;
const FILL_MODE_PAT: i16 = 1;

const MAX_PATTERN_LEN: usize = 4096;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_IOCTL1_RESET: c_uint = 0;
const SNDRV_PCM_ACCESS_RW_NONINTERLEAVED: c_int = 3;
const SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED: c_int = 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 1;
const SNDRV_PCM_INFO_NONINTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const THIS_MODULE: *mut c_void = ptr::null_mut();

// Kernel/module constants and helpers supplied by other files.
extern "C" {
    static mut HZ: c_ulong;
    static mut jiffies: c_ulong;

    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn get_random_bytes(buf: *mut c_void, nbytes: c_int);
    fn copy_from_user(to: *mut c_void, from: *const c_char, n: c_ulong) -> c_ulong;
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: c_ulong) -> c_ulong;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> ssize_t;
    fn strscpy_pad(dst: *mut c_char, src: *const c_char, count: size_t) -> ssize_t;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;

    fn timer_setup(timer: *mut timer_list, callback: Option<unsafe extern "C" fn(*mut timer_list)>, flags: c_uint);
    fn timer_shutdown_sync(timer: *mut timer_list);
    fn timer_delete(timer: *mut timer_list) -> c_int;
    fn timer_delete_sync(timer: *mut timer_list) -> c_int;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;

    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: size_t) -> snd_pcm_uframes_t;
    fn samples_to_bytes(runtime: *mut snd_pcm_runtime, samples: size_t) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_lib_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int,
                   capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, type_: c_int, dev: *mut device,
                                      size: size_t, max: size_t) -> c_int;
    fn snd_device_new(card: *mut snd_card, type_: c_int, device_data: *mut c_void,
                      ops: *const snd_device_ops) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void,
                         extra_size: c_int, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_device_register(pdev: *mut platform_device) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_register(pdrv: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(pdrv: *mut platform_driver);

    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_u8(name: *const c_char, mode: c_uint, parent: *mut dentry, value: *mut u8);
    fn debugfs_create_u32(name: *const c_char, mode: c_uint, parent: *mut dentry, value: *mut u32);
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry,
                           data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: u64,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: size_t,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
}

#[repr(C)]
struct pcmtst {
    pcm: *mut snd_pcm,
    card: *mut snd_card,
    pdev: *mut platform_device,
}

#[repr(C)]
struct pcmtst_buf_iter {
    buf_pos: size_t,          // position in the DMA buffer
    period_pos: size_t,       // period-relative position
    b_rw: size_t,             // Bytes to write on every timer tick
    s_rw_ch: size_t,          // Samples to write to one channel on every tick
    sample_bytes: c_uint,     // sample_bits / 8
    is_buf_corrupted: bool_t, // playback test result indicator
    period_bytes: size_t,     // bytes in a one period
    interleaved: bool_t,      // Interleaved/Non-interleaved mode
    total_bytes: size_t,      // Total bytes read/written
    chan_block: size_t,       // Bytes in one channel buffer when non-interleaved
    substream: *mut snd_pcm_substream,
    suspend: bool_t,          // We need to pause timer without shutting it down
    timer_instance: timer_list,
}

#[repr(C)]
struct pattern_buf {
    buf: *mut c_char,
    len: u32,
}

#[repr(C)]
struct timer_list {
    _private: [usize; 0],
}

#[repr(C)]
struct snd_pcm {
    private_data: *mut c_void,
    name: [c_char; 80],
}

#[repr(C)]
struct snd_card {
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    private_data: *mut c_void,
    dma_area: *mut u8,
    dma_bytes: size_t,
    channels: c_uint,
    access: c_int,
    rate: c_uint,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
}

#[repr(C)]
struct device {
    release: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
struct platform_device {
    name: *const c_char,
    dev: device,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: device_driver,
}

#[repr(C)]
struct snd_device {
    _private: [usize; 0],
}

#[repr(C)]
struct snd_device_ops {
    dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [usize; 0],
}

#[repr(C)]
struct inode {
    i_private: *mut c_void,
}

#[repr(C)]
struct file {
    f_inode: *mut inode,
}

#[repr(C)]
struct file_operations {
    read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    ioctl: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint, *mut c_void) -> c_int>,
    sync_stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
struct dentry {
    _private: [usize; 0],
}

static mut index: c_int = -1;
static mut id: *mut c_char = b"pcmtest\0".as_ptr() as *mut c_char;
static mut enable: bool_t = true;
static mut inject_delay: c_int = 0;
static mut inject_hwpars_err: bool_t = false;
static mut inject_prepare_err: bool_t = false;
static mut inject_trigger_err: bool_t = false;
static mut inject_open_err: bool_t = false;

static mut fill_mode: i16 = FILL_MODE_PAT;

static mut playback_capture_test: u8 = 0;
static mut ioctl_reset_test: u8 = 0;
static mut driver_debug_dir: *mut dentry = ptr::null_mut();

// module_param and MODULE_PARM_DESC declarations are Linux module metadata in C.

static mut snd_pcmtst_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_NONINTERLEAVED
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: MAX_CHANNELS_NUM as c_uint,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 4096,
    period_bytes_max: 32768,
    periods_min: 1,
    periods_max: 1024,
};

static mut buf_allocated: c_int = 0;
static mut patt_bufs: [pattern_buf; MAX_CHANNELS_NUM] = [
    pattern_buf { buf: ptr::null_mut(), len: 0 },
    pattern_buf { buf: ptr::null_mut(), len: 0 },
    pattern_buf { buf: ptr::null_mut(), len: 0 },
    pattern_buf { buf: ptr::null_mut(), len: 0 },
];

#[inline]
unsafe fn inc_buf_pos(v_iter: *mut pcmtst_buf_iter, by: size_t, bytes: size_t) {
    (*v_iter).total_bytes = (*v_iter).total_bytes.wrapping_add(by);
    (*v_iter).buf_pos = (*v_iter).buf_pos.wrapping_add(by);
    if (*v_iter).buf_pos >= bytes {
        (*v_iter).buf_pos %= bytes;
    }
}

/*
 * Position in the DMA buffer when we are in the non-interleaved mode. We increment buf_pos
 * every time we write a byte to any channel, so the position in the current channel buffer is
 * (position in the DMA buffer) / count_of_channels + size_of_channel_buf * current_channel
 */
#[inline]
unsafe fn buf_pos_n(v_iter: *mut pcmtst_buf_iter, channels: c_uint, chan_num: c_uint) -> size_t {
    (*v_iter).buf_pos / channels as size_t + (*v_iter).chan_block * chan_num as size_t
}

/*
 * Get the count of bytes written for the current channel in the interleaved mode.
 * This is (count of samples written for the current channel) * bytes_in_sample +
 * (relative position in the current sample)
 */
#[inline]
fn ch_pos_i(b_total: size_t, channels: c_uint, b_sample: c_uint) -> size_t {
    b_total / channels as size_t / b_sample as size_t * b_sample as size_t
        + (b_total % b_sample as size_t)
}

unsafe extern "C" fn check_buf_block_i(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    let mut i: size_t = 0;
    let mut ch_num: i16;
    let mut current_byte: u8;

    while i < (*v_iter).b_rw {
        current_byte = *(*runtime).dma_area.add((*v_iter).buf_pos);
        if current_byte == 0 {
            break;
        }
        ch_num = (((*v_iter).total_bytes / (*v_iter).sample_bytes as size_t)
            % (*runtime).channels as size_t) as i16;
        if current_byte
            != *patt_bufs[ch_num as usize].buf.add(
                ch_pos_i((*v_iter).total_bytes, (*runtime).channels, (*v_iter).sample_bytes)
                    % patt_bufs[ch_num as usize].len as size_t,
            ) as u8
        {
            (*v_iter).is_buf_corrupted = true;
            break;
        }
        inc_buf_pos(v_iter, 1, (*runtime).dma_bytes);
        i += 1;
    }
    // If we broke during the loop, add remaining bytes to the buffer position.
    inc_buf_pos(v_iter, (*v_iter).b_rw - i, (*runtime).dma_bytes);
}

unsafe extern "C" fn check_buf_block_ni(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    let channels: c_uint = (*runtime).channels;
    let mut i: size_t = 0;
    let mut ch_num: i16;
    let mut current_byte: u8;

    while i < (*v_iter).b_rw {
        ch_num = (i % channels as size_t) as i16;
        current_byte = *(*runtime)
            .dma_area
            .add(buf_pos_n(v_iter, channels, ch_num as c_uint));
        if current_byte == 0 {
            break;
        }
        if current_byte
            != *patt_bufs[ch_num as usize]
                .buf
                .add(((*v_iter).total_bytes / channels as size_t) % patt_bufs[ch_num as usize].len as size_t)
                as u8
        {
            (*v_iter).is_buf_corrupted = true;
            break;
        }
        inc_buf_pos(v_iter, 1, (*runtime).dma_bytes);
        i += 1;
    }
    inc_buf_pos(v_iter, (*v_iter).b_rw - i, (*runtime).dma_bytes);
}

/*
 * Check one block of the buffer. Here we iterate the buffer until we find '0'. This condition is
 * necessary because we need to detect when the reading/writing ends, so we assume that the pattern
 * doesn't contain zeros.
 */
unsafe extern "C" fn check_buf_block(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    if (*v_iter).interleaved {
        check_buf_block_i(v_iter, runtime);
    } else {
        check_buf_block_ni(v_iter, runtime);
    }
}

/*
 * Fill buffer in the non-interleaved mode. The order of samples is C0, ..., C0, C1, ..., C1, C2...
 * The channel buffers lay in the DMA buffer continuously (see default copy
 * handlers in the pcm_lib.c file).
 *
 * Here we increment the DMA buffer position every time we write a byte to any channel 'buffer'.
 * We need this to simulate the correct hardware pointer moving.
 */
unsafe extern "C" fn fill_block_pattern_n(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    let mut i: size_t = 0;
    let channels: c_uint = (*runtime).channels;
    let mut ch_num: i16;

    while i < (*v_iter).b_rw {
        ch_num = (i % channels as size_t) as i16;
        *(*runtime)
            .dma_area
            .add(buf_pos_n(v_iter, channels, ch_num as c_uint)) =
            *patt_bufs[ch_num as usize]
                .buf
                .add(((*v_iter).total_bytes / channels as size_t) % patt_bufs[ch_num as usize].len as size_t)
                as u8;
        inc_buf_pos(v_iter, 1, (*runtime).dma_bytes);
        i += 1;
    }
}

// Fill buffer in the interleaved mode. The order of samples is C0, C1, C2, C0, C1, C2, ...
unsafe extern "C" fn fill_block_pattern_i(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    let mut sample: size_t;
    let pos_in_ch: size_t;
    let mut pos_pattern: size_t;
    let mut ch: i16;
    let mut pos_sample: i16;

    pos_in_ch = ch_pos_i((*v_iter).total_bytes, (*runtime).channels, (*v_iter).sample_bytes);

    sample = 0;
    while sample < (*v_iter).s_rw_ch {
        ch = 0;
        while ch < (*runtime).channels as i16 {
            pos_sample = 0;
            while pos_sample < (*v_iter).sample_bytes as i16 {
                pos_pattern = (pos_in_ch
                    + sample * (*v_iter).sample_bytes as size_t
                    + pos_sample as size_t)
                    % patt_bufs[ch as usize].len as size_t;
                *(*runtime).dma_area.add((*v_iter).buf_pos) =
                    *patt_bufs[ch as usize].buf.add(pos_pattern) as u8;
                inc_buf_pos(v_iter, 1, (*runtime).dma_bytes);
                pos_sample += 1;
            }
            ch += 1;
        }
        sample += 1;
    }
}

unsafe extern "C" fn fill_block_pattern(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    if (*v_iter).interleaved {
        fill_block_pattern_i(v_iter, runtime);
    } else {
        fill_block_pattern_n(v_iter, runtime);
    }
}

unsafe extern "C" fn fill_block_rand_n(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    let channels: c_uint = (*runtime).channels;
    // Remaining space in all channel buffers
    let bytes_remain: size_t = (*runtime).dma_bytes - (*v_iter).buf_pos;
    let mut i: c_uint = 0;

    while i < channels {
        if (*v_iter).b_rw <= bytes_remain {
            //b_rw - count of bytes must be written for all channels at each timer tick
            get_random_bytes(
                (*runtime).dma_area.add(buf_pos_n(v_iter, channels, i)) as *mut c_void,
                ((*v_iter).b_rw / channels as size_t) as c_int,
            );
        } else {
            // Write to the end of buffer and start from the beginning of it
            get_random_bytes(
                (*runtime).dma_area.add(buf_pos_n(v_iter, channels, i)) as *mut c_void,
                (bytes_remain / channels as size_t) as c_int,
            );
            get_random_bytes(
                (*runtime).dma_area.add((*v_iter).chan_block * i as size_t) as *mut c_void,
                (((*v_iter).b_rw - bytes_remain) / channels as size_t) as c_int,
            );
        }
        i += 1;
    }
    inc_buf_pos(v_iter, (*v_iter).b_rw, (*runtime).dma_bytes);
}

unsafe extern "C" fn fill_block_rand_i(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    let in_cur_block: size_t = (*runtime).dma_bytes - (*v_iter).buf_pos;

    if (*v_iter).b_rw <= in_cur_block {
        get_random_bytes((*runtime).dma_area.add((*v_iter).buf_pos) as *mut c_void, (*v_iter).b_rw as c_int);
    } else {
        get_random_bytes((*runtime).dma_area.add((*v_iter).buf_pos) as *mut c_void, in_cur_block as c_int);
        get_random_bytes((*runtime).dma_area as *mut c_void, ((*v_iter).b_rw - in_cur_block) as c_int);
    }
    inc_buf_pos(v_iter, (*v_iter).b_rw, (*runtime).dma_bytes);
}

unsafe extern "C" fn fill_block_random(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    if (*v_iter).interleaved {
        fill_block_rand_i(v_iter, runtime);
    } else {
        fill_block_rand_n(v_iter, runtime);
    }
}

unsafe extern "C" fn fill_block(v_iter: *mut pcmtst_buf_iter, runtime: *mut snd_pcm_runtime) {
    match fill_mode {
        FILL_MODE_RAND => fill_block_random(v_iter, runtime),
        FILL_MODE_PAT => fill_block_pattern(v_iter, runtime),
        _ => {}
    }
}

/*
 * Here we iterate through the buffer by (buffer_size / iterates_per_second) bytes.
 * The driver uses timer to simulate the hardware pointer moving, and notify the PCM middle layer
 * about period elapsed.
 */
unsafe extern "C" fn timer_timeout(data: *mut timer_list) {
    let v_iter: *mut pcmtst_buf_iter =
        (data as *mut u8).sub(offset_of_timer_instance()) as *mut pcmtst_buf_iter;
    let substream: *mut snd_pcm_substream = (*v_iter).substream;

    if (*v_iter).suspend {
        return;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK && !(*v_iter).is_buf_corrupted {
        check_buf_block(v_iter, (*substream).runtime);
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        fill_block(v_iter, (*substream).runtime);
    } else {
        inc_buf_pos(v_iter, (*v_iter).b_rw, (*(*substream).runtime).dma_bytes);
    }

    (*v_iter).period_pos = (*v_iter).period_pos.wrapping_add((*v_iter).b_rw);
    if (*v_iter).period_pos >= (*v_iter).period_bytes {
        (*v_iter).period_pos %= (*v_iter).period_bytes;
        snd_pcm_period_elapsed(substream);
    }

    if !(*v_iter).suspend {
        mod_timer(
            &mut (*v_iter).timer_instance,
            jiffies + TIMER_INTERVAL + inject_delay as c_ulong,
        );
    }
}

const fn offset_of_timer_instance() -> usize {
    0
}

unsafe extern "C" fn snd_pcmtst_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let v_iter: *mut pcmtst_buf_iter;

    if inject_open_err {
        return -EBUSY;
    }

    v_iter = kzalloc(size_of::<pcmtst_buf_iter>(), GFP_KERNEL) as *mut pcmtst_buf_iter;
    if v_iter.is_null() {
        return -ENOMEM;
    }

    (*v_iter).substream = substream;
    (*runtime).hw = snd_pcmtst_hw;
    (*runtime).private_data = v_iter as *mut c_void;

    playback_capture_test = 0;
    ioctl_reset_test = 0;

    timer_setup(&mut (*v_iter).timer_instance, Some(timer_timeout), 0);

    0
}

unsafe extern "C" fn snd_pcmtst_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let v_iter: *mut pcmtst_buf_iter = (*(*substream).runtime).private_data as *mut pcmtst_buf_iter;

    timer_shutdown_sync(&mut (*v_iter).timer_instance);
    playback_capture_test = (!(*v_iter).is_buf_corrupted) as u8;
    kfree(v_iter as *mut c_void);
    0
}

#[inline]
unsafe fn reset_buf_iterator(v_iter: *mut pcmtst_buf_iter) {
    (*v_iter).buf_pos = 0;
    (*v_iter).is_buf_corrupted = false;
    (*v_iter).period_pos = 0;
    (*v_iter).total_bytes = 0;
}

#[inline]
unsafe fn start_pcmtest_timer(v_iter: *mut pcmtst_buf_iter) {
    (*v_iter).suspend = false;
    mod_timer(&mut (*v_iter).timer_instance, jiffies + TIMER_INTERVAL);
}

unsafe extern "C" fn snd_pcmtst_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let v_iter: *mut pcmtst_buf_iter = (*(*substream).runtime).private_data as *mut pcmtst_buf_iter;

    if inject_trigger_err {
        return -EINVAL;
    }
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            reset_buf_iterator(v_iter);
            start_pcmtest_timer(v_iter);
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            start_pcmtest_timer(v_iter);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            // We can't call timer_shutdown_sync here, as it is forbidden to sleep here
            (*v_iter).suspend = true;
            timer_delete(&mut (*v_iter).timer_instance);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn snd_pcmtst_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let v_iter: *mut pcmtst_buf_iter = (*(*substream).runtime).private_data as *mut pcmtst_buf_iter;

    bytes_to_frames((*substream).runtime, (*v_iter).buf_pos)
}

unsafe extern "C" fn snd_pcmtst_free(pcmtst: *mut pcmtst) -> c_int {
    if pcmtst.is_null() {
        return 0;
    }
    kfree(pcmtst as *mut c_void);
    0
}

// These callbacks are required, but empty - all freeing occurs in pdev_remove
unsafe extern "C" fn snd_pcmtst_dev_free(_device: *mut snd_device) -> c_int {
    0
}

unsafe extern "C" fn pcmtst_pdev_release(_dev: *mut device) {}

unsafe extern "C" fn snd_pcmtst_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let v_iter: *mut pcmtst_buf_iter = (*runtime).private_data as *mut pcmtst_buf_iter;

    if inject_prepare_err {
        return -EINVAL;
    }

    (*v_iter).sample_bytes = samples_to_bytes(runtime, 1);
    (*v_iter).period_bytes = snd_pcm_lib_period_bytes(substream);
    (*v_iter).interleaved = true;
    if (*runtime).access == SNDRV_PCM_ACCESS_RW_NONINTERLEAVED
        || (*runtime).access == SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED
    {
        (*v_iter).chan_block = snd_pcm_lib_buffer_bytes(substream) / (*runtime).channels as size_t;
        (*v_iter).interleaved = false;
    }
    // We want to record RATE * ch_cnt samples per sec, it is rate * sample_bytes * ch_cnt bytes
    (*v_iter).s_rw_ch = ((*runtime).rate / TIMER_PER_SEC) as size_t;
    (*v_iter).b_rw = (*v_iter).s_rw_ch * (*v_iter).sample_bytes as size_t * (*runtime).channels as size_t;

    0
}

unsafe extern "C" fn snd_pcmtst_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    if inject_hwpars_err {
        return -EBUSY;
    }
    0
}

unsafe extern "C" fn snd_pcmtst_pcm_hw_free(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_pcmtst_ioctl(
    substream: *mut snd_pcm_substream,
    cmd: c_uint,
    arg: *mut c_void,
) -> c_int {
    match cmd {
        SNDRV_PCM_IOCTL1_RESET => {
            ioctl_reset_test = 1;
        }
        _ => {}
    }
    snd_pcm_lib_ioctl(substream, cmd, arg)
}

unsafe extern "C" fn snd_pcmtst_sync_stop(substream: *mut snd_pcm_substream) -> c_int {
    let v_iter: *mut pcmtst_buf_iter = (*(*substream).runtime).private_data as *mut pcmtst_buf_iter;

    timer_delete_sync(&mut (*v_iter).timer_instance);

    0
}

static snd_pcmtst_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_pcmtst_pcm_open),
    close: Some(snd_pcmtst_pcm_close),
    trigger: Some(snd_pcmtst_pcm_trigger),
    hw_params: Some(snd_pcmtst_pcm_hw_params),
    ioctl: Some(snd_pcmtst_ioctl),
    sync_stop: Some(snd_pcmtst_sync_stop),
    hw_free: Some(snd_pcmtst_pcm_hw_free),
    prepare: Some(snd_pcmtst_pcm_prepare),
    pointer: Some(snd_pcmtst_pcm_pointer),
};

static snd_pcmtst_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_pcmtst_pcm_open),
    close: Some(snd_pcmtst_pcm_close),
    trigger: Some(snd_pcmtst_pcm_trigger),
    hw_params: Some(snd_pcmtst_pcm_hw_params),
    hw_free: Some(snd_pcmtst_pcm_hw_free),
    ioctl: Some(snd_pcmtst_ioctl),
    sync_stop: Some(snd_pcmtst_sync_stop),
    prepare: Some(snd_pcmtst_pcm_prepare),
    pointer: Some(snd_pcmtst_pcm_pointer),
};

unsafe extern "C" fn snd_pcmtst_new_pcm(pcmtst: *mut pcmtst) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new(
        (*pcmtst).card,
        b"PCMTest\0".as_ptr() as *const c_char,
        0,
        PLAYBACK_SUBSTREAM_CNT,
        CAPTURE_SUBSTREAM_CNT,
        &mut pcm,
    );
    if err < 0 {
        return err;
    }
    (*pcm).private_data = pcmtst as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), b"PCMTest\0".as_ptr() as *const c_char);
    (*pcmtst).pcm = pcm;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_pcmtst_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_pcmtst_capture_ops);

    err = snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        &mut (*(*pcmtst).pdev).dev,
        0,
        128 * 1024,
    );
    err
}

unsafe extern "C" fn snd_pcmtst_create(
    card: *mut snd_card,
    pdev: *mut platform_device,
    r_pcmtst: *mut *mut pcmtst,
) -> c_int {
    let pcmtst: *mut pcmtst;
    let mut err: c_int;
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_pcmtst_dev_free),
    };

    pcmtst = kzalloc(size_of::<pcmtst>(), GFP_KERNEL) as *mut pcmtst;
    if pcmtst.is_null() {
        return -ENOMEM;
    }
    (*pcmtst).card = card;
    (*pcmtst).pdev = pdev;

    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, pcmtst as *mut c_void, &ops);
    if err < 0 {
        snd_pcmtst_free(pcmtst);
        return err;
    }

    err = snd_pcmtst_new_pcm(pcmtst);
    if err < 0 {
        snd_pcmtst_free(pcmtst);
        return err;
    }

    *r_pcmtst = pcmtst;
    0
}

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 {
        !0u64
    } else {
        (1u64 << n) - 1
    }
}

unsafe extern "C" fn pcmtst_probe(pdev: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut pcmtst: *mut pcmtst = ptr::null_mut();
    let mut err: c_int;

    err = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(32));
    if err != 0 {
        return err;
    }

    err = snd_devm_card_new(&mut (*pdev).dev, index, id, THIS_MODULE, 0, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_pcmtst_create(card, pdev, &mut pcmtst);
    if err < 0 {
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), b"PCM-TEST Driver\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"PCM-Test\0".as_ptr() as *const c_char);
    strscpy((*card).longname.as_mut_ptr(), b"PCM-Test virtual driver\0".as_ptr() as *const c_char);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    platform_set_drvdata(pdev, pcmtst as *mut c_void);

    0
}

unsafe extern "C" fn pdev_remove(pdev: *mut platform_device) {
    let pcmtst: *mut pcmtst = platform_get_drvdata(pdev) as *mut pcmtst;

    snd_pcmtst_free(pcmtst);
}

static mut pcmtst_pdev: platform_device = platform_device {
    name: b"pcmtest\0".as_ptr() as *const c_char,
    dev: device {
        release: Some(pcmtst_pdev_release),
    },
};

static mut pcmtst_pdrv: platform_driver = platform_driver {
    probe: Some(pcmtst_probe),
    remove: Some(pdev_remove),
    driver: device_driver {
        name: b"pcmtest\0".as_ptr() as *const c_char,
    },
};

unsafe extern "C" fn pattern_write(
    file: *mut file,
    u_buff: *const c_char,
    len: size_t,
    off: *mut loff_t,
) -> ssize_t {
    let patt_buf: *mut pattern_buf = (*(*file).f_inode).i_private as *mut pattern_buf;
    let mut to_write: ssize_t = len as ssize_t;

    if *off + to_write as loff_t > MAX_PATTERN_LEN as loff_t {
        to_write = (MAX_PATTERN_LEN as loff_t - *off) as ssize_t;
    }

    // Crop silently everything over the buffer
    if to_write <= 0 {
        return len as ssize_t;
    }

    if copy_from_user((*patt_buf).buf.add(*off as usize) as *mut c_void, u_buff, to_write as c_ulong) != 0 {
        return -EFAULT as ssize_t;
    }

    (*patt_buf).len = (*off + to_write as loff_t) as u32;
    *off += to_write as loff_t;

    to_write
}

unsafe extern "C" fn pattern_read(
    file: *mut file,
    u_buff: *mut c_char,
    len: size_t,
    off: *mut loff_t,
) -> ssize_t {
    let patt_buf: *mut pattern_buf = (*(*file).f_inode).i_private as *mut pattern_buf;
    let mut to_read: ssize_t = len as ssize_t;

    if *off + to_read as loff_t >= MAX_PATTERN_LEN as loff_t {
        to_read = (MAX_PATTERN_LEN as loff_t - *off) as ssize_t;
    }
    if to_read <= 0 {
        return 0;
    }

    if copy_to_user(u_buff, (*patt_buf).buf.add(*off as usize) as *const c_void, to_read as c_ulong) != 0 {
        return -EFAULT as ssize_t;
    }

    *off += to_read as loff_t;

    to_read
}

static fill_pattern_fops: file_operations = file_operations {
    read: Some(pattern_read),
    write: Some(pattern_write),
};

unsafe extern "C" fn setup_patt_bufs() -> c_int {
    let mut i: size_t = 0;

    while i < patt_bufs.len() {
        patt_bufs[i].buf = kmalloc(MAX_PATTERN_LEN, GFP_KERNEL) as *mut c_char;
        if patt_bufs[i].buf.is_null() {
            break;
        }
        strscpy_pad(patt_bufs[i].buf, DEFAULT_PATTERN.as_ptr() as *const c_char, MAX_PATTERN_LEN);
        patt_bufs[i].len = DEFAULT_PATTERN_LEN;
        i += 1;
    }

    i as c_int
}

static pattern_files: [*const c_char; 4] = [
    b"fill_pattern0\0".as_ptr() as *const c_char,
    b"fill_pattern1\0".as_ptr() as *const c_char,
    b"fill_pattern2\0".as_ptr() as *const c_char,
    b"fill_pattern3\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn init_debug_files(buf_count: c_int) -> c_int {
    let mut i: size_t = 0;
    let mut len_file_name: [c_char; 32] = [0; 32];

    driver_debug_dir = debugfs_create_dir(b"pcmtest\0".as_ptr() as *const c_char, ptr::null_mut());
    if IS_ERR(driver_debug_dir as *const c_void) {
        return PTR_ERR(driver_debug_dir as *const c_void);
    }
    debugfs_create_u8(b"pc_test\0".as_ptr() as *const c_char, 0o444, driver_debug_dir, &mut playback_capture_test);
    debugfs_create_u8(b"ioctl_test\0".as_ptr() as *const c_char, 0o444, driver_debug_dir, &mut ioctl_reset_test);

    while i < buf_count as size_t {
        debugfs_create_file(
            pattern_files[i],
            0o600,
            driver_debug_dir,
            &mut patt_bufs[i] as *mut pattern_buf as *mut c_void,
            &fill_pattern_fops,
        );
        snprintf(
            len_file_name.as_mut_ptr(),
            len_file_name.len(),
            b"%s_len\0".as_ptr() as *const c_char,
            pattern_files[i],
        );
        debugfs_create_u32(len_file_name.as_mut_ptr(), 0o444, driver_debug_dir, &mut patt_bufs[i].len);
        i += 1;
    }

    0
}

unsafe extern "C" fn free_pattern_buffers() {
    let mut i: c_int = 0;

    while i < buf_allocated {
        kfree(patt_bufs[i as usize].buf as *mut c_void);
        i += 1;
    }
}

unsafe extern "C" fn clear_debug_files() {
    debugfs_remove_recursive(driver_debug_dir);
}

unsafe extern "C" fn mod_init() -> c_int {
    let mut err: c_int = 0;

    buf_allocated = setup_patt_bufs();
    if buf_allocated == 0 {
        return -ENOMEM;
    }

    snd_pcmtst_hw.channels_max = buf_allocated as c_uint;

    err = init_debug_files(buf_allocated);
    if err != 0 {
        free_pattern_buffers();
        return err;
    }
    err = platform_device_register(&mut pcmtst_pdev);
    if err != 0 {
        clear_debug_files();
        free_pattern_buffers();
        return err;
    }
    err = platform_driver_register(&mut pcmtst_pdrv);
    if err != 0 {
        platform_device_unregister(&mut pcmtst_pdev);
        clear_debug_files();
        free_pattern_buffers();
        return err;
    }

    0
}

unsafe extern "C" fn mod_exit() {
    clear_debug_files();
    free_pattern_buffers();

    platform_driver_unregister(&mut pcmtst_pdrv);
    platform_device_unregister(&mut pcmtst_pdev);
}

// MODULE_DESCRIPTION("Virtual ALSA driver for PCM testing/fuzzing");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Ivan Orlov");
// module_init(mod_init);
// module_exit(mod_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
