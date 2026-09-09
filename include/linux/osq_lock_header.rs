/* SPDX-License-Identifier: GPL-2.0 */

/*
 * An MCS like lock especially tailored for optimistic spinning for sleeping
 * lock implementations (mutex, rwsem, etc).
 */

#[repr(C)]
pub struct optimistic_spin_queue {
    /*
     * Stores an encoded value of the CPU # of the tail node in the queue.
     * If the queue is empty, then it's set to OSQ_UNLOCKED_VAL.
     */
    pub tail: atomic_t,
}

pub const OSQ_UNLOCKED_VAL: i32 = 0;

/* Init macro and function. */
/* OSQ_LOCK_UNLOCKED: { ATOMIC_INIT(OSQ_UNLOCKED_VAL) } */

#[inline]
pub unsafe fn osq_lock_init(lock: *mut optimistic_spin_queue) {
    atomic_set(&mut (*lock).tail, OSQ_UNLOCKED_VAL);
}

extern "C" {
    pub fn osq_lock(lock: *mut optimistic_spin_queue) -> bool;
    pub fn osq_unlock(lock: *mut optimistic_spin_queue);
}

#[inline]
pub unsafe fn osq_is_locked(lock: *mut optimistic_spin_queue) -> bool {
    atomic_read(&(*lock).tail) != OSQ_UNLOCKED_VAL
}

/* External atomic operations and type supplied by the surrounding kernel translation. */
extern "C" {
    fn atomic_set(v: *mut atomic_t, i: i32);
    fn atomic_read(v: *const atomic_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
