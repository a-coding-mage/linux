// SPDX-License-Identifier: GPL-2.0-only
/*
 *  compress_core.c - compress offload core
 *
 *  Copyright (C) 2011 Intel Corporation
 *  Authors:	Vinod Koul <vinod.koul@linux.intel.com>
 *		Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* C includes removed:
 * linux/file.h, linux/fs.h, linux/list.h, linux/math64.h, linux/mm.h,
 * linux/mutex.h, linux/poll.h, linux/slab.h, linux/sched.h, linux/types.h,
 * linux/uio.h, linux/uaccess.h, linux/dma-buf.h, linux/module.h,
 * linux/compat.h, sound/core.h, sound/initval.h, sound/info.h,
 * sound/compress_params.h, sound/compress_offload.h,
 * sound/compress_driver.h.
 */

type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type u32 = u32;
type u64 = u64;
type __u64 = u64;
type __poll_t = u32;
type bool_t = bool;
type snd_pcm_state_t = c_int;

const O_WRONLY: c_int = 1;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_ACCMODE: c_int = 3;
const O_CLOEXEC: c_int = 0o2000000;
const GFP_KERNEL: c_uint = 0;
const U32_MAX: u32 = u32::MAX;

const EINVAL: c_int = 22;
const EBADFD: c_int = 77;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ENOTSUPP: c_int = 524;
const EPIPE: c_int = 32;
const EFAULT: c_int = 14;
const ENXIO: c_int = 6;
const EPERM: c_int = 1;
const EBUSY: c_int = 16;
const ENOTTY: c_int = 25;
const ERESTARTSYS: c_int = 512;

const EPOLLIN: __poll_t = 0x0001;
const EPOLLOUT: __poll_t = 0x0004;
const EPOLLERR: __poll_t = 0x0008;
const EPOLLRDNORM: __poll_t = 0x0040;
const EPOLLWRNORM: __poll_t = 0x0100;

/* External constants from ALSA/kernel headers. */
extern "C" {
    static snd_major: c_int;
    static system_power_efficient_wq: *mut workqueue_struct;
    static THIS_MODULE: *mut module;

    static SNDRV_DEVICE_TYPE_COMPRESS: c_int;
    static SNDRV_DEV_COMPRESS: c_int;
    static SNDRV_COMPRESS_VERSION: c_int;
    static SND_AUDIOCODEC_MAX: u32;

    static SNDRV_PCM_STATE_OPEN: snd_pcm_state_t;
    static SNDRV_PCM_STATE_SETUP: snd_pcm_state_t;
    static SNDRV_PCM_STATE_PREPARED: snd_pcm_state_t;
    static SNDRV_PCM_STATE_RUNNING: snd_pcm_state_t;
    static SNDRV_PCM_STATE_DRAINING: snd_pcm_state_t;
    static SNDRV_PCM_STATE_PAUSED: snd_pcm_state_t;
    static SNDRV_PCM_STATE_XRUN: snd_pcm_state_t;
    static SNDRV_PCM_STATE_SUSPENDED: snd_pcm_state_t;
    static SNDRV_PCM_STATE_DISCONNECTED: snd_pcm_state_t;

    static SND_COMPRESS_PLAYBACK: snd_compr_direction;
    static SND_COMPRESS_CAPTURE: snd_compr_direction;
    static SND_COMPRESS_ACCEL: snd_compr_direction;

    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SND_COMPR_TRIGGER_DRAIN: c_int;
    static SND_COMPR_TRIGGER_NEXT_TRACK: c_int;
    static SND_COMPR_TRIGGER_PARTIAL_DRAIN: c_int;

    static SND_COMPRESS_TASK_STATE_IDLE: c_int;
    static SND_COMPRESS_TASK_STATE_ACTIVE: c_int;
    static SND_COMPRESS_TASK_STATE_FINISHED: c_int;

    static SNDRV_COMPRESS_IOCTL_VERSION: c_uint;
    static SNDRV_COMPRESS_GET_CAPS: c_uint;
    static SNDRV_COMPRESS_GET_CODEC_CAPS: c_uint;
    static SNDRV_COMPRESS_SET_PARAMS: c_uint;
    static SNDRV_COMPRESS_GET_PARAMS: c_uint;
    static SNDRV_COMPRESS_SET_METADATA: c_uint;
    static SNDRV_COMPRESS_GET_METADATA: c_uint;
    static SNDRV_COMPRESS_TASK_CREATE: c_uint;
    static SNDRV_COMPRESS_TASK_FREE: c_uint;
    static SNDRV_COMPRESS_TASK_START: c_uint;
    static SNDRV_COMPRESS_TASK_STOP: c_uint;
    static SNDRV_COMPRESS_TASK_STATUS: c_uint;
    static SNDRV_COMPRESS_TSTAMP: c_uint;
    static SNDRV_COMPRESS_TSTAMP64: c_uint;
    static SNDRV_COMPRESS_AVAIL: c_uint;
    static SNDRV_COMPRESS_AVAIL64: c_uint;
    static SNDRV_COMPRESS_PAUSE: c_uint;
    static SNDRV_COMPRESS_RESUME: c_uint;
    static SNDRV_COMPRESS_START: c_uint;
    static SNDRV_COMPRESS_STOP: c_uint;
    static SNDRV_COMPRESS_DRAIN: c_uint;
    static SNDRV_COMPRESS_PARTIAL_DRAIN: c_uint;
    static SNDRV_COMPRESS_NEXT_TRACK: c_uint;
}

#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct file { pub f_flags: c_int, pub private_data: *mut c_void }
#[repr(C)]
pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)]
pub struct poll_table { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct delayed_work { pub work: work_struct }
#[repr(C)]
pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)]
pub struct module { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct snd_card { pub number: c_int, pub proc_root: *mut snd_info_entry }
#[repr(C)]
pub struct snd_info_entry { pub private_data: *mut c_void, pub mode: c_uint }
#[repr(C)]
pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)]
pub struct snd_device { pub device_data: *mut c_void }
#[repr(C)]
pub struct dma_buf { pub file: *mut file, pub size: size_t }
#[repr(C)]
pub struct snd_dma_device { pub type_: c_int, pub dev: *mut device }
#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device,
    pub area: *mut c_void,
    pub bytes: size_t,
}

type snd_compr_direction = c_int;

#[repr(C)]
pub struct snd_compr_tstamp {
    pub byte_offset: u32,
    pub copied_total: u32,
    pub pcm_frames: u32,
    pub pcm_io_frames: u32,
    pub sampling_rate: u32,
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
    pub byte_offset: u32,
    pub copied_total: u64,
    pub pcm_frames: u64,
    pub pcm_io_frames: u64,
    pub sampling_rate: u32,
}

#[repr(C)]
pub struct snd_compr_avail {
    pub avail: size_t,
    pub tstamp: snd_compr_tstamp,
}

#[repr(C)]
pub struct snd_compr_avail64 {
    pub avail: size_t,
    pub tstamp: snd_compr_tstamp64,
}

#[repr(C)]
pub struct snd_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_caps { _private: [u8; 0] }
#[repr(C)]
pub struct snd_compr_codec_caps { _private: [u8; 0] }
#[repr(C)]
pub struct snd_compr_metadata { _private: [u8; 0] }

#[repr(C)]
pub struct snd_compr_buffer {
    pub fragment_size: u32,
    pub fragments: u32,
}

#[repr(C)]
pub struct snd_compr_codec {
    pub id: u32,
    pub ch_in: u32,
    pub ch_out: u32,
}

#[repr(C)]
pub struct snd_compr_params {
    pub buffer: snd_compr_buffer,
    pub codec: snd_compr_codec,
}

#[repr(C)]
pub struct snd_compr_task {
    pub seqno: __u64,
    pub origin_seqno: __u64,
    pub input_size: size_t,
    pub input_fd: c_int,
    pub output_fd: c_int,
    pub flags: c_uint,
}

#[repr(C)]
pub struct snd_compr_task_status {
    pub seqno: __u64,
    pub input_size: size_t,
    pub output_size: size_t,
    pub state: c_int,
}

#[repr(C)]
pub struct snd_compr_task_runtime {
    pub list: list_head,
    pub seqno: __u64,
    pub input_size: size_t,
    pub output_size: size_t,
    pub flags: c_uint,
    pub state: c_int,
    pub input: *mut dma_buf,
    pub output: *mut dma_buf,
}

#[repr(C)]
pub struct snd_compr_runtime {
    pub state: snd_pcm_state_t,
    pub sleep: wait_queue_head_t,
    pub tasks: list_head,
    pub buffer: *mut u8,
    pub buffer_size: size_t,
    pub fragment_size: u32,
    pub fragments: u32,
    pub dma_area: *mut c_void,
    pub dma_buffer_p: *mut snd_dma_buffer,
    pub dma_bytes: size_t,
    pub total_bytes_available: u64,
    pub total_bytes_transferred: u64,
    pub task_seqno: u64,
    pub total_tasks: u32,
    pub active_tasks: u32,
}

#[repr(C)]
pub struct snd_compr_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_compr_stream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_tstamp64) -> c_int>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_compr_stream, size_t) -> c_int>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut c_char, size_t) -> c_int>,
    pub get_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_caps) -> c_int>,
    pub get_codec_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_codec_caps) -> c_int>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_params) -> c_int>,
    pub get_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_codec) -> c_int>,
    pub get_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>,
    pub set_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>,
    pub task_create: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> c_int>,
    pub task_free: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime)>,
    pub task_start: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> c_int>,
    pub task_stop: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime)>,
}

#[repr(C)]
pub struct snd_compr {
    pub lock: mutex,
    pub ops: *mut snd_compr_ops,
    pub direction: snd_compr_direction,
    pub private_data: *mut c_void,
    pub card: *mut snd_card,
    pub device: c_int,
    pub dev: *mut device,
    pub name: *const c_char,
    pub id: [c_char; 64],
    pub proc_root: *mut snd_info_entry,
    pub proc_info_entry: *mut snd_info_entry,
    pub use_pause_in_draining: bool,
}

#[repr(C)]
pub struct snd_compr_stream {
    pub ops: *mut snd_compr_ops,
    pub direction: snd_compr_direction,
    pub private_data: *mut c_void,
    pub device: *mut snd_compr,
    pub runtime: *mut snd_compr_runtime,
    pub dma_buffer: snd_dma_buffer,
    pub error_work: delayed_work,
    pub metadata_set: bool,
    pub next_track: bool,
    pub partial_drain: bool,
    pub pause_in_draining: bool,
}

#[repr(C)]
pub struct snd_compr_file {
    pub caps: c_ulong,
    pub stream: snd_compr_stream,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

extern "C" {
    fn imajor(inode: *mut inode) -> c_int;
    fn iminor(inode: *mut inode) -> c_int;
    fn snd_lookup_minor_data(minor: c_int, ty: c_int) -> *mut snd_compr;
    fn snd_card_unref(card: *mut snd_card);
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, val: c_int, size: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: size_t) -> *mut c_void;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, size: size_t) -> c_int;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, size: size_t) -> c_int;
    fn memdup_user(src: *const c_void, size: size_t) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn put_user(value: c_int, ptr: *mut c_int) -> c_int;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, f: unsafe extern "C" fn(*mut work_struct));
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_move_tail(list: *mut list_head, head: *mut list_head);
    fn list_first_entry_or_null(head: *mut list_head) -> *mut snd_compr_task_runtime;
    fn div64_u64(dividend: u64, divisor: u64) -> u64;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn poll_wait(file: *mut file, wait: *mut wait_queue_head_t, p: *mut poll_table);
    fn wake_up(wait: *mut wait_queue_head_t);
    fn wait_event_interruptible(wait: wait_queue_head_t, condition: bool) -> c_int;
    fn queue_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_ulong) -> bool;
    fn snd_dma_alloc_pages(ty: c_int, dev: *mut device, size: size_t, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn snd_compr_set_runtime_buffer(stream: *mut snd_compr_stream, dmab: *mut snd_dma_buffer);
    fn dma_buf_put(buf: *mut dma_buf);
    fn get_dma_buf(buf: *mut dma_buf);
    fn get_unused_fd_flags(flags: c_int) -> c_int;
    fn put_unused_fd(fd: c_int);
    fn fd_install(fd: c_int, file: *mut file);
    fn snd_register_device(ty: c_int, card: *mut snd_card, dev: c_int, ops: *const file_operations, data: *mut snd_compr, device: *mut device) -> c_int;
    fn snd_unregister_device(dev: *mut device);
    fn snd_device_alloc(dev: *mut *mut device, card: *mut snd_card) -> c_int;
    fn snd_device_new(card: *mut snd_card, ty: c_int, data: *mut snd_compr, ops: *const snd_device_ops) -> c_int;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn put_device(dev: *mut device);
    fn snd_info_create_card_entry(card: *mut snd_card, name: *const c_char, parent: *mut snd_info_entry) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(entry: *mut snd_info_entry, data: *mut snd_compr, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> isize;
    fn compat_ptr(arg: c_ulong) -> *mut c_void;
}

unsafe extern "C" fn error_delayed_work(work: *mut work_struct);

#[inline]
unsafe fn op_open(stream: *mut snd_compr_stream) -> c_int {
    ((*(*stream).ops).open.unwrap())(stream)
}
#[inline]
unsafe fn op_free(stream: *mut snd_compr_stream) -> c_int {
    ((*(*stream).ops).free.unwrap())(stream)
}
#[inline]
unsafe fn op_trigger(stream: *mut snd_compr_stream, cmd: c_int) -> c_int {
    ((*(*stream).ops).trigger.unwrap())(stream, cmd)
}

unsafe extern "C" fn snd_compr_task_free_all(_stream: *mut snd_compr_stream) {
    /* Empty inline fallback when CONFIG_SND_COMPRESS_ACCEL is disabled. */
}

unsafe extern "C" fn snd_compr_open(inode: *mut inode, f: *mut file) -> c_int {
    let compr: *mut snd_compr;
    let data: *mut snd_compr_file;
    let runtime: *mut snd_compr_runtime;
    let dirn: snd_compr_direction;
    let maj = imajor(inode);
    let ret: c_int;

    if ((*f).f_flags & O_ACCMODE) == O_WRONLY {
        dirn = SND_COMPRESS_PLAYBACK;
    } else if ((*f).f_flags & O_ACCMODE) == O_RDONLY {
        dirn = SND_COMPRESS_CAPTURE;
    } else if ((*f).f_flags & O_ACCMODE) == O_RDWR {
        dirn = SND_COMPRESS_ACCEL;
    } else {
        return -EINVAL;
    }

    if maj == snd_major {
        compr = snd_lookup_minor_data(iminor(inode), SNDRV_DEVICE_TYPE_COMPRESS);
    } else {
        return -EBADFD;
    }

    if compr.is_null() {
        return -ENODEV;
    }

    if dirn != (*compr).direction {
        snd_card_unref((*compr).card);
        return -EINVAL;
    }

    data = kzalloc(size_of::<snd_compr_file>(), GFP_KERNEL) as *mut snd_compr_file;
    if data.is_null() {
        snd_card_unref((*compr).card);
        return -ENOMEM;
    }

    INIT_DELAYED_WORK(&mut (*data).stream.error_work, error_delayed_work);
    (*data).stream.ops = (*compr).ops;
    (*data).stream.direction = dirn;
    (*data).stream.private_data = (*compr).private_data;
    (*data).stream.device = compr;

    runtime = kzalloc(size_of::<snd_compr_runtime>(), GFP_KERNEL) as *mut snd_compr_runtime;
    if runtime.is_null() {
        kfree(data as *mut c_void);
        snd_card_unref((*compr).card);
        return -ENOMEM;
    }
    (*runtime).state = SNDRV_PCM_STATE_OPEN;
    init_waitqueue_head(&mut (*runtime).sleep);
    INIT_LIST_HEAD(&mut (*runtime).tasks);
    (*data).stream.runtime = runtime;
    (*f).private_data = data as *mut c_void;

    mutex_lock(&mut (*compr).lock);
    ret = op_open(&mut (*data).stream);
    mutex_unlock(&mut (*compr).lock);
    if ret != 0 {
        kfree(runtime as *mut c_void);
        kfree(data as *mut c_void);
    }
    snd_card_unref((*compr).card);
    ret
}

unsafe extern "C" fn snd_compr_free(_inode: *mut inode, f: *mut file) -> c_int {
    let data = (*f).private_data as *mut snd_compr_file;
    let runtime = (*data).stream.runtime;

    cancel_delayed_work_sync(&mut (*data).stream.error_work);

    match (*runtime).state {
        s if s == SNDRV_PCM_STATE_RUNNING || s == SNDRV_PCM_STATE_DRAINING || s == SNDRV_PCM_STATE_PAUSED => {
            op_trigger(&mut (*data).stream, SNDRV_PCM_TRIGGER_STOP);
        }
        _ => {}
    }

    snd_compr_task_free_all(&mut (*data).stream);
    op_free(&mut (*data).stream);
    if (*(*data).stream.runtime).dma_buffer_p.is_null() {
        kfree((*(*data).stream.runtime).buffer as *mut c_void);
    }
    kfree((*data).stream.runtime as *mut c_void);
    kfree(data as *mut c_void);
    0
}

unsafe fn snd_compr_tstamp32_from_64(tstamp32: *mut snd_compr_tstamp, tstamp64: *const snd_compr_tstamp64) {
    (*tstamp32).byte_offset = (*tstamp64).byte_offset;
    (*tstamp32).copied_total = (*tstamp64).copied_total as u32;
    (*tstamp32).pcm_frames = (*tstamp64).pcm_frames as u32;
    (*tstamp32).pcm_io_frames = (*tstamp64).pcm_io_frames as u32;
    (*tstamp32).sampling_rate = (*tstamp64).sampling_rate;
}

unsafe fn snd_compr_update_tstamp(stream: *mut snd_compr_stream, tstamp: *mut snd_compr_tstamp64) -> c_int {
    let ret: c_int;

    if (*(*stream).ops).pointer.is_none() {
        return -ENOTSUPP;
    }

    if (*(*stream).runtime).state == SNDRV_PCM_STATE_OPEN {
        return -EBADFD;
    }

    ret = ((*(*stream).ops).pointer.unwrap())(stream, tstamp);
    if ret != 0 {
        return ret;
    }
    if (*stream).direction == SND_COMPRESS_PLAYBACK {
        (*(*stream).runtime).total_bytes_transferred = (*tstamp).copied_total;
    } else {
        (*(*stream).runtime).total_bytes_available = (*tstamp).copied_total;
    }
    0
}

unsafe fn snd_compr_calc_avail(stream: *mut snd_compr_stream, avail: *mut snd_compr_avail64) -> size_t {
    memset(avail as *mut c_void, 0, size_of::<snd_compr_avail64>());
    snd_compr_update_tstamp(stream, &mut (*avail).tstamp);

    if (*(*stream).runtime).total_bytes_available == 0
        && (*(*stream).runtime).state == SNDRV_PCM_STATE_SETUP
        && (*stream).direction == SND_COMPRESS_PLAYBACK
    {
        return (*(*stream).runtime).buffer_size;
    }

    if (*(*stream).runtime).total_bytes_available == (*(*stream).runtime).total_bytes_transferred {
        if (*stream).direction == SND_COMPRESS_PLAYBACK {
            return (*(*stream).runtime).buffer_size;
        } else {
            return 0;
        }
    }

    (*avail).avail = ((*(*stream).runtime).total_bytes_available - (*(*stream).runtime).total_bytes_transferred) as size_t;
    if (*stream).direction == SND_COMPRESS_PLAYBACK {
        (*avail).avail = (*(*stream).runtime).buffer_size - (*avail).avail;
    }
    (*avail).avail
}

#[inline]
unsafe fn snd_compr_get_avail(stream: *mut snd_compr_stream) -> size_t {
    let mut avail: snd_compr_avail64 = core::mem::zeroed();
    snd_compr_calc_avail(stream, &mut avail)
}

unsafe fn snd_compr_avail32_from_64(avail32: *mut snd_compr_avail, avail64: *const snd_compr_avail64) {
    (*avail32).avail = (*avail64).avail;
    snd_compr_tstamp32_from_64(&mut (*avail32).tstamp, &(*avail64).tstamp);
}

unsafe fn snd_compr_ioctl_avail(stream: *mut snd_compr_stream, arg: c_ulong, is_32bit: bool) -> c_int {
    let mut ioctl_avail64: snd_compr_avail64 = core::mem::zeroed();
    let mut ioctl_avail32: snd_compr_avail = core::mem::zeroed();
    let mut copy_from: *const c_void = &ioctl_avail64 as *const _ as *const c_void;
    let mut copy_size = size_of::<snd_compr_avail64>();

    if (*stream).direction == SND_COMPRESS_ACCEL {
        return -EBADFD;
    }

    let avail = snd_compr_calc_avail(stream, &mut ioctl_avail64);
    ioctl_avail64.avail = avail;
    if is_32bit {
        snd_compr_avail32_from_64(&mut ioctl_avail32, &ioctl_avail64);
        copy_from = &ioctl_avail32 as *const _ as *const c_void;
        copy_size = size_of::<snd_compr_avail>();
    }

    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_OPEN => return -EBADFD,
        s if s == SNDRV_PCM_STATE_XRUN => return -EPIPE,
        _ => {}
    }

    if copy_to_user(arg as *mut c_void, copy_from, copy_size) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_compr_write_data(stream: *mut snd_compr_stream, buf: *const c_char, count: size_t) -> c_int {
    let runtime = (*stream).runtime;
    let app_pointer_div = div64_u64((*runtime).total_bytes_available, (*runtime).buffer_size as u64);
    let app_pointer = (*runtime).total_bytes_available - app_pointer_div * (*runtime).buffer_size as u64;
    let dstn = (*runtime).buffer.add(app_pointer as usize) as *mut c_void;

    if count < (*runtime).buffer_size - app_pointer as usize {
        if copy_from_user(dstn, buf as *const c_void, count) != 0 {
            return -EFAULT;
        }
    } else {
        let copy = (*runtime).buffer_size - app_pointer as usize;
        if copy_from_user(dstn, buf as *const c_void, copy) != 0 {
            return -EFAULT;
        }
        if copy_from_user((*runtime).buffer as *mut c_void, buf.add(copy) as *const c_void, count - copy) != 0 {
            return -EFAULT;
        }
    }
    if let Some(ack) = (*(*stream).ops).ack {
        ack(stream, count);
    }
    count as c_int
}

unsafe extern "C" fn snd_compr_write(f: *mut file, buf: *const c_char, count: size_t, _offset: *mut loff_t) -> ssize_t {
    let data = (*f).private_data as *mut snd_compr_file;
    let stream: *mut snd_compr_stream;
    let mut avail: size_t;
    let retval: c_int;

    if snd_BUG_ON(data.is_null()) {
        return -EFAULT as ssize_t;
    }
    stream = &mut (*data).stream;
    if (*stream).direction == SND_COMPRESS_ACCEL {
        return -EBADFD as ssize_t;
    }

    mutex_lock(&mut (*(*stream).device).lock);
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_SETUP || s == SNDRV_PCM_STATE_PREPARED || s == SNDRV_PCM_STATE_RUNNING => {}
        _ => {
            mutex_unlock(&mut (*(*stream).device).lock);
            return -EBADFD as ssize_t;
        }
    }

    avail = snd_compr_get_avail(stream);
    if avail > count {
        avail = count;
    }

    if let Some(copy) = (*(*stream).ops).copy {
        retval = copy(stream, buf as *mut c_char, avail);
    } else {
        retval = snd_compr_write_data(stream, buf, avail);
    }
    if retval > 0 {
        (*(*stream).runtime).total_bytes_available += retval as u64;
    }
    if (*(*stream).runtime).state == SNDRV_PCM_STATE_SETUP {
        (*(*stream).runtime).state = SNDRV_PCM_STATE_PREPARED;
    }
    mutex_unlock(&mut (*(*stream).device).lock);
    retval as ssize_t
}

unsafe extern "C" fn snd_compr_read(f: *mut file, buf: *mut c_char, count: size_t, _offset: *mut loff_t) -> ssize_t {
    let data = (*f).private_data as *mut snd_compr_file;
    let stream: *mut snd_compr_stream;
    let mut avail: size_t;
    let retval: c_int;

    if snd_BUG_ON(data.is_null()) {
        return -EFAULT as ssize_t;
    }
    stream = &mut (*data).stream;
    if (*stream).direction == SND_COMPRESS_ACCEL {
        return -EBADFD as ssize_t;
    }

    mutex_lock(&mut (*(*stream).device).lock);
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_OPEN || s == SNDRV_PCM_STATE_PREPARED || s == SNDRV_PCM_STATE_SUSPENDED || s == SNDRV_PCM_STATE_DISCONNECTED => {
            mutex_unlock(&mut (*(*stream).device).lock);
            return -EBADFD as ssize_t;
        }
        s if s == SNDRV_PCM_STATE_XRUN => {
            mutex_unlock(&mut (*(*stream).device).lock);
            return -EPIPE as ssize_t;
        }
        _ => {}
    }

    avail = snd_compr_get_avail(stream);
    if avail > count {
        avail = count;
    }

    if let Some(copy) = (*(*stream).ops).copy {
        retval = copy(stream, buf, avail);
    } else {
        mutex_unlock(&mut (*(*stream).device).lock);
        return -ENXIO as ssize_t;
    }
    if retval > 0 {
        (*(*stream).runtime).total_bytes_transferred += retval as u64;
    }
    mutex_unlock(&mut (*(*stream).device).lock);
    retval as ssize_t
}

unsafe extern "C" fn snd_compr_mmap(_f: *mut file, _vma: *mut vm_area_struct) -> c_int {
    -ENXIO
}

unsafe fn snd_compr_get_poll(stream: *mut snd_compr_stream) -> __poll_t {
    if (*stream).direction == SND_COMPRESS_PLAYBACK {
        EPOLLOUT | EPOLLWRNORM
    } else {
        EPOLLIN | EPOLLRDNORM
    }
}

unsafe extern "C" fn snd_compr_poll(f: *mut file, wait: *mut poll_table) -> __poll_t {
    let data = (*f).private_data as *mut snd_compr_file;
    let stream: *mut snd_compr_stream;
    let runtime: *mut snd_compr_runtime;
    let mut retval: __poll_t = 0;

    if snd_BUG_ON(data.is_null()) {
        return EPOLLERR;
    }
    stream = &mut (*data).stream;
    runtime = (*stream).runtime;
    mutex_lock(&mut (*(*stream).device).lock);

    match (*runtime).state {
        s if s == SNDRV_PCM_STATE_OPEN || s == SNDRV_PCM_STATE_XRUN => {
            retval = snd_compr_get_poll(stream) | EPOLLERR;
            mutex_unlock(&mut (*(*stream).device).lock);
            return retval;
        }
        _ => {}
    }

    poll_wait(f, &mut (*runtime).sleep, wait);

    if (*stream).direction == SND_COMPRESS_ACCEL {
        if (*runtime).fragments > (*runtime).active_tasks {
            retval |= EPOLLOUT | EPOLLWRNORM;
        }
        let task = list_first_entry_or_null(&mut (*runtime).tasks);
        if !task.is_null() && (*task).state == SND_COMPRESS_TASK_STATE_FINISHED {
            retval |= EPOLLIN | EPOLLRDNORM;
        }
        mutex_unlock(&mut (*(*stream).device).lock);
        return retval;
    }

    let avail = snd_compr_get_avail(stream);
    match (*runtime).state {
        s if s == SNDRV_PCM_STATE_DRAINING => {
            retval = snd_compr_get_poll(stream);
            (*runtime).state = SNDRV_PCM_STATE_SETUP;
        }
        s if s == SNDRV_PCM_STATE_RUNNING || s == SNDRV_PCM_STATE_PREPARED || s == SNDRV_PCM_STATE_PAUSED => {
            if avail >= (*runtime).fragment_size as usize {
                retval = snd_compr_get_poll(stream);
            }
        }
        _ => {
            retval = snd_compr_get_poll(stream) | EPOLLERR;
        }
    }
    mutex_unlock(&mut (*(*stream).device).lock);
    retval
}

unsafe fn snd_compr_get_caps(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    let mut retval: c_int;
    let mut caps: snd_compr_caps = core::mem::zeroed();
    if (*(*stream).ops).get_caps.is_none() {
        return -ENXIO;
    }
    retval = ((*(*stream).ops).get_caps.unwrap())(stream, &mut caps);
    if retval != 0 {
        return retval;
    }
    if copy_to_user(arg as *mut c_void, &caps as *const _ as *const c_void, size_of::<snd_compr_caps>()) != 0 {
        retval = -EFAULT;
    }
    retval
}

/* struct snd_compr_codec_caps overflows the ioctl bit size for some
 * architectures, so the C source conditionally disables this ioctl when
 * _IOC_SIZEBITS < 14.
 */
unsafe fn snd_compr_get_codec_caps(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    if (*(*stream).ops).get_codec_caps.is_none() {
        return -ENXIO;
    }
    let caps = kzalloc(size_of::<snd_compr_codec_caps>(), GFP_KERNEL) as *mut snd_compr_codec_caps;
    if caps.is_null() {
        return -ENOMEM;
    }
    let retval = ((*(*stream).ops).get_codec_caps.unwrap())(stream, caps);
    if retval != 0 {
        kfree(caps as *mut c_void);
        return retval;
    }
    let ret = if copy_to_user(arg as *mut c_void, caps as *const c_void, size_of::<snd_compr_codec_caps>()) != 0 {
        -EFAULT
    } else {
        retval
    };
    kfree(caps as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_compr_malloc_pages(stream: *mut snd_compr_stream, size: size_t) -> c_int {
    if snd_BUG_ON(stream.is_null() || (*stream).runtime.is_null()) {
        return -EINVAL;
    }
    let dmab = kzalloc(size_of::<snd_dma_buffer>(), GFP_KERNEL) as *mut snd_dma_buffer;
    if dmab.is_null() {
        return -ENOMEM;
    }
    (*dmab).dev = (*stream).dma_buffer.dev;
    let ret = snd_dma_alloc_pages((*dmab).dev.type_, (*dmab).dev.dev, size, dmab);
    if ret < 0 {
        kfree(dmab as *mut c_void);
        return ret;
    }
    snd_compr_set_runtime_buffer(stream, dmab);
    (*(*stream).runtime).dma_bytes = size;
    1
}

#[no_mangle]
pub unsafe extern "C" fn snd_compr_free_pages(stream: *mut snd_compr_stream) -> c_int {
    if snd_BUG_ON(stream.is_null() || (*stream).runtime.is_null()) {
        return -EINVAL;
    }
    let runtime = (*stream).runtime;
    if (*runtime).dma_area.is_null() {
        return 0;
    }
    if (*runtime).dma_buffer_p != &mut (*stream).dma_buffer {
        snd_dma_free_pages((*runtime).dma_buffer_p);
        kfree((*runtime).dma_buffer_p as *mut c_void);
    }
    snd_compr_set_runtime_buffer(stream, ptr::null_mut());
    0
}

unsafe fn snd_compr_allocate_buffer(stream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int {
    let mut buffer: *mut c_void = ptr::null_mut();

    if (*stream).direction != SND_COMPRESS_ACCEL {
        let buffer_size = ((*params).buffer.fragment_size as usize).wrapping_mul((*params).buffer.fragments as usize);
        if (*(*stream).ops).copy.is_some() {
            buffer = ptr::null_mut();
        } else if !(*(*stream).runtime).dma_buffer_p.is_null() {
            if buffer_size <= (*(*(*stream).runtime).dma_buffer_p).bytes {
                buffer = (*(*(*stream).runtime).dma_buffer_p).area;
            }
        } else {
            buffer = kmalloc(buffer_size, GFP_KERNEL);
        }
        if buffer.is_null() && (*(*stream).ops).copy.is_none() {
            return -ENOMEM;
        }
        (*(*stream).runtime).buffer = buffer as *mut u8;
        (*(*stream).runtime).buffer_size = buffer_size;
    }
    (*(*stream).runtime).fragment_size = (*params).buffer.fragment_size;
    (*(*stream).runtime).fragments = (*params).buffer.fragments;
    0
}

unsafe fn snd_compress_check_input(stream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int {
    let max_fragments: u32;
    if (*params).buffer.fragment_size == 0 {
        return -EINVAL;
    }
    if (*stream).direction == SND_COMPRESS_ACCEL {
        max_fragments = 64;
    } else {
        max_fragments = U32_MAX / (*params).buffer.fragment_size;
    }
    if (*params).buffer.fragments > max_fragments || (*params).buffer.fragments == 0 {
        return -EINVAL;
    }
    if (*params).codec.id == 0 || (*params).codec.id > SND_AUDIOCODEC_MAX {
        return -EINVAL;
    }
    if (*params).codec.ch_in == 0 || (*params).codec.ch_out == 0 {
        return -EINVAL;
    }
    0
}

unsafe fn snd_compr_set_params(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    let mut retval: c_int;
    if (*(*stream).runtime).state == SNDRV_PCM_STATE_OPEN || (*stream).next_track {
        let params = memdup_user(arg as *const c_void, size_of::<snd_compr_params>()) as *mut snd_compr_params;
        if IS_ERR(params as *const c_void) {
            return PTR_ERR(params as *const c_void);
        }
        retval = snd_compress_check_input(stream, params);
        if retval != 0 {
            kfree(params as *mut c_void);
            return retval;
        }
        retval = snd_compr_allocate_buffer(stream, params);
        if retval != 0 {
            kfree(params as *mut c_void);
            return -ENOMEM;
        }
        retval = ((*(*stream).ops).set_params.unwrap())(stream, params);
        kfree(params as *mut c_void);
        if retval != 0 {
            return retval;
        }
        if (*stream).next_track {
            return retval;
        }
        (*stream).metadata_set = false;
        (*stream).next_track = false;
        (*(*stream).runtime).state = SNDRV_PCM_STATE_SETUP;
    } else {
        return -EPERM;
    }
    retval
}

unsafe fn snd_compr_get_params(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    if (*(*stream).ops).get_params.is_none() {
        return -EBADFD;
    }
    let params = kzalloc(size_of::<snd_codec>(), GFP_KERNEL) as *mut snd_codec;
    if params.is_null() {
        return -ENOMEM;
    }
    let retval = ((*(*stream).ops).get_params.unwrap())(stream, params);
    if retval != 0 {
        kfree(params as *mut c_void);
        return retval;
    }
    let ret = if copy_to_user(arg as *mut c_void, params as *const c_void, size_of::<snd_codec>()) != 0 {
        -EFAULT
    } else {
        retval
    };
    kfree(params as *mut c_void);
    ret
}

unsafe fn snd_compr_get_metadata(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    let mut metadata: snd_compr_metadata = core::mem::zeroed();
    if (*(*stream).ops).get_metadata.is_none() {
        return -ENXIO;
    }
    if copy_from_user(&mut metadata as *mut _ as *mut c_void, arg as *const c_void, size_of::<snd_compr_metadata>()) != 0 {
        return -EFAULT;
    }
    let retval = ((*(*stream).ops).get_metadata.unwrap())(stream, &mut metadata);
    if retval != 0 {
        return retval;
    }
    if copy_to_user(arg as *mut c_void, &metadata as *const _ as *const c_void, size_of::<snd_compr_metadata>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_compr_set_metadata(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    let mut metadata: snd_compr_metadata = core::mem::zeroed();
    if (*(*stream).ops).set_metadata.is_none() {
        return -ENXIO;
    }
    if copy_from_user(&mut metadata as *mut _ as *mut c_void, arg as *const c_void, size_of::<snd_compr_metadata>()) != 0 {
        return -EFAULT;
    }
    let retval = ((*(*stream).ops).set_metadata.unwrap())(stream, &mut metadata);
    (*stream).metadata_set = true;
    retval
}

unsafe fn snd_compr_tstamp(stream: *mut snd_compr_stream, arg: c_ulong, is_32bit: bool) -> c_int {
    let mut tstamp64: snd_compr_tstamp64 = core::mem::zeroed();
    let mut tstamp32: snd_compr_tstamp = core::mem::zeroed();
    let mut copy_from: *const c_void = &tstamp64 as *const _ as *const c_void;
    let mut copy_size = size_of::<snd_compr_tstamp64>();
    let mut ret = snd_compr_update_tstamp(stream, &mut tstamp64);
    if ret == 0 {
        if is_32bit {
            snd_compr_tstamp32_from_64(&mut tstamp32, &tstamp64);
            copy_from = &tstamp32 as *const _ as *const c_void;
            copy_size = size_of::<snd_compr_tstamp>();
        }
        ret = if copy_to_user(arg as *mut c_void, copy_from, copy_size) != 0 { -EFAULT } else { 0 };
    }
    ret
}

unsafe fn snd_compr_pause(stream: *mut snd_compr_stream) -> c_int {
    let retval: c_int;
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_RUNNING => {
            retval = op_trigger(stream, SNDRV_PCM_TRIGGER_PAUSE_PUSH);
            if retval == 0 {
                (*(*stream).runtime).state = SNDRV_PCM_STATE_PAUSED;
            }
        }
        s if s == SNDRV_PCM_STATE_DRAINING => {
            if !(*(*stream).device).use_pause_in_draining {
                return -EPERM;
            }
            retval = op_trigger(stream, SNDRV_PCM_TRIGGER_PAUSE_PUSH);
            if retval == 0 {
                (*stream).pause_in_draining = true;
            }
        }
        _ => return -EPERM,
    }
    retval
}

unsafe fn snd_compr_resume(stream: *mut snd_compr_stream) -> c_int {
    let retval: c_int;
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_PAUSED => {
            retval = op_trigger(stream, SNDRV_PCM_TRIGGER_PAUSE_RELEASE);
            if retval == 0 {
                (*(*stream).runtime).state = SNDRV_PCM_STATE_RUNNING;
            }
        }
        s if s == SNDRV_PCM_STATE_DRAINING => {
            if !(*stream).pause_in_draining {
                return -EPERM;
            }
            retval = op_trigger(stream, SNDRV_PCM_TRIGGER_PAUSE_RELEASE);
            if retval == 0 {
                (*stream).pause_in_draining = false;
            }
        }
        _ => return -EPERM,
    }
    retval
}

unsafe fn snd_compr_start(stream: *mut snd_compr_stream) -> c_int {
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_SETUP => {
            if (*stream).direction != SND_COMPRESS_CAPTURE {
                return -EPERM;
            }
        }
        s if s == SNDRV_PCM_STATE_PREPARED => {}
        _ => return -EPERM,
    }
    let retval = op_trigger(stream, SNDRV_PCM_TRIGGER_START);
    if retval == 0 {
        (*(*stream).runtime).state = SNDRV_PCM_STATE_RUNNING;
    }
    retval
}

unsafe fn snd_compr_stop(stream: *mut snd_compr_stream) -> c_int {
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_OPEN || s == SNDRV_PCM_STATE_SETUP || s == SNDRV_PCM_STATE_PREPARED => return -EPERM,
        _ => {}
    }
    let retval = op_trigger(stream, SNDRV_PCM_TRIGGER_STOP);
    if retval == 0 {
        (*stream).partial_drain = false;
        (*stream).metadata_set = false;
        (*stream).pause_in_draining = false;
        snd_compr_drain_notify(stream);
        (*(*stream).runtime).total_bytes_available = 0;
        (*(*stream).runtime).total_bytes_transferred = 0;
    }
    retval
}

extern "C" {
    fn snd_compr_drain_notify(stream: *mut snd_compr_stream);
}

unsafe extern "C" fn error_delayed_work(work: *mut work_struct) {
    let stream = (work as *mut u8).sub(core::mem::offset_of!(snd_compr_stream, error_work) + core::mem::offset_of!(delayed_work, work)) as *mut snd_compr_stream;
    mutex_lock(&mut (*(*stream).device).lock);
    op_trigger(stream, SNDRV_PCM_TRIGGER_STOP);
    wake_up(&mut (*(*stream).runtime).sleep);
    mutex_unlock(&mut (*(*stream).device).lock);
}

#[no_mangle]
pub unsafe extern "C" fn snd_compr_stop_error(stream: *mut snd_compr_stream, state: snd_pcm_state_t) -> c_int {
    if (*(*stream).runtime).state == state {
        return 0;
    }
    (*(*stream).runtime).state = state;
    queue_delayed_work(system_power_efficient_wq, &mut (*stream).error_work, 0);
    0
}

unsafe fn snd_compress_wait_for_drain(stream: *mut snd_compr_stream) -> c_int {
    (*(*stream).runtime).state = SNDRV_PCM_STATE_DRAINING;
    mutex_unlock(&mut (*(*stream).device).lock);
    let ret = wait_event_interruptible((*(*stream).runtime).sleep, (*(*stream).runtime).state != SNDRV_PCM_STATE_DRAINING);
    wake_up(&mut (*(*stream).runtime).sleep);
    mutex_lock(&mut (*(*stream).device).lock);
    ret
}

unsafe fn snd_compr_drain(stream: *mut snd_compr_stream) -> c_int {
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_OPEN || s == SNDRV_PCM_STATE_SETUP || s == SNDRV_PCM_STATE_PREPARED || s == SNDRV_PCM_STATE_PAUSED => return -EPERM,
        s if s == SNDRV_PCM_STATE_XRUN => return -EPIPE,
        _ => {}
    }
    let retval = op_trigger(stream, SND_COMPR_TRIGGER_DRAIN);
    if retval != 0 {
        wake_up(&mut (*(*stream).runtime).sleep);
        return retval;
    }
    snd_compress_wait_for_drain(stream)
}

unsafe fn snd_compr_next_track(stream: *mut snd_compr_stream) -> c_int {
    if (*(*stream).runtime).state != SNDRV_PCM_STATE_RUNNING {
        return -EPERM;
    }
    if (*stream).direction == SND_COMPRESS_CAPTURE {
        return -EPERM;
    }
    if (*stream).metadata_set == false {
        return -EPERM;
    }
    let retval = op_trigger(stream, SND_COMPR_TRIGGER_NEXT_TRACK);
    if retval != 0 {
        return retval;
    }
    (*stream).metadata_set = false;
    (*stream).next_track = true;
    0
}

unsafe fn snd_compr_partial_drain(stream: *mut snd_compr_stream) -> c_int {
    match (*(*stream).runtime).state {
        s if s == SNDRV_PCM_STATE_OPEN || s == SNDRV_PCM_STATE_SETUP || s == SNDRV_PCM_STATE_PREPARED || s == SNDRV_PCM_STATE_PAUSED => return -EPERM,
        s if s == SNDRV_PCM_STATE_XRUN => return -EPIPE,
        _ => {}
    }
    if (*stream).direction == SND_COMPRESS_CAPTURE {
        return -EPERM;
    }
    if (*stream).next_track == false {
        return -EPERM;
    }
    (*stream).partial_drain = true;
    let retval = op_trigger(stream, SND_COMPR_TRIGGER_PARTIAL_DRAIN);
    if retval != 0 {
        wake_up(&mut (*(*stream).runtime).sleep);
        return retval;
    }
    (*stream).next_track = false;
    snd_compress_wait_for_drain(stream)
}

unsafe fn snd_compr_find_task(stream: *mut snd_compr_stream, seqno: __u64) -> *mut snd_compr_task_runtime {
    let head = &mut (*(*stream).runtime).tasks as *mut list_head;
    let mut pos = (*head).next;
    while !pos.is_null() && pos != head {
        let task = pos as *mut snd_compr_task_runtime;
        if (*task).seqno == seqno {
            return task;
        }
        pos = (*pos).next;
    }
    ptr::null_mut()
}

unsafe fn snd_compr_task_free(task: *mut snd_compr_task_runtime) {
    if !(*task).output.is_null() {
        dma_buf_put((*task).output);
    }
    if !(*task).input.is_null() {
        dma_buf_put((*task).input);
    }
    kfree(task as *mut c_void);
}

unsafe fn snd_compr_seqno_next(stream: *mut snd_compr_stream) -> u64 {
    (*(*stream).runtime).task_seqno = (*(*stream).runtime).task_seqno.wrapping_add(1);
    let mut seqno = (*(*stream).runtime).task_seqno;
    if seqno == 0 {
        (*(*stream).runtime).task_seqno = (*(*stream).runtime).task_seqno.wrapping_add(1);
        seqno = (*(*stream).runtime).task_seqno;
    }
    seqno
}

unsafe fn snd_compr_task_new(stream: *mut snd_compr_stream, utask: *mut snd_compr_task) -> c_int {
    if (*(*stream).runtime).total_tasks >= (*(*stream).runtime).fragments {
        return -EBUSY;
    }
    if (*utask).origin_seqno != 0 || (*utask).input_size != 0 {
        return -EINVAL;
    }
    let task = kzalloc(size_of::<snd_compr_task_runtime>(), GFP_KERNEL) as *mut snd_compr_task_runtime;
    if task.is_null() {
        return -ENOMEM;
    }
    (*task).seqno = snd_compr_seqno_next(stream);
    (*utask).seqno = (*task).seqno;
    (*task).input_size = (*utask).input_size;
    let mut retval = ((*(*stream).ops).task_create.unwrap())(stream, task);
    if retval < 0 {
        snd_compr_task_free(task);
        return retval;
    }
    if (*task).input.is_null() || (*(*task).input).file.is_null() || (*task).output.is_null() || (*(*task).output).file.is_null() {
        retval = -EINVAL;
        ((*(*stream).ops).task_free.unwrap())(stream, task);
        snd_compr_task_free(task);
        return retval;
    }
    let fd_i = get_unused_fd_flags(O_WRONLY | O_CLOEXEC);
    if fd_i < 0 {
        ((*(*stream).ops).task_free.unwrap())(stream, task);
        snd_compr_task_free(task);
        return fd_i;
    }
    let fd_o = get_unused_fd_flags(O_RDONLY | O_CLOEXEC);
    if fd_o < 0 {
        put_unused_fd(fd_i);
        ((*(*stream).ops).task_free.unwrap())(stream, task);
        snd_compr_task_free(task);
        return fd_o;
    }
    get_dma_buf((*task).input);
    get_dma_buf((*task).output);
    fd_install(fd_i, (*(*task).input).file);
    fd_install(fd_o, (*(*task).output).file);
    (*utask).input_fd = fd_i;
    (*utask).output_fd = fd_o;
    list_add_tail(&mut (*task).list, &mut (*(*stream).runtime).tasks);
    (*(*stream).runtime).total_tasks += 1;
    0
}

unsafe fn snd_compr_task_create(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    if (*(*stream).runtime).state != SNDRV_PCM_STATE_SETUP {
        return -EPERM;
    }
    let task = memdup_user(arg as *const c_void, size_of::<snd_compr_task>()) as *mut snd_compr_task;
    if IS_ERR(task as *const c_void) {
        return PTR_ERR(task as *const c_void);
    }
    let mut retval = snd_compr_task_new(stream, task);
    if retval >= 0 && copy_to_user(arg as *mut c_void, task as *const c_void, size_of::<snd_compr_task>()) != 0 {
        retval = -EFAULT;
    }
    kfree(task as *mut c_void);
    retval
}

unsafe fn snd_compr_task_start_prepare(task: *mut snd_compr_task_runtime, utask: *mut snd_compr_task) -> c_int {
    if task.is_null() {
        return -EINVAL;
    }
    if (*task).state >= SND_COMPRESS_TASK_STATE_FINISHED {
        return -EBUSY;
    }
    if (*utask).input_size > (*(*task).input).size {
        return -EINVAL;
    }
    (*task).flags = (*utask).flags;
    (*task).input_size = (*utask).input_size;
    (*task).state = SND_COMPRESS_TASK_STATE_IDLE;
    0
}

unsafe fn snd_compr_task_start(stream: *mut snd_compr_stream, utask: *mut snd_compr_task) -> c_int {
    let task: *mut snd_compr_task_runtime;
    let mut retval: c_int;
    if (*utask).origin_seqno > 0 {
        task = snd_compr_find_task(stream, (*utask).origin_seqno);
        retval = snd_compr_task_start_prepare(task, utask);
        if retval < 0 {
            return retval;
        }
        (*task).seqno = snd_compr_seqno_next(stream);
        (*utask).seqno = (*task).seqno;
        (*utask).origin_seqno = 0;
        list_move_tail(&mut (*task).list, &mut (*(*stream).runtime).tasks);
    } else {
        task = snd_compr_find_task(stream, (*utask).seqno);
        if !task.is_null() && (*task).state != SND_COMPRESS_TASK_STATE_IDLE {
            return -EBUSY;
        }
        retval = snd_compr_task_start_prepare(task, utask);
        if retval < 0 {
            return retval;
        }
    }
    retval = ((*(*stream).ops).task_start.unwrap())(stream, task);
    if retval >= 0 {
        (*task).state = SND_COMPRESS_TASK_STATE_ACTIVE;
        (*(*stream).runtime).active_tasks += 1;
    }
    retval
}

unsafe fn snd_compr_task_start_ioctl(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    if (*(*stream).runtime).state != SNDRV_PCM_STATE_SETUP {
        return -EPERM;
    }
    let task = memdup_user(arg as *const c_void, size_of::<snd_compr_task>()) as *mut snd_compr_task;
    if IS_ERR(task as *const c_void) {
        return PTR_ERR(task as *const c_void);
    }
    let mut retval = snd_compr_task_start(stream, task);
    if retval >= 0 && copy_to_user(arg as *mut c_void, task as *const c_void, size_of::<snd_compr_task>()) != 0 {
        retval = -EFAULT;
    }
    kfree(task as *mut c_void);
    retval
}

unsafe extern "C" fn snd_compr_task_stop_one(stream: *mut snd_compr_stream, task: *mut snd_compr_task_runtime) {
    if (*task).state != SND_COMPRESS_TASK_STATE_ACTIVE {
        return;
    }
    ((*(*stream).ops).task_stop.unwrap())(stream, task);
    if !snd_BUG_ON((*(*stream).runtime).active_tasks == 0) {
        (*(*stream).runtime).active_tasks -= 1;
    }
    list_move_tail(&mut (*task).list, &mut (*(*stream).runtime).tasks);
    (*task).state = SND_COMPRESS_TASK_STATE_IDLE;
}

unsafe extern "C" fn snd_compr_task_free_one(stream: *mut snd_compr_stream, task: *mut snd_compr_task_runtime) {
    snd_compr_task_stop_one(stream, task);
    ((*(*stream).ops).task_free.unwrap())(stream, task);
    list_del(&mut (*task).list);
    snd_compr_task_free(task);
    (*(*stream).runtime).total_tasks -= 1;
}

unsafe fn snd_compr_task_seq(stream: *mut snd_compr_stream, arg: c_ulong, fcn: unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime)) -> c_int {
    let mut seqno: __u64 = 0;
    if (*(*stream).runtime).state != SNDRV_PCM_STATE_SETUP {
        return -EPERM;
    }
    if copy_from_user(&mut seqno as *mut _ as *mut c_void, arg as *const c_void, size_of::<__u64>()) != 0 {
        return -EFAULT;
    }
    if seqno == 0 {
        let head = &mut (*(*stream).runtime).tasks as *mut list_head;
        let mut pos = (*head).prev;
        while !pos.is_null() && pos != head {
            let prev = (*pos).prev;
            fcn(stream, pos as *mut snd_compr_task_runtime);
            pos = prev;
        }
        0
    } else {
        let task = snd_compr_find_task(stream, seqno);
        if task.is_null() {
            -EINVAL
        } else {
            fcn(stream, task);
            0
        }
    }
}

unsafe fn snd_compr_task_status(stream: *mut snd_compr_stream, status: *mut snd_compr_task_status) -> c_int {
    let task = snd_compr_find_task(stream, (*status).seqno);
    if task.is_null() {
        return -EINVAL;
    }
    (*status).input_size = (*task).input_size;
    (*status).output_size = (*task).output_size;
    (*status).state = (*task).state;
    0
}

unsafe fn snd_compr_task_status_ioctl(stream: *mut snd_compr_stream, arg: c_ulong) -> c_int {
    if (*(*stream).runtime).state != SNDRV_PCM_STATE_SETUP {
        return -EPERM;
    }
    let status = memdup_user(arg as *const c_void, size_of::<snd_compr_task_status>()) as *mut snd_compr_task_status;
    if IS_ERR(status as *const c_void) {
        return PTR_ERR(status as *const c_void);
    }
    let mut retval = snd_compr_task_status(stream, status);
    if retval >= 0 && copy_to_user(arg as *mut c_void, status as *const c_void, size_of::<snd_compr_task_status>()) != 0 {
        retval = -EFAULT;
    }
    kfree(status as *mut c_void);
    retval
}

#[no_mangle]
pub unsafe extern "C" fn snd_compr_task_finished(stream: *mut snd_compr_stream, task: *mut snd_compr_task_runtime) {
    mutex_lock(&mut (*(*stream).device).lock);
    if !snd_BUG_ON((*(*stream).runtime).active_tasks == 0) {
        (*(*stream).runtime).active_tasks -= 1;
    }
    (*task).state = SND_COMPRESS_TASK_STATE_FINISHED;
    wake_up(&mut (*(*stream).runtime).sleep);
    mutex_unlock(&mut (*(*stream).device).lock);
}

unsafe extern "C" fn snd_compr_ioctl(f: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let data = (*f).private_data as *mut snd_compr_file;
    if snd_BUG_ON(data.is_null()) {
        return -EFAULT as c_long;
    }
    let stream = &mut (*data).stream as *mut snd_compr_stream;

    mutex_lock(&mut (*(*stream).device).lock);
    let ret = if cmd == SNDRV_COMPRESS_IOCTL_VERSION {
        if put_user(SNDRV_COMPRESS_VERSION, arg as *mut c_int) != 0 { -EFAULT } else { 0 }
    } else if cmd == SNDRV_COMPRESS_GET_CAPS {
        snd_compr_get_caps(stream, arg)
    } else if cmd == SNDRV_COMPRESS_GET_CODEC_CAPS {
        snd_compr_get_codec_caps(stream, arg)
    } else if cmd == SNDRV_COMPRESS_SET_PARAMS {
        snd_compr_set_params(stream, arg)
    } else if cmd == SNDRV_COMPRESS_GET_PARAMS {
        snd_compr_get_params(stream, arg)
    } else if cmd == SNDRV_COMPRESS_SET_METADATA {
        snd_compr_set_metadata(stream, arg)
    } else if cmd == SNDRV_COMPRESS_GET_METADATA {
        snd_compr_get_metadata(stream, arg)
    } else if (*stream).direction == SND_COMPRESS_ACCEL {
        if cmd == SNDRV_COMPRESS_TASK_CREATE {
            snd_compr_task_create(stream, arg)
        } else if cmd == SNDRV_COMPRESS_TASK_FREE {
            snd_compr_task_seq(stream, arg, snd_compr_task_free_one)
        } else if cmd == SNDRV_COMPRESS_TASK_START {
            snd_compr_task_start_ioctl(stream, arg)
        } else if cmd == SNDRV_COMPRESS_TASK_STOP {
            snd_compr_task_seq(stream, arg, snd_compr_task_stop_one)
        } else if cmd == SNDRV_COMPRESS_TASK_STATUS {
            snd_compr_task_status_ioctl(stream, arg)
        } else {
            -ENOTTY
        }
    } else if cmd == SNDRV_COMPRESS_TSTAMP {
        snd_compr_tstamp(stream, arg, true)
    } else if cmd == SNDRV_COMPRESS_TSTAMP64 {
        snd_compr_tstamp(stream, arg, false)
    } else if cmd == SNDRV_COMPRESS_AVAIL {
        snd_compr_ioctl_avail(stream, arg, true)
    } else if cmd == SNDRV_COMPRESS_AVAIL64 {
        snd_compr_ioctl_avail(stream, arg, false)
    } else if cmd == SNDRV_COMPRESS_PAUSE {
        snd_compr_pause(stream)
    } else if cmd == SNDRV_COMPRESS_RESUME {
        snd_compr_resume(stream)
    } else if cmd == SNDRV_COMPRESS_START {
        snd_compr_start(stream)
    } else if cmd == SNDRV_COMPRESS_STOP {
        snd_compr_stop(stream)
    } else if cmd == SNDRV_COMPRESS_DRAIN {
        snd_compr_drain(stream)
    } else if cmd == SNDRV_COMPRESS_PARTIAL_DRAIN {
        snd_compr_partial_drain(stream)
    } else if cmd == SNDRV_COMPRESS_NEXT_TRACK {
        snd_compr_next_track(stream)
    } else {
        -ENOTTY
    };
    mutex_unlock(&mut (*(*stream).device).lock);
    ret as c_long
}

/* support of 32bit userspace on 64bit platforms */
unsafe extern "C" fn snd_compr_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    snd_compr_ioctl(file, cmd, compat_ptr(arg) as c_ulong)
}

static snd_compr_file_ops: file_operations = file_operations {
    owner: unsafe { THIS_MODULE },
    open: Some(snd_compr_open),
    release: Some(snd_compr_free),
    write: Some(snd_compr_write),
    read: Some(snd_compr_read),
    unlocked_ioctl: Some(snd_compr_ioctl),
    compat_ioctl: Some(snd_compr_ioctl_compat),
    mmap: Some(snd_compr_mmap),
    poll: Some(snd_compr_poll),
};

unsafe extern "C" fn snd_compress_dev_register(device: *mut snd_device) -> c_int {
    if snd_BUG_ON(device.is_null() || (*device).device_data.is_null()) {
        return -EBADFD;
    }
    let compr = (*device).device_data as *mut snd_compr;
    let ret = snd_register_device(
        SNDRV_DEVICE_TYPE_COMPRESS,
        (*compr).card,
        (*compr).device,
        &snd_compr_file_ops,
        compr,
        (*compr).dev,
    );
    if ret < 0 {
        return ret;
    }
    ret
}

unsafe extern "C" fn snd_compress_dev_disconnect(device: *mut snd_device) -> c_int {
    let compr = (*device).device_data as *mut snd_compr;
    snd_unregister_device((*compr).dev);
    0
}

/* CONFIG_SND_VERBOSE_PROCFS branch translated; when disabled these functions are
 * no-op/zero inline equivalents in the C source.
 */
unsafe extern "C" fn snd_compress_proc_info_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let compr = (*entry).private_data as *mut snd_compr;
    snd_iprintf(buffer, b"card: %d\n\0".as_ptr() as *const c_char, (*(*compr).card).number);
    snd_iprintf(buffer, b"device: %d\n\0".as_ptr() as *const c_char, (*compr).device);
    snd_iprintf(
        buffer,
        b"stream: %s\n\0".as_ptr() as *const c_char,
        if (*compr).direction == SND_COMPRESS_PLAYBACK {
            b"PLAYBACK\0".as_ptr() as *const c_char
        } else {
            b"CAPTURE\0".as_ptr() as *const c_char
        },
    );
    snd_iprintf(buffer, b"id: %s\n\0".as_ptr() as *const c_char, (*compr).id.as_ptr());
}

unsafe fn snd_compress_proc_init(compr: *mut snd_compr) -> c_int {
    let mut name = [0 as c_char; 16];
    sprintf(name.as_mut_ptr(), b"compr%i\0".as_ptr() as *const c_char, (*compr).device);
    let entry = snd_info_create_card_entry((*compr).card, name.as_ptr(), (*(*compr).card).proc_root);
    if entry.is_null() {
        return -ENOMEM;
    }
    (*entry).mode = 0o040000 | 0o555;
    (*compr).proc_root = entry;
    let info = snd_info_create_card_entry((*compr).card, b"info\0".as_ptr() as *const c_char, (*compr).proc_root);
    if !info.is_null() {
        snd_info_set_text_ops(info, compr, snd_compress_proc_info_read);
    }
    (*compr).proc_info_entry = info;
    0
}

unsafe fn snd_compress_proc_done(compr: *mut snd_compr) {
    snd_info_free_entry((*compr).proc_info_entry);
    (*compr).proc_info_entry = ptr::null_mut();
    snd_info_free_entry((*compr).proc_root);
    (*compr).proc_root = ptr::null_mut();
}

unsafe fn snd_compress_set_id(compr: *mut snd_compr, id: *const c_char) {
    strscpy((*compr).id.as_mut_ptr(), id, (*compr).id.len());
}

unsafe extern "C" fn snd_compress_dev_free(device: *mut snd_device) -> c_int {
    let compr = (*device).device_data as *mut snd_compr;
    snd_compress_proc_done(compr);
    put_device((*compr).dev);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_compress_new(
    card: *mut snd_card,
    device: c_int,
    dirn: c_int,
    id: *const c_char,
    compr: *mut snd_compr,
) -> c_int {
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_compress_dev_free),
        dev_register: Some(snd_compress_dev_register),
        dev_disconnect: Some(snd_compress_dev_disconnect),
    };

    (*compr).card = card;
    (*compr).device = device;
    (*compr).direction = dirn;
    mutex_init(&mut (*compr).lock);

    snd_compress_set_id(compr, id);

    let mut ret = snd_device_alloc(&mut (*compr).dev, card);
    if ret != 0 {
        return ret;
    }
    dev_set_name((*compr).dev, b"comprC%iD%i\0".as_ptr() as *const c_char, (*card).number, device);

    ret = snd_device_new(card, SNDRV_DEV_COMPRESS, compr as *mut c_void as *mut snd_compr, &ops);
    if ret == 0 {
        snd_compress_proc_init(compr);
    } else {
        put_device((*compr).dev);
    }
    ret
}

/* EXPORT_SYMBOL/EXPORT_SYMBOL_GPL/MODULE_IMPORT_NS/MODULE_DESCRIPTION/
 * MODULE_AUTHOR/MODULE_LICENSE are module metadata in C and have no direct
 * executable Rust equivalent in this isolated translation.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
