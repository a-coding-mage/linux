// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Lee Revell <rlrevell@joe-job.com>
 *                   Clemens Ladisch <clemens@ladisch.de>
 *                   Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
 *
 *  Routines for control of EMU10K1 chips
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn snd_timer_chip(timer: *mut snd_timer) -> *mut c_void;
    fn snd_emu10k1_intr_enable(emu: *mut snd_emu10k1, what: c_uint);
    fn snd_emu10k1_intr_disable(emu: *mut snd_emu10k1, what: c_uint);
    fn outw(value: c_uint, port: c_ulong);
    fn snd_timer_new(
        card: *mut snd_card,
        id: *const c_char,
        tid: *mut snd_timer_id,
        rtimer: *mut *mut snd_timer,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
}

#[repr(C)]
pub struct snd_emu10k1_card_capabilities {
    pub emu_model: c_int,
}

#[repr(C)]
pub struct snd_emu1010 {
    pub word_clock: c_ulong,
}

#[repr(C)]
pub struct snd_emu10k1 {
    pub port: c_ulong,
    pub card_capabilities: *mut snd_emu10k1_card_capabilities,
    pub emu1010: snd_emu1010,
    pub card: *mut snd_card,
    pub timer: *mut snd_timer,
}

#[repr(C)]
pub struct snd_timer {
    pub sticks: c_uint,
    pub name: *mut c_char,
    pub private_data: *mut c_void,
    pub hw: snd_timer_hardware,
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
#[derive(Copy, Clone)]
pub struct snd_timer_hardware {
    pub flags: c_uint,
    pub ticks: c_uint,
    pub start: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> c_int>,
    pub c_resolution: Option<unsafe extern "C" fn(timer: *mut snd_timer) -> c_ulong>,
    pub precise_resolution: Option<
        unsafe extern "C" fn(
            timer: *mut snd_timer,
            num: *mut c_ulong,
            den: *mut c_ulong,
        ) -> c_int,
    >,
}

unsafe extern "C" fn snd_emu10k1_timer_start(timer: *mut snd_timer) -> c_int {
    let emu: *mut snd_emu10k1;
    let mut delay: c_uint;

    emu = snd_timer_chip(timer) as *mut snd_emu10k1;
    delay = (*timer).sticks.wrapping_sub(1);
    if delay < 5 {
        /* minimum time is 5 ticks */
        delay = 5;
    }
    snd_emu10k1_intr_enable(emu, INTE_INTERVALTIMERENB);
    outw(delay & TIMER_RATE_MASK, (*emu).port + TIMER);
    0
}

unsafe extern "C" fn snd_emu10k1_timer_stop(timer: *mut snd_timer) -> c_int {
    let emu: *mut snd_emu10k1;

    emu = snd_timer_chip(timer) as *mut snd_emu10k1;
    snd_emu10k1_intr_disable(emu, INTE_INTERVALTIMERENB);
    0
}

unsafe extern "C" fn snd_emu10k1_timer_c_resolution(timer: *mut snd_timer) -> c_ulong {
    let emu: *mut snd_emu10k1 = snd_timer_chip(timer) as *mut snd_emu10k1;

    if (*(*emu).card_capabilities).emu_model != 0 && (*emu).emu1010.word_clock == 44100 {
        22676 // 1 sample @ 44.1 kHz = 22.675736...us
    } else {
        20833 // 1 sample @ 48 kHz = 20.833...us
    }
}

unsafe extern "C" fn snd_emu10k1_timer_precise_resolution(
    timer: *mut snd_timer,
    num: *mut c_ulong,
    den: *mut c_ulong,
) -> c_int {
    let emu: *mut snd_emu10k1 = snd_timer_chip(timer) as *mut snd_emu10k1;

    *num = 1;
    if (*(*emu).card_capabilities).emu_model != 0 {
        *den = (*emu).emu1010.word_clock;
    } else {
        *den = 48000;
    }
    0
}

static snd_emu10k1_timer_hw: snd_timer_hardware = snd_timer_hardware {
    flags: SNDRV_TIMER_HW_AUTO,
    ticks: 1024,
    start: Some(snd_emu10k1_timer_start),
    stop: Some(snd_emu10k1_timer_stop),
    c_resolution: Some(snd_emu10k1_timer_c_resolution),
    precise_resolution: Some(snd_emu10k1_timer_precise_resolution),
};

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_timer(emu: *mut snd_emu10k1, device: c_int) -> c_int {
    let mut timer: *mut snd_timer = core::ptr::null_mut();
    let mut tid: snd_timer_id = core::mem::zeroed();
    let err: c_int;

    tid.dev_class = SNDRV_TIMER_CLASS_CARD;
    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.card = (*(*emu).card).number;
    tid.device = device;
    tid.subdevice = 0;
    err = snd_timer_new(
        (*emu).card,
        b"EMU10K1\0".as_ptr() as *const c_char,
        &mut tid,
        &mut timer,
    );
    if err >= 0 {
        strscpy((*timer).name, b"EMU10K1 timer\0".as_ptr() as *const c_char);
        (*timer).private_data = emu as *mut c_void;
        (*timer).hw = snd_emu10k1_timer_hw;
    }
    (*emu).timer = timer;
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
