// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * Timer control routines
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

/*
 * Translated from C implementation source. Original dependencies:
 * "seq_oss_timer.h", "seq_oss_event.h", <sound/seq_oss_legacy.h>,
 * and <linux/slab.h>.
 */

const MIN_OSS_TEMPO: c_int = 8;
const MAX_OSS_TEMPO: c_int = 360;
const MIN_OSS_TIMEBASE: c_int = 1;
const MAX_OSS_TIMEBASE: c_int = 1000;

/*
 */
unsafe fn calc_alsa_tempo(timer: *mut seq_oss_timer);
unsafe fn send_timer_event(dp: *mut seq_oss_devinfo, type_: c_int, value: c_int) -> c_int;

/*
 * create and register a new timer.
 * if queue is not started yet, start it.
 */
pub unsafe extern "C" fn snd_seq_oss_timer_new(
    dp: *mut seq_oss_devinfo,
) -> *mut seq_oss_timer {
    let rec: *mut seq_oss_timer;

    rec = kzalloc_obj::<seq_oss_timer>();
    if rec.is_null() {
        return core::ptr::null_mut();
    }

    (*rec).dp = dp;
    (*rec).cur_tick = 0;
    (*rec).realtime = 0;
    (*rec).running = 0;
    (*rec).oss_tempo = 60;
    (*rec).oss_timebase = 100;
    calc_alsa_tempo(rec);

    rec
}

/*
 * delete timer.
 * if no more timer exists, stop the queue.
 */
pub unsafe extern "C" fn snd_seq_oss_timer_delete(rec: *mut seq_oss_timer) {
    if !rec.is_null() {
        snd_seq_oss_timer_stop(rec);
        kfree(rec as *mut c_void);
    }
}

/*
 * process one timing event
 * return 1 : event proceseed -- skip this event
 *        0 : not a timer event -- enqueue this event
 */
pub unsafe extern "C" fn snd_seq_oss_process_timer_event(
    rec: *mut seq_oss_timer,
    ev: *mut evrec,
) -> c_int {
    let mut parm: abstime_t = (*ev).t.time;

    if (*ev).t.code == EV_TIMING {
        match (*ev).t.cmd {
            TMR_WAIT_REL => {
                parm += (*rec).cur_tick;
                (*rec).realtime = 0;
                if parm == 0 {
                    (*rec).realtime = 1;
                } else if parm >= (*rec).cur_tick {
                    (*rec).realtime = 0;
                    (*rec).cur_tick = parm;
                }
                return 1; /* skip this event */
            }
            TMR_WAIT_ABS => {
                if parm == 0 {
                    (*rec).realtime = 1;
                } else if parm >= (*rec).cur_tick {
                    (*rec).realtime = 0;
                    (*rec).cur_tick = parm;
                }
                return 1; /* skip this event */
            }

            TMR_START => {
                snd_seq_oss_timer_start(rec);
                return 1;
            }

            _ => {}
        }
    } else if (*ev).s.code == SEQ_WAIT {
        /* time = from 1 to 3 bytes */
        parm = ((*ev).echo >> 8) & 0xffffff;
        if parm > (*rec).cur_tick {
            /* set next event time */
            (*rec).cur_tick = parm;
            (*rec).realtime = 0;
        }
        return 1;
    }

    0
}

/*
 * convert tempo units
 */
unsafe fn calc_alsa_tempo(timer: *mut seq_oss_timer) {
    (*timer).tempo = (60 * 1000000) / (*timer).oss_tempo;
    (*timer).ppq = (*timer).oss_timebase;
}

/*
 * dispatch a timer event
 */
unsafe fn send_timer_event(dp: *mut seq_oss_devinfo, type_: c_int, value: c_int) -> c_int {
    let mut ev: snd_seq_event = core::mem::zeroed();

    ev.type_ = type_;
    ev.source.client = (*dp).cseq;
    ev.source.port = 0;
    ev.dest.client = SNDRV_SEQ_CLIENT_SYSTEM;
    ev.dest.port = SNDRV_SEQ_PORT_SYSTEM_TIMER;
    ev.queue = (*dp).queue;
    ev.data.queue.queue = (*dp).queue;
    ev.data.queue.param.value = value;
    snd_seq_kernel_client_dispatch((*dp).cseq, &mut ev, 1, 0)
}

/*
 * set queue tempo and start queue
 */
pub unsafe extern "C" fn snd_seq_oss_timer_start(timer: *mut seq_oss_timer) -> c_int {
    let dp: *mut seq_oss_devinfo = (*timer).dp;
    let mut tmprec: snd_seq_queue_tempo = core::mem::zeroed();

    if (*timer).running != 0 {
        snd_seq_oss_timer_stop(timer);
    }

    tmprec.queue = (*dp).queue;
    tmprec.ppq = (*timer).ppq;
    tmprec.tempo = (*timer).tempo;
    snd_seq_set_queue_tempo((*dp).cseq, &mut tmprec);

    send_timer_event(dp, SNDRV_SEQ_EVENT_START, 0);
    (*timer).running = 1;
    (*timer).cur_tick = 0;
    0
}

/*
 * stop queue
 */
pub unsafe extern "C" fn snd_seq_oss_timer_stop(timer: *mut seq_oss_timer) -> c_int {
    if (*timer).running == 0 {
        return 0;
    }
    send_timer_event((*timer).dp, SNDRV_SEQ_EVENT_STOP, 0);
    (*timer).running = 0;
    0
}

/*
 * continue queue
 */
pub unsafe extern "C" fn snd_seq_oss_timer_continue(timer: *mut seq_oss_timer) -> c_int {
    if (*timer).running != 0 {
        return 0;
    }
    send_timer_event((*timer).dp, SNDRV_SEQ_EVENT_CONTINUE, 0);
    (*timer).running = 1;
    0
}

/*
 * change queue tempo
 */
pub unsafe extern "C" fn snd_seq_oss_timer_tempo(
    timer: *mut seq_oss_timer,
    mut value: c_int,
) -> c_int {
    if value < MIN_OSS_TEMPO {
        value = MIN_OSS_TEMPO;
    } else if value > MAX_OSS_TEMPO {
        value = MAX_OSS_TEMPO;
    }
    (*timer).oss_tempo = value;
    calc_alsa_tempo(timer);
    if (*timer).running != 0 {
        send_timer_event((*timer).dp, SNDRV_SEQ_EVENT_TEMPO, (*timer).tempo);
    }
    0
}

/*
 * ioctls
 */
pub unsafe extern "C" fn snd_seq_oss_timer_ioctl(
    timer: *mut seq_oss_timer,
    cmd: c_uint,
    arg: *mut c_int,
) -> c_int {
    let mut value: c_int = 0;

    if cmd == SNDCTL_SEQ_CTRLRATE {
        /* if *arg == 0, just return the current rate */
        if get_user(&mut value, arg) != 0 {
            return -EFAULT;
        }
        if value != 0 {
            return -EINVAL;
        }
        value = (((*timer).oss_tempo * (*timer).oss_timebase) + 30) / 60;
        return if put_user(value, arg) != 0 { -EFAULT } else { 0 };
    }

    if (*(*timer).dp).seq_mode == SNDRV_SEQ_OSS_MODE_SYNTH {
        return 0;
    }

    match cmd {
        SNDCTL_TMR_START => snd_seq_oss_timer_start(timer),
        SNDCTL_TMR_STOP => snd_seq_oss_timer_stop(timer),
        SNDCTL_TMR_CONTINUE => snd_seq_oss_timer_continue(timer),
        SNDCTL_TMR_TEMPO => {
            if get_user(&mut value, arg) != 0 {
                return -EFAULT;
            }
            snd_seq_oss_timer_tempo(timer, value)
        }
        SNDCTL_TMR_TIMEBASE => {
            if get_user(&mut value, arg) != 0 {
                return -EFAULT;
            }
            if value < MIN_OSS_TIMEBASE {
                value = MIN_OSS_TIMEBASE;
            } else if value > MAX_OSS_TIMEBASE {
                value = MAX_OSS_TIMEBASE;
            }
            (*timer).oss_timebase = value;
            calc_alsa_tempo(timer);
            0
        }

        SNDCTL_TMR_METRONOME | SNDCTL_TMR_SELECT | SNDCTL_TMR_SOURCE => {
            /* not supported */
            0
        }
        _ => 0,
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
