// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Digital Audio (PCM) abstract layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/*
 * Dependencies from the original C file:
 * <linux/time.h>, <linux/gcd.h>, <sound/core.h>, <sound/pcm.h>,
 * <sound/timer.h>, and "pcm_local.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub device: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
    pub period_size: c_ulong,
    pub timer_resolution: c_long_compat,
}

type c_long_compat = isize;

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
    pub number: c_int,
    pub stream: c_int,
    pub timer_running: c_int,
    pub timer: *mut snd_timer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_id {
    pub dev_sclass: c_int,
    pub dev_class: c_int,
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_hardware {
    pub flags: c_uint,
    pub resolution: c_ulong,
    pub ticks: c_ulong,
    pub c_resolution: Option<unsafe extern "C" fn(*mut snd_timer) -> c_ulong>,
    pub start: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
}

#[repr(C)]
pub struct snd_timer {
    pub private_data: *mut c_void,
    pub name: [c_char; 64],
    pub hw: snd_timer_hardware,
    pub card: *mut snd_card,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_timer)>,
}

pub const SNDRV_TIMER_HW_AUTO: c_uint = 1 << 0;
pub const SNDRV_TIMER_HW_SLAVE: c_uint = 1 << 1;
pub const SNDRV_TIMER_SCLASS_NONE: c_int = 0;
pub const SNDRV_TIMER_CLASS_PCM: c_int = 4;

unsafe extern "C" {
    fn gcd(a: c_ulong, b: c_ulong) -> c_ulong;
    fn snd_BUG_ON(condition: bool) -> bool;
    fn pcm_err(pcm: *mut snd_pcm, fmt: *const c_char, ...);
    fn snd_timer_chip(timer: *mut snd_timer) -> *mut snd_pcm_substream;
    fn snd_timer_new(
        card: *mut snd_card,
        id: *const c_char,
        tid: *mut snd_timer_id,
        rtimer: *mut *mut snd_timer,
    ) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_pcm_direction_name(stream: c_int) -> *const c_char;
    fn snd_device_register(card: *mut snd_card, device_data: *mut snd_timer) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut snd_timer) -> c_int;
}

/*
 *  Timer functions
 */

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_timer_resolution_change(substream: *mut snd_pcm_substream) {
    let mut rate: c_ulong;
    let mut mult: c_ulong;
    let mut fsize: c_ulong;
    let mut l: c_ulong;
    let mut post: c_ulong;
    let runtime: *mut snd_pcm_runtime = unsafe { (*substream).runtime };

    mult = 1000000000;
    rate = unsafe { (*runtime).rate as c_ulong };
    if unsafe { snd_BUG_ON(rate == 0) } {
        return;
    }
    l = unsafe { gcd(mult, rate) };
    mult /= l;
    rate /= l;
    fsize = unsafe { (*runtime).period_size };
    if unsafe { snd_BUG_ON(fsize == 0) } {
        return;
    }
    l = unsafe { gcd(rate, fsize) };
    rate /= l;
    fsize /= l;
    post = 1;
    while mult.wrapping_mul(fsize) / fsize != mult {
        mult /= 2;
        post = post.wrapping_mul(2);
    }
    if rate == 0 {
        unsafe {
            pcm_err(
                (*substream).pcm,
                c"pcm timer resolution out of range (rate = %u, period_size = %lu)\n".as_ptr(),
                (*runtime).rate,
                (*runtime).period_size,
            );
            (*runtime).timer_resolution = -1;
        }
        return;
    }
    unsafe {
        (*runtime).timer_resolution =
            (mult.wrapping_mul(fsize) / rate).wrapping_mul(post) as c_long_compat;
    }
}

unsafe extern "C" fn snd_pcm_timer_resolution(timer: *mut snd_timer) -> c_ulong {
    let substream: *mut snd_pcm_substream;

    substream = unsafe { (*timer).private_data as *mut snd_pcm_substream };
    unsafe {
        if !(*substream).runtime.is_null() {
            (*(*substream).runtime).timer_resolution as c_ulong
        } else {
            0
        }
    }
}

unsafe extern "C" fn snd_pcm_timer_start(timer: *mut snd_timer) -> c_int {
    let substream: *mut snd_pcm_substream;

    substream = unsafe { snd_timer_chip(timer) };
    unsafe {
        (*substream).timer_running = 1;
    }
    0
}

unsafe extern "C" fn snd_pcm_timer_stop(timer: *mut snd_timer) -> c_int {
    let substream: *mut snd_pcm_substream;

    substream = unsafe { snd_timer_chip(timer) };
    unsafe {
        (*substream).timer_running = 0;
    }
    0
}

static snd_pcm_timer: snd_timer_hardware = snd_timer_hardware {
    flags: SNDRV_TIMER_HW_AUTO | SNDRV_TIMER_HW_SLAVE,
    resolution: 0,
    ticks: 1,
    c_resolution: Some(snd_pcm_timer_resolution),
    start: Some(snd_pcm_timer_start),
    stop: Some(snd_pcm_timer_stop),
};

/*
 *  Init functions
 */

unsafe extern "C" fn snd_pcm_timer_free(timer: *mut snd_timer) {
    let substream: *mut snd_pcm_substream =
        unsafe { (*timer).private_data as *mut snd_pcm_substream };
    unsafe {
        (*substream).timer = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_timer_init(substream: *mut snd_pcm_substream) {
    let mut tid: snd_timer_id = unsafe { core::mem::zeroed() };
    let mut timer: *mut snd_timer = core::ptr::null_mut();

    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.dev_class = SNDRV_TIMER_CLASS_PCM;
    unsafe {
        tid.card = (*(*(*substream).pcm).card).number;
        tid.device = (*(*substream).pcm).device;
        tid.subdevice = ((*substream).number << 1) | ((*substream).stream & 1);
        if snd_timer_new((*(*substream).pcm).card, c"PCM".as_ptr(), &mut tid, &mut timer) < 0 {
            return;
        }
        sprintf(
            (*timer).name.as_mut_ptr(),
            c"PCM %s %i-%i-%i".as_ptr(),
            snd_pcm_direction_name((*substream).stream),
            tid.card,
            tid.device,
            tid.subdevice,
        );
        (*timer).hw = snd_pcm_timer;
        if snd_device_register((*timer).card, timer) < 0 {
            snd_device_free((*timer).card, timer);
            return;
        }
        (*timer).private_data = substream as *mut c_void;
        (*timer).private_free = Some(snd_pcm_timer_free);
        (*substream).timer = timer;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_timer_done(substream: *mut snd_pcm_substream) {
    unsafe {
        if !(*substream).timer.is_null() {
            snd_device_free((*(*substream).pcm).card, (*substream).timer);
            (*substream).timer = core::ptr::null_mut();
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
