// Translation of linux/rwlock_types.h.
// The original header requires linux/spinlock_types.h to be included first.

#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
macro_rules! RW_DEP_MAP_INIT {
    ($lockname:ident) => {
        .dep_map = lockdep_map {
            .name = stringify!($lockname),
            .wait_type_inner = LD_WAIT_CONFIG,
        }
    };
}

#[cfg(not(feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
macro_rules! RW_DEP_MAP_INIT {
    ($lockname:ident) => {};
}

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
/*
 * generic rwlock type definitions and initializers
 *
 * portions Copyright 2005, Red Hat, Inc., Ingo Molnar
 * Released under the General Public License (GPL).
 */
#[repr(C)]
pub struct rwlock {
    pub raw_lock: arch_rwlock_t,
    #[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
    pub magic: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
    pub owner_cpu: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
    pub owner: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub dep_map: lockdep_map,
}

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
pub type rwlock_t = rwlock;

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
pub const RWLOCK_MAGIC: core::ffi::c_uint = 0xdeaf1eed;

#[cfg(all(not(feature = "CONFIG_PREEMPT_RT"), feature = "CONFIG_DEBUG_SPINLOCK"))]
macro_rules! __RW_LOCK_UNLOCKED {
    ($lockname:ident) => {
        rwlock_t {
            raw_lock: __ARCH_RW_LOCK_UNLOCKED,
            magic: RWLOCK_MAGIC,
            owner: SPINLOCK_OWNER_INIT,
            owner_cpu: (-1i32) as core::ffi::c_uint,
            RW_DEP_MAP_INIT!($lockname)
        }
    };
}

#[cfg(all(not(feature = "CONFIG_PREEMPT_RT"), not(feature = "CONFIG_DEBUG_SPINLOCK")))]
macro_rules! __RW_LOCK_UNLOCKED {
    ($lockname:ident) => {
        rwlock_t {
            raw_lock: __ARCH_RW_LOCK_UNLOCKED,
            RW_DEP_MAP_INIT!($lockname)
        }
    };
}

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
macro_rules! DEFINE_RWLOCK {
    ($x:ident) => {
        static mut $x: rwlock_t = __RW_LOCK_UNLOCKED!($x);
    };
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
// The C header includes <linux/rwbase_rt.h>; its declarations are external dependencies.
#[cfg(feature = "CONFIG_PREEMPT_RT")]
#[repr(C)]
pub struct rwlock {
    pub rwbase: rwbase_rt,
    pub readers: atomic_t,
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub dep_map: lockdep_map,
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
pub type rwlock_t = rwlock;

#[cfg(feature = "CONFIG_PREEMPT_RT")]
macro_rules! __RWLOCK_RT_INITIALIZER {
    ($name:ident) => {
        rwlock_t {
            rwbase: __RWBASE_INITIALIZER!($name),
            RW_DEP_MAP_INIT!($name)
        }
    };
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
macro_rules! __RW_LOCK_UNLOCKED {
    ($name:ident) => { __RWLOCK_RT_INITIALIZER!($name) };
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
macro_rules! DEFINE_RWLOCK {
    ($name:ident) => {
        static mut $name: rwlock_t = __RW_LOCK_UNLOCKED!($name);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
