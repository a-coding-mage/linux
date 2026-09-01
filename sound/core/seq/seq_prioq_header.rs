/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA sequencer Priority Queue
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/* Depends on declarations from "seq_memory.h". */
use crate::*;

/* === PRIOQ === */

#[repr(C)]
pub struct snd_seq_prioq {
    pub head: *mut snd_seq_event_cell, /* pointer to head of prioq */
    pub tail: *mut snd_seq_event_cell, /* pointer to tail of prioq */
    pub cells: ::std::os::raw::c_int,
    pub lock: spinlock_t,
}

unsafe extern "C" {
    /* create new prioq (constructor) */
    pub fn snd_seq_prioq_new() -> *mut snd_seq_prioq;

    /* delete prioq (destructor) */
    pub fn snd_seq_prioq_delete(fifo: *mut *mut snd_seq_prioq);

    /* enqueue cell to prioq */
    pub fn snd_seq_prioq_cell_in(
        f: *mut snd_seq_prioq,
        cell: *mut snd_seq_event_cell,
    ) -> ::std::os::raw::c_int;

    /* dequeue cell from prioq */
    pub fn snd_seq_prioq_cell_out(
        f: *mut snd_seq_prioq,
        current_time: *mut ::std::os::raw::c_void,
    ) -> *mut snd_seq_event_cell;

    /* return number of events available in prioq */
    pub fn snd_seq_prioq_avail(f: *mut snd_seq_prioq) -> ::std::os::raw::c_int;

    /* client left queue */
    pub fn snd_seq_prioq_leave(
        f: *mut snd_seq_prioq,
        client: ::std::os::raw::c_int,
        timestamp: ::std::os::raw::c_int,
    );

    /* Remove events */
    pub fn snd_seq_prioq_remove_events(
        f: *mut snd_seq_prioq,
        client: ::std::os::raw::c_int,
        info: *mut snd_seq_remove_events,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
