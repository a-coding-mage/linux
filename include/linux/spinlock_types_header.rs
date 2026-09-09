/*
 * Translated from include/linux/spinlock_types.h.
 * Dependency headers are intentionally supplied externally.
 */

/* #include <linux/spinlock_types_raw.h> */

#[cfg(not(CONFIG_PREEMPT_RT))]
#[repr(C)]
pub struct spinlock {
    pub rlock: raw_spinlock,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
}

#[cfg(not(CONFIG_PREEMPT_RT))]
pub type spinlock_t = spinlock;

/*
 * C's LOCK_PADSIZE anonymous padding preserves the raw-spinlock layout when
 * lock debugging is enabled; the Rust declaration above relies on the
 * externally supplied raw_spinlock layout and lockdep_map declaration.
 */

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! ___SPIN_LOCK_INITIALIZER {
    ($lockname:expr) => {
        {
            raw_lock: __ARCH_SPIN_LOCK_UNLOCKED,
            $(SPIN_DEBUG_INIT!($lockname))?
            $(SPIN_DEP_MAP_INIT!($lockname))?
        }
    };
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __SPIN_LOCK_INITIALIZER {
    ($lockname:expr) => {
        {{ rlock: $crate::___SPIN_LOCK_INITIALIZER!($lockname) }}
    };
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __SPIN_LOCK_UNLOCKED {
    ($lockname:expr) => {
        $crate::__SPIN_LOCK_INITIALIZER!($lockname)
    };
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! DEFINE_SPINLOCK {
    ($x:ident) => {
        static mut $x: spinlock_t = $crate::__SPIN_LOCK_UNLOCKED!($x);
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
    ($name:expr) => {
        {
            lock: $crate::__RT_MUTEX_BASE_INITIALIZER!($name.lock),
            $crate::SPIN_DEP_MAP_INIT!($name)
        }
    };
}

#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __LOCAL_SPIN_LOCK_UNLOCKED {
    ($name:expr) => {
        {
            lock: $crate::__RT_MUTEX_BASE_INITIALIZER!($name.lock),
            $crate::LOCAL_SPIN_DEP_MAP_INIT!($name)
        }
    };
}

#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! DEFINE_SPINLOCK {
    ($name:ident) => {
        static mut $name: spinlock_t = $crate::__SPIN_LOCK_UNLOCKED!($name);
    };
}

/* #include <linux/rwlock_types.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
