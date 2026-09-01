/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA sequencer FIFO
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/* C header dependencies: "seq_memory.h" and "seq_lock.h". */

/* === FIFO === */

#[repr(C)]
pub struct snd_seq_fifo {
    pub pool: *mut snd_seq_pool,              /* FIFO pool */
    pub head: *mut snd_seq_event_cell,        /* pointer to head of fifo */
    pub tail: *mut snd_seq_event_cell,        /* pointer to tail of fifo */
    pub cells: core::ffi::c_int,
    pub lock: spinlock_t,
    pub use_lock: snd_use_lock_t,
    pub input_sleep: wait_queue_head_t,
    pub overflow: atomic_t,
}

unsafe extern "C" {
    /* create new fifo (constructor) */
    pub fn snd_seq_fifo_new(poolsize: core::ffi::c_int) -> *mut snd_seq_fifo;

    /* delete fifo (destructor) */
    pub fn snd_seq_fifo_delete(f: *mut *mut snd_seq_fifo);

    /* enqueue event to fifo */
    pub fn snd_seq_fifo_event_in(
        f: *mut snd_seq_fifo,
        event: *mut snd_seq_event,
    ) -> core::ffi::c_int;

    /* get a cell from fifo - fifo should be locked */
    pub fn snd_seq_fifo_cell_out(
        f: *mut snd_seq_fifo,
        cellp: *mut *mut snd_seq_event_cell,
        nonblock: core::ffi::c_int,
    ) -> core::ffi::c_int;

    /* free dequeued cell - fifo should be locked */
    pub fn snd_seq_fifo_cell_putback(f: *mut snd_seq_fifo, cell: *mut snd_seq_event_cell);

    /* clean up queue */
    pub fn snd_seq_fifo_clear(f: *mut snd_seq_fifo);

    /* polling */
    pub fn snd_seq_fifo_poll_wait(
        f: *mut snd_seq_fifo,
        file: *mut file,
        wait: *mut poll_table,
    ) -> core::ffi::c_int;

    /* resize pool in fifo */
    pub fn snd_seq_fifo_resize(
        f: *mut snd_seq_fifo,
        poolsize: core::ffi::c_int,
    ) -> core::ffi::c_int;

    /* get the number of unused cells safely */
    pub fn snd_seq_fifo_unused_cells(f: *mut snd_seq_fifo) -> core::ffi::c_int;
}

/* lock fifo from release */
#[inline]
pub unsafe fn snd_seq_fifo_lock(fifo: *mut snd_seq_fifo) {
    unsafe {
        snd_use_lock_use(core::ptr::addr_of_mut!((*fifo).use_lock));
    }
}

#[inline]
pub unsafe fn snd_seq_fifo_unlock(fifo: *mut snd_seq_fifo) {
    unsafe {
        snd_use_lock_free(core::ptr::addr_of_mut!((*fifo).use_lock));
    }
}

/*
 * C source used:
 * DEFINE_GUARD(snd_seq_fifo, struct snd_seq_fifo *,
 *              snd_seq_fifo_lock(_T), snd_seq_fifo_unlock(_T))
 *
 * Its RAII guard intent is preserved here as a Rust guard type using the same
 * lock and unlock operations.
 */
pub struct snd_seq_fifo_guard {
    fifo: *mut snd_seq_fifo,
}

impl snd_seq_fifo_guard {
    #[inline]
    pub unsafe fn new(fifo: *mut snd_seq_fifo) -> Self {
        unsafe {
            snd_seq_fifo_lock(fifo);
        }
        Self { fifo }
    }
}

impl Drop for snd_seq_fifo_guard {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            snd_seq_fifo_unlock(self.fifo);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
