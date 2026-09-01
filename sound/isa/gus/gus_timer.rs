// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for Gravis UltraSound soundcards - Timers
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 *  GUS have similar timers as AdLib (OPL2/OPL3 chips).
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies supplied by the original C includes:
// <linux/time.h>, <sound/core.h>, <sound/gus.h>

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
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
    pub resolution: c_uint,
    pub ticks: c_uint,
    pub start: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
}

#[repr(C)]
pub struct snd_timer {
    pub name: *mut c_char,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_timer)>,
    pub hw: snd_timer_hardware,
    pub sticks: c_uint,
}

#[repr(C)]
pub struct snd_gf1 {
    pub timer_enabled: u8,
    pub timer1: *mut snd_timer,
    pub timer2: *mut snd_timer,
    pub interrupt_handler_timer1: Option<unsafe extern "C" fn(*mut snd_gus_card)>,
    pub interrupt_handler_timer2: Option<unsafe extern "C" fn(*mut snd_gus_card)>,
}

#[repr(C)]
pub struct snd_gus_card {
    pub reg_lock: c_void,
    pub gf1: snd_gf1,
    pub card: *mut snd_card,
    pub timer_dev: c_int,
}

unsafe extern "C" {
    static SNDRV_TIMER_HW_STOP: c_uint;
    static SNDRV_TIMER_CLASS_CARD: c_int;
    static SNDRV_TIMER_SCLASS_NONE: c_int;
    static SNDRV_GF1_GB_ADLIB_TIMER_1: c_int;
    static SNDRV_GF1_GB_SOUND_BLASTER_CONTROL: c_int;
    static SNDRV_GF1_GB_ADLIB_TIMER_2: c_int;
    static SNDRV_GF1_HANDLER_TIMER1: c_int;
    static SNDRV_GF1_HANDLER_TIMER2: c_int;

    fn snd_timer_chip(timer: *mut snd_timer) -> *mut snd_gus_card;
    fn snd_gf1_write8(gus: *mut snd_gus_card, reg: c_int, data: c_int);
    fn snd_gf1_adlib_write(gus: *mut snd_gus_card, reg: c_int, data: c_int);
    fn snd_timer_interrupt(timer: *mut snd_timer, ticks: c_uint);
    fn snd_timer_new(
        card: *mut snd_card,
        id: *const c_char,
        tid: *mut snd_timer_id,
        rtimer: *mut *mut snd_timer,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn snd_gf1_set_default_handlers(gus: *mut snd_gus_card, what: c_int);
    fn spin_lock_irqsave(lock: *mut c_void) -> c_uint;
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_uint);
}

/*
 *  Timer 1 - 80us
 */

unsafe extern "C" fn snd_gf1_timer1_start(timer: *mut snd_timer) -> c_int {
    let mut tmp: u8;
    let ticks: c_uint;
    let gus: *mut snd_gus_card;

    gus = unsafe { snd_timer_chip(timer) };
    let flags = unsafe { spin_lock_irqsave(&mut (*gus).reg_lock) };
    ticks = unsafe { (*timer).sticks };
    unsafe {
        (*gus).gf1.timer_enabled |= 4;
        tmp = (*gus).gf1.timer_enabled;
        snd_gf1_write8(
            gus,
            SNDRV_GF1_GB_ADLIB_TIMER_1,
            (256u32.wrapping_sub(ticks)) as c_int,
        ); /* timer 1 count */
        snd_gf1_write8(gus, SNDRV_GF1_GB_SOUND_BLASTER_CONTROL, tmp as c_int); /* enable timer 1 IRQ */
        snd_gf1_adlib_write(gus, 0x04, (tmp >> 2) as c_int); /* timer 2 start */
        spin_unlock_irqrestore(&mut (*gus).reg_lock, flags);
    }
    0
}

unsafe extern "C" fn snd_gf1_timer1_stop(timer: *mut snd_timer) -> c_int {
    let tmp: u8;
    let gus: *mut snd_gus_card;

    gus = unsafe { snd_timer_chip(timer) };
    let flags = unsafe { spin_lock_irqsave(&mut (*gus).reg_lock) };
    unsafe {
        (*gus).gf1.timer_enabled &= !4u8;
        tmp = (*gus).gf1.timer_enabled;
        snd_gf1_write8(gus, SNDRV_GF1_GB_SOUND_BLASTER_CONTROL, tmp as c_int); /* disable timer #1 */
        spin_unlock_irqrestore(&mut (*gus).reg_lock, flags);
    }
    0
}

/*
 *  Timer 2 - 320us
 */

unsafe extern "C" fn snd_gf1_timer2_start(timer: *mut snd_timer) -> c_int {
    let mut tmp: u8;
    let ticks: c_uint;
    let gus: *mut snd_gus_card;

    gus = unsafe { snd_timer_chip(timer) };
    let flags = unsafe { spin_lock_irqsave(&mut (*gus).reg_lock) };
    ticks = unsafe { (*timer).sticks };
    unsafe {
        (*gus).gf1.timer_enabled |= 8;
        tmp = (*gus).gf1.timer_enabled;
        snd_gf1_write8(
            gus,
            SNDRV_GF1_GB_ADLIB_TIMER_2,
            (256u32.wrapping_sub(ticks)) as c_int,
        ); /* timer 2 count */
        snd_gf1_write8(gus, SNDRV_GF1_GB_SOUND_BLASTER_CONTROL, tmp as c_int); /* enable timer 2 IRQ */
        snd_gf1_adlib_write(gus, 0x04, (tmp >> 2) as c_int); /* timer 2 start */
        spin_unlock_irqrestore(&mut (*gus).reg_lock, flags);
    }
    0
}

unsafe extern "C" fn snd_gf1_timer2_stop(timer: *mut snd_timer) -> c_int {
    let tmp: u8;
    let gus: *mut snd_gus_card;

    gus = unsafe { snd_timer_chip(timer) };
    let flags = unsafe { spin_lock_irqsave(&mut (*gus).reg_lock) };
    unsafe {
        (*gus).gf1.timer_enabled &= !8u8;
        tmp = (*gus).gf1.timer_enabled;
        snd_gf1_write8(gus, SNDRV_GF1_GB_SOUND_BLASTER_CONTROL, tmp as c_int); /* disable timer #1 */
        spin_unlock_irqrestore(&mut (*gus).reg_lock, flags);
    }
    0
}

/*

 */

unsafe extern "C" fn snd_gf1_interrupt_timer1(gus: *mut snd_gus_card) {
    let timer: *mut snd_timer = unsafe { (*gus).gf1.timer1 };

    if timer.is_null() {
        return;
    }
    unsafe {
        snd_timer_interrupt(timer, (*timer).sticks);
    }
}

unsafe extern "C" fn snd_gf1_interrupt_timer2(gus: *mut snd_gus_card) {
    let timer: *mut snd_timer = unsafe { (*gus).gf1.timer2 };

    if timer.is_null() {
        return;
    }
    unsafe {
        snd_timer_interrupt(timer, (*timer).sticks);
    }
}

/*

 */

static snd_gf1_timer1: snd_timer_hardware = snd_timer_hardware {
    flags: unsafe { SNDRV_TIMER_HW_STOP },
    resolution: 80000,
    ticks: 256,
    start: Some(snd_gf1_timer1_start),
    stop: Some(snd_gf1_timer1_stop),
};

static snd_gf1_timer2: snd_timer_hardware = snd_timer_hardware {
    flags: unsafe { SNDRV_TIMER_HW_STOP },
    resolution: 320000,
    ticks: 256,
    start: Some(snd_gf1_timer2_start),
    stop: Some(snd_gf1_timer2_stop),
};

unsafe extern "C" fn snd_gf1_timer1_free(timer: *mut snd_timer) {
    let gus: *mut snd_gus_card = unsafe { (*timer).private_data as *mut snd_gus_card };
    unsafe {
        (*gus).gf1.timer1 = core::ptr::null_mut();
    }
}

unsafe extern "C" fn snd_gf1_timer2_free(timer: *mut snd_timer) {
    let gus: *mut snd_gus_card = unsafe { (*timer).private_data as *mut snd_gus_card };
    unsafe {
        (*gus).gf1.timer2 = core::ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_timers_init(gus: *mut snd_gus_card) {
    let mut timer: *mut snd_timer = core::ptr::null_mut();
    let mut tid: snd_timer_id = unsafe { core::mem::zeroed() };

    if unsafe { !(*gus).gf1.timer1.is_null() || !(*gus).gf1.timer2.is_null() } {
        return;
    }

    unsafe {
        (*gus).gf1.interrupt_handler_timer1 = Some(snd_gf1_interrupt_timer1);
        (*gus).gf1.interrupt_handler_timer2 = Some(snd_gf1_interrupt_timer2);

        tid.dev_class = SNDRV_TIMER_CLASS_CARD;
        tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
        tid.card = (*(*gus).card).number;
        tid.device = (*gus).timer_dev;
        tid.subdevice = 0;

        if snd_timer_new(
            (*gus).card,
            b"GF1 timer\0".as_ptr() as *const c_char,
            &mut tid,
            &mut timer,
        ) >= 0
        {
            strscpy((*timer).name, b"GF1 timer #1\0".as_ptr() as *const c_char);
            (*timer).private_data = gus as *mut c_void;
            (*timer).private_free = Some(snd_gf1_timer1_free);
            (*timer).hw = snd_gf1_timer1;
        }
        (*gus).gf1.timer1 = timer;

        tid.device += 1;

        if snd_timer_new(
            (*gus).card,
            b"GF1 timer\0".as_ptr() as *const c_char,
            &mut tid,
            &mut timer,
        ) >= 0
        {
            strscpy((*timer).name, b"GF1 timer #2\0".as_ptr() as *const c_char);
            (*timer).private_data = gus as *mut c_void;
            (*timer).private_free = Some(snd_gf1_timer2_free);
            (*timer).hw = snd_gf1_timer2;
        }
        (*gus).gf1.timer2 = timer;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_timers_done(gus: *mut snd_gus_card) {
    unsafe {
        snd_gf1_set_default_handlers(
            gus,
            SNDRV_GF1_HANDLER_TIMER1 | SNDRV_GF1_HANDLER_TIMER2,
        );
        if !(*gus).gf1.timer1.is_null() {
            snd_device_free((*gus).card, (*gus).gf1.timer1 as *mut c_void);
            (*gus).gf1.timer1 = core::ptr::null_mut();
        }
        if !(*gus).gf1.timer2.is_null() {
            snd_device_free((*gus).card, (*gus).gf1.timer2 as *mut c_void);
            (*gus).gf1.timer2 = core::ptr::null_mut();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_timers_resume(gus: *mut snd_gus_card) {
    unsafe {
        if !(*gus).gf1.timer1.is_null() {
            (*gus).gf1.interrupt_handler_timer1 = Some(snd_gf1_interrupt_timer1);
            if (*gus).gf1.timer_enabled & 4 != 0 {
                snd_gf1_timer1_start((*gus).gf1.timer1);
            }
        }
        if !(*gus).gf1.timer2.is_null() {
            (*gus).gf1.interrupt_handler_timer2 = Some(snd_gf1_interrupt_timer2);
            if (*gus).gf1.timer_enabled & 8 != 0 {
                snd_gf1_timer2_start((*gus).gf1.timer2);
            }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
