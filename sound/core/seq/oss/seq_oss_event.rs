// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type c_int = i32;
type c_uint = u32;
type c_uchar = u8;
type snd_use_lock_t = c_void;

const EINVAL: c_int = 22;
const ENXIO: c_int = 6;

const LONG_EVENT_SIZE: usize = 8;

/* Constants are supplied by the OSS/ALSA sequencer headers in the full tree. */
const SEQ_EXTENDED: c_int = 0;
const EV_CHN_VOICE: c_int = 0;
const EV_CHN_COMMON: c_int = 0;
const EV_TIMING: c_int = 0;
const EV_SEQ_LOCAL: c_int = 0;
const EV_SYSEX: c_int = 0;
const SEQ_MIDIPUTC: c_int = 0;
const SEQ_ECHO: c_int = 0;
const SEQ_PRIVATE: c_int = 0;
const SEQ_NOTEOFF: c_int = 0;
const SEQ_NOTEON: c_int = 0;
const SEQ_WAIT: c_int = 0;
const SEQ_PGMCHANGE: c_int = 0;
const SEQ_SYNCTIMER: c_int = 0;
const SEQ_AFTERTOUCH: c_int = 0;
const SEQ_BALANCE: c_int = 0;
const SEQ_CONTROLLER: c_int = 0;
const SEQ_VOLMODE: c_int = 0;
const CTRL_PITCH_BENDER: c_int = 0;
const CTRL_PITCH_BENDER_RANGE: c_int = 0;
const CTL_PAN: c_int = 0;
const MIDI_NOTEON: c_int = 0;
const MIDI_NOTEOFF: c_int = 0;
const MIDI_KEY_PRESSURE: c_int = 0;
const MIDI_PGM_CHANGE: c_int = 0;
const MIDI_CTL_CHANGE: c_int = 0;
const MIDI_PITCH_BEND: c_int = 0;
const MIDI_CHN_PRESSURE: c_int = 0;
const TMR_ECHO: c_int = 0;
const TMR_STOP: c_int = 0;
const TMR_CONTINUE: c_int = 0;
const TMR_TEMPO: c_int = 0;
const SNDRV_SEQ_OSS_MODE_MUSIC: c_int = 0;
const SNDRV_SEQ_OSS_FILE_WRITE: c_int = 0;
const SNDRV_SEQ_OSS_PROCESS_EVENTS: c_int = 0;
const SNDRV_SEQ_OSS_PASS_EVENTS: c_int = 0;
const SNDRV_SEQ_OSS_PROCESS_KEYPRESS: c_int = 0;
const SNDRV_SEQ_EVENT_PGMCHANGE: c_int = 0;
const SNDRV_SEQ_EVENT_CHANPRESS: c_int = 0;
const SNDRV_SEQ_EVENT_CONTROLLER: c_int = 0;
const SNDRV_SEQ_EVENT_PITCHBEND: c_int = 0;
const SNDRV_SEQ_EVENT_REGPARAM: c_int = 0;
const SNDRV_SEQ_EVENT_CONTROL14: c_int = 0;
const SNDRV_SEQ_EVENT_KEYPRESS: c_int = 0;
const SNDRV_SEQ_EVENT_NOTEON: c_int = 0;
const SNDRV_SEQ_EVENT_NOTEOFF: c_int = 0;
const SNDRV_SEQ_EVENT_ECHO: c_int = 0;

#[repr(C)]
pub struct seq_oss_addr {
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
pub struct seq_oss_devinfo {
    pub seq_mode: c_int,
    pub file_mode: c_int,
    pub timer: *mut c_void,
    pub addr: seq_oss_addr,
    pub cseq: c_int,
    pub writeq: *mut c_void,
    pub readq: *mut c_void,
}

#[repr(C)]
pub struct seq_oss_synth_arg {
    pub event_passing: c_int,
}

#[repr(C)]
pub struct seq_oss_chinfo {
    pub note: c_int,
    pub vel: c_int,
}

#[repr(C)]
pub struct seq_oss_synthinfo {
    pub arg: seq_oss_synth_arg,
    pub ch: *mut seq_oss_chinfo,
    pub nr_voices: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_short {
    pub code: c_uchar,
    pub dev: c_uchar,
    pub parm1: c_uchar,
    pub parm2: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_note {
    pub code: c_uchar,
    pub chn: c_uchar,
    pub note: c_uchar,
    pub vel: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_extended {
    pub code: c_uchar,
    pub cmd: c_uchar,
    pub dev: c_uchar,
    pub chn: c_uchar,
    pub p1: c_uchar,
    pub p2: c_uchar,
    pub p3: c_uchar,
    pub p4: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_voice {
    pub code: c_uchar,
    pub dev: c_uchar,
    pub cmd: c_uchar,
    pub chn: c_uchar,
    pub note: c_uchar,
    pub parm: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_long {
    pub code: c_uchar,
    pub dev: c_uchar,
    pub cmd: c_uchar,
    pub chn: c_uchar,
    pub p1: c_int,
    pub val: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_timer {
    pub code: c_uchar,
    pub cmd: c_uchar,
    pub time: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_sysex {
    pub code: c_uchar,
    pub dev: c_uchar,
    pub buf: [c_uchar; 6],
}

#[repr(C)]
pub union evrec {
    pub s: evrec_short,
    pub n: evrec_note,
    pub e: evrec_extended,
    pub v: evrec_voice,
    pub l: evrec_long,
    pub t: evrec_timer,
    pub x: evrec_sysex,
    pub c: [c_uchar; LONG_EVENT_SIZE],
    pub echo: c_uint,
}

#[repr(C)]
pub struct snd_seq_addr {
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
pub struct snd_seq_note {
    pub channel: c_int,
    pub note: c_int,
    pub velocity: c_int,
}

#[repr(C)]
pub struct snd_seq_control {
    pub channel: c_int,
    pub param: c_int,
    pub value: c_int,
}

#[repr(C)]
pub union snd_seq_event_data {
    pub note: core::mem::ManuallyDrop<snd_seq_note>,
    pub control: core::mem::ManuallyDrop<snd_seq_control>,
    pub raw: [c_uchar; LONG_EVENT_SIZE],
}

#[repr(C)]
pub struct snd_seq_event {
    pub type_: c_int,
    pub source: snd_seq_addr,
    pub data: snd_seq_event_data,
}

unsafe extern "C" {
    fn snd_seq_oss_synth_sysex(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        buf: *const c_uchar,
        ev: *mut snd_seq_event,
    ) -> c_int;
    fn snd_seq_oss_midi_open(dp: *mut seq_oss_devinfo, dev: c_int, mode: c_int) -> c_int;
    fn snd_seq_oss_midi_filemode(dp: *mut seq_oss_devinfo, dev: c_int) -> c_int;
    fn snd_seq_oss_midi_putc(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        c: c_int,
        ev: *mut snd_seq_event,
        lockp: *mut *mut snd_use_lock_t,
    ) -> c_int;
    fn snd_seq_oss_synth_raw_event(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        data: *const c_uchar,
        ev: *mut snd_seq_event,
    ) -> c_int;
    fn snd_seq_oss_timer_reset(timer: *mut c_void) -> c_int;
    fn snd_seq_oss_timer_stop(timer: *mut c_void) -> c_int;
    fn snd_seq_oss_timer_continue(timer: *mut c_void) -> c_int;
    fn snd_seq_oss_timer_tempo(timer: *mut c_void, tempo: c_int) -> c_int;
    fn snd_seq_oss_synth_info(dp: *mut seq_oss_devinfo, dev: c_int) -> *mut seq_oss_synthinfo;
    fn snd_seq_oss_synth_addr(dp: *mut seq_oss_devinfo, dev: c_int, ev: *mut snd_seq_event);
    fn snd_seq_oss_fill_addr(
        dp: *mut seq_oss_devinfo,
        ev: *mut snd_seq_event,
        client: c_int,
        port: c_int,
    );
    fn snd_seq_oss_midi_input(
        ev: *mut snd_seq_event,
        direct: c_int,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_seq_oss_writeq_wakeup(writeq: *mut c_void, time: c_int);
    fn snd_seq_oss_readq_put_event(readq: *mut c_void, rec: *mut evrec);
}

fn is_write_mode(mode: c_int) -> bool {
    (mode & SNDRV_SEQ_OSS_FILE_WRITE) != 0
}

fn array_index_nospec(index: c_int, _size: c_int) -> c_int {
    index
}

/*
 * convert an OSS event to ALSA event
 * return 0 : enqueued
 *        non-zero : invalid - ignored
 */
pub unsafe extern "C" fn snd_seq_oss_process_event(
    dp: *mut seq_oss_devinfo,
    q: *mut evrec,
    ev: *mut snd_seq_event,
    lockp: *mut *mut snd_use_lock_t,
) -> c_int {
    unsafe {
        *lockp = ptr::null_mut();
        match (*q).s.code as c_int {
            SEQ_EXTENDED => return extended_event(dp, q, ev),
            EV_CHN_VOICE => return chn_voice_event(dp, q, ev),
            EV_CHN_COMMON => return chn_common_event(dp, q, ev),
            EV_TIMING => return timing_event(dp, q, ev),
            EV_SEQ_LOCAL => return local_event(dp, q, ev),
            EV_SYSEX => return snd_seq_oss_synth_sysex(dp, (*q).x.dev as c_int, (*q).x.buf.as_ptr(), ev),
            SEQ_MIDIPUTC => {
                if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
                    return -EINVAL;
                }
                /* put a midi byte */
                if !is_write_mode((*dp).file_mode) {
                    return -EINVAL;
                }
                if snd_seq_oss_midi_open(dp, (*q).s.dev as c_int, SNDRV_SEQ_OSS_FILE_WRITE) != 0 {
                    return -EINVAL;
                }
                if (snd_seq_oss_midi_filemode(dp, (*q).s.dev as c_int) & SNDRV_SEQ_OSS_FILE_WRITE) != 0 {
                    return snd_seq_oss_midi_putc(dp, (*q).s.dev as c_int, (*q).s.parm1 as c_int, ev, lockp);
                }
            }
            SEQ_ECHO => {
                if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
                    return -EINVAL;
                }
                return set_echo_event(dp, q, ev);
            }
            SEQ_PRIVATE => {
                if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
                    return -EINVAL;
                }
                return snd_seq_oss_synth_raw_event(dp, (*q).c[1] as c_int, (*q).c.as_ptr(), ev);
            }
            _ => {
                if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
                    return -EINVAL;
                }
                return old_event(dp, q, ev);
            }
        }
        -EINVAL
    }
}

/* old type events: mode1 only */
unsafe fn old_event(dp: *mut seq_oss_devinfo, q: *mut evrec, ev: *mut snd_seq_event) -> c_int {
    unsafe {
        match (*q).s.code as c_int {
            SEQ_NOTEOFF => return note_off_event(dp, 0, (*q).n.chn as c_int, (*q).n.note as c_int, (*q).n.vel as c_int, ev),
            SEQ_NOTEON => return note_on_event(dp, 0, (*q).n.chn as c_int, (*q).n.note as c_int, (*q).n.vel as c_int, ev),
            SEQ_WAIT => {}
            SEQ_PGMCHANGE => {
                return set_control_event(dp, 0, SNDRV_SEQ_EVENT_PGMCHANGE, (*q).n.chn as c_int, 0, (*q).n.note as c_int, ev);
            }
            SEQ_SYNCTIMER => return snd_seq_oss_timer_reset((*dp).timer),
            _ => {}
        }
        -EINVAL
    }
}

/* 8bytes extended event: mode1 only */
unsafe fn extended_event(dp: *mut seq_oss_devinfo, q: *mut evrec, ev: *mut snd_seq_event) -> c_int {
    unsafe {
        let mut val: c_int;

        match (*q).e.cmd as c_int {
            SEQ_NOTEOFF => return note_off_event(dp, (*q).e.dev as c_int, (*q).e.chn as c_int, (*q).e.p1 as c_int, (*q).e.p2 as c_int, ev),
            SEQ_NOTEON => return note_on_event(dp, (*q).e.dev as c_int, (*q).e.chn as c_int, (*q).e.p1 as c_int, (*q).e.p2 as c_int, ev),
            SEQ_PGMCHANGE => {
                return set_control_event(dp, (*q).e.dev as c_int, SNDRV_SEQ_EVENT_PGMCHANGE, (*q).e.chn as c_int, 0, (*q).e.p1 as c_int, ev);
            }
            SEQ_AFTERTOUCH => {
                return set_control_event(dp, (*q).e.dev as c_int, SNDRV_SEQ_EVENT_CHANPRESS, (*q).e.chn as c_int, 0, (*q).e.p1 as c_int, ev);
            }
            SEQ_BALANCE => {
                /* convert -128:127 to 0:127 */
                val = (*q).e.p1 as i8 as c_int;
                val = (val + 128) / 2;
                return set_control_event(dp, (*q).e.dev as c_int, SNDRV_SEQ_EVENT_CONTROLLER, (*q).e.chn as c_int, CTL_PAN, val, ev);
            }
            SEQ_CONTROLLER => {
                val = (((*q).e.p3 as i16 as c_int) << 8) | ((*q).e.p2 as i16 as c_int);
                match (*q).e.p1 as c_int {
                    CTRL_PITCH_BENDER => {
                        /* SEQ1 V2 control */
                        /* -0x2000:0x1fff */
                        return set_control_event(dp, (*q).e.dev as c_int, SNDRV_SEQ_EVENT_PITCHBEND, (*q).e.chn as c_int, 0, val, ev);
                    }
                    CTRL_PITCH_BENDER_RANGE => {
                        /* conversion: 100/semitone -> 128/semitone */
                        return set_control_event(dp, (*q).e.dev as c_int, SNDRV_SEQ_EVENT_REGPARAM, (*q).e.chn as c_int, 0, val * 128 / 100, ev);
                    }
                    _ => {
                        return set_control_event(dp, (*q).e.dev as c_int, SNDRV_SEQ_EVENT_CONTROL14, (*q).e.chn as c_int, (*q).e.p1 as c_int, val, ev);
                    }
                }
            }
            SEQ_VOLMODE => return snd_seq_oss_synth_raw_event(dp, (*q).e.dev as c_int, (*q).c.as_ptr(), ev),
            _ => {}
        }
        -EINVAL
    }
}

/* channel voice events: mode1 and 2 */
unsafe fn chn_voice_event(dp: *mut seq_oss_devinfo, q: *mut evrec, ev: *mut snd_seq_event) -> c_int {
    unsafe {
        if (*q).v.chn as c_int >= 32 {
            return -EINVAL;
        }
        match (*q).v.cmd as c_int {
            MIDI_NOTEON => return note_on_event(dp, (*q).v.dev as c_int, (*q).v.chn as c_int, (*q).v.note as c_int, (*q).v.parm as c_int, ev),
            MIDI_NOTEOFF => return note_off_event(dp, (*q).v.dev as c_int, (*q).v.chn as c_int, (*q).v.note as c_int, (*q).v.parm as c_int, ev),
            MIDI_KEY_PRESSURE => {
                return set_note_event(dp, (*q).v.dev as c_int, SNDRV_SEQ_EVENT_KEYPRESS, (*q).v.chn as c_int, (*q).v.note as c_int, (*q).v.parm as c_int, ev);
            }
            _ => {}
        }
        -EINVAL
    }
}

/* channel common events: mode1 and 2 */
unsafe fn chn_common_event(dp: *mut seq_oss_devinfo, q: *mut evrec, ev: *mut snd_seq_event) -> c_int {
    unsafe {
        if (*q).l.chn as c_int >= 32 {
            return -EINVAL;
        }
        match (*q).l.cmd as c_int {
            MIDI_PGM_CHANGE => return set_control_event(dp, (*q).l.dev as c_int, SNDRV_SEQ_EVENT_PGMCHANGE, (*q).l.chn as c_int, 0, (*q).l.p1, ev),
            MIDI_CTL_CHANGE => return set_control_event(dp, (*q).l.dev as c_int, SNDRV_SEQ_EVENT_CONTROLLER, (*q).l.chn as c_int, (*q).l.p1, (*q).l.val, ev),
            MIDI_PITCH_BEND => {
                /* conversion: 0:0x3fff -> -0x2000:0x1fff */
                return set_control_event(dp, (*q).l.dev as c_int, SNDRV_SEQ_EVENT_PITCHBEND, (*q).l.chn as c_int, 0, (*q).l.val - 8192, ev);
            }
            MIDI_CHN_PRESSURE => return set_control_event(dp, (*q).l.dev as c_int, SNDRV_SEQ_EVENT_CHANPRESS, (*q).l.chn as c_int, 0, (*q).l.val, ev),
            _ => {}
        }
        -EINVAL
    }
}

/* timer events: mode1 and mode2 */
unsafe fn timing_event(dp: *mut seq_oss_devinfo, q: *mut evrec, ev: *mut snd_seq_event) -> c_int {
    unsafe {
        match (*q).t.cmd as c_int {
            TMR_ECHO => {
                if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
                    return set_echo_event(dp, q, ev);
                } else {
                    let mut tmp: evrec = core::mem::zeroed();
                    /* XXX: only for little-endian! */
                    tmp.echo = (((*q).t.time as c_uint) << 8) | (SEQ_ECHO as c_uint);
                    return set_echo_event(dp, &mut tmp, ev);
                }
            }
            TMR_STOP => {
                if (*dp).seq_mode != 0 {
                    return snd_seq_oss_timer_stop((*dp).timer);
                }
                return 0;
            }
            TMR_CONTINUE => {
                if (*dp).seq_mode != 0 {
                    return snd_seq_oss_timer_continue((*dp).timer);
                }
                return 0;
            }
            TMR_TEMPO => {
                if (*dp).seq_mode != 0 {
                    return snd_seq_oss_timer_tempo((*dp).timer, (*q).t.time);
                }
                return 0;
            }
            _ => {}
        }
        -EINVAL
    }
}

/* local events: mode1 and 2 */
unsafe fn local_event(_dp: *mut seq_oss_devinfo, _q: *mut evrec, _ev: *mut snd_seq_event) -> c_int {
    -EINVAL
}

/*
 * process note-on event for OSS synth
 * three different modes are available:
 * - SNDRV_SEQ_OSS_PROCESS_EVENTS  (for one-voice per channel mode)
 *	Accept note 255 as volume change.
 * - SNDRV_SEQ_OSS_PASS_EVENTS
 *	Pass all events to lowlevel driver anyway
 * - SNDRV_SEQ_OSS_PROCESS_KEYPRESS  (mostly for Emu8000)
 *	Use key-pressure if note >= 128
 */
unsafe fn note_on_event(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    mut ch: c_int,
    note: c_int,
    vel: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    unsafe {
        let info = snd_seq_oss_synth_info(dp, dev);
        if info.is_null() {
            return -ENXIO;
        }

        match (*info).arg.event_passing {
            SNDRV_SEQ_OSS_PROCESS_EVENTS => {
                if (*info).ch.is_null() || ch < 0 || ch >= (*info).nr_voices {
                    /* pass directly */
                    return set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEON, ch, note, vel, ev);
                }

                ch = array_index_nospec(ch, (*info).nr_voices);
                let chp = (*info).ch.add(ch as usize);
                if note == 255 && (*chp).note >= 0 {
                    /* volume control */
                    let type_: c_int;

                    if (*chp).vel != 0 {
                        /* sample already started -- volume change */
                        type_ = SNDRV_SEQ_EVENT_KEYPRESS;
                    } else {
                        /* sample not started -- start now */
                        type_ = SNDRV_SEQ_EVENT_NOTEON;
                    }

                    (*chp).vel = vel;
                    return set_note_event(dp, dev, type_, ch, (*chp).note, vel, ev);
                } else if note >= 128 {
                    return -EINVAL; /* invalid */
                }

                if note != (*chp).note && (*chp).note >= 0 {
                    /* note changed - note off at beginning */
                    set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEOFF, ch, (*chp).note, 0, ev);
                }
                /* set current status */
                (*chp).note = note;
                (*chp).vel = vel;
                if vel != 0 {
                    /* non-zero velocity - start the note now */
                    return set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEON, ch, note, vel, ev);
                }
                return -EINVAL;
            }
            SNDRV_SEQ_OSS_PASS_EVENTS => {
                /* pass the event anyway */
                return set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEON, ch, note, vel, ev);
            }
            SNDRV_SEQ_OSS_PROCESS_KEYPRESS => {
                if note >= 128 {
                    /* key pressure: shifted by 128 */
                    return set_note_event(dp, dev, SNDRV_SEQ_EVENT_KEYPRESS, ch, note - 128, vel, ev);
                } else {
                    /* normal note-on event */
                    return set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEON, ch, note, vel, ev);
                }
            }
            _ => {}
        }
        -EINVAL
    }
}

/*
 * process note-off event for OSS synth
 */
unsafe fn note_off_event(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    ch: c_int,
    mut note: c_int,
    vel: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    unsafe {
        let info = snd_seq_oss_synth_info(dp, dev);
        if info.is_null() {
            return -ENXIO;
        }

        match (*info).arg.event_passing {
            SNDRV_SEQ_OSS_PROCESS_EVENTS => {
                if (*info).ch.is_null() || ch < 0 || ch >= (*info).nr_voices {
                    /* pass directly */
                    return set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEON, ch, note, vel, ev);
                }

                let ch = array_index_nospec(ch, (*info).nr_voices);
                let chp = (*info).ch.add(ch as usize);
                if (*chp).note >= 0 {
                    note = (*chp).note;
                    (*chp).vel = 0;
                    (*chp).note = -1;
                    return set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEOFF, ch, note, vel, ev);
                }
                return -EINVAL; /* invalid */
            }
            SNDRV_SEQ_OSS_PASS_EVENTS | SNDRV_SEQ_OSS_PROCESS_KEYPRESS => {
                /* pass the event anyway */
                return set_note_event(dp, dev, SNDRV_SEQ_EVENT_NOTEOFF, ch, note, vel, ev);
            }
            _ => {}
        }
        -EINVAL
    }
}

/*
 * create a note event
 */
unsafe fn set_note_event(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    type_: c_int,
    ch: c_int,
    note: c_int,
    vel: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    unsafe {
        if snd_seq_oss_synth_info(dp, dev).is_null() {
            return -ENXIO;
        }

        (*ev).type_ = type_;
        snd_seq_oss_synth_addr(dp, dev, ev);
        (*ev).data.note.channel = ch;
        (*ev).data.note.note = note;
        (*ev).data.note.velocity = vel;

        0
    }
}

/*
 * create a control event
 */
unsafe fn set_control_event(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    type_: c_int,
    ch: c_int,
    param: c_int,
    val: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    unsafe {
        if snd_seq_oss_synth_info(dp, dev).is_null() {
            return -ENXIO;
        }

        (*ev).type_ = type_;
        snd_seq_oss_synth_addr(dp, dev, ev);
        (*ev).data.control.channel = ch;
        (*ev).data.control.param = param;
        (*ev).data.control.value = val;

        0
    }
}

/*
 * create an echo event
 */
unsafe fn set_echo_event(dp: *mut seq_oss_devinfo, rec: *mut evrec, ev: *mut snd_seq_event) -> c_int {
    unsafe {
        (*ev).type_ = SNDRV_SEQ_EVENT_ECHO;
        /* echo back to itself */
        snd_seq_oss_fill_addr(dp, ev, (*dp).addr.client, (*dp).addr.port);
        ptr::copy_nonoverlapping(
            rec as *const c_uchar,
            &mut (*ev).data as *mut snd_seq_event_data as *mut c_uchar,
            LONG_EVENT_SIZE.min(size_of::<snd_seq_event_data>()),
        );
        0
    }
}

/*
 * event input callback from ALSA sequencer:
 * the echo event is processed here.
 */
pub unsafe extern "C" fn snd_seq_oss_event_input(
    ev: *mut snd_seq_event,
    direct: c_int,
    private_data: *mut c_void,
    _atomic: c_int,
    _hop: c_int,
) -> c_int {
    unsafe {
        let dp = private_data as *mut seq_oss_devinfo;
        let rec: *mut evrec;

        if (*ev).type_ != SNDRV_SEQ_EVENT_ECHO {
            return snd_seq_oss_midi_input(ev, direct, private_data);
        }

        if (*ev).source.client != (*dp).cseq {
            return 0; /* ignored */
        }

        rec = &mut (*ev).data as *mut snd_seq_event_data as *mut evrec;
        if (*rec).s.code as c_int == SEQ_SYNCTIMER {
            /* sync echo back */
            snd_seq_oss_writeq_wakeup((*dp).writeq, (*rec).t.time);
        } else {
            /* echo back event */
            if (*dp).readq.is_null() {
                return 0;
            }
            snd_seq_oss_readq_put_event((*dp).readq, rec);
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
