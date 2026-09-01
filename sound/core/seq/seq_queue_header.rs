/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA sequencer Queue handling
 *   Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/* Dependencies in the original header:
 * seq_memory.h, seq_prioq.h, seq_timer.h, seq_lock.h,
 * linux/interrupt.h, linux/list.h, linux/bitops.h
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

pub const SEQ_QUEUE_NO_OWNER: c_int = -1;

/* Mirrors the Linux DECLARE_BITMAP(name, bits) storage sizing. */
pub const BITS_PER_LONG: usize = core::mem::size_of::<c_ulong>() * 8;
pub const fn bits_to_longs(nr: usize) -> usize {
    (nr + BITS_PER_LONG - 1) / BITS_PER_LONG
}

#[repr(C)]
pub struct snd_seq_queue {
    pub queue: c_int, /* queue number */

    pub name: [c_char; 64], /* name of this queue */

    pub tickq: *mut snd_seq_prioq, /* midi tick event queue */
    pub timeq: *mut snd_seq_prioq, /* real-time event queue */

    pub timer: *mut snd_seq_timer, /* time keeper for this queue */
    pub owner: c_int,              /* client that 'owns' the timer */
    pub locked: bool,              /* timer is only accesibble by owner if set */
    pub klocked: bool,             /* kernel lock (after START) */
    pub check_again: bool,         /* concurrent access happened during check */
    pub check_blocked: bool,       /* queue being checked */

    pub flags: c_uint,      /* status flags */
    pub info_flags: c_uint, /* info for sync */

    pub owner_lock: spinlock_t,
    pub check_lock: spinlock_t,

    /* clients which uses this queue (bitmap) */
    pub clients_bitmap: [c_ulong; bits_to_longs(SNDRV_SEQ_MAX_CLIENTS as usize)],
    pub clients: c_uint, /* users of this queue */
    pub timer_mutex: mutex,

    pub use_lock: snd_use_lock_t,
}

unsafe extern "C" {
    /* get the number of current queues */
    pub fn snd_seq_queue_get_cur_queues() -> c_int;

    /* delete queues */
    pub fn snd_seq_queues_delete();

    /* create new queue (constructor) */
    pub fn snd_seq_queue_alloc(
        client: c_int,
        locked: c_int,
        flags: c_uint,
    ) -> *mut snd_seq_queue;

    /* delete queue (destructor) */
    pub fn snd_seq_queue_delete(client: c_int, queueid: c_int) -> c_int;

    /* final stage */
    pub fn snd_seq_queue_client_leave(client: c_int);

    /* enqueue a event received from one the clients */
    pub fn snd_seq_enqueue_event(cell: *mut snd_seq_event_cell, atomic: c_int, hop: c_int) -> c_int;

    /* Remove events */
    pub fn snd_seq_queue_remove_cells(client: c_int, info: *mut snd_seq_remove_events);

    /* return pointer to queue structure for specified id */
    pub fn queueptr(queueid: c_int) -> *mut snd_seq_queue;

    pub fn snd_use_lock_free(lock: *mut snd_use_lock_t);

    /* return the (first) queue matching with the specified name */
    pub fn snd_seq_queue_find_name(name: *mut c_char) -> *mut snd_seq_queue;

    /* check single queue and dispatch events */
    pub fn snd_seq_check_queue(q: *mut snd_seq_queue, atomic: c_int, hop: c_int);

    /* access to queue's parameters */
    pub fn snd_seq_queue_check_access(queueid: c_int, client: c_int) -> c_int;
    pub fn snd_seq_queue_timer_set_tempo(
        queueid: c_int,
        client: c_int,
        info: *mut snd_seq_queue_tempo,
    ) -> c_int;
    pub fn snd_seq_queue_set_owner(queueid: c_int, client: c_int, locked: c_int) -> c_int;
    pub fn snd_seq_queue_timer_open(queueid: c_int) -> c_int;
    pub fn snd_seq_queue_timer_close(queueid: c_int) -> c_int;
    pub fn snd_seq_queue_use(queueid: c_int, client: c_int, use_: c_int) -> c_int;
    pub fn snd_seq_queue_is_used(queueid: c_int, client: c_int) -> c_int;

    pub fn snd_seq_control_queue(ev: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int;
}

/* unlock */
#[inline]
pub unsafe fn queuefree(q: *mut snd_seq_queue) {
    unsafe {
        snd_use_lock_free(&mut (*q).use_lock);
    }
}

/* Original C cleanup helper:
 * DEFINE_FREE(snd_seq_queue, struct snd_seq_queue *,
 *             if (!IS_ERR_OR_NULL(_T)) queuefree(_T))
 *
 * This depends on Linux's DEFINE_FREE and IS_ERR_OR_NULL machinery; the cleanup
 * registration is preserved here as dependency intent rather than executable
 * Rust because Rust has no direct file-local equivalent for that C macro.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
