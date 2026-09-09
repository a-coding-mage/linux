/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated Linux headers:
// linux/atomic.h, linux/lockdep_types.h, linux/osq_lock.h,
// linux/spinlock_types.h, linux/types.h

// C build condition: !CONFIG_PREEMPT_RT
#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
pub struct mutex {
    /*
     * Simple, straightforward mutexes with strict semantics:
     *
     * - only one task can hold the mutex at a time
     * - only the owner can unlock the mutex
     * - multiple unlocks are not permitted
     * - recursive locking is not permitted
     * - a mutex object must be initialized via the API
     * - a mutex object must not be initialized via memset or copying
     * - task may not exit with mutex held
     * - memory areas where held locks reside must not be freed
     * - held mutexes must not be reinitialized
     * - mutexes may not be used in hardware or software interrupt
     *   contexts such as tasklets and timers
     *
     * These semantics are fully enforced when DEBUG_MUTEXES is
     * enabled. Furthermore, besides enforcing the above rules, the mutex
     * debugging code also implements a number of additional features
     * that make lock debugging easier and faster:
     *
     * - uses symbolic names of mutexes, whenever they are printed in debug output
     * - point-of-acquire tracking, symbolic lookup of function names
     * - list of all locks held in the system, printout of them
     * - owner tracking
     * - detects self-recursing locks and prints out all relevant info
     * - detects multi-task circular deadlocks and prints out all affected
     *   locks and tasks (and only those tasks)
     */
    pub owner: atomic_long_t,
    pub wait_lock: raw_spinlock_t,

    // C build condition: CONFIG_MUTEX_SPIN_ON_OWNER
    #[cfg(feature = "CONFIG_MUTEX_SPIN_ON_OWNER")]
    pub osq: optimistic_spin_queue, // Spinner MCS lock

    // *first_waiter is guarded by &wait_lock in the C declaration.
    pub first_waiter: *mut mutex_waiter,

    // C build condition: CONFIG_DEBUG_MUTEXES
    #[cfg(feature = "CONFIG_DEBUG_MUTEXES")]
    pub magic: *mut core::ffi::c_void,

    // C build condition: CONFIG_DEBUG_LOCK_ALLOC
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub dep_map: lockdep_map,
}

// C build condition: CONFIG_PREEMPT_RT
#[cfg(feature = "CONFIG_PREEMPT_RT")]
pub struct mutex {
    pub rtmutex: rt_mutex_base,

    // C build condition: CONFIG_DEBUG_LOCK_ALLOC
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub dep_map: lockdep_map,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
