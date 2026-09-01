// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Loopback soundcard
 *
 *  Original code:
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 *  More accurate positioning and full-duplex support:
 *  Copyright (c) Ahmet İnan <ainan at mathematik.uni-freiburg.de>
 *
 *  Major (almost complete) rewrite:
 *  Copyright (c) by Takashi Iwai <tiwai@suse.de>
 *
 *  A next major update in 2010 (separate timers for playback and capture):
 *  Copyright (c) Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = bool;
type u_int32_t = u32;
type u64 = u64;
type snd_pcm_format_t = c_int;
type snd_pcm_access_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type hrtimer_restart = c_int;

const MAX_PCM_SUBSTREAMS: usize = 8;
const NO_PITCH: c_uint = 100000;

const fn BIT(n: c_int) -> c_uint {
    1u32 << n
}

const CABLE_VALID_PLAYBACK: c_uint = BIT(SNDRV_PCM_STREAM_PLAYBACK);
const CABLE_VALID_CAPTURE: c_uint = BIT(SNDRV_PCM_STREAM_CAPTURE);
const CABLE_VALID_BOTH: c_uint = CABLE_VALID_PLAYBACK | CABLE_VALID_CAPTURE;

const ACTIVE_IDX: usize = 2;
const FORMAT_IDX: usize = 3;
const RATE_IDX: usize = 4;
const CHANNELS_IDX: usize = 5;
const ACCESS_IDX: usize = 6;
const SND_LOOPBACK_DRIVER: *const c_char = b"snd_aloop\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_refcount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timer_list {
    pub expires: c_ulong,
}
#[repr(C)]
pub struct hrtimer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_timer_id {
    pub dev_class: c_int,
    pub dev_sclass: c_int,
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
}

#[repr(C)]
pub struct snd_timer_instance {
    pub flags: c_uint,
    pub callback: Option<unsafe extern "C" fn(*mut snd_timer_instance, c_ulong, c_ulong)>,
    pub callback_data: *mut c_void,
    pub ccallback:
        Option<unsafe extern "C" fn(*mut snd_timer_instance, c_int, *mut timespec64, c_ulong)>,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u64,
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_ulong,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
}

#[repr(C)]
pub struct snd_pcm_control {
    pub appl_ptr: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm_runtime)>,
    pub hw: snd_pcm_hardware,
    pub format: snd_pcm_format_t,
    pub rate: c_uint,
    pub channels: c_uint,
    pub access: snd_pcm_access_t,
    pub state: c_int,
    pub dma_area: *mut c_char,
    pub buffer_size: snd_pcm_uframes_t,
    pub period_size: snd_pcm_uframes_t,
    pub timer_resolution: c_ulong,
    pub control: *mut snd_pcm_control,
}

#[repr(C)]
pub struct snd_pcm_str {
    pub substream_count: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub device: c_int,
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: [c_char; 80],
    pub streams: [snd_pcm_str; 2],
}

#[repr(C)]
pub struct snd_pcm_pstr {
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub private_data: *mut c_void,
    pub pcm: *mut snd_pcm,
    pub pstr: *mut snd_pcm_pstr,
    pub stream: c_int,
    pub number: c_int,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub id: *mut c_char,
    pub mixername: [c_char; 80],
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct platform_device {
    pub id: c_int,
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub name: *mut c_char,
}
#[repr(C)]
pub struct snd_mask {
    pub bits: [u_int32_t; 2],
}
#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub openmin: c_uint,
    pub openmax: c_uint,
    pub integer: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
    pub var: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_id {
    pub device: c_uint,
    pub subdevice: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_kcontrol_id,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
    pub step: i64,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct loopback_ops {
    /* optional
     * call in loopback->cable_lock
     */
    open: Option<unsafe extern "C" fn(*mut loopback_pcm) -> c_int>,
    /* required
     * call in cable->lock
     */
    start: Option<unsafe extern "C" fn(*mut loopback_pcm) -> c_int>,
    /* required
     * call in cable->lock
     */
    stop: Option<unsafe extern "C" fn(*mut loopback_pcm) -> c_int>,
    /* optional */
    stop_sync: Option<unsafe extern "C" fn(*mut loopback_pcm) -> c_int>,
    /* optional */
    close_substream: Option<unsafe extern "C" fn(*mut loopback_pcm) -> c_int>,
    /* optional
     * call in loopback->cable_lock
     */
    close_cable: Option<unsafe extern "C" fn(*mut loopback_pcm) -> c_int>,
    /* optional
     * call in cable->lock
     */
    pos_update: Option<unsafe extern "C" fn(*mut loopback_cable) -> c_uint>,
    /* optional */
    dpcm_info: Option<unsafe extern "C" fn(*mut loopback_pcm, *mut snd_info_buffer)>,
}

#[repr(C)]
struct loopback_snd_timer {
    stream: c_int,
    id: snd_timer_id,
    event_work: work_struct,
    instance: *mut snd_timer_instance,
}

#[repr(C)]
struct loopback_cable {
    lock: spinlock_t,
    streams: [*mut loopback_pcm; 2],
    /* in-flight peer stops running outside cable->lock */
    stop_count: snd_refcount,
    hw: snd_pcm_hardware,
    /* flags */
    valid: c_uint,
    running: c_uint,
    pause: c_uint,
    /* timer specific */
    ops: *const loopback_ops,
    /* If sound timer is used */
    snd_timer: loopback_snd_timer,
}

#[repr(C)]
struct loopback_setup {
    notify: c_uint,
    rate_shift: c_uint,
    format: snd_pcm_format_t,
    rate: c_uint,
    access: snd_pcm_access_t,
    channels: c_uint,
    active_id: snd_ctl_elem_id,
    format_id: snd_ctl_elem_id,
    rate_id: snd_ctl_elem_id,
    channels_id: snd_ctl_elem_id,
    access_id: snd_ctl_elem_id,
}

#[repr(C)]
struct loopback {
    card: *mut snd_card,
    cable_lock: mutex,
    cables: [[*mut loopback_cable; 2]; MAX_PCM_SUBSTREAMS],
    pcm: [*mut snd_pcm; 2],
    setup: [[loopback_setup; 2]; MAX_PCM_SUBSTREAMS],
    timer_source: *const c_char,
}

#[repr(C)]
struct loopback_pcm {
    loopback: *mut loopback,
    substream: *mut snd_pcm_substream,
    cable: *mut loopback_cable,
    pcm_buffer_size: c_uint,
    buf_pos: c_uint, /* position in buffer */
    silent_size: c_uint,
    /* PCM parameters */
    pcm_period_size: c_uint,
    pcm_bps: c_uint,       /* bytes per second */
    pcm_salign: c_uint,    /* bytes per sample * channels */
    pcm_rate_shift: c_uint, /* rate shift value */
    /* flags */
    period_update_pending: c_uint,
    /* timer stuff */
    irq_pos: c_uint, /* fractional IRQ position in jiffies
                      * ticks
                      */
    period_size_frac: c_uint, /* period size in jiffies ticks */
    last_drift: c_uint,
    last_jiffies: c_ulong,
    /* If jiffies / hrtimer is used */
    timer: timer_list,
    /* CONFIG_HIGH_RES_TIMERS */
    hrtimer: hrtimer,
    /* size of per channel buffer in case of non-interleaved access */
    channel_buf_n: c_uint,
}

unsafe extern "C" {
    static mut index: [c_int; SNDRV_CARDS as usize];
    static mut id: [*mut c_char; SNDRV_CARDS as usize];
    static mut enable: [bool_; SNDRV_CARDS as usize];
    static mut pcm_substreams: [c_int; SNDRV_CARDS as usize];
    static mut pcm_notify: [c_int; SNDRV_CARDS as usize];
    static mut timer_source: [*mut c_char; SNDRV_CARDS as usize];
    static mut snd_ecards_limit: c_int;
    static mut jiffies: c_ulong;
    static mut current: *mut task_struct;
    static mut THIS_MODULE: *mut module;

    fn div_u64(dividend: u64, divisor: u64) -> c_uint;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_delete(timer: *mut timer_list) -> c_int;
    fn timer_delete_sync(timer: *mut timer_list) -> c_int;
    fn timer_setup(timer: *mut timer_list, func: unsafe extern "C" fn(*mut timer_list), flags: c_uint);
    fn hrtimer_start(timer: *mut hrtimer, time: ktime_t, mode: c_int) -> c_int;
    fn hrtimer_try_to_cancel(timer: *mut hrtimer) -> c_int;
    fn hrtimer_cancel(timer: *mut hrtimer) -> c_int;
    fn hrtimer_setup(
        timer: *mut hrtimer,
        func: unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart,
        clock_id: c_int,
        mode: c_int,
    );
    fn hrtimer_get_expires(timer: *mut hrtimer) -> ktime_t;
    fn ns_to_ktime(ns: u64) -> ktime_t;
    fn ktime_to_ns(kt: ktime_t) -> u64;
    fn snd_timer_start(timeri: *mut snd_timer_instance, ticks: c_uint) -> c_int;
    fn snd_timer_stop(timeri: *mut snd_timer_instance) -> c_int;
    fn snd_timer_close(timeri: *mut snd_timer_instance) -> c_int;
    fn snd_timer_instance_free(timeri: *mut snd_timer_instance);
    fn snd_timer_instance_new(owner: *const c_char) -> *mut snd_timer_instance;
    fn snd_timer_open(timeri: *mut snd_timer_instance, tid: *mut snd_timer_id, pid: c_int) -> c_int;
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn schedule_work(work: *mut work_struct) -> bool_;
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn kstrtoint(s: *const c_char, base: c_uint, res: *mut c_int) -> c_int;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn devm_kfree(dev: *mut device, p: *const c_void);
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn pcm_err(pcm: *mut snd_pcm, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_int);
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_format_set_silence(format: snd_pcm_format_t, data: *mut c_void, samples: snd_pcm_uframes_t) -> c_int;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> u64;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_playback_hw_avail(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_pcm_direction_name(stream: c_int) -> *const c_char;
    fn snd_mask_none(mask: *mut snd_mask);
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_mask_refine(mask: *mut snd_mask, v: *mut snd_mask) -> c_int;
    fn snd_interval_refine(i: *mut snd_interval, v: *mut snd_interval) -> c_int;
    fn snd_refcount_get(r: *mut snd_refcount);
    fn snd_refcount_put(r: *mut snd_refcount);
    fn snd_refcount_sync(r: *mut snd_refcount);
    fn snd_refcount_init(r: *mut snd_refcount);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut c_void,
        dep: c_int,
        ...
    ) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, typ: c_int, data: *mut c_void, size: usize, max: usize);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)) -> c_int;
    fn snd_card_rw_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer), write: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)) -> c_int;
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: usize) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut module, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(name: *const c_char, id: c_int, res: *mut c_void, nres: c_uint) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn snd_card_ref(idx: c_int) -> *mut snd_card;
    fn snd_card_unref(card: *mut snd_card);
}

#[repr(C)]
pub struct task_struct {
    pub pid: c_int,
}
type ktime_t = i64;

const SNDRV_CARDS: c_int = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_ACCESS_MMAP_INTERLEAVED: snd_pcm_access_t = 0;
const SNDRV_PCM_ACCESS_RW_INTERLEAVED: snd_pcm_access_t = 3;
const SNDRV_PCM_STATE_RUNNING: c_int = 3;
const SNDRV_PCM_STATE_DRAINING: c_int = 4;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_TIMER_EVENT_TICK: c_int = 1;
const SNDRV_TIMER_EVENT_MSTOP: c_int = 5;
const SNDRV_TIMER_CLASS_GLOBAL: c_int = 0;
const SNDRV_TIMER_CLASS_PCM: c_int = 3;
const SNDRV_TIMER_SCLASS_APPLICATION: c_int = 2;
const SNDRV_TIMER_IFLG_AUTO: c_uint = 1;
const SNDRV_PCM_INFO_INTERLEAVED: u64 = 1 << 0;
const SNDRV_PCM_INFO_MMAP: u64 = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: u64 = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: u64 = 1 << 3;
const SNDRV_PCM_INFO_RESUME: u64 = 1 << 4;
const SNDRV_PCM_INFO_NONINTERLEAVED: u64 = 1 << 5;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_BE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_S24_3BE: u64 = 1 << 5;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_BE: u64 = 1 << 7;
const SNDRV_PCM_FMTBIT_FLOAT_LE: u64 = 1 << 8;
const SNDRV_PCM_FMTBIT_FLOAT_BE: u64 = 1 << 9;
const SNDRV_PCM_FMTBIT_DSD_U8: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_DSD_U16_LE: u64 = 1 << 11;
const SNDRV_PCM_FMTBIT_DSD_U16_BE: u64 = 1 << 12;
const SNDRV_PCM_FMTBIT_DSD_U32_LE: u64 = 1 << 13;
const SNDRV_PCM_FMTBIT_DSD_U32_BE: u64 = 1 << 14;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1;
const SNDRV_PCM_RATE_8000_768000: c_uint = 2;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_LAST: c_int = 64;
const SNDRV_PCM_ACCESS_RW_INTERLEAVED_CONST: snd_pcm_access_t = SNDRV_PCM_ACCESS_RW_INTERLEAVED;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 1;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 2;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 3;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 4;
const SNDRV_DMA_TYPE_VMALLOC: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const GFP_KERNEL: c_uint = 0;
const HZ: c_uint = 100;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const HRTIMER_MODE_REL_SOFT: c_int = 0;
const HRTIMER_NORESTART: hrtimer_restart = 0;
const CLOCK_MONOTONIC: c_int = 1;
const EBUSY: c_int = 16;
const EIO: c_int = 5;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

static mut devices: [*mut platform_device; SNDRV_CARDS as usize] =
    [null_mut(); SNDRV_CARDS as usize];

unsafe fn DIV_ROUND_UP(n: c_ulong, d: c_uint) -> c_ulong {
    (n + d as c_ulong - 1) / d as c_ulong
}

unsafe fn byte_pos(dpcm: *mut loopback_pcm, mut x: c_uint) -> c_uint {
    if (*dpcm).pcm_rate_shift == NO_PITCH {
        x /= HZ;
    } else {
        x = div_u64(
            NO_PITCH as u64 * x as u64,
            HZ as u64 * (*dpcm).pcm_rate_shift as u64,
        );
    }
    x - (x % (*dpcm).pcm_salign)
}

unsafe fn frac_pos(dpcm: *mut loopback_pcm, mut x: c_uint) -> c_uint {
    if (*dpcm).pcm_rate_shift == NO_PITCH {
        return x * HZ;
    } else {
        x = div_u64((*dpcm).pcm_rate_shift as u64 * x as u64 * HZ as u64, NO_PITCH as u64);
    }
    x
}

unsafe fn get_setup(dpcm: *mut loopback_pcm) -> *mut loopback_setup {
    let mut device = (*(*(*(*dpcm).substream).pstr).pcm).device;
    if (*(*dpcm).substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        device ^= 1;
    }
    &mut (*(*dpcm).loopback).setup[(*(*dpcm).substream).number as usize][device as usize]
}

unsafe fn get_notify(dpcm: *mut loopback_pcm) -> c_uint {
    (*get_setup(dpcm)).notify
}

unsafe fn get_rate_shift(dpcm: *mut loopback_pcm) -> c_uint {
    (*get_setup(dpcm)).rate_shift
}

/* call in cable->lock */
unsafe extern "C" fn loopback_jiffies_timer_start(dpcm: *mut loopback_pcm) -> c_int {
    let mut tick: c_ulong;
    let rate_shift = get_rate_shift(dpcm);

    if rate_shift != (*dpcm).pcm_rate_shift {
        (*dpcm).pcm_rate_shift = rate_shift;
        (*dpcm).period_size_frac = frac_pos(dpcm, (*dpcm).pcm_period_size);
    }
    if (*dpcm).period_size_frac <= (*dpcm).irq_pos {
        (*dpcm).irq_pos %= (*dpcm).period_size_frac;
        (*dpcm).period_update_pending = 1;
    }
    tick = ((*dpcm).period_size_frac - (*dpcm).irq_pos) as c_ulong;
    tick = DIV_ROUND_UP(tick, (*dpcm).pcm_bps);
    mod_timer(&mut (*dpcm).timer, jiffies + tick);

    0
}

/* CONFIG_HIGH_RES_TIMERS: call in cable->lock */
unsafe extern "C" fn loopback_hrtimer_start(dpcm: *mut loopback_pcm) -> c_int {
    let mut tick: c_ulong;
    let rate_shift = get_rate_shift(dpcm);

    if rate_shift != (*dpcm).pcm_rate_shift {
        (*dpcm).pcm_rate_shift = rate_shift;
        (*dpcm).period_size_frac = frac_pos(dpcm, (*dpcm).pcm_period_size);
    }
    if (*dpcm).period_size_frac <= (*dpcm).irq_pos {
        (*dpcm).irq_pos %= (*dpcm).period_size_frac;
        (*dpcm).period_update_pending = 1;
    }
    tick = ((*dpcm).period_size_frac - (*dpcm).irq_pos) as c_ulong;
    tick = DIV_ROUND_UP(tick, (*dpcm).pcm_bps);
    hrtimer_start(
        &mut (*dpcm).hrtimer,
        ns_to_ktime(div_u64(tick as u64 * NSEC_PER_SEC, HZ as u64) as u64),
        HRTIMER_MODE_REL_SOFT,
    );

    0
}

/* call in cable->lock */
unsafe extern "C" fn loopback_snd_timer_start(dpcm: *mut loopback_pcm) -> c_int {
    let cable = (*dpcm).cable;
    let err = snd_timer_start((*cable).snd_timer.instance, 1);
    if err < 0 {
        if err == -EBUSY {
            return 0;
        }
        pcm_err(
            (*(*dpcm).substream).pcm,
            b"snd_timer_start(%d,%d,%d) failed with %d\0".as_ptr() as *const c_char,
            (*cable).snd_timer.id.card,
            (*cable).snd_timer.id.device,
            (*cable).snd_timer.id.subdevice,
            err,
        );
    }
    err
}

/* call in cable->lock */
unsafe extern "C" fn loopback_jiffies_timer_stop(dpcm: *mut loopback_pcm) -> c_int {
    timer_delete(&mut (*dpcm).timer);
    (*dpcm).timer.expires = 0;
    0
}

/* CONFIG_HIGH_RES_TIMERS: call in cable->lock */
unsafe extern "C" fn loopback_hrtimer_stop(dpcm: *mut loopback_pcm) -> c_int {
    hrtimer_try_to_cancel(&mut (*dpcm).hrtimer);
    0
}

/* call in cable->lock */
unsafe extern "C" fn loopback_snd_timer_stop(dpcm: *mut loopback_pcm) -> c_int {
    let cable = (*dpcm).cable;
    let err: c_int;

    /* only stop if both devices (playback and capture) are not running */
    if ((*cable).running ^ (*cable).pause) != 0 {
        return 0;
    }

    err = snd_timer_stop((*cable).snd_timer.instance);
    if err < 0 {
        pcm_err(
            (*(*dpcm).substream).pcm,
            b"snd_timer_stop(%d,%d,%d) failed with %d\0".as_ptr() as *const c_char,
            (*cable).snd_timer.id.card,
            (*cable).snd_timer.id.device,
            (*cable).snd_timer.id.subdevice,
            err,
        );
    }
    err
}

unsafe extern "C" fn loopback_jiffies_timer_stop_sync(dpcm: *mut loopback_pcm) -> c_int {
    timer_delete_sync(&mut (*dpcm).timer);
    0
}

/* CONFIG_HIGH_RES_TIMERS */
unsafe extern "C" fn loopback_hrtimer_stop_sync(dpcm: *mut loopback_pcm) -> c_int {
    hrtimer_cancel(&mut (*dpcm).hrtimer);
    0
}

/* call in loopback->cable_lock */
unsafe extern "C" fn loopback_snd_timer_close_cable(dpcm: *mut loopback_pcm) -> c_int {
    let cable = (*dpcm).cable;

    /* snd_timer was not opened */
    if (*cable).snd_timer.instance.is_null() {
        return 0;
    }

    snd_timer_close((*cable).snd_timer.instance);
    cancel_work_sync(&mut (*cable).snd_timer.event_work);
    snd_timer_instance_free((*cable).snd_timer.instance);
    memset(
        &mut (*cable).snd_timer as *mut _ as *mut c_void,
        0,
        size_of::<loopback_snd_timer>(),
    );

    0
}

unsafe fn is_access_interleaved(access: snd_pcm_access_t) -> bool_ {
    match access {
        SNDRV_PCM_ACCESS_MMAP_INTERLEAVED | SNDRV_PCM_ACCESS_RW_INTERLEAVED => true,
        _ => false,
    }
}

unsafe fn loopback_check_format(cable: *mut loopback_cable, stream: c_int) -> c_int {
    let dpcm_play: *mut loopback_pcm;
    let dpcm_capt: *mut loopback_pcm;
    let mut runtime: *mut snd_pcm_runtime;
    let cruntime: *mut snd_pcm_runtime;
    let setup: *mut loopback_setup;
    let card: *mut snd_card;
    let mut stop_capture = false;
    let check: c_int;

    dpcm_play = (*cable).streams[SNDRV_PCM_STREAM_PLAYBACK as usize];
    dpcm_capt = (*cable).streams[SNDRV_PCM_STREAM_CAPTURE as usize];

    if (*cable).valid != CABLE_VALID_BOTH {
        if stream == SNDRV_PCM_STREAM_CAPTURE || dpcm_play.is_null() {
            return 0;
        }
    } else {
        if dpcm_play.is_null() || dpcm_capt.is_null() {
            return -EIO;
        }
        runtime = (*(*dpcm_play).substream).runtime;
        cruntime = (*(*dpcm_capt).substream).runtime;
        if runtime.is_null() || cruntime.is_null() {
            return -EIO;
        }
        check = ((*runtime).format != (*cruntime).format
            || (*runtime).rate != (*cruntime).rate
            || (*runtime).channels != (*cruntime).channels
            || is_access_interleaved((*runtime).access) != is_access_interleaved((*cruntime).access))
            as c_int;
        if check == 0 {
            return 0;
        }
        if stream == SNDRV_PCM_STREAM_CAPTURE {
            return -EIO;
        } else if (*cruntime).state == SNDRV_PCM_STATE_RUNNING {
            /* close must not free the peer runtime below */
            snd_refcount_get(&mut (*cable).stop_count);
            stop_capture = true;
        }
    }

    setup = get_setup(dpcm_play);
    card = (*dpcm_play).loopback.as_ref().unwrap().card;
    runtime = (*(*dpcm_play).substream).runtime;
    if (*setup).format != (*runtime).format {
        snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*setup).format_id);
        (*setup).format = (*runtime).format;
    }
    if (*setup).rate != (*runtime).rate {
        snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*setup).rate_id);
        (*setup).rate = (*runtime).rate;
    }
    if (*setup).channels != (*runtime).channels {
        snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*setup).channels_id);
        (*setup).channels = (*runtime).channels;
    }
    if is_access_interleaved((*setup).access) != is_access_interleaved((*runtime).access) {
        snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*setup).access_id);
        (*setup).access = (*runtime).access;
    }

    if stop_capture {
        snd_pcm_stop((*dpcm_capt).substream, SNDRV_PCM_STATE_DRAINING);
        snd_refcount_put(&mut (*cable).stop_count);
    }

    0
}

unsafe fn loopback_active_notify(dpcm: *mut loopback_pcm) {
    snd_ctl_notify(
        (*(*dpcm).loopback).card,
        SNDRV_CTL_EVENT_MASK_VALUE,
        &mut (*get_setup(dpcm)).active_id,
    );
}

unsafe extern "C" fn loopback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut loopback_pcm;
    let cable = (*dpcm).cable;
    let mut err = 0;
    let stream = 1 << (*substream).stream;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            err = loopback_check_format(cable, (*substream).stream);
            if err < 0 {
                return err;
            }
            (*dpcm).last_jiffies = jiffies;
            (*dpcm).pcm_rate_shift = 0;
            (*dpcm).last_drift = 0;
            (*cable).running |= stream as c_uint;
            (*cable).pause &= !(stream as c_uint);
            err = ((*(*cable).ops).start.unwrap())(dpcm);
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                loopback_active_notify(dpcm);
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*cable).running &= !(stream as c_uint);
            (*cable).pause &= !(stream as c_uint);
            err = ((*(*cable).ops).stop.unwrap())(dpcm);
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                loopback_active_notify(dpcm);
            }
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*cable).pause |= stream as c_uint;
            err = ((*(*cable).ops).stop.unwrap())(dpcm);
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                loopback_active_notify(dpcm);
            }
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            (*dpcm).last_jiffies = jiffies;
            (*cable).pause &= !(stream as c_uint);
            err = ((*(*cable).ops).start.unwrap())(dpcm);
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                loopback_active_notify(dpcm);
            }
        }
        _ => return -EINVAL,
    }
    err
}

unsafe fn params_change(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut loopback_pcm;
    let cable = (*dpcm).cable;

    (*cable).hw.formats = pcm_format_to_bits((*runtime).format);
    (*cable).hw.rate_min = (*runtime).rate;
    (*cable).hw.rate_max = (*runtime).rate;
    (*cable).hw.channels_min = (*runtime).channels;
    (*cable).hw.channels_max = (*runtime).channels;

    if !(*cable).snd_timer.instance.is_null() {
        (*cable).hw.period_bytes_min = frames_to_bytes(runtime, (*runtime).period_size);
        (*cable).hw.period_bytes_max = (*cable).hw.period_bytes_min;
    }
}

unsafe extern "C" fn loopback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut loopback_pcm;
    let cable = (*dpcm).cable;
    let err: c_int;
    let bps: c_int;
    let salign: c_int;

    if let Some(stop_sync) = (*(*cable).ops).stop_sync {
        err = stop_sync(dpcm);
        if err < 0 {
            return err;
        }
    }

    salign = (snd_pcm_format_physical_width((*runtime).format) * (*runtime).channels as c_int) / 8;
    bps = salign * (*runtime).rate as c_int;
    if bps <= 0 || salign <= 0 {
        return -EINVAL;
    }

    (*dpcm).buf_pos = 0;
    (*dpcm).pcm_buffer_size = frames_to_bytes(runtime, (*runtime).buffer_size);
    (*dpcm).channel_buf_n = (*dpcm).pcm_buffer_size / (*runtime).channels;
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        /* clear capture buffer */
        (*dpcm).silent_size = (*dpcm).pcm_buffer_size;
        snd_pcm_format_set_silence(
            (*runtime).format,
            (*runtime).dma_area as *mut c_void,
            (*runtime).buffer_size * (*runtime).channels as c_ulong,
        );
    }

    (*dpcm).irq_pos = 0;
    (*dpcm).period_update_pending = 0;
    (*dpcm).pcm_bps = bps as c_uint;
    (*dpcm).pcm_salign = salign as c_uint;
    (*dpcm).pcm_period_size = frames_to_bytes(runtime, (*runtime).period_size);

    if ((*cable).valid & !(1 << (*substream).stream)) == 0
        || (get_notify(dpcm) != 0 && (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK)
    {
        params_change(substream);
    }
    (*cable).valid |= (1 << (*substream).stream) as c_uint;

    0
}

unsafe fn clear_capture_buf(dpcm: *mut loopback_pcm, mut bytes: c_uint) {
    let runtime = (*(*dpcm).substream).runtime;
    let dst = (*runtime).dma_area;
    let mut dst_off = (*dpcm).buf_pos;

    if (*dpcm).silent_size >= (*dpcm).pcm_buffer_size {
        return;
    }
    if (*dpcm).silent_size + bytes > (*dpcm).pcm_buffer_size {
        bytes = (*dpcm).pcm_buffer_size - (*dpcm).silent_size;
    }

    loop {
        let mut size = bytes;
        if dst_off + size > (*dpcm).pcm_buffer_size {
            size = (*dpcm).pcm_buffer_size - dst_off;
        }
        snd_pcm_format_set_silence(
            (*runtime).format,
            dst.add(dst_off as usize) as *mut c_void,
            bytes_to_frames(runtime, size) * (*runtime).channels as c_ulong,
        );
        (*dpcm).silent_size += size;
        bytes -= size;
        if bytes == 0 {
            break;
        }
        dst_off = 0;
    }
}

unsafe fn copy_play_buf_part_n(
    play: *mut loopback_pcm,
    capt: *mut loopback_pcm,
    size: c_uint,
    src_off: c_uint,
    dst_off: c_uint,
) {
    let channels = (*(*(*capt).substream).runtime).channels;
    let size_p_ch = size / channels;
    let src_off_ch = src_off / channels;
    let dst_off_ch = dst_off / channels;
    let mut i = 0;

    while i < channels {
        memcpy(
            (*(*(*capt).substream).runtime)
                .dma_area
                .add(((*capt).channel_buf_n * i + dst_off_ch) as usize) as *mut c_void,
            (*(*(*play).substream).runtime)
                .dma_area
                .add(((*play).channel_buf_n * i + src_off_ch) as usize) as *const c_void,
            size_p_ch as usize,
        );
        i += 1;
    }
}

unsafe fn copy_play_buf(play: *mut loopback_pcm, capt: *mut loopback_pcm, mut bytes: c_uint) {
    let runtime = (*(*play).substream).runtime;
    let src = (*runtime).dma_area;
    let dst = (*(*(*capt).substream).runtime).dma_area;
    let mut src_off = (*play).buf_pos;
    let mut dst_off = (*capt).buf_pos;
    let mut clear_bytes = 0;

    /* check if playback is draining, trim the capture copy size
     * when our pointer is at the end of playback ring buffer */
    if (*runtime).state == SNDRV_PCM_STATE_DRAINING
        && snd_pcm_playback_hw_avail(runtime) < (*runtime).buffer_size
    {
        let appl_ptr: snd_pcm_uframes_t;
        let mut appl_ptr1: snd_pcm_uframes_t;
        let diff: snd_pcm_uframes_t;
        appl_ptr = (*(*runtime).control).appl_ptr;
        appl_ptr1 = appl_ptr;
        appl_ptr1 -= appl_ptr1 % (*runtime).buffer_size;
        appl_ptr1 += (*play).buf_pos as c_ulong / (*play).pcm_salign as c_ulong;
        if appl_ptr < appl_ptr1 {
            appl_ptr1 -= (*runtime).buffer_size;
        }
        diff = (appl_ptr - appl_ptr1) * (*play).pcm_salign as c_ulong;
        if diff < bytes as c_ulong {
            clear_bytes = bytes - diff as c_uint;
            bytes = diff as c_uint;
        }
    }

    loop {
        let mut size = bytes;
        if src_off + size > (*play).pcm_buffer_size {
            size = (*play).pcm_buffer_size - src_off;
        }
        if dst_off + size > (*capt).pcm_buffer_size {
            size = (*capt).pcm_buffer_size - dst_off;
        }
        if !is_access_interleaved((*runtime).access) {
            copy_play_buf_part_n(play, capt, size, src_off, dst_off);
        } else {
            memcpy(
                dst.add(dst_off as usize) as *mut c_void,
                src.add(src_off as usize) as *const c_void,
                size as usize,
            );
        }
        (*capt).silent_size = 0;
        bytes -= size;
        if bytes == 0 {
            break;
        }
        src_off = (src_off + size) % (*play).pcm_buffer_size;
        dst_off = (dst_off + size) % (*capt).pcm_buffer_size;
    }

    if clear_bytes > 0 {
        clear_capture_buf(capt, clear_bytes);
        (*capt).silent_size = 0;
    }
}

unsafe fn bytepos_delta(dpcm: *mut loopback_pcm, jiffies_delta: c_uint) -> c_uint {
    let last_pos: c_ulong;
    let mut delta: c_uint;

    last_pos = byte_pos(dpcm, (*dpcm).irq_pos) as c_ulong;
    (*dpcm).irq_pos += jiffies_delta * (*dpcm).pcm_bps;
    delta = byte_pos(dpcm, (*dpcm).irq_pos) - last_pos as c_uint;
    if delta >= (*dpcm).last_drift {
        delta -= (*dpcm).last_drift;
    }
    (*dpcm).last_drift = 0;
    if (*dpcm).irq_pos >= (*dpcm).period_size_frac {
        (*dpcm).irq_pos %= (*dpcm).period_size_frac;
        (*dpcm).period_update_pending = 1;
    }
    delta
}

unsafe fn bytepos_finish(dpcm: *mut loopback_pcm, delta: c_uint) {
    (*dpcm).buf_pos += delta;
    (*dpcm).buf_pos %= (*dpcm).pcm_buffer_size;
}

/* call in cable->lock */
unsafe extern "C" fn loopback_jiffies_timer_pos_update(cable: *mut loopback_cable) -> c_uint {
    let dpcm_play = (*cable).streams[SNDRV_PCM_STREAM_PLAYBACK as usize];
    let dpcm_capt = (*cable).streams[SNDRV_PCM_STREAM_CAPTURE as usize];
    let mut delta_play: c_ulong = 0;
    let mut delta_capt: c_ulong = 0;
    let cur_jiffies: c_ulong;
    let running: c_uint;
    let mut count1: c_uint;
    let count2: c_uint;

    cur_jiffies = jiffies;
    running = (*cable).running ^ (*cable).pause;
    if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
        delta_play = cur_jiffies - (*dpcm_play).last_jiffies;
        (*dpcm_play).last_jiffies += delta_play;
    }

    if (running & (1 << SNDRV_PCM_STREAM_CAPTURE)) != 0 {
        delta_capt = cur_jiffies - (*dpcm_capt).last_jiffies;
        (*dpcm_capt).last_jiffies += delta_capt;
    }

    if delta_play == 0 && delta_capt == 0 {
        return running;
    }

    if delta_play > delta_capt {
        count1 = bytepos_delta(dpcm_play, (delta_play - delta_capt) as c_uint);
        bytepos_finish(dpcm_play, count1);
        delta_play = delta_capt;
    } else if delta_play < delta_capt {
        count1 = bytepos_delta(dpcm_capt, (delta_capt - delta_play) as c_uint);
        clear_capture_buf(dpcm_capt, count1);
        bytepos_finish(dpcm_capt, count1);
        delta_capt = delta_play;
    }

    if delta_play == 0 && delta_capt == 0 {
        return running;
    }

    /* note delta_capt == delta_play at this moment */
    count1 = bytepos_delta(dpcm_play, delta_play as c_uint);
    count2 = bytepos_delta(dpcm_capt, delta_capt as c_uint);
    if count1 < count2 {
        (*dpcm_capt).last_drift = count2 - count1;
        count1 = count2;
    } else if count1 > count2 {
        (*dpcm_play).last_drift = count1 - count2;
    }
    copy_play_buf(dpcm_play, dpcm_capt, count1);
    bytepos_finish(dpcm_play, count1);
    bytepos_finish(dpcm_capt, count1);
    running
}

unsafe extern "C" fn loopback_jiffies_timer_function(t: *mut timer_list) {
    let dpcm = t as *mut loopback_pcm;
    let mut period_elapsed = false;

    if (loopback_jiffies_timer_pos_update((*dpcm).cable) & (1 << (*(*dpcm).substream).stream)) != 0 {
        loopback_jiffies_timer_start(dpcm);
        if (*dpcm).period_update_pending != 0 {
            (*dpcm).period_update_pending = 0;
            period_elapsed = true;
        }
    }

    if period_elapsed {
        snd_pcm_period_elapsed((*dpcm).substream);
    }
}

/* CONFIG_HIGH_RES_TIMERS */
unsafe extern "C" fn loopback_hrtimer_function(t: *mut hrtimer) -> hrtimer_restart {
    let dpcm = t as *mut loopback_pcm;
    let mut period_elapsed = false;

    if (loopback_jiffies_timer_pos_update((*dpcm).cable) & (1 << (*(*dpcm).substream).stream)) != 0 {
        loopback_hrtimer_start(dpcm);
        if (*dpcm).period_update_pending != 0 {
            (*dpcm).period_update_pending = 0;
            period_elapsed = true;
        }
    }

    if period_elapsed {
        snd_pcm_period_elapsed((*dpcm).substream);
    }

    HRTIMER_NORESTART
}

/* call in cable->lock */
unsafe fn loopback_snd_timer_check_resolution(
    runtime: *mut snd_pcm_runtime,
    resolution: c_ulong,
) -> c_int {
    if resolution != (*runtime).timer_resolution {
        let dpcm = (*runtime).private_data as *mut loopback_pcm;
        let cable = (*dpcm).cable;
        /* Worst case estimation of possible values for resolution ... */
        let period_size_usec: snd_pcm_uframes_t = resolution / 1000 * (*runtime).rate as c_ulong;
        /* round to nearest sample rate */
        let period_size: snd_pcm_uframes_t = (period_size_usec + 500 * 1000) / (1000 * 1000);

        pcm_err(
            (*(*dpcm).substream).pcm,
            b"Period size (%lu frames) of loopback device is not corresponding to timer resolution (%lu nsec = %lu frames) of card timer %d,%d,%d. Use period size of %lu frames for loopback device.\0".as_ptr() as *const c_char,
            (*runtime).period_size,
            resolution,
            period_size,
            (*cable).snd_timer.id.card,
            (*cable).snd_timer.id.device,
            (*cable).snd_timer.id.subdevice,
            period_size,
        );
        return -EINVAL;
    }
    0
}

unsafe fn loopback_snd_timer_period_elapsed(
    cable: *mut loopback_cable,
    event: c_int,
    resolution: c_ulong,
) {
    let dpcm_play: *mut loopback_pcm;
    let dpcm_capt: *mut loopback_pcm;
    let substream_play: *mut snd_pcm_substream;
    let substream_capt: *mut snd_pcm_substream;
    let valid_runtime: *mut snd_pcm_runtime;
    let running: c_uint;
    let elapsed_bytes: c_uint;
    let mut xrun = false;

    running = (*cable).running ^ (*cable).pause;
    /* no need to do anything if no stream is running */
    if running == 0 {
        return;
    }

    dpcm_play = (*cable).streams[SNDRV_PCM_STREAM_PLAYBACK as usize];
    dpcm_capt = (*cable).streams[SNDRV_PCM_STREAM_CAPTURE as usize];

    if event == SNDRV_TIMER_EVENT_MSTOP {
        if dpcm_play.is_null() || (*(*dpcm_play).substream).runtime.as_ref().unwrap().state != SNDRV_PCM_STATE_DRAINING {
            return;
        }
    }

    substream_play = if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
        (*dpcm_play).substream
    } else {
        null_mut()
    };
    substream_capt = if (running & (1 << SNDRV_PCM_STREAM_CAPTURE)) != 0 {
        (*dpcm_capt).substream
    } else {
        null_mut()
    };
    valid_runtime = if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
        (*(*dpcm_play).substream).runtime
    } else {
        (*(*dpcm_capt).substream).runtime
    };

    /* resolution is only valid for SNDRV_TIMER_EVENT_TICK events */
    if event == SNDRV_TIMER_EVENT_TICK {
        if loopback_snd_timer_check_resolution(valid_runtime, resolution) < 0 {
            xrun = true;
        }
    }

    if !xrun {
        elapsed_bytes = frames_to_bytes(valid_runtime, (*valid_runtime).period_size);
        /* The same timer interrupt is used for playback and capture device */
        if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) != 0
            && (running & (1 << SNDRV_PCM_STREAM_CAPTURE)) != 0
        {
            copy_play_buf(dpcm_play, dpcm_capt, elapsed_bytes);
            bytepos_finish(dpcm_play, elapsed_bytes);
            bytepos_finish(dpcm_capt, elapsed_bytes);
        } else if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
            bytepos_finish(dpcm_play, elapsed_bytes);
        } else if (running & (1 << SNDRV_PCM_STREAM_CAPTURE)) != 0 {
            clear_capture_buf(dpcm_capt, elapsed_bytes);
            bytepos_finish(dpcm_capt, elapsed_bytes);
        }
    }

    if xrun {
        if !substream_play.is_null() {
            snd_pcm_stop_xrun(substream_play);
        }
        if !substream_capt.is_null() {
            snd_pcm_stop_xrun(substream_capt);
        }
        return;
    }

    if !substream_play.is_null() {
        snd_pcm_period_elapsed(substream_play);
    }
    if !substream_capt.is_null() {
        snd_pcm_period_elapsed(substream_capt);
    }
}

unsafe extern "C" fn loopback_snd_timer_function(
    timeri: *mut snd_timer_instance,
    resolution: c_ulong,
    _ticks: c_ulong,
) {
    let cable = (*timeri).callback_data as *mut loopback_cable;
    loopback_snd_timer_period_elapsed(cable, SNDRV_TIMER_EVENT_TICK, resolution);
}

unsafe extern "C" fn loopback_snd_timer_work(work: *mut work_struct) {
    let cable = work as *mut loopback_cable;
    loopback_snd_timer_period_elapsed(cable, SNDRV_TIMER_EVENT_MSTOP, 0);
}

unsafe extern "C" fn loopback_snd_timer_event(
    timeri: *mut snd_timer_instance,
    event: c_int,
    _tstamp: *mut timespec64,
    _resolution: c_ulong,
) {
    /* Do not lock cable->lock here because timer->lock is already hold. */
    if event == SNDRV_TIMER_EVENT_MSTOP {
        let cable = (*timeri).callback_data as *mut loopback_cable;
        schedule_work(&mut (*cable).snd_timer.event_work);
    }
}

unsafe extern "C" fn loopback_jiffies_timer_dpcm_info(
    dpcm: *mut loopback_pcm,
    buffer: *mut snd_info_buffer,
) {
    snd_iprintf(buffer, b"    update_pending:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).period_update_pending);
    snd_iprintf(buffer, b"    irq_pos:\t\t%u\n\0".as_ptr() as *const c_char, (*dpcm).irq_pos);
    snd_iprintf(buffer, b"    period_frac:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).period_size_frac);
    snd_iprintf(buffer, b"    last_jiffies:\t%lu (%lu)\n\0".as_ptr() as *const c_char, (*dpcm).last_jiffies, jiffies);
    snd_iprintf(buffer, b"    timer_expires:\t%lu\n\0".as_ptr() as *const c_char, (*dpcm).timer.expires);
}

/* CONFIG_HIGH_RES_TIMERS */
unsafe extern "C" fn loopback_hrtimer_dpcm_info(dpcm: *mut loopback_pcm, buffer: *mut snd_info_buffer) {
    snd_iprintf(buffer, b"    update_pending:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).period_update_pending);
    snd_iprintf(buffer, b"    irq_pos:\t\t%u\n\0".as_ptr() as *const c_char, (*dpcm).irq_pos);
    snd_iprintf(buffer, b"    period_frac:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).period_size_frac);
    snd_iprintf(buffer, b"    last_jiffies:\t%lu (%lu)\n\0".as_ptr() as *const c_char, (*dpcm).last_jiffies, jiffies);
    snd_iprintf(buffer, b"    timer_expires:\t%llu\n\0".as_ptr() as *const c_char, ktime_to_ns(hrtimer_get_expires(&mut (*dpcm).hrtimer)));
}

unsafe extern "C" fn loopback_snd_timer_dpcm_info(dpcm: *mut loopback_pcm, buffer: *mut snd_info_buffer) {
    let cable = (*dpcm).cable;
    snd_iprintf(buffer, b"    sound timer:\thw:%d,%d,%d\n\0".as_ptr() as *const c_char, (*cable).snd_timer.id.card, (*cable).snd_timer.id.device, (*cable).snd_timer.id.subdevice);
    snd_iprintf(buffer, b"    timer open:\t\t%s\n\0".as_ptr() as *const c_char, snd_pcm_direction_name((*cable).snd_timer.stream));
}

unsafe extern "C" fn loopback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut loopback_pcm;
    let pos: snd_pcm_uframes_t;

    if let Some(pos_update) = (*(*(*dpcm).cable).ops).pos_update {
        pos_update((*dpcm).cable);
    }
    pos = (*dpcm).buf_pos as snd_pcm_uframes_t;
    bytes_to_frames(runtime, pos as c_uint)
}

static loopback_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NONINTERLEAVED,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S16_BE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S24_BE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S24_3BE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_S32_BE
        | SNDRV_PCM_FMTBIT_FLOAT_LE
        | SNDRV_PCM_FMTBIT_FLOAT_BE
        | SNDRV_PCM_FMTBIT_DSD_U8
        | SNDRV_PCM_FMTBIT_DSD_U16_LE
        | SNDRV_PCM_FMTBIT_DSD_U16_BE
        | SNDRV_PCM_FMTBIT_DSD_U32_LE
        | SNDRV_PCM_FMTBIT_DSD_U32_BE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_768000,
    rate_min: 8000,
    rate_max: 768000,
    channels_min: 1,
    channels_max: 32,
    buffer_bytes_max: 2 * 1024 * 1024,
    period_bytes_min: 64,
    /* note check overflow in frac_pos() using pcm_rate_shift before
       changing period_bytes_max value */
    period_bytes_max: 1024 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe extern "C" fn loopback_runtime_free(runtime: *mut snd_pcm_runtime) {
    let dpcm = (*runtime).private_data as *mut loopback_pcm;
    kfree(dpcm as *mut c_void);
}

unsafe extern "C" fn loopback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut loopback_pcm;
    let cable = (*dpcm).cable;

    (*cable).valid &= !(1 << (*substream).stream);
    0
}

unsafe fn get_cable_index(substream: *mut snd_pcm_substream) -> c_uint {
    if (*(*substream).pcm).device == 0 {
        (*substream).stream as c_uint
    } else {
        (!(*substream).stream) as c_uint
    }
}

unsafe extern "C" fn rule_format(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let dpcm = (*rule).private as *mut loopback_pcm;
    let cable = (*dpcm).cable;
    let mut m: snd_mask = zeroed();

    snd_mask_none(&mut m);
    m.bits[0] = (*cable).hw.formats as u_int32_t;
    m.bits[1] = ((*cable).hw.formats >> 32) as u_int32_t;
    snd_mask_refine(hw_param_mask(params, (*rule).var), &mut m)
}

unsafe extern "C" fn rule_rate(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let dpcm = (*rule).private as *mut loopback_pcm;
    let cable = (*dpcm).cable;
    let mut t: snd_interval = zeroed();

    t.min = (*cable).hw.rate_min;
    t.max = (*cable).hw.rate_max;
    t.openmin = 0;
    t.openmax = 0;
    t.integer = 0;
    snd_interval_refine(hw_param_interval(params, (*rule).var), &mut t)
}

unsafe extern "C" fn rule_channels(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let dpcm = (*rule).private as *mut loopback_pcm;
    let cable = (*dpcm).cable;
    let mut t: snd_interval = zeroed();

    t.min = (*cable).hw.channels_min;
    t.max = (*cable).hw.channels_max;
    t.openmin = 0;
    t.openmax = 0;
    t.integer = 0;
    snd_interval_refine(hw_param_interval(params, (*rule).var), &mut t)
}

unsafe extern "C" fn rule_period_bytes(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let dpcm = (*rule).private as *mut loopback_pcm;
    let cable = (*dpcm).cable;
    let mut t: snd_interval = zeroed();

    t.min = (*cable).hw.period_bytes_min;
    t.max = (*cable).hw.period_bytes_max;
    t.openmin = 0;
    t.openmax = 0;
    t.integer = 0;
    snd_interval_refine(hw_param_interval(params, (*rule).var), &mut t)
}

unsafe fn free_cable(substream: *mut snd_pcm_substream) {
    let loopback = (*substream).private_data as *mut loopback;
    let dev = get_cable_index(substream) as usize;
    let cable: *mut loopback_cable;
    let dpcm: *mut loopback_pcm;
    let other_alive: bool_;

    cable = (*loopback).cables[(*substream).number as usize][dev];
    if cable.is_null() {
        return;
    }

    (*cable).streams[(*substream).stream as usize] = null_mut();
    other_alive = !(*cable).streams[(!(*substream).stream) as usize].is_null();

    /* Pair with the stop_count increment in loopback_check_format(). */
    snd_refcount_sync(&mut (*cable).stop_count);
    if other_alive {
        return;
    }

    dpcm = (*(*substream).runtime).private_data as *mut loopback_pcm;
    if !(*cable).ops.is_null() && (*(*cable).ops).close_cable.is_some() && !dpcm.is_null() {
        ((*(*cable).ops).close_cable.unwrap())(dpcm);
    }
    /* free the cable */
    (*loopback).cables[(*substream).number as usize][dev] = null_mut();
    kfree(cable as *mut c_void);
}

unsafe extern "C" fn loopback_jiffies_timer_open(dpcm: *mut loopback_pcm) -> c_int {
    timer_setup(&mut (*dpcm).timer, loopback_jiffies_timer_function, 0);
    0
}

static loopback_jiffies_timer_ops: loopback_ops = loopback_ops {
    open: Some(loopback_jiffies_timer_open),
    start: Some(loopback_jiffies_timer_start),
    stop: Some(loopback_jiffies_timer_stop),
    stop_sync: Some(loopback_jiffies_timer_stop_sync),
    close_substream: Some(loopback_jiffies_timer_stop_sync),
    close_cable: None,
    pos_update: Some(loopback_jiffies_timer_pos_update),
    dpcm_info: Some(loopback_jiffies_timer_dpcm_info),
};

/* CONFIG_HIGH_RES_TIMERS */
unsafe extern "C" fn loopback_hrtimer_open(dpcm: *mut loopback_pcm) -> c_int {
    hrtimer_setup(
        &mut (*dpcm).hrtimer,
        loopback_hrtimer_function,
        CLOCK_MONOTONIC,
        HRTIMER_MODE_REL_SOFT,
    );
    0
}

static loopback_hrtimer_ops: loopback_ops = loopback_ops {
    open: Some(loopback_hrtimer_open),
    start: Some(loopback_hrtimer_start),
    stop: Some(loopback_hrtimer_stop),
    stop_sync: Some(loopback_hrtimer_stop_sync),
    close_substream: Some(loopback_hrtimer_stop_sync),
    close_cable: None,
    pos_update: Some(loopback_jiffies_timer_pos_update),
    dpcm_info: Some(loopback_hrtimer_dpcm_info),
};

unsafe fn loopback_parse_timer_id(str_: *const c_char, tid: *mut snd_timer_id) -> c_int {
    /* [<pref>:](<card name>|<card idx>)[{.,}<dev idx>[{.,}<subdev idx>]] */
    let sep_dev = b".,\0".as_ptr() as *const c_char;
    let sep_pref = b":\0".as_ptr() as *const c_char;
    let mut name = str_;
    let mut sep: *mut c_char;
    let mut save: c_char = 0;
    let mut card_idx: c_int = 0;
    let mut dev: c_int = 0;
    let mut subdev: c_int = 0;
    let mut err: c_int;

    sep = strpbrk(str_, sep_pref);
    if !sep.is_null() {
        name = sep.add(1);
    }
    sep = strpbrk(name, sep_dev);
    if !sep.is_null() {
        save = *sep;
        *sep = 0;
    }
    err = kstrtoint(name, 0, &mut card_idx);
    if err == -EINVAL {
        /* Must be the name, not number */
        card_idx = 0;
        while card_idx < snd_ecards_limit {
            let card = snd_card_ref(card_idx);

            if !card.is_null() {
                if strcmp((*card).id, name) == 0 {
                    err = 0;
                }
                snd_card_unref(card);
            }
            if err == 0 {
                break;
            }
            card_idx += 1;
        }
    }
    if !sep.is_null() {
        *sep = save;
        if err == 0 {
            let mut sep2: *mut c_char;
            let mut save2: c_char = 0;

            sep2 = strpbrk(sep.add(1), sep_dev);
            if !sep2.is_null() {
                save2 = *sep2;
                *sep2 = 0;
            }
            err = kstrtoint(sep.add(1), 0, &mut dev);
            if !sep2.is_null() {
                *sep2 = save2;
                if err == 0 {
                    err = kstrtoint(sep2.add(1), 0, &mut subdev);
                }
            }
        }
    }
    if card_idx == -1 {
        (*tid).dev_class = SNDRV_TIMER_CLASS_GLOBAL;
    }
    if err == 0 && !tid.is_null() {
        (*tid).card = card_idx;
        (*tid).device = dev;
        (*tid).subdevice = subdev;
    }
    err
}

/* call in loopback->cable_lock */
unsafe extern "C" fn loopback_snd_timer_open(dpcm: *mut loopback_pcm) -> c_int {
    let mut err = 0;
    let mut tid = snd_timer_id {
        dev_class: SNDRV_TIMER_CLASS_PCM,
        dev_sclass: SNDRV_TIMER_SCLASS_APPLICATION,
        card: 0,
        device: 0,
        subdevice: 0,
    };
    let timeri: *mut snd_timer_instance;
    let cable = (*dpcm).cable;

    if !(*cable).snd_timer.instance.is_null() {
        return err;
    }

    err = loopback_parse_timer_id((*(*dpcm).loopback).timer_source, &mut tid);
    if err < 0 {
        pcm_err((*(*dpcm).substream).pcm, b"Parsing timer source '%s' failed with %d\0".as_ptr() as *const c_char, (*(*dpcm).loopback).timer_source, err);
        return err;
    }

    (*cable).snd_timer.stream = (*(*dpcm).substream).stream;
    (*cable).snd_timer.id = tid;

    timeri = snd_timer_instance_new((*(*(*dpcm).loopback).card).id);
    if timeri.is_null() {
        return -ENOMEM;
    }
    (*timeri).flags |= SNDRV_TIMER_IFLG_AUTO;
    (*timeri).callback = Some(loopback_snd_timer_function);
    (*timeri).callback_data = cable as *mut c_void;
    (*timeri).ccallback = Some(loopback_snd_timer_event);

    /* initialise a work used for draining */
    INIT_WORK(&mut (*cable).snd_timer.event_work, loopback_snd_timer_work);

    err = snd_timer_open(timeri, &mut (*cable).snd_timer.id, (*current).pid);
    if err < 0 {
        pcm_err(
            (*(*dpcm).substream).pcm,
            b"snd_timer_open (%d,%d,%d) failed with %d\0".as_ptr() as *const c_char,
            (*cable).snd_timer.id.card,
            (*cable).snd_timer.id.device,
            (*cable).snd_timer.id.subdevice,
            err,
        );
        snd_timer_instance_free(timeri);
        return err;
    }

    (*cable).snd_timer.instance = timeri;
    err
}

/* stop_sync() is not required for sound timer because it does not need to be
 * restarted in loopback_prepare() on Xrun recovery
 */
static loopback_snd_timer_ops: loopback_ops = loopback_ops {
    open: Some(loopback_snd_timer_open),
    start: Some(loopback_snd_timer_start),
    stop: Some(loopback_snd_timer_stop),
    stop_sync: None,
    close_substream: None,
    close_cable: Some(loopback_snd_timer_close_cable),
    pos_update: None,
    dpcm_info: Some(loopback_snd_timer_dpcm_info),
};

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

unsafe extern "C" fn loopback_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let loopback = (*substream).private_data as *mut loopback;
    let dpcm: *mut loopback_pcm;
    let mut cable: *mut loopback_cable;
    let mut err = 0;
    let dev = get_cable_index(substream) as usize;

    dpcm = kzalloc(size_of::<loopback_pcm>(), GFP_KERNEL) as *mut loopback_pcm;
    if dpcm.is_null() {
        return -ENOMEM;
    }
    (*dpcm).loopback = loopback;
    (*dpcm).substream = substream;

    cable = (*loopback).cables[(*substream).number as usize][dev];
    if cable.is_null() {
        cable = kzalloc(size_of::<loopback_cable>(), GFP_KERNEL) as *mut loopback_cable;
        if cable.is_null() {
            err = -ENOMEM;
            kfree(dpcm as *mut c_void);
            return err;
        }
        spin_lock_init(&mut (*cable).lock);
        snd_refcount_init(&mut (*cable).stop_count);
        (*cable).hw = loopback_pcm_hardware;
        if !(*loopback).timer_source.is_null()
            && strcmp((*loopback).timer_source, b"hrtimer\0".as_ptr() as *const c_char) == 0
        {
            (*cable).ops = &loopback_hrtimer_ops;
        } else if !(*loopback).timer_source.is_null() && *(*loopback).timer_source != 0 {
            (*cable).ops = &loopback_snd_timer_ops;
        } else {
            (*cable).ops = &loopback_jiffies_timer_ops;
        }
        (*loopback).cables[(*substream).number as usize][dev] = cable;
    }
    (*dpcm).cable = cable;
    (*runtime).private_data = dpcm as *mut c_void;

    if let Some(open) = (*(*cable).ops).open {
        err = open(dpcm);
        if err < 0 {
            free_cable(substream);
            kfree(dpcm as *mut c_void);
            return err;
        }
    }

    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);

    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT, rule_format, dpcm as *mut c_void, SNDRV_PCM_HW_PARAM_FORMAT, -1);
    if err < 0 {
        free_cable(substream);
        kfree(dpcm as *mut c_void);
        return err;
    }
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, rule_rate, dpcm as *mut c_void, SNDRV_PCM_HW_PARAM_RATE, -1);
    if err < 0 {
        free_cable(substream);
        kfree(dpcm as *mut c_void);
        return err;
    }
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, rule_channels, dpcm as *mut c_void, SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if err < 0 {
        free_cable(substream);
        kfree(dpcm as *mut c_void);
        return err;
    }

    if !(*cable).snd_timer.instance.is_null() {
        err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, rule_period_bytes, dpcm as *mut c_void, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, -1);
        if err < 0 {
            free_cable(substream);
            kfree(dpcm as *mut c_void);
            return err;
        }
    }

    (*runtime).private_free = Some(loopback_runtime_free);
    if get_notify(dpcm) != 0 {
        (*runtime).hw = loopback_pcm_hardware;
    } else {
        (*runtime).hw = (*cable).hw;
    }

    (*cable).streams[(*substream).stream as usize] = dpcm;
    err
}

unsafe extern "C" fn loopback_close(substream: *mut snd_pcm_substream) -> c_int {
    let loopback = (*substream).private_data as *mut loopback;
    let dpcm = (*(*substream).runtime).private_data as *mut loopback_pcm;
    let mut err = 0;

    if let Some(close_substream) = (*(*(*dpcm).cable).ops).close_substream {
        err = close_substream(dpcm);
    }
    let _ = loopback;
    free_cable(substream);
    err
}

static loopback_pcm_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(loopback_open),
    close: Some(loopback_close),
    hw_free: Some(loopback_hw_free),
    prepare: Some(loopback_prepare),
    trigger: Some(loopback_trigger),
    pointer: Some(loopback_pointer),
};

unsafe fn loopback_pcm_new(loopback: *mut loopback, device: c_int, substreams: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = null_mut();
    let mut err: c_int;

    err = snd_pcm_new((*loopback).card, b"Loopback PCM\0".as_ptr() as *const c_char, device, substreams, substreams, &mut pcm);
    if err < 0 {
        return err;
    }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &loopback_pcm_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &loopback_pcm_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, null_mut(), 0, 0);

    (*pcm).private_data = loopback as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), b"Loopback PCM\0".as_ptr() as *const c_char);

    (*loopback).pcm[device as usize] = pcm;
    0
}

unsafe extern "C" fn loopback_rate_shift_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 80000;
    (*uinfo).value.integer.max = 120000;
    (*uinfo).value.integer.step = 1;
    0
}

unsafe extern "C" fn loopback_rate_shift_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    (*ucontrol).value.integer.value[0] =
        (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].rate_shift as i64;
    0
}

unsafe extern "C" fn loopback_rate_shift_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    let mut val: c_uint;
    let mut change = 0;

    val = (*ucontrol).value.integer.value[0] as c_uint;
    if val < 80000 {
        val = 80000;
    }
    if val > 120000 {
        val = 120000;
    }
    if val != (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].rate_shift {
        (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].rate_shift = val;
        change = 1;
    }
    change
}

unsafe extern "C" fn loopback_notify_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    (*ucontrol).value.integer.value[0] =
        (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].notify as i64;
    0
}

unsafe extern "C" fn loopback_notify_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    let val: c_uint = if (*ucontrol).value.integer.value[0] != 0 { 1 } else { 0 };
    let mut change = 0;

    if val != (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].notify {
        (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].notify = val;
        change = 1;
    }
    change
}

unsafe extern "C" fn loopback_active_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    let cable: *mut loopback_cable;
    let mut val: c_uint = 0;

    cable = (*loopback).cables[(*kcontrol).id.subdevice as usize][((*kcontrol).id.device ^ 1) as usize];
    if !cable.is_null() {
        let running = (*cable).running ^ (*cable).pause;
        val = if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) != 0 { 1 } else { 0 };
    }
    (*ucontrol).value.integer.value[0] = val as i64;
    0
}

unsafe extern "C" fn loopback_format_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = SNDRV_PCM_FORMAT_LAST as i64;
    (*uinfo).value.integer.step = 1;
    0
}

unsafe extern "C" fn loopback_format_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    (*ucontrol).value.integer.value[0] =
        (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].format as i64;
    0
}

unsafe extern "C" fn loopback_rate_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 192000;
    (*uinfo).value.integer.step = 1;
    0
}

unsafe extern "C" fn loopback_rate_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    (*ucontrol).value.integer.value[0] =
        (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].rate as i64;
    0
}

unsafe extern "C" fn loopback_channels_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 1;
    (*uinfo).value.integer.max = 1024;
    (*uinfo).value.integer.step = 1;
    0
}

unsafe extern "C" fn loopback_channels_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    (*ucontrol).value.integer.value[0] =
        (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].channels as i64;
    0
}

unsafe extern "C" fn loopback_access_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 2] = [
        b"Interleaved\0".as_ptr() as *const c_char,
        b"Non-interleaved\0".as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, texts.len() as c_uint, texts.as_ptr())
}

unsafe extern "C" fn loopback_access_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let loopback = snd_kcontrol_chip(kcontrol) as *mut loopback;
    let access: snd_pcm_access_t;

    access = (*loopback).setup[(*kcontrol).id.subdevice as usize][(*kcontrol).id.device as usize].access;
    (*ucontrol).value.enumerated.item[0] = (!is_access_interleaved(access)) as c_uint;
    0
}

static loopback_controls: [snd_kcontrol_new; 7] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"PCM Rate Shift 100000\0".as_ptr() as *const c_char,
        access: 0,
        info: Some(loopback_rate_shift_info),
        get: Some(loopback_rate_shift_get),
        put: Some(loopback_rate_shift_put),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"PCM Notify\0".as_ptr() as *const c_char,
        access: 0,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(loopback_notify_get),
        put: Some(loopback_notify_put),
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"PCM Slave Active\0".as_ptr() as *const c_char,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(loopback_active_get),
        put: None,
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"PCM Slave Format\0".as_ptr() as *const c_char,
        info: Some(loopback_format_info),
        get: Some(loopback_format_get),
        put: None,
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"PCM Slave Rate\0".as_ptr() as *const c_char,
        info: Some(loopback_rate_info),
        get: Some(loopback_rate_get),
        put: None,
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"PCM Slave Channels\0".as_ptr() as *const c_char,
        info: Some(loopback_channels_info),
        get: Some(loopback_channels_get),
        put: None,
    },
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"PCM Slave Access Mode\0".as_ptr() as *const c_char,
        info: Some(loopback_access_info),
        get: Some(loopback_access_get),
        put: None,
    },
];

unsafe fn loopback_mixer_new(loopback: *mut loopback, notify: c_int) -> c_int {
    let card = (*loopback).card;
    let mut pcm: *mut snd_pcm;
    let mut kctl: *mut snd_kcontrol;
    let mut setup: *mut loopback_setup;
    let mut err: c_int;
    let mut dev: c_int;
    let mut substr: c_int;
    let substr_count: c_int;
    let mut idx: usize;

    strscpy((*card).mixername.as_mut_ptr(), b"Loopback Mixer\0".as_ptr() as *const c_char);
    dev = 0;
    while dev < 2 {
        pcm = (*loopback).pcm[dev as usize];
        substr_count = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream_count;
        substr = 0;
        while substr < substr_count {
            setup = &mut (*loopback).setup[substr as usize][dev as usize];
            (*setup).notify = notify as c_uint;
            (*setup).rate_shift = NO_PITCH;
            (*setup).format = SNDRV_PCM_FORMAT_S16_LE;
            (*setup).access = SNDRV_PCM_ACCESS_RW_INTERLEAVED_CONST;
            (*setup).rate = 48000;
            (*setup).channels = 2;
            idx = 0;
            while idx < loopback_controls.len() {
                kctl = snd_ctl_new1(&loopback_controls[idx], loopback as *mut c_void);
                if kctl.is_null() {
                    return -ENOMEM;
                }
                (*kctl).id.device = dev as c_uint;
                (*kctl).id.subdevice = substr as c_uint;

                /* Add the control before copying the id so that
                 * the numid field of the id is set in the copy.
                 */
                err = snd_ctl_add(card, kctl);
                if err < 0 {
                    return err;
                }

                match idx {
                    ACTIVE_IDX => (*setup).active_id = core::ptr::read(&(*kctl).id as *const _ as *const snd_ctl_elem_id),
                    FORMAT_IDX => (*setup).format_id = core::ptr::read(&(*kctl).id as *const _ as *const snd_ctl_elem_id),
                    RATE_IDX => (*setup).rate_id = core::ptr::read(&(*kctl).id as *const _ as *const snd_ctl_elem_id),
                    CHANNELS_IDX => (*setup).channels_id = core::ptr::read(&(*kctl).id as *const _ as *const snd_ctl_elem_id),
                    ACCESS_IDX => (*setup).access_id = core::ptr::read(&(*kctl).id as *const _ as *const snd_ctl_elem_id),
                    _ => {}
                }
                idx += 1;
            }
            substr += 1;
        }
        dev += 1;
    }
    0
}

unsafe fn print_dpcm_info(buffer: *mut snd_info_buffer, dpcm: *mut loopback_pcm, id: *const c_char) {
    snd_iprintf(buffer, b"  %s\n\0".as_ptr() as *const c_char, id);
    if dpcm.is_null() {
        snd_iprintf(buffer, b"    inactive\n\0".as_ptr() as *const c_char);
        return;
    }
    snd_iprintf(buffer, b"    buffer_size:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).pcm_buffer_size);
    snd_iprintf(buffer, b"    buffer_pos:\t\t%u\n\0".as_ptr() as *const c_char, (*dpcm).buf_pos);
    snd_iprintf(buffer, b"    silent_size:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).silent_size);
    snd_iprintf(buffer, b"    period_size:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).pcm_period_size);
    snd_iprintf(buffer, b"    bytes_per_sec:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).pcm_bps);
    snd_iprintf(buffer, b"    sample_align:\t%u\n\0".as_ptr() as *const c_char, (*dpcm).pcm_salign);
    snd_iprintf(buffer, b"    rate_shift:\t\t%u\n\0".as_ptr() as *const c_char, (*dpcm).pcm_rate_shift);
    if let Some(dpcm_info) = (*(*(*dpcm).cable).ops).dpcm_info {
        dpcm_info(dpcm, buffer);
    }
}

unsafe fn print_substream_info(buffer: *mut snd_info_buffer, loopback: *mut loopback, sub: c_int, num: c_int) {
    let cable = (*loopback).cables[sub as usize][num as usize];

    snd_iprintf(buffer, b"Cable %i substream %i:\n\0".as_ptr() as *const c_char, num, sub);
    if cable.is_null() {
        snd_iprintf(buffer, b"  inactive\n\0".as_ptr() as *const c_char);
        return;
    }
    snd_iprintf(buffer, b"  valid: %u\n\0".as_ptr() as *const c_char, (*cable).valid);
    snd_iprintf(buffer, b"  running: %u\n\0".as_ptr() as *const c_char, (*cable).running);
    snd_iprintf(buffer, b"  pause: %u\n\0".as_ptr() as *const c_char, (*cable).pause);
    print_dpcm_info(buffer, (*cable).streams[0], b"Playback\0".as_ptr() as *const c_char);
    print_dpcm_info(buffer, (*cable).streams[1], b"Capture\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn print_cable_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let loopback = (*entry).private_data as *mut loopback;
    let mut sub: c_int;
    let mut num: c_int;

    num = *(*entry).name.add(strlen((*entry).name) - 1) as c_int;
    num = if num == '0' as c_int { 0 } else { 1 };
    sub = 0;
    while sub < MAX_PCM_SUBSTREAMS as c_int {
        print_substream_info(buffer, loopback, sub, num);
        sub += 1;
    }
}

unsafe fn loopback_cable_proc_new(loopback: *mut loopback, cidx: c_int) -> c_int {
    let mut name = [0 as c_char; 32];

    snprintf(name.as_mut_ptr(), size_of::<[c_char; 32]>(), b"cable#%d\0".as_ptr() as *const c_char, cidx);
    snd_card_ro_proc_new((*loopback).card, name.as_ptr(), loopback as *mut c_void, print_cable_info)
}

unsafe fn loopback_set_timer_source(loopback: *mut loopback, value: *const c_char) {
    if !(*loopback).timer_source.is_null() {
        devm_kfree((*(*loopback).card).dev, (*loopback).timer_source as *const c_void);
        (*loopback).timer_source = null();
    }
    if !value.is_null() && *value != 0 {
        (*loopback).timer_source = devm_kstrdup((*(*loopback).card).dev, value, GFP_KERNEL);
    }
}

unsafe extern "C" fn print_timer_source_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let loopback = (*entry).private_data as *mut loopback;
    snd_iprintf(
        buffer,
        b"%s\n\0".as_ptr() as *const c_char,
        if !(*loopback).timer_source.is_null() { (*loopback).timer_source } else { b"\0".as_ptr() as *const c_char },
    );
}

unsafe extern "C" fn change_timer_source_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let loopback = (*entry).private_data as *mut loopback;
    let mut line = [0 as c_char; 64];

    if snd_info_get_line(buffer, line.as_mut_ptr(), size_of::<[c_char; 64]>()) == 0 {
        loopback_set_timer_source(loopback, strim(line.as_mut_ptr()));
    }
}

unsafe fn loopback_timer_source_proc_new(loopback: *mut loopback) -> c_int {
    snd_card_rw_proc_new(
        (*loopback).card,
        b"timer_source\0".as_ptr() as *const c_char,
        loopback as *mut c_void,
        print_timer_source_info,
        change_timer_source_info,
    )
}

unsafe extern "C" fn loopback_probe(devptr: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = null_mut();
    let loopback: *mut loopback;
    let mut dev = (*devptr).id;
    let mut err: c_int;

    if dev < 0 || dev >= SNDRV_CARDS {
        dev_warn(&mut (*devptr).dev, b"Invalid card index %d, using default 0\n\0".as_ptr() as *const c_char, dev);
        dev = 0;
    }

    err = snd_devm_card_new(&mut (*devptr).dev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<loopback>(), &mut card);
    if err < 0 {
        return err;
    }
    loopback = (*card).private_data as *mut loopback;

    if pcm_substreams[dev as usize] < 1 {
        pcm_substreams[dev as usize] = 1;
    }
    if pcm_substreams[dev as usize] > MAX_PCM_SUBSTREAMS as c_int {
        pcm_substreams[dev as usize] = MAX_PCM_SUBSTREAMS as c_int;
    }

    (*loopback).card = card;
    loopback_set_timer_source(loopback, timer_source[dev as usize]);

    mutex_init(&mut (*loopback).cable_lock);

    err = loopback_pcm_new(loopback, 0, pcm_substreams[dev as usize]);
    if err < 0 {
        return err;
    }
    err = loopback_pcm_new(loopback, 1, pcm_substreams[dev as usize]);
    if err < 0 {
        return err;
    }
    err = loopback_mixer_new(loopback, if pcm_notify[dev as usize] != 0 { 1 } else { 0 });
    if err < 0 {
        return err;
    }
    loopback_cable_proc_new(loopback, 0);
    loopback_cable_proc_new(loopback, 1);
    loopback_timer_source_proc_new(loopback);
    strscpy((*card).driver.as_mut_ptr(), b"Loopback\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"Loopback\0".as_ptr() as *const c_char);
    sprintf((*card).longname.as_mut_ptr(), b"Loopback %i\0".as_ptr() as *const c_char, dev + 1);
    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    platform_set_drvdata(devptr, card as *mut c_void);
    0
}

unsafe extern "C" fn loopback_suspend(pdev: *mut device) -> c_int {
    let card = dev_get_drvdata(pdev) as *mut snd_card;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    0
}

unsafe extern "C" fn loopback_resume(pdev: *mut device) -> c_int {
    let card = dev_get_drvdata(pdev) as *mut snd_card;
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static loopback_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(loopback_suspend),
    resume: Some(loopback_resume),
};

static mut loopback_driver: platform_driver = platform_driver {
    probe: Some(loopback_probe),
    driver: platform_driver_driver {
        name: SND_LOOPBACK_DRIVER,
        pm: &loopback_pm,
    },
};

unsafe fn loopback_unregister_all() {
    let mut i = 0;
    while i < devices.len() {
        platform_device_unregister(devices[i]);
        i += 1;
    }
    platform_driver_unregister(&mut loopback_driver);
}

unsafe extern "C" fn alsa_card_loopback_init() -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let mut cards: c_int;

    err = platform_driver_register(&mut loopback_driver);
    if err < 0 {
        return err;
    }

    cards = 0;
    i = 0;
    while i < SNDRV_CARDS {
        let device: *mut platform_device;
        if !enable[i as usize] {
            i += 1;
            continue;
        }
        device = platform_device_register_simple(SND_LOOPBACK_DRIVER, i, null_mut(), 0);
        if device as isize as isize < 0 {
            i += 1;
            continue;
        }
        if platform_get_drvdata(device).is_null() {
            platform_device_unregister(device);
            i += 1;
            continue;
        }
        devices[i as usize] = device;
        cards += 1;
        i += 1;
    }
    if cards == 0 {
        /* MODULE */
        pr_err(b"aloop: No loopback enabled\n\0".as_ptr() as *const c_char);
        loopback_unregister_all();
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn alsa_card_loopback_exit() {
    loopback_unregister_all();
}

/* module_init(alsa_card_loopback_init) */
/* module_exit(alsa_card_loopback_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
