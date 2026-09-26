/*
 * Translated from include/linux/spinlock_types.h.
 * Dependency headers are intentionally supplied externally.
 */

/* #include <linux/spinlock_types_raw.h> */

/*
 * In C the lock is a union of the raw lock and, with lockdep, padding
 * followed by a dep_map that aliases rlock.dep_map; the union therefore has
 * exactly the raw lock's layout, and dep_map is reached through rlock.
 */
#[cfg(not(CONFIG_PREEMPT_RT))]
#[repr(C)]
pub struct spinlock {
    pub rlock: raw_spinlock,
}

#[cfg(not(CONFIG_PREEMPT_RT))]
pub type spinlock_t = spinlock;

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! ___SPIN_LOCK_INITIALIZER {
    ($($lockname:tt)+) => {
        __raw_spin_lock_value!(LD_WAIT_CONFIG, 0, $($lockname)+)
    };
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __SPIN_LOCK_INITIALIZER {
    ($($lockname:tt)+) => {
        spinlock { rlock: ___SPIN_LOCK_INITIALIZER!($($lockname)+) }
    };
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __SPIN_LOCK_UNLOCKED {
    ($($lockname:tt)+) => {
        __SPIN_LOCK_INITIALIZER!($($lockname)+)
    };
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __LOCAL_SPIN_LOCK_UNLOCKED {
    ($($lockname:tt)+) => {
        spinlock { rlock: __raw_spin_lock_value!(LD_WAIT_CONFIG, LD_LOCK_PERCPU, $($lockname)+) }
    };
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! DEFINE_SPINLOCK {
    ($x:ident) => {
        static mut $x: spinlock_t = __SPIN_LOCK_UNLOCKED!($x);
    };
}

/* PREEMPT_RT kernels map spinlock to rt_mutex. */
/* #include <linux/rtmutex.h> */

#[cfg(CONFIG_PREEMPT_RT)]
#[repr(C)]
pub struct spinlock {
    pub lock: rt_mutex_base,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
}

#[cfg(CONFIG_PREEMPT_RT)]
pub type spinlock_t = spinlock;

#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __SPIN_LOCK_UNLOCKED {
    ($($name:tt)+) => {
        spinlock {
            lock: __RT_MUTEX_BASE_INITIALIZER!($($name)+ . lock),
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            dep_map: lockdep_map {
                name: concat!(stringify!($($name)+), "\0").as_ptr().cast(),
                wait_type_inner: LD_WAIT_CONFIG as _,
                // SAFETY: the remaining lockdep fields are zero-initialized in C.
                ..unsafe { ::core::mem::zeroed() }
            },
        }
    };
}

#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __LOCAL_SPIN_LOCK_UNLOCKED {
    ($($name:tt)+) => {
        spinlock {
            lock: __RT_MUTEX_BASE_INITIALIZER!($($name)+ . lock),
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            dep_map: lockdep_map {
                name: concat!(stringify!($($name)+), "\0").as_ptr().cast(),
                wait_type_inner: LD_WAIT_CONFIG as _,
                lock_type: LD_LOCK_PERCPU as _,
                // SAFETY: the remaining lockdep fields are zero-initialized in C.
                ..unsafe { ::core::mem::zeroed() }
            },
        }
    };
}

#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! DEFINE_SPINLOCK {
    ($name:ident) => {
        static mut $name: spinlock_t = __SPIN_LOCK_UNLOCKED!($name);
    };
}

/* #include <linux/rwlock_types.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
