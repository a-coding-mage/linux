// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   32bit -> 64bit ioctl wrapper for PCM API
 *   Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

/* This file included from pcm_native.c */

use core::ffi::c_void;
use core::mem::{size_of, zeroed};
use core::ptr;

type Bool = bool;

extern "C" {
    fn snd_pcm_delay(substream: *mut snd_pcm_substream, delay: *mut snd_pcm_sframes_t) -> i32;
    fn snd_pcm_rewind(substream: *mut snd_pcm_substream, frames: snd_pcm_uframes_t) -> i32;
    fn snd_pcm_forward(substream: *mut snd_pcm_substream, frames: snd_pcm_uframes_t) -> i32;
    fn snd_pcm_sw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_sw_params) -> i32;
    fn recalculate_boundary(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    fn snd_pcm_channel_info(substream: *mut snd_pcm_substream, info: *mut snd_pcm_channel_info) -> i32;
    fn snd_pcm_status64(substream: *mut snd_pcm_substream, status: *mut snd_pcm_status64) -> i32;
    fn clear_user(to: *mut c_void, n: usize) -> i32;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> i32;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> i32;
    fn snd_pcm_hw_refine(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> i32;
    fn fixup_unreferenced_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> i32;
    fn snd_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> i32;
    fn snd_pcm_get_state(substream: *mut snd_pcm_substream) -> snd_pcm_state_t;
    fn snd_pcm_lib_write(substream: *mut snd_pcm_substream, buf: *mut c_void, frames: u32) -> i32;
    fn snd_pcm_lib_read(substream: *mut snd_pcm_substream, buf: *mut c_void, frames: u32) -> i32;
    fn snd_pcm_lib_writev(substream: *mut snd_pcm_substream, bufs: *mut *mut c_void, frames: u32) -> i32;
    fn snd_pcm_lib_readv(substream: *mut snd_pcm_substream, bufs: *mut *mut c_void, frames: u32) -> i32;
    fn snd_pcm_hwsync(substream: *mut snd_pcm_substream) -> i32;
    fn pcm_lib_apply_appl_ptr(substream: *mut snd_pcm_substream, appl_ptr: snd_pcm_uframes_t) -> i32;
    fn snd_pcm_dma_buffer_sync(substream: *mut snd_pcm_substream, direction: i32);
    fn snd_pcm_common_ioctl(
        file: *mut file,
        substream: *mut snd_pcm_substream,
        cmd: u32,
        arg: *mut c_void,
    ) -> isize;
    fn snd_pcm_status_user32(substream: *mut snd_pcm_substream, src: *mut c_void, ext: Bool) -> isize;
}

#[allow(non_camel_case_types)]
type u8 = ::core::ffi::c_uchar;
#[allow(non_camel_case_types)]
type u32 = ::core::ffi::c_uint;
#[allow(non_camel_case_types)]
type s32 = ::core::ffi::c_int;
#[allow(non_camel_case_types)]
type s64 = ::core::ffi::c_longlong;
#[allow(non_camel_case_types)]
type compat_caddr_t = u32;
#[allow(non_camel_case_types)]
type snd_pcm_sframes_t = isize;
#[allow(non_camel_case_types)]
type snd_pcm_uframes_t = usize;
#[allow(non_camel_case_types)]
type snd_pcm_state_t = i32;

const EFAULT: i32 = 14;
const ENOTTY: i32 = 25;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const EBADFD: i32 = 77;
const ENOIOCTLCMD: i32 = 515;
const GFP_KERNEL: u32 = 0;

extern "C" {
    static SNDRV_PCM_HW_PARAM_LAST_MASK: usize;
    static SNDRV_PCM_HW_PARAM_FIRST_MASK: usize;
    static SNDRV_PCM_HW_PARAM_LAST_INTERVAL: usize;
    static SNDRV_PCM_HW_PARAM_FIRST_INTERVAL: usize;
    static SNDRV_PCM_STATE_OPEN: snd_pcm_state_t;
    static SNDRV_PCM_STREAM_PLAYBACK: i32;
    static SNDRV_PCM_STREAM_CAPTURE: i32;
    static SNDRV_PCM_SYNC_PTR_HWSYNC: u32;
    static SNDRV_PCM_SYNC_PTR_APPL: u32;
    static SNDRV_PCM_SYNC_PTR_AVAIL_MIN: u32;
    static SNDRV_DMA_SYNC_DEVICE: i32;
    static SNDRV_PCM_IOCTL_PVERSION: u32;
    static SNDRV_PCM_IOCTL_INFO: u32;
    static SNDRV_PCM_IOCTL_TSTAMP: u32;
    static SNDRV_PCM_IOCTL_TTSTAMP: u32;
    static SNDRV_PCM_IOCTL_USER_PVERSION: u32;
    static SNDRV_PCM_IOCTL_HWSYNC: u32;
    static SNDRV_PCM_IOCTL_PREPARE: u32;
    static SNDRV_PCM_IOCTL_RESET: u32;
    static SNDRV_PCM_IOCTL_START: u32;
    static SNDRV_PCM_IOCTL_DROP: u32;
    static SNDRV_PCM_IOCTL_DRAIN: u32;
    static SNDRV_PCM_IOCTL_PAUSE: u32;
    static SNDRV_PCM_IOCTL_HW_FREE: u32;
    static SNDRV_PCM_IOCTL_RESUME: u32;
    static SNDRV_PCM_IOCTL_XRUN: u32;
    static SNDRV_PCM_IOCTL_LINK: u32;
    static SNDRV_PCM_IOCTL_UNLINK: u32;
    static __SNDRV_PCM_IOCTL_SYNC_PTR32: u32;
    static __SNDRV_PCM_IOCTL_SYNC_PTR64: u32;
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __snd_timespec64 {
    pub tv_sec: s64,
    pub tv_nsec: s64,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: i32,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub status: *mut snd_pcm_mmap_status,
    pub control: *mut snd_pcm_mmap_control,
    pub channels: i32,
    pub boundary: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_sw_params {
    pub tstamp_mode: s32,
    pub period_step: u32,
    pub sleep_min: u32,
    pub avail_min: u32,
    pub xfer_align: u32,
    pub start_threshold: u32,
    pub stop_threshold: u32,
    pub silence_threshold: u32,
    pub silence_size: u32,
    pub boundary: snd_pcm_uframes_t,
    pub proto: u32,
    pub tstamp_type: u32,
}

#[repr(C)]
pub struct snd_pcm_channel_info {
    pub channel: u32,
    pub offset: u32,
    pub first: u32,
    pub step: u32,
}

#[repr(C)]
pub struct snd_pcm_status64 {
    pub state: snd_pcm_state_t,
    pub trigger_tstamp_sec: s64,
    pub trigger_tstamp_nsec: s64,
    pub tstamp_sec: s64,
    pub tstamp_nsec: s64,
    pub appl_ptr: u32,
    pub hw_ptr: u32,
    pub delay: s32,
    pub avail: u32,
    pub avail_max: u32,
    pub overrange: u32,
    pub suspended_state: snd_pcm_state_t,
    pub audio_tstamp_data: u32,
    pub audio_tstamp_sec: s64,
    pub audio_tstamp_nsec: s64,
    pub audio_tstamp_accuracy: u32,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub flags: u32,
    pub fifo_size: u32,
}

#[repr(C)]
pub struct snd_pcm_mmap_status {
    pub state: snd_pcm_state_t,
    pub hw_ptr: snd_pcm_uframes_t,
    pub tstamp: __snd_timespec64,
    pub suspended_state: snd_pcm_state_t,
    pub audio_tstamp: __snd_timespec64,
}

#[repr(C)]
pub struct snd_pcm_mmap_control {
    pub appl_ptr: snd_pcm_uframes_t,
    pub avail_min: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_sync_ptr_status_union {
    pub status: snd_pcm_mmap_status,
}

#[repr(C)]
pub struct snd_pcm_sync_ptr_control_union {
    pub control: snd_pcm_mmap_control,
}

#[repr(C)]
pub struct snd_pcm_sync_ptr {
    pub flags: u32,
    pub s: snd_pcm_sync_ptr_status_union,
    pub c: snd_pcm_sync_ptr_control_union,
}

#[repr(C)]
pub struct snd_pcm_file {
    pub substream: *mut snd_pcm_substream,
    pub no_compat_mmap: i32,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut snd_pcm_file,
}

unsafe fn get_user<T: Copy>(dst: *mut T, src: *const T) -> i32 {
    if src.is_null() || dst.is_null() {
        return 1;
    }
    ptr::write(dst, ptr::read(src));
    0
}

unsafe fn put_user<T>(val: T, dst: *mut T) -> i32 {
    if dst.is_null() {
        return 1;
    }
    ptr::write(dst, val);
    0
}

fn compat_ptr(addr: compat_caddr_t) -> *mut c_void {
    addr as usize as *mut c_void
}

unsafe fn kfree<T>(ptr: *mut T) {
    extern "C" {
        fn kfree(ptr: *mut c_void);
    }
    if !ptr.is_null() {
        kfree(ptr as *mut c_void);
    }
}

unsafe fn kmalloc_obj<T>() -> *mut T {
    extern "C" {
        fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    }
    kmalloc(size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn kmalloc_array<T>(n: usize) -> *mut T {
    extern "C" {
        fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut c_void;
    }
    kmalloc_array(n, size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn snd_pcm_ioctl_delay_compat(substream: *mut snd_pcm_substream, src: *mut s32) -> i32 {
    let mut delay: snd_pcm_sframes_t = 0;
    let mut err: i32;

    err = snd_pcm_delay(substream, &mut delay);
    if err != 0 {
        return err;
    }
    if put_user(delay as s32, src) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_pcm_ioctl_rewind_compat(substream: *mut snd_pcm_substream, src: *mut u32) -> i32 {
    let mut frames: snd_pcm_uframes_t = 0;
    let err: i32;

    if get_user(&mut frames as *mut _ as *mut u32, src) != 0 {
        return -EFAULT;
    }
    err = snd_pcm_rewind(substream, frames);
    if put_user(err as u32, src) != 0 {
        return -EFAULT;
    }
    if err < 0 { err } else { 0 }
}

unsafe fn snd_pcm_ioctl_forward_compat(substream: *mut snd_pcm_substream, src: *mut u32) -> i32 {
    let mut frames: snd_pcm_uframes_t = 0;
    let err: i32;

    if get_user(&mut frames as *mut _ as *mut u32, src) != 0 {
        return -EFAULT;
    }
    err = snd_pcm_forward(substream, frames);
    if put_user(err as u32, src) != 0 {
        return -EFAULT;
    }
    if err < 0 { err } else { 0 }
}

#[repr(C)]
pub struct snd_pcm_hw_params32 {
    pub flags: u32,
    pub masks: [snd_mask; 1], /* this must be identical */
    pub mres: [snd_mask; 5], /* reserved masks */
    pub intervals: [snd_interval; 1],
    pub ires: [snd_interval; 9], /* reserved intervals */
    pub rmask: u32,
    pub cmask: u32,
    pub info: u32,
    pub msbits: u32,
    pub rate_num: u32,
    pub rate_den: u32,
    pub fifo_size: u32,
    pub reserved: [u8; 64],
}

#[repr(C)]
pub struct snd_pcm_sw_params32 {
    pub tstamp_mode: s32,
    pub period_step: u32,
    pub sleep_min: u32,
    pub avail_min: u32,
    pub xfer_align: u32,
    pub start_threshold: u32,
    pub stop_threshold: u32,
    pub silence_threshold: u32,
    pub silence_size: u32,
    pub boundary: u32,
    pub proto: u32,
    pub tstamp_type: u32,
    pub reserved: [u8; 56],
}

unsafe fn snd_pcm_ioctl_sw_params_compat(
    substream: *mut snd_pcm_substream,
    src: *mut snd_pcm_sw_params32,
) -> i32 {
    let mut params: snd_pcm_sw_params = zeroed();
    let boundary: snd_pcm_uframes_t;
    let err: i32;

    if get_user(&mut params.tstamp_mode, &(*src).tstamp_mode)
        | get_user(&mut params.period_step, &(*src).period_step)
        | get_user(&mut params.sleep_min, &(*src).sleep_min)
        | get_user(&mut params.avail_min, &(*src).avail_min)
        | get_user(&mut params.xfer_align, &(*src).xfer_align)
        | get_user(&mut params.start_threshold, &(*src).start_threshold)
        | get_user(&mut params.stop_threshold, &(*src).stop_threshold)
        | get_user(&mut params.silence_threshold, &(*src).silence_threshold)
        | get_user(&mut params.silence_size, &(*src).silence_size)
        | get_user(&mut params.tstamp_type, &(*src).tstamp_type)
        | get_user(&mut params.proto, &(*src).proto)
        != 0
    {
        return -EFAULT;
    }
    /*
     * Check silent_size parameter.  Since we have 64bit boundary,
     * silence_size must be compared with the 32bit boundary.
     */
    boundary = recalculate_boundary((*substream).runtime);
    if boundary != 0 && params.silence_size as snd_pcm_uframes_t >= boundary {
        params.silence_size = (*(*substream).runtime).boundary as u32;
    }
    err = snd_pcm_sw_params(substream, &mut params);
    if err < 0 {
        return err;
    }
    if boundary != 0 && put_user(boundary as u32, &mut (*src).boundary) != 0 {
        return -EFAULT;
    }
    err
}

#[repr(C)]
pub struct snd_pcm_channel_info32 {
    pub channel: u32,
    pub offset: u32,
    pub first: u32,
    pub step: u32,
}

unsafe fn snd_pcm_ioctl_channel_info_compat(
    substream: *mut snd_pcm_substream,
    src: *mut snd_pcm_channel_info32,
) -> i32 {
    let mut info: snd_pcm_channel_info = zeroed();
    let err: i32;

    if get_user(&mut info.channel, &(*src).channel)
        | get_user(&mut info.offset, &(*src).offset)
        | get_user(&mut info.first, &(*src).first)
        | get_user(&mut info.step, &(*src).step)
        != 0
    {
        return -EFAULT;
    }
    err = snd_pcm_channel_info(substream, &mut info);
    if err < 0 {
        return err;
    }
    if put_user(info.channel, &mut (*src).channel)
        | put_user(info.offset, &mut (*src).offset)
        | put_user(info.first, &mut (*src).first)
        | put_user(info.step, &mut (*src).step)
        != 0
    {
        return -EFAULT;
    }
    err
}

// CONFIG_X86_X32_ABI: X32 ABI has the same struct as x86-64 for snd_pcm_channel_info.
#[cfg(CONFIG_X86_X32_ABI)]
extern "C" {
    fn snd_pcm_channel_info_user(
        substream: *mut snd_pcm_substream,
        src: *mut snd_pcm_channel_info,
    ) -> i32;
}

#[repr(C, packed)]
pub struct compat_snd_pcm_status64 {
    pub state: snd_pcm_state_t,
    pub rsvd: [u8; 4], /* alignment */
    pub trigger_tstamp_sec: s64,
    pub trigger_tstamp_nsec: s64,
    pub tstamp_sec: s64,
    pub tstamp_nsec: s64,
    pub appl_ptr: u32,
    pub hw_ptr: u32,
    pub delay: s32,
    pub avail: u32,
    pub avail_max: u32,
    pub overrange: u32,
    pub suspended_state: snd_pcm_state_t,
    pub audio_tstamp_data: u32,
    pub audio_tstamp_sec: s64,
    pub audio_tstamp_nsec: s64,
    pub driver_tstamp_sec: s64,
    pub driver_tstamp_nsec: s64,
    pub audio_tstamp_accuracy: u32,
    pub reserved: [u8; 52 - 4 * 8],
}

unsafe fn snd_pcm_status_user_compat64(
    substream: *mut snd_pcm_substream,
    src: *mut compat_snd_pcm_status64,
    ext: Bool,
) -> i32 {
    let mut status: snd_pcm_status64 = zeroed();
    let mut compat_status64: compat_snd_pcm_status64 = zeroed();
    let err: i32;

    /*
     * with extension, parameters are read/write,
     * get audio_tstamp_data from user,
     * ignore rest of status structure
     */
    if ext
        && get_user(
            &mut status.audio_tstamp_data,
            &(*src).audio_tstamp_data as *const _,
        ) != 0
    {
        return -EFAULT;
    }
    err = snd_pcm_status64(substream, &mut status);
    if err < 0 {
        return err;
    }

    if clear_user(src as *mut c_void, size_of::<compat_snd_pcm_status64>()) != 0 {
        return -EFAULT;
    }

    compat_status64 = compat_snd_pcm_status64 {
        state: status.state,
        rsvd: [0; 4],
        trigger_tstamp_sec: status.trigger_tstamp_sec,
        trigger_tstamp_nsec: status.trigger_tstamp_nsec,
        tstamp_sec: status.tstamp_sec,
        tstamp_nsec: status.tstamp_nsec,
        appl_ptr: status.appl_ptr,
        hw_ptr: status.hw_ptr,
        delay: status.delay,
        avail: status.avail,
        avail_max: status.avail_max,
        overrange: status.overrange,
        suspended_state: status.suspended_state,
        audio_tstamp_data: status.audio_tstamp_data,
        audio_tstamp_sec: status.audio_tstamp_sec,
        audio_tstamp_nsec: status.audio_tstamp_nsec,
        driver_tstamp_sec: status.audio_tstamp_sec,
        driver_tstamp_nsec: status.audio_tstamp_nsec,
        audio_tstamp_accuracy: status.audio_tstamp_accuracy,
        reserved: [0; 52 - 4 * 8],
    };

    if copy_to_user(
        src as *mut c_void,
        &compat_status64 as *const _ as *const c_void,
        size_of::<compat_snd_pcm_status64>(),
    ) != 0
    {
        return -EFAULT;
    }

    err
}

/* both for HW_PARAMS and HW_REFINE */
unsafe fn snd_pcm_ioctl_hw_params_compat(
    substream: *mut snd_pcm_substream,
    refine: i32,
    data32: *mut snd_pcm_hw_params32,
) -> i32 {
    let runtime: *mut snd_pcm_runtime;
    let mut err: i32;

    runtime = (*substream).runtime;
    if runtime.is_null() {
        return -ENOTTY;
    }

    let data: *mut snd_pcm_hw_params = kmalloc_obj();
    if data.is_null() {
        return -ENOMEM;
    }

    /* only fifo_size (RO from userspace) is different, so just copy all */
    if copy_from_user(
        data as *mut c_void,
        data32 as *const c_void,
        size_of::<snd_pcm_hw_params32>(),
    ) != 0
    {
        kfree(data);
        return -EFAULT;
    }

    if refine != 0 {
        err = snd_pcm_hw_refine(substream, data);
        if err < 0 {
            kfree(data);
            return err;
        }
        err = fixup_unreferenced_params(substream, data);
    } else {
        err = snd_pcm_hw_params(substream, data);
    }
    if err < 0 {
        kfree(data);
        return err;
    }
    if copy_to_user(
        data32 as *mut c_void,
        data as *const c_void,
        size_of::<snd_pcm_hw_params32>(),
    ) != 0
        || put_user((*data).fifo_size, &mut (*data32).fifo_size) != 0
    {
        kfree(data);
        return -EFAULT;
    }

    if refine == 0 {
        let new_boundary: u32 = recalculate_boundary(runtime) as u32;
        if new_boundary != 0 {
            (*runtime).boundary = new_boundary as snd_pcm_uframes_t;
        }
    }
    kfree(data);
    err
}

/*
 */
#[repr(C)]
pub struct snd_xferi32 {
    pub result: s32,
    pub buf: u32,
    pub frames: u32,
}

unsafe fn snd_pcm_ioctl_xferi_compat(
    substream: *mut snd_pcm_substream,
    dir: i32,
    data32: *mut snd_xferi32,
) -> i32 {
    let mut buf: compat_caddr_t = 0;
    let mut frames: u32 = 0;
    let err: i32;

    if (*substream).runtime.is_null() {
        return -ENOTTY;
    }
    if (*substream).stream != dir {
        return -EINVAL;
    }
    if snd_pcm_get_state(substream) == SNDRV_PCM_STATE_OPEN {
        return -EBADFD;
    }

    if get_user(&mut buf, &(*data32).buf) | get_user(&mut frames, &(*data32).frames) != 0 {
        return -EFAULT;
    }

    if dir == SNDRV_PCM_STREAM_PLAYBACK {
        err = snd_pcm_lib_write(substream, compat_ptr(buf), frames);
    } else {
        err = snd_pcm_lib_read(substream, compat_ptr(buf), frames);
    }
    if err < 0 {
        return err;
    }
    /* copy the result */
    if put_user(err, &mut (*data32).result) != 0 {
        return -EFAULT;
    }
    0
}

/* snd_xfern needs remapping of bufs */
#[repr(C)]
pub struct snd_xfern32 {
    pub result: s32,
    pub bufs: u32, /* this is void **; */
    pub frames: u32,
}

/*
 * xfern ioctl nees to copy (up to) 128 pointers on stack.
 * although we may pass the copied pointers through f_op->ioctl, but the ioctl
 * handler there expands again the same 128 pointers on stack, so it is better
 * to handle the function (calling pcm_readv/writev) directly in this handler.
 */
unsafe fn snd_pcm_ioctl_xfern_compat(
    substream: *mut snd_pcm_substream,
    dir: i32,
    data32: *mut snd_xfern32,
) -> i32 {
    let mut buf: compat_caddr_t = 0;
    let mut bufptr: *mut compat_caddr_t;
    let mut frames: u32 = 0;
    let err: i32;
    let ch: i32;
    let mut i: i32;

    if (*substream).runtime.is_null() {
        return -ENOTTY;
    }
    if (*substream).stream != dir {
        return -EINVAL;
    }
    if snd_pcm_get_state(substream) == SNDRV_PCM_STATE_OPEN {
        return -EBADFD;
    }

    ch = (*(*substream).runtime).channels;
    if ch > 128 {
        return -EINVAL;
    }
    if get_user(&mut buf, &(*data32).bufs) | get_user(&mut frames, &(*data32).frames) != 0 {
        return -EFAULT;
    }
    bufptr = compat_ptr(buf) as *mut compat_caddr_t;

    let bufs: *mut *mut c_void = kmalloc_array(ch as usize);
    if bufs.is_null() {
        return -ENOMEM;
    }
    i = 0;
    while i < ch {
        let mut ptr32: u32 = 0;
        if get_user(&mut ptr32, bufptr) != 0 {
            kfree(bufs);
            return -EFAULT;
        }
        *bufs.add(i as usize) = compat_ptr(ptr32);
        bufptr = bufptr.add(1);
        i += 1;
    }
    if dir == SNDRV_PCM_STREAM_PLAYBACK {
        err = snd_pcm_lib_writev(substream, bufs, frames);
    } else {
        err = snd_pcm_lib_readv(substream, bufs, frames);
    }
    if err >= 0 && put_user(err, &mut (*data32).result) != 0 {
        kfree(bufs);
        return -EFAULT;
    }
    kfree(bufs);
    err
}

// CONFIG_X86_X32_ABI: X32 ABI has 64bit timespec and 64bit alignment.
#[cfg(CONFIG_X86_X32_ABI)]
#[repr(C, packed)]
pub struct snd_pcm_mmap_status_x32 {
    pub state: snd_pcm_state_t,
    pub pad1: s32,
    pub hw_ptr: u32,
    pub pad2: u32, /* alignment */
    pub tstamp: __snd_timespec64,
    pub suspended_state: snd_pcm_state_t,
    pub pad3: s32,
    pub audio_tstamp: __snd_timespec64,
}

#[cfg(CONFIG_X86_X32_ABI)]
#[repr(C)]
pub struct snd_pcm_mmap_control_x32 {
    pub appl_ptr: u32,
    pub avail_min: u32,
}

#[cfg(CONFIG_X86_X32_ABI)]
#[repr(C, packed)]
pub struct snd_pcm_sync_ptr_x32_status_union {
    pub status: snd_pcm_mmap_status_x32,
}

#[cfg(CONFIG_X86_X32_ABI)]
#[repr(C, packed)]
pub struct snd_pcm_sync_ptr_x32_control_union {
    pub control: snd_pcm_mmap_control_x32,
}

#[cfg(CONFIG_X86_X32_ABI)]
#[repr(C, packed)]
pub struct snd_pcm_sync_ptr_x32 {
    pub flags: u32,
    pub rsvd: u32, /* alignment */
    pub s: snd_pcm_sync_ptr_x32_status_union,
    pub c: snd_pcm_sync_ptr_x32_control_union,
}

#[cfg(CONFIG_X86_X32_ABI)]
extern "C" {
    fn snd_pcm_sync_ptr_get_user(
        sflags: *mut u32,
        scontrol: *mut snd_pcm_mmap_control,
        src: *mut snd_pcm_sync_ptr_x32,
    ) -> i32;
    fn snd_pcm_sync_ptr_put_user(
        sstatus: *mut snd_pcm_mmap_status,
        scontrol: *mut snd_pcm_mmap_control,
        src: *mut snd_pcm_sync_ptr_x32,
    ) -> i32;
    fn pcm_stream_lock_irq(substream: *mut snd_pcm_substream);
    fn pcm_stream_unlock_irq(substream: *mut snd_pcm_substream);
    fn in_x32_syscall() -> Bool;
}

#[cfg(CONFIG_X86_X32_ABI)]
unsafe fn snd_pcm_ioctl_sync_ptr_x32(
    substream: *mut snd_pcm_substream,
    src: *mut snd_pcm_sync_ptr_x32,
) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let status: *mut snd_pcm_mmap_status;
    let control: *mut snd_pcm_mmap_control;
    let mut sflags: u32 = 0;
    let mut scontrol: snd_pcm_mmap_control = zeroed();
    let mut sstatus: snd_pcm_mmap_status = zeroed();
    let mut boundary: snd_pcm_uframes_t;
    let mut err: i32;

    if runtime.is_null() {
        return -EINVAL;
    }

    if snd_pcm_sync_ptr_get_user(&mut sflags, &mut scontrol, src) != 0 {
        return -EFAULT;
    }
    if sflags & SNDRV_PCM_SYNC_PTR_HWSYNC != 0 {
        err = snd_pcm_hwsync(substream);
        if err < 0 {
            return err;
        }
    }
    status = (*runtime).status;
    control = (*runtime).control;
    boundary = recalculate_boundary(runtime);
    if boundary == 0 {
        boundary = 0x7fffffff;
    }
    pcm_stream_lock_irq(substream);
    if sflags & SNDRV_PCM_SYNC_PTR_APPL == 0 {
        err = pcm_lib_apply_appl_ptr(substream, scontrol.appl_ptr);
        if err < 0 {
            pcm_stream_unlock_irq(substream);
            return err;
        }
    } else {
        scontrol.appl_ptr = (*control).appl_ptr % boundary;
    }
    if sflags & SNDRV_PCM_SYNC_PTR_AVAIL_MIN == 0 {
        (*control).avail_min = scontrol.avail_min;
    } else {
        scontrol.avail_min = (*control).avail_min;
    }
    sstatus.state = (*status).state;
    sstatus.hw_ptr = (*status).hw_ptr % boundary;
    sstatus.tstamp = (*status).tstamp;
    sstatus.suspended_state = (*status).suspended_state;
    sstatus.audio_tstamp = (*status).audio_tstamp;
    pcm_stream_unlock_irq(substream);
    if sflags & SNDRV_PCM_SYNC_PTR_APPL == 0 {
        snd_pcm_dma_buffer_sync(substream, SNDRV_DMA_SYNC_DEVICE);
    }
    if snd_pcm_sync_ptr_put_user(&mut sstatus, &mut scontrol, src) != 0 {
        return -EFAULT;
    }

    0
}

#[cfg(target_endian = "big")]
#[allow(non_camel_case_types)]
type __pad_before_u32 = [u8; 4];
#[cfg(target_endian = "big")]
#[allow(non_camel_case_types)]
type __pad_after_u32 = [u8; 0];
#[cfg(not(target_endian = "big"))]
#[allow(non_camel_case_types)]
type __pad_before_u32 = [u8; 0];
#[cfg(not(target_endian = "big"))]
#[allow(non_camel_case_types)]
type __pad_after_u32 = [u8; 4];

#[allow(non_camel_case_types)]
type __pad_after_uframe = [u8; 4];

/* PCM 2.0.15 API definition had a bug in mmap control; it puts the avail_min
 * at the wrong offset due to a typo in padding type.
 * The bug hits only 32bit.
 * A workaround for incorrect read/write is needed only in 32bit compat mode.
 */
#[repr(C)]
pub struct __snd_pcm_mmap_control64_buggy {
    pub __pad1: __pad_before_u32,
    pub appl_ptr: u32,
    pub __pad2: __pad_before_u32, /* SiC! here is the bug */
    pub __pad3: __pad_before_u32,
    pub avail_min: u32,
    pub __pad4: __pad_after_uframe,
}

extern "C" {
    fn pcm_stream_lock_irq(substream: *mut snd_pcm_substream);
    fn pcm_stream_unlock_irq(substream: *mut snd_pcm_substream);
}

unsafe fn snd_pcm_ioctl_sync_ptr_buggy(
    substream: *mut snd_pcm_substream,
    _sync_ptr: *mut snd_pcm_sync_ptr,
) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut sync_ptr: snd_pcm_sync_ptr = zeroed();
    let sync_cp: *mut __snd_pcm_mmap_control64_buggy;
    let status: *mut snd_pcm_mmap_status;
    let control: *mut snd_pcm_mmap_control;
    let mut err: i32;

    sync_cp = &mut sync_ptr.c.control as *mut _ as *mut __snd_pcm_mmap_control64_buggy;
    if get_user(&mut sync_ptr.flags, &(*_sync_ptr).flags) != 0 {
        return -EFAULT;
    }
    if copy_from_user(
        sync_cp as *mut c_void,
        &mut (*_sync_ptr).c.control as *mut _ as *const c_void,
        size_of::<__snd_pcm_mmap_control64_buggy>(),
    ) != 0
    {
        return -EFAULT;
    }
    status = (*runtime).status;
    control = (*runtime).control;
    if sync_ptr.flags & SNDRV_PCM_SYNC_PTR_HWSYNC != 0 {
        err = snd_pcm_hwsync(substream);
        if err < 0 {
            return err;
        }
    }
    pcm_stream_lock_irq(substream);
    if sync_ptr.flags & SNDRV_PCM_SYNC_PTR_APPL == 0 {
        err = pcm_lib_apply_appl_ptr(substream, (*sync_cp).appl_ptr as snd_pcm_uframes_t);
        if err < 0 {
            pcm_stream_unlock_irq(substream);
            return err;
        }
    } else {
        (*sync_cp).appl_ptr = (*control).appl_ptr as u32;
    }
    if sync_ptr.flags & SNDRV_PCM_SYNC_PTR_AVAIL_MIN == 0 {
        (*control).avail_min = (*sync_cp).avail_min as snd_pcm_uframes_t;
    } else {
        (*sync_cp).avail_min = (*control).avail_min as u32;
    }
    sync_ptr.s.status.state = (*status).state;
    sync_ptr.s.status.hw_ptr = (*status).hw_ptr;
    sync_ptr.s.status.tstamp = (*status).tstamp;
    sync_ptr.s.status.suspended_state = (*status).suspended_state;
    sync_ptr.s.status.audio_tstamp = (*status).audio_tstamp;
    pcm_stream_unlock_irq(substream);
    if sync_ptr.flags & SNDRV_PCM_SYNC_PTR_APPL == 0 {
        snd_pcm_dma_buffer_sync(substream, SNDRV_DMA_SYNC_DEVICE);
    }
    if copy_to_user(
        _sync_ptr as *mut c_void,
        &sync_ptr as *const _ as *const c_void,
        size_of::<snd_pcm_sync_ptr>(),
    ) != 0
    {
        return -EFAULT;
    }
    0
}

/*
 */
const SNDRV_PCM_IOCTL_HW_REFINE32: u32 = iowr(b'A', 0x10, size_of::<snd_pcm_hw_params32>());
const SNDRV_PCM_IOCTL_HW_PARAMS32: u32 = iowr(b'A', 0x11, size_of::<snd_pcm_hw_params32>());
const SNDRV_PCM_IOCTL_SW_PARAMS32: u32 = iowr(b'A', 0x13, size_of::<snd_pcm_sw_params32>());
const SNDRV_PCM_IOCTL_STATUS_COMPAT32: u32 = ior(b'A', 0x20, 0);
const SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT32: u32 = iowr(b'A', 0x24, 0);
const SNDRV_PCM_IOCTL_DELAY32: u32 = ior(b'A', 0x21, size_of::<s32>());
const SNDRV_PCM_IOCTL_CHANNEL_INFO32: u32 = ior(b'A', 0x32, size_of::<snd_pcm_channel_info32>());
const SNDRV_PCM_IOCTL_REWIND32: u32 = iow(b'A', 0x46, size_of::<u32>());
const SNDRV_PCM_IOCTL_FORWARD32: u32 = iow(b'A', 0x49, size_of::<u32>());
const SNDRV_PCM_IOCTL_WRITEI_FRAMES32: u32 = iow(b'A', 0x50, size_of::<snd_xferi32>());
const SNDRV_PCM_IOCTL_READI_FRAMES32: u32 = ior(b'A', 0x51, size_of::<snd_xferi32>());
const SNDRV_PCM_IOCTL_WRITEN_FRAMES32: u32 = iow(b'A', 0x52, size_of::<snd_xfern32>());
const SNDRV_PCM_IOCTL_READN_FRAMES32: u32 = ior(b'A', 0x53, size_of::<snd_xfern32>());
const SNDRV_PCM_IOCTL_STATUS_COMPAT64: u32 = ior(b'A', 0x20, size_of::<compat_snd_pcm_status64>());
const SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT64: u32 =
    iowr(b'A', 0x24, size_of::<compat_snd_pcm_status64>());
#[cfg(CONFIG_X86_X32_ABI)]
const SNDRV_PCM_IOCTL_CHANNEL_INFO_X32: u32 = ior(b'A', 0x32, size_of::<snd_pcm_channel_info>());
#[cfg(CONFIG_X86_X32_ABI)]
const SNDRV_PCM_IOCTL_SYNC_PTR_X32: u32 = iowr(b'A', 0x23, size_of::<snd_pcm_sync_ptr_x32>());

const fn ior(_type: u8, nr: u8, size: usize) -> u32 {
    ioctl_encode(2, _type, nr, size)
}

const fn iow(_type: u8, nr: u8, size: usize) -> u32 {
    ioctl_encode(1, _type, nr, size)
}

const fn iowr(_type: u8, nr: u8, size: usize) -> u32 {
    ioctl_encode(3, _type, nr, size)
}

const fn ioctl_encode(dir: u32, _type: u8, nr: u8, size: usize) -> u32 {
    (dir << 30) | ((size as u32) << 16) | ((_type as u32) << 8) | nr as u32
}

unsafe fn snd_pcm_ioctl_compat(file: *mut file, cmd: u32, arg: usize) -> isize {
    let pcm_file: *mut snd_pcm_file;
    let substream: *mut snd_pcm_substream;
    let argp: *mut c_void = compat_ptr(arg as compat_caddr_t);

    pcm_file = (*file).private_data;
    if pcm_file.is_null() {
        return -ENOTTY as isize;
    }
    substream = (*pcm_file).substream;
    if substream.is_null() {
        return -ENOTTY as isize;
    }

    /*
     * When PCM is used on 32bit mode, we need to disable
     * mmap of the old PCM status/control records because
     * of the size incompatibility.
     */
    (*pcm_file).no_compat_mmap = 1;

    if cmd == SNDRV_PCM_IOCTL_PVERSION
        || cmd == SNDRV_PCM_IOCTL_INFO
        || cmd == SNDRV_PCM_IOCTL_TSTAMP
        || cmd == SNDRV_PCM_IOCTL_TTSTAMP
        || cmd == SNDRV_PCM_IOCTL_USER_PVERSION
        || cmd == SNDRV_PCM_IOCTL_HWSYNC
        || cmd == SNDRV_PCM_IOCTL_PREPARE
        || cmd == SNDRV_PCM_IOCTL_RESET
        || cmd == SNDRV_PCM_IOCTL_START
        || cmd == SNDRV_PCM_IOCTL_DROP
        || cmd == SNDRV_PCM_IOCTL_DRAIN
        || cmd == SNDRV_PCM_IOCTL_PAUSE
        || cmd == SNDRV_PCM_IOCTL_HW_FREE
        || cmd == SNDRV_PCM_IOCTL_RESUME
        || cmd == SNDRV_PCM_IOCTL_XRUN
        || cmd == SNDRV_PCM_IOCTL_LINK
        || cmd == SNDRV_PCM_IOCTL_UNLINK
        || cmd == __SNDRV_PCM_IOCTL_SYNC_PTR32
    {
        return snd_pcm_common_ioctl(file, substream, cmd, argp);
    }
    if cmd == __SNDRV_PCM_IOCTL_SYNC_PTR64 {
        #[cfg(CONFIG_X86_X32_ABI)]
        {
            if in_x32_syscall() {
                return snd_pcm_ioctl_sync_ptr_x32(substream, argp as *mut snd_pcm_sync_ptr_x32)
                    as isize;
            }
        }
        return snd_pcm_ioctl_sync_ptr_buggy(substream, argp as *mut snd_pcm_sync_ptr) as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_HW_REFINE32 {
        return snd_pcm_ioctl_hw_params_compat(substream, 1, argp as *mut snd_pcm_hw_params32)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_HW_PARAMS32 {
        return snd_pcm_ioctl_hw_params_compat(substream, 0, argp as *mut snd_pcm_hw_params32)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_SW_PARAMS32 {
        return snd_pcm_ioctl_sw_params_compat(substream, argp as *mut snd_pcm_sw_params32) as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_STATUS_COMPAT32 {
        return snd_pcm_status_user32(substream, argp, false);
    }
    if cmd == SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT32 {
        return snd_pcm_status_user32(substream, argp, true);
    }
    if cmd == SNDRV_PCM_IOCTL_CHANNEL_INFO32 {
        return snd_pcm_ioctl_channel_info_compat(substream, argp as *mut snd_pcm_channel_info32)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_WRITEI_FRAMES32 {
        return snd_pcm_ioctl_xferi_compat(substream, SNDRV_PCM_STREAM_PLAYBACK, argp as *mut snd_xferi32)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_READI_FRAMES32 {
        return snd_pcm_ioctl_xferi_compat(substream, SNDRV_PCM_STREAM_CAPTURE, argp as *mut snd_xferi32)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_WRITEN_FRAMES32 {
        return snd_pcm_ioctl_xfern_compat(substream, SNDRV_PCM_STREAM_PLAYBACK, argp as *mut snd_xfern32)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_READN_FRAMES32 {
        return snd_pcm_ioctl_xfern_compat(substream, SNDRV_PCM_STREAM_CAPTURE, argp as *mut snd_xfern32)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_DELAY32 {
        return snd_pcm_ioctl_delay_compat(substream, argp as *mut s32) as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_REWIND32 {
        return snd_pcm_ioctl_rewind_compat(substream, argp as *mut u32) as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_FORWARD32 {
        return snd_pcm_ioctl_forward_compat(substream, argp as *mut u32) as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_STATUS_COMPAT64 {
        return snd_pcm_status_user_compat64(substream, argp as *mut compat_snd_pcm_status64, false)
            as isize;
    }
    if cmd == SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT64 {
        return snd_pcm_status_user_compat64(substream, argp as *mut compat_snd_pcm_status64, true)
            as isize;
    }
    #[cfg(CONFIG_X86_X32_ABI)]
    {
        if cmd == SNDRV_PCM_IOCTL_CHANNEL_INFO_X32 {
            return snd_pcm_channel_info_user(substream, argp as *mut snd_pcm_channel_info) as isize;
        }
    }

    -ENOIOCTLCMD as isize
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
