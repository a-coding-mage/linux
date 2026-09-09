// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Queued read/write locks
 *
 * (C) Copyright 2013-2014 Hewlett-Packard Development Company, L.P.
 *
 * Authors: Waiman Long <waiman.long@hp.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/**
 * queued_read_lock_slowpath - acquire read lock of a queued rwlock
 * @lock: Pointer to queued rwlock structure
 */
pub unsafe fn queued_read_lock_slowpath(lock: *mut qrwlock) {
    /*
     * Readers come here when they cannot get the lock without waiting
     */
    if unlikely(in_interrupt()) {
        /*
         * Readers in interrupt context will get the lock immediately
         * if the writer is just waiting (not holding the lock yet),
         * so spin with ACQUIRE semantics until the lock is available
         * without waiting in the queue.
         */
        atomic_cond_read_acquire(&(*lock).cnts, |val| !(val & _QW_LOCKED));
        return;
    }
    atomic_sub(_QR_BIAS, &(*lock).cnts);

    trace_contention_begin(lock, LCB_F_SPIN | LCB_F_READ);

    /*
     * Put the reader into the wait queue
     */
    arch_spin_lock(&mut (*lock).wait_lock);
    atomic_add(_QR_BIAS, &(*lock).cnts);

    /*
     * The ACQUIRE semantics of the following spinning code ensure
     * that accesses can't leak upwards out of our subsequent critical
     * section in the case that the lock is currently held for write.
     */
    atomic_cond_read_acquire(&(*lock).cnts, |val| !(val & _QW_LOCKED));

    /*
     * Signal the next one in queue to become queue head
     */
    arch_spin_unlock(&mut (*lock).wait_lock);

    trace_contention_end(lock, 0);
}

/**
 * queued_write_lock_slowpath - acquire write lock of a queued rwlock
 * @lock : Pointer to queued rwlock structure
 */
pub unsafe fn queued_write_lock_slowpath(lock: *mut qrwlock) {
    let mut cnts: i32;

    trace_contention_begin(lock, LCB_F_SPIN | LCB_F_WRITE);

    /* Put the writer into the wait queue */
    arch_spin_lock(&mut (*lock).wait_lock);

    /* Try to acquire the lock directly if no reader is present */
    cnts = atomic_read(&(*lock).cnts);
    if cnts == 0 && atomic_try_cmpxchg_acquire(&(*lock).cnts, &mut cnts, _QW_LOCKED) {
        arch_spin_unlock(&mut (*lock).wait_lock);
        trace_contention_end(lock, 0);
        return;
    }

    /* Set the waiting flag to notify readers that a writer is pending */
    atomic_or(_QW_WAITING, &(*lock).cnts);

    /* When no more readers or writers, set the locked flag */
    loop {
        cnts = atomic_cond_read_relaxed(&(*lock).cnts, |val| val == _QW_WAITING);
        if atomic_try_cmpxchg_acquire(&(*lock).cnts, &mut cnts, _QW_LOCKED) {
            break;
        }
    }

    arch_spin_unlock(&mut (*lock).wait_lock);

    trace_contention_end(lock, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
