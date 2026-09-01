// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA timer back-end using hrtimer
 * Copyright (C) 2008 Takashi Iwai
 */

// C dependencies: linux/init.h, linux/slab.h, linux/string.h,
// linux/module.h, linux/moduleparam.h, linux/hrtimer.h,
// sound/core.h, sound/timer.h

// MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
// MODULE_DESCRIPTION("ALSA hrtimer backend");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("snd-timer-" __stringify(SNDRV_TIMER_GLOBAL_HRTIMER));

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const NANO_SEC: c_ulong = 1000000000u64 as c_ulong; /* 10^9 in sec */
static mut resolution: c_uint = 0;

type bool_ = bool;
type ktime_t = i64;
type gfp_t = c_uint;

const ENOMEM: c_int = 12;
const CLOCK_MONOTONIC: c_int = 1;
const HRTIMER_MODE_REL: hrtimer_mode = 0;
const HRTIMER_NORESTART: hrtimer_restart = 0;
const HRTIMER_RESTART: hrtimer_restart = 1;
const GFP_KERNEL: gfp_t = 0;

type hrtimer_restart = c_int;
type hrtimer_mode = c_int;

extern "C" {
    static mut hrtimer_resolution: c_uint;
    static mut THIS_MODULE: *mut c_void;

    fn hrtimer_cb_get_time(timer: *mut hrtimer) -> ktime_t;
    fn hrtimer_get_expires(timer: *mut hrtimer) -> ktime_t;
    fn hrtimer_add_expires_ns(timer: *mut hrtimer, ns: u64);
    fn hrtimer_setup(
        timer: *mut hrtimer,
        function: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>,
        clock_id: c_int,
        mode: hrtimer_mode,
    );
    fn hrtimer_cancel(timer: *mut hrtimer) -> c_int;
    fn hrtimer_start(timer: *mut hrtimer, tim: ktime_t, mode: hrtimer_mode) -> c_int;
    fn hrtimer_try_to_cancel(timer: *mut hrtimer) -> c_int;
    fn ns_to_ktime(ns: u64) -> ktime_t;
    fn ktime_sub(lhs: ktime_t, rhs: ktime_t) -> ktime_t;
    fn ktime_divns(kt: ktime_t, div: u64) -> c_ulong;

    fn snd_timer_interrupt(timer: *mut snd_timer, ticks: c_ulong);
    fn snd_timer_global_new(
        id: *const c_char,
        device: c_int,
        rtimer: *mut *mut snd_timer,
    ) -> c_int;
    fn snd_timer_global_register(timer: *mut snd_timer) -> c_int;
    fn snd_timer_global_free(timer: *mut snd_timer);

    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
}

#[repr(C)]
pub struct hrtimer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_timer {
    pub lock: spinlock_t,
    pub running: c_int,
    pub sticks: c_ulong,
    pub private_data: *mut c_void,
    pub module: *mut c_void,
    pub name: [c_char; 80],
    pub hw: snd_timer_hardware,
    pub max_instances: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_hardware {
    pub flags: c_uint,
    pub open: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub start: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub resolution: c_uint,
    pub ticks: c_ulong,
}

const SNDRV_TIMER_GLOBAL_HRTIMER: c_int = 0;
const SNDRV_TIMER_HW_AUTO: c_uint = 0;
const SNDRV_TIMER_HW_WORK: c_uint = 0;

#[repr(C)]
struct snd_hrtimer {
    timer: *mut snd_timer,
    hrt: hrtimer,
    in_callback: bool_,
}

unsafe extern "C" fn snd_hrtimer_callback(hrt: *mut hrtimer) -> hrtimer_restart {
    let stime = (hrt as *mut u8).sub(offset_of!(snd_hrtimer, hrt)) as *mut snd_hrtimer;
    let t = (*stime).timer;
    let mut ticks: c_ulong;
    let mut ret: hrtimer_restart = HRTIMER_NORESTART;

    spin_lock(&mut (*t).lock);
    if (*t).running == 0 {
        spin_unlock(&mut (*t).lock);
        return HRTIMER_NORESTART; /* fast path */
    }
    (*stime).in_callback = true;
    ticks = (*t).sticks;
    spin_unlock(&mut (*t).lock);

    /* calculate the drift */
    let delta = ktime_sub(hrtimer_cb_get_time(hrt), hrtimer_get_expires(hrt));
    if delta > 0 {
        ticks = ticks.wrapping_add(ktime_divns(
            delta,
            ticks.wrapping_mul(resolution as c_ulong) as u64,
        ));
    }

    snd_timer_interrupt((*stime).timer, ticks);

    spin_lock(&mut (*t).lock);
    if (*t).running != 0 {
        hrtimer_add_expires_ns(
            hrt,
            (*t).sticks.wrapping_mul(resolution as c_ulong) as u64,
        );
        ret = HRTIMER_RESTART;
    }

    (*stime).in_callback = false;
    spin_unlock(&mut (*t).lock);
    ret
}

unsafe extern "C" fn snd_hrtimer_open(t: *mut snd_timer) -> c_int {
    let stime = kzalloc(size_of::<snd_hrtimer>(), GFP_KERNEL) as *mut snd_hrtimer;
    if stime.is_null() {
        return -ENOMEM;
    }
    (*stime).timer = t;
    hrtimer_setup(
        &mut (*stime).hrt,
        Some(snd_hrtimer_callback),
        CLOCK_MONOTONIC,
        HRTIMER_MODE_REL,
    );
    (*t).private_data = stime as *mut c_void;
    0
}

unsafe extern "C" fn snd_hrtimer_close(t: *mut snd_timer) -> c_int {
    let stime = (*t).private_data as *mut snd_hrtimer;

    if !stime.is_null() {
        spin_lock_irq(&mut (*t).lock);
        (*t).running = 0; /* just to be sure */
        (*stime).in_callback = true; /* skip start/stop */
        spin_unlock_irq(&mut (*t).lock);

        hrtimer_cancel(&mut (*stime).hrt);
        kfree(stime as *mut c_void);
        (*t).private_data = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn snd_hrtimer_start(t: *mut snd_timer) -> c_int {
    let stime = (*t).private_data as *mut snd_hrtimer;

    if (*stime).in_callback {
        return 0;
    }
    hrtimer_start(
        &mut (*stime).hrt,
        ns_to_ktime((*t).sticks.wrapping_mul(resolution as c_ulong) as u64),
        HRTIMER_MODE_REL,
    );
    0
}

unsafe extern "C" fn snd_hrtimer_stop(t: *mut snd_timer) -> c_int {
    let stime = (*t).private_data as *mut snd_hrtimer;

    if (*stime).in_callback {
        return 0;
    }
    hrtimer_try_to_cancel(&mut (*stime).hrt);
    0
}

// __initconst
static hrtimer_hw: snd_timer_hardware = snd_timer_hardware {
    flags: SNDRV_TIMER_HW_AUTO | SNDRV_TIMER_HW_WORK,
    open: Some(snd_hrtimer_open),
    close: Some(snd_hrtimer_close),
    start: Some(snd_hrtimer_start),
    stop: Some(snd_hrtimer_stop),
    resolution: 0,
    ticks: 0,
};

/*
 * entry functions
 */

static mut mytimer: *mut snd_timer = ptr::null_mut();

// __init
unsafe extern "C" fn snd_hrtimer_init() -> c_int {
    let mut timer: *mut snd_timer = ptr::null_mut();
    let mut err: c_int;

    resolution = hrtimer_resolution;

    /* Create a new timer and set up the fields */
    err = snd_timer_global_new(
        b"hrtimer\0".as_ptr() as *const c_char,
        SNDRV_TIMER_GLOBAL_HRTIMER,
        &mut timer,
    );
    if err < 0 {
        return err;
    }

    (*timer).module = THIS_MODULE;
    strscpy(
        (*timer).name.as_mut_ptr(),
        b"HR timer\0".as_ptr() as *const c_char,
    );
    (*timer).hw = hrtimer_hw;
    (*timer).hw.resolution = resolution;
    (*timer).hw.ticks = NANO_SEC / resolution as c_ulong;
    (*timer).max_instances = 100; /* lower the limit */

    err = snd_timer_global_register(timer);
    if err < 0 {
        snd_timer_global_free(timer);
        return err;
    }
    mytimer = timer; /* remember this */

    0
}

// __exit
unsafe extern "C" fn snd_hrtimer_exit() {
    if !mytimer.is_null() {
        snd_timer_global_free(mytimer);
        mytimer = ptr::null_mut();
    }
}

// module_init(snd_hrtimer_init);
// module_exit(snd_hrtimer_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
