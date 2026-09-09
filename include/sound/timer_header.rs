/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Timer abstract layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 *             Abramo Bagnara <abramo@alsa-project.org>
 */

// Dependencies supplied by other headers: snd_card, module, snd_timer_id,
// timespec64, spinlock_t, list_head, work_struct, and kref.

pub const SNDRV_TIMER_DEVICES: u32 = 16;
pub const SNDRV_TIMER_DEV_FLG_PCM: u32 = 0x10000000;

pub const SNDRV_TIMER_HW_AUTO: u32 = 0x00000001; // auto trigger is supported
pub const SNDRV_TIMER_HW_STOP: u32 = 0x00000002; // call stop before start
pub const SNDRV_TIMER_HW_SLAVE: u32 = 0x00000004; // only slave timer (variable resolution)
pub const SNDRV_TIMER_HW_FIRST: u32 = 0x00000008; // first tick can be incomplete
pub const SNDRV_TIMER_HW_WORK: u32 = 0x00000010; // timer is called from work

pub const SNDRV_TIMER_IFLG_SLAVE: u32 = 0x00000001;
pub const SNDRV_TIMER_IFLG_RUNNING: u32 = 0x00000002;
pub const SNDRV_TIMER_IFLG_START: u32 = 0x00000004;
pub const SNDRV_TIMER_IFLG_AUTO: u32 = 0x00000008; // auto restart
pub const SNDRV_TIMER_IFLG_FAST: u32 = 0x00000010; // fast callback (do not use work)
pub const SNDRV_TIMER_IFLG_CALLBACK: u32 = 0x00000020; // timer callback is active
pub const SNDRV_TIMER_IFLG_EXCLUSIVE: u32 = 0x00000040; // exclusive owner - no more instances
pub const SNDRV_TIMER_IFLG_EARLY_EVENT: u32 = 0x00000080; // write early event to the poll queue

pub const SNDRV_TIMER_FLG_CHANGE: u32 = 0x00000001;
pub const SNDRV_TIMER_FLG_RESCHED: u32 = 0x00000002; // need reschedule

#[repr(C)]
pub struct snd_timer_hardware {
    pub flags: u32,
    pub resolution: ::core::ffi::c_ulong,
    pub resolution_min: ::core::ffi::c_ulong,
    pub resolution_max: ::core::ffi::c_ulong,
    pub ticks: ::core::ffi::c_ulong,
    pub open: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> i32>,
    pub close: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> i32>,
    pub c_resolution: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> ::core::ffi::c_ulong>,
    pub start: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> i32>,
    pub stop: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> i32>,
    pub set_period: Option<unsafe extern "C" fn(timer: *mut snd_timer, period_num: ::core::ffi::c_ulong, period_den: ::core::ffi::c_ulong) -> i32>,
    pub precise_resolution: Option<unsafe extern "C" fn(timer: *mut snd_timer, num: *mut ::core::ffi::c_ulong, den: *mut ::core::ffi::c_ulong) -> i32>,
}

#[repr(C)]
pub struct snd_timer {
    pub tmr_class: i32,
    pub card: *mut snd_card,
    pub module: *mut module,
    pub tmr_device: i32,
    pub tmr_subdevice: i32,
    pub id: [::core::ffi::c_char; 64],
    pub name: [::core::ffi::c_char; 80],
    pub flags: u32,
    pub running: i32,
    pub sticks: ::core::ffi::c_ulong,
    pub private_data: *mut ::core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(timer: *mut snd_timer)>,
    pub hw: snd_timer_hardware,
    pub lock: spinlock_t,
    pub device_list: list_head,
    pub open_list_head: list_head,
    pub active_list_head: list_head,
    pub ack_list_head: list_head,
    pub sack_list_head: list_head,
    pub task_work: work_struct,
    pub kref: kref,
    pub max_instances: i32,
    pub num_instances: i32,
}

#[repr(C)]
pub struct snd_timer_instance {
    pub timer: *mut snd_timer,
    pub owner: *mut ::core::ffi::c_char,
    pub flags: u32,
    pub private_data: *mut ::core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(ti: *mut snd_timer_instance)>,
    pub callback: Option<unsafe extern "C" fn(timeri: *mut snd_timer_instance, ticks: ::core::ffi::c_ulong, resolution: ::core::ffi::c_ulong)>,
    pub ccallback: Option<unsafe extern "C" fn(timeri: *mut snd_timer_instance, event: i32, tstamp: *mut timespec64, resolution: ::core::ffi::c_ulong)>,
    pub disconnect: Option<unsafe extern "C" fn(timeri: *mut snd_timer_instance)>,
    pub callback_data: *mut ::core::ffi::c_void,
    pub ticks: ::core::ffi::c_ulong,
    pub cticks: ::core::ffi::c_ulong,
    pub pticks: ::core::ffi::c_ulong,
    pub resolution: ::core::ffi::c_ulong,
    pub lost: ::core::ffi::c_ulong,
    pub slave_class: i32,
    pub slave_id: u32,
    pub open_list: list_head,
    pub active_list: list_head,
    pub master_list: list_head,
    pub ack_list: list_head,
    pub slave_list_head: list_head,
    pub slave_active_head: list_head,
    pub master: *mut snd_timer_instance,
}

unsafe extern "C" {
    pub fn snd_timer_new(card: *mut snd_card, id: *mut ::core::ffi::c_char, tid: *mut snd_timer_id, rtimer: *mut *mut snd_timer) -> i32;
    pub fn snd_timer_notify(timer: *mut snd_timer, event: i32, tstamp: *mut timespec64);
    pub fn snd_timer_global_new(id: *mut ::core::ffi::c_char, device: i32, rtimer: *mut *mut snd_timer) -> i32;
    pub fn snd_timer_global_free(timer: *mut snd_timer) -> i32;
    pub fn snd_timer_global_register(timer: *mut snd_timer) -> i32;
    pub fn snd_timer_instance_new(owner: *const ::core::ffi::c_char) -> *mut snd_timer_instance;
    pub fn snd_timer_instance_free(timeri: *mut snd_timer_instance);
    pub fn snd_timer_open(timeri: *mut snd_timer_instance, tid: *mut snd_timer_id, slave_id: u32) -> i32;
    pub fn snd_timer_close(timeri: *mut snd_timer_instance);
    pub fn snd_timer_resolution(timeri: *mut snd_timer_instance) -> ::core::ffi::c_ulong;
    pub fn snd_timer_start(timeri: *mut snd_timer_instance, ticks: u32) -> i32;
    pub fn snd_timer_stop(timeri: *mut snd_timer_instance) -> i32;
    pub fn snd_timer_continue(timeri: *mut snd_timer_instance) -> i32;
    pub fn snd_timer_pause(timeri: *mut snd_timer_instance) -> i32;
    pub fn snd_timer_interrupt(timer: *mut snd_timer, ticks_left: ::core::ffi::c_ulong);
    pub fn snd_timeri_timer_get(timeri: *mut snd_timer_instance) -> *mut snd_timer;
    pub fn snd_timeri_timer_put(timer: *mut snd_timer);
}

// DEFINE_FREE(snd_timeri_timer, struct snd_timer *, if (_T) snd_timeri_timer_put(_T))

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
