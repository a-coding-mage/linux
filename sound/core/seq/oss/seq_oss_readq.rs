// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * seq_oss_readq.c - MIDI input queue
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies from:
 * "seq_oss_readq.h"
 * "seq_oss_event.h"
 * <sound/seq_oss_legacy.h>
 * "../seq_lock.h"
 * <linux/wait.h>
 * <linux/slab.h>
 */

/*
 * constants
 */
// #define SNDRV_SEQ_OSS_MAX_TIMEOUT	(unsigned long)(-1)
pub const SNDRV_SEQ_OSS_MAX_TIMEOUT: c_ulong = HZ * 3600;

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type c_uchar = u8;
pub type c_void = core::ffi::c_void;

pub const ENOMEM: c_int = 12;
pub const EAGAIN: c_int = 11;

extern "C" {
    pub static HZ: c_ulong;
    pub static SEQ_MIDIPUTC: c_uchar;
    pub static SEQ_WAIT: c_ulong;
    pub static SNDRV_SEQ_EVENT_LENGTH_MASK: c_uint;
    pub static SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_uint;
    pub static SNDRV_SEQ_OSS_MODE_SYNTH: c_int;
    pub static SNDRV_SEQ_OSS_MODE_MUSIC: c_int;
    pub static EV_TIMING: c_uchar;
    pub static TMR_WAIT_ABS: c_uchar;

    pub fn kzalloc_flex_readq(maxlen: c_int) -> *mut seq_oss_readq;
    pub fn kfree(ptr: *mut c_void);
    pub fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    pub fn spin_lock_init(lock: *mut spinlock_t);
    pub fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    pub fn wake_up(wq: *mut wait_queue_head_t);
    pub fn wait_event_interruptible_timeout(
        wq: wait_queue_head_t,
        condition: bool,
        timeout: c_ulong,
    ) -> c_int;
    pub fn poll_wait(file: *mut file, wq: *mut wait_queue_head_t, wait: *mut poll_table);
    pub fn snd_seq_dump_var_event(
        ev: *mut snd_seq_event,
        dump: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int>,
        private_data: *mut c_void,
    ) -> c_int;

    /*
     * proc interface dependency.
     */
    pub fn waitqueue_active(wq: *mut wait_queue_head_t) -> c_int;
    pub fn snd_iprintf(buf: *mut snd_info_buffer, fmt: *const u8, ...);
}

#[repr(C)]
pub struct seq_oss_devinfo {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_event {
    pub flags: c_uint,
}

#[repr(C)]
pub struct file {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing_event {
    pub code: c_uchar,
    pub cmd: c_uchar,
    pub time: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union evrec {
    pub c: [c_uchar; 8],
    pub echo: c_ulong,
    pub t: timing_event,
}

#[repr(C)]
pub struct seq_oss_readq {
    pub maxlen: c_int,
    pub qlen: c_int,
    pub head: c_int,
    pub tail: c_int,
    pub midi_sleep: wait_queue_head_t,
    pub lock: spinlock_t,
    pub pre_event_timeout: c_ulong,
    pub input_time: c_ulong,
    pub q: [evrec; 0],
}

/*
 * prototypes
 */

/*
 * create a read queue
 */
pub unsafe extern "C" fn snd_seq_oss_readq_new(
    _dp: *mut seq_oss_devinfo,
    maxlen: c_int,
) -> *mut seq_oss_readq {
    let q: *mut seq_oss_readq = kzalloc_flex_readq(maxlen);
    if q.is_null() {
        return core::ptr::null_mut();
    }

    (*q).maxlen = maxlen;
    (*q).qlen = 0;
    (*q).head = 0;
    (*q).tail = 0;
    init_waitqueue_head(&mut (*q).midi_sleep);
    spin_lock_init(&mut (*q).lock);
    (*q).pre_event_timeout = SNDRV_SEQ_OSS_MAX_TIMEOUT;
    (*q).input_time = -1isize as c_ulong;

    q
}

/*
 * delete the read queue
 */
pub unsafe extern "C" fn snd_seq_oss_readq_delete(q: *mut seq_oss_readq) {
    kfree(q as *mut c_void);
}

/*
 * reset the read queue
 */
pub unsafe extern "C" fn snd_seq_oss_readq_clear(q: *mut seq_oss_readq) {
    let flags = spin_lock_irqsave(&mut (*q).lock);
    if (*q).qlen != 0 {
        (*q).qlen = 0;
        (*q).head = 0;
        (*q).tail = 0;
    }
    (*q).input_time = -1isize as c_ulong;
    spin_unlock_irqrestore(&mut (*q).lock, flags);

    /* if someone sleeping, wake'em up */
    wake_up(&mut (*q).midi_sleep);
}

/*
 * put a midi byte
 */
pub unsafe extern "C" fn snd_seq_oss_readq_puts(
    q: *mut seq_oss_readq,
    dev: c_int,
    mut data: *mut c_uchar,
    mut len: c_int,
) -> c_int {
    let mut rec: evrec = core::mem::zeroed();
    let mut result: c_int;

    rec.c[0] = SEQ_MIDIPUTC;
    rec.c[2] = dev as c_uchar;

    while {
        let old = len;
        len -= 1;
        old > 0
    } {
        rec.c[1] = *data;
        data = data.add(1);
        result = snd_seq_oss_readq_put_event(q, &mut rec);
        if result < 0 {
            return result;
        }
    }
    0
}

/*
 * put MIDI sysex bytes; the event buffer may be chained, thus it has
 * to be expanded via snd_seq_dump_var_event().
 */
#[repr(C)]
pub struct readq_sysex_ctx {
    pub readq: *mut seq_oss_readq,
    pub dev: c_int,
}

unsafe extern "C" fn readq_dump_sysex(ptr: *mut c_void, buf: *mut c_void, count: c_int) -> c_int {
    let ctx: *mut readq_sysex_ctx = ptr as *mut readq_sysex_ctx;

    snd_seq_oss_readq_puts((*ctx).readq, (*ctx).dev, buf as *mut c_uchar, count)
}

pub unsafe extern "C" fn snd_seq_oss_readq_sysex(
    q: *mut seq_oss_readq,
    dev: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    let mut ctx = readq_sysex_ctx { readq: q, dev };

    if ((*ev).flags & SNDRV_SEQ_EVENT_LENGTH_MASK) != SNDRV_SEQ_EVENT_LENGTH_VARIABLE {
        return 0;
    }
    snd_seq_dump_var_event(
        ev,
        Some(readq_dump_sysex),
        &mut ctx as *mut readq_sysex_ctx as *mut c_void,
    )
}

/*
 * copy an event to input queue:
 * return zero if enqueued
 * caller must hold lock
 */
unsafe extern "C" fn snd_seq_oss_readq_put_event_locked(
    q: *mut seq_oss_readq,
    ev: *mut evrec,
) -> c_int {
    if (*q).qlen >= (*q).maxlen - 1 {
        return -ENOMEM;
    }

    core::ptr::copy_nonoverlapping(
        ev,
        ((*q).q.as_mut_ptr()).add((*q).tail as usize),
        1,
    );
    (*q).tail = ((*q).tail + 1) % (*q).maxlen;
    (*q).qlen += 1;

    0
}

/*
 * copy an event to input queue:
 * return zero if enqueued
 */
pub unsafe extern "C" fn snd_seq_oss_readq_put_event(
    q: *mut seq_oss_readq,
    ev: *mut evrec,
) -> c_int {
    let rc: c_int;

    let flags = spin_lock_irqsave(&mut (*q).lock);
    rc = snd_seq_oss_readq_put_event_locked(q, ev);
    if rc == 0 {
        wake_up(&mut (*q).midi_sleep);
    }
    spin_unlock_irqrestore(&mut (*q).lock, flags);

    rc
}

/*
 * pop queue
 * caller must hold lock
 */
pub unsafe extern "C" fn snd_seq_oss_readq_pick(q: *mut seq_oss_readq, rec: *mut evrec) -> c_int {
    if (*q).qlen == 0 {
        return -EAGAIN;
    }
    core::ptr::copy_nonoverlapping(((*q).q.as_ptr()).add((*q).head as usize), rec, 1);
    0
}

/*
 * sleep until ready
 */
pub unsafe extern "C" fn snd_seq_oss_readq_wait(q: *mut seq_oss_readq) {
    wait_event_interruptible_timeout(
        core::ptr::read(&(*q).midi_sleep),
        (*q).qlen > 0 || (*q).head == (*q).tail,
        (*q).pre_event_timeout,
    );
}

/*
 * drain one record
 * caller must hold lock
 */
pub unsafe extern "C" fn snd_seq_oss_readq_free(q: *mut seq_oss_readq) {
    if (*q).qlen > 0 {
        (*q).head = ((*q).head + 1) % (*q).maxlen;
        (*q).qlen -= 1;
    }
}

/*
 * polling/select:
 * return non-zero if readq is not empty.
 */
pub unsafe extern "C" fn snd_seq_oss_readq_poll(
    q: *mut seq_oss_readq,
    file: *mut file,
    wait: *mut poll_table,
) -> c_uint {
    poll_wait(file, &mut (*q).midi_sleep, wait);
    (*q).qlen as c_uint
}

/*
 * put a timestamp
 */
pub unsafe extern "C" fn snd_seq_oss_readq_put_timestamp(
    q: *mut seq_oss_readq,
    curt: c_ulong,
    seq_mode: c_int,
) -> c_int {
    let mut queued: c_int = 0;

    let flags = spin_lock_irqsave(&mut (*q).lock);
    if curt != (*q).input_time {
        let mut rec: evrec = core::mem::zeroed();

        if seq_mode == SNDRV_SEQ_OSS_MODE_SYNTH {
            rec.echo = (curt << 8) | SEQ_WAIT;
            queued = (snd_seq_oss_readq_put_event_locked(q, &mut rec) == 0) as c_int;
        } else if seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
            rec.t.code = EV_TIMING;
            rec.t.cmd = TMR_WAIT_ABS;
            rec.t.time = curt;
            queued = (snd_seq_oss_readq_put_event_locked(q, &mut rec) == 0) as c_int;
        }
        (*q).input_time = curt;
    }
    spin_unlock_irqrestore(&mut (*q).lock, flags);
    if queued != 0 {
        wake_up(&mut (*q).midi_sleep);
    }

    0
}

/*
 * CONFIG_SND_PROC_FS conditional code from the C source.
 */
/*
 * proc interface
 */
pub unsafe extern "C" fn snd_seq_oss_readq_info_read(
    q: *mut seq_oss_readq,
    buf: *mut snd_info_buffer,
) {
    let state = if waitqueue_active(&mut (*q).midi_sleep) != 0 {
        b"sleeping\0".as_ptr()
    } else {
        b"running\0".as_ptr()
    };
    snd_iprintf(
        buf,
        b"  read queue [%s] length = %d : tick = %ld\n\0".as_ptr(),
        state,
        (*q).qlen,
        (*q).input_time,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
