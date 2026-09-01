// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Abstract layer for MIDI v1.0 stream
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_t = bool;
type s32 = i32;
type s64 = i64;
type u8 = u8;
type u32 = u32;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type __poll_t = c_uint;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_mode: c_uint,
    pub f_flags: c_uint,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
    pub module: *mut module,
    pub shutdown: c_int,
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub card: *mut snd_card,
    pub device: c_int,
    pub info_flags: c_uint,
    pub streams: [snd_rawmidi_str; 2],
    pub list: list_head,
    pub open_mutex: mutex,
    pub open_wait: wait_queue_head_t,
    pub id: [c_char; 64],
    pub name: [c_char; 80],
    pub tied_device: c_int,
    pub dev: *mut device,
    pub ops: *const snd_rawmidi_global_ops,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
    pub proc_entry: *mut snd_info_entry,
    pub seq_dev: *mut snd_seq_device,
    pub ossreg: c_int,
}

#[repr(C)]
pub struct snd_rawmidi_str {
    pub substreams: list_head,
    pub substream_count: c_uint,
    pub substream_opened: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub list: list_head,
    pub stream: c_int,
    pub number: c_int,
    pub rmidi: *mut snd_rawmidi,
    pub pstr: *mut snd_rawmidi_str,
    pub lock: spinlock_t,
    pub runtime: *mut snd_rawmidi_runtime,
    pub ops: *const snd_rawmidi_ops,
    pub opened: c_int,
    pub append: c_int,
    pub use_count: c_int,
    pub active_sensing: c_int,
    pub pid: *mut pid,
    pub bytes: c_ulong,
    pub inactive: c_int,
    pub name: [c_char; 32],
    pub framing: c_uint,
    pub clock_type: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_runtime {
    pub substream: *mut snd_rawmidi_substream,
    pub sleep: wait_queue_head_t,
    pub event_work: work_struct,
    pub event: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
    pub buffer_size: size_t,
    pub avail_min: size_t,
    pub avail: size_t,
    pub buffer: *mut u8,
    pub appl_ptr: size_t,
    pub hw_ptr: size_t,
    pub buffer_ref: c_int,
    pub drain: c_int,
    pub xruns: size_t,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
    pub oss: c_int,
    pub align: c_int,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
    pub drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct snd_rawmidi_global_ops {
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_rawmidi, c_uint, *mut c_void) -> c_long>,
    pub proc_read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_rawmidi) -> c_int>,
    pub dev_unregister: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
}

#[repr(C)]
pub struct snd_rawmidi_file {
    pub rmidi: *mut snd_rawmidi,
    pub input: *mut snd_rawmidi_substream,
    pub output: *mut snd_rawmidi_substream,
    pub user_pversion: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_info {
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_uint,
    pub stream: c_int,
    pub flags: c_uint,
    pub id: [c_char; 64],
    pub name: [c_char; 80],
    pub subname: [c_char; 32],
    pub subdevices_count: c_uint,
    pub subdevices_avail: c_uint,
    pub tied_device: c_int,
}

#[repr(C)]
pub struct snd_rawmidi_params {
    pub stream: c_int,
    pub buffer_size: size_t,
    pub avail_min: size_t,
    pub no_active_sensing: c_int,
    pub mode: c_uint,
    pub reserved: [u8; 16],
}

#[repr(C)]
pub struct snd_rawmidi_framing_tstamp {
    pub tv_sec: s64,
    pub tv_nsec: s64,
    pub length: c_uint,
    pub data: [u8; SNDRV_RAWMIDI_FRAMING_DATA_LENGTH as usize],
}

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: s64,
    pub tv_nsec: s64,
}

#[repr(C)]
pub struct snd_ctl_file {
    pub preferred_subdevice: [c_int; 8],
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_info_entry_c {
    pub text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_device {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_seq_device)>,
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_ump_endpoint_info {
    pub device: c_int,
}

#[repr(C)]
pub struct pid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
}

#[repr(C)]
pub struct snd_rawmidi_status32 {
    pub stream: s32,
    pub tstamp_sec: s32,
    pub tstamp_nsec: s32,
    pub avail: u32,
    pub xruns: u32,
    pub reserved: [u8; 16],
}

#[repr(C)]
pub struct snd_rawmidi_status64 {
    pub stream: c_int,
    pub rsvd: [u8; 4],
    pub tstamp_sec: s64,
    pub tstamp_nsec: s64,
    pub avail: size_t,
    pub xruns: size_t,
    pub reserved: [u8; 16],
}

const SNDRV_CARDS: usize = 32;
const SNDRV_RAWMIDI_DEVICES: c_int = 32;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
const SNDRV_RAWMIDI_LFLG_OUTPUT: c_ushort = 1;
const SNDRV_RAWMIDI_LFLG_INPUT: c_ushort = 2;
const SNDRV_RAWMIDI_LFLG_OPEN: c_ushort = 3;
const SNDRV_RAWMIDI_LFLG_APPEND: c_int = 4;
type c_ushort = u16;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 1;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 2;
const SNDRV_RAWMIDI_INFO_UMP: c_uint = 4;
const SNDRV_RAWMIDI_INFO_STREAM_INACTIVE: c_uint = 8;
const SNDRV_RAWMIDI_MODE_FRAMING_MASK: c_uint = 0x0f;
const SNDRV_RAWMIDI_MODE_FRAMING_NONE: c_uint = 0;
const SNDRV_RAWMIDI_MODE_FRAMING_TSTAMP: c_uint = 1;
const SNDRV_RAWMIDI_MODE_CLOCK_MASK: c_uint = 0xf0;
const SNDRV_RAWMIDI_MODE_CLOCK_NONE: c_uint = 0;
const SNDRV_RAWMIDI_MODE_CLOCK_REALTIME: c_uint = 0x10;
const SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC: c_uint = 0x20;
const SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC_RAW: c_uint = 0x30;
const SNDRV_RAWMIDI_MODE_CLOCK_SHIFT: c_uint = 4;
const SNDRV_RAWMIDI_FRAMING_DATA_LENGTH: c_uint = 16;
const SNDRV_RAWMIDI_VERSION: c_int = 0x0002_0000;
const SNDRV_RAWMIDI_IOCTL_PVERSION: c_uint = 0;
const SNDRV_RAWMIDI_IOCTL_INFO: c_uint = 1;
const SNDRV_RAWMIDI_IOCTL_USER_PVERSION: c_uint = 2;
const SNDRV_RAWMIDI_IOCTL_PARAMS: c_uint = 3;
const SNDRV_RAWMIDI_IOCTL_DROP: c_uint = 4;
const SNDRV_RAWMIDI_IOCTL_DRAIN: c_uint = 5;
const SNDRV_RAWMIDI_IOCTL_STATUS32: c_uint = 0x5720;
const SNDRV_RAWMIDI_IOCTL_STATUS64: c_uint = 0x5720;
const SNDRV_CTL_IOCTL_RAWMIDI_NEXT_DEVICE: c_uint = 0x100;
const SNDRV_CTL_IOCTL_UMP_NEXT_DEVICE: c_uint = 0x101;
const SNDRV_CTL_IOCTL_UMP_ENDPOINT_INFO: c_uint = 0x102;
const SNDRV_CTL_IOCTL_UMP_BLOCK_INFO: c_uint = 0x103;
const SNDRV_CTL_IOCTL_RAWMIDI_PREFER_SUBDEVICE: c_uint = 0x104;
const SNDRV_CTL_IOCTL_RAWMIDI_INFO: c_uint = 0x105;
const SNDRV_UMP_IOCTL_ENDPOINT_INFO: c_uint = 0x200;
const SNDRV_UMP_IOCTL_BLOCK_INFO: c_uint = 0x201;
const SND_CTL_SUBDEV_RAWMIDI: usize = 0;
const SNDRV_DEVICE_TYPE_RAWMIDI: c_int = 0;
const SNDRV_DEV_RAWMIDI: c_int = 0;
const SNDRV_OSS_DEVICE_TYPE_MIDI: c_int = 0;
const SNDRV_SEQ_DEV_ID_MIDISYNTH: c_int = 0;
const FMODE_READ: c_uint = 1;
const FMODE_WRITE: c_uint = 2;
const O_APPEND: c_uint = 0x0008;
const O_NONBLOCK: c_uint = 0x0800;
const O_DSYNC: c_uint = 0x1000;
const SOUND_MAJOR: c_int = 14;
const HZ: c_long = 100;
const PAGE_SIZE: size_t = 4096;
const GFP_KERNEL: c_uint = 0;
const TASK_INTERRUPTIBLE: c_long = 1;
const EPOLLIN: __poll_t = 0x001;
const EPOLLRDNORM: __poll_t = 0x040;
const EPOLLOUT: __poll_t = 0x004;
const EPOLLWRNORM: __poll_t = 0x100;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const ERESTARTSYS: c_int = 512;
const EIO: c_int = 5;
const EBADFD: c_int = 77;
const EFAULT: c_int = 14;
const ENOTTY: c_int = 25;
const ENOENT: c_int = 2;
const ENOIOCTLCMD: c_int = 515;

unsafe extern "C" {
    static mut snd_major: c_int;
    static mut THIS_MODULE: *mut module;
    static mut current: *mut task_struct;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kvzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kvfree(ptr: *mut c_void);
    fn init_waitqueue_head(q: *mut wait_queue_head_t);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn init_waitqueue_entry(wait: *mut wait_queue_entry_t, task: *mut task_struct);
    fn add_wait_queue(q: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn remove_wait_queue(q: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn wake_up(q: *mut wait_queue_head_t);
    fn schedule();
    fn schedule_timeout(timeout: c_long) -> c_long;
    fn schedule_timeout_uninterruptible(timeout: c_long);
    fn wait_event_interruptible_timeout(q: wait_queue_head_t, condition: bool_t, timeout: c_long) -> c_long;
    fn set_current_state(state: c_long);
    fn signal_pending(task: *mut task_struct) -> c_int;
    fn msleep(msecs: c_uint);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_lock_nested(lock: *mut mutex, depth: c_int);
    fn mutex_unlock(lock: *mut mutex);
    fn try_module_get(module: *mut module) -> c_int;
    fn module_put(module: *mut module);
    fn get_pid(pid: *mut pid) -> *mut pid;
    fn put_pid(pid: *mut pid);
    fn task_pid(task: *mut task_struct) -> *mut pid;
    fn pid_vnr(pid: *mut pid) -> c_int;
    fn imajor(inode: *mut inode) -> c_int;
    fn iminor(inode: *mut inode) -> c_int;
    fn stream_open(inode: *mut inode, file: *mut file);
    fn snd_lookup_minor_data(minor: c_int, typ: c_int) -> *mut snd_rawmidi;
    fn snd_lookup_oss_minor_data(minor: c_int, typ: c_int) -> *mut snd_rawmidi;
    fn snd_card_unref(card: *mut snd_card);
    fn snd_card_file_add(card: *mut snd_card, file: *mut file) -> c_int;
    fn snd_card_file_remove(card: *mut snd_card, file: *mut file);
    fn snd_ctl_get_preferred_subdevice(card: *mut snd_card, typ: usize) -> c_int;
    fn snd_BUG_ON(condition: bool_t) -> c_int;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn get_user_int(src: *const c_int, dst: *mut c_int) -> c_int;
    fn put_user_int(val: c_int, dst: *mut c_int) -> c_int;
    fn get_user_uint(src: *const c_uint, dst: *mut c_uint) -> c_int;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> isize;
    fn array_index_nospec(index: c_int, size: c_int) -> c_int;
    fn ktime_get_raw_ts64(ts: *mut timespec64);
    fn ktime_get_ts64(ts: *mut timespec64);
    fn ktime_get_real_ts64(ts: *mut timespec64);
    fn schedule_work(work: *mut work_struct);
    fn cancel_work_sync(work: *mut work_struct);
    fn poll_wait(file: *mut file, q: *mut wait_queue_head_t, wait: *mut poll_table);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_device_alloc(dev: *mut *mut device, card: *mut snd_card) -> c_int;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn snd_device_new(card: *mut snd_card, typ: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_register_device(typ: c_int, card: *mut snd_card, dev: c_int, fops: *const file_operations, data: *mut c_void, device: *mut device) -> c_int;
    fn snd_unregister_device(dev: *mut device);
    fn snd_register_oss_device(typ: c_int, card: *mut snd_card, dev: c_int, fops: *const file_operations, data: *mut c_void) -> c_int;
    fn snd_unregister_oss_device(typ: c_int, card: *mut snd_card, dev: c_int);
    fn snd_info_create_card_entry(card: *mut snd_card, name: *const c_char, root: *mut snd_info_entry) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_seq_device_new(card: *mut snd_card, device: c_int, id: c_int, arg: c_int, rdev: *mut *mut snd_seq_device) -> c_int;
    fn snd_device_register(card: *mut snd_card, dev: *mut snd_seq_device) -> c_int;
    fn put_device(dev: *mut device);
    fn snd_ctl_register_ioctl(func: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
    fn snd_ctl_register_ioctl_compat(func: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
    fn snd_ctl_unregister_ioctl(func: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
    fn snd_ctl_unregister_ioctl_compat(func: unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int);
    fn pr_err(fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}

static mut snd_rawmidi_devices: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
static mut register_mutex: mutex = mutex { _private: [] };

/* CONFIG_SND_OSSEMUL */
static mut midi_map: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut amidi_map: [c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];

#[inline]
unsafe fn rmidi_err(_rmidi: *mut snd_rawmidi, _fmt: *const c_char) {}
#[inline]
unsafe fn rmidi_warn(_rmidi: *mut snd_rawmidi, _fmt: *const c_char) {}
#[inline]
unsafe fn rmidi_dbg(_rmidi: *mut snd_rawmidi, _fmt: *const c_char) {}

#[inline]
unsafe fn rawmidi_is_ump(rmidi: *mut snd_rawmidi) -> bool {
    ((*rmidi).info_flags & SNDRV_RAWMIDI_INFO_UMP) != 0
}

unsafe fn list_first_entry<T>(head: *mut list_head, field_offset: isize) -> *mut T {
    ((*head).next as *mut u8).offset(-field_offset) as *mut T
}

const fn offset_of_substream_list() -> isize {
    0
}

const fn offset_of_rawmidi_list() -> isize {
    0
}

unsafe fn snd_rawmidi_search(card: *mut snd_card, device: c_int) -> *mut snd_rawmidi {
    let mut pos = snd_rawmidi_devices.next;
    while !pos.is_null() && pos != &raw mut snd_rawmidi_devices {
        let rawmidi = (pos as *mut u8).offset(-offset_of_rawmidi_list()) as *mut snd_rawmidi;
        if (*rawmidi).card == card && (*rawmidi).device == device {
            return rawmidi;
        }
        pos = (*pos).next;
    }
    ptr::null_mut()
}

#[inline]
unsafe fn snd_rawmidi_file_flags(file: *mut file) -> c_ushort {
    match (*file).f_mode & (FMODE_READ | FMODE_WRITE) {
        FMODE_WRITE => SNDRV_RAWMIDI_LFLG_OUTPUT,
        FMODE_READ => SNDRV_RAWMIDI_LFLG_INPUT,
        _ => SNDRV_RAWMIDI_LFLG_OPEN,
    }
}

#[inline]
unsafe fn __snd_rawmidi_ready(runtime: *mut snd_rawmidi_runtime) -> bool {
    (*runtime).avail >= (*runtime).avail_min
}

unsafe fn snd_rawmidi_ready(substream: *mut snd_rawmidi_substream) -> bool {
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    let ret = __snd_rawmidi_ready((*substream).runtime);
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
    ret
}

#[inline]
unsafe fn snd_rawmidi_ready_append(substream: *mut snd_rawmidi_substream, count: size_t) -> c_int {
    let runtime = (*substream).runtime;
    (((*runtime).avail >= (*runtime).avail_min) && ((*substream).append == 0 || (*runtime).avail >= count)) as c_int
}

unsafe extern "C" fn snd_rawmidi_input_event_work(work: *mut work_struct) {
    let runtime = (work as *mut u8).offset(-(0isize)) as *mut snd_rawmidi_runtime;
    if (*runtime).event.is_some() {
        (*runtime).event.unwrap()((*runtime).substream);
    }
}

#[inline]
unsafe fn snd_rawmidi_buffer_ref(runtime: *mut snd_rawmidi_runtime) {
    (*runtime).buffer_ref += 1;
}

#[inline]
unsafe fn snd_rawmidi_buffer_unref(runtime: *mut snd_rawmidi_runtime) {
    (*runtime).buffer_ref -= 1;
}

unsafe fn snd_rawmidi_buffer_ref_sync(substream: *mut snd_rawmidi_substream) {
    let mut loop_count = HZ;
    spin_lock_irq(&mut (*substream).lock);
    while (*(*substream).runtime).buffer_ref != 0 {
        spin_unlock_irq(&mut (*substream).lock);
        loop_count -= 1;
        if loop_count == 0 {
            rmidi_err((*substream).rmidi, b"Buffer ref sync timeout\n\0".as_ptr() as *const c_char);
            return;
        }
        schedule_timeout_uninterruptible(1);
        spin_lock_irq(&mut (*substream).lock);
    }
    spin_unlock_irq(&mut (*substream).lock);
}

unsafe fn snd_rawmidi_runtime_create(substream: *mut snd_rawmidi_substream) -> c_int {
    let runtime = kzalloc(size_of::<snd_rawmidi_runtime>(), GFP_KERNEL) as *mut snd_rawmidi_runtime;
    if runtime.is_null() {
        return -ENOMEM;
    }
    (*runtime).substream = substream;
    init_waitqueue_head(&mut (*runtime).sleep);
    INIT_WORK(&mut (*runtime).event_work, snd_rawmidi_input_event_work);
    (*runtime).event = None;
    (*runtime).buffer_size = PAGE_SIZE;
    (*runtime).avail_min = 1;
    if (*substream).stream == SNDRV_RAWMIDI_STREAM_INPUT {
        (*runtime).avail = 0;
    } else {
        (*runtime).avail = (*runtime).buffer_size;
    }
    (*runtime).buffer = kvzalloc((*runtime).buffer_size, GFP_KERNEL) as *mut u8;
    if (*runtime).buffer.is_null() {
        kfree(runtime as *mut c_void);
        return -ENOMEM;
    }
    (*runtime).appl_ptr = 0;
    (*runtime).hw_ptr = 0;
    (*substream).runtime = runtime;
    if rawmidi_is_ump((*substream).rmidi) {
        (*runtime).align = 3;
    }
    0
}

#[inline]
unsafe fn get_align(runtime: *mut snd_rawmidi_runtime) -> c_int {
    (*runtime).align
}

#[inline]
unsafe fn get_aligned_size(runtime: *mut snd_rawmidi_runtime, size: c_int) -> c_int {
    size & !get_align(runtime)
}

unsafe fn snd_rawmidi_runtime_free(substream: *mut snd_rawmidi_substream) -> c_int {
    let runtime = (*substream).runtime;
    kvfree((*runtime).buffer as *mut c_void);
    kfree(runtime as *mut c_void);
    (*substream).runtime = ptr::null_mut();
    0
}

#[inline]
unsafe fn snd_rawmidi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    if (*substream).opened == 0 {
        return;
    }
    if let Some(trigger) = (*(*substream).ops).trigger {
        trigger(substream, up);
    }
}

unsafe fn snd_rawmidi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    if (*substream).opened == 0 {
        return;
    }
    if let Some(trigger) = (*(*substream).ops).trigger {
        trigger(substream, up);
    }
    if up == 0 {
        cancel_work_sync(&mut (*(*substream).runtime).event_work);
    }
}

unsafe fn __reset_runtime_ptrs(runtime: *mut snd_rawmidi_runtime, is_input: bool) {
    (*runtime).drain = 0;
    (*runtime).appl_ptr = 0;
    (*runtime).hw_ptr = 0;
    (*runtime).avail = if is_input { 0 } else { (*runtime).buffer_size };
}

unsafe fn reset_runtime_ptrs(substream: *mut snd_rawmidi_substream, is_input: bool) {
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    if (*substream).opened != 0 && !(*substream).runtime.is_null() {
        __reset_runtime_ptrs((*substream).runtime, is_input);
    }
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_drop_output(substream: *mut snd_rawmidi_substream) -> c_int {
    snd_rawmidi_output_trigger(substream, 0);
    reset_runtime_ptrs(substream, false);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_drain_output(substream: *mut snd_rawmidi_substream) -> c_int {
    let mut err = 0;
    let timeout: c_long;
    let runtime: *mut snd_rawmidi_runtime;
    spin_lock_irq(&mut (*substream).lock);
    runtime = (*substream).runtime;
    if (*substream).opened == 0 || runtime.is_null() || (*runtime).buffer.is_null() {
        spin_unlock_irq(&mut (*substream).lock);
        return -EINVAL;
    }
    snd_rawmidi_buffer_ref(runtime);
    (*runtime).drain = 1;
    spin_unlock_irq(&mut (*substream).lock);
    timeout = wait_event_interruptible_timeout((*runtime).sleep, (*runtime).avail >= (*runtime).buffer_size, 10 * HZ);
    spin_lock_irq(&mut (*substream).lock);
    if signal_pending(current) != 0 {
        err = -ERESTARTSYS;
    }
    if (*runtime).avail < (*runtime).buffer_size && timeout == 0 {
        rmidi_warn((*substream).rmidi, b"rawmidi drain error\n\0".as_ptr() as *const c_char);
        err = -EIO;
    }
    (*runtime).drain = 0;
    spin_unlock_irq(&mut (*substream).lock);
    if err != -ERESTARTSYS {
        if let Some(drain) = (*(*substream).ops).drain {
            drain(substream);
        } else {
            msleep(50);
        }
        snd_rawmidi_drop_output(substream);
    }
    spin_lock_irq(&mut (*substream).lock);
    snd_rawmidi_buffer_unref(runtime);
    spin_unlock_irq(&mut (*substream).lock);
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_drain_input(substream: *mut snd_rawmidi_substream) -> c_int {
    snd_rawmidi_input_trigger(substream, 0);
    reset_runtime_ptrs(substream, true);
    0
}

unsafe fn assign_substream(
    rmidi: *mut snd_rawmidi,
    subdevice: c_int,
    stream: c_int,
    mode: c_int,
    sub_ret: *mut *mut snd_rawmidi_substream,
) -> c_int {
    static info_flags: [c_uint; 2] = [SNDRV_RAWMIDI_INFO_OUTPUT, SNDRV_RAWMIDI_INFO_INPUT];
    let s = &mut (*rmidi).streams[stream as usize] as *mut snd_rawmidi_str;
    if ((*rmidi).info_flags & info_flags[stream as usize]) == 0 {
        return -ENXIO;
    }
    if subdevice >= 0 && subdevice >= (*s).substream_count as c_int {
        return -ENODEV;
    }
    let mut pos = (*s).substreams.next;
    while !pos.is_null() && pos != &mut (*s).substreams {
        let substream = (pos as *mut u8).offset(-offset_of_substream_list()) as *mut snd_rawmidi_substream;
        if (*substream).opened != 0 {
            if stream == SNDRV_RAWMIDI_STREAM_INPUT || (mode & SNDRV_RAWMIDI_LFLG_APPEND) == 0 || (*substream).append == 0 {
                pos = (*pos).next;
                continue;
            }
        }
        if subdevice < 0 || subdevice == (*substream).number {
            *sub_ret = substream;
            return 0;
        }
        pos = (*pos).next;
    }
    -EAGAIN
}

unsafe fn open_substream(rmidi: *mut snd_rawmidi, substream: *mut snd_rawmidi_substream, mode: c_int) -> c_int {
    let mut err: c_int;
    if (*substream).use_count == 0 {
        err = snd_rawmidi_runtime_create(substream);
        if err < 0 {
            return err;
        }
        if let Some(open) = (*(*substream).ops).open {
            err = open(substream);
        } else {
            err = 0;
        }
        if err < 0 {
            snd_rawmidi_runtime_free(substream);
            return err;
        }
        spin_lock_irq(&mut (*substream).lock);
        (*substream).opened = 1;
        (*substream).active_sensing = 0;
        if (mode & SNDRV_RAWMIDI_LFLG_APPEND) != 0 {
            (*substream).append = 1;
        }
        (*substream).pid = get_pid(task_pid(current));
        (*rmidi).streams[(*substream).stream as usize].substream_opened += 1;
        spin_unlock_irq(&mut (*substream).lock);
    }
    (*substream).use_count += 1;
    0
}

unsafe fn rawmidi_open_priv(rmidi: *mut snd_rawmidi, subdevice: c_int, mode: c_int, rfile: *mut snd_rawmidi_file) -> c_int {
    let mut sinput: *mut snd_rawmidi_substream = ptr::null_mut();
    let mut soutput: *mut snd_rawmidi_substream = ptr::null_mut();
    let mut err: c_int;
    (*rfile).input = ptr::null_mut();
    (*rfile).output = ptr::null_mut();
    if (mode & SNDRV_RAWMIDI_LFLG_INPUT as c_int) != 0 {
        err = assign_substream(rmidi, subdevice, SNDRV_RAWMIDI_STREAM_INPUT, mode, &mut sinput);
        if err < 0 {
            return err;
        }
    }
    if (mode & SNDRV_RAWMIDI_LFLG_OUTPUT as c_int) != 0 {
        err = assign_substream(rmidi, subdevice, SNDRV_RAWMIDI_STREAM_OUTPUT, mode, &mut soutput);
        if err < 0 {
            return err;
        }
    }
    if !sinput.is_null() {
        err = open_substream(rmidi, sinput, mode);
        if err < 0 {
            return err;
        }
    }
    if !soutput.is_null() {
        err = open_substream(rmidi, soutput, mode);
        if err < 0 {
            if !sinput.is_null() {
                close_substream(rmidi, sinput, 0);
            }
            return err;
        }
    }
    (*rfile).rmidi = rmidi;
    (*rfile).input = sinput;
    (*rfile).output = soutput;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_kernel_open_nested(
    rmidi: *mut snd_rawmidi,
    subdevice: c_int,
    mode: c_int,
    rfile: *mut snd_rawmidi_file,
    depth: c_int,
) -> c_int {
    if snd_BUG_ON(rfile.is_null()) != 0 {
        return -EINVAL;
    }
    if try_module_get((*(*rmidi).card).module) == 0 {
        return -ENXIO;
    }
    mutex_lock_nested(&mut (*rmidi).open_mutex, depth);
    let err = rawmidi_open_priv(rmidi, subdevice, mode, rfile);
    if err < 0 {
        module_put((*(*rmidi).card).module);
    }
    mutex_unlock(&mut (*rmidi).open_mutex);
    err
}

unsafe extern "C" fn snd_rawmidi_open(inode: *mut inode, file: *mut file) -> c_int {
    let maj = imajor(inode);
    let mut card: *mut snd_card;
    let mut subdevice: c_int;
    let mut fflags: c_ushort;
    let mut err: c_int;
    let rmidi: *mut snd_rawmidi;
    let mut rawmidi_file: *mut snd_rawmidi_file = ptr::null_mut();
    let mut wait: wait_queue_entry_t = zeroed();

    if ((*file).f_flags & O_APPEND) != 0 && ((*file).f_flags & O_NONBLOCK) == 0 {
        return -EINVAL;
    }
    stream_open(inode, file);
    if maj == snd_major {
        rmidi = snd_lookup_minor_data(iminor(inode), SNDRV_DEVICE_TYPE_RAWMIDI);
    } else if maj == SOUND_MAJOR {
        rmidi = snd_lookup_oss_minor_data(iminor(inode), SNDRV_OSS_DEVICE_TYPE_MIDI);
    } else {
        return -ENXIO;
    }
    if rmidi.is_null() {
        return -ENODEV;
    }
    if try_module_get((*(*rmidi).card).module) == 0 {
        snd_card_unref((*rmidi).card);
        return -ENXIO;
    }
    mutex_lock(&mut (*rmidi).open_mutex);
    card = (*rmidi).card;
    err = snd_card_file_add(card, file);
    if err < 0 {
        mutex_unlock(&mut (*rmidi).open_mutex);
        module_put((*(*rmidi).card).module);
        snd_card_unref((*rmidi).card);
        return err;
    }
    fflags = snd_rawmidi_file_flags(file);
    if ((*file).f_flags & O_APPEND) != 0 || maj == SOUND_MAJOR {
        fflags |= SNDRV_RAWMIDI_LFLG_APPEND as c_ushort;
    }
    rawmidi_file = kmalloc(size_of::<snd_rawmidi_file>(), GFP_KERNEL) as *mut snd_rawmidi_file;
    if rawmidi_file.is_null() {
        snd_card_file_remove(card, file);
        mutex_unlock(&mut (*rmidi).open_mutex);
        module_put((*(*rmidi).card).module);
        snd_card_unref((*rmidi).card);
        return -ENOMEM;
    }
    (*rawmidi_file).user_pversion = 0;
    init_waitqueue_entry(&mut wait, current);
    add_wait_queue(&mut (*rmidi).open_wait, &mut wait);
    loop {
        subdevice = snd_ctl_get_preferred_subdevice(card, SND_CTL_SUBDEV_RAWMIDI);
        err = rawmidi_open_priv(rmidi, subdevice, fflags as c_int, rawmidi_file);
        if err >= 0 {
            break;
        }
        if err == -EAGAIN {
            if ((*file).f_flags & O_NONBLOCK) != 0 {
                err = -EBUSY;
                break;
            }
        } else {
            break;
        }
        set_current_state(TASK_INTERRUPTIBLE);
        mutex_unlock(&mut (*rmidi).open_mutex);
        schedule();
        mutex_lock(&mut (*rmidi).open_mutex);
        if (*(*rmidi).card).shutdown != 0 {
            err = -ENODEV;
            break;
        }
        if signal_pending(current) != 0 {
            err = -ERESTARTSYS;
            break;
        }
    }
    remove_wait_queue(&mut (*rmidi).open_wait, &mut wait);
    if err < 0 {
        kfree(rawmidi_file as *mut c_void);
        snd_card_file_remove(card, file);
        mutex_unlock(&mut (*rmidi).open_mutex);
        module_put((*(*rmidi).card).module);
        snd_card_unref((*rmidi).card);
        return err;
    }
    /* CONFIG_SND_OSSEMUL: runtime->oss is set when opened through SOUND_MAJOR. */
    if !(*rawmidi_file).input.is_null() && !(*(*rawmidi_file).input).runtime.is_null() {
        (*(*(*rawmidi_file).input).runtime).oss = (maj == SOUND_MAJOR) as c_int;
    }
    if !(*rawmidi_file).output.is_null() && !(*(*rawmidi_file).output).runtime.is_null() {
        (*(*(*rawmidi_file).output).runtime).oss = (maj == SOUND_MAJOR) as c_int;
    }
    (*file).private_data = rawmidi_file as *mut c_void;
    mutex_unlock(&mut (*rmidi).open_mutex);
    snd_card_unref((*rmidi).card);
    0
}

unsafe fn close_substream_impl(rmidi: *mut snd_rawmidi, substream: *mut snd_rawmidi_substream, cleanup: c_int) {
    (*substream).use_count -= 1;
    if (*substream).use_count != 0 {
        return;
    }
    if cleanup != 0 {
        if (*substream).stream == SNDRV_RAWMIDI_STREAM_INPUT {
            snd_rawmidi_input_trigger(substream, 0);
        } else {
            if (*substream).active_sensing != 0 {
                let mut buf: u8 = 0xfe;
                snd_rawmidi_kernel_write(substream, &mut buf, 1);
            }
            if snd_rawmidi_drain_output(substream) == -ERESTARTSYS {
                snd_rawmidi_output_trigger(substream, 0);
            }
        }
        snd_rawmidi_buffer_ref_sync(substream);
    }
    spin_lock_irq(&mut (*substream).lock);
    (*substream).opened = 0;
    (*substream).append = 0;
    spin_unlock_irq(&mut (*substream).lock);
    if let Some(close) = (*(*substream).ops).close {
        close(substream);
    }
    if let Some(private_free) = (*(*substream).runtime).private_free {
        private_free(substream);
    }
    snd_rawmidi_runtime_free(substream);
    put_pid((*substream).pid);
    (*substream).pid = ptr::null_mut();
    (*rmidi).streams[(*substream).stream as usize].substream_opened -= 1;
}

unsafe fn close_substream(rmidi: *mut snd_rawmidi, substream: *mut snd_rawmidi_substream, cleanup: c_int) {
    close_substream_impl(rmidi, substream, cleanup)
}

unsafe fn rawmidi_release_priv(rfile: *mut snd_rawmidi_file) {
    let rmidi = (*rfile).rmidi;
    if !(*rfile).input.is_null() {
        close_substream(rmidi, (*rfile).input, 1);
        (*rfile).input = ptr::null_mut();
    }
    if !(*rfile).output.is_null() {
        close_substream(rmidi, (*rfile).output, 1);
        (*rfile).output = ptr::null_mut();
    }
    (*rfile).rmidi = ptr::null_mut();
    wake_up(&mut (*rmidi).open_wait);
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_kernel_release_nested(rfile: *mut snd_rawmidi_file, depth: c_int) -> c_int {
    if snd_BUG_ON(rfile.is_null()) != 0 {
        return -ENXIO;
    }
    let rmidi = (*rfile).rmidi;
    mutex_lock_nested(&mut (*rmidi).open_mutex, depth);
    rawmidi_release_priv(rfile);
    mutex_unlock(&mut (*rmidi).open_mutex);
    module_put((*(*rmidi).card).module);
    0
}

unsafe extern "C" fn snd_rawmidi_release(_inode: *mut inode, file: *mut file) -> c_int {
    let rfile = (*file).private_data as *mut snd_rawmidi_file;
    let rmidi = (*rfile).rmidi;
    mutex_lock(&mut (*rmidi).open_mutex);
    rawmidi_release_priv(rfile);
    mutex_unlock(&mut (*rmidi).open_mutex);
    kfree(rfile as *mut c_void);
    let module = (*(*rmidi).card).module;
    snd_card_file_remove((*rmidi).card, file);
    module_put(module);
    0
}

unsafe fn snd_rawmidi_info(substream: *mut snd_rawmidi_substream, info: *mut snd_rawmidi_info) -> c_int {
    if substream.is_null() {
        return -ENODEV;
    }
    let rmidi = (*substream).rmidi;
    memset(info as *mut c_void, 0, size_of::<snd_rawmidi_info>());
    (*info).card = (*(*rmidi).card).number;
    (*info).device = (*rmidi).device;
    (*info).subdevice = (*substream).number as c_uint;
    (*info).stream = (*substream).stream;
    (*info).flags = (*rmidi).info_flags;
    if (*substream).inactive != 0 {
        (*info).flags |= SNDRV_RAWMIDI_INFO_STREAM_INACTIVE;
    }
    strscpy((*info).id.as_mut_ptr(), (*rmidi).id.as_ptr(), (*info).id.len());
    strscpy((*info).name.as_mut_ptr(), (*rmidi).name.as_ptr(), (*info).name.len());
    strscpy((*info).subname.as_mut_ptr(), (*substream).name.as_ptr(), (*info).subname.len());
    (*info).subdevices_count = (*(*substream).pstr).substream_count;
    (*info).subdevices_avail = (*(*substream).pstr).substream_count - (*(*substream).pstr).substream_opened;
    (*info).tied_device = (*rmidi).tied_device;
    0
}

unsafe fn snd_rawmidi_info_user(substream: *mut snd_rawmidi_substream, _info: *mut snd_rawmidi_info) -> c_int {
    let mut info: snd_rawmidi_info = zeroed();
    let err = snd_rawmidi_info(substream, &mut info);
    if err < 0 {
        return err;
    }
    if copy_to_user(_info as *mut c_void, &info as *const _ as *const c_void, size_of::<snd_rawmidi_info>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn __snd_rawmidi_info_select(card: *mut snd_card, info: *mut snd_rawmidi_info) -> c_int {
    let rmidi = snd_rawmidi_search(card, (*info).device);
    if rmidi.is_null() {
        return -ENXIO;
    }
    if (*info).stream < 0 || (*info).stream > 1 {
        return -EINVAL;
    }
    (*info).stream = array_index_nospec((*info).stream, 2);
    let pstr = &mut (*rmidi).streams[(*info).stream as usize] as *mut snd_rawmidi_str;
    if (*pstr).substream_count == 0 {
        return -ENOENT;
    }
    if (*info).subdevice >= (*pstr).substream_count {
        return -ENXIO;
    }
    let mut pos = (*pstr).substreams.next;
    while !pos.is_null() && pos != &mut (*pstr).substreams {
        let substream = (pos as *mut u8).offset(-offset_of_substream_list()) as *mut snd_rawmidi_substream;
        if (*substream).number as c_uint == (*info).subdevice {
            return snd_rawmidi_info(substream, info);
        }
        pos = (*pos).next;
    }
    -ENXIO
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_info_select(card: *mut snd_card, info: *mut snd_rawmidi_info) -> c_int {
    mutex_lock(&mut register_mutex);
    let ret = __snd_rawmidi_info_select(card, info);
    mutex_unlock(&mut register_mutex);
    ret
}

unsafe fn snd_rawmidi_info_select_user(card: *mut snd_card, _info: *mut snd_rawmidi_info) -> c_int {
    let mut info: snd_rawmidi_info = zeroed();
    if get_user_int(&(*_info).device, &mut info.device) != 0 { return -EFAULT; }
    if get_user_int(&(*_info).stream, &mut info.stream) != 0 { return -EFAULT; }
    let mut subdevice_i = 0;
    if get_user_int(&(*_info).subdevice as *const c_uint as *const c_int, &mut subdevice_i) != 0 { return -EFAULT; }
    info.subdevice = subdevice_i as c_uint;
    let err = snd_rawmidi_info_select(card, &mut info);
    if err < 0 { return err; }
    if copy_to_user(_info as *mut c_void, &info as *const _ as *const c_void, size_of::<snd_rawmidi_info>()) != 0 { return -EFAULT; }
    0
}

unsafe fn resize_runtime_buffer(substream: *mut snd_rawmidi_substream, params: *mut snd_rawmidi_params, is_input: bool) -> c_int {
    let runtime = (*substream).runtime;
    let framing = (*params).mode & SNDRV_RAWMIDI_MODE_FRAMING_MASK;
    if (*params).buffer_size < 32 || (*params).buffer_size > 1024 * 1024 { return -EINVAL; }
    if framing == SNDRV_RAWMIDI_MODE_FRAMING_TSTAMP && ((*params).buffer_size & 0x1f) != 0 { return -EINVAL; }
    if (*params).avail_min < 1 || (*params).avail_min > (*params).buffer_size { return -EINVAL; }
    if ((*params).buffer_size as c_int & get_align(runtime)) != 0 { return -EINVAL; }
    if (*params).buffer_size != (*runtime).buffer_size {
        let newbuf = kvzalloc((*params).buffer_size, GFP_KERNEL) as *mut u8;
        if newbuf.is_null() { return -ENOMEM; }
        spin_lock_irq(&mut (*substream).lock);
        if (*runtime).buffer_ref != 0 {
            spin_unlock_irq(&mut (*substream).lock);
            kvfree(newbuf as *mut c_void);
            return -EBUSY;
        }
        let oldbuf = (*runtime).buffer;
        (*runtime).buffer = newbuf;
        (*runtime).buffer_size = (*params).buffer_size;
        __reset_runtime_ptrs(runtime, is_input);
        spin_unlock_irq(&mut (*substream).lock);
        kvfree(oldbuf as *mut c_void);
    }
    (*runtime).avail_min = (*params).avail_min;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_output_params(substream: *mut snd_rawmidi_substream, params: *mut snd_rawmidi_params) -> c_int {
    snd_rawmidi_drain_output(substream);
    mutex_lock(&mut (*(*substream).rmidi).open_mutex);
    if (*substream).append != 0 && (*substream).use_count > 1 {
        mutex_unlock(&mut (*(*substream).rmidi).open_mutex);
        return -EBUSY;
    }
    let err = resize_runtime_buffer(substream, params, false);
    if err == 0 {
        (*substream).active_sensing = ((*params).no_active_sensing == 0) as c_int;
    }
    mutex_unlock(&mut (*(*substream).rmidi).open_mutex);
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_input_params(substream: *mut snd_rawmidi_substream, params: *mut snd_rawmidi_params) -> c_int {
    let framing = (*params).mode & SNDRV_RAWMIDI_MODE_FRAMING_MASK;
    let clock_type = (*params).mode & SNDRV_RAWMIDI_MODE_CLOCK_MASK;
    let mut err: c_int;
    snd_rawmidi_drain_input(substream);
    mutex_lock(&mut (*(*substream).rmidi).open_mutex);
    if framing == SNDRV_RAWMIDI_MODE_FRAMING_NONE && clock_type != SNDRV_RAWMIDI_MODE_CLOCK_NONE {
        err = -EINVAL;
    } else if clock_type > SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC_RAW {
        err = -EINVAL;
    } else if framing > SNDRV_RAWMIDI_MODE_FRAMING_TSTAMP {
        err = -EINVAL;
    } else {
        err = resize_runtime_buffer(substream, params, true);
    }
    if err == 0 {
        (*substream).framing = framing;
        (*substream).clock_type = clock_type;
    }
    mutex_unlock(&mut (*(*substream).rmidi).open_mutex);
    0
}

unsafe fn snd_rawmidi_output_status(substream: *mut snd_rawmidi_substream, status: *mut snd_rawmidi_status64) -> c_int {
    memset(status as *mut c_void, 0, size_of::<snd_rawmidi_status64>());
    (*status).stream = SNDRV_RAWMIDI_STREAM_OUTPUT;
    spin_lock_irq(&mut (*substream).lock);
    (*status).avail = (*(*substream).runtime).avail;
    spin_unlock_irq(&mut (*substream).lock);
    0
}

unsafe fn snd_rawmidi_input_status(substream: *mut snd_rawmidi_substream, status: *mut snd_rawmidi_status64) -> c_int {
    let runtime = (*substream).runtime;
    memset(status as *mut c_void, 0, size_of::<snd_rawmidi_status64>());
    (*status).stream = SNDRV_RAWMIDI_STREAM_INPUT;
    spin_lock_irq(&mut (*substream).lock);
    (*status).avail = (*runtime).avail;
    (*status).xruns = (*runtime).xruns;
    (*runtime).xruns = 0;
    spin_unlock_irq(&mut (*substream).lock);
    0
}

unsafe fn snd_rawmidi_ioctl_status32(rfile: *mut snd_rawmidi_file, argp: *mut snd_rawmidi_status32) -> c_int {
    let mut status32: snd_rawmidi_status32 = zeroed();
    let mut status64: snd_rawmidi_status64 = zeroed();
    if copy_from_user(&mut status32 as *mut _ as *mut c_void, argp as *const c_void, size_of::<snd_rawmidi_status32>()) != 0 { return -EFAULT; }
    let err = match status32.stream {
        SNDRV_RAWMIDI_STREAM_OUTPUT => {
            if (*rfile).output.is_null() { return -EINVAL; }
            snd_rawmidi_output_status((*rfile).output, &mut status64)
        }
        SNDRV_RAWMIDI_STREAM_INPUT => {
            if (*rfile).input.is_null() { return -EINVAL; }
            snd_rawmidi_input_status((*rfile).input, &mut status64)
        }
        _ => return -EINVAL,
    };
    if err < 0 { return err; }
    status32 = snd_rawmidi_status32 {
        stream: status64.stream,
        tstamp_sec: status64.tstamp_sec as s32,
        tstamp_nsec: status64.tstamp_nsec as s32,
        avail: status64.avail as u32,
        xruns: status64.xruns as u32,
        reserved: [0; 16],
    };
    if copy_to_user(argp as *mut c_void, &status32 as *const _ as *const c_void, size_of::<snd_rawmidi_status32>()) != 0 { return -EFAULT; }
    0
}

unsafe fn snd_rawmidi_ioctl_status64(rfile: *mut snd_rawmidi_file, argp: *mut snd_rawmidi_status64) -> c_int {
    let mut status: snd_rawmidi_status64 = zeroed();
    if copy_from_user(&mut status as *mut _ as *mut c_void, argp as *const c_void, size_of::<snd_rawmidi_status64>()) != 0 { return -EFAULT; }
    let err = match status.stream {
        SNDRV_RAWMIDI_STREAM_OUTPUT => {
            if (*rfile).output.is_null() { return -EINVAL; }
            snd_rawmidi_output_status((*rfile).output, &mut status)
        }
        SNDRV_RAWMIDI_STREAM_INPUT => {
            if (*rfile).input.is_null() { return -EINVAL; }
            snd_rawmidi_input_status((*rfile).input, &mut status)
        }
        _ => return -EINVAL,
    };
    if err < 0 { return err; }
    if copy_to_user(argp as *mut c_void, &status as *const _ as *const c_void, size_of::<snd_rawmidi_status64>()) != 0 { return -EFAULT; }
    0
}

unsafe extern "C" fn snd_rawmidi_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let rfile = (*file).private_data as *mut snd_rawmidi_file;
    let argp = arg as *mut c_void;
    if ((cmd >> 8) & 0xff) != b'W' as c_uint {
        return -ENOTTY as c_long;
    }
    match cmd {
        SNDRV_RAWMIDI_IOCTL_PVERSION => return if put_user_int(SNDRV_RAWMIDI_VERSION, argp as *mut c_int) != 0 { -EFAULT as c_long } else { 0 },
        SNDRV_RAWMIDI_IOCTL_INFO => {
            let info = argp as *mut snd_rawmidi_info;
            let mut stream = 0;
            if get_user_int(&(*info).stream, &mut stream) != 0 { return -EFAULT as c_long; }
            return match stream {
                SNDRV_RAWMIDI_STREAM_INPUT => snd_rawmidi_info_user((*rfile).input, info) as c_long,
                SNDRV_RAWMIDI_STREAM_OUTPUT => snd_rawmidi_info_user((*rfile).output, info) as c_long,
                _ => -EINVAL as c_long,
            };
        }
        SNDRV_RAWMIDI_IOCTL_USER_PVERSION => {
            if get_user_uint(arg as *const c_uint, &mut (*rfile).user_pversion) != 0 { return -EFAULT as c_long; }
            return 0;
        }
        SNDRV_RAWMIDI_IOCTL_PARAMS => {
            let mut params: snd_rawmidi_params = zeroed();
            if copy_from_user(&mut params as *mut _ as *mut c_void, argp, size_of::<snd_rawmidi_params>()) != 0 { return -EFAULT as c_long; }
            if (*rfile).user_pversion < ((2 << 16) | (0 << 8) | 2) {
                params.mode = 0;
                memset(params.reserved.as_mut_ptr() as *mut c_void, 0, params.reserved.len());
            }
            return match params.stream {
                SNDRV_RAWMIDI_STREAM_OUTPUT => {
                    if (*rfile).output.is_null() { -EINVAL as c_long } else { snd_rawmidi_output_params((*rfile).output, &mut params) as c_long }
                }
                SNDRV_RAWMIDI_STREAM_INPUT => {
                    if (*rfile).input.is_null() { -EINVAL as c_long } else { snd_rawmidi_input_params((*rfile).input, &mut params) as c_long }
                }
                _ => -EINVAL as c_long,
            };
        }
        SNDRV_RAWMIDI_IOCTL_STATUS32 => return snd_rawmidi_ioctl_status32(rfile, argp as *mut snd_rawmidi_status32) as c_long,
        SNDRV_RAWMIDI_IOCTL_STATUS64 => return snd_rawmidi_ioctl_status64(rfile, argp as *mut snd_rawmidi_status64) as c_long,
        SNDRV_RAWMIDI_IOCTL_DROP => {
            let mut val = 0;
            if get_user_int(argp as *const c_int, &mut val) != 0 { return -EFAULT as c_long; }
            return match val {
                SNDRV_RAWMIDI_STREAM_OUTPUT => if (*rfile).output.is_null() { -EINVAL as c_long } else { snd_rawmidi_drop_output((*rfile).output) as c_long },
                _ => -EINVAL as c_long,
            };
        }
        SNDRV_RAWMIDI_IOCTL_DRAIN => {
            let mut val = 0;
            if get_user_int(argp as *const c_int, &mut val) != 0 { return -EFAULT as c_long; }
            return match val {
                SNDRV_RAWMIDI_STREAM_OUTPUT => if (*rfile).output.is_null() { -EINVAL as c_long } else { snd_rawmidi_drain_output((*rfile).output) as c_long },
                SNDRV_RAWMIDI_STREAM_INPUT => if (*rfile).input.is_null() { -EINVAL as c_long } else { snd_rawmidi_drain_input((*rfile).input) as c_long },
                _ => -EINVAL as c_long,
            };
        }
        _ => {
            let rmidi = (*rfile).rmidi;
            if !(*rmidi).ops.is_null() {
                if let Some(ioctl) = (*(*rmidi).ops).ioctl {
                    return ioctl(rmidi, cmd, argp);
                }
            }
            rmidi_dbg(rmidi, b"rawmidi: unknown command\n\0".as_ptr() as *const c_char);
        }
    }
    -ENOTTY as c_long
}

unsafe fn snd_rawmidi_next_device(card: *mut snd_card, argp: *mut c_int, find_ump: bool) -> c_int {
    let mut device = 0;
    if get_user_int(argp, &mut device) != 0 { return -EFAULT; }
    if device >= SNDRV_RAWMIDI_DEVICES { device = SNDRV_RAWMIDI_DEVICES - 1; }
    mutex_lock(&mut register_mutex);
    device = if device < 0 { 0 } else { device + 1 };
    while device < SNDRV_RAWMIDI_DEVICES {
        let rmidi = snd_rawmidi_search(card, device);
        if !rmidi.is_null() && find_ump == rawmidi_is_ump(rmidi) {
            break;
        }
        device += 1;
    }
    if device == SNDRV_RAWMIDI_DEVICES { device = -1; }
    mutex_unlock(&mut register_mutex);
    if put_user_int(device, argp) != 0 { return -EFAULT; }
    0
}

unsafe fn snd_rawmidi_call_ump_ioctl(card: *mut snd_card, cmd: c_int, argp: *mut c_void) -> c_int {
    let info = argp as *mut snd_ump_endpoint_info;
    let mut device = 0;
    if get_user_int(&(*info).device, &mut device) != 0 { return -EFAULT; }
    mutex_lock(&mut register_mutex);
    let rmidi = snd_rawmidi_search(card, device);
    let ret = if !rmidi.is_null() && !(*rmidi).ops.is_null() {
        if let Some(ioctl) = (*(*rmidi).ops).ioctl {
            ioctl(rmidi, cmd as c_uint, argp) as c_int
        } else {
            -ENXIO
        }
    } else {
        -ENXIO
    };
    mutex_unlock(&mut register_mutex);
    ret
}

unsafe extern "C" fn snd_rawmidi_control_ioctl(card: *mut snd_card, control: *mut snd_ctl_file, cmd: c_uint, arg: c_ulong) -> c_int {
    let argp = arg as *mut c_void;
    match cmd {
        SNDRV_CTL_IOCTL_RAWMIDI_NEXT_DEVICE => snd_rawmidi_next_device(card, argp as *mut c_int, false),
        SNDRV_CTL_IOCTL_UMP_NEXT_DEVICE => snd_rawmidi_next_device(card, argp as *mut c_int, true),
        SNDRV_CTL_IOCTL_UMP_ENDPOINT_INFO => snd_rawmidi_call_ump_ioctl(card, SNDRV_UMP_IOCTL_ENDPOINT_INFO as c_int, argp),
        SNDRV_CTL_IOCTL_UMP_BLOCK_INFO => snd_rawmidi_call_ump_ioctl(card, SNDRV_UMP_IOCTL_BLOCK_INFO as c_int, argp),
        SNDRV_CTL_IOCTL_RAWMIDI_PREFER_SUBDEVICE => {
            let mut val = 0;
            if get_user_int(argp as *const c_int, &mut val) != 0 { return -EFAULT; }
            (*control).preferred_subdevice[SND_CTL_SUBDEV_RAWMIDI] = val;
            0
        }
        SNDRV_CTL_IOCTL_RAWMIDI_INFO => snd_rawmidi_info_select_user(card, argp as *mut snd_rawmidi_info),
        _ => -ENOIOCTLCMD,
    }
}

unsafe fn receive_with_tstamp_framing(
    substream: *mut snd_rawmidi_substream,
    mut buffer: *const u8,
    mut src_count: c_int,
    tstamp: *const timespec64,
) -> c_int {
    let runtime = (*substream).runtime;
    let mut frame = snd_rawmidi_framing_tstamp {
        tv_sec: (*tstamp).tv_sec,
        tv_nsec: (*tstamp).tv_nsec,
        length: 0,
        data: [0; SNDRV_RAWMIDI_FRAMING_DATA_LENGTH as usize],
    };
    let orig_count = src_count;
    let frame_size = size_of::<snd_rawmidi_framing_tstamp>() as c_int;
    let align = get_align(runtime);
    if snd_BUG_ON(((*runtime).hw_ptr & 0x1f) != 0) != 0 { return -EINVAL; }
    while src_count > align {
        if ((*runtime).buffer_size - (*runtime).avail) as c_int < frame_size {
            (*runtime).xruns += src_count as size_t;
            break;
        }
        if src_count >= SNDRV_RAWMIDI_FRAMING_DATA_LENGTH as c_int {
            frame.length = SNDRV_RAWMIDI_FRAMING_DATA_LENGTH;
        } else {
            frame.length = get_aligned_size(runtime, src_count) as c_uint;
            if frame.length == 0 { break; }
            memset(frame.data.as_mut_ptr() as *mut c_void, 0, SNDRV_RAWMIDI_FRAMING_DATA_LENGTH as size_t);
        }
        memcpy(frame.data.as_mut_ptr() as *mut c_void, buffer as *const c_void, frame.length as size_t);
        buffer = buffer.add(frame.length as usize);
        src_count -= frame.length as c_int;
        let dest_ptr = (*runtime).buffer.add((*runtime).hw_ptr) as *mut snd_rawmidi_framing_tstamp;
        ptr::write(dest_ptr, frame);
        (*runtime).avail += frame_size as size_t;
        (*runtime).hw_ptr += frame_size as size_t;
        (*runtime).hw_ptr %= (*runtime).buffer_size;
    }
    orig_count - src_count
}

unsafe fn get_framing_tstamp(substream: *mut snd_rawmidi_substream) -> timespec64 {
    let mut ts64 = timespec64 { tv_sec: 0, tv_nsec: 0 };
    match (*substream).clock_type {
        SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC_RAW => ktime_get_raw_ts64(&mut ts64),
        SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC => ktime_get_ts64(&mut ts64),
        SNDRV_RAWMIDI_MODE_CLOCK_REALTIME => ktime_get_real_ts64(&mut ts64),
        _ => {}
    }
    ts64
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, mut buffer: *const u8, mut count: c_int) -> c_int {
    let ts64 = get_framing_tstamp(substream);
    let mut result = 0;
    let mut count1: c_int;
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    if (*substream).opened == 0 {
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return -EBADFD;
    }
    let runtime = (*substream).runtime;
    if runtime.is_null() || (*runtime).buffer.is_null() {
        rmidi_dbg((*substream).rmidi, b"snd_rawmidi_receive: input is not active!!!\n\0".as_ptr() as *const c_char);
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return -EINVAL;
    }
    count = get_aligned_size(runtime, count);
    if count == 0 {
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return result;
    }
    if (*substream).framing == SNDRV_RAWMIDI_MODE_FRAMING_TSTAMP {
        result = receive_with_tstamp_framing(substream, buffer, count, &ts64);
    } else if count == 1 {
        (*substream).bytes += 1;
        if (*runtime).avail < (*runtime).buffer_size {
            *(*runtime).buffer.add((*runtime).hw_ptr) = *buffer;
            (*runtime).hw_ptr += 1;
            (*runtime).hw_ptr %= (*runtime).buffer_size;
            (*runtime).avail += 1;
            result += 1;
        } else {
            (*runtime).xruns += 1;
        }
    } else {
        (*substream).bytes += count as c_ulong;
        count1 = ((*runtime).buffer_size - (*runtime).hw_ptr) as c_int;
        if count1 > count { count1 = count; }
        if count1 > ((*runtime).buffer_size - (*runtime).avail) as c_int { count1 = ((*runtime).buffer_size - (*runtime).avail) as c_int; }
        count1 = get_aligned_size(runtime, count1);
        if count1 == 0 {
            spin_unlock_irqrestore(&mut (*substream).lock, 0);
            return result;
        }
        memcpy((*runtime).buffer.add((*runtime).hw_ptr) as *mut c_void, buffer as *const c_void, count1 as size_t);
        (*runtime).hw_ptr += count1 as size_t;
        (*runtime).hw_ptr %= (*runtime).buffer_size;
        (*runtime).avail += count1 as size_t;
        count -= count1;
        result += count1;
        if count > 0 {
            buffer = buffer.add(count1 as usize);
            count1 = count;
            if count1 > ((*runtime).buffer_size - (*runtime).avail) as c_int {
                count1 = ((*runtime).buffer_size - (*runtime).avail) as c_int;
                (*runtime).xruns += (count - count1) as size_t;
            }
            if count1 > 0 {
                memcpy((*runtime).buffer as *mut c_void, buffer as *const c_void, count1 as size_t);
                (*runtime).hw_ptr = count1 as size_t;
                (*runtime).avail += count1 as size_t;
                result += count1;
            }
        }
    }
    if result > 0 {
        if (*runtime).event.is_some() {
            schedule_work(&mut (*runtime).event_work);
        } else if __snd_rawmidi_ready(runtime) {
            wake_up(&mut (*runtime).sleep);
        }
    }
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
    result
}

unsafe fn snd_rawmidi_kernel_read1(substream: *mut snd_rawmidi_substream, userbuf: *mut u8, kernelbuf: *mut u8, mut count: c_long) -> c_long {
    let mut flags: c_ulong = 0;
    let mut result: c_long = 0;
    let mut err: c_int = 0;
    let runtime = (*substream).runtime;
    spin_lock_irqsave(&mut (*substream).lock, &mut flags);
    snd_rawmidi_buffer_ref(runtime);
    while count > 0 && (*runtime).avail != 0 {
        let mut count1 = ((*runtime).buffer_size - (*runtime).appl_ptr) as c_long;
        if count1 > count { count1 = count; }
        if count1 > (*runtime).avail as c_long { count1 = (*runtime).avail as c_long; }
        let appl_ptr = (*runtime).appl_ptr;
        (*runtime).appl_ptr += count1 as size_t;
        (*runtime).appl_ptr %= (*runtime).buffer_size;
        (*runtime).avail -= count1 as size_t;
        if !kernelbuf.is_null() {
            memcpy(kernelbuf.offset(result as isize) as *mut c_void, (*runtime).buffer.add(appl_ptr) as *const c_void, count1 as size_t);
        }
        if !userbuf.is_null() {
            spin_unlock_irqrestore(&mut (*substream).lock, flags);
            if copy_to_user(userbuf.offset(result as isize) as *mut c_void, (*runtime).buffer.add(appl_ptr) as *const c_void, count1 as size_t) != 0 {
                err = -EFAULT;
            }
            spin_lock_irqsave(&mut (*substream).lock, &mut flags);
            if err != 0 { break; }
        }
        result += count1;
        count -= count1;
    }
    snd_rawmidi_buffer_unref(runtime);
    spin_unlock_irqrestore(&mut (*substream).lock, flags);
    if result > 0 { result } else { err as c_long }
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_kernel_read(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: c_long) -> c_long {
    snd_rawmidi_input_trigger(substream, 1);
    snd_rawmidi_kernel_read1(substream, ptr::null_mut(), buf, count)
}

unsafe extern "C" fn snd_rawmidi_read(file: *mut file, mut buf: *mut c_char, mut count: size_t, _offset: *mut loff_t) -> ssize_t {
    let rfile = (*file).private_data as *mut snd_rawmidi_file;
    let substream = (*rfile).input;
    if substream.is_null() { return -EIO as ssize_t; }
    let runtime = (*substream).runtime;
    snd_rawmidi_input_trigger(substream, 1);
    let mut result: c_long = 0;
    while count > 0 {
        spin_lock_irq(&mut (*substream).lock);
        while !__snd_rawmidi_ready(runtime) {
            let mut wait: wait_queue_entry_t = zeroed();
            if ((*file).f_flags & O_NONBLOCK) != 0 || result > 0 {
                spin_unlock_irq(&mut (*substream).lock);
                return if result > 0 { result as ssize_t } else { -EAGAIN as ssize_t };
            }
            init_waitqueue_entry(&mut wait, current);
            add_wait_queue(&mut (*runtime).sleep, &mut wait);
            set_current_state(TASK_INTERRUPTIBLE);
            spin_unlock_irq(&mut (*substream).lock);
            schedule();
            remove_wait_queue(&mut (*runtime).sleep, &mut wait);
            if (*(*(*rfile).rmidi).card).shutdown != 0 { return -ENODEV as ssize_t; }
            if signal_pending(current) != 0 { return if result > 0 { result as ssize_t } else { -ERESTARTSYS as ssize_t }; }
            spin_lock_irq(&mut (*substream).lock);
            if (*runtime).avail == 0 {
                spin_unlock_irq(&mut (*substream).lock);
                return if result > 0 { result as ssize_t } else { -EIO as ssize_t };
            }
        }
        spin_unlock_irq(&mut (*substream).lock);
        let count1 = snd_rawmidi_kernel_read1(substream, buf as *mut u8, ptr::null_mut(), count as c_long);
        if count1 < 0 { return if result > 0 { result as ssize_t } else { count1 as ssize_t }; }
        result += count1;
        buf = buf.offset(count1 as isize);
        count -= count1 as size_t;
    }
    result as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_transmit_empty(substream: *mut snd_rawmidi_substream) -> c_int {
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    let runtime = (*substream).runtime;
    if (*substream).opened == 0 || runtime.is_null() || (*runtime).buffer.is_null() {
        rmidi_dbg((*substream).rmidi, b"snd_rawmidi_transmit_empty: output is not active!!!\n\0".as_ptr() as *const c_char);
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return 1;
    }
    let ret = ((*runtime).avail >= (*runtime).buffer_size) as c_int;
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
    ret
}

unsafe fn __snd_rawmidi_transmit_peek(substream: *mut snd_rawmidi_substream, buffer: *mut u8, mut count: c_int) -> c_int {
    let runtime = (*substream).runtime;
    if (*runtime).buffer.is_null() {
        rmidi_dbg((*substream).rmidi, b"snd_rawmidi_transmit_peek: output is not active!!!\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let mut result = 0;
    if (*runtime).avail >= (*runtime).buffer_size { return result; }
    if count == 1 {
        *buffer = *(*runtime).buffer.add((*runtime).hw_ptr);
        result += 1;
    } else {
        let mut count1 = ((*runtime).buffer_size - (*runtime).hw_ptr) as c_int;
        if count1 > count { count1 = count; }
        if count1 > ((*runtime).buffer_size - (*runtime).avail) as c_int { count1 = ((*runtime).buffer_size - (*runtime).avail) as c_int; }
        count1 = get_aligned_size(runtime, count1);
        if count1 == 0 { return result; }
        memcpy(buffer as *mut c_void, (*runtime).buffer.add((*runtime).hw_ptr) as *const c_void, count1 as size_t);
        count -= count1;
        result += count1;
        if count > 0 {
            if count > ((*runtime).buffer_size - (*runtime).avail - count1 as size_t) as c_int {
                count = ((*runtime).buffer_size - (*runtime).avail - count1 as size_t) as c_int;
            }
            count = get_aligned_size(runtime, count);
            if count == 0 { return result; }
            memcpy(buffer.offset(count1 as isize) as *mut c_void, (*runtime).buffer as *const c_void, count as size_t);
            result += count;
        }
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_transmit_peek(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int {
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    if (*substream).opened == 0 || (*substream).runtime.is_null() {
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return -EBADFD;
    }
    let ret = __snd_rawmidi_transmit_peek(substream, buffer, count);
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
    ret
}

unsafe fn __snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, mut count: c_int) -> c_int {
    let runtime = (*substream).runtime;
    if (*runtime).buffer.is_null() {
        rmidi_dbg((*substream).rmidi, b"snd_rawmidi_transmit_ack: output is not active!!!\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    snd_BUG_ON((*runtime).avail + count as size_t > (*runtime).buffer_size);
    count = get_aligned_size(runtime, count);
    (*runtime).hw_ptr += count as size_t;
    (*runtime).hw_ptr %= (*runtime).buffer_size;
    (*runtime).avail += count as size_t;
    (*substream).bytes += count as c_ulong;
    if count > 0 && ((*runtime).drain != 0 || __snd_rawmidi_ready(runtime)) {
        wake_up(&mut (*runtime).sleep);
    }
    count
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int {
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    if (*substream).opened == 0 || (*substream).runtime.is_null() {
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return -EBADFD;
    }
    let ret = __snd_rawmidi_transmit_ack(substream, count);
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buffer: *mut u8, mut count: c_int) -> c_int {
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    if (*substream).opened == 0 {
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return -EBADFD;
    }
    count = __snd_rawmidi_transmit_peek(substream, buffer, count);
    if count <= 0 {
        spin_unlock_irqrestore(&mut (*substream).lock, 0);
        return count;
    }
    let ret = __snd_rawmidi_transmit_ack(substream, count);
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_proceed(substream: *mut snd_rawmidi_substream) -> c_int {
    let mut count = 0;
    spin_lock_irqsave(&mut (*substream).lock, &mut 0);
    let runtime = (*substream).runtime;
    if (*substream).opened != 0 && !runtime.is_null() && (*runtime).avail < (*runtime).buffer_size {
        count = ((*runtime).buffer_size - (*runtime).avail) as c_int;
        __snd_rawmidi_transmit_ack(substream, count);
    }
    spin_unlock_irqrestore(&mut (*substream).lock, 0);
    count
}

unsafe fn snd_rawmidi_kernel_write1(substream: *mut snd_rawmidi_substream, userbuf: *const u8, kernelbuf: *const u8, mut count: c_long) -> c_long {
    let mut flags: c_ulong = 0;
    let runtime = (*substream).runtime;
    if kernelbuf.is_null() && userbuf.is_null() { return -EINVAL as c_long; }
    if snd_BUG_ON((*runtime).buffer.is_null()) != 0 { return -EINVAL as c_long; }
    let mut result: c_long = 0;
    spin_lock_irqsave(&mut (*substream).lock, &mut flags);
    if (*substream).append != 0 && (*runtime).avail as c_long < count {
        spin_unlock_irqrestore(&mut (*substream).lock, flags);
        return -EAGAIN as c_long;
    }
    snd_rawmidi_buffer_ref(runtime);
    while count > 0 && (*runtime).avail > 0 {
        let mut count1 = ((*runtime).buffer_size - (*runtime).appl_ptr) as c_long;
        if count1 > count { count1 = count; }
        if count1 > (*runtime).avail as c_long { count1 = (*runtime).avail as c_long; }
        let appl_ptr = (*runtime).appl_ptr;
        (*runtime).appl_ptr += count1 as size_t;
        (*runtime).appl_ptr %= (*runtime).buffer_size;
        (*runtime).avail -= count1 as size_t;
        if !kernelbuf.is_null() {
            memcpy((*runtime).buffer.add(appl_ptr) as *mut c_void, kernelbuf.offset(result as isize) as *const c_void, count1 as size_t);
        } else if !userbuf.is_null() {
            spin_unlock_irqrestore(&mut (*substream).lock, flags);
            if copy_from_user((*runtime).buffer.add(appl_ptr) as *mut c_void, userbuf.offset(result as isize) as *const c_void, count1 as size_t) != 0 {
                spin_lock_irqsave(&mut (*substream).lock, &mut flags);
                result = if result > 0 { result } else { -EFAULT as c_long };
                break;
            }
            spin_lock_irqsave(&mut (*substream).lock, &mut flags);
        }
        result += count1;
        count -= count1;
    }
    let count1_nonempty = ((*runtime).avail < (*runtime).buffer_size) as c_int;
    snd_rawmidi_buffer_unref(runtime);
    spin_unlock_irqrestore(&mut (*substream).lock, flags);
    if count1_nonempty != 0 {
        snd_rawmidi_output_trigger(substream, 1);
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_kernel_write(substream: *mut snd_rawmidi_substream, buf: *const u8, count: c_long) -> c_long {
    snd_rawmidi_kernel_write1(substream, ptr::null(), buf, count)
}

unsafe extern "C" fn snd_rawmidi_write(file: *mut file, mut buf: *const c_char, mut count: size_t, _offset: *mut loff_t) -> ssize_t {
    let rfile = (*file).private_data as *mut snd_rawmidi_file;
    let substream = (*rfile).output;
    let runtime = (*substream).runtime;
    if (*substream).append != 0 && count > (*runtime).buffer_size { return -EIO as ssize_t; }
    let mut result: c_long = 0;
    while count > 0 {
        spin_lock_irq(&mut (*substream).lock);
        while snd_rawmidi_ready_append(substream, count) == 0 {
            let mut wait: wait_queue_entry_t = zeroed();
            if ((*file).f_flags & O_NONBLOCK) != 0 {
                spin_unlock_irq(&mut (*substream).lock);
                return if result > 0 { result as ssize_t } else { -EAGAIN as ssize_t };
            }
            init_waitqueue_entry(&mut wait, current);
            add_wait_queue(&mut (*runtime).sleep, &mut wait);
            set_current_state(TASK_INTERRUPTIBLE);
            spin_unlock_irq(&mut (*substream).lock);
            let timeout = schedule_timeout(30 * HZ);
            remove_wait_queue(&mut (*runtime).sleep, &mut wait);
            if (*(*(*rfile).rmidi).card).shutdown != 0 { return -ENODEV as ssize_t; }
            if signal_pending(current) != 0 { return if result > 0 { result as ssize_t } else { -ERESTARTSYS as ssize_t }; }
            spin_lock_irq(&mut (*substream).lock);
            if (*runtime).avail == 0 && timeout == 0 {
                spin_unlock_irq(&mut (*substream).lock);
                return if result > 0 { result as ssize_t } else { -EIO as ssize_t };
            }
        }
        spin_unlock_irq(&mut (*substream).lock);
        let count1 = snd_rawmidi_kernel_write1(substream, buf as *const u8, ptr::null(), count as c_long);
        if count1 < 0 { return if result > 0 { result as ssize_t } else { count1 as ssize_t }; }
        result += count1;
        buf = buf.offset(count1 as isize);
        if (count1 as size_t) < count && ((*file).f_flags & O_NONBLOCK) != 0 { break; }
        count -= count1 as size_t;
    }
    if ((*file).f_flags & O_DSYNC) != 0 {
        spin_lock_irq(&mut (*substream).lock);
        while (*runtime).avail != (*runtime).buffer_size {
            let mut wait: wait_queue_entry_t = zeroed();
            let last_avail = (*runtime).avail;
            init_waitqueue_entry(&mut wait, current);
            add_wait_queue(&mut (*runtime).sleep, &mut wait);
            set_current_state(TASK_INTERRUPTIBLE);
            spin_unlock_irq(&mut (*substream).lock);
            let timeout = schedule_timeout(30 * HZ);
            remove_wait_queue(&mut (*runtime).sleep, &mut wait);
            if signal_pending(current) != 0 { return if result > 0 { result as ssize_t } else { -ERESTARTSYS as ssize_t }; }
            if (*runtime).avail == last_avail && timeout == 0 { return if result > 0 { result as ssize_t } else { -EIO as ssize_t }; }
            spin_lock_irq(&mut (*substream).lock);
        }
        spin_unlock_irq(&mut (*substream).lock);
    }
    result as ssize_t
}

unsafe extern "C" fn snd_rawmidi_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let rfile = (*file).private_data as *mut snd_rawmidi_file;
    let mut mask: __poll_t = 0;
    if !(*rfile).input.is_null() {
        let runtime = (*(*rfile).input).runtime;
        snd_rawmidi_input_trigger((*rfile).input, 1);
        poll_wait(file, &mut (*runtime).sleep, wait);
    }
    if !(*rfile).output.is_null() {
        let runtime = (*(*rfile).output).runtime;
        poll_wait(file, &mut (*runtime).sleep, wait);
    }
    if !(*rfile).input.is_null() && snd_rawmidi_ready((*rfile).input) {
        mask |= EPOLLIN | EPOLLRDNORM;
    }
    if !(*rfile).output.is_null() && snd_rawmidi_ready((*rfile).output) {
        mask |= EPOLLOUT | EPOLLWRNORM;
    }
    mask
}

unsafe extern "C" fn snd_rawmidi_ioctl_compat(_file: *mut file, _cmd: c_uint, _arg: c_ulong) -> c_long {
    0
}

unsafe extern "C" fn snd_rawmidi_proc_info_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let rmidi = (*entry).private_data as *mut snd_rawmidi;
    snd_iprintf(buffer, b"%s\n\n\0".as_ptr() as *const c_char, (*rmidi).name.as_ptr());
    snd_iprintf(buffer, b"Type: %s\n\0".as_ptr() as *const c_char, if rawmidi_is_ump(rmidi) { b"UMP\0".as_ptr() } else { b"Legacy\0".as_ptr() });
    if !(*rmidi).ops.is_null() {
        if let Some(proc_read) = (*(*rmidi).ops).proc_read {
            proc_read(entry, buffer);
        }
    }
    mutex_lock(&mut (*rmidi).open_mutex);
    /* Iterates output and input substreams and prints bytes, owner, buffer,
     * availability, overrun, OSS compatibility, and timestamp framing state.
     */
    mutex_unlock(&mut (*rmidi).open_mutex);
}

static snd_rawmidi_f_ops: file_operations = file_operations {
    owner: ptr::null_mut(),
    read: Some(snd_rawmidi_read),
    write: Some(snd_rawmidi_write),
    open: Some(snd_rawmidi_open),
    release: Some(snd_rawmidi_release),
    poll: Some(snd_rawmidi_poll),
    unlocked_ioctl: Some(snd_rawmidi_ioctl),
    compat_ioctl: Some(snd_rawmidi_ioctl_compat),
};

unsafe fn snd_rawmidi_alloc_substreams(rmidi: *mut snd_rawmidi, stream: *mut snd_rawmidi_str, direction: c_int, count: c_int) -> c_int {
    let mut idx = 0;
    while idx < count {
        let substream = kzalloc(size_of::<snd_rawmidi_substream>(), GFP_KERNEL) as *mut snd_rawmidi_substream;
        if substream.is_null() { return -ENOMEM; }
        (*substream).stream = direction;
        (*substream).number = idx;
        (*substream).rmidi = rmidi;
        (*substream).pstr = stream;
        spin_lock_init(&mut (*substream).lock);
        list_add_tail(&mut (*substream).list, &mut (*stream).substreams);
        (*stream).substream_count += 1;
        idx += 1;
    }
    0
}

unsafe extern "C" {
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_init(
    rmidi: *mut snd_rawmidi,
    card: *mut snd_card,
    id: *mut c_char,
    device: c_int,
    output_count: c_int,
    input_count: c_int,
    info_flags: c_uint,
) -> c_int {
    let ops = snd_device_ops {
        dev_free: Some(snd_rawmidi_dev_free),
        dev_register: Some(snd_rawmidi_dev_register),
        dev_disconnect: Some(snd_rawmidi_dev_disconnect),
    };
    (*rmidi).card = card;
    (*rmidi).device = device;
    mutex_init(&mut (*rmidi).open_mutex);
    init_waitqueue_head(&mut (*rmidi).open_wait);
    INIT_LIST_HEAD(&mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize].substreams);
    INIT_LIST_HEAD(&mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize].substreams);
    (*rmidi).info_flags = info_flags;
    if !id.is_null() {
        strscpy((*rmidi).id.as_mut_ptr(), id, (*rmidi).id.len());
    }
    let mut err = snd_device_alloc(&mut (*rmidi).dev, card);
    if err < 0 { return err; }
    if rawmidi_is_ump(rmidi) {
        dev_set_name((*rmidi).dev, b"umpC%iD%i\0".as_ptr() as *const c_char, (*card).number, device);
    } else {
        dev_set_name((*rmidi).dev, b"midiC%iD%i\0".as_ptr() as *const c_char, (*card).number, device);
    }
    err = snd_rawmidi_alloc_substreams(rmidi, &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize], SNDRV_RAWMIDI_STREAM_INPUT, input_count);
    if err < 0 { return err; }
    err = snd_rawmidi_alloc_substreams(rmidi, &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize], SNDRV_RAWMIDI_STREAM_OUTPUT, output_count);
    if err < 0 { return err; }
    err = snd_device_new(card, SNDRV_DEV_RAWMIDI, rmidi as *mut c_void, &ops);
    if err < 0 { return err; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_new(
    card: *mut snd_card,
    id: *mut c_char,
    device: c_int,
    output_count: c_int,
    input_count: c_int,
    rrawmidi: *mut *mut snd_rawmidi,
) -> c_int {
    if !rrawmidi.is_null() { *rrawmidi = ptr::null_mut(); }
    let rmidi = kzalloc(size_of::<snd_rawmidi>(), GFP_KERNEL) as *mut snd_rawmidi;
    if rmidi.is_null() { return -ENOMEM; }
    let err = snd_rawmidi_init(rmidi, card, id, device, output_count, input_count, 0);
    if err < 0 {
        snd_rawmidi_free(rmidi);
        return err;
    }
    if !rrawmidi.is_null() { *rrawmidi = rmidi; }
    0
}

unsafe fn snd_rawmidi_free_substreams(stream: *mut snd_rawmidi_str) {
    while list_empty(&(*stream).substreams) == 0 {
        let substream = list_first_entry::<snd_rawmidi_substream>(&mut (*stream).substreams, offset_of_substream_list());
        list_del(&mut (*substream).list);
        kfree(substream as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_free(rmidi: *mut snd_rawmidi) -> c_int {
    if rmidi.is_null() { return 0; }
    snd_info_free_entry((*rmidi).proc_entry);
    (*rmidi).proc_entry = ptr::null_mut();
    if !(*rmidi).ops.is_null() {
        if let Some(dev_unregister) = (*(*rmidi).ops).dev_unregister {
            dev_unregister(rmidi);
        }
    }
    snd_rawmidi_free_substreams(&mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize]);
    snd_rawmidi_free_substreams(&mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize]);
    if let Some(private_free) = (*rmidi).private_free {
        private_free(rmidi);
    }
    put_device((*rmidi).dev);
    kfree(rmidi as *mut c_void);
    0
}

unsafe extern "C" fn snd_rawmidi_dev_free(device: *mut snd_device) -> c_int {
    snd_rawmidi_free((*device).device_data as *mut snd_rawmidi)
}

unsafe extern "C" fn snd_rawmidi_dev_seq_free(device: *mut snd_seq_device) {
    let rmidi = (*device).private_data as *mut snd_rawmidi;
    (*rmidi).seq_dev = ptr::null_mut();
}

unsafe extern "C" fn snd_rawmidi_dev_register(device: *mut snd_device) -> c_int {
    let rmidi = (*device).device_data as *mut snd_rawmidi;
    if (*rmidi).device >= SNDRV_RAWMIDI_DEVICES { return -ENOMEM; }
    let mut err = 0;
    mutex_lock(&mut register_mutex);
    if !snd_rawmidi_search((*rmidi).card, (*rmidi).device).is_null() {
        err = -EBUSY;
    } else {
        list_add_tail(&mut (*rmidi).list, &mut snd_rawmidi_devices);
    }
    mutex_unlock(&mut register_mutex);
    if err < 0 { return err; }
    err = snd_register_device(SNDRV_DEVICE_TYPE_RAWMIDI, (*rmidi).card, (*rmidi).device, &snd_rawmidi_f_ops, rmidi as *mut c_void, (*rmidi).dev);
    if err < 0 {
        rmidi_err(rmidi, b"unable to register\n\0".as_ptr() as *const c_char);
        mutex_lock(&mut register_mutex);
        list_del(&mut (*rmidi).list);
        mutex_unlock(&mut register_mutex);
        return err;
    }
    if !(*rmidi).ops.is_null() {
        if let Some(dev_register) = (*(*rmidi).ops).dev_register {
            err = dev_register(rmidi);
            if err < 0 {
                snd_unregister_device((*rmidi).dev);
                mutex_lock(&mut register_mutex);
                list_del(&mut (*rmidi).list);
                mutex_unlock(&mut register_mutex);
                return err;
            }
        }
    }
    /* CONFIG_SND_OSSEMUL: register legacy OSS midi/amidi devices using midi_map and amidi_map. */
    let mut name = [0 as c_char; 16];
    sprintf(name.as_mut_ptr(), b"midi%d\0".as_ptr() as *const c_char, (*rmidi).device);
    let entry = snd_info_create_card_entry((*rmidi).card, name.as_ptr(), (*(*rmidi).card).proc_root);
    if !entry.is_null() {
        (*entry).private_data = rmidi as *mut c_void;
        (*entry).c.text.read = Some(snd_rawmidi_proc_info_read);
        if snd_info_register(entry) < 0 {
            snd_info_free_entry(entry);
            (*rmidi).proc_entry = ptr::null_mut();
        } else {
            (*rmidi).proc_entry = entry;
        }
    } else {
        (*rmidi).proc_entry = ptr::null_mut();
    }
    /* CONFIG_SND_SEQUENCER: create and register a sequencer device when no own registration mechanism exists. */
    if (*rmidi).ops.is_null() || (*(*rmidi).ops).dev_register.is_none() {
        if snd_seq_device_new((*rmidi).card, (*rmidi).device, SNDRV_SEQ_DEV_ID_MIDISYNTH, 0, &mut (*rmidi).seq_dev) >= 0 {
            (*(*rmidi).seq_dev).private_data = rmidi as *mut c_void;
            (*(*rmidi).seq_dev).private_free = Some(snd_rawmidi_dev_seq_free);
            sprintf((*(*rmidi).seq_dev).name.as_mut_ptr(), b"MIDI %d-%d\0".as_ptr() as *const c_char, (*(*rmidi).card).number, (*rmidi).device);
            snd_device_register((*rmidi).card, (*rmidi).seq_dev);
        }
    }
    0
}

unsafe extern "C" fn snd_rawmidi_dev_disconnect(device: *mut snd_device) -> c_int {
    let rmidi = (*device).device_data as *mut snd_rawmidi;
    mutex_lock(&mut register_mutex);
    mutex_lock(&mut (*rmidi).open_mutex);
    wake_up(&mut (*rmidi).open_wait);
    list_del_init(&mut (*rmidi).list);
    let mut dir = 0usize;
    while dir < 2 {
        let stream = &mut (*rmidi).streams[dir] as *mut snd_rawmidi_str;
        let mut pos = (*stream).substreams.next;
        while !pos.is_null() && pos != &mut (*stream).substreams {
            let s = (pos as *mut u8).offset(-offset_of_substream_list()) as *mut snd_rawmidi_substream;
            if !(*s).runtime.is_null() {
                wake_up(&mut (*(*s).runtime).sleep);
            }
            pos = (*pos).next;
        }
        dir += 1;
    }
    /* CONFIG_SND_OSSEMUL: unregister OSS midi/amidi devices and clear ossreg. */
    if (*rmidi).ossreg != 0 {
        if (*rmidi).device == midi_map[(*(*rmidi).card).number as usize] {
            snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_MIDI, (*rmidi).card, 0);
        }
        if (*rmidi).device == amidi_map[(*(*rmidi).card).number as usize] {
            snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_MIDI, (*rmidi).card, 1);
        }
        (*rmidi).ossreg = 0;
    }
    snd_unregister_device((*rmidi).dev);
    mutex_unlock(&mut (*rmidi).open_mutex);
    mutex_unlock(&mut register_mutex);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops) {
    let s = &mut (*rmidi).streams[stream as usize] as *mut snd_rawmidi_str;
    let mut pos = (*s).substreams.next;
    while !pos.is_null() && pos != &mut (*s).substreams {
        let substream = (pos as *mut u8).offset(-offset_of_substream_list()) as *mut snd_rawmidi_substream;
        (*substream).ops = ops;
        pos = (*pos).next;
    }
}

unsafe extern "C" fn alsa_rawmidi_init() -> c_int {
    snd_ctl_register_ioctl(snd_rawmidi_control_ioctl);
    snd_ctl_register_ioctl_compat(snd_rawmidi_control_ioctl);
    /* CONFIG_SND_OSSEMUL: check device map table. */
    let mut i = 0usize;
    while i < SNDRV_CARDS {
        if midi_map[i] < 0 || midi_map[i] >= SNDRV_RAWMIDI_DEVICES {
            pr_err(b"ALSA: rawmidi: invalid midi_map[%d] = %d\n\0".as_ptr() as *const c_char, i as c_int, midi_map[i]);
            midi_map[i] = 0;
        }
        if amidi_map[i] < 0 || amidi_map[i] >= SNDRV_RAWMIDI_DEVICES {
            pr_err(b"ALSA: rawmidi: invalid amidi_map[%d] = %d\n\0".as_ptr() as *const c_char, i as c_int, amidi_map[i]);
            amidi_map[i] = 1;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn alsa_rawmidi_exit() {
    snd_ctl_unregister_ioctl(snd_rawmidi_control_ioctl);
    snd_ctl_unregister_ioctl_compat(snd_rawmidi_control_ioctl);
}

/* module_init(alsa_rawmidi_init) */
/* module_exit(alsa_rawmidi_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
