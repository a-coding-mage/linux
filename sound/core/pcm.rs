// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Digital Audio (PCM) abstract layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type bool_t = bool;
type snd_pcm_format_t = usize;
type snd_pcm_access_t = usize;
type snd_pcm_subformat_t = usize;
type snd_pcm_state_t = usize;

const EBUSY: c_int = 16;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const ENOENT: c_int = 2;
const ENOIOCTLCMD: c_int = 515;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EAGAIN: c_int = 11;
const EBADFD: c_int = 77;
const GFP_KERNEL: c_uint = 0;
const O_APPEND: c_int = 0o2000;
const UINT_MAX: c_uint = c_uint::MAX;
const S_IFDIR: c_uint = 0o040000;
const S_IFREG: c_uint = 0o100000;

const SNDRV_CTL_IOCTL_PCM_NEXT_DEVICE: c_uint = 0;
const SNDRV_CTL_IOCTL_PCM_INFO: c_uint = 1;
const SNDRV_CTL_IOCTL_PCM_PREFER_SUBDEVICE: c_uint = 2;
const SND_CTL_SUBDEV_PCM: usize = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_STATE_OPEN: c_int = 0;
const SNDRV_PCM_STATE_DISCONNECTED: c_int = 8;
const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 1;
const SNDRV_DEV_PCM: c_int = 0;
const SNDRV_DEVICE_TYPE_PCM_PLAYBACK: c_int = 0;
const SNDRV_DEVICE_TYPE_PCM_CAPTURE: c_int = 1;
const SNDRV_PCM_CLASS_LAST: usize = 3;
const SNDRV_PCM_CLASS_GENERIC: usize = 0;
const SNDRV_PCM_CLASS_MULTI: usize = 1;
const SNDRV_PCM_CLASS_MODEM: usize = 2;
const SNDRV_PCM_CLASS_DIGITIZER: usize = 3;

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    counter: c_int,
}

#[repr(C)]
pub struct snd_card {
    number: c_int,
    proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_device {
    device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct device {
    groups: *const *const attribute_group,
    type_: *const device_type,
}

#[repr(C)]
pub struct device_type {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_attribute {
    attr: attribute,
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct file {
    f_flags: c_int,
}

#[repr(C)]
pub struct snd_ctl_file {
    preferred_subdevice: [c_int; 1],
}

#[repr(C)]
pub struct snd_pcm {
    card: *mut snd_card,
    device: c_int,
    internal: bool_t,
    list: list_head,
    streams: [snd_pcm_str; 2],
    open_mutex: mutex,
    open_wait: wait_queue_head_t,
    id: [c_char; 64],
    name: [c_char; 80],
    no_device_suspend: bool_t,
    private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>,
    private_data: *mut c_void,
    info_flags: c_uint,
    dev_class: c_uint,
}

#[repr(C)]
pub struct snd_pcm_str {
    stream: c_int,
    pcm: *mut snd_pcm,
    substream_count: c_int,
    substream: *mut snd_pcm_substream,
    proc_root: *mut snd_info_entry,
    dev: *mut device,
    chmap_kctl: *mut c_void,
    substream_opened: c_int,
    xrun_debug: c_int,
    oss: snd_pcm_str_oss,
}

#[repr(C)]
pub struct snd_pcm_str_oss {
    setup_mutex: mutex,
    setup_list: *mut snd_pcm_oss_setup,
}

#[repr(C)]
pub struct snd_pcm_oss_setup {
    next: *mut snd_pcm_oss_setup,
    task_name: *mut c_char,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pcm: *mut snd_pcm,
    pstr: *mut snd_pcm_str,
    number: c_int,
    stream: c_int,
    name: [c_char; 32],
    buffer_bytes_max: c_uint,
    next: *mut snd_pcm_substream,
    proc_root: *mut snd_info_entry,
    group: *mut snd_pcm_group,
    self_group: snd_pcm_group,
    link_list: list_head,
    mmap_count: atomic_t,
    runtime: *mut snd_pcm_runtime,
    private_data: *mut c_void,
    ref_count: c_int,
    f_flags: c_int,
    pid: *mut pid,
    timer: *mut snd_timer,
    xrun_counter: c_int,
    oss: snd_pcm_substream_oss,
}

#[repr(C)]
pub struct snd_pcm_substream_oss {
    oss: bool_t,
}

#[repr(C)]
pub struct snd_pcm_group {
    substreams: list_head,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    status: *mut snd_pcm_mmap_status,
    control: *mut snd_pcm_mmap_control,
    sleep: wait_queue_head_t,
    tsleep: wait_queue_head_t,
    state: c_int,
    buffer_mutex: mutex,
    buffer_accessing: atomic_t,
    private_free: Option<unsafe extern "C" fn(*mut snd_pcm_runtime)>,
    hw_constraints: snd_pcm_hw_constraints,
    fasync: *mut c_void,
    access: snd_pcm_access_t,
    format: snd_pcm_format_t,
    subformat: snd_pcm_subformat_t,
    channels: c_uint,
    rate: c_uint,
    rate_num: c_uint,
    rate_den: c_uint,
    period_size: c_ulong,
    buffer_size: c_ulong,
    tstamp_mode: c_int,
    period_step: c_uint,
    start_threshold: c_ulong,
    stop_threshold: c_ulong,
    silence_threshold: c_ulong,
    silence_size: c_ulong,
    boundary: c_ulong,
    oss: snd_pcm_runtime_oss,
}

#[repr(C)]
pub struct snd_pcm_runtime_oss {
    format: c_int,
    channels: c_uint,
    rate: c_uint,
    period_bytes: c_ulong,
    periods: c_uint,
    period_frames: c_ulong,
}

#[repr(C)]
pub struct snd_pcm_hw_constraints {
    rules: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_mmap_status {
    hw_ptr: c_long,
}

#[repr(C)]
pub struct snd_pcm_mmap_control {
    appl_ptr: c_long,
    avail_min: c_ulong,
}

type c_long = isize;

#[repr(C)]
pub struct snd_pcm_info {
    device: c_uint,
    subdevice: c_uint,
    stream: c_int,
    card: c_int,
    id: [c_char; 64],
    name: [c_char; 80],
    subname: [c_char; 32],
    dev_class: c_int,
    dev_subclass: c_int,
    subdevices_count: c_int,
    subdevices_avail: c_int,
}

#[repr(C)]
pub struct snd_pcm_status64 {
    state: snd_pcm_state_t,
    trigger_tstamp_sec: i64,
    trigger_tstamp_nsec: i64,
    tstamp_sec: i64,
    tstamp_nsec: i64,
    delay: c_long,
    avail: c_long,
    avail_max: c_long,
}

#[repr(C)]
pub struct snd_info_entry {
    private_data: *mut c_void,
    mode: c_uint,
    c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_info_entry_c {
    text: snd_info_text_ops,
}

#[repr(C)]
pub struct snd_info_text_ops {
    write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_notify {
    list: list_head,
    n_register: Option<unsafe extern "C" fn(*mut snd_pcm)>,
    n_unregister: Option<unsafe extern "C" fn(*mut snd_pcm)>,
    n_disconnect: Option<unsafe extern "C" fn(*mut snd_pcm)>,
}

#[repr(C)]
pub struct snd_timer {
    lock: spinlock_t,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pid {
    _private: [u8; 0],
}

extern "C" {
    static mut snd_pcm_f_ops: [c_void; 2];
    static mut current: *mut c_void;
    static mut THIS_MODULE: *mut c_void;
    fn mutex_init(mutex: *mut mutex);
    fn mutex_destroy(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    fn wake_up(wq: *mut wait_queue_head_t);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(list: *mut list_head);
    fn list_del_init(list: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn snd_device_alloc(dev: *mut *mut device, card: *mut snd_card) -> c_int;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_device_new(card: *mut snd_card, typ: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_register_device(typ: c_int, card: *mut snd_card, device: c_int, fops: *const c_void, data: *mut c_void, dev: *mut device) -> c_int;
    fn snd_unregister_device(dev: *mut device);
    fn put_device(dev: *mut device);
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn alloc_pages_exact(size: size_t, flags: c_uint) -> *mut c_void;
    fn free_pages_exact(p: *mut c_void, size: size_t);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> isize;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn snd_BUG_ON(cond: bool_t) -> bool_t;
    fn snd_pcm_info(substream: *mut snd_pcm_substream, info: *mut snd_pcm_info) -> c_int;
    fn snd_pcm_info_user(substream: *mut snd_pcm_substream, info: *mut snd_pcm_info) -> c_int;
    fn snd_pcm_status64(substream: *mut snd_pcm_substream, status: *mut snd_pcm_status64) -> c_int;
    fn snd_pcm_suspend_all(pcm: *mut snd_pcm);
    fn snd_pcm_group_init(group: *mut snd_pcm_group);
    fn snd_pcm_timer_init(substream: *mut snd_pcm_substream);
    fn snd_pcm_timer_done(substream: *mut snd_pcm_substream);
    fn snd_pcm_lib_preallocate_free_for_all(pcm: *mut snd_pcm);
    fn snd_ctl_remove(card: *mut snd_card, kctl: *mut c_void);
    fn snd_ctl_get_preferred_subdevice(card: *mut snd_card, subdev: usize) -> c_int;
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> bool_t;
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_int);
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_pcm_sync_stop(substream: *mut snd_pcm_substream, sync: bool_t);
    fn __snd_pcm_set_state(runtime: *mut snd_pcm_runtime, state: c_int);
    fn snd_pcm_stream_lock_irq(substream: *mut snd_pcm_substream);
    fn snd_pcm_stream_unlock_irq(substream: *mut snd_pcm_substream);
    fn snd_fasync_free(fasync: *mut c_void);
    fn get_pid(pid: *mut pid) -> *mut pid;
    fn put_pid(pid: *mut pid);
    fn task_pid(task: *mut c_void) -> *mut pid;
    fn pid_vnr(pid: *mut pid) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_create_card_entry(card: *mut snd_card, name: *const c_char, parent: *mut snd_info_entry) -> *mut snd_info_entry;
    fn snd_info_create_module_entry(module: *mut c_void, name: *const c_char, parent: *mut snd_info_entry) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(entry: *mut snd_info_entry, private: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn simple_strtoul(cp: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn snd_ctl_register_ioctl(f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
    fn snd_ctl_register_ioctl_compat(f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
    fn snd_ctl_unregister_ioctl(f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
    fn snd_ctl_unregister_ioctl_compat(f: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
}

static mut snd_pcm_devices: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut register_mutex: mutex = mutex { _private: [] };
/* IS_ENABLED(CONFIG_SND_PCM_OSS): static LIST_HEAD(snd_pcm_notify_list); */
static mut snd_pcm_notify_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

unsafe fn PAGE_ALIGN(size: size_t) -> size_t {
    (size + 4095) & !4095
}

unsafe fn SUBSTREAM_BUSY(substream: *mut snd_pcm_substream) -> bool {
    !(*substream).runtime.is_null()
}

unsafe fn snd_pcm_get(card: *mut snd_card, device: c_int) -> *mut snd_pcm {
    let mut pos = snd_pcm_devices.next as *mut snd_pcm;
    while !pos.is_null() && pos as *mut list_head != &raw mut snd_pcm_devices {
        if (*pos).card == card && (*pos).device == device {
            return pos;
        }
        pos = (*pos).list.next as *mut snd_pcm;
    }
    ptr::null_mut()
}

unsafe fn snd_pcm_next(card: *mut snd_card, device: c_int) -> c_int {
    let mut pcm = snd_pcm_devices.next as *mut snd_pcm;
    while !pcm.is_null() && pcm as *mut list_head != &raw mut snd_pcm_devices {
        if (*pcm).card == card && (*pcm).device > device {
            return (*pcm).device;
        } else if (*(*pcm).card).number > (*card).number {
            return -1;
        }
        pcm = (*pcm).list.next as *mut snd_pcm;
    }
    -1
}

unsafe fn snd_pcm_add(newpcm: *mut snd_pcm) -> c_int {
    if (*newpcm).internal {
        return 0;
    }
    let mut pcm = snd_pcm_devices.next as *mut snd_pcm;
    while !pcm.is_null() && pcm as *mut list_head != &raw mut snd_pcm_devices {
        if (*pcm).card == (*newpcm).card && (*pcm).device == (*newpcm).device {
            return -EBUSY;
        }
        if (*(*pcm).card).number > (*(*newpcm).card).number ||
            ((*pcm).card == (*newpcm).card && (*pcm).device > (*newpcm).device) {
            list_add(&mut (*newpcm).list, (*pcm).list.prev);
            return 0;
        }
        pcm = (*pcm).list.next as *mut snd_pcm;
    }
    list_add_tail(&mut (*newpcm).list, &raw mut snd_pcm_devices);
    0
}

unsafe extern "C" fn snd_pcm_control_ioctl(card: *mut snd_card, control: *mut snd_ctl_file, cmd: c_uint, arg: c_ulong) -> c_int {
    match cmd {
        SNDRV_CTL_IOCTL_PCM_NEXT_DEVICE => {
            let devicep = arg as *mut c_int;
            let mut device = *devicep;
            mutex_lock(&raw mut register_mutex);
            device = snd_pcm_next(card, device);
            mutex_unlock(&raw mut register_mutex);
            *devicep = device;
            0
        }
        SNDRV_CTL_IOCTL_PCM_INFO => {
            let info = arg as *mut snd_pcm_info;
            let device = (*info).device;
            let mut stream = (*info).stream;
            if stream < 0 || stream > 1 {
                return -EINVAL;
            }
            let subdevice = (*info).subdevice;
            mutex_lock(&raw mut register_mutex);
            let pcm = snd_pcm_get(card, device as c_int);
            if pcm.is_null() {
                mutex_unlock(&raw mut register_mutex);
                return -ENXIO;
            }
            let pstr = &mut (*pcm).streams[stream as usize] as *mut snd_pcm_str;
            if (*pstr).substream_count == 0 {
                mutex_unlock(&raw mut register_mutex);
                return -ENOENT;
            }
            if subdevice >= (*pstr).substream_count as c_uint {
                mutex_unlock(&raw mut register_mutex);
                return -ENXIO;
            }
            let mut substream = (*pstr).substream;
            while !substream.is_null() {
                if (*substream).number == subdevice as c_int {
                    break;
                }
                substream = (*substream).next;
            }
            if substream.is_null() {
                mutex_unlock(&raw mut register_mutex);
                return -ENXIO;
            }
            mutex_lock(&mut (*pcm).open_mutex);
            let ret = snd_pcm_info_user(substream, info);
            mutex_unlock(&mut (*pcm).open_mutex);
            mutex_unlock(&raw mut register_mutex);
            ret
        }
        SNDRV_CTL_IOCTL_PCM_PREFER_SUBDEVICE => {
            let val = *(arg as *mut c_int);
            (*control).preferred_subdevice[SND_CTL_SUBDEV_PCM] = val;
            0
        }
        _ => -ENOIOCTLCMD,
    }
}

static snd_pcm_format_names: [Option<&'static [u8]>; 51] = [
    Some(b"S8\0"), Some(b"U8\0"), Some(b"S16_LE\0"), Some(b"S16_BE\0"),
    Some(b"U16_LE\0"), Some(b"U16_BE\0"), Some(b"S24_LE\0"), Some(b"S24_BE\0"),
    Some(b"U24_LE\0"), Some(b"U24_BE\0"), Some(b"S32_LE\0"), Some(b"S32_BE\0"),
    Some(b"U32_LE\0"), Some(b"U32_BE\0"), Some(b"FLOAT_LE\0"), Some(b"FLOAT_BE\0"),
    Some(b"FLOAT64_LE\0"), Some(b"FLOAT64_BE\0"), Some(b"IEC958_SUBFRAME_LE\0"),
    Some(b"IEC958_SUBFRAME_BE\0"), Some(b"MU_LAW\0"), Some(b"A_LAW\0"),
    Some(b"IMA_ADPCM\0"), Some(b"MPEG\0"), Some(b"GSM\0"), Some(b"SPECIAL\0"),
    Some(b"S24_3LE\0"), Some(b"S24_3BE\0"), Some(b"U24_3LE\0"), Some(b"U24_3BE\0"),
    Some(b"S20_3LE\0"), Some(b"S20_3BE\0"), Some(b"U20_3LE\0"), Some(b"U20_3BE\0"),
    Some(b"S18_3LE\0"), Some(b"S18_3BE\0"), Some(b"U18_3LE\0"), Some(b"U18_3BE\0"),
    Some(b"G723_24\0"), Some(b"G723_24_1B\0"), Some(b"G723_40\0"), Some(b"G723_40_1B\0"),
    Some(b"DSD_U8\0"), Some(b"DSD_U16_LE\0"), Some(b"DSD_U32_LE\0"),
    Some(b"DSD_U16_BE\0"), Some(b"DSD_U32_BE\0"), Some(b"S20_LE\0"),
    Some(b"S20_BE\0"), Some(b"U20_LE\0"), Some(b"U20_BE\0"),
];

/**
 * snd_pcm_format_name - Return a name string for the given PCM format
 * @format: PCM format
 *
 * Return: the format name string
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_name(format: snd_pcm_format_t) -> *const c_char {
    if format >= snd_pcm_format_names.len() || snd_pcm_format_names[format].is_none() {
        return b"Unknown\0".as_ptr() as *const c_char;
    }
    snd_pcm_format_names[format].unwrap().as_ptr() as *const c_char
}

/* CONFIG_SND_VERBOSE_PROCFS */
static snd_pcm_stream_names: [&[u8]; 2] = [b"PLAYBACK\0", b"CAPTURE\0"];
static snd_pcm_state_names: [&[u8]; 9] = [b"OPEN\0", b"SETUP\0", b"PREPARED\0", b"RUNNING\0", b"XRUN\0", b"DRAINING\0", b"PAUSED\0", b"SUSPENDED\0", b"DISCONNECTED\0"];
static snd_pcm_access_names: [&[u8]; 5] = [b"MMAP_INTERLEAVED\0", b"MMAP_NONINTERLEAVED\0", b"MMAP_COMPLEX\0", b"RW_INTERLEAVED\0", b"RW_NONINTERLEAVED\0"];
static snd_pcm_subformat_names: [&[u8]; 4] = [b"STD\0", b"MSBITS_MAX\0", b"MSBITS_20\0", b"MSBITS_24\0"];
static snd_pcm_tstamp_mode_names: [&[u8]; 2] = [b"NONE\0", b"ENABLE\0"];

unsafe fn snd_pcm_stream_name(stream: c_int) -> *const c_char { snd_pcm_stream_names[stream as usize].as_ptr() as *const c_char }
unsafe fn snd_pcm_access_name(access: snd_pcm_access_t) -> *const c_char { snd_pcm_access_names[access].as_ptr() as *const c_char }
unsafe fn snd_pcm_subformat_name(subformat: snd_pcm_subformat_t) -> *const c_char { snd_pcm_subformat_names[subformat].as_ptr() as *const c_char }
unsafe fn snd_pcm_tstamp_mode_name(mode: c_int) -> *const c_char { snd_pcm_tstamp_mode_names[mode as usize].as_ptr() as *const c_char }
unsafe fn snd_pcm_state_name(state: snd_pcm_state_t) -> *const c_char { snd_pcm_state_names[state].as_ptr() as *const c_char }

unsafe fn snd_pcm_oss_format_name(format: c_int) -> *const c_char {
    match format {
        1 => b"MU_LAW\0".as_ptr() as *const c_char,
        2 => b"A_LAW\0".as_ptr() as *const c_char,
        3 => b"IMA_ADPCM\0".as_ptr() as *const c_char,
        4 => b"U8\0".as_ptr() as *const c_char,
        5 => b"S16_LE\0".as_ptr() as *const c_char,
        6 => b"S16_BE\0".as_ptr() as *const c_char,
        7 => b"S8\0".as_ptr() as *const c_char,
        8 => b"U16_LE\0".as_ptr() as *const c_char,
        9 => b"U16_BE\0".as_ptr() as *const c_char,
        10 => b"MPEG\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

unsafe extern "C" fn snd_pcm_proc_info_read(substream: *mut snd_pcm_substream, buffer: *mut snd_info_buffer) {
    if substream.is_null() { return; }
    let info = kmalloc(size_of::<snd_pcm_info>(), GFP_KERNEL) as *mut snd_pcm_info;
    if info.is_null() { return; }
    let err = snd_pcm_info(substream, info);
    if err < 0 {
        snd_iprintf(buffer, b"error %d\n\0".as_ptr() as *const c_char, err);
        kfree(info as *mut c_void);
        return;
    }
    snd_iprintf(buffer, b"card: %d\n\0".as_ptr() as *const c_char, (*info).card);
    snd_iprintf(buffer, b"device: %d\n\0".as_ptr() as *const c_char, (*info).device);
    snd_iprintf(buffer, b"subdevice: %d\n\0".as_ptr() as *const c_char, (*info).subdevice);
    snd_iprintf(buffer, b"stream: %s\n\0".as_ptr() as *const c_char, snd_pcm_stream_name((*info).stream));
    snd_iprintf(buffer, b"id: %s\n\0".as_ptr() as *const c_char, (*info).id.as_ptr());
    snd_iprintf(buffer, b"name: %s\n\0".as_ptr() as *const c_char, (*info).name.as_ptr());
    snd_iprintf(buffer, b"subname: %s\n\0".as_ptr() as *const c_char, (*info).subname.as_ptr());
    snd_iprintf(buffer, b"class: %d\n\0".as_ptr() as *const c_char, (*info).dev_class);
    snd_iprintf(buffer, b"subclass: %d\n\0".as_ptr() as *const c_char, (*info).dev_subclass);
    snd_iprintf(buffer, b"subdevices_count: %d\n\0".as_ptr() as *const c_char, (*info).subdevices_count);
    snd_iprintf(buffer, b"subdevices_avail: %d\n\0".as_ptr() as *const c_char, (*info).subdevices_avail);
    kfree(info as *mut c_void);
}

unsafe extern "C" fn snd_pcm_stream_proc_info_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_pcm_proc_info_read(((*((*entry).private_data as *mut snd_pcm_str)).substream), buffer);
}

unsafe extern "C" fn snd_pcm_substream_proc_info_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_pcm_proc_info_read((*entry).private_data as *mut snd_pcm_substream, buffer);
}

unsafe extern "C" fn snd_pcm_substream_proc_hw_params_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let substream = (*entry).private_data as *mut snd_pcm_substream;
    mutex_lock(&mut (*(*substream).pcm).open_mutex);
    let runtime = (*substream).runtime;
    if runtime.is_null() {
        snd_iprintf(buffer, b"closed\n\0".as_ptr() as *const c_char);
        mutex_unlock(&mut (*(*substream).pcm).open_mutex);
        return;
    }
    if (*runtime).state == SNDRV_PCM_STATE_OPEN {
        snd_iprintf(buffer, b"no setup\n\0".as_ptr() as *const c_char);
        mutex_unlock(&mut (*(*substream).pcm).open_mutex);
        return;
    }
    snd_iprintf(buffer, b"access: %s\n\0".as_ptr() as *const c_char, snd_pcm_access_name((*runtime).access));
    snd_iprintf(buffer, b"format: %s\n\0".as_ptr() as *const c_char, snd_pcm_format_name((*runtime).format));
    snd_iprintf(buffer, b"subformat: %s\n\0".as_ptr() as *const c_char, snd_pcm_subformat_name((*runtime).subformat));
    snd_iprintf(buffer, b"channels: %u\n\0".as_ptr() as *const c_char, (*runtime).channels);
    snd_iprintf(buffer, b"rate: %u (%u/%u)\n\0".as_ptr() as *const c_char, (*runtime).rate, (*runtime).rate_num, (*runtime).rate_den);
    snd_iprintf(buffer, b"period_size: %lu\n\0".as_ptr() as *const c_char, (*runtime).period_size);
    snd_iprintf(buffer, b"buffer_size: %lu\n\0".as_ptr() as *const c_char, (*runtime).buffer_size);
    if (*substream).oss.oss {
        snd_iprintf(buffer, b"OSS format: %s\n\0".as_ptr() as *const c_char, snd_pcm_oss_format_name((*runtime).oss.format));
        snd_iprintf(buffer, b"OSS channels: %u\n\0".as_ptr() as *const c_char, (*runtime).oss.channels);
        snd_iprintf(buffer, b"OSS rate: %u\n\0".as_ptr() as *const c_char, (*runtime).oss.rate);
        snd_iprintf(buffer, b"OSS period bytes: %lu\n\0".as_ptr() as *const c_char, (*runtime).oss.period_bytes);
        snd_iprintf(buffer, b"OSS periods: %u\n\0".as_ptr() as *const c_char, (*runtime).oss.periods);
        snd_iprintf(buffer, b"OSS period frames: %lu\n\0".as_ptr() as *const c_char, (*runtime).oss.period_frames);
    }
    mutex_unlock(&mut (*(*substream).pcm).open_mutex);
}

unsafe extern "C" fn snd_pcm_substream_proc_sw_params_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let substream = (*entry).private_data as *mut snd_pcm_substream;
    mutex_lock(&mut (*(*substream).pcm).open_mutex);
    let runtime = (*substream).runtime;
    if runtime.is_null() {
        snd_iprintf(buffer, b"closed\n\0".as_ptr() as *const c_char);
    } else if (*runtime).state == SNDRV_PCM_STATE_OPEN {
        snd_iprintf(buffer, b"no setup\n\0".as_ptr() as *const c_char);
    } else {
        snd_iprintf(buffer, b"tstamp_mode: %s\n\0".as_ptr() as *const c_char, snd_pcm_tstamp_mode_name((*runtime).tstamp_mode));
        snd_iprintf(buffer, b"period_step: %u\n\0".as_ptr() as *const c_char, (*runtime).period_step);
        snd_iprintf(buffer, b"avail_min: %lu\n\0".as_ptr() as *const c_char, (*(*runtime).control).avail_min);
        snd_iprintf(buffer, b"start_threshold: %lu\n\0".as_ptr() as *const c_char, (*runtime).start_threshold);
        snd_iprintf(buffer, b"stop_threshold: %lu\n\0".as_ptr() as *const c_char, (*runtime).stop_threshold);
        snd_iprintf(buffer, b"silence_threshold: %lu\n\0".as_ptr() as *const c_char, (*runtime).silence_threshold);
        snd_iprintf(buffer, b"silence_size: %lu\n\0".as_ptr() as *const c_char, (*runtime).silence_size);
        snd_iprintf(buffer, b"boundary: %lu\n\0".as_ptr() as *const c_char, (*runtime).boundary);
    }
    mutex_unlock(&mut (*(*substream).pcm).open_mutex);
}

unsafe extern "C" fn snd_pcm_substream_proc_status_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let substream = (*entry).private_data as *mut snd_pcm_substream;
    let mut status: snd_pcm_status64 = core::mem::zeroed();
    mutex_lock(&mut (*(*substream).pcm).open_mutex);
    let runtime = (*substream).runtime;
    if runtime.is_null() {
        snd_iprintf(buffer, b"closed\n\0".as_ptr() as *const c_char);
        mutex_unlock(&mut (*(*substream).pcm).open_mutex);
        return;
    }
    let err = snd_pcm_status64(substream, &mut status);
    if err < 0 {
        snd_iprintf(buffer, b"error %d\n\0".as_ptr() as *const c_char, err);
        mutex_unlock(&mut (*(*substream).pcm).open_mutex);
        return;
    }
    snd_iprintf(buffer, b"state: %s\n\0".as_ptr() as *const c_char, snd_pcm_state_name(status.state));
    snd_iprintf(buffer, b"owner_pid   : %d\n\0".as_ptr() as *const c_char, pid_vnr((*substream).pid));
    snd_iprintf(buffer, b"trigger_time: %lld.%09lld\n\0".as_ptr() as *const c_char, status.trigger_tstamp_sec, status.trigger_tstamp_nsec);
    snd_iprintf(buffer, b"tstamp      : %lld.%09lld\n\0".as_ptr() as *const c_char, status.tstamp_sec, status.tstamp_nsec);
    snd_iprintf(buffer, b"delay       : %ld\n\0".as_ptr() as *const c_char, status.delay);
    snd_iprintf(buffer, b"avail       : %ld\n\0".as_ptr() as *const c_char, status.avail);
    snd_iprintf(buffer, b"avail_max   : %ld\n\0".as_ptr() as *const c_char, status.avail_max);
    snd_iprintf(buffer, b"-----\n\0".as_ptr() as *const c_char);
    snd_iprintf(buffer, b"hw_ptr      : %ld\n\0".as_ptr() as *const c_char, (*(*runtime).status).hw_ptr);
    snd_iprintf(buffer, b"appl_ptr    : %ld\n\0".as_ptr() as *const c_char, (*(*runtime).control).appl_ptr);
    snd_iprintf(buffer, b"xrun_counter: %d\n\0".as_ptr() as *const c_char, (*substream).xrun_counter);
    mutex_unlock(&mut (*(*substream).pcm).open_mutex);
}

unsafe extern "C" fn snd_pcm_xrun_injection_write(entry: *mut snd_info_entry, _buffer: *mut snd_info_buffer) {
    snd_pcm_stop_xrun((*entry).private_data as *mut snd_pcm_substream);
}

unsafe extern "C" fn snd_pcm_xrun_debug_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let pstr = (*entry).private_data as *mut snd_pcm_str;
    snd_iprintf(buffer, b"%d\n\0".as_ptr() as *const c_char, (*pstr).xrun_debug);
}

unsafe extern "C" fn snd_pcm_xrun_debug_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let pstr = (*entry).private_data as *mut snd_pcm_str;
    let mut line = [0 as c_char; 64];
    if snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
        (*pstr).xrun_debug = simple_strtoul(line.as_ptr(), ptr::null_mut(), 10) as c_int;
    }
}

unsafe fn snd_pcm_stream_proc_init(pstr: *mut snd_pcm_str) -> c_int {
    let pcm = (*pstr).pcm;
    let mut name = [0 as c_char; 16];
    sprintf(name.as_mut_ptr(), b"pcm%i%c\0".as_ptr() as *const c_char, (*pcm).device, if (*pstr).stream == SNDRV_PCM_STREAM_PLAYBACK { b'p' as c_int } else { b'c' as c_int });
    let mut entry = snd_info_create_card_entry((*pcm).card, name.as_ptr(), (*(*pcm).card).proc_root);
    if entry.is_null() { return -ENOMEM; }
    (*entry).mode = S_IFDIR | 0o555;
    (*pstr).proc_root = entry;
    entry = snd_info_create_card_entry((*pcm).card, b"info\0".as_ptr() as *const c_char, (*pstr).proc_root);
    if !entry.is_null() { snd_info_set_text_ops(entry, pstr as *mut c_void, Some(snd_pcm_stream_proc_info_read)); }
    entry = snd_info_create_card_entry((*pcm).card, b"xrun_debug\0".as_ptr() as *const c_char, (*pstr).proc_root);
    if !entry.is_null() {
        snd_info_set_text_ops(entry, pstr as *mut c_void, Some(snd_pcm_xrun_debug_read));
        (*entry).c.text.write = Some(snd_pcm_xrun_debug_write);
        (*entry).mode |= 0o200;
    }
    0
}

unsafe fn snd_pcm_stream_proc_done(pstr: *mut snd_pcm_str) -> c_int {
    snd_info_free_entry((*pstr).proc_root);
    (*pstr).proc_root = ptr::null_mut();
    0
}

unsafe fn create_substream_info_entry(substream: *mut snd_pcm_substream, name: *const c_char, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>) -> *mut snd_info_entry {
    let entry = snd_info_create_card_entry((*(*substream).pcm).card, name, (*substream).proc_root);
    if !entry.is_null() {
        snd_info_set_text_ops(entry, substream as *mut c_void, read);
    }
    entry
}

unsafe fn snd_pcm_substream_proc_init(substream: *mut snd_pcm_substream) -> c_int {
    let card = (*(*substream).pcm).card;
    let mut name = [0 as c_char; 16];
    sprintf(name.as_mut_ptr(), b"sub%i\0".as_ptr() as *const c_char, (*substream).number);
    let mut entry = snd_info_create_card_entry(card, name.as_ptr(), (*(*substream).pstr).proc_root);
    if entry.is_null() { return -ENOMEM; }
    (*entry).mode = S_IFDIR | 0o555;
    (*substream).proc_root = entry;
    create_substream_info_entry(substream, b"info\0".as_ptr() as *const c_char, Some(snd_pcm_substream_proc_info_read));
    create_substream_info_entry(substream, b"hw_params\0".as_ptr() as *const c_char, Some(snd_pcm_substream_proc_hw_params_read));
    create_substream_info_entry(substream, b"sw_params\0".as_ptr() as *const c_char, Some(snd_pcm_substream_proc_sw_params_read));
    create_substream_info_entry(substream, b"status\0".as_ptr() as *const c_char, Some(snd_pcm_substream_proc_status_read));
    entry = create_substream_info_entry(substream, b"xrun_injection\0".as_ptr() as *const c_char, None);
    if !entry.is_null() {
        (*entry).c.text.write = Some(snd_pcm_xrun_injection_write);
        (*entry).mode = S_IFREG | 0o200;
    }
    0
}

static mut pcm_dev_attr_groups: [*const attribute_group; 2] = [ptr::null(), ptr::null()];

/*
 * PM callbacks: we need to deal only with suspend here, as the resume is
 * triggered either from user-space or the driver's resume callback
 */
unsafe extern "C" fn do_pcm_suspend(dev: *mut device) -> c_int {
    let pstr = dev_get_drvdata(dev) as *mut snd_pcm_str;
    if !(*(*pstr).pcm).no_device_suspend {
        snd_pcm_suspend_all((*pstr).pcm);
    }
    0
}

static pcm_dev_pm_ops: dev_pm_ops = dev_pm_ops { suspend: Some(do_pcm_suspend) };
static pcm_dev_type: device_type = device_type { name: b"pcm\0".as_ptr() as *const c_char, pm: &pcm_dev_pm_ops };

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_new_stream(pcm: *mut snd_pcm, stream: c_int, substream_count: c_int) -> c_int {
    let pstr = &mut (*pcm).streams[stream as usize] as *mut snd_pcm_str;
    mutex_init(&mut (*pstr).oss.setup_mutex);
    (*pstr).stream = stream;
    (*pstr).pcm = pcm;
    (*pstr).substream_count = substream_count;
    if substream_count == 0 { return 0; }
    let mut err = snd_device_alloc(&mut (*pstr).dev, (*pcm).card);
    if err < 0 { return err; }
    dev_set_name((*pstr).dev, b"pcmC%iD%i%c\0".as_ptr() as *const c_char, (*(*pcm).card).number, (*pcm).device, if stream == SNDRV_PCM_STREAM_PLAYBACK { b'p' as c_int } else { b'c' as c_int });
    (*(*pstr).dev).groups = pcm_dev_attr_groups.as_ptr();
    (*(*pstr).dev).type_ = &pcm_dev_type;
    dev_set_drvdata((*pstr).dev, pstr as *mut c_void);
    if !(*pcm).internal {
        err = snd_pcm_stream_proc_init(pstr);
        if err < 0 { return err; }
    }
    let mut prev: *mut snd_pcm_substream = ptr::null_mut();
    let mut idx = 0;
    while idx < substream_count {
        let substream = kzalloc(size_of::<snd_pcm_substream>(), GFP_KERNEL) as *mut snd_pcm_substream;
        if substream.is_null() { return -ENOMEM; }
        (*substream).pcm = pcm;
        (*substream).pstr = pstr;
        (*substream).number = idx;
        (*substream).stream = stream;
        sprintf((*substream).name.as_mut_ptr(), b"subdevice #%i\0".as_ptr() as *const c_char, idx);
        (*substream).buffer_bytes_max = UINT_MAX;
        if prev.is_null() { (*pstr).substream = substream; } else { (*prev).next = substream; }
        if !(*pcm).internal {
            err = snd_pcm_substream_proc_init(substream);
            if err < 0 {
                if prev.is_null() { (*pstr).substream = ptr::null_mut(); } else { (*prev).next = ptr::null_mut(); }
                kfree(substream as *mut c_void);
                return err;
            }
        }
        (*substream).group = &mut (*substream).self_group;
        snd_pcm_group_init(&mut (*substream).self_group);
        list_add_tail(&mut (*substream).link_list, &mut (*substream).self_group.substreams);
        atomic_set(&mut (*substream).mmap_count, 0);
        prev = substream;
        idx += 1;
    }
    0
}

unsafe fn _snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, internal: bool_t, rpcm: *mut *mut snd_pcm) -> c_int {
    static ops: snd_device_ops = snd_device_ops { dev_free: Some(snd_pcm_dev_free), dev_register: Some(snd_pcm_dev_register), dev_disconnect: Some(snd_pcm_dev_disconnect) };
    static internal_ops: snd_device_ops = snd_device_ops { dev_free: Some(snd_pcm_dev_free), dev_register: None, dev_disconnect: None };
    if snd_BUG_ON(card.is_null()) { return -ENXIO; }
    if !rpcm.is_null() { *rpcm = ptr::null_mut(); }
    let pcm = kzalloc(size_of::<snd_pcm>(), GFP_KERNEL) as *mut snd_pcm;
    if pcm.is_null() { return -ENOMEM; }
    (*pcm).card = card;
    (*pcm).device = device;
    (*pcm).internal = internal;
    mutex_init(&mut (*pcm).open_mutex);
    init_waitqueue_head(&mut (*pcm).open_wait);
    INIT_LIST_HEAD(&mut (*pcm).list);
    if !id.is_null() { strscpy((*pcm).id.as_mut_ptr(), id, (*pcm).id.len()); }
    let mut err = snd_pcm_new_stream(pcm, SNDRV_PCM_STREAM_PLAYBACK, playback_count);
    if err < 0 { snd_pcm_free(pcm); return err; }
    err = snd_pcm_new_stream(pcm, SNDRV_PCM_STREAM_CAPTURE, capture_count);
    if err < 0 { snd_pcm_free(pcm); return err; }
    err = snd_device_new(card, SNDRV_DEV_PCM, pcm as *mut c_void, if internal { &internal_ops } else { &ops });
    if err < 0 { snd_pcm_free(pcm); return err; }
    if !rpcm.is_null() { *rpcm = pcm; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int {
    _snd_pcm_new(card, id, device, playback_count, capture_count, false, rpcm)
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_new_internal(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int {
    _snd_pcm_new(card, id, device, playback_count, capture_count, true, rpcm)
}

unsafe fn free_chmap(pstr: *mut snd_pcm_str) {
    if !(*pstr).chmap_kctl.is_null() {
        let card = (*(*pstr).pcm).card;
        snd_ctl_remove(card, (*pstr).chmap_kctl);
        (*pstr).chmap_kctl = ptr::null_mut();
    }
}

unsafe fn snd_pcm_free_stream(pstr: *mut snd_pcm_str) {
    snd_pcm_stream_proc_done(pstr);
    let mut substream = (*pstr).substream;
    while !substream.is_null() {
        let substream_next = (*substream).next;
        snd_pcm_timer_done(substream);
        kfree(substream as *mut c_void);
        substream = substream_next;
    }
    let mut setup = (*pstr).oss.setup_list;
    while !setup.is_null() {
        let setupn = (*setup).next;
        kfree((*setup).task_name as *mut c_void);
        kfree(setup as *mut c_void);
        setup = setupn;
    }
    free_chmap(pstr);
    if (*pstr).substream_count != 0 { put_device((*pstr).dev); }
}

unsafe fn pcm_call_notify(pcm: *mut snd_pcm, call: unsafe extern "C" fn(*mut snd_pcm_notify) -> Option<unsafe extern "C" fn(*mut snd_pcm)>) {
    let mut notify = snd_pcm_notify_list.next as *mut snd_pcm_notify;
    while !notify.is_null() && notify as *mut list_head != &raw mut snd_pcm_notify_list {
        if let Some(f) = call(notify) { f(pcm); }
        notify = (*notify).list.next as *mut snd_pcm_notify;
    }
}

unsafe fn notify_register(n: *mut snd_pcm_notify) -> Option<unsafe extern "C" fn(*mut snd_pcm)> { (*n).n_register }
unsafe fn notify_unregister(n: *mut snd_pcm_notify) -> Option<unsafe extern "C" fn(*mut snd_pcm)> { (*n).n_unregister }
unsafe fn notify_disconnect(n: *mut snd_pcm_notify) -> Option<unsafe extern "C" fn(*mut snd_pcm)> { (*n).n_disconnect }

unsafe fn snd_pcm_free(pcm: *mut snd_pcm) -> c_int {
    if pcm.is_null() { return 0; }
    if !(*pcm).internal { pcm_call_notify(pcm, notify_unregister); }
    if let Some(f) = (*pcm).private_free { f(pcm); }
    snd_pcm_lib_preallocate_free_for_all(pcm);
    snd_pcm_free_stream(&mut (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize]);
    snd_pcm_free_stream(&mut (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize]);
    kfree(pcm as *mut c_void);
    0
}

unsafe extern "C" fn snd_pcm_dev_free(device: *mut snd_device) -> c_int {
    snd_pcm_free((*device).device_data as *mut snd_pcm)
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_attach_substream(pcm: *mut snd_pcm, stream: c_int, file: *mut file, rsubstream: *mut *mut snd_pcm_substream) -> c_int {
    if snd_BUG_ON(pcm.is_null() || rsubstream.is_null()) { return -ENXIO; }
    if snd_BUG_ON(stream != SNDRV_PCM_STREAM_PLAYBACK && stream != SNDRV_PCM_STREAM_CAPTURE) { return -EINVAL; }
    *rsubstream = ptr::null_mut();
    let pstr = &mut (*pcm).streams[stream as usize] as *mut snd_pcm_str;
    if (*pstr).substream.is_null() || (*pstr).substream_count == 0 { return -ENODEV; }
    let card = (*pcm).card;
    let prefer_subdevice = snd_ctl_get_preferred_subdevice(card, SND_CTL_SUBDEV_PCM);
    if ((*pcm).info_flags & SNDRV_PCM_INFO_HALF_DUPLEX) != 0 {
        let opposite = if stream == 0 { 1 } else { 0 };
        let mut substream = (*pcm).streams[opposite].substream;
        while !substream.is_null() {
            if SUBSTREAM_BUSY(substream) { return -EAGAIN; }
            substream = (*substream).next;
        }
    }
    if ((*file).f_flags & O_APPEND) != 0 {
        let mut substream: *mut snd_pcm_substream;
        if prefer_subdevice < 0 {
            if (*pstr).substream_count > 1 { return -EINVAL; }
            substream = (*pstr).substream;
        } else {
            substream = (*pstr).substream;
            while !substream.is_null() {
                if (*substream).number == prefer_subdevice { break; }
                substream = (*substream).next;
            }
        }
        if substream.is_null() { return -ENODEV; }
        if !SUBSTREAM_BUSY(substream) { return -EBADFD; }
        (*substream).ref_count += 1;
        *rsubstream = substream;
        return 0;
    }
    let mut substream = (*pstr).substream;
    while !substream.is_null() {
        if !SUBSTREAM_BUSY(substream) && (prefer_subdevice == -1 || (*substream).number == prefer_subdevice) { break; }
        substream = (*substream).next;
    }
    if substream.is_null() { return -EAGAIN; }
    let runtime = kzalloc(size_of::<snd_pcm_runtime>(), GFP_KERNEL) as *mut snd_pcm_runtime;
    if runtime.is_null() { return -ENOMEM; }
    let mut size = PAGE_ALIGN(size_of::<snd_pcm_mmap_status>());
    (*runtime).status = alloc_pages_exact(size, GFP_KERNEL) as *mut snd_pcm_mmap_status;
    if (*runtime).status.is_null() { kfree(runtime as *mut c_void); return -ENOMEM; }
    memset((*runtime).status as *mut c_void, 0, size);
    size = PAGE_ALIGN(size_of::<snd_pcm_mmap_control>());
    (*runtime).control = alloc_pages_exact(size, GFP_KERNEL) as *mut snd_pcm_mmap_control;
    if (*runtime).control.is_null() {
        free_pages_exact((*runtime).status as *mut c_void, PAGE_ALIGN(size_of::<snd_pcm_mmap_status>()));
        kfree(runtime as *mut c_void);
        return -ENOMEM;
    }
    memset((*runtime).control as *mut c_void, 0, size);
    init_waitqueue_head(&mut (*runtime).sleep);
    init_waitqueue_head(&mut (*runtime).tsleep);
    __snd_pcm_set_state(runtime, SNDRV_PCM_STATE_OPEN);
    mutex_init(&mut (*runtime).buffer_mutex);
    atomic_set(&mut (*runtime).buffer_accessing, 0);
    (*substream).runtime = runtime;
    (*substream).private_data = (*pcm).private_data;
    (*substream).ref_count = 1;
    (*substream).f_flags = (*file).f_flags;
    (*substream).pid = get_pid(task_pid(current));
    (*pstr).substream_opened += 1;
    *rsubstream = substream;
    (*substream).xrun_counter = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_detach_substream(substream: *mut snd_pcm_substream) {
    if substream.is_null() || (*substream).runtime.is_null() { return; }
    let runtime = (*substream).runtime;
    if let Some(f) = (*runtime).private_free { f(runtime); }
    free_pages_exact((*runtime).status as *mut c_void, PAGE_ALIGN(size_of::<snd_pcm_mmap_status>()));
    free_pages_exact((*runtime).control as *mut c_void, PAGE_ALIGN(size_of::<snd_pcm_mmap_control>()));
    kfree((*runtime).hw_constraints.rules);
    (*substream).runtime = ptr::null_mut();
    mutex_destroy(&mut (*runtime).buffer_mutex);
    snd_fasync_free((*runtime).fasync);
    kfree(runtime as *mut c_void);
    put_pid((*substream).pid);
    (*substream).pid = ptr::null_mut();
    (*(*substream).pstr).substream_opened -= 1;
}

unsafe extern "C" fn pcm_class_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let pstr = dev_get_drvdata(dev) as *mut snd_pcm_str;
    let pcm = (*pstr).pcm;
    static strs: [*const c_char; SNDRV_PCM_CLASS_LAST + 1] = [
        b"generic\0".as_ptr() as *const c_char,
        b"multi\0".as_ptr() as *const c_char,
        b"modem\0".as_ptr() as *const c_char,
        b"digitizer\0".as_ptr() as *const c_char,
    ];
    let strp = if (*pcm).dev_class as usize > SNDRV_PCM_CLASS_LAST {
        b"none\0".as_ptr() as *const c_char
    } else {
        strs[(*pcm).dev_class as usize]
    };
    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, strp)
}

static mut dev_attr_pcm_class: device_attribute = device_attribute { attr: attribute { _private: [] } };
static mut pcm_dev_attrs: [*mut attribute; 2] = [ptr::null_mut(), ptr::null_mut()];
static mut pcm_dev_attr_group: attribute_group = attribute_group { attrs: ptr::null_mut() };

unsafe extern "C" fn snd_pcm_dev_register(device: *mut snd_device) -> c_int {
    if snd_BUG_ON(device.is_null() || (*device).device_data.is_null()) { return -ENXIO; }
    let pcm = (*device).device_data as *mut snd_pcm;
    mutex_lock(&raw mut register_mutex);
    let mut err = snd_pcm_add(pcm);
    if err != 0 {
        mutex_unlock(&raw mut register_mutex);
        return err;
    }
    let mut cidx = 0;
    while cidx < 2 {
        let mut devtype = -1;
        if (*pcm).streams[cidx].substream.is_null() { cidx += 1; continue; }
        match cidx as c_int {
            SNDRV_PCM_STREAM_PLAYBACK => devtype = SNDRV_DEVICE_TYPE_PCM_PLAYBACK,
            SNDRV_PCM_STREAM_CAPTURE => devtype = SNDRV_DEVICE_TYPE_PCM_CAPTURE,
            _ => {}
        }
        err = snd_register_device(devtype, (*pcm).card, (*pcm).device, (&mut snd_pcm_f_ops[cidx]) as *mut c_void, pcm as *mut c_void, (*pcm).streams[cidx].dev);
        if err < 0 {
            list_del_init(&mut (*pcm).list);
            mutex_unlock(&raw mut register_mutex);
            return err;
        }
        let mut substream = (*pcm).streams[cidx].substream;
        while !substream.is_null() {
            snd_pcm_timer_init(substream);
            substream = (*substream).next;
        }
        cidx += 1;
    }
    pcm_call_notify(pcm, notify_register);
    mutex_unlock(&raw mut register_mutex);
    err
}

unsafe extern "C" fn snd_pcm_dev_disconnect(device: *mut snd_device) -> c_int {
    let pcm = (*device).device_data as *mut snd_pcm;
    mutex_lock(&raw mut register_mutex);
    mutex_lock(&mut (*pcm).open_mutex);
    wake_up(&mut (*pcm).open_wait);
    list_del_init(&mut (*pcm).list);
    let mut cidx = 0;
    while cidx < 2 {
        let mut substream = (*pcm).streams[cidx].substream;
        while !substream.is_null() {
            snd_pcm_stream_lock_irq(substream);
            if !(*substream).runtime.is_null() {
                if snd_pcm_running(substream) { snd_pcm_stop(substream, SNDRV_PCM_STATE_DISCONNECTED); }
                __snd_pcm_set_state((*substream).runtime, SNDRV_PCM_STATE_DISCONNECTED);
                wake_up(&mut (*(*substream).runtime).sleep);
                wake_up(&mut (*(*substream).runtime).tsleep);
            }
            snd_pcm_stream_unlock_irq(substream);
            substream = (*substream).next;
        }
        cidx += 1;
    }
    cidx = 0;
    while cidx < 2 {
        let mut substream = (*pcm).streams[cidx].substream;
        while !substream.is_null() {
            snd_pcm_sync_stop(substream, false);
            substream = (*substream).next;
        }
        cidx += 1;
    }
    pcm_call_notify(pcm, notify_disconnect);
    cidx = 0;
    while cidx < 2 {
        if !(*pcm).streams[cidx].dev.is_null() { snd_unregister_device((*pcm).streams[cidx].dev); }
        free_chmap(&mut (*pcm).streams[cidx]);
        cidx += 1;
    }
    mutex_unlock(&mut (*pcm).open_mutex);
    mutex_unlock(&raw mut register_mutex);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_notify(notify: *mut snd_pcm_notify, nfree: c_int) -> c_int {
    if snd_BUG_ON(notify.is_null() || (*notify).n_register.is_none() || (*notify).n_unregister.is_none() || (*notify).n_disconnect.is_none()) {
        return -EINVAL;
    }
    mutex_lock(&raw mut register_mutex);
    if nfree != 0 {
        list_del(&mut (*notify).list);
        let mut pcm = snd_pcm_devices.next as *mut snd_pcm;
        while !pcm.is_null() && pcm as *mut list_head != &raw mut snd_pcm_devices {
            (*notify).n_unregister.unwrap()(pcm);
            pcm = (*pcm).list.next as *mut snd_pcm;
        }
    } else {
        list_add_tail(&mut (*notify).list, &raw mut snd_pcm_notify_list);
        let mut pcm = snd_pcm_devices.next as *mut snd_pcm;
        while !pcm.is_null() && pcm as *mut list_head != &raw mut snd_pcm_devices {
            (*notify).n_register.unwrap()(pcm);
            pcm = (*pcm).list.next as *mut snd_pcm;
        }
    }
    mutex_unlock(&raw mut register_mutex);
    0
}

/* CONFIG_SND_PROC_FS */
unsafe extern "C" fn snd_pcm_proc_read(_entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    mutex_lock(&raw mut register_mutex);
    let mut pcm = snd_pcm_devices.next as *mut snd_pcm;
    while !pcm.is_null() && pcm as *mut list_head != &raw mut snd_pcm_devices {
        snd_iprintf(buffer, b"%02i-%02i: %s : %s\0".as_ptr() as *const c_char, (*(*pcm).card).number, (*pcm).device, (*pcm).id.as_ptr(), (*pcm).name.as_ptr());
        if !(*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream.is_null() {
            snd_iprintf(buffer, b" : playback %i\0".as_ptr() as *const c_char, (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream_count);
        }
        if !(*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream.is_null() {
            snd_iprintf(buffer, b" : capture %i\0".as_ptr() as *const c_char, (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream_count);
        }
        snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
        pcm = (*pcm).list.next as *mut snd_pcm;
    }
    mutex_unlock(&raw mut register_mutex);
}

static mut snd_pcm_proc_entry: *mut snd_info_entry = ptr::null_mut();

unsafe fn snd_pcm_proc_init() {
    let mut entry = snd_info_create_module_entry(THIS_MODULE, b"pcm\0".as_ptr() as *const c_char, ptr::null_mut());
    if !entry.is_null() {
        snd_info_set_text_ops(entry, ptr::null_mut(), Some(snd_pcm_proc_read));
        if snd_info_register(entry) < 0 {
            snd_info_free_entry(entry);
            entry = ptr::null_mut();
        }
    }
    snd_pcm_proc_entry = entry;
}

unsafe fn snd_pcm_proc_done() {
    snd_info_free_entry(snd_pcm_proc_entry);
}

/*
 *  ENTRY functions
 */
unsafe extern "C" fn alsa_pcm_init() -> c_int {
    snd_ctl_register_ioctl(snd_pcm_control_ioctl);
    snd_ctl_register_ioctl_compat(snd_pcm_control_ioctl);
    snd_pcm_proc_init();
    0
}

unsafe extern "C" fn alsa_pcm_exit() {
    snd_ctl_unregister_ioctl(snd_pcm_control_ioctl);
    snd_ctl_unregister_ioctl_compat(snd_pcm_control_ioctl);
    snd_pcm_proc_done();
}

/* module_init(alsa_pcm_init) */
/* module_exit(alsa_pcm_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
