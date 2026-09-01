// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA sequencer Timer
 *   Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 *                              Jaroslav Kysela <perex@perex.cz>
 */

/* Dependencies from C includes:
 * <sound/core.h>, <linux/slab.h>, "seq_timer.h", "seq_queue.h", "seq_info.h"
 */

/* allowed sequencer timer frequencies, in Hz */
const MIN_FREQUENCY: c_ulong = 10;
const MAX_FREQUENCY: c_ulong = 6250;
const DEFAULT_FREQUENCY: c_ulong = 1000;

const SKEW_BASE: c_uint = 0x10000; /* 16bit shift */

unsafe fn snd_seq_timer_set_tick_resolution(tmr: *mut snd_seq_timer) {
    let threshold: c_uint = if (*tmr).tempo_base == 1000 { 1000000 } else { 10000 };

    if (*tmr).tempo < threshold {
        (*tmr).tick.resolution = ((*tmr).tempo * (*tmr).tempo_base) / (*tmr).ppq;
    } else {
        /* might overflow.. */
        let mut s: c_uint;
        s = (*tmr).tempo % (*tmr).ppq;
        s = (s * (*tmr).tempo_base) / (*tmr).ppq;
        (*tmr).tick.resolution = ((*tmr).tempo / (*tmr).ppq) * (*tmr).tempo_base;
        (*tmr).tick.resolution += s;
    }
    if (*tmr).tick.resolution <= 0 {
        (*tmr).tick.resolution = 1;
    }
    snd_seq_timer_update_tick(&mut (*tmr).tick, 0);
}

/* create new timer (constructor) */
pub unsafe extern "C" fn snd_seq_timer_new() -> *mut snd_seq_timer {
    let tmr: *mut snd_seq_timer;

    tmr = kzalloc_obj_snd_seq_timer();
    if tmr.is_null() {
        return core::ptr::null_mut();
    }
    spin_lock_init(&mut (*tmr).lock);

    /* reset setup to defaults */
    snd_seq_timer_defaults(tmr);

    /* reset time */
    snd_seq_timer_reset(tmr);

    tmr
}

/* delete timer (destructor) */
pub unsafe extern "C" fn snd_seq_timer_delete(tmr: *mut *mut snd_seq_timer) {
    let t: *mut snd_seq_timer = *tmr;
    let mut ti: *mut snd_timer_instance;

    if t.is_null() {
        pr_debug(c"ALSA: seq: snd_seq_timer_delete() called with NULL timer\n".as_ptr());
        return;
    }

    /* scoped_guard(spinlock_irq, &t->lock) */
    ti = (*t).timeri;
    (*t).timeri = core::ptr::null_mut();

    if !ti.is_null() {
        snd_timer_close(ti);
        snd_timer_instance_free(ti);
    }

    *tmr = core::ptr::null_mut();
    (*t).running = 0;

    /* reset time */
    snd_seq_timer_stop(t);
    snd_seq_timer_reset(t);

    kfree(t.cast());
}

pub unsafe extern "C" fn snd_seq_timer_defaults(tmr: *mut snd_seq_timer) {
    /* guard(spinlock_irqsave)(&tmr->lock); */
    /* setup defaults */
    (*tmr).ppq = 96;        /* 96 PPQ */
    (*tmr).tempo = 500000;  /* 120 BPM */
    (*tmr).tempo_base = 1000; /* 1us */
    snd_seq_timer_set_tick_resolution(tmr);
    (*tmr).running = 0;

    (*tmr).type_ = SNDRV_SEQ_TIMER_ALSA;
    (*tmr).alsa_id.dev_class = seq_default_timer_class;
    (*tmr).alsa_id.dev_sclass = seq_default_timer_sclass;
    (*tmr).alsa_id.card = seq_default_timer_card;
    (*tmr).alsa_id.device = seq_default_timer_device;
    (*tmr).alsa_id.subdevice = seq_default_timer_subdevice;
    (*tmr).preferred_resolution = seq_default_timer_resolution;

    (*tmr).skew_base = SKEW_BASE;
    (*tmr).skew = (*tmr).skew_base;
}

unsafe fn seq_timer_reset(tmr: *mut snd_seq_timer) {
    /* reset time & songposition */
    (*tmr).cur_time.tv_sec = 0;
    (*tmr).cur_time.tv_nsec = 0;

    (*tmr).tick.cur_tick = 0;
    (*tmr).tick.fraction = 0;
}

pub unsafe extern "C" fn snd_seq_timer_reset(tmr: *mut snd_seq_timer) {
    /* guard(spinlock_irqsave)(&tmr->lock); */
    seq_timer_reset(tmr);
}

/* called by timer interrupt routine. the period time since previous invocation is passed */
unsafe extern "C" fn snd_seq_timer_interrupt(
    timeri: *mut snd_timer_instance,
    mut resolution: c_ulong,
    ticks: c_ulong,
) {
    let q: *mut snd_seq_queue = (*timeri).callback_data.cast();
    let tmr: *mut snd_seq_timer;

    if q.is_null() {
        return;
    }
    tmr = (*q).timer;
    if tmr.is_null() {
        return;
    }

    /* scoped_guard(spinlock_irqsave, &tmr->lock) */
    if (*tmr).running == 0 {
        return;
    }

    resolution = resolution.wrapping_mul(ticks);
    if (*tmr).skew != (*tmr).skew_base {
        /* FIXME: assuming skew_base = 0x10000 */
        resolution = (resolution >> 16).wrapping_mul((*tmr).skew as c_ulong)
            + (((resolution & 0xffff).wrapping_mul((*tmr).skew as c_ulong)) >> 16);
    }

    /* update timer */
    snd_seq_inc_time_nsec(&mut (*tmr).cur_time, resolution);

    /* calculate current tick */
    snd_seq_timer_update_tick(&mut (*tmr).tick, resolution);

    /* register actual time of this timer update */
    ktime_get_ts64(&mut (*tmr).last_update);

    /* check queues and dispatch events */
    snd_seq_check_queue(q, 1, 0);
}

/* set current tempo */
pub unsafe extern "C" fn snd_seq_timer_set_tempo(tmr: *mut snd_seq_timer, tempo: c_int) -> c_int {
    if snd_BUG_ON(tmr.is_null()) != 0 {
        return -EINVAL;
    }
    if tempo <= 0 {
        return -EINVAL;
    }
    /* guard(spinlock_irqsave)(&tmr->lock); */
    if tempo as c_uint != (*tmr).tempo {
        (*tmr).tempo = tempo as c_uint;
        snd_seq_timer_set_tick_resolution(tmr);
    }
    0
}

/* set current tempo, ppq and base in a shot */
pub unsafe extern "C" fn snd_seq_timer_set_tempo_ppq(
    tmr: *mut snd_seq_timer,
    tempo: c_int,
    ppq: c_int,
    tempo_base: c_uint,
) -> c_int {
    let changed: c_int;

    if snd_BUG_ON(tmr.is_null()) != 0 {
        return -EINVAL;
    }
    if tempo <= 0 || ppq <= 0 {
        return -EINVAL;
    }
    /* allow only 10ns or 1us tempo base for now */
    if tempo_base != 0 && tempo_base != 10 && tempo_base != 1000 {
        return -EINVAL;
    }
    /* guard(spinlock_irqsave)(&tmr->lock); */
    if (*tmr).running != 0 && ppq as c_uint != (*tmr).ppq {
        /* refuse to change ppq on running timers */
        /* because it will upset the song position (ticks) */
        pr_debug(c"ALSA: seq: cannot change ppq of a running timer\n".as_ptr());
        return -EBUSY;
    }
    changed = ((tempo as c_uint != (*tmr).tempo) || (ppq as c_uint != (*tmr).ppq)) as c_int;
    (*tmr).tempo = tempo as c_uint;
    (*tmr).ppq = ppq as c_uint;
    (*tmr).tempo_base = if tempo_base != 0 { tempo_base } else { 1000 };
    if changed != 0 {
        snd_seq_timer_set_tick_resolution(tmr);
    }
    0
}

/* set current tick position */
pub unsafe extern "C" fn snd_seq_timer_set_position_tick(
    tmr: *mut snd_seq_timer,
    position: snd_seq_tick_time_t,
) -> c_int {
    if snd_BUG_ON(tmr.is_null()) != 0 {
        return -EINVAL;
    }

    /* guard(spinlock_irqsave)(&tmr->lock); */
    (*tmr).tick.cur_tick = position;
    (*tmr).tick.fraction = 0;
    0
}

/* set current real-time position */
pub unsafe extern "C" fn snd_seq_timer_set_position_time(
    tmr: *mut snd_seq_timer,
    mut position: snd_seq_real_time_t,
) -> c_int {
    if snd_BUG_ON(tmr.is_null()) != 0 {
        return -EINVAL;
    }

    snd_seq_sanity_real_time(&mut position);
    /* guard(spinlock_irqsave)(&tmr->lock); */
    (*tmr).cur_time = position;
    0
}

/* set timer skew */
pub unsafe extern "C" fn snd_seq_timer_set_skew(
    tmr: *mut snd_seq_timer,
    skew: c_uint,
    base: c_uint,
) -> c_int {
    if snd_BUG_ON(tmr.is_null()) != 0 {
        return -EINVAL;
    }

    /* FIXME */
    if base != SKEW_BASE {
        pr_debug(c"ALSA: seq: invalid skew base 0x%x\n".as_ptr(), base);
        return -EINVAL;
    }
    /* guard(spinlock_irqsave)(&tmr->lock); */
    (*tmr).skew = skew;
    0
}

pub unsafe extern "C" fn snd_seq_timer_open(q: *mut snd_seq_queue) -> c_int {
    let t: *mut snd_timer_instance;
    let tmr: *mut snd_seq_timer;
    let mut str_: [c_char; 32] = [0; 32];
    let mut err: c_int;

    tmr = (*q).timer;
    if snd_BUG_ON(tmr.is_null()) != 0 {
        return -EINVAL;
    }
    if !(*tmr).timeri.is_null() {
        return -EBUSY;
    }
    sprintf(str_.as_mut_ptr(), c"sequencer queue %i".as_ptr(), (*q).queue);
    if (*tmr).type_ != SNDRV_SEQ_TIMER_ALSA {
        /* standard ALSA timer */
        return -EINVAL;
    }
    if (*tmr).alsa_id.dev_class != SNDRV_TIMER_CLASS_SLAVE {
        (*tmr).alsa_id.dev_sclass = SNDRV_TIMER_SCLASS_SEQUENCER;
    }
    t = snd_timer_instance_new(str_.as_mut_ptr());
    if t.is_null() {
        return -ENOMEM;
    }
    (*t).callback = Some(snd_seq_timer_interrupt);
    (*t).callback_data = q.cast();
    (*t).flags |= SNDRV_TIMER_IFLG_AUTO;
    err = snd_timer_open(t, &mut (*tmr).alsa_id, (*q).queue);
    if err < 0 && (*tmr).alsa_id.dev_class != SNDRV_TIMER_CLASS_SLAVE {
        if (*tmr).alsa_id.dev_class != SNDRV_TIMER_CLASS_GLOBAL
            || (*tmr).alsa_id.device != SNDRV_TIMER_GLOBAL_SYSTEM
        {
            let mut tid: snd_timer_id = core::mem::zeroed();
            tid.dev_class = SNDRV_TIMER_CLASS_GLOBAL;
            tid.dev_sclass = SNDRV_TIMER_SCLASS_SEQUENCER;
            tid.card = -1;
            tid.device = SNDRV_TIMER_GLOBAL_SYSTEM;
            err = snd_timer_open(t, &mut tid, (*q).queue);
        }
    }
    if err < 0 {
        pr_err(c"ALSA: seq fatal error: cannot create timer (%i)\n".as_ptr(), err);
        snd_timer_instance_free(t);
        return err;
    }
    /* scoped_guard(spinlock_irq, &tmr->lock) */
    if !(*tmr).timeri.is_null() {
        err = -EBUSY;
    } else {
        (*tmr).timeri = t;
    }
    if err < 0 {
        snd_timer_close(t);
        snd_timer_instance_free(t);
        return err;
    }
    0
}

pub unsafe extern "C" fn snd_seq_timer_close(q: *mut snd_seq_queue) -> c_int {
    let tmr: *mut snd_seq_timer;
    let t: *mut snd_timer_instance;

    tmr = (*q).timer;
    if snd_BUG_ON(tmr.is_null()) != 0 {
        return -EINVAL;
    }
    /* scoped_guard(spinlock_irq, &tmr->lock) */
    t = (*tmr).timeri;
    (*tmr).timeri = core::ptr::null_mut();

    if !t.is_null() {
        snd_timer_close(t);
        snd_timer_instance_free(t);
    }
    0
}

unsafe fn seq_timer_stop(tmr: *mut snd_seq_timer) -> c_int {
    if (*tmr).timeri.is_null() {
        return -EINVAL;
    }
    if (*tmr).running == 0 {
        return 0;
    }
    (*tmr).running = 0;
    snd_timer_pause((*tmr).timeri);
    0
}

pub unsafe extern "C" fn snd_seq_timer_stop(tmr: *mut snd_seq_timer) -> c_int {
    /* guard(spinlock_irqsave)(&tmr->lock); */
    seq_timer_stop(tmr)
}

unsafe fn initialize_timer(tmr: *mut snd_seq_timer) -> c_int {
    let mut freq: c_ulong;

    let t: *mut snd_timer = snd_timeri_timer_get((*tmr).timeri);
    if t.is_null() {
        return -EINVAL;
    }

    freq = (*tmr).preferred_resolution;
    if freq == 0 {
        freq = DEFAULT_FREQUENCY;
    } else if freq < MIN_FREQUENCY {
        freq = MIN_FREQUENCY;
    } else if freq > MAX_FREQUENCY {
        freq = MAX_FREQUENCY;
    }

    (*tmr).ticks = 1;
    if ((*t).hw.flags & SNDRV_TIMER_HW_SLAVE) == 0 {
        let r: c_ulong = snd_timer_resolution((*tmr).timeri);
        let mut den: c_ulong = 0;

        if r != 0 && check_mul_overflow(r, freq, &mut den) == 0 {
            (*tmr).ticks = max(1_u32, (1000000000_u64 / den as u64) as c_uint);
        }
    }
    snd_timeri_timer_put(t);
    (*tmr).initialized = 1;
    0
}

unsafe fn seq_timer_start(tmr: *mut snd_seq_timer) -> c_int {
    if (*tmr).timeri.is_null() {
        return -EINVAL;
    }
    if (*tmr).running != 0 {
        seq_timer_stop(tmr);
    }
    seq_timer_reset(tmr);
    if initialize_timer(tmr) < 0 {
        return -EINVAL;
    }
    snd_timer_start((*tmr).timeri, (*tmr).ticks);
    (*tmr).running = 1;
    ktime_get_ts64(&mut (*tmr).last_update);
    0
}

pub unsafe extern "C" fn snd_seq_timer_start(tmr: *mut snd_seq_timer) -> c_int {
    /* guard(spinlock_irqsave)(&tmr->lock); */
    seq_timer_start(tmr)
}

unsafe fn seq_timer_continue(tmr: *mut snd_seq_timer) -> c_int {
    if (*tmr).timeri.is_null() {
        return -EINVAL;
    }
    if (*tmr).running != 0 {
        return -EBUSY;
    }
    if (*tmr).initialized == 0 {
        seq_timer_reset(tmr);
        if initialize_timer(tmr) < 0 {
            return -EINVAL;
        }
    }
    snd_timer_start((*tmr).timeri, (*tmr).ticks);
    (*tmr).running = 1;
    ktime_get_ts64(&mut (*tmr).last_update);
    0
}

pub unsafe extern "C" fn snd_seq_timer_continue(tmr: *mut snd_seq_timer) -> c_int {
    /* guard(spinlock_irqsave)(&tmr->lock); */
    seq_timer_continue(tmr)
}

/* return current 'real' time. use timeofday() to get better granularity. */
pub unsafe extern "C" fn snd_seq_timer_get_cur_time(
    tmr: *mut snd_seq_timer,
    adjust_ktime: bool,
) -> snd_seq_real_time_t {
    let mut cur_time: snd_seq_real_time_t;

    /* guard(spinlock_irqsave)(&tmr->lock); */
    cur_time = (*tmr).cur_time;
    if adjust_ktime && (*tmr).running != 0 {
        let mut tm: timespec64 = core::mem::zeroed();

        ktime_get_ts64(&mut tm);
        tm = timespec64_sub(tm, (*tmr).last_update);
        cur_time.tv_nsec += tm.tv_nsec;
        cur_time.tv_sec += tm.tv_sec;
        snd_seq_sanity_real_time(&mut cur_time);
    }
    cur_time
}

/* TODO: use interpolation on tick queue (will only be useful for very
 high PPQ values) */
pub unsafe extern "C" fn snd_seq_timer_get_cur_tick(tmr: *mut snd_seq_timer) -> snd_seq_tick_time_t {
    /* guard(spinlock_irqsave)(&tmr->lock); */
    (*tmr).tick.cur_tick
}

/* CONFIG_SND_PROC_FS: exported to seq_info.c */
#[cfg(CONFIG_SND_PROC_FS)]
pub unsafe extern "C" fn snd_seq_info_timer_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let mut idx: c_int;
    let mut tmr: *mut snd_seq_timer;
    let mut ti: *mut snd_timer_instance;
    let mut resolution: c_ulong;

    idx = 0;
    while idx < SNDRV_SEQ_MAX_QUEUES {
        let q: *mut snd_seq_queue = queueptr(idx);

        if q.is_null() {
            idx += 1;
            continue;
        }
        /* scoped_guard(mutex, &q->timer_mutex) */
        tmr = (*q).timer;
        if tmr.is_null() {
            snd_seq_queue_put(q);
            break;
        }
        ti = (*tmr).timeri;
        if ti.is_null() {
            snd_seq_queue_put(q);
            break;
        }

        let t: *mut snd_timer = snd_timeri_timer_get(ti);
        snd_iprintf(
            buffer,
            c"Timer for queue %i : %s\n".as_ptr(),
            (*q).queue,
            if !t.is_null() { (*t).name.as_ptr() } else { c"DEAD".as_ptr() },
        );
        resolution = snd_timer_resolution(ti).wrapping_mul((*tmr).ticks as c_ulong);
        snd_iprintf(
            buffer,
            c"  Period time : %lu.%09lu\n".as_ptr(),
            resolution / 1000000000,
            resolution % 1000000000,
        );
        snd_iprintf(
            buffer,
            c"  Skew : %u / %u\n".as_ptr(),
            (*tmr).skew,
            (*tmr).skew_base,
        );
        if !t.is_null() {
            snd_timeri_timer_put(t);
        }
        snd_seq_queue_put(q);
        idx += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
