/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  ALSA sequencer Memory Manager
 *  Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

use core::ffi::{c_char, c_int};

/* Dependencies from the original header:
 * #include <sound/seq_kernel.h>
 * #include <linux/poll.h>
 */

/* aliasing for legacy and UMP event packet handling */
#[repr(C)]
pub union __snd_seq_event {
    pub legacy: snd_seq_event,
    #[cfg(CONFIG_SND_SEQ_UMP)]
    pub ump: snd_seq_ump_event,
    pub raw: __snd_seq_event_raw,
}

#[repr(C, packed)]
pub struct __snd_seq_event_raw {
    pub event: snd_seq_event,
    #[cfg(CONFIG_SND_SEQ_UMP)]
    pub extra: u32,
}

/* container for sequencer event (internal use) */
#[repr(C)]
pub struct snd_seq_event_cell {
    pub event_ump: snd_seq_event_cell_event_ump,
    pub pool: *mut snd_seq_pool, /* used pool */
    pub next: *mut snd_seq_event_cell, /* next cell */
}

#[repr(C)]
pub union snd_seq_event_cell_event_ump {
    pub event: snd_seq_event,
    pub ump: __snd_seq_event,
}

/* design note: the pool is a contiguous block of memory, if we dynamicly
   want to add additional cells to the pool be better store this in another
   pool as we need to know the base address of the pool when releasing
   memory. */

#[repr(C)]
pub struct snd_seq_pool {
    pub ptr: *mut snd_seq_event_cell, /* pointer to first event chunk */
    pub free: *mut snd_seq_event_cell, /* pointer to the head of the free list */

    pub total_elements: c_int, /* pool size actually allocated */
    pub counter: atomic_t, /* cells free */

    pub size: c_int, /* pool size to be allocated */
    pub room: c_int, /* watermark for sleep/wakeup */

    pub closing: c_int,

    /* statistics */
    pub max_used: c_int,
    pub event_alloc_nopool: c_int,
    pub event_alloc_failures: c_int,
    pub event_alloc_success: c_int,

    /* Write locking */
    pub output_sleep: wait_queue_head_t,

    /* Pool lock */
    pub lock: spinlock_t,
}

unsafe extern "C" {
    pub fn snd_seq_cell_free(cell: *mut snd_seq_event_cell);

    pub fn snd_seq_event_dup(
        pool: *mut snd_seq_pool,
        event: *mut snd_seq_event,
        cellp: *mut *mut snd_seq_event_cell,
        nonblock: c_int,
        file: *mut file,
        mutexp: *mut mutex,
    ) -> c_int;
}

/* return number of unused (free) cells */
#[inline]
pub unsafe fn snd_seq_unused_cells(pool: *mut snd_seq_pool) -> c_int {
    if !pool.is_null() {
        unsafe { (*pool).total_elements - atomic_read(&raw const (*pool).counter) }
    } else {
        0
    }
}

/* return total number of allocated cells */
#[inline]
pub unsafe fn snd_seq_total_cells(pool: *mut snd_seq_pool) -> c_int {
    if !pool.is_null() {
        unsafe { (*pool).total_elements }
    } else {
        0
    }
}

unsafe extern "C" {
    /* init pool - allocate events */
    pub fn snd_seq_pool_init(pool: *mut snd_seq_pool) -> c_int;

    /* done pool - free events */
    pub fn snd_seq_pool_mark_closing(pool: *mut snd_seq_pool);
    pub fn snd_seq_pool_done(pool: *mut snd_seq_pool) -> c_int;

    /* create pool */
    pub fn snd_seq_pool_new(poolsize: c_int) -> *mut snd_seq_pool;

    /* remove pool */
    pub fn snd_seq_pool_delete(pool: *mut *mut snd_seq_pool) -> c_int;

    /* polling */
    pub fn snd_seq_pool_poll_wait(
        pool: *mut snd_seq_pool,
        file: *mut file,
        wait: *mut poll_table,
    ) -> c_int;

    pub fn snd_seq_info_pool(
        buffer: *mut snd_info_buffer,
        pool: *mut snd_seq_pool,
        space: *mut c_char,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
