// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  MIDI byte <-> sequencer event coder
 *
 *  Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>,
 *                        Jaroslav Kysela <perex@perex.cz>
 */

// C includes translated as external dependencies:
// linux/slab.h, linux/errno.h, linux/string.h, linux/module.h,
// sound/core.h, sound/seq_kernel.h, sound/seq_midi_event.h, sound/asoundef.h

use core::ffi::{c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

extern "C" {
    static GFP_KERNEL: c_uint;

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);

    fn snd_seq_expand_var_event(
        ev: *mut snd_seq_event,
        count: c_long,
        buf: *mut c_uchar,
        in_kernel: c_int,
        size_aligned: c_int,
    ) -> c_long;
}

pub const ENOMEM: c_int = 12;
pub const ENOENT: c_int = 2;

pub const SNDRV_SEQ_EVENT_LENGTH_MASK: c_uchar = 0x03;
pub const SNDRV_SEQ_EVENT_LENGTH_FIXED: c_uchar = 0x00;
pub const SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_uchar = 0x01;

pub const SNDRV_SEQ_EVENT_NONE: c_int = 0;
pub const SNDRV_SEQ_EVENT_NOTEOFF: c_int = 5;
pub const SNDRV_SEQ_EVENT_NOTEON: c_int = 6;
pub const SNDRV_SEQ_EVENT_KEYPRESS: c_int = 7;
pub const SNDRV_SEQ_EVENT_CONTROLLER: c_int = 10;
pub const SNDRV_SEQ_EVENT_PGMCHANGE: c_int = 11;
pub const SNDRV_SEQ_EVENT_CHANPRESS: c_int = 12;
pub const SNDRV_SEQ_EVENT_PITCHBEND: c_int = 13;
pub const SNDRV_SEQ_EVENT_CONTROL14: c_int = 14;
pub const SNDRV_SEQ_EVENT_NONREGPARAM: c_int = 15;
pub const SNDRV_SEQ_EVENT_REGPARAM: c_int = 16;
pub const SNDRV_SEQ_EVENT_SONGPOS: c_int = 20;
pub const SNDRV_SEQ_EVENT_SONGSEL: c_int = 21;
pub const SNDRV_SEQ_EVENT_QFRAME: c_int = 22;
pub const SNDRV_SEQ_EVENT_TIMESIGN: c_int = 23;
pub const SNDRV_SEQ_EVENT_KEYSIGN: c_int = 24;
pub const SNDRV_SEQ_EVENT_START: c_int = 30;
pub const SNDRV_SEQ_EVENT_CONTINUE: c_int = 31;
pub const SNDRV_SEQ_EVENT_STOP: c_int = 32;
pub const SNDRV_SEQ_EVENT_SETPOS_TICK: c_int = 33;
pub const SNDRV_SEQ_EVENT_SETPOS_TIME: c_int = 34;
pub const SNDRV_SEQ_EVENT_TEMPO: c_int = 35;
pub const SNDRV_SEQ_EVENT_CLOCK: c_int = 36;
pub const SNDRV_SEQ_EVENT_TICK: c_int = 37;
pub const SNDRV_SEQ_EVENT_QUEUE_SKEW: c_int = 38;
pub const SNDRV_SEQ_EVENT_TUNE_REQUEST: c_int = 40;
pub const SNDRV_SEQ_EVENT_RESET: c_int = 41;
pub const SNDRV_SEQ_EVENT_SENSING: c_int = 42;
pub const SNDRV_SEQ_EVENT_SYSEX: c_int = 130;

pub const MIDI_CMD_COMMON_SYSEX: c_uchar = 0xf0;
pub const MIDI_CMD_COMMON_MTC_QUARTER: c_uchar = 0xf1;
pub const MIDI_CMD_COMMON_SONG_POS: c_uchar = 0xf2;
pub const MIDI_CMD_COMMON_SONG_SELECT: c_uchar = 0xf3;
pub const MIDI_CMD_COMMON_TUNE_REQUEST: c_uchar = 0xf6;
pub const MIDI_CMD_COMMON_SYSEX_END: c_uchar = 0xf7;
pub const MIDI_CMD_COMMON_CLOCK: c_uchar = 0xf8;
pub const MIDI_CMD_COMMON_START: c_uchar = 0xfa;
pub const MIDI_CMD_COMMON_CONTINUE: c_uchar = 0xfb;
pub const MIDI_CMD_COMMON_STOP: c_uchar = 0xfc;
pub const MIDI_CMD_COMMON_SENSING: c_uchar = 0xfe;
pub const MIDI_CMD_COMMON_RESET: c_uchar = 0xff;

pub const MIDI_CMD_CONTROL: c_uchar = 0xb0;
pub const MIDI_CTL_NONREG_PARM_NUM_MSB: c_uchar = 99;
pub const MIDI_CTL_NONREG_PARM_NUM_LSB: c_uchar = 98;
pub const MIDI_CTL_REGIST_PARM_NUM_MSB: c_uchar = 101;
pub const MIDI_CTL_REGIST_PARM_NUM_LSB: c_uchar = 100;
pub const MIDI_CTL_MSB_DATA_ENTRY: c_uchar = 6;
pub const MIDI_CTL_LSB_DATA_ENTRY: c_uchar = 38;

/* event type, index into status_event[] */
/* from 0 to 6 are normal commands (note off, on, etc.) for 0x9?-0xe? */
const ST_INVALID: usize = 7;
const ST_SPECIAL: usize = 8;
const ST_SYSEX: usize = ST_SPECIAL;
/* from 8 to 15 are events for 0xf0-0xf7 */

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_event {
    pub qlen: c_int,
    pub read: c_int,
    pub type_: c_int,
    pub lastcmd: c_uchar,
    pub nostat: c_uchar,
    pub lock: spinlock_t,
    pub bufsize: c_int,
    pub buf: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_note {
    pub channel: c_uchar,
    pub note: c_uchar,
    pub velocity: c_uchar,
    pub off_velocity: c_uchar,
    pub duration: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_ctrl {
    pub channel: c_uchar,
    pub unused: [c_uchar; 3],
    pub param: c_uint,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_ext {
    pub len: c_uint,
    pub ptr: *mut c_void,
}

#[repr(C)]
pub union snd_seq_event_data {
    pub note: snd_seq_ev_note,
    pub control: snd_seq_ev_ctrl,
    pub ext: snd_seq_ev_ext,
}

#[repr(C)]
pub struct snd_seq_event {
    pub type_: c_uchar,
    pub flags: c_uchar,
    pub tag: c_uchar,
    pub queue: c_uchar,
    pub time: [c_uchar; 12],
    pub source: [c_uchar; 2],
    pub dest: [c_uchar; 2],
    pub data: snd_seq_event_data,
}

type EncodeFn = unsafe fn(*mut snd_midi_event, *mut snd_seq_event);
type DecodeFn = unsafe fn(*mut snd_seq_event, *mut c_uchar);

#[repr(C)]
#[derive(Copy, Clone)]
struct status_event_list {
    event: c_int,
    qlen: c_int,
    encode: Option<EncodeFn>,
    decode: Option<DecodeFn>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct extra_event_list {
    event: c_int,
    decode: unsafe fn(*mut snd_midi_event, *mut c_uchar, c_int, *mut snd_seq_event) -> c_int,
}

/*
 * event list
 */
static STATUS_EVENT: [status_event_list; 24] = [
    /* 0x80 - 0xef */
    status_event_list { event: SNDRV_SEQ_EVENT_NOTEOFF, qlen: 2, encode: Some(note_event), decode: Some(note_decode) },
    status_event_list { event: SNDRV_SEQ_EVENT_NOTEON, qlen: 2, encode: Some(note_event), decode: Some(note_decode) },
    status_event_list { event: SNDRV_SEQ_EVENT_KEYPRESS, qlen: 2, encode: Some(note_event), decode: Some(note_decode) },
    status_event_list { event: SNDRV_SEQ_EVENT_CONTROLLER, qlen: 2, encode: Some(two_param_ctrl_event), decode: Some(two_param_decode) },
    status_event_list { event: SNDRV_SEQ_EVENT_PGMCHANGE, qlen: 1, encode: Some(one_param_ctrl_event), decode: Some(one_param_decode) },
    status_event_list { event: SNDRV_SEQ_EVENT_CHANPRESS, qlen: 1, encode: Some(one_param_ctrl_event), decode: Some(one_param_decode) },
    status_event_list { event: SNDRV_SEQ_EVENT_PITCHBEND, qlen: 2, encode: Some(pitchbend_ctrl_event), decode: Some(pitchbend_decode) },
    /* invalid */
    status_event_list { event: SNDRV_SEQ_EVENT_NONE, qlen: -1, encode: None, decode: None },
    /* 0xf0 - 0xff */
    status_event_list { event: SNDRV_SEQ_EVENT_SYSEX, qlen: 1, encode: None, decode: None }, /* sysex: 0xf0 */
    status_event_list { event: SNDRV_SEQ_EVENT_QFRAME, qlen: 1, encode: Some(one_param_event), decode: Some(one_param_decode) }, /* 0xf1 */
    status_event_list { event: SNDRV_SEQ_EVENT_SONGPOS, qlen: 2, encode: Some(songpos_event), decode: Some(songpos_decode) }, /* 0xf2 */
    status_event_list { event: SNDRV_SEQ_EVENT_SONGSEL, qlen: 1, encode: Some(one_param_event), decode: Some(one_param_decode) }, /* 0xf3 */
    status_event_list { event: SNDRV_SEQ_EVENT_NONE, qlen: -1, encode: None, decode: None }, /* 0xf4 */
    status_event_list { event: SNDRV_SEQ_EVENT_NONE, qlen: -1, encode: None, decode: None }, /* 0xf5 */
    status_event_list { event: SNDRV_SEQ_EVENT_TUNE_REQUEST, qlen: 0, encode: None, decode: None }, /* 0xf6 */
    status_event_list { event: SNDRV_SEQ_EVENT_NONE, qlen: -1, encode: None, decode: None }, /* 0xf7 */
    status_event_list { event: SNDRV_SEQ_EVENT_CLOCK, qlen: 0, encode: None, decode: None }, /* 0xf8 */
    status_event_list { event: SNDRV_SEQ_EVENT_NONE, qlen: -1, encode: None, decode: None }, /* 0xf9 */
    status_event_list { event: SNDRV_SEQ_EVENT_START, qlen: 0, encode: None, decode: None }, /* 0xfa */
    status_event_list { event: SNDRV_SEQ_EVENT_CONTINUE, qlen: 0, encode: None, decode: None }, /* 0xfb */
    status_event_list { event: SNDRV_SEQ_EVENT_STOP, qlen: 0, encode: None, decode: None }, /* 0xfc */
    status_event_list { event: SNDRV_SEQ_EVENT_NONE, qlen: -1, encode: None, decode: None }, /* 0xfd */
    status_event_list { event: SNDRV_SEQ_EVENT_SENSING, qlen: 0, encode: None, decode: None }, /* 0xfe */
    status_event_list { event: SNDRV_SEQ_EVENT_RESET, qlen: 0, encode: None, decode: None }, /* 0xff */
];

static EXTRA_EVENT: [extra_event_list; 3] = [
    extra_event_list { event: SNDRV_SEQ_EVENT_CONTROL14, decode: extra_decode_ctrl14 },
    extra_event_list { event: SNDRV_SEQ_EVENT_NONREGPARAM, decode: extra_decode_xrpn },
    extra_event_list { event: SNDRV_SEQ_EVENT_REGPARAM, decode: extra_decode_xrpn },
];

/*
 *  new/delete record
 */

#[no_mangle]
pub unsafe extern "C" fn snd_midi_event_new(bufsize: c_int, rdev: *mut *mut snd_midi_event) -> c_int {
    let dev: *mut snd_midi_event;

    *rdev = ptr::null_mut();
    dev = kzalloc(mem::size_of::<snd_midi_event>(), GFP_KERNEL) as *mut snd_midi_event;
    if dev.is_null() {
        return -ENOMEM;
    }
    if bufsize > 0 {
        (*dev).buf = kmalloc(bufsize as usize, GFP_KERNEL) as *mut c_uchar;
        if (*dev).buf.is_null() {
            kfree(dev as *const c_void);
            return -ENOMEM;
        }
    }
    (*dev).bufsize = bufsize;
    (*dev).lastcmd = 0xff;
    (*dev).type_ = ST_INVALID as c_int;
    spin_lock_init(&mut (*dev).lock);
    *rdev = dev;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_midi_event_free(dev: *mut snd_midi_event) {
    if !dev.is_null() {
        kfree((*dev).buf as *const c_void);
        kfree(dev as *const c_void);
    }
}

/*
 * initialize record
 */
unsafe fn reset_encode(dev: *mut snd_midi_event) {
    (*dev).read = 0;
    (*dev).qlen = 0;
    (*dev).type_ = ST_INVALID as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn snd_midi_event_reset_encode(dev: *mut snd_midi_event) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*dev).lock, &mut flags);
    reset_encode(dev);
    spin_unlock_irqrestore(&mut (*dev).lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn snd_midi_event_reset_decode(dev: *mut snd_midi_event) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*dev).lock, &mut flags);
    (*dev).lastcmd = 0xff;
    spin_unlock_irqrestore(&mut (*dev).lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn snd_midi_event_no_status(dev: *mut snd_midi_event, on: c_int) {
    (*dev).nostat = if on != 0 { 1 } else { 0 };
}

/*
 *  read one byte and encode to sequencer event:
 *  return true if MIDI bytes are encoded to an event
 *         false data is not finished
 */
#[no_mangle]
pub unsafe extern "C" fn snd_midi_event_encode_byte(
    dev: *mut snd_midi_event,
    c: c_uchar,
    ev: *mut snd_seq_event,
) -> bool {
    let mut rc = false;

    if c >= MIDI_CMD_COMMON_CLOCK {
        /* real-time event */
        (*ev).type_ = STATUS_EVENT[ST_SPECIAL + c as usize - 0xf0].event as c_uchar;
        (*ev).flags &= !SNDRV_SEQ_EVENT_LENGTH_MASK;
        (*ev).flags |= SNDRV_SEQ_EVENT_LENGTH_FIXED;
        return (*ev).type_ as c_int != SNDRV_SEQ_EVENT_NONE;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*dev).lock, &mut flags);
    if (c & 0x80) != 0 && (c != MIDI_CMD_COMMON_SYSEX_END || (*dev).type_ != ST_SYSEX as c_int) {
        /* new command */
        *(*dev).buf.add(0) = c;
        if (c & 0xf0) == 0xf0 {
            /* system messages */
            (*dev).type_ = ((c & 0x0f) as usize + ST_SPECIAL) as c_int;
        } else {
            (*dev).type_ = ((c >> 4) & 0x07) as c_int;
        }
        (*dev).read = 1;
        (*dev).qlen = STATUS_EVENT[(*dev).type_ as usize].qlen;
    } else if (*dev).qlen > 0 {
        /* rest of command */
        *(*dev).buf.add((*dev).read as usize) = c;
        (*dev).read += 1;
        if (*dev).type_ != ST_SYSEX as c_int {
            (*dev).qlen -= 1;
        }
    } else {
        /* running status */
        *(*dev).buf.add(1) = c;
        (*dev).qlen = STATUS_EVENT[(*dev).type_ as usize].qlen - 1;
        (*dev).read = 2;
    }

    if (*dev).qlen == 0 {
        (*ev).type_ = STATUS_EVENT[(*dev).type_ as usize].event as c_uchar;
        (*ev).flags &= !SNDRV_SEQ_EVENT_LENGTH_MASK;
        (*ev).flags |= SNDRV_SEQ_EVENT_LENGTH_FIXED;
        if let Some(encode) = STATUS_EVENT[(*dev).type_ as usize].encode {
            /* set data values */
            encode(dev, ev);
        }
        if (*dev).type_ >= ST_SPECIAL as c_int {
            (*dev).type_ = ST_INVALID as c_int;
        }
        rc = true;
    } else if (*dev).type_ == ST_SYSEX as c_int {
        if c == MIDI_CMD_COMMON_SYSEX_END || (*dev).read >= (*dev).bufsize {
            (*ev).flags &= !SNDRV_SEQ_EVENT_LENGTH_MASK;
            (*ev).flags |= SNDRV_SEQ_EVENT_LENGTH_VARIABLE;
            (*ev).type_ = SNDRV_SEQ_EVENT_SYSEX as c_uchar;
            (*ev).data.ext.len = (*dev).read as c_uint;
            (*ev).data.ext.ptr = (*dev).buf as *mut c_void;
            if c != MIDI_CMD_COMMON_SYSEX_END {
                (*dev).read = 0; /* continue to parse */
            } else {
                reset_encode(dev); /* all parsed */
            }
            rc = true;
        }
    }

    spin_unlock_irqrestore(&mut (*dev).lock, flags);
    rc
}

/* encode note event */
unsafe fn note_event(dev: *mut snd_midi_event, ev: *mut snd_seq_event) {
    (*ev).data.note.channel = *(*dev).buf.add(0) & 0x0f;
    (*ev).data.note.note = *(*dev).buf.add(1);
    (*ev).data.note.velocity = *(*dev).buf.add(2);
}

/* encode one parameter controls */
unsafe fn one_param_ctrl_event(dev: *mut snd_midi_event, ev: *mut snd_seq_event) {
    (*ev).data.control.channel = *(*dev).buf.add(0) & 0x0f;
    (*ev).data.control.value = *(*dev).buf.add(1) as c_int;
}

/* encode pitch wheel change */
unsafe fn pitchbend_ctrl_event(dev: *mut snd_midi_event, ev: *mut snd_seq_event) {
    (*ev).data.control.channel = *(*dev).buf.add(0) & 0x0f;
    (*ev).data.control.value = (*(*dev).buf.add(2) as c_int) * 128 + (*(*dev).buf.add(1) as c_int) - 8192;
}

/* encode midi control change */
unsafe fn two_param_ctrl_event(dev: *mut snd_midi_event, ev: *mut snd_seq_event) {
    (*ev).data.control.channel = *(*dev).buf.add(0) & 0x0f;
    (*ev).data.control.param = *(*dev).buf.add(1) as c_uint;
    (*ev).data.control.value = *(*dev).buf.add(2) as c_int;
}

/* encode one parameter value*/
unsafe fn one_param_event(dev: *mut snd_midi_event, ev: *mut snd_seq_event) {
    (*ev).data.control.value = *(*dev).buf.add(1) as c_int;
}

/* encode song position */
unsafe fn songpos_event(dev: *mut snd_midi_event, ev: *mut snd_seq_event) {
    (*ev).data.control.value = (*(*dev).buf.add(2) as c_int) * 128 + (*(*dev).buf.add(1) as c_int);
}

/*
 * decode from a sequencer event to midi bytes
 * return the size of decoded midi events
 */
#[no_mangle]
pub unsafe extern "C" fn snd_midi_event_decode(
    dev: *mut snd_midi_event,
    buf: *mut c_uchar,
    count: c_long,
    ev: *mut snd_seq_event,
) -> c_long {
    let mut cmd: c_uint;
    let mut type_: usize;

    if (*ev).type_ as c_int == SNDRV_SEQ_EVENT_NONE {
        return -ENOENT as c_long;
    }

    type_ = 0;
    while type_ < STATUS_EVENT.len() {
        if (*ev).type_ as c_int == STATUS_EVENT[type_].event {
            break;
        }
        type_ += 1;
    }
    if type_ == STATUS_EVENT.len() {
        type_ = 0;
        while type_ < EXTRA_EVENT.len() {
            if (*ev).type_ as c_int == EXTRA_EVENT[type_].event {
                return (EXTRA_EVENT[type_].decode)(dev, buf, count as c_int, ev) as c_long;
            }
            type_ += 1;
        }
        return -ENOENT as c_long;
    }

    if type_ >= ST_SPECIAL {
        cmd = 0xf0 + (type_ - ST_SPECIAL) as c_uint;
    } else {
        /* data.note.channel and data.control.channel is identical */
        cmd = 0x80 | ((type_ as c_uint) << 4) | ((*ev).data.note.channel as c_uint & 0x0f);
    }

    if cmd as c_uchar == MIDI_CMD_COMMON_SYSEX {
        snd_midi_event_reset_decode(dev);
        snd_seq_expand_var_event(ev, count, buf, 1, 0)
    } else {
        let qlen: c_int;
        let mut xbuf: [c_uchar; 4] = [0; 4];
        let mut flags: c_ulong = 0;

        spin_lock_irqsave(&mut (*dev).lock, &mut flags);
        if (cmd & 0xf0) == 0xf0 || (*dev).lastcmd as c_uint != cmd || (*dev).nostat != 0 {
            (*dev).lastcmd = cmd as c_uchar;
            spin_unlock_irqrestore(&mut (*dev).lock, flags);
            xbuf[0] = cmd as c_uchar;
            if let Some(decode) = STATUS_EVENT[type_].decode {
                decode(ev, xbuf.as_mut_ptr().add(1));
            }
            qlen = STATUS_EVENT[type_].qlen + 1;
        } else {
            spin_unlock_irqrestore(&mut (*dev).lock, flags);
            if let Some(decode) = STATUS_EVENT[type_].decode {
                decode(ev, xbuf.as_mut_ptr().add(0));
            }
            qlen = STATUS_EVENT[type_].qlen;
        }
        if count < qlen as c_long {
            return -ENOMEM as c_long;
        }
        memcpy(buf as *mut c_void, xbuf.as_ptr() as *const c_void, qlen as usize);
        qlen as c_long
    }
}

/* decode note event */
unsafe fn note_decode(ev: *mut snd_seq_event, buf: *mut c_uchar) {
    *buf.add(0) = (*ev).data.note.note & 0x7f;
    *buf.add(1) = (*ev).data.note.velocity & 0x7f;
}

/* decode one parameter controls */
unsafe fn one_param_decode(ev: *mut snd_seq_event, buf: *mut c_uchar) {
    *buf.add(0) = ((*ev).data.control.value & 0x7f) as c_uchar;
}

/* decode pitch wheel change */
unsafe fn pitchbend_decode(ev: *mut snd_seq_event, buf: *mut c_uchar) {
    let value: c_int = (*ev).data.control.value + 8192;
    *buf.add(0) = (value & 0x7f) as c_uchar;
    *buf.add(1) = ((value >> 7) & 0x7f) as c_uchar;
}

/* decode midi control change */
unsafe fn two_param_decode(ev: *mut snd_seq_event, buf: *mut c_uchar) {
    *buf.add(0) = ((*ev).data.control.param & 0x7f) as c_uchar;
    *buf.add(1) = ((*ev).data.control.value & 0x7f) as c_uchar;
}

/* decode song position */
unsafe fn songpos_decode(ev: *mut snd_seq_event, buf: *mut c_uchar) {
    *buf.add(0) = ((*ev).data.control.value & 0x7f) as c_uchar;
    *buf.add(1) = (((*ev).data.control.value >> 7) & 0x7f) as c_uchar;
}

/* decode 14bit control */
unsafe fn extra_decode_ctrl14(
    dev: *mut snd_midi_event,
    buf: *mut c_uchar,
    count: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    let cmd: c_uchar;
    let mut idx: c_int = 0;

    cmd = MIDI_CMD_CONTROL | ((*ev).data.control.channel & 0x0f);
    if (*ev).data.control.param < 0x20 {
        if count < 4 {
            return -ENOMEM;
        }
        if (*dev).nostat != 0 && count < 6 {
            return -ENOMEM;
        }
        if cmd != (*dev).lastcmd || (*dev).nostat != 0 {
            if count < 5 {
                return -ENOMEM;
            }
            (*dev).lastcmd = cmd;
            *buf.add(idx as usize) = (*dev).lastcmd;
            idx += 1;
        }
        *buf.add(idx as usize) = (*ev).data.control.param as c_uchar;
        idx += 1;
        *buf.add(idx as usize) = (((*ev).data.control.value >> 7) & 0x7f) as c_uchar;
        idx += 1;
        if (*dev).nostat != 0 {
            *buf.add(idx as usize) = cmd;
            idx += 1;
        }
        *buf.add(idx as usize) = ((*ev).data.control.param + 0x20) as c_uchar;
        idx += 1;
        *buf.add(idx as usize) = ((*ev).data.control.value & 0x7f) as c_uchar;
        idx += 1;
    } else {
        if count < 2 {
            return -ENOMEM;
        }
        if cmd != (*dev).lastcmd || (*dev).nostat != 0 {
            if count < 3 {
                return -ENOMEM;
            }
            (*dev).lastcmd = cmd;
            *buf.add(idx as usize) = (*dev).lastcmd;
            idx += 1;
        }
        *buf.add(idx as usize) = ((*ev).data.control.param & 0x7f) as c_uchar;
        idx += 1;
        *buf.add(idx as usize) = ((*ev).data.control.value & 0x7f) as c_uchar;
        idx += 1;
    }
    idx
}

/* decode reg/nonreg param */
unsafe fn extra_decode_xrpn(
    dev: *mut snd_midi_event,
    buf: *mut c_uchar,
    count: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    let cmd: c_uchar;
    let cbytes: *const c_uchar;
    static CBYTES_NRPN: [c_uchar; 4] = [
        MIDI_CTL_NONREG_PARM_NUM_MSB,
        MIDI_CTL_NONREG_PARM_NUM_LSB,
        MIDI_CTL_MSB_DATA_ENTRY,
        MIDI_CTL_LSB_DATA_ENTRY,
    ];
    static CBYTES_RPN: [c_uchar; 4] = [
        MIDI_CTL_REGIST_PARM_NUM_MSB,
        MIDI_CTL_REGIST_PARM_NUM_LSB,
        MIDI_CTL_MSB_DATA_ENTRY,
        MIDI_CTL_LSB_DATA_ENTRY,
    ];
    let mut bytes: [c_uchar; 4] = [0; 4];
    let mut idx: c_int = 0;
    let mut i: c_int;

    if count < 8 {
        return -ENOMEM;
    }
    if (*dev).nostat != 0 && count < 12 {
        return -ENOMEM;
    }
    cmd = MIDI_CMD_CONTROL | ((*ev).data.control.channel & 0x0f);
    bytes[0] = (((*ev).data.control.param & 0x3f80) >> 7) as c_uchar;
    bytes[1] = ((*ev).data.control.param & 0x007f) as c_uchar;
    bytes[2] = (((*ev).data.control.value & 0x3f80) >> 7) as c_uchar;
    bytes[3] = ((*ev).data.control.value & 0x007f) as c_uchar;
    if cmd != (*dev).lastcmd && (*dev).nostat == 0 {
        if count < 9 {
            return -ENOMEM;
        }
        (*dev).lastcmd = cmd;
        *buf.add(idx as usize) = (*dev).lastcmd;
        idx += 1;
    }
    cbytes = if (*ev).type_ as c_int == SNDRV_SEQ_EVENT_NONREGPARAM {
        CBYTES_NRPN.as_ptr()
    } else {
        CBYTES_RPN.as_ptr()
    };
    i = 0;
    while i < 4 {
        if (*dev).nostat != 0 {
            (*dev).lastcmd = cmd;
            *buf.add(idx as usize) = (*dev).lastcmd;
            idx += 1;
        }
        *buf.add(idx as usize) = *cbytes.add(i as usize);
        idx += 1;
        *buf.add(idx as usize) = bytes[i as usize];
        idx += 1;
        i += 1;
    }
    idx
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
