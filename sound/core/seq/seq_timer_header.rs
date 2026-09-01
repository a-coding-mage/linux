/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  ALSA sequencer Timer
 *  Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/* C header dependencies: <sound/timer.h>, <sound/seq_kernel.h> */

#[repr(C)]
pub struct snd_seq_timer_tick {
    pub cur_tick: snd_seq_tick_time_t, /* current tick */
    pub resolution: c_ulong,           /* time per tick in nsec */
    pub fraction: c_ulong,             /* current time per tick in nsec */
}

#[repr(C)]
pub struct snd_seq_timer {
    /* ... tempo / offset / running state */

    /* C bitfields: unsigned int running:1, initialized:1 */
    pub running: c_uint,     /* running state of queue */
    pub initialized: c_uint, /* timer is initialized */

    pub tempo: c_uint, /* current tempo, us/tick */
    pub ppq: c_int,    /* time resolution, ticks/quarter */

    pub cur_time: snd_seq_real_time_t,     /* current time */
    pub tick: snd_seq_timer_tick,          /* current tick */
    pub tick_updated: c_int,

    pub type_: c_int,                      /* timer type */
    pub alsa_id: snd_timer_id,             /* ALSA's timer ID */
    pub timeri: *mut snd_timer_instance,   /* timer instance */
    pub ticks: c_uint,
    pub preferred_resolution: c_ulong,     /* timer resolution, ticks/sec */

    pub skew: c_uint,
    pub skew_base: c_uint,
    pub tempo_base: c_uint,

    pub last_update: timespec64, /* time of last clock update, used for interpolation */

    pub lock: spinlock_t,
}

unsafe extern "C" {
    /* create new timer (constructor) */
    pub fn snd_seq_timer_new() -> *mut snd_seq_timer;

    /* delete timer (destructor) */
    pub fn snd_seq_timer_delete(tmr: *mut *mut snd_seq_timer);
}

/* */
#[inline]
pub unsafe fn snd_seq_timer_update_tick(tick: *mut snd_seq_timer_tick, resolution: c_ulong) {
    if (*tick).resolution > 0 {
        (*tick).fraction = (*tick).fraction.wrapping_add(resolution);
        (*tick).cur_tick = (*tick)
            .cur_tick
            .wrapping_add(((*tick).fraction / (*tick).resolution) as c_uint);
        (*tick).fraction %= (*tick).resolution;
    }
}

/* compare timestamp between events */
/* return 1 if a >= b; otherwise return 0 */
#[inline]
pub unsafe fn snd_seq_compare_tick_time(
    a: *mut snd_seq_tick_time_t,
    b: *mut snd_seq_tick_time_t,
) -> c_int {
    /* compare ticks */
    (*a >= *b) as c_int
}

#[inline]
pub unsafe fn snd_seq_compare_real_time(
    a: *mut snd_seq_real_time_t,
    b: *mut snd_seq_real_time_t,
) -> c_int {
    /* compare real time */
    if (*a).tv_sec > (*b).tv_sec {
        return 1;
    }
    if ((*a).tv_sec == (*b).tv_sec) && ((*a).tv_nsec >= (*b).tv_nsec) {
        return 1;
    }
    0
}

#[inline]
pub unsafe fn snd_seq_sanity_real_time(tm: *mut snd_seq_real_time_t) {
    while (*tm).tv_nsec >= 1000000000 {
        /* roll-over */
        (*tm).tv_nsec -= 1000000000;
        (*tm).tv_sec += 1;
    }
}

/* increment timestamp */
#[inline]
pub unsafe fn snd_seq_inc_real_time(
    tm: *mut snd_seq_real_time_t,
    inc: *mut snd_seq_real_time_t,
) {
    (*tm).tv_sec += (*inc).tv_sec;
    (*tm).tv_nsec += (*inc).tv_nsec;
    snd_seq_sanity_real_time(tm);
}

#[inline]
pub unsafe fn snd_seq_inc_time_nsec(tm: *mut snd_seq_real_time_t, nsec: c_ulong) {
    (*tm).tv_nsec += nsec;
    snd_seq_sanity_real_time(tm);
}

#[repr(C)]
pub struct snd_seq_queue {
    _private: [u8; 0],
}

unsafe extern "C" {
    /* called by timer isr */
    pub fn snd_seq_timer_open(q: *mut snd_seq_queue) -> c_int;
    pub fn snd_seq_timer_close(q: *mut snd_seq_queue) -> c_int;
    pub fn snd_seq_timer_defaults(tmr: *mut snd_seq_timer);
    pub fn snd_seq_timer_reset(tmr: *mut snd_seq_timer);
    pub fn snd_seq_timer_stop(tmr: *mut snd_seq_timer) -> c_int;
    pub fn snd_seq_timer_start(tmr: *mut snd_seq_timer) -> c_int;
    pub fn snd_seq_timer_continue(tmr: *mut snd_seq_timer) -> c_int;
    pub fn snd_seq_timer_set_tempo(tmr: *mut snd_seq_timer, tempo: c_int) -> c_int;
    pub fn snd_seq_timer_set_tempo_ppq(
        tmr: *mut snd_seq_timer,
        tempo: c_int,
        ppq: c_int,
        tempo_base: c_uint,
    ) -> c_int;
    pub fn snd_seq_timer_set_position_tick(
        tmr: *mut snd_seq_timer,
        position: snd_seq_tick_time_t,
    ) -> c_int;
    pub fn snd_seq_timer_set_position_time(
        tmr: *mut snd_seq_timer,
        position: snd_seq_real_time_t,
    ) -> c_int;
    pub fn snd_seq_timer_set_skew(
        tmr: *mut snd_seq_timer,
        skew: c_uint,
        base: c_uint,
    ) -> c_int;
    pub fn snd_seq_timer_get_cur_time(
        tmr: *mut snd_seq_timer,
        adjust_ktime: bool,
    ) -> snd_seq_real_time_t;
    pub fn snd_seq_timer_get_cur_tick(tmr: *mut snd_seq_timer) -> snd_seq_tick_time_t;

    pub static mut seq_default_timer_class: c_int;
    pub static mut seq_default_timer_sclass: c_int;
    pub static mut seq_default_timer_card: c_int;
    pub static mut seq_default_timer_device: c_int;
    pub static mut seq_default_timer_subdevice: c_int;
    pub static mut seq_default_timer_resolution: c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
