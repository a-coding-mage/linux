// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * read/write/select interface to device file
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies from:
 * "seq_oss_device.h"
 * "seq_oss_readq.h"
 * "seq_oss_writeq.h"
 * "seq_oss_synth.h"
 * <sound/seq_oss_legacy.h>
 * "seq_oss_event.h"
 * "seq_oss_timer.h"
 * "../seq_clientmgr.h"
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub const SHORT_EVENT_SIZE: c_int = 4;
pub const LONG_EVENT_SIZE: c_int = 8;
pub const SEQ_FULLSIZE: c_uchar = 0xfd;
pub const SEQ_EXTENDED: c_uchar = 0xfe;
pub const SNDRV_SEQ_OSS_MODE_MUSIC: c_int = 1;
pub const SNDRV_SEQ_EVENT_NOTEOFF: c_uchar = 7;

pub const ENXIO: c_int = 6;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EFAULT: c_int = 14;
pub const EINVAL: c_int = 22;
pub const ERESTARTSYS: c_int = 512;

pub type PollT = c_uint;
pub const EPOLLIN: PollT = 0x00000001;
pub const EPOLLOUT: PollT = 0x00000004;
pub const EPOLLRDNORM: PollT = 0x00000040;
pub const EPOLLWRNORM: PollT = 0x00000100;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_oss_readq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_oss_writeq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_client {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_use_lock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_oss_timer {
    pub realtime: c_int,
    pub running: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_oss_addr {
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
pub struct seq_oss_devinfo {
    pub readq: *mut seq_oss_readq,
    pub writeq: *mut seq_oss_writeq,
    pub file_mode: c_int,
    pub seq_mode: c_int,
    pub timer: *mut seq_oss_timer,
    pub addr: seq_oss_addr,
    pub cseq: *mut snd_seq_client,
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
pub union evrec {
    pub s: evrec_short,
    pub c: [c_uchar; LONG_EVENT_SIZE as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_real_time {
    pub tv_sec: c_uint,
    pub tv_nsec: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_seq_timestamp {
    pub tick: c_uint,
    pub time: snd_seq_real_time,
}

#[repr(C)]
pub struct snd_seq_event {
    pub type_: c_uchar,
    pub flags: c_uchar,
    pub tag: c_uchar,
    pub queue: c_uchar,
    pub time: snd_seq_timestamp,
    pub source: seq_oss_addr,
    pub dest: seq_oss_addr,
    pub data: [c_uchar; 12],
}

unsafe extern "C" {
    static mut current: *mut task_struct;

    fn is_read_mode(mode: c_int) -> c_int;
    fn is_write_mode(mode: c_int) -> c_int;
    fn is_nonblock_mode(mode: c_int) -> c_int;
    fn ev_length(rec: *const evrec) -> c_int;
    fn ev_is_long(rec: *const evrec) -> c_int;

    fn snd_seq_oss_readq_lock(readq: *mut seq_oss_readq, flags: *mut c_ulong);
    fn snd_seq_oss_readq_unlock(readq: *mut seq_oss_readq, flags: c_ulong);
    fn snd_seq_oss_readq_pick(readq: *mut seq_oss_readq, rec: *mut evrec) -> c_int;
    fn snd_seq_oss_readq_wait(readq: *mut seq_oss_readq);
    fn snd_seq_oss_readq_free(readq: *mut seq_oss_readq);
    fn snd_seq_oss_readq_poll(
        readq: *mut seq_oss_readq,
        file: *mut file,
        wait: *mut poll_table,
    ) -> c_int;

    fn signal_pending(task: *mut task_struct) -> c_int;
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: c_ulong) -> c_ulong;
    fn copy_from_user(to: *mut c_void, from: *const c_char, n: c_ulong) -> c_ulong;

    fn snd_seq_oss_synth_load_patch(
        dp: *mut seq_oss_devinfo,
        dev: c_uchar,
        fmt: c_int,
        buf: *const c_char,
        offs: c_int,
        count: c_int,
    ) -> c_int;

    fn snd_seq_oss_process_timer_event(timer: *mut seq_oss_timer, rec: *mut evrec) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: c_ulong) -> *mut c_void;
    fn snd_seq_oss_fill_addr(
        dp: *mut seq_oss_devinfo,
        event: *mut snd_seq_event,
        client: c_int,
        port: c_int,
    );
    fn snd_seq_oss_process_event(
        dp: *mut seq_oss_devinfo,
        rec: *mut evrec,
        event: *mut snd_seq_event,
        lock: *mut *mut snd_use_lock_t,
    ) -> c_int;
    fn snd_seq_oss_timer_cur_tick(timer: *mut seq_oss_timer) -> c_uint;
    fn snd_seq_oss_dispatch(
        dp: *mut seq_oss_devinfo,
        event: *mut snd_seq_event,
        atomic: c_int,
        hop: c_int,
    );
    fn snd_seq_kernel_client_enqueue(
        cseq: *mut snd_seq_client,
        event: *mut snd_seq_event,
        file: *mut file,
        blocking: c_int,
    ) -> c_int;
    fn snd_seq_kernel_client_write_poll(
        cseq: *mut snd_seq_client,
        file: *mut file,
        wait: *mut poll_table,
    ) -> c_int;
}

/*
 * prototypes
 */
unsafe fn insert_queue(
    dp: *mut seq_oss_devinfo,
    rec: *mut evrec,
    opt: *mut file,
) -> c_int;

/*
 * read interface
 */

#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_read(
    dp: *mut seq_oss_devinfo,
    mut buf: *mut c_char,
    mut count: c_int,
) -> c_int {
    let readq: *mut seq_oss_readq = unsafe { (*dp).readq };
    let mut result: c_int = 0;
    let mut err: c_int = 0;
    let mut ev_len: c_int;
    let mut rec: evrec = unsafe { core::mem::zeroed() };
    let mut flags: c_ulong = 0;

    if readq.is_null() || unsafe { is_read_mode((*dp).file_mode) } == 0 {
        return -ENXIO;
    }

    while count >= SHORT_EVENT_SIZE {
        unsafe { snd_seq_oss_readq_lock(readq, &mut flags) };
        err = unsafe { snd_seq_oss_readq_pick(readq, &mut rec) };
        if err == -EAGAIN
            && unsafe { is_nonblock_mode((*dp).file_mode) } == 0
            && result == 0
        {
            unsafe { snd_seq_oss_readq_unlock(readq, flags) };
            unsafe { snd_seq_oss_readq_wait(readq) };
            unsafe { snd_seq_oss_readq_lock(readq, &mut flags) };
            if unsafe { signal_pending(current) } != 0 {
                err = -ERESTARTSYS;
            } else {
                err = unsafe { snd_seq_oss_readq_pick(readq, &mut rec) };
            }
        }
        if err < 0 {
            unsafe { snd_seq_oss_readq_unlock(readq, flags) };
            break;
        }
        ev_len = unsafe { ev_length(&rec) };
        if count < ev_len {
            err = -EINVAL;
            unsafe { snd_seq_oss_readq_unlock(readq, flags) };
            break;
        }
        unsafe { snd_seq_oss_readq_free(readq) };
        unsafe { snd_seq_oss_readq_unlock(readq, flags) };
        if unsafe { copy_to_user(buf, &rec as *const _ as *const c_void, ev_len as c_ulong) } != 0 {
            err = -EFAULT;
            break;
        }
        result += ev_len;
        buf = unsafe { buf.add(ev_len as usize) };
        count -= ev_len;
    }
    if result > 0 { result } else { err }
}

/*
 * write interface
 */

#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_write(
    dp: *mut seq_oss_devinfo,
    mut buf: *const c_char,
    mut count: c_int,
    opt: *mut file,
) -> c_int {
    let mut result: c_int = 0;
    let mut err: c_int = 0;
    let mut ev_size: c_int;
    let mut fmt: c_int;
    let mut rec: evrec = unsafe { core::mem::zeroed() };

    if unsafe { is_write_mode((*dp).file_mode) } == 0 || unsafe { (*dp).writeq }.is_null() {
        return -ENXIO;
    }

    while count >= SHORT_EVENT_SIZE {
        if unsafe {
            copy_from_user(
                &mut rec as *mut _ as *mut c_void,
                buf,
                SHORT_EVENT_SIZE as c_ulong,
            )
        } != 0
        {
            err = -EFAULT;
            break;
        }
        if unsafe { rec.s.code } == SEQ_FULLSIZE {
            /* load patch */
            if result > 0 {
                err = -EINVAL;
                break;
            }
            fmt = unsafe { ptr::read_unaligned(rec.c.as_ptr() as *const u16) as c_int } & 0xffff;
            err = unsafe { snd_seq_oss_synth_load_patch(dp, rec.s.dev, fmt, buf, 0, count) };
            return if err < 0 { err } else { count };
        }
        if unsafe { ev_is_long(&rec) } != 0 {
            /* extended code */
            if unsafe { rec.s.code } == SEQ_EXTENDED
                && unsafe { (*dp).seq_mode } == SNDRV_SEQ_OSS_MODE_MUSIC
            {
                err = -EINVAL;
                break;
            }
            ev_size = LONG_EVENT_SIZE;
            if count < ev_size {
                break;
            }
            /* copy the reset 4 bytes */
            if unsafe {
                copy_from_user(
                    rec.c.as_mut_ptr().add(SHORT_EVENT_SIZE as usize) as *mut c_void,
                    buf.add(SHORT_EVENT_SIZE as usize),
                    (LONG_EVENT_SIZE - SHORT_EVENT_SIZE) as c_ulong,
                )
            } != 0
            {
                err = -EFAULT;
                break;
            }
        } else {
            /* old-type code */
            if unsafe { (*dp).seq_mode } == SNDRV_SEQ_OSS_MODE_MUSIC {
                err = -EINVAL;
                break;
            }
            ev_size = SHORT_EVENT_SIZE;
        }

        /* insert queue */
        err = unsafe { insert_queue(dp, &mut rec, opt) };
        if err < 0 {
            break;
        }

        result += ev_size;
        buf = unsafe { buf.add(ev_size as usize) };
        count -= ev_size;
    }
    if result > 0 { result } else { err }
}

/*
 * insert event record to write queue
 * return: 0 = OK, non-zero = NG
 */
unsafe fn insert_queue(
    dp: *mut seq_oss_devinfo,
    rec: *mut evrec,
    opt: *mut file,
) -> c_int {
    let mut rc: c_int = 0;
    let mut event: snd_seq_event = unsafe { core::mem::zeroed() };

    /* if this is a timing event, process the current time */
    if unsafe { snd_seq_oss_process_timer_event((*dp).timer, rec) } != 0 {
        return 0; /* no need to insert queue */
    }

    /* parse this event */
    unsafe {
        memset(
            &mut event as *mut _ as *mut c_void,
            0,
            size_of::<snd_seq_event>() as c_ulong,
        );
    }
    /* set dummy -- to be sure */
    event.type_ = SNDRV_SEQ_EVENT_NOTEOFF;
    unsafe { snd_seq_oss_fill_addr(dp, &mut event, (*dp).addr.client, (*dp).addr.port) };

    /*
     * C used:
     * snd_use_lock_t *lock __free(seq_oss_use_lock) = NULL;
     * Keep the local lock pointer and pass its address to the event processor.
     */
    let mut lock: *mut snd_use_lock_t = ptr::null_mut();

    if unsafe { snd_seq_oss_process_event(dp, rec, &mut event, &mut lock) } != 0 {
        return 0; /* invalid event - no need to insert queue */
    }

    event.time.tick = unsafe { snd_seq_oss_timer_cur_tick((*dp).timer) };
    if unsafe { (*(*dp).timer).realtime } != 0 || unsafe { (*(*dp).timer).running } == 0 {
        unsafe { snd_seq_oss_dispatch(dp, &mut event, 0, 0) };
    } else {
        rc = unsafe {
            snd_seq_kernel_client_enqueue(
                (*dp).cseq,
                &mut event,
                opt,
                (is_nonblock_mode((*dp).file_mode) == 0) as c_int,
            )
        };
    }
    rc
}

/*
 * select / poll
 */

#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_poll(
    dp: *mut seq_oss_devinfo,
    file: *mut file,
    wait: *mut poll_table,
) -> PollT {
    let mut mask: PollT = 0;

    /* input */
    if unsafe { !(*dp).readq.is_null() && is_read_mode((*dp).file_mode) != 0 } {
        if unsafe { snd_seq_oss_readq_poll((*dp).readq, file, wait) } != 0 {
            mask |= EPOLLIN | EPOLLRDNORM;
        }
    }

    /* output */
    if unsafe { !(*dp).writeq.is_null() && is_write_mode((*dp).file_mode) != 0 } {
        if unsafe { snd_seq_kernel_client_write_poll((*dp).cseq, file, wait) } != 0 {
            mask |= EPOLLOUT | EPOLLWRNORM;
        }
    }
    mask
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
