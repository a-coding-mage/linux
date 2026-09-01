// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for driver control interface
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{self, size_of};
use core::ptr;

type bool_ = bool;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type u32 = u32;
type u64 = u64;
type uintptr_t = usize;
type __poll_t = c_uint;

const GFP_KERNEL: c_uint = 0;
const GFP_ATOMIC: c_uint = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EFAULT: c_int = 14;
const ENXIO: c_int = 6;
const ENOSPC: c_int = 28;
const ENOIOCTLCMD: c_int = 515;
const ENOTTY: c_int = 25;
const EBADFD: c_int = 77;
const EAGAIN: c_int = 11;
const ERESTARTSYS: c_int = 512;
const ENOPROTOOPT: c_int = 92;
const UINT_MAX: c_uint = c_uint::MAX;
const LONG_MAX: c_ulong = c_long::MAX as c_ulong;

const SIGIO: c_int = 29;
const POLL_IN: c_int = 1;
const POLL_ERR: c_int = 8;
const EPOLLIN: __poll_t = 0x001;
const EPOLLRDNORM: __poll_t = 0x040;
const O_NONBLOCK: c_int = 0o4000;
const TASK_INTERRUPTIBLE: c_long = 1;

const SNDRV_CARDS: c_int = 32;
const SND_CTL_SUBDEV_ITEMS: usize = 4;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;
const MAX_CONTROL_COUNT: c_uint = 1028;
const MULTIPLIER: c_ulong = 37;

const SNDRV_DEVICE_TYPE_CONTROL: c_int = 0;
const SNDRV_DEV_CONTROL: c_int = 0;
const SNDRV_CTL_VERSION: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;

const SNDRV_CTL_ELEM_IFACE_CARD: usize = 0;
const SNDRV_CTL_ELEM_IFACE_HWDEP: usize = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: usize = 2;
const SNDRV_CTL_ELEM_IFACE_PCM: usize = 3;
const SNDRV_CTL_ELEM_IFACE_RAWMIDI: usize = 4;
const SNDRV_CTL_ELEM_IFACE_TIMER: usize = 5;
const SNDRV_CTL_ELEM_IFACE_SEQUENCER: usize = 6;

const SNDRV_CTL_ELEM_TYPE_BOOLEAN: usize = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: usize = 2;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: usize = 3;
const SNDRV_CTL_ELEM_TYPE_BYTES: usize = 4;
const SNDRV_CTL_ELEM_TYPE_IEC958: usize = 5;
const SNDRV_CTL_ELEM_TYPE_INTEGER64: usize = 6;

const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint = 1 << 1;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint =
    SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint = 1 << 8;
const SNDRV_CTL_ELEM_ACCESS_LOCK: c_uint = 1 << 9;
const SNDRV_CTL_ELEM_ACCESS_OWNER: c_uint = 1 << 10;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 11;
const SNDRV_CTL_ELEM_ACCESS_TLV_WRITE: c_uint = 1 << 12;
const SNDRV_CTL_ELEM_ACCESS_TLV_READWRITE: c_uint =
    SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_TLV_WRITE;
const SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND: c_uint = 1 << 13;
const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: c_uint = 1 << 14;
const SNDRV_CTL_ELEM_ACCESS_USER: c_uint = 1 << 29;
const SNDRV_CTL_ELEM_ACCESS_LED_MASK: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK: c_uint = 1 << 30;

const SNDRV_CTL_EVENT_ELEM: c_int = 0;
const SNDRV_CTL_EVENT_MASK_ADD: c_uint = 1 << 0;
const SNDRV_CTL_EVENT_MASK_REMOVE: c_uint = 1 << 1;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1 << 2;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 1 << 3;
const SNDRV_CTL_EVENT_MASK_TLV: c_uint = 1 << 4;

const SNDRV_CTL_TLV_OP_READ: c_int = 0;
const SNDRV_CTL_TLV_OP_WRITE: c_int = 1;
const SNDRV_CTL_TLV_OP_CMD: c_int = 2;

const SND_CTL_CARD_BTYPE_COMPONENTS: c_uint = 0;

const SNDRV_CTL_IOCTL_PVERSION: c_uint = 0;
const SNDRV_CTL_IOCTL_CARD_INFO: c_uint = 1;
const SNDRV_CTL_IOCTL_CARD_BYTES: c_uint = 2;
const SNDRV_CTL_IOCTL_ELEM_LIST: c_uint = 3;
const SNDRV_CTL_IOCTL_ELEM_INFO: c_uint = 4;
const SNDRV_CTL_IOCTL_ELEM_READ: c_uint = 5;
const SNDRV_CTL_IOCTL_ELEM_WRITE: c_uint = 6;
const SNDRV_CTL_IOCTL_ELEM_LOCK: c_uint = 7;
const SNDRV_CTL_IOCTL_ELEM_UNLOCK: c_uint = 8;
const SNDRV_CTL_IOCTL_ELEM_ADD: c_uint = 9;
const SNDRV_CTL_IOCTL_ELEM_REPLACE: c_uint = 10;
const SNDRV_CTL_IOCTL_ELEM_REMOVE: c_uint = 11;
const SNDRV_CTL_IOCTL_SUBSCRIBE_EVENTS: c_uint = 12;
const SNDRV_CTL_IOCTL_TLV_READ: c_uint = 13;
const SNDRV_CTL_IOCTL_TLV_WRITE: c_uint = 14;
const SNDRV_CTL_IOCTL_TLV_COMMAND: c_uint = 15;
const SNDRV_CTL_IOCTL_POWER: c_uint = 16;
const SNDRV_CTL_IOCTL_POWER_STATE: c_uint = 17;

// Max allocation size for user controls.
static mut max_user_ctl_alloc_size: c_int = 8 * 1024 * 1024;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rw_semaphore {
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
    pub private_data: *mut c_void,
    pub f_flags: c_int,
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fasync_struct {
    _private: [u8; 0],
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
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
    pub owner: *mut snd_ctl_file,
}

pub type snd_kcontrol_info_t =
    Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>;
pub type snd_kcontrol_get_t =
    Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
pub type snd_kcontrol_put_t =
    Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
pub type snd_kcontrol_tlv_rw_t =
    Option<unsafe extern "C" fn(*mut snd_kcontrol, c_int, c_uint, *mut c_uint) -> c_int>;
pub type snd_kcontrol_private_free_t = Option<unsafe extern "C" fn(*mut snd_kcontrol)>;

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
    pub c: snd_kcontrol_tlv_rw_t,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub list: list_head,
    pub id: snd_ctl_elem_id,
    pub count: c_uint,
    pub info: snd_kcontrol_info_t,
    pub get: snd_kcontrol_get_t,
    pub put: snd_kcontrol_put_t,
    pub tlv: snd_kcontrol_tlv,
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub private_free: snd_kcontrol_private_free_t,
    pub vd: [snd_kcontrol_volatile; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub count: c_uint,
    pub info: snd_kcontrol_info_t,
    pub get: snd_kcontrol_get_t,
    pub put: snd_kcontrol_put_t,
    pub tlv: snd_kcontrol_tlv,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_file {
    pub list: list_head,
    pub events: list_head,
    pub change_sleep: wait_queue_head_t,
    pub read_lock: spinlock_t,
    pub card: *mut snd_card,
    pub subscribed: c_int,
    pub preferred_subdevice: [c_int; SND_CTL_SUBDEV_ITEMS],
    pub pid: *mut pid,
    pub fasync: *mut fasync_struct,
}

#[repr(C)]
pub struct snd_card {
    pub module: *mut module,
    pub dev: *mut device,
    pub ctl_dev: *mut device,
    pub number: c_int,
    pub shutdown: bool_,
    pub controls_rwlock: spinlock_t,
    pub controls_rwsem: rw_semaphore,
    pub ctl_files: list_head,
    pub controls: list_head,
    pub controls_count: c_uint,
    pub last_numid: c_uint,
    pub components: *const c_char,
    pub id: [c_char; 16],
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub mixername: [c_char; 80],
    pub user_ctl_alloc_size: ssize_t,
    pub value_buf: *mut snd_ctl_elem_value,
    pub ctl_numids: xarray,
    pub ctl_hash: xarray,
    pub ctl_hash_collision: bool_,
}

#[repr(C)]
pub struct snd_ctl_layer_ops {
    pub next: *mut snd_ctl_layer_ops,
    pub module_name: *const c_char,
    pub lnotify: unsafe extern "C" fn(*mut snd_card, c_uint, *mut snd_kcontrol, c_uint),
    pub lregister: unsafe extern "C" fn(*mut snd_card),
    pub ldisconnect: unsafe extern "C" fn(*mut snd_card),
}

#[repr(C)]
pub struct snd_kctl_event {
    pub list: list_head,
    pub id: snd_ctl_elem_id,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_ctl_card_info {
    pub card: c_int,
    pub id: [c_char; 16],
    pub driver: [c_char; 16],
    pub name: [c_char; 32],
    pub longname: [c_char; 80],
    pub mixername: [c_char; 80],
    pub components: [c_char; 128],
}

#[repr(C)]
pub struct snd_ctl_card_bytes {
    pub type_: c_uint,
    pub data: u64,
    pub data_allocated: c_uint,
    pub data_len: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_list {
    pub offset: c_uint,
    pub space: c_uint,
    pub used: c_uint,
    pub count: c_uint,
    pub pids: *mut snd_ctl_elem_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aes_iec958 {
    pub status: [u8; 24],
    pub subcode: [u8; 147],
    pub pad: u8,
    pub dig_subframe: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
    pub step: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer64 {
    pub min: i64,
    pub max: i64,
    pub step: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_enumerated {
    pub items: c_uint,
    pub item: c_uint,
    pub name: [c_char; 64],
    pub names_ptr: u64,
    pub names_length: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
    pub integer64: snd_ctl_elem_info_integer64,
    pub enumerated: snd_ctl_elem_info_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info {
    pub id: snd_ctl_elem_id,
    pub type_: c_int,
    pub access: c_uint,
    pub count: c_uint,
    pub owner: c_int,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer64 {
    pub value: [i64; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_ctl_elem_value_data {
    pub integer: snd_ctl_elem_value_integer,
    pub integer64: snd_ctl_elem_value_integer64,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub bytes: snd_ctl_elem_value_bytes,
    pub iec958: snd_aes_iec958,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_data,
}

#[repr(C)]
pub struct snd_ctl_event_elem {
    pub mask: c_uint,
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub union snd_ctl_event_data {
    pub elem: snd_ctl_event_elem,
}

#[repr(C)]
pub struct snd_ctl_event {
    pub type_: c_int,
    pub data: snd_ctl_event_data,
}

#[repr(C)]
pub struct snd_ctl_tlv {
    pub numid: c_uint,
    pub length: c_uint,
    pub tlv: *mut c_uint,
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
pub struct file_operations {
    pub owner: *mut module,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub fasync: Option<unsafe extern "C" fn(c_int, *mut file, c_int) -> c_int>,
}

pub type snd_kctl_ioctl_func_t =
    Option<unsafe extern "C" fn(*mut snd_card, *mut snd_ctl_file, c_uint, c_ulong) -> c_int>;

#[repr(C)]
pub struct snd_kctl_ioctl {
    pub list: list_head,
    pub fioctl: snd_kctl_ioctl_func_t,
}

#[repr(C)]
pub struct user_element {
    pub info: snd_ctl_elem_info,
    pub card: *mut snd_card,
    pub elem_data: *mut c_char,
    pub elem_data_size: c_ulong,
    pub tlv_data: *mut c_void,
    pub tlv_data_size: c_ulong,
    pub priv_data: *mut c_void,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static mut current: *mut task_struct;
    static mut snd_ioctl_rwsem: rw_semaphore;
    fn stream_open(inode: *mut inode, file: *mut file);
    fn iminor(inode: *mut inode) -> c_uint;
    fn snd_lookup_minor_data(minor: c_uint, ty: c_int) -> *mut snd_card;
    fn snd_card_file_add(card: *mut snd_card, file: *mut file) -> c_int;
    fn snd_card_file_remove(card: *mut snd_card, file: *mut file);
    fn try_module_get(module: *mut module) -> bool_;
    fn module_put(module: *mut module);
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kvfree(ptr: *mut c_void);
    fn snd_card_unref(card: *mut snd_card);
    fn snd_card_ref(card_number: c_int) -> *mut snd_card;
    fn get_pid(pid: *mut pid) -> *mut pid;
    fn put_pid(pid: *mut pid);
    fn task_pid(task: *mut task_struct) -> *mut pid;
    fn pid_vnr(pid: *mut pid) -> c_int;
    fn init_waitqueue_head(head: *mut wait_queue_head_t);
    fn wake_up(head: *mut wait_queue_head_t);
    fn init_waitqueue_entry(wait: *mut wait_queue_entry_t, task: *mut task_struct);
    fn add_wait_queue(head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn remove_wait_queue(head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn set_current_state(state: c_long);
    fn schedule();
    fn signal_pending(task: *mut task_struct) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn downgrade_write(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn snd_fasync_free(fasync: *mut fasync_struct);
    fn snd_fasync_helper(fd: c_int, file: *mut file, on: c_int, fasync: *mut *mut fasync_struct) -> c_int;
    fn snd_kill_fasync(fasync: *mut fasync_struct, sig: c_int, band: c_int);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn request_module(name: *const c_char) -> c_int;
    fn snd_register_device(ty: c_int, card: *mut snd_card, dev: c_int, fops: *const file_operations, data: *mut c_void, device: *mut device) -> c_int;
    fn snd_unregister_device(device: *mut device) -> c_int;
    fn put_device(device: *mut device);
    fn snd_device_alloc(devp: *mut *mut device, card: *mut snd_card) -> c_int;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn snd_device_new(card: *mut snd_card, ty: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_power_ref_and_wait(card: *mut snd_card) -> c_int;
    fn snd_power_unref(card: *mut snd_card);
    fn copy_to_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_ulong;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_ulong;
    fn memdup_user(src: *const c_void, n: size_t) -> *mut c_void;
    fn vmemdup_user(src: *const c_void, n: size_t) -> *mut c_void;
    fn u64_to_user_ptr(x: u64) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> ssize_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strnlen(s: *const c_char, max: size_t) -> size_t;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memset32(dst: *mut u32, val: u32, count: size_t);
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn div64_u64_rem(dividend: u64, divisor: u64, rem: *mut u64) -> u64;
    fn poll_wait(file: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table);
    fn xa_store_range(xa: *mut xarray, first: c_ulong, last: c_ulong, entry: *mut c_void, gfp: c_uint) -> c_int;
    fn xa_insert(xa: *mut xarray, index: c_ulong, entry: *mut c_void, gfp: c_uint) -> c_int;
    fn xa_erase(xa: *mut xarray, index: c_ulong) -> *mut c_void;
    fn xa_load(xa: *mut xarray, index: c_ulong) -> *mut c_void;
    fn xa_destroy(xa: *mut xarray);
}

static mut snd_ctl_layer_rwsem: rw_semaphore = rw_semaphore { _private: [] };
static mut snd_control_ioctls: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
// CONFIG_COMPAT: compat ioctl list exists only in compat builds.
static mut snd_control_compat_ioctls: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
static mut snd_ctl_layer: *mut snd_ctl_layer_ops = ptr::null_mut();

#[inline]
unsafe fn snd_BUG_ON(cond: bool_) -> bool_ {
    cond
}

#[inline]
unsafe fn snd_BUG() {}

#[inline]
unsafe fn WARN(cond: bool_, _fmt: *const c_char, _arg: *const c_char) -> bool_ {
    cond
}

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

#[inline]
unsafe fn list_empty(head: *const list_head) -> bool_ {
    (*head).next == head as *mut list_head
}

#[inline]
unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}

#[inline]
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    __list_add(new, (*head).prev, head);
}

#[inline]
unsafe fn list_del(entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
}

#[inline]
unsafe fn snd_kctl_event(ptr: *mut list_head) -> *mut snd_kctl_event {
    ptr as *mut snd_kctl_event
}

#[inline]
unsafe fn snd_kcontrol(ptr: *mut list_head) -> *mut snd_kcontrol {
    ptr as *mut snd_kcontrol
}

#[inline]
unsafe fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut user_element {
    (*kcontrol).private_data as *mut user_element
}

#[inline]
unsafe fn snd_ctl_get_ioff(kctl: *mut snd_kcontrol, id: *const snd_ctl_elem_id) -> c_uint {
    (*id).index.wrapping_sub((*kctl).id.index)
}

#[inline]
unsafe fn snd_ctl_build_ioff(id: *mut snd_ctl_elem_id, kctl: *mut snd_kcontrol, ioff: c_uint) {
    *id = (*kctl).id;
    (*id).index = (*kctl).id.index.wrapping_add(ioff);
    (*id).numid = (*kctl).id.numid.wrapping_add(ioff);
}

#[inline]
unsafe fn snd_ctl_skip_validation(info: *const snd_ctl_elem_info) -> bool_ {
    ((*info).access & SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK) != 0
}

#[inline]
fn DIV_ROUND_UP(n: size_t, d: size_t) -> size_t {
    (n + d - 1) / d
}

#[inline]
fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

#[inline]
unsafe fn put_user_int(value: c_int, ptr: *mut c_int) -> c_int {
    if ptr.is_null() {
        return 1;
    }
    *ptr = value;
    0
}

#[inline]
unsafe fn put_user_uint(value: c_uint, ptr: *mut c_uint) -> c_int {
    if ptr.is_null() {
        return 1;
    }
    *ptr = value;
    0
}

#[inline]
unsafe fn get_user_int(out: *mut c_int, ptr: *const c_int) -> c_int {
    if ptr.is_null() {
        return 1;
    }
    *out = *ptr;
    0
}

unsafe fn snd_ctl_remove_locked(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int {
    __snd_ctl_remove(card, kcontrol, true)
}

unsafe extern "C" fn snd_ctl_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut err: c_int;
    stream_open(inode, file);

    let card = snd_lookup_minor_data(iminor(inode), SNDRV_DEVICE_TYPE_CONTROL);
    if card.is_null() {
        return -ENODEV;
    }
    err = snd_card_file_add(card, file);
    if err < 0 {
        snd_card_unref(card);
        return err;
    }
    if !try_module_get((*card).module) {
        snd_card_file_remove(card, file);
        snd_card_unref(card);
        return -ENODEV;
    }
    let ctl = kzalloc(size_of::<snd_ctl_file>(), GFP_KERNEL) as *mut snd_ctl_file;
    if ctl.is_null() {
        module_put((*card).module);
        snd_card_file_remove(card, file);
        snd_card_unref(card);
        return -ENOMEM;
    }
    INIT_LIST_HEAD(&mut (*ctl).events);
    init_waitqueue_head(&mut (*ctl).change_sleep);
    spin_lock_init(&mut (*ctl).read_lock);
    (*ctl).card = card;
    for i in 0..SND_CTL_SUBDEV_ITEMS {
        (*ctl).preferred_subdevice[i] = -1;
    }
    (*ctl).pid = get_pid(task_pid(current));
    (*file).private_data = ctl as *mut c_void;
    list_add_tail(&mut (*ctl).list, &mut (*card).ctl_files);
    snd_card_unref(card);
    0
}

unsafe fn snd_ctl_empty_read_queue(ctl: *mut snd_ctl_file) {
    while !list_empty(&(*ctl).events) {
        let cread = snd_kctl_event((*ctl).events.next);
        list_del(&mut (*cread).list);
        kfree(cread as *mut c_void);
    }
}

unsafe extern "C" fn snd_ctl_release(_inode: *mut inode, file: *mut file) -> c_int {
    let ctl = (*file).private_data as *mut snd_ctl_file;
    (*file).private_data = ptr::null_mut();
    let card = (*ctl).card;

    list_del(&mut (*ctl).list);
    let mut pos = (*card).controls.next;
    while pos != &mut (*card).controls {
        let control = snd_kcontrol(pos);
        for idx in 0..(*control).count as usize {
            if (*control).vd.as_ptr().add(idx).read().owner == ctl {
                (*control).vd.as_mut_ptr().add(idx).write(snd_kcontrol_volatile {
                    access: (*control).vd.as_ptr().add(idx).read().access,
                    owner: ptr::null_mut(),
                });
            }
        }
        pos = (*pos).next;
    }

    snd_fasync_free((*ctl).fasync);
    snd_ctl_empty_read_queue(ctl);
    put_pid((*ctl).pid);
    kfree(ctl as *mut c_void);
    module_put((*card).module);
    snd_card_file_remove(card, file);
    0
}

/**
 * snd_ctl_notify - Send notification to user-space for a control change
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_notify(
    card: *mut snd_card,
    mask: c_uint,
    id: *mut snd_ctl_elem_id,
) {
    if snd_BUG_ON(card.is_null() || id.is_null()) || (*card).shutdown {
        return;
    }

    // IS_ENABLED(CONFIG_SND_MIXER_OSS): mixer_oss_change_count increment is omitted because
    // that field is supplied by the conditional external card layout.
    let mut ctl_pos = (*card).ctl_files.next;
    while ctl_pos != &mut (*card).ctl_files {
        let ctl = ctl_pos as *mut snd_ctl_file;
        if (*ctl).subscribed != 0 {
            let mut ev_pos = (*ctl).events.next;
            let mut found = false;
            while ev_pos != &mut (*ctl).events {
                let ev = snd_kctl_event(ev_pos);
                if (*ev).id.numid == (*id).numid {
                    (*ev).mask |= mask;
                    found = true;
                    break;
                }
                ev_pos = (*ev_pos).next;
            }
            if !found {
                let ev = kzalloc(size_of::<snd_kctl_event>(), GFP_ATOMIC) as *mut snd_kctl_event;
                if !ev.is_null() {
                    (*ev).id = *id;
                    (*ev).mask = mask;
                    list_add_tail(&mut (*ev).list, &mut (*ctl).events);
                } else {
                    dev_err((*card).dev, c"No memory available to allocate event\n".as_ptr());
                }
            }
            wake_up(&mut (*ctl).change_sleep);
            snd_kill_fasync((*ctl).fasync, SIGIO, POLL_IN);
        }
        ctl_pos = (*ctl_pos).next;
    }
}

/**
 * snd_ctl_notify_one - Send notification to user-space for a control change
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_notify_one(
    card: *mut snd_card,
    mask: c_uint,
    kctl: *mut snd_kcontrol,
    ioff: c_uint,
) {
    let mut id = (*kctl).id;
    id.index = id.index.wrapping_add(ioff);
    id.numid = id.numid.wrapping_add(ioff);
    snd_ctl_notify(card, mask, &mut id);
    let mut lops = snd_ctl_layer;
    while !lops.is_null() {
        ((*lops).lnotify)(card, mask, kctl, ioff);
        lops = (*lops).next;
    }
}

unsafe fn snd_ctl_new(
    kctl: *mut *mut snd_kcontrol,
    count: c_uint,
    access: c_uint,
    file: *mut snd_ctl_file,
) -> c_int {
    if count == 0 || count > MAX_CONTROL_COUNT {
        return -EINVAL;
    }
    let alloc = size_of::<snd_kcontrol>() + size_of::<snd_kcontrol_volatile>() * count as usize;
    *kctl = kzalloc(alloc, GFP_KERNEL) as *mut snd_kcontrol;
    if (*kctl).is_null() {
        return -ENOMEM;
    }
    (**kctl).count = count;
    for idx in 0..count as usize {
        let vd = (**kctl).vd.as_mut_ptr().add(idx);
        (*vd).access = access;
        (*vd).owner = file;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_new1(
    ncontrol: *const snd_kcontrol_new,
    private_data: *mut c_void,
) -> *mut snd_kcontrol {
    if snd_BUG_ON(ncontrol.is_null() || (*ncontrol).info.is_none()) {
        return ptr::null_mut();
    }
    let mut count = (*ncontrol).count;
    if count == 0 {
        count = 1;
    }
    let mut access = (*ncontrol).access;
    if access == 0 {
        access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
    }
    access &= SNDRV_CTL_ELEM_ACCESS_READWRITE
        | SNDRV_CTL_ELEM_ACCESS_VOLATILE
        | SNDRV_CTL_ELEM_ACCESS_INACTIVE
        | SNDRV_CTL_ELEM_ACCESS_TLV_READWRITE
        | SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND
        | SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK
        | SNDRV_CTL_ELEM_ACCESS_LED_MASK
        | SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK;

    let mut kctl: *mut snd_kcontrol = ptr::null_mut();
    if snd_ctl_new(&mut kctl, count, access, ptr::null_mut()) < 0 {
        return ptr::null_mut();
    }
    (*kctl).id.iface = (*ncontrol).iface;
    (*kctl).id.device = (*ncontrol).device;
    (*kctl).id.subdevice = (*ncontrol).subdevice;
    if !(*ncontrol).name.is_null() {
        strscpy((*kctl).id.name.as_mut_ptr(), (*ncontrol).name, (*kctl).id.name.len());
        if strcmp((*ncontrol).name, (*kctl).id.name.as_ptr()) != 0 {
            pr_warn(
                c"ALSA: Control name '%s' truncated to '%s'\n".as_ptr(),
                (*ncontrol).name,
                (*kctl).id.name.as_ptr(),
            );
        }
    }
    (*kctl).id.index = (*ncontrol).index;
    (*kctl).info = (*ncontrol).info;
    (*kctl).get = (*ncontrol).get;
    (*kctl).put = (*ncontrol).put;
    (*kctl).tlv.p = (*ncontrol).tlv.p;
    (*kctl).private_value = (*ncontrol).private_value;
    (*kctl).private_data = private_data;
    kctl
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_free_one(kcontrol: *mut snd_kcontrol) {
    if !kcontrol.is_null() {
        if let Some(private_free) = (*kcontrol).private_free {
            private_free(kcontrol);
        }
        kfree(kcontrol as *mut c_void);
    }
}

unsafe fn snd_ctl_remove_numid_conflict(card: *mut snd_card, count: c_uint) -> bool_ {
    if (*card).last_numid >= UINT_MAX - count {
        (*card).last_numid = 0;
    }
    let mut pos = (*card).controls.next;
    while pos != &mut (*card).controls {
        let kctl = snd_kcontrol(pos);
        if (*kctl).id.numid < (*card).last_numid + 1 + count
            && (*kctl).id.numid + (*kctl).count > (*card).last_numid + 1
        {
            (*card).last_numid = (*kctl).id.numid + (*kctl).count - 1;
            return true;
        }
        pos = (*pos).next;
    }
    false
}

unsafe fn snd_ctl_find_hole(card: *mut snd_card, count: c_uint) -> c_int {
    let mut iter: c_uint = 100000;
    while snd_ctl_remove_numid_conflict(card, count) {
        iter -= 1;
        if iter == 0 {
            dev_err((*card).dev, c"unable to allocate new control numid\n".as_ptr());
            return -ENOMEM;
        }
    }
    0
}

unsafe fn elem_id_matches(kctl: *const snd_kcontrol, id: *const snd_ctl_elem_id) -> bool_ {
    (*kctl).id.iface == (*id).iface
        && (*kctl).id.device == (*id).device
        && (*kctl).id.subdevice == (*id).subdevice
        && strncmp(
            (*kctl).id.name.as_ptr(),
            (*id).name.as_ptr(),
            (*kctl).id.name.len(),
        ) == 0
        && (*kctl).id.index <= (*id).index
        && (*kctl).id.index + (*kctl).count > (*id).index
}

unsafe fn get_ctl_id_hash(id: *const snd_ctl_elem_id) -> c_ulong {
    let mut h = (*id).iface as c_ulong;
    h = MULTIPLIER * h + (*id).device as c_ulong;
    h = MULTIPLIER * h + (*id).subdevice as c_ulong;
    let mut i = 0;
    while i < SNDRV_CTL_ELEM_ID_NAME_MAXLEN && (*id).name[i] != 0 {
        h = MULTIPLIER * h + (*id).name[i] as c_ulong;
        i += 1;
    }
    h = MULTIPLIER * h + (*id).index as c_ulong;
    h &= LONG_MAX;
    h
}

unsafe fn add_hash_entries(card: *mut snd_card, kcontrol: *mut snd_kcontrol) {
    let mut id = (*kcontrol).id;
    xa_store_range(
        &mut (*card).ctl_numids,
        (*kcontrol).id.numid as c_ulong,
        ((*kcontrol).id.numid + (*kcontrol).count - 1) as c_ulong,
        kcontrol as *mut c_void,
        GFP_KERNEL,
    );
    for i in 0..(*kcontrol).count {
        id.index = (*kcontrol).id.index + i;
        if xa_insert(
            &mut (*card).ctl_hash,
            get_ctl_id_hash(&id),
            kcontrol as *mut c_void,
            GFP_KERNEL,
        ) != 0
        {
            (*card).ctl_hash_collision = true;
            dev_dbg(
                (*card).dev,
                c"ctl_hash collision %d:%s:%d\n".as_ptr(),
                id.iface,
                id.name.as_ptr(),
                id.index,
            );
        }
    }
}

unsafe fn remove_hash_entries(card: *mut snd_card, kcontrol: *mut snd_kcontrol) {
    let mut id = (*kcontrol).id;
    for _ in 0..(*kcontrol).count {
        xa_erase(&mut (*card).ctl_numids, id.numid as c_ulong);
        let h = get_ctl_id_hash(&id);
        let matched = xa_load(&mut (*card).ctl_hash, h) as *mut snd_kcontrol;
        if !matched.is_null() && (matched == kcontrol || elem_id_matches(matched, &id)) {
            xa_erase(&mut (*card).ctl_hash, h);
        }
        id.index += 1;
        id.numid += 1;
    }
}

#[repr(C)]
enum snd_ctl_add_mode {
    CTL_ADD_EXCLUSIVE,
    CTL_REPLACE,
    CTL_ADD_ON_REPLACE,
}

unsafe fn __snd_ctl_add_replace(
    card: *mut snd_card,
    kcontrol: *mut snd_kcontrol,
    mode: snd_ctl_add_mode,
) -> c_int {
    let id = (*kcontrol).id;
    if id.index > UINT_MAX - (*kcontrol).count {
        return -EINVAL;
    }
    let old = snd_ctl_find_id(card, &id);
    if old.is_null() {
        if matches!(mode, snd_ctl_add_mode::CTL_REPLACE) {
            return -EINVAL;
        }
    } else {
        if matches!(mode, snd_ctl_add_mode::CTL_ADD_EXCLUSIVE) {
            dev_err(
                (*card).dev,
                c"control %i:%i:%i:%s:%i is already present\n".as_ptr(),
                id.iface,
                id.device,
                id.subdevice,
                id.name.as_ptr(),
                id.index,
            );
            return -EBUSY;
        }
        let err = snd_ctl_remove_locked(card, old);
        if err < 0 {
            return err;
        }
    }
    if snd_ctl_find_hole(card, (*kcontrol).count) < 0 {
        return -ENOMEM;
    }
    list_add_tail(&mut (*kcontrol).list, &mut (*card).controls);
    (*card).controls_count += (*kcontrol).count;
    (*kcontrol).id.numid = (*card).last_numid + 1;
    (*card).last_numid += (*kcontrol).count;
    add_hash_entries(card, kcontrol);
    for idx in 0..(*kcontrol).count {
        snd_ctl_notify_one(card, SNDRV_CTL_EVENT_MASK_ADD, kcontrol, idx);
    }
    0
}

unsafe fn snd_ctl_add_replace(
    card: *mut snd_card,
    kcontrol: *mut snd_kcontrol,
    mode: snd_ctl_add_mode,
) -> c_int {
    if kcontrol.is_null() {
        return -EINVAL;
    }
    if snd_BUG_ON(card.is_null() || (*kcontrol).info.is_none()) {
        snd_ctl_free_one(kcontrol);
        return -EINVAL;
    }
    let err = __snd_ctl_add_replace(card, kcontrol, mode);
    if err < 0 {
        snd_ctl_free_one(kcontrol);
        return err;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int {
    snd_ctl_add_replace(card, kcontrol, snd_ctl_add_mode::CTL_ADD_EXCLUSIVE)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_replace(
    card: *mut snd_card,
    kcontrol: *mut snd_kcontrol,
    add_on_replace: bool_,
) -> c_int {
    snd_ctl_add_replace(
        card,
        kcontrol,
        if add_on_replace {
            snd_ctl_add_mode::CTL_ADD_ON_REPLACE
        } else {
            snd_ctl_add_mode::CTL_REPLACE
        },
    )
}

unsafe fn __snd_ctl_remove(
    card: *mut snd_card,
    kcontrol: *mut snd_kcontrol,
    remove_hash: bool_,
) -> c_int {
    if snd_BUG_ON(card.is_null() || kcontrol.is_null()) {
        return -EINVAL;
    }
    if remove_hash {
        remove_hash_entries(card, kcontrol);
    }
    list_del(&mut (*kcontrol).list);
    (*card).controls_count -= (*kcontrol).count;
    for idx in 0..(*kcontrol).count {
        snd_ctl_notify_one(card, SNDRV_CTL_EVENT_MASK_REMOVE, kcontrol, idx);
    }
    snd_ctl_free_one(kcontrol);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int {
    if kcontrol.is_null() {
        return 0;
    }
    snd_ctl_remove_locked(card, kcontrol)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int {
    let kctl = snd_ctl_find_id(card, id);
    if kctl.is_null() {
        return -ENOENT;
    }
    snd_ctl_remove_locked(card, kctl)
}

unsafe fn snd_ctl_remove_user_ctl(file: *mut snd_ctl_file, id: *mut snd_ctl_elem_id) -> c_int {
    let card = (*file).card;
    let kctl = snd_ctl_find_id(card, id);
    if kctl.is_null() {
        return -ENOENT;
    }
    if ((*(*kctl).vd.as_ptr()).access & SNDRV_CTL_ELEM_ACCESS_USER) == 0 {
        return -EINVAL;
    }
    for idx in 0..(*kctl).count as usize {
        let owner = (*(*kctl).vd.as_ptr().add(idx)).owner;
        if !owner.is_null() && owner != file {
            return -EBUSY;
        }
    }
    snd_ctl_remove_locked(card, kctl)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_activate_id(
    card: *mut snd_card,
    id: *mut snd_ctl_elem_id,
    active: c_int,
) -> c_int {
    down_write(&mut (*card).controls_rwsem);
    let kctl = snd_ctl_find_id(card, id);
    if kctl.is_null() {
        up_write(&mut (*card).controls_rwsem);
        return -ENOENT;
    }
    let index_offset = snd_ctl_get_ioff(kctl, id);
    let vd = (*kctl).vd.as_mut_ptr().add(index_offset as usize);
    if active != 0 {
        if ((*vd).access & SNDRV_CTL_ELEM_ACCESS_INACTIVE) == 0 {
            up_write(&mut (*card).controls_rwsem);
            return 0;
        }
        (*vd).access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    } else {
        if ((*vd).access & SNDRV_CTL_ELEM_ACCESS_INACTIVE) != 0 {
            up_write(&mut (*card).controls_rwsem);
            return 0;
        }
        (*vd).access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    }
    snd_ctl_build_ioff(id, kctl, index_offset);
    downgrade_write(&mut (*card).controls_rwsem);
    snd_ctl_notify_one(card, SNDRV_CTL_EVENT_MASK_INFO, kctl, index_offset);
    up_read(&mut (*card).controls_rwsem);
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_rename_id(
    card: *mut snd_card,
    src_id: *mut snd_ctl_elem_id,
    dst_id: *mut snd_ctl_elem_id,
) -> c_int {
    let kctl = snd_ctl_find_id(card, src_id);
    if kctl.is_null() {
        return -ENOENT;
    }
    let saved_numid = (*kctl).id.numid;
    remove_hash_entries(card, kctl);
    (*kctl).id = *dst_id;
    (*kctl).id.numid = saved_numid;
    add_hash_entries(card, kctl);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_rename(
    card: *mut snd_card,
    kctl: *mut snd_kcontrol,
    name: *const c_char,
) {
    remove_hash_entries(card, kctl);
    if strscpy((*kctl).id.name.as_mut_ptr(), name, (*kctl).id.name.len()) < 0 {
        pr_warn(
            c"ALSA: Renamed control new name '%s' truncated to '%s'\n".as_ptr(),
            name,
            (*kctl).id.name.as_ptr(),
        );
    }
    add_hash_entries(card, kctl);
}

unsafe fn snd_ctl_find_numid_slow(card: *mut snd_card, numid: c_uint) -> *mut snd_kcontrol {
    let mut pos = (*card).controls.next;
    while pos != &mut (*card).controls {
        let kctl = snd_kcontrol(pos);
        if (*kctl).id.numid <= numid && (*kctl).id.numid + (*kctl).count > numid {
            return kctl;
        }
        pos = (*pos).next;
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_find_numid(card: *mut snd_card, numid: c_uint) -> *mut snd_kcontrol {
    if snd_BUG_ON(card.is_null() || numid == 0) {
        return ptr::null_mut();
    }
    let found = xa_load(&mut (*card).ctl_numids, numid as c_ulong) as *mut snd_kcontrol;
    if !found.is_null() {
        found
    } else {
        snd_ctl_find_numid_slow(card, numid)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_find_id(
    card: *mut snd_card,
    id: *const snd_ctl_elem_id,
) -> *mut snd_kcontrol {
    if snd_BUG_ON(card.is_null() || id.is_null()) {
        return ptr::null_mut();
    }
    if (*id).numid != 0 {
        return snd_ctl_find_numid(card, (*id).numid);
    }
    let kctl = xa_load(&mut (*card).ctl_hash, get_ctl_id_hash(id)) as *mut snd_kcontrol;
    if !kctl.is_null() && elem_id_matches(kctl, id) {
        return kctl;
    }
    if !(*card).ctl_hash_collision {
        return ptr::null_mut();
    }
    let mut pos = (*card).controls.next;
    while pos != &mut (*card).controls {
        let kctl2 = snd_kcontrol(pos);
        if elem_id_matches(kctl2, id) {
            return kctl2;
        }
        pos = (*pos).next;
    }
    ptr::null_mut()
}

unsafe fn snd_ctl_card_info(
    card: *mut snd_card,
    _ctl: *mut snd_ctl_file,
    _cmd: c_uint,
    arg: *mut c_void,
) -> c_int {
    let info = kzalloc(size_of::<snd_ctl_card_info>(), GFP_KERNEL) as *mut snd_ctl_card_info;
    if info.is_null() {
        return -ENOMEM;
    }
    let mut components = (*card).components;
    if components.is_null() {
        components = c"".as_ptr();
    }
    (*info).card = (*card).number;
    strscpy((*info).id.as_mut_ptr(), (*card).id.as_ptr(), (*info).id.len());
    strscpy((*info).driver.as_mut_ptr(), (*card).driver.as_ptr(), (*info).driver.len());
    strscpy((*info).name.as_mut_ptr(), (*card).shortname.as_ptr(), (*info).name.len());
    strscpy((*info).longname.as_mut_ptr(), (*card).longname.as_ptr(), (*info).longname.len());
    strscpy((*info).mixername.as_mut_ptr(), (*card).mixername.as_ptr(), (*info).mixername.len());
    let n = strscpy((*info).components.as_mut_ptr(), components, (*info).components.len());
    if n < 0 {
        (*info).components[(*info).components.len() - 2] = b'>' as c_char;
    }
    let ret = if copy_to_user(arg, info as *const c_void, size_of::<snd_ctl_card_info>()) != 0 {
        -EFAULT
    } else {
        0
    };
    kfree(info as *mut c_void);
    ret
}

unsafe fn snd_ctl_card_bytes(
    card: *mut snd_card,
    info: *mut snd_ctl_card_bytes,
    data_len_out: *mut c_uint,
) -> c_int {
    let data_len: c_uint;
    match (*info).type_ {
        SND_CTL_CARD_BTYPE_COMPONENTS => {
            let mut components = (*card).components;
            if components.is_null() {
                components = c"".as_ptr();
            }
            data_len = (strlen(components) + 1) as c_uint;
            if !((*info).data == 0 || (*info).data_allocated == 0) {
                if (*info).data_allocated < data_len {
                    return -ENOMEM;
                }
                if copy_to_user(u64_to_user_ptr((*info).data), components as *const c_void, data_len as usize) != 0 {
                    return -EFAULT;
                }
            }
        }
        _ => return -EINVAL,
    }
    if put_user_uint(data_len, data_len_out) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_ctl_card_bytes_user(card: *mut snd_card, _info: *mut snd_ctl_card_bytes) -> c_int {
    let mut info: snd_ctl_card_bytes = mem::zeroed();
    if copy_from_user(&mut info as *mut _ as *mut c_void, _info as *const c_void, size_of::<snd_ctl_card_bytes>()) != 0 {
        return -EFAULT;
    }
    snd_ctl_card_bytes(card, &mut info, &mut (*_info).data_len)
}

unsafe fn snd_ctl_elem_list(card: *mut snd_card, list: *mut snd_ctl_elem_list) -> c_int {
    let mut offset = (*list).offset;
    let mut space = (*list).space;
    (*list).count = (*card).controls_count;
    (*list).used = 0;
    if space == 0 {
        return 0;
    }
    let mut pos = (*card).controls.next;
    while pos != &mut (*card).controls {
        let kctl = snd_kcontrol(pos);
        if offset >= (*kctl).count {
            offset -= (*kctl).count;
            pos = (*pos).next;
            continue;
        }
        let mut jidx = offset;
        while jidx < (*kctl).count {
            let mut id: snd_ctl_elem_id = mem::zeroed();
            snd_ctl_build_ioff(&mut id, kctl, jidx);
            if copy_to_user((*list).pids.add((*list).used as usize) as *mut c_void, &id as *const _ as *const c_void, size_of::<snd_ctl_elem_id>()) != 0 {
                return -EFAULT;
            }
            (*list).used += 1;
            space -= 1;
            if space == 0 {
                return 0;
            }
            jidx += 1;
        }
        offset = 0;
        pos = (*pos).next;
    }
    0
}

unsafe fn snd_ctl_elem_list_user(card: *mut snd_card, _list: *mut snd_ctl_elem_list) -> c_int {
    let mut list: snd_ctl_elem_list = mem::zeroed();
    if copy_from_user(&mut list as *mut _ as *mut c_void, _list as *const c_void, size_of::<snd_ctl_elem_list>()) != 0 {
        return -EFAULT;
    }
    let err = snd_ctl_elem_list(card, &mut list);
    if err != 0 {
        return err;
    }
    if copy_to_user(_list as *mut c_void, &list as *const _ as *const c_void, size_of::<snd_ctl_elem_list>()) != 0 {
        return -EFAULT;
    }
    0
}

static max_value_counts: [c_uint; 7] = [0, 128, 128, 128, 512, 1, 64];
static value_sizes: [c_uint; 7] = [
    0,
    size_of::<c_long>() as c_uint,
    size_of::<c_long>() as c_uint,
    size_of::<c_uint>() as c_uint,
    size_of::<u8>() as c_uint,
    size_of::<snd_aes_iec958>() as c_uint,
    size_of::<i64>() as c_uint,
];

unsafe fn snd_ctl_check_elem_info(card: *mut snd_card, info: *const snd_ctl_elem_info) -> c_int {
    let ty = (*info).type_ as usize;
    if ty < SNDRV_CTL_ELEM_TYPE_BOOLEAN || ty > SNDRV_CTL_ELEM_TYPE_INTEGER64 {
        if !card.is_null() {
            dev_err((*card).dev, c"control %i:%i:%i:%s:%i: invalid type %d\n".as_ptr(), (*info).id.iface, (*info).id.device, (*info).id.subdevice, (*info).id.name.as_ptr(), (*info).id.index, (*info).type_);
        }
        return -EINVAL;
    }
    if ty == SNDRV_CTL_ELEM_TYPE_ENUMERATED && (*info).value.enumerated.items == 0 {
        if !card.is_null() {
            dev_err((*card).dev, c"control %i:%i:%i:%s:%i: zero enum items\n".as_ptr(), (*info).id.iface, (*info).id.device, (*info).id.subdevice, (*info).id.name.as_ptr(), (*info).id.index);
        }
        return -EINVAL;
    }
    if (*info).count > max_value_counts[ty] {
        if !card.is_null() {
            dev_err((*card).dev, c"control %i:%i:%i:%s:%i: invalid count %d\n".as_ptr(), (*info).id.iface, (*info).id.device, (*info).id.subdevice, (*info).id.name.as_ptr(), (*info).id.index, (*info).count);
        }
        return -EINVAL;
    }
    0
}

unsafe fn fill_remaining_elem_value(control: *mut snd_ctl_elem_value, info: *mut snd_ctl_elem_info, pattern: u32) {
    let mut offset = value_sizes[(*info).type_ as usize] as usize * (*info).count as usize;
    offset = DIV_ROUND_UP(offset, size_of::<u32>());
    let base = (*control).value.bytes.data.as_mut_ptr() as *mut u32;
    memset32(base.add(offset), pattern, size_of::<snd_ctl_elem_value_data>() / size_of::<u32>() - offset);
}

unsafe fn sanity_check_int_value(
    card: *mut snd_card,
    control: *const snd_ctl_elem_value,
    info: *const snd_ctl_elem_info,
    i: c_int,
    print_error: bool_,
) -> c_int {
    let (lval, lmin, lmax, lstep): (i64, i64, i64, i64) = match (*info).type_ as usize {
        SNDRV_CTL_ELEM_TYPE_INTEGER => ((*control).value.integer.value[i as usize] as i64, (*info).value.integer.min as i64, (*info).value.integer.max as i64, (*info).value.integer.step as i64),
        SNDRV_CTL_ELEM_TYPE_INTEGER64 => ((*control).value.integer64.value[i as usize], (*info).value.integer64.min, (*info).value.integer64.max, (*info).value.integer64.step),
        SNDRV_CTL_ELEM_TYPE_ENUMERATED => ((*control).value.enumerated.item[i as usize] as i64, 0, ((*info).value.enumerated.items - 1) as i64, 0),
        _ => ((*control).value.integer.value[i as usize] as i64, 0, 1, 0),
    };
    if lval < lmin || lval > lmax {
        if print_error {
            dev_err((*card).dev, c"control %i:%i:%i:%s:%i: value out of range %lld (%lld/%lld) at count %i\n".as_ptr(), (*control).id.iface, (*control).id.device, (*control).id.subdevice, (*control).id.name.as_ptr(), (*control).id.index, lval, lmin, lmax, i);
        }
        return -EINVAL;
    }
    if lstep != 0 {
        let mut rem: u64 = 0;
        div64_u64_rem(lval as u64, lstep as u64, &mut rem);
        if rem != 0 {
            if print_error {
                dev_err((*card).dev, c"control %i:%i:%i:%s:%i: unaligned value %lld (step %lld) at count %i\n".as_ptr(), (*control).id.iface, (*control).id.device, (*control).id.subdevice, (*control).id.name.as_ptr(), (*control).id.index, lval, lstep, i);
            }
            return -EINVAL;
        }
    }
    0
}

unsafe fn sanity_check_input_values(
    card: *mut snd_card,
    control: *const snd_ctl_elem_value,
    info: *const snd_ctl_elem_info,
    print_error: bool_,
) -> c_int {
    match (*info).type_ as usize {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
        | SNDRV_CTL_ELEM_TYPE_INTEGER
        | SNDRV_CTL_ELEM_TYPE_INTEGER64
        | SNDRV_CTL_ELEM_TYPE_ENUMERATED => {
            for i in 0..(*info).count as c_int {
                let ret = sanity_check_int_value(card, control, info, i, print_error);
                if ret < 0 {
                    return ret;
                }
            }
        }
        _ => {}
    }
    0
}

unsafe fn sanity_check_elem_value(
    card: *mut snd_card,
    control: *const snd_ctl_elem_value,
    info: *const snd_ctl_elem_info,
    pattern: u32,
) -> c_int {
    let mut ret = sanity_check_input_values(card, control, info, true);
    if ret < 0 {
        return ret;
    }
    let mut offset = value_sizes[(*info).type_ as usize] as usize * (*info).count as usize;
    offset = DIV_ROUND_UP(offset, size_of::<u32>());
    let mut p = (*control).value.bytes.data.as_ptr().add(offset * size_of::<u32>()) as *mut u32;
    while offset < size_of::<snd_ctl_elem_value_data>() / size_of::<u32>() {
        if *p != pattern {
            ret = -EINVAL;
            break;
        }
        *p = 0;
        offset += 1;
        p = p.add(1);
    }
    ret
}

unsafe fn __snd_ctl_elem_info(
    card: *mut snd_card,
    kctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
    ctl: *mut snd_ctl_file,
) -> c_int {
    let mut result = (*kctl).info.unwrap()(kctl, info);
    if result >= 0 {
        snd_BUG_ON((*info).access != 0);
        let index_offset = snd_ctl_get_ioff(kctl, &(*info).id);
        let vd = (*kctl).vd.as_mut_ptr().add(index_offset as usize);
        snd_ctl_build_ioff(&mut (*info).id, kctl, index_offset);
        (*info).access = (*vd).access;
        if !(*vd).owner.is_null() {
            (*info).access |= SNDRV_CTL_ELEM_ACCESS_LOCK;
            if (*vd).owner == ctl {
                (*info).access |= SNDRV_CTL_ELEM_ACCESS_OWNER;
            }
            (*info).owner = pid_vnr((*(*vd).owner).pid);
        } else {
            (*info).owner = -1;
        }
        if !snd_ctl_skip_validation(info) && snd_ctl_check_elem_info(card, info) < 0 {
            result = -EINVAL;
        }
    }
    result
}

unsafe fn snd_ctl_elem_info(ctl: *mut snd_ctl_file, info: *mut snd_ctl_elem_info) -> c_int {
    let card = (*ctl).card;
    let kctl = snd_ctl_find_id(card, &(*info).id);
    if kctl.is_null() {
        return -ENOENT;
    }
    __snd_ctl_elem_info(card, kctl, info, ctl)
}

unsafe fn snd_ctl_elem_info_user(ctl: *mut snd_ctl_file, _info: *mut snd_ctl_elem_info) -> c_int {
    let card = (*ctl).card;
    let mut info: snd_ctl_elem_info = mem::zeroed();
    if copy_from_user(&mut info as *mut _ as *mut c_void, _info as *const c_void, size_of::<snd_ctl_elem_info>()) != 0 {
        return -EFAULT;
    }
    let mut result = snd_power_ref_and_wait(card);
    if result != 0 {
        return result;
    }
    result = snd_ctl_elem_info(ctl, &mut info);
    snd_power_unref(card);
    if result < 0 {
        return result;
    }
    info.access &= !(SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK | SNDRV_CTL_ELEM_ACCESS_LED_MASK);
    if copy_to_user(_info as *mut c_void, &info as *const _ as *const c_void, size_of::<snd_ctl_elem_info>()) != 0 {
        return -EFAULT;
    }
    result
}

unsafe fn snd_ctl_elem_read(card: *mut snd_card, control: *mut snd_ctl_elem_value) -> c_int {
    let kctl = snd_ctl_find_id(card, &(*control).id);
    if kctl.is_null() {
        return -ENOENT;
    }
    let index_offset = snd_ctl_get_ioff(kctl, &(*control).id);
    let vd = (*kctl).vd.as_mut_ptr().add(index_offset as usize);
    if ((*vd).access & SNDRV_CTL_ELEM_ACCESS_READ) == 0 || (*kctl).get.is_none() {
        return -EPERM;
    }
    snd_ctl_build_ioff(&mut (*control).id, kctl, index_offset);
    let mut info: snd_ctl_elem_info = mem::zeroed();
    info.id = (*control).id;
    let pattern: u32 = 0xdeadbeef;
    let ret_info = __snd_ctl_elem_info(card, kctl, &mut info, ptr::null_mut());
    if ret_info < 0 {
        return ret_info;
    }
    if !snd_ctl_skip_validation(&info) {
        fill_remaining_elem_value(control, &mut info, pattern);
    }
    let ret = (*kctl).get.unwrap()(kctl, control);
    if ret < 0 {
        return ret;
    }
    if !snd_ctl_skip_validation(&info) && sanity_check_elem_value(card, control, &info, pattern) < 0 {
        dev_err((*card).dev, c"control %i:%i:%i:%s:%i: access overflow\n".as_ptr(), (*control).id.iface, (*control).id.device, (*control).id.subdevice, (*control).id.name.as_ptr(), (*control).id.index);
        return -EINVAL;
    }
    0
}

unsafe fn snd_ctl_elem_read_user(card: *mut snd_card, _control: *mut snd_ctl_elem_value) -> c_int {
    let control = memdup_user(_control as *const c_void, size_of::<snd_ctl_elem_value>()) as *mut snd_ctl_elem_value;
    if control.is_null() {
        return -ENOMEM;
    }
    let mut result = snd_power_ref_and_wait(card);
    if result != 0 {
        kfree(control as *mut c_void);
        return result;
    }
    result = snd_ctl_elem_read(card, control);
    snd_power_unref(card);
    if result >= 0 && copy_to_user(_control as *mut c_void, control as *const c_void, size_of::<snd_ctl_elem_value>()) != 0 {
        result = -EFAULT;
    }
    kfree(control as *mut c_void);
    result
}

static snd_ctl_elem_iface_names: [*const c_char; 7] = [
    c"CARD".as_ptr(),
    c"HWDEP".as_ptr(),
    c"MIXER".as_ptr(),
    c"PCM".as_ptr(),
    c"RAWMIDI".as_ptr(),
    c"TIMER".as_ptr(),
    c"SEQUENCER".as_ptr(),
];

unsafe fn trace_snd_ctl_put(
    _id: *const snd_ctl_elem_id,
    _iname: *const c_char,
    _card: c_int,
    _expected: c_int,
    _actual: c_int,
) {
}

unsafe fn snd_ctl_put_verify(
    card: *mut snd_card,
    kctl: *mut snd_kcontrol,
    control: *mut snd_ctl_elem_value,
) -> c_int {
    let original = (*card).value_buf;
    let mut info: snd_ctl_elem_info = mem::zeroed();
    memset(original as *mut c_void, 0, size_of::<snd_ctl_elem_value>());
    let mut ret = (*kctl).info.unwrap()(kctl, &mut info);
    if ret != 0 {
        return ret;
    }
    ret = (*kctl).get.unwrap()(kctl, original);
    if ret != 0 {
        return ret;
    }
    ret = (*kctl).put.unwrap()(kctl, control);
    if ret < 0 {
        return ret;
    }
    fill_remaining_elem_value(control, &mut info, 0);
    let mut retcmp = memcmp(
        &(*original).value as *const _ as *const c_void,
        &(*control).value as *const _ as *const c_void,
        size_of::<snd_ctl_elem_value_data>(),
    );
    if retcmp != 0 {
        retcmp = 1;
    }
    let iname = snd_ctl_elem_iface_names[(*kctl).id.iface as usize];
    trace_snd_ctl_put(&(*kctl).id, iname, (*card).number, ret, retcmp);
    ret
}

unsafe fn snd_ctl_put(
    card: *mut snd_card,
    kctl: *mut snd_kcontrol,
    control: *mut snd_ctl_elem_value,
    access: c_uint,
) -> c_int {
    if (access & SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK) != 0
        || (access & SNDRV_CTL_ELEM_ACCESS_VOLATILE) != 0
    {
        return (*kctl).put.unwrap()(kctl, control);
    }
    snd_ctl_put_verify(card, kctl, control)
}

unsafe fn snd_ctl_elem_write(
    card: *mut snd_card,
    file: *mut snd_ctl_file,
    control: *mut snd_ctl_elem_value,
) -> c_int {
    down_write(&mut (*card).controls_rwsem);
    let kctl = snd_ctl_find_id(card, &(*control).id);
    if kctl.is_null() {
        up_write(&mut (*card).controls_rwsem);
        return -ENOENT;
    }
    let index_offset = snd_ctl_get_ioff(kctl, &(*control).id);
    let vd = (*kctl).vd.as_mut_ptr().add(index_offset as usize);
    if ((*vd).access & SNDRV_CTL_ELEM_ACCESS_WRITE) == 0
        || (*kctl).put.is_none()
        || (!file.is_null() && !(*vd).owner.is_null() && (*vd).owner != file)
    {
        up_write(&mut (*card).controls_rwsem);
        return -EPERM;
    }
    snd_ctl_build_ioff(&mut (*control).id, kctl, index_offset);
    let mut result = 0;
    // IS_ENABLED(CONFIG_SND_CTL_INPUT_VALIDATION)
    {
        let mut info: snd_ctl_elem_info = mem::zeroed();
        info.id = (*control).id;
        result = __snd_ctl_elem_info(card, kctl, &mut info, ptr::null_mut());
        if result == 0 {
            result = sanity_check_input_values(card, control, &info, false);
        }
    }
    if result == 0 {
        result = snd_ctl_put(card, kctl, control, (*vd).access);
    }
    if result < 0 {
        up_write(&mut (*card).controls_rwsem);
        return result;
    }
    if result > 0 {
        downgrade_write(&mut (*card).controls_rwsem);
        snd_ctl_notify_one(card, SNDRV_CTL_EVENT_MASK_VALUE, kctl, index_offset);
        up_read(&mut (*card).controls_rwsem);
    } else {
        up_write(&mut (*card).controls_rwsem);
    }
    0
}

unsafe fn snd_ctl_elem_write_user(file: *mut snd_ctl_file, _control: *mut snd_ctl_elem_value) -> c_int {
    let control = memdup_user(_control as *const c_void, size_of::<snd_ctl_elem_value>()) as *mut snd_ctl_elem_value;
    if control.is_null() {
        return -ENOMEM;
    }
    let card = (*file).card;
    let mut result = snd_power_ref_and_wait(card);
    if result >= 0 {
        result = snd_ctl_elem_write(card, file, control);
        snd_power_unref(card);
    }
    if result >= 0 && copy_to_user(_control as *mut c_void, control as *const c_void, size_of::<snd_ctl_elem_value>()) != 0 {
        result = -EFAULT;
    }
    kfree(control as *mut c_void);
    result
}

unsafe fn snd_ctl_elem_lock(file: *mut snd_ctl_file, _id: *mut snd_ctl_elem_id) -> c_int {
    let card = (*file).card;
    let mut id: snd_ctl_elem_id = mem::zeroed();
    if copy_from_user(&mut id as *mut _ as *mut c_void, _id as *const c_void, size_of::<snd_ctl_elem_id>()) != 0 {
        return -EFAULT;
    }
    let kctl = snd_ctl_find_id(card, &id);
    if kctl.is_null() {
        return -ENOENT;
    }
    let vd = (*kctl).vd.as_mut_ptr().add(snd_ctl_get_ioff(kctl, &id) as usize);
    if !(*vd).owner.is_null() {
        return -EBUSY;
    }
    (*vd).owner = file;
    0
}

unsafe fn snd_ctl_elem_unlock(file: *mut snd_ctl_file, _id: *mut snd_ctl_elem_id) -> c_int {
    let card = (*file).card;
    let mut id: snd_ctl_elem_id = mem::zeroed();
    if copy_from_user(&mut id as *mut _ as *mut c_void, _id as *const c_void, size_of::<snd_ctl_elem_id>()) != 0 {
        return -EFAULT;
    }
    let kctl = snd_ctl_find_id(card, &id);
    if kctl.is_null() {
        return -ENOENT;
    }
    let vd = (*kctl).vd.as_mut_ptr().add(snd_ctl_get_ioff(kctl, &id) as usize);
    if (*vd).owner.is_null() {
        return -EINVAL;
    }
    if (*vd).owner != file {
        return -EPERM;
    }
    (*vd).owner = ptr::null_mut();
    0
}

unsafe fn check_user_elem_overflow(card: *mut snd_card, add: ssize_t) -> bool_ {
    (*card).user_ctl_alloc_size + add > max_user_ctl_alloc_size as ssize_t
}

unsafe extern "C" fn snd_ctl_elem_user_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let ue = snd_kcontrol_chip(kcontrol);
    let offset = snd_ctl_get_ioff(kcontrol, &(*uinfo).id);
    *uinfo = (*ue).info;
    snd_ctl_build_ioff(&mut (*uinfo).id, kcontrol, offset);
    0
}

unsafe extern "C" fn snd_ctl_elem_user_enum_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let ue = snd_kcontrol_chip(kcontrol);
    let mut item = (*uinfo).value.enumerated.item;
    let offset = snd_ctl_get_ioff(kcontrol, &(*uinfo).id);
    *uinfo = (*ue).info;
    snd_ctl_build_ioff(&mut (*uinfo).id, kcontrol, offset);
    if item > (*uinfo).value.enumerated.items - 1 {
        item = (*uinfo).value.enumerated.items - 1;
    }
    (*uinfo).value.enumerated.item = item;
    let mut names = (*ue).priv_data as *const c_char;
    while item > 0 {
        names = names.add(strlen(names) + 1);
        item -= 1;
    }
    strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), names, (*uinfo).value.enumerated.name.len());
    0
}

unsafe extern "C" fn snd_ctl_elem_user_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ue = snd_kcontrol_chip(kcontrol);
    let size = (*ue).elem_data_size as usize;
    let src = (*ue).elem_data.add(snd_ctl_get_ioff(kcontrol, &(*ucontrol).id) as usize * size);
    memcpy(&mut (*ucontrol).value as *mut _ as *mut c_void, src as *const c_void, size);
    0
}

unsafe extern "C" fn snd_ctl_elem_user_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ue = snd_kcontrol_chip(kcontrol);
    let size = (*ue).elem_data_size as usize;
    let dst = (*ue).elem_data.add(snd_ctl_get_ioff(kcontrol, &(*ucontrol).id) as usize * size);
    let err = sanity_check_input_values((*ue).card, ucontrol, &(*ue).info, false);
    if err < 0 {
        return err;
    }
    let change = memcmp(&(*ucontrol).value as *const _ as *const c_void, dst as *const c_void, size) != 0;
    if change {
        memcpy(dst as *mut c_void, &(*ucontrol).value as *const _ as *const c_void, size);
    }
    change as c_int
}

unsafe fn replace_user_tlv(kctl: *mut snd_kcontrol, buf: *mut c_uint, size: c_uint) -> c_int {
    let ue = snd_kcontrol_chip(kctl);
    let mut mask: c_uint = 0;
    if size > 1024 * 128 {
        return -EINVAL;
    }
    if check_user_elem_overflow((*ue).card, size.wrapping_sub((*ue).tlv_data_size as c_uint) as ssize_t) {
        return -ENOMEM;
    }
    let container = vmemdup_user(buf as *const c_void, size as usize);
    if container.is_null() {
        return -ENOMEM;
    }
    let mut change = (*ue).tlv_data_size != size as c_ulong;
    if !change {
        change = memcmp((*ue).tlv_data, container, size as usize) != 0;
    }
    if !change {
        kvfree(container);
        return 0;
    }
    if (*ue).tlv_data.is_null() {
        for i in 0..(*kctl).count as usize {
            (*(*kctl).vd.as_mut_ptr().add(i)).access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ;
        }
        mask = SNDRV_CTL_EVENT_MASK_INFO;
    } else {
        (*(*ue).card).user_ctl_alloc_size -= (*ue).tlv_data_size as ssize_t;
        (*ue).tlv_data_size = 0;
        kvfree((*ue).tlv_data);
    }
    (*ue).tlv_data = container;
    (*ue).tlv_data_size = size as c_ulong;
    (*(*ue).card).user_ctl_alloc_size += size as ssize_t;
    mask |= SNDRV_CTL_EVENT_MASK_TLV;
    for i in 0..(*kctl).count {
        snd_ctl_notify_one((*ue).card, mask, kctl, i);
    }
    change as c_int
}

unsafe fn read_user_tlv(kctl: *mut snd_kcontrol, buf: *mut c_uint, size: c_uint) -> c_int {
    let ue = snd_kcontrol_chip(kctl);
    if (*ue).tlv_data_size == 0 || (*ue).tlv_data.is_null() {
        return -ENXIO;
    }
    if (size as c_ulong) < (*ue).tlv_data_size {
        return -ENOSPC;
    }
    if copy_to_user(buf as *mut c_void, (*ue).tlv_data, (*ue).tlv_data_size as usize) != 0 {
        return -EFAULT;
    }
    0
}

unsafe extern "C" fn snd_ctl_elem_user_tlv(
    kctl: *mut snd_kcontrol,
    op_flag: c_int,
    size: c_uint,
    buf: *mut c_uint,
) -> c_int {
    if op_flag == SNDRV_CTL_TLV_OP_WRITE {
        replace_user_tlv(kctl, buf, size)
    } else {
        read_user_tlv(kctl, buf, size)
    }
}

unsafe fn snd_ctl_elem_init_enum_names(ue: *mut user_element) -> c_int {
    let mut buf_len = (*ue).info.value.enumerated.names_length as usize;
    let user_ptrval = (*ue).info.value.enumerated.names_ptr as uintptr_t;
    if buf_len > 64 * 1024 {
        return -EINVAL;
    }
    if check_user_elem_overflow((*ue).card, buf_len as ssize_t) {
        return -ENOMEM;
    }
    let names = vmemdup_user(user_ptrval as *const c_void, buf_len) as *mut c_char;
    if names.is_null() {
        return -ENOMEM;
    }
    let mut p = names;
    for _ in 0..(*ue).info.value.enumerated.items {
        if buf_len == 0 {
            kvfree(names as *mut c_void);
            return -EINVAL;
        }
        let name_len = strnlen(p, buf_len);
        if name_len == 0 || name_len >= 64 || name_len == buf_len {
            kvfree(names as *mut c_void);
            return -EINVAL;
        }
        p = p.add(name_len + 1);
        buf_len -= name_len + 1;
    }
    (*ue).priv_data = names as *mut c_void;
    (*ue).info.value.enumerated.names_ptr = 0;
    (*(*ue).card).user_ctl_alloc_size += (*ue).info.value.enumerated.names_length as ssize_t;
    0
}

unsafe fn compute_user_elem_size(size: size_t, count: c_uint) -> size_t {
    size_of::<user_element>() + size * count as usize
}

unsafe extern "C" fn snd_ctl_elem_user_free(kcontrol: *mut snd_kcontrol) {
    let ue = snd_kcontrol_chip(kcontrol);
    (*(*ue).card).user_ctl_alloc_size -= compute_user_elem_size((*ue).elem_data_size as usize, (*kcontrol).count) as ssize_t;
    (*(*ue).card).user_ctl_alloc_size -= (*ue).tlv_data_size as ssize_t;
    if !(*ue).priv_data.is_null() {
        (*(*ue).card).user_ctl_alloc_size -= (*ue).info.value.enumerated.names_length as ssize_t;
    }
    kvfree((*ue).tlv_data);
    kvfree((*ue).priv_data);
    kfree(ue as *mut c_void);
}

unsafe fn snd_ctl_elem_add(file: *mut snd_ctl_file, info: *mut snd_ctl_elem_info, replace: c_int) -> c_int {
    let card = (*file).card;
    if (*info).id.name[0] == 0 {
        return -EINVAL;
    }
    if strnlen((*info).id.name.as_ptr(), (*info).id.name.len()) >= (*info).id.name.len() {
        return -EINVAL;
    }
    if replace != 0 {
        (*info).id.numid = 0;
        let err = snd_ctl_remove_user_ctl(file, &mut (*info).id);
        if err != 0 {
            return err;
        }
    }
    let mut count = (*info).owner as c_uint;
    if count == 0 {
        count = 1;
    }
    if count > MAX_CONTROL_COUNT {
        return -EINVAL;
    }
    let mut access = (*info).access;
    if access == 0 {
        access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
    }
    access &= SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE | SNDRV_CTL_ELEM_ACCESS_TLV_WRITE;
    if (access & SNDRV_CTL_ELEM_ACCESS_TLV_WRITE) != 0 {
        access |= SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK;
    }
    access |= SNDRV_CTL_ELEM_ACCESS_USER;
    let mut err = snd_ctl_check_elem_info(ptr::null_mut(), info);
    if err < 0 {
        return err;
    }
    if (*info).count < 1 {
        return -EINVAL;
    }
    let private_size = value_sizes[(*info).type_ as usize] as usize * (*info).count as usize;
    let alloc_size = compute_user_elem_size(private_size, count);
    if check_user_elem_overflow(card, alloc_size as ssize_t) {
        return -ENOMEM;
    }
    let mut kctl: *mut snd_kcontrol = ptr::null_mut();
    err = snd_ctl_new(&mut kctl, count, access, file);
    if err < 0 {
        return err;
    }
    memcpy(&mut (*kctl).id as *mut _ as *mut c_void, &(*info).id as *const _ as *const c_void, size_of::<snd_ctl_elem_id>());
    let ue = kzalloc(alloc_size, GFP_KERNEL) as *mut user_element;
    if ue.is_null() {
        kfree(kctl as *mut c_void);
        return -ENOMEM;
    }
    (*kctl).private_data = ue as *mut c_void;
    (*kctl).private_free = Some(snd_ctl_elem_user_free);
    (*card).user_ctl_alloc_size += alloc_size as ssize_t;
    (*ue).card = card;
    (*ue).info = *info;
    (*ue).info.access = 0;
    (*ue).elem_data = (ue as *mut c_char).add(size_of::<user_element>());
    (*ue).elem_data_size = private_size as c_ulong;
    if (*ue).info.type_ as usize == SNDRV_CTL_ELEM_TYPE_ENUMERATED {
        err = snd_ctl_elem_init_enum_names(ue);
        if err < 0 {
            snd_ctl_free_one(kctl);
            return err;
        }
    }
    if (*info).type_ as usize == SNDRV_CTL_ELEM_TYPE_ENUMERATED {
        (*kctl).info = Some(snd_ctl_elem_user_enum_info);
    } else {
        (*kctl).info = Some(snd_ctl_elem_user_info);
    }
    if (access & SNDRV_CTL_ELEM_ACCESS_READ) != 0 {
        (*kctl).get = Some(snd_ctl_elem_user_get);
    }
    if (access & SNDRV_CTL_ELEM_ACCESS_WRITE) != 0 {
        (*kctl).put = Some(snd_ctl_elem_user_put);
    }
    if (access & SNDRV_CTL_ELEM_ACCESS_TLV_WRITE) != 0 {
        (*kctl).tlv.c = Some(snd_ctl_elem_user_tlv);
    }
    err = __snd_ctl_add_replace(card, kctl, snd_ctl_add_mode::CTL_ADD_EXCLUSIVE);
    if err < 0 {
        snd_ctl_free_one(kctl);
        return err;
    }
    let offset = snd_ctl_get_ioff(kctl, &(*info).id);
    snd_ctl_build_ioff(&mut (*info).id, kctl, offset);
    0
}

unsafe fn snd_ctl_elem_add_user(file: *mut snd_ctl_file, _info: *mut snd_ctl_elem_info, replace: c_int) -> c_int {
    let mut info: snd_ctl_elem_info = mem::zeroed();
    if copy_from_user(&mut info as *mut _ as *mut c_void, _info as *const c_void, size_of::<snd_ctl_elem_info>()) != 0 {
        return -EFAULT;
    }
    let err = snd_ctl_elem_add(file, &mut info, replace);
    if err < 0 {
        return err;
    }
    if copy_to_user(_info as *mut c_void, &info as *const _ as *const c_void, size_of::<snd_ctl_elem_info>()) != 0 {
        snd_ctl_remove_user_ctl(file, &mut info.id);
        return -EFAULT;
    }
    0
}

unsafe fn snd_ctl_elem_remove(file: *mut snd_ctl_file, _id: *mut snd_ctl_elem_id) -> c_int {
    let mut id: snd_ctl_elem_id = mem::zeroed();
    if copy_from_user(&mut id as *mut _ as *mut c_void, _id as *const c_void, size_of::<snd_ctl_elem_id>()) != 0 {
        return -EFAULT;
    }
    snd_ctl_remove_user_ctl(file, &mut id)
}

unsafe fn snd_ctl_subscribe_events(file: *mut snd_ctl_file, ptr: *mut c_int) -> c_int {
    let mut subscribe = 0;
    if get_user_int(&mut subscribe, ptr) != 0 {
        return -EFAULT;
    }
    if subscribe < 0 {
        subscribe = (*file).subscribed;
        if put_user_int(subscribe, ptr) != 0 {
            return -EFAULT;
        }
        return 0;
    }
    if subscribe != 0 {
        (*file).subscribed = 1;
        return 0;
    } else if (*file).subscribed != 0 {
        snd_ctl_empty_read_queue(file);
        (*file).subscribed = 0;
    }
    0
}

unsafe fn call_tlv_handler(
    file: *mut snd_ctl_file,
    op_flag: c_int,
    kctl: *mut snd_kcontrol,
    id: *mut snd_ctl_elem_id,
    buf: *mut c_uint,
    size: c_uint,
) -> c_int {
    #[repr(C)]
    struct pair {
        op: c_int,
        perm: c_int,
    }
    let pairs = [
        pair { op: SNDRV_CTL_TLV_OP_READ, perm: SNDRV_CTL_ELEM_ACCESS_TLV_READ as c_int },
        pair { op: SNDRV_CTL_TLV_OP_WRITE, perm: SNDRV_CTL_ELEM_ACCESS_TLV_WRITE as c_int },
        pair { op: SNDRV_CTL_TLV_OP_CMD, perm: SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND as c_int },
    ];
    let vd = (*kctl).vd.as_mut_ptr().add(snd_ctl_get_ioff(kctl, id) as usize);
    let mut i = 0;
    while i < ARRAY_SIZE(&pairs) {
        if op_flag == pairs[i].op && ((*vd).access & pairs[i].perm as c_uint) != 0 {
            break;
        }
        i += 1;
    }
    if i == ARRAY_SIZE(&pairs) {
        return -ENXIO;
    }
    if (*kctl).tlv.c.is_none() {
        return -ENXIO;
    }
    if op_flag != SNDRV_CTL_TLV_OP_READ && !(*vd).owner.is_null() && (*vd).owner != file {
        return -EPERM;
    }
    (*kctl).tlv.c.unwrap()(kctl, op_flag, size, buf)
}

unsafe fn read_tlv_buf(kctl: *mut snd_kcontrol, id: *mut snd_ctl_elem_id, buf: *mut c_uint, size: c_uint) -> c_int {
    let vd = (*kctl).vd.as_mut_ptr().add(snd_ctl_get_ioff(kctl, id) as usize);
    if ((*vd).access & SNDRV_CTL_ELEM_ACCESS_TLV_READ) == 0 {
        return -ENXIO;
    }
    let p = (*kctl).tlv.p;
    if p.is_null() {
        return -ENXIO;
    }
    let len = size_of::<c_uint>() as c_uint * 2 + *p.add(1);
    if size < len {
        return -ENOMEM;
    }
    if copy_to_user(buf as *mut c_void, p as *const c_void, len as usize) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_ctl_tlv_ioctl(file: *mut snd_ctl_file, buf: *mut snd_ctl_tlv, op_flag: c_int) -> c_int {
    let mut header: snd_ctl_tlv = mem::zeroed();
    if copy_from_user(&mut header as *mut _ as *mut c_void, buf as *const c_void, size_of::<snd_ctl_tlv>()) != 0 {
        return -EFAULT;
    }
    if header.numid == 0 {
        return -EINVAL;
    }
    if header.length < (size_of::<c_uint>() * 2) as c_uint {
        return -EINVAL;
    }
    let container_size = header.length;
    let container = (*buf).tlv;
    let kctl = snd_ctl_find_numid((*file).card, header.numid);
    if kctl.is_null() {
        return -ENOENT;
    }
    let mut id = (*kctl).id;
    snd_ctl_build_ioff(&mut id, kctl, header.numid - id.numid);
    let vd = (*kctl).vd.as_mut_ptr().add(snd_ctl_get_ioff(kctl, &id) as usize);
    if ((*vd).access & SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK) != 0 {
        return call_tlv_handler(file, op_flag, kctl, &mut id, container, container_size);
    } else if op_flag == SNDRV_CTL_TLV_OP_READ {
        return read_tlv_buf(kctl, &mut id, container, container_size);
    }
    -ENXIO
}

unsafe extern "C" fn snd_ctl_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let ctl = (*file).private_data as *mut snd_ctl_file;
    let card = (*ctl).card;
    let argp = arg as *mut c_void;
    let ip = argp as *mut c_int;
    if snd_BUG_ON(card.is_null()) {
        return -ENXIO as c_long;
    }
    match cmd {
        SNDRV_CTL_IOCTL_PVERSION => return if put_user_int(SNDRV_CTL_VERSION, ip) != 0 { -EFAULT as c_long } else { 0 },
        SNDRV_CTL_IOCTL_CARD_INFO => return snd_ctl_card_info(card, ctl, cmd, argp) as c_long,
        SNDRV_CTL_IOCTL_CARD_BYTES => return snd_ctl_card_bytes_user(card, argp as *mut snd_ctl_card_bytes) as c_long,
        SNDRV_CTL_IOCTL_ELEM_LIST => return snd_ctl_elem_list_user(card, argp as *mut snd_ctl_elem_list) as c_long,
        SNDRV_CTL_IOCTL_ELEM_INFO => return snd_ctl_elem_info_user(ctl, argp as *mut snd_ctl_elem_info) as c_long,
        SNDRV_CTL_IOCTL_ELEM_READ => return snd_ctl_elem_read_user(card, argp as *mut snd_ctl_elem_value) as c_long,
        SNDRV_CTL_IOCTL_ELEM_WRITE => return snd_ctl_elem_write_user(ctl, argp as *mut snd_ctl_elem_value) as c_long,
        SNDRV_CTL_IOCTL_ELEM_LOCK => return snd_ctl_elem_lock(ctl, argp as *mut snd_ctl_elem_id) as c_long,
        SNDRV_CTL_IOCTL_ELEM_UNLOCK => return snd_ctl_elem_unlock(ctl, argp as *mut snd_ctl_elem_id) as c_long,
        SNDRV_CTL_IOCTL_ELEM_ADD => return snd_ctl_elem_add_user(ctl, argp as *mut snd_ctl_elem_info, 0) as c_long,
        SNDRV_CTL_IOCTL_ELEM_REPLACE => return snd_ctl_elem_add_user(ctl, argp as *mut snd_ctl_elem_info, 1) as c_long,
        SNDRV_CTL_IOCTL_ELEM_REMOVE => return snd_ctl_elem_remove(ctl, argp as *mut snd_ctl_elem_id) as c_long,
        SNDRV_CTL_IOCTL_SUBSCRIBE_EVENTS => return snd_ctl_subscribe_events(ctl, ip) as c_long,
        SNDRV_CTL_IOCTL_TLV_READ => {
            let mut err = snd_power_ref_and_wait(card);
            if err < 0 { return err as c_long; }
            err = snd_ctl_tlv_ioctl(ctl, argp as *mut snd_ctl_tlv, SNDRV_CTL_TLV_OP_READ);
            snd_power_unref(card);
            return err as c_long;
        }
        SNDRV_CTL_IOCTL_TLV_WRITE => {
            let mut err = snd_power_ref_and_wait(card);
            if err < 0 { return err as c_long; }
            err = snd_ctl_tlv_ioctl(ctl, argp as *mut snd_ctl_tlv, SNDRV_CTL_TLV_OP_WRITE);
            snd_power_unref(card);
            return err as c_long;
        }
        SNDRV_CTL_IOCTL_TLV_COMMAND => {
            let mut err = snd_power_ref_and_wait(card);
            if err < 0 { return err as c_long; }
            err = snd_ctl_tlv_ioctl(ctl, argp as *mut snd_ctl_tlv, SNDRV_CTL_TLV_OP_CMD);
            snd_power_unref(card);
            return err as c_long;
        }
        SNDRV_CTL_IOCTL_POWER => return -ENOPROTOOPT as c_long,
        SNDRV_CTL_IOCTL_POWER_STATE => return if put_user_int(SNDRV_CTL_POWER_D0, ip) != 0 { -EFAULT as c_long } else { 0 },
        _ => {}
    }
    let mut p_pos = snd_control_ioctls.next;
    while p_pos != &raw mut snd_control_ioctls {
        let p = p_pos as *mut snd_kctl_ioctl;
        let err = (*p).fioctl.unwrap()(card, ctl, cmd, arg);
        if err != -ENOIOCTLCMD {
            return err as c_long;
        }
        p_pos = (*p_pos).next;
    }
    dev_dbg((*card).dev, c"unknown ioctl = 0x%x\n".as_ptr(), cmd);
    -ENOTTY as c_long
}

unsafe extern "C" fn snd_ctl_read(file: *mut file, mut buffer: *mut c_char, mut count: size_t, _offset: *mut loff_t) -> ssize_t {
    let ctl = (*file).private_data as *mut snd_ctl_file;
    let mut err: c_int = 0;
    let mut result: ssize_t = 0;
    if snd_BUG_ON(ctl.is_null() || (*ctl).card.is_null()) {
        return -ENXIO as ssize_t;
    }
    if (*ctl).subscribed == 0 {
        return -EBADFD as ssize_t;
    }
    if count < size_of::<snd_ctl_event>() {
        return -EINVAL as ssize_t;
    }
    spin_lock_irq(&mut (*ctl).read_lock);
    while count >= size_of::<snd_ctl_event>() {
        while list_empty(&(*ctl).events) {
            let mut wait: wait_queue_entry_t = mem::zeroed();
            if ((*file).f_flags & O_NONBLOCK) != 0 || result > 0 {
                err = -EAGAIN;
                spin_unlock_irq(&mut (*ctl).read_lock);
                return if result > 0 { result } else { err as ssize_t };
            }
            init_waitqueue_entry(&mut wait, current);
            add_wait_queue(&mut (*ctl).change_sleep, &mut wait);
            set_current_state(TASK_INTERRUPTIBLE);
            spin_unlock_irq(&mut (*ctl).read_lock);
            schedule();
            remove_wait_queue(&mut (*ctl).change_sleep, &mut wait);
            if (*(*ctl).card).shutdown {
                return -ENODEV as ssize_t;
            }
            if signal_pending(current) != 0 {
                return -ERESTARTSYS as ssize_t;
            }
            spin_lock_irq(&mut (*ctl).read_lock);
        }
        let kev = snd_kctl_event((*ctl).events.next);
        let mut ev: snd_ctl_event = mem::zeroed();
        ev.type_ = SNDRV_CTL_EVENT_ELEM;
        ev.data.elem.mask = (*kev).mask;
        ev.data.elem.id = (*kev).id;
        list_del(&mut (*kev).list);
        spin_unlock_irq(&mut (*ctl).read_lock);
        kfree(kev as *mut c_void);
        if copy_to_user(buffer as *mut c_void, &ev as *const _ as *const c_void, size_of::<snd_ctl_event>()) != 0 {
            err = -EFAULT;
            return if result > 0 { result } else { err as ssize_t };
        }
        spin_lock_irq(&mut (*ctl).read_lock);
        buffer = buffer.add(size_of::<snd_ctl_event>());
        count -= size_of::<snd_ctl_event>();
        result += size_of::<snd_ctl_event>() as ssize_t;
    }
    spin_unlock_irq(&mut (*ctl).read_lock);
    if result > 0 { result } else { err as ssize_t }
}

unsafe extern "C" fn snd_ctl_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let ctl = (*file).private_data as *mut snd_ctl_file;
    if (*ctl).subscribed == 0 {
        return 0;
    }
    poll_wait(file, &mut (*ctl).change_sleep, wait);
    let mut mask: __poll_t = 0;
    if !list_empty(&(*ctl).events) {
        mask |= EPOLLIN | EPOLLRDNORM;
    }
    mask
}

unsafe fn _snd_ctl_register_ioctl(fcn: snd_kctl_ioctl_func_t, lists: *mut list_head) -> c_int {
    let pn = kzalloc(size_of::<snd_kctl_ioctl>(), GFP_KERNEL) as *mut snd_kctl_ioctl;
    if pn.is_null() {
        return -ENOMEM;
    }
    (*pn).fioctl = fcn;
    list_add_tail(&mut (*pn).list, lists);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_register_ioctl(fcn: snd_kctl_ioctl_func_t) -> c_int {
    _snd_ctl_register_ioctl(fcn, &raw mut snd_control_ioctls)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_register_ioctl_compat(fcn: snd_kctl_ioctl_func_t) -> c_int {
    _snd_ctl_register_ioctl(fcn, &raw mut snd_control_compat_ioctls)
}

unsafe fn _snd_ctl_unregister_ioctl(fcn: snd_kctl_ioctl_func_t, lists: *mut list_head) -> c_int {
    if snd_BUG_ON(fcn.is_none()) {
        return -EINVAL;
    }
    let mut pos = (*lists).next;
    while pos != lists {
        let p = pos as *mut snd_kctl_ioctl;
        if (*p).fioctl == fcn {
            list_del(&mut (*p).list);
            kfree(p as *mut c_void);
            return 0;
        }
        pos = (*pos).next;
    }
    snd_BUG();
    -EINVAL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_unregister_ioctl(fcn: snd_kctl_ioctl_func_t) -> c_int {
    _snd_ctl_unregister_ioctl(fcn, &raw mut snd_control_ioctls)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_unregister_ioctl_compat(fcn: snd_kctl_ioctl_func_t) -> c_int {
    _snd_ctl_unregister_ioctl(fcn, &raw mut snd_control_compat_ioctls)
}

unsafe extern "C" fn snd_ctl_fasync(fd: c_int, file: *mut file, on: c_int) -> c_int {
    let ctl = (*file).private_data as *mut snd_ctl_file;
    snd_fasync_helper(fd, file, on, &mut (*ctl).fasync)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_get_preferred_subdevice(card: *mut snd_card, type_: c_int) -> c_int {
    let mut subdevice = -1;
    let mut pos = (*card).ctl_files.next;
    while pos != &mut (*card).ctl_files {
        let kctl = pos as *mut snd_ctl_file;
        if (*kctl).pid == task_pid(current) {
            subdevice = (*kctl).preferred_subdevice[type_ as usize];
            if subdevice != -1 {
                break;
            }
        }
        pos = (*pos).next;
    }
    subdevice
}

// CONFIG_COMPAT includes control_compat.c in C; when disabled snd_ctl_ioctl_compat is NULL.
unsafe extern "C" fn snd_ctl_ioctl_compat(_file: *mut file, _cmd: c_uint, _arg: c_ulong) -> c_long {
    -ENOTTY as c_long
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_request_layer(module_name: *const c_char) -> c_int {
    if module_name.is_null() {
        return 0;
    }
    let mut lops = snd_ctl_layer;
    while !lops.is_null() {
        if strcmp((*lops).module_name, module_name) == 0 {
            return 0;
        }
        lops = (*lops).next;
    }
    request_module(module_name)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_register_layer(lops: *mut snd_ctl_layer_ops) {
    (*lops).next = snd_ctl_layer;
    snd_ctl_layer = lops;
    for card_number in 0..SNDRV_CARDS {
        let card = snd_card_ref(card_number);
        if !card.is_null() {
            ((*lops).lregister)(card);
            snd_card_unref(card);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_disconnect_layer(lops: *mut snd_ctl_layer_ops) {
    let mut lops2 = snd_ctl_layer;
    let mut prev_lops2: *mut snd_ctl_layer_ops = ptr::null_mut();
    while !lops2.is_null() {
        if lops2 == lops {
            if prev_lops2.is_null() {
                snd_ctl_layer = (*lops).next;
            } else {
                (*prev_lops2).next = (*lops).next;
            }
            break;
        }
        prev_lops2 = lops2;
        lops2 = (*lops2).next;
    }
}

static snd_ctl_f_ops: file_operations = file_operations {
    owner: unsafe { THIS_MODULE },
    read: Some(snd_ctl_read),
    open: Some(snd_ctl_open),
    release: Some(snd_ctl_release),
    poll: Some(snd_ctl_poll),
    unlocked_ioctl: Some(snd_ctl_ioctl),
    compat_ioctl: Some(snd_ctl_ioctl_compat),
    fasync: Some(snd_ctl_fasync),
};

unsafe fn call_snd_ctl_lops(card: *mut snd_card, op: unsafe extern "C" fn(*mut snd_ctl_layer_ops, *mut snd_card)) {
    let mut lops = snd_ctl_layer;
    while !lops.is_null() {
        op(lops, card);
        lops = (*lops).next;
    }
}

unsafe extern "C" fn call_lregister(lops: *mut snd_ctl_layer_ops, card: *mut snd_card) {
    ((*lops).lregister)(card);
}

unsafe extern "C" fn call_ldisconnect(lops: *mut snd_ctl_layer_ops, card: *mut snd_card) {
    ((*lops).ldisconnect)(card);
}

unsafe extern "C" fn snd_ctl_dev_register(device: *mut snd_device) -> c_int {
    let card = (*device).device_data as *mut snd_card;
    let err = snd_register_device(
        SNDRV_DEVICE_TYPE_CONTROL,
        card,
        -1,
        &snd_ctl_f_ops,
        card as *mut c_void,
        (*card).ctl_dev,
    );
    if err < 0 {
        return err;
    }
    call_snd_ctl_lops(card, call_lregister);
    0
}

unsafe extern "C" fn snd_ctl_dev_disconnect(device: *mut snd_device) -> c_int {
    let card = (*device).device_data as *mut snd_card;
    let mut pos = (*card).ctl_files.next;
    while pos != &mut (*card).ctl_files {
        let ctl = pos as *mut snd_ctl_file;
        wake_up(&mut (*ctl).change_sleep);
        snd_kill_fasync((*ctl).fasync, SIGIO, POLL_ERR);
        pos = (*pos).next;
    }
    call_snd_ctl_lops(card, call_ldisconnect);
    snd_unregister_device((*card).ctl_dev)
}

unsafe extern "C" fn snd_ctl_dev_free(device: *mut snd_device) -> c_int {
    let card = (*device).device_data as *mut snd_card;
    while !list_empty(&(*card).controls) {
        let control = snd_kcontrol((*card).controls.next);
        __snd_ctl_remove(card, control, false);
    }
    xa_destroy(&mut (*card).ctl_numids);
    xa_destroy(&mut (*card).ctl_hash);
    put_device((*card).ctl_dev);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_create(card: *mut snd_card) -> c_int {
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_ctl_dev_free),
        dev_register: Some(snd_ctl_dev_register),
        dev_disconnect: Some(snd_ctl_dev_disconnect),
    };
    if snd_BUG_ON(card.is_null()) {
        return -ENXIO;
    }
    if snd_BUG_ON((*card).number < 0 || (*card).number >= SNDRV_CARDS) {
        return -ENXIO;
    }
    let mut err = snd_device_alloc(&mut (*card).ctl_dev, card);
    if err < 0 {
        return err;
    }
    dev_set_name((*card).ctl_dev, c"controlC%d".as_ptr(), (*card).number);
    err = snd_device_new(card, SNDRV_DEV_CONTROL, card as *mut c_void, &ops);
    if err < 0 {
        put_device((*card).ctl_dev);
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_boolean_mono_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN as c_int;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_boolean_stereo_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN as c_int;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ctl_enum_info(
    info: *mut snd_ctl_elem_info,
    channels: c_uint,
    items: c_uint,
    names: *const *const c_char,
) -> c_int {
    (*info).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED as c_int;
    (*info).count = channels;
    (*info).value.enumerated.items = items;
    if items == 0 {
        return 0;
    }
    if (*info).value.enumerated.item >= items {
        (*info).value.enumerated.item = items - 1;
    }
    let name = *names.add((*info).value.enumerated.item as usize);
    WARN(
        strlen(name) >= (*info).value.enumerated.name.len(),
        c"ALSA: too long item name '%s'\n".as_ptr(),
        name,
    );
    strscpy(
        (*info).value.enumerated.name.as_mut_ptr(),
        name,
        (*info).value.enumerated.name.len(),
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
