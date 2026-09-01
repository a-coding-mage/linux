// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA sequencer Priority Queue
 *   Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

// C includes translated as external dependencies:
// <linux/time.h>, <linux/slab.h>, <sound/core.h>, "seq_timer.h", "seq_prioq.h"

use core::ffi::c_void;
use core::ptr;

pub type c_int = i32;
pub type c_uint = u32;
pub type c_uchar = u8;
pub type c_long = i64;

pub const EINVAL: c_int = 22;

extern "C" {
    pub static SNDRV_SEQ_TIME_STAMP_MASK: c_int;
    pub static SNDRV_SEQ_TIME_STAMP_TICK: c_int;
    pub static SNDRV_SEQ_TIME_STAMP_REAL: c_int;
    pub static SNDRV_SEQ_PRIORITY_MASK: c_int;
    pub static SNDRV_SEQ_REMOVE_DEST: c_int;
    pub static SNDRV_SEQ_REMOVE_DEST_CHANNEL: c_int;
    pub static SNDRV_SEQ_REMOVE_TIME_AFTER: c_int;
    pub static SNDRV_SEQ_REMOVE_TIME_BEFORE: c_int;
    pub static SNDRV_SEQ_REMOVE_TIME_TICK: c_int;
    pub static SNDRV_SEQ_REMOVE_EVENT_TYPE: c_int;
    pub static SNDRV_SEQ_REMOVE_IGNORE_OFF: c_int;
    pub static SNDRV_SEQ_REMOVE_TAG_MATCH: c_int;
    pub static SNDRV_SEQ_EVENT_NOTEOFF: c_int;

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn snd_seq_cell_free(cell: *mut snd_seq_event_cell);
    fn snd_seq_compare_tick_time(a: *const snd_seq_tick_time_t, b: *const snd_seq_tick_time_t) -> c_int;
    fn snd_seq_compare_real_time(a: *const snd_seq_real_time_t, b: *const snd_seq_real_time_t) -> c_int;
    fn snd_seq_ev_is_channel_type(ev: *mut snd_seq_event) -> bool;
    fn pr_debug(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
}

pub type c_ulong = u64;
pub type snd_seq_tick_time_t = u32;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_real_time_t {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct snd_seq_addr {
    pub client: c_uchar,
    pub port: c_uchar,
}

#[repr(C)]
pub struct snd_seq_ev_note {
    pub channel: c_uchar,
}

#[repr(C)]
pub union snd_seq_event_data {
    pub note: snd_seq_ev_note,
}

#[repr(C)]
pub union snd_seq_timestamp {
    pub tick: snd_seq_tick_time_t,
    pub time: snd_seq_real_time_t,
}

#[repr(C)]
pub struct snd_seq_event {
    pub type_: c_uchar,
    pub flags: c_uchar,
    pub tag: c_uchar,
    pub queue: c_uchar,
    pub time: snd_seq_timestamp,
    pub source: snd_seq_addr,
    pub dest: snd_seq_addr,
    pub data: snd_seq_event_data,
}

#[repr(C)]
pub struct snd_seq_event_cell {
    pub next: *mut snd_seq_event_cell,
    pub event: snd_seq_event,
}

#[repr(C)]
pub struct snd_seq_prioq {
    pub lock: spinlock_t,
    pub head: *mut snd_seq_event_cell,
    pub tail: *mut snd_seq_event_cell,
    pub cells: c_int,
}

#[repr(C)]
pub struct snd_seq_remove_events {
    pub remove_mode: c_int,
    pub dest: snd_seq_addr,
    pub channel: c_uchar,
    pub time: snd_seq_timestamp,
    pub type_: c_uchar,
    pub tag: c_uchar,
}

unsafe fn snd_bug_on(cond: bool) -> bool {
    cond
}

unsafe fn lock_irqsave(lock: *mut spinlock_t) -> c_ulong {
    spin_lock_irqsave(lock)
}

unsafe fn unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong) {
    spin_unlock_irqrestore(lock, flags);
}

/* Implementation is a simple linked list for now...

   This priority queue orders the events on timestamp. For events with an
   equeal timestamp the queue behaves as a FIFO.

   *
   *           +-------+
   *  Head --> | first |
   *           +-------+
   *                 |next
   *           +-----v-+
   *           |       |
   *           +-------+
   *                 |
   *           +-----v-+
   *           |       |
   *           +-------+
   *                 |
   *           +-----v-+
   *  Tail --> | last  |
   *           +-------+
   *

 */

/* create new prioq (constructor) */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_prioq_new() -> *mut snd_seq_prioq {
    let f: *mut snd_seq_prioq;

    f = kzalloc(core::mem::size_of::<snd_seq_prioq>(), 0) as *mut snd_seq_prioq;
    if f.is_null() {
        return ptr::null_mut();
    }

    spin_lock_init(&mut (*f).lock);
    (*f).head = ptr::null_mut();
    (*f).tail = ptr::null_mut();
    (*f).cells = 0;

    f
}

/* delete prioq (destructor) */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_prioq_delete(fifo: *mut *mut snd_seq_prioq) {
    let f: *mut snd_seq_prioq = *fifo;
    *fifo = ptr::null_mut();

    if f.is_null() {
        pr_debug(b"ALSA: seq: snd_seq_prioq_delete() called with NULL prioq\n\0".as_ptr());
        return;
    }

    /* release resources...*/
    /*....................*/

    if (*f).cells > 0 {
        /* drain prioQ */
        while (*f).cells > 0 {
            snd_seq_cell_free(snd_seq_prioq_cell_out(f, ptr::null_mut()));
        }
    }

    kfree(f as *mut c_void);
}

/* compare timestamp between events */
/* return 1 if a >= b; 0 */
#[inline]
unsafe fn compare_timestamp(a: *mut snd_seq_event, b: *mut snd_seq_event) -> c_int {
    if (((*a).flags as c_int) & SNDRV_SEQ_TIME_STAMP_MASK) == SNDRV_SEQ_TIME_STAMP_TICK {
        /* compare ticks */
        snd_seq_compare_tick_time(&(*a).time.tick, &(*b).time.tick)
    } else {
        /* compare real time */
        snd_seq_compare_real_time(&(*a).time.time, &(*b).time.time)
    }
}

/* compare timestamp between events */
/* return negative if a < b;
 *        zero     if a = b;
 *        positive if a > b;
 */
#[inline]
unsafe fn compare_timestamp_rel(a: *mut snd_seq_event, b: *mut snd_seq_event) -> c_int {
    if (((*a).flags as c_int) & SNDRV_SEQ_TIME_STAMP_MASK) == SNDRV_SEQ_TIME_STAMP_TICK {
        /* compare ticks */
        if (*a).time.tick > (*b).time.tick {
            1
        } else if (*a).time.tick == (*b).time.tick {
            0
        } else {
            -1
        }
    } else {
        /* compare real time */
        if (*a).time.time.tv_sec > (*b).time.time.tv_sec {
            1
        } else if (*a).time.time.tv_sec == (*b).time.time.tv_sec {
            if (*a).time.time.tv_nsec > (*b).time.time.tv_nsec {
                1
            } else if (*a).time.time.tv_nsec == (*b).time.time.tv_nsec {
                0
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

/* enqueue cell to prioq */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_prioq_cell_in(
    f: *mut snd_seq_prioq,
    cell: *mut snd_seq_event_cell,
) -> c_int {
    let mut cur: *mut snd_seq_event_cell;
    let mut prev: *mut snd_seq_event_cell;
    let mut remaining: c_int;
    let prior: c_int;

    if snd_bug_on(f.is_null() || cell.is_null()) {
        return -EINVAL;
    }

    /* check flags */
    prior = ((*cell).event.flags as c_int) & SNDRV_SEQ_PRIORITY_MASK;

    let flags = lock_irqsave(&mut (*f).lock);

    /* check if this element needs to inserted at the end (ie. ordered
       data is inserted) This will be very likeley if a sequencer
       application or midi file player is feeding us (sequential) data */
    if !(*f).tail.is_null() && prior == 0 {
        if compare_timestamp(&mut (*cell).event, &mut (*(*f).tail).event) != 0 {
            /* add new cell to tail of the fifo */
            (*(*f).tail).next = cell;
            (*f).tail = cell;
            (*cell).next = ptr::null_mut();
            (*f).cells += 1;
            unlock_irqrestore(&mut (*f).lock, flags);
            return 0;
        }
    }
    /* traverse list of elements to find the place where the new cell is
       to be inserted... Note that this is a order n process ! */

    prev = ptr::null_mut(); /* previous cell */
    cur = (*f).head; /* cursor */

    remaining = (*f).cells;
    while !cur.is_null() {
        /* compare timestamps */
        let rel: c_int = compare_timestamp_rel(&mut (*cell).event, &mut (*cur).event);

        if {
            let old = remaining;
            remaining -= 1;
            old <= 0
        } {
            pr_err(b"ALSA: seq: inconsistent prioq cell count\n\0".as_ptr());
            unlock_irqrestore(&mut (*f).lock, flags);
            return -EINVAL;
        }

        if rel < 0 {
            /* new cell has earlier schedule time, */
            break;
        } else if rel == 0 && prior != 0 {
            /* equal schedule time and prior to others */
            break;
        }
        /* new cell has equal or larger schedule time, */
        /* move cursor to next cell */
        prev = cur;
        cur = (*cur).next;
    }

    /* insert it before cursor */
    if !prev.is_null() {
        (*prev).next = cell;
    }
    (*cell).next = cur;

    if (*f).head == cur {
        /* this is the first cell, set head to it */
        (*f).head = cell;
    }
    if cur.is_null() {
        /* reached end of the list */
        (*f).tail = cell;
    }
    (*f).cells += 1;
    unlock_irqrestore(&mut (*f).lock, flags);
    0
}

/* return 1 if the current time >= event timestamp */
unsafe fn event_is_ready(ev: *mut snd_seq_event, current_time: *mut c_void) -> c_int {
    if (((*ev).flags as c_int) & SNDRV_SEQ_TIME_STAMP_MASK) == SNDRV_SEQ_TIME_STAMP_TICK {
        snd_seq_compare_tick_time(current_time as *const snd_seq_tick_time_t, &(*ev).time.tick)
    } else {
        snd_seq_compare_real_time(current_time as *const snd_seq_real_time_t, &(*ev).time.time)
    }
}

/* dequeue cell from prioq */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_prioq_cell_out(
    f: *mut snd_seq_prioq,
    current_time: *mut c_void,
) -> *mut snd_seq_event_cell {
    let mut cell: *mut snd_seq_event_cell;

    if f.is_null() {
        pr_debug(b"ALSA: seq: snd_seq_prioq_cell_in() called with NULL prioq\n\0".as_ptr());
        return ptr::null_mut();
    }

    let flags = lock_irqsave(&mut (*f).lock);
    cell = (*f).head;
    if !cell.is_null() && !current_time.is_null() && event_is_ready(&mut (*cell).event, current_time) == 0 {
        cell = ptr::null_mut();
    }
    if !cell.is_null() {
        (*f).head = (*cell).next;

        /* reset tail if this was the last element */
        if (*f).tail == cell {
            (*f).tail = ptr::null_mut();
        }

        (*cell).next = ptr::null_mut();
        (*f).cells -= 1;
    }

    unlock_irqrestore(&mut (*f).lock, flags);
    cell
}

/* return number of events available in prioq */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_prioq_avail(f: *mut snd_seq_prioq) -> c_int {
    if f.is_null() {
        pr_debug(b"ALSA: seq: snd_seq_prioq_cell_in() called with NULL prioq\n\0".as_ptr());
        return 0;
    }
    (*f).cells
}

/* remove cells matching with the condition */
unsafe fn prioq_remove_cells(
    f: *mut snd_seq_prioq,
    match_fn: Option<unsafe extern "C" fn(cell: *mut snd_seq_event_cell, arg: *mut c_void) -> bool>,
    arg: *mut c_void,
) {
    let mut cell: *mut snd_seq_event_cell;
    let mut next: *mut snd_seq_event_cell;
    let mut prev: *mut snd_seq_event_cell = ptr::null_mut();
    let mut freefirst: *mut snd_seq_event_cell = ptr::null_mut();
    let mut freeprev: *mut snd_seq_event_cell = ptr::null_mut();
    let mut freenext: *mut snd_seq_event_cell;

    /* collect all removed cells */
    let flags = lock_irqsave(&mut (*f).lock);
    cell = (*f).head;
    while !cell.is_null() {
        next = (*cell).next;
        if !match_fn.unwrap()(cell, arg) {
            prev = cell;
            cell = next;
            continue;
        }

        /* remove cell from prioq */
        if cell == (*f).head {
            (*f).head = (*cell).next;
        } else {
            (*prev).next = (*cell).next;
        }
        if cell == (*f).tail {
            (*f).tail = (*cell).next;
        }
        (*f).cells -= 1;

        /* add cell to free list */
        (*cell).next = ptr::null_mut();
        if freefirst.is_null() {
            freefirst = cell;
        } else {
            (*freeprev).next = cell;
        }
        freeprev = cell;
        cell = next;
    }
    unlock_irqrestore(&mut (*f).lock, flags);

    /* remove selected cells */
    while !freefirst.is_null() {
        freenext = (*freefirst).next;
        snd_seq_cell_free(freefirst);
        freefirst = freenext;
    }
}

#[repr(C)]
pub struct prioq_match_arg {
    pub client: c_int,
    pub timestamp: c_int,
}

#[inline]
unsafe extern "C" fn prioq_match(cell: *mut snd_seq_event_cell, arg: *mut c_void) -> bool {
    let v: *mut prioq_match_arg = arg as *mut prioq_match_arg;

    if (*cell).event.source.client as c_int == (*v).client
        || (*cell).event.dest.client as c_int == (*v).client
    {
        return true;
    }
    if (*v).timestamp == 0 {
        return false;
    }
    match ((*cell).event.flags as c_int) & SNDRV_SEQ_TIME_STAMP_MASK {
        x if x == SNDRV_SEQ_TIME_STAMP_TICK => {
            if (*cell).event.time.tick != 0 {
                return true;
            }
        }
        x if x == SNDRV_SEQ_TIME_STAMP_REAL => {
            if (*cell).event.time.time.tv_sec != 0 || (*cell).event.time.time.tv_nsec != 0 {
                return true;
            }
        }
        _ => {}
    }
    false
}

/* remove cells for left client */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_prioq_leave(
    f: *mut snd_seq_prioq,
    client: c_int,
    timestamp: c_int,
) {
    let mut arg = prioq_match_arg { client, timestamp };

    return prioq_remove_cells(f, Some(prioq_match), &mut arg as *mut _ as *mut c_void);
}

#[repr(C)]
pub struct prioq_remove_match_arg {
    pub client: c_int,
    pub info: *mut snd_seq_remove_events,
}

unsafe extern "C" fn prioq_remove_match(cell: *mut snd_seq_event_cell, arg: *mut c_void) -> bool {
    let v: *mut prioq_remove_match_arg = arg as *mut prioq_remove_match_arg;
    let ev: *mut snd_seq_event = &mut (*cell).event;
    let info: *mut snd_seq_remove_events = (*v).info;
    let mut res: c_int;

    if (*ev).source.client as c_int != (*v).client {
        return false;
    }

    if (*info).remove_mode & SNDRV_SEQ_REMOVE_DEST != 0 {
        if (*ev).dest.client != (*info).dest.client || (*ev).dest.port != (*info).dest.port {
            return false;
        }
    }
    if (*info).remove_mode & SNDRV_SEQ_REMOVE_DEST_CHANNEL != 0 {
        if !snd_seq_ev_is_channel_type(ev) {
            return false;
        }
        /* data.note.channel and data.control.channel are identical */
        if (*ev).data.note.channel != (*info).channel {
            return false;
        }
    }
    if (*info).remove_mode & SNDRV_SEQ_REMOVE_TIME_AFTER != 0 {
        if (*info).remove_mode & SNDRV_SEQ_REMOVE_TIME_TICK != 0 {
            res = snd_seq_compare_tick_time(&(*ev).time.tick, &(*info).time.tick);
        } else {
            res = snd_seq_compare_real_time(&(*ev).time.time, &(*info).time.time);
        }
        if res == 0 {
            return false;
        }
    }
    if (*info).remove_mode & SNDRV_SEQ_REMOVE_TIME_BEFORE != 0 {
        if (*info).remove_mode & SNDRV_SEQ_REMOVE_TIME_TICK != 0 {
            res = snd_seq_compare_tick_time(&(*ev).time.tick, &(*info).time.tick);
        } else {
            res = snd_seq_compare_real_time(&(*ev).time.time, &(*info).time.time);
        }
        if res != 0 {
            return false;
        }
    }
    if (*info).remove_mode & SNDRV_SEQ_REMOVE_EVENT_TYPE != 0 {
        if (*ev).type_ != (*info).type_ {
            return false;
        }
    }
    if (*info).remove_mode & SNDRV_SEQ_REMOVE_IGNORE_OFF != 0 {
        /* Do not remove off events */
        match (*ev).type_ as c_int {
            x if x == SNDRV_SEQ_EVENT_NOTEOFF => {
                return false;
            }
            /* case SNDRV_SEQ_EVENT_SAMPLE_STOP: */
            _ => {}
        }
    }
    if (*info).remove_mode & SNDRV_SEQ_REMOVE_TAG_MATCH != 0 {
        if (*info).tag != (*ev).tag {
            return false;
        }
    }

    true
}

/* remove cells matching remove criteria */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_prioq_remove_events(
    f: *mut snd_seq_prioq,
    client: c_int,
    info: *mut snd_seq_remove_events,
) {
    let mut arg = prioq_remove_match_arg { client, info };

    return prioq_remove_cells(f, Some(prioq_remove_match), &mut arg as *mut _ as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
