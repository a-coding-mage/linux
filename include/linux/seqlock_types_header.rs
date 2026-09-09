/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the C header:
// linux/lockdep_types.h, linux/mutex_types.h, linux/spinlock_types.h

/*
 * Sequence counters (seqcount_t)
 *
 * This is the raw counting mechanism, without any writer protection.
 *
 * Write side critical sections must be serialized and non-preemptible.
 *
 * If readers can be invoked from hardirq or softirq contexts,
 * interrupts or bottom halves must also be respectively disabled before
 * entering the write section.
 *
 * This mechanism can't be used if the protected data contains pointers,
 * as the writer can invalidate a pointer that a reader is following.
 *
 * If the write serialization mechanism is one of the common kernel
 * locking primitives, use a sequence counter with associated lock
 * (seqcount_LOCKNAME_t) instead.
 *
 * If it's desired to automatically handle the sequence counter writer
 * serialization and non-preemptibility requirements, use a sequential
 * lock (seqlock_t) instead.
 *
 * See Documentation/locking/seqlock.rst
 */
#[repr(C)]
pub struct seqcount {
    pub sequence: ::core::ffi::c_uint,
    // Present in C when CONFIG_DEBUG_LOCK_ALLOC is enabled.
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub dep_map: lockdep_map,
}
pub type seqcount_t = seqcount;

/*
 * For PREEMPT_RT, seqcount_LOCKNAME_t write side critical sections cannot
 * disable preemption. It can lead to higher latencies, and the write side
 * sections will not be able to acquire locks which become sleeping locks
 * (e.g. spinlock_t).
 *
 * To remain preemptible while avoiding a possible livelock caused by the
 * reader preempting the writer, use a different technique: let the reader
 * detect if a seqcount_LOCKNAME_t writer is in progress. If that is
 * the case, acquire then release the associated LOCKNAME writer serialization
 * lock. This will allow any possibly-preempted writer to make progress
 * until the end of its writer serialization lock critical section.
 *
 * This lock-unlock technique must be implemented for all of PREEMPT_RT
 * sleeping locks.  See Documentation/locking/locktypes.rst
 */

#[repr(C)]
pub struct seqcount_raw_spinlock {
    pub seqcount: seqcount_t,
    // __SEQ_LOCK(locktype *lock) is present for CONFIG_LOCKDEP or CONFIG_PREEMPT_RT.
    #[cfg(any(feature = "CONFIG_LOCKDEP", feature = "CONFIG_PREEMPT_RT"))]
    pub lock: *mut raw_spinlock_t,
}
pub type seqcount_raw_spinlock_t = seqcount_raw_spinlock;

#[repr(C)]
pub struct seqcount_spinlock {
    pub seqcount: seqcount_t,
    #[cfg(any(feature = "CONFIG_LOCKDEP", feature = "CONFIG_PREEMPT_RT"))]
    pub lock: *mut spinlock_t,
}
pub type seqcount_spinlock_t = seqcount_spinlock;

#[repr(C)]
pub struct seqcount_rwlock {
    pub seqcount: seqcount_t,
    #[cfg(any(feature = "CONFIG_LOCKDEP", feature = "CONFIG_PREEMPT_RT"))]
    pub lock: *mut rwlock_t,
}
pub type seqcount_rwlock_t = seqcount_rwlock;

#[repr(C)]
pub struct seqcount_mutex {
    pub seqcount: seqcount_t,
    #[cfg(any(feature = "CONFIG_LOCKDEP", feature = "CONFIG_PREEMPT_RT"))]
    pub lock: *mut mutex,
}
pub type seqcount_mutex_t = seqcount_mutex;

/*
 * Sequential locks (seqlock_t)
 *
 * Sequence counters with an embedded spinlock for writer serialization
 * and non-preemptibility.
 *
 * For more info, see:
 *    - Comments on top of seqcount_t
 *    - Documentation/locking/seqlock.rst
 */
#[repr(C)]
pub struct seqlock {
    /*
     * Make sure that readers don't starve writers on PREEMPT_RT: use
     * seqcount_spinlock_t instead of seqcount_t. Check __SEQ_LOCK().
     */
    pub seqcount: seqcount_spinlock_t,
    pub lock: spinlock_t,
}
pub type seqlock_t = seqlock;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
