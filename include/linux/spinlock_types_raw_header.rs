// Translated from linux/spinlock_types_raw.h.
// C dependencies supplied by other translation units:
// arch_spinlock_t, __ARCH_SPIN_LOCK_UNLOCKED, lockdep_map, LD_WAIT_SPIN,
// LD_WAIT_CONFIG, and LD_LOCK_PERCPU.

#[repr(C)]
pub struct raw_spinlock {
    pub raw_lock: arch_spinlock_t,
    #[cfg(CONFIG_DEBUG_SPINLOCK)]
    pub magic: ::core::ffi::c_uint,
    #[cfg(CONFIG_DEBUG_SPINLOCK)]
    pub owner_cpu: ::core::ffi::c_uint,
    #[cfg(CONFIG_DEBUG_SPINLOCK)]
    pub owner: *mut ::core::ffi::c_void,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
}

pub type raw_spinlock_t = raw_spinlock;

pub const SPINLOCK_MAGIC: ::core::ffi::c_uint = 0xdead4ead;

pub const SPINLOCK_OWNER_INIT: *mut ::core::ffi::c_void = (-1isize) as *mut ::core::ffi::c_void;

/*
 * C composes the initializer from SPIN_DEBUG_INIT() and *_DEP_MAP_INIT()
 * field lists; Rust macros expand to whole expressions, so the lock value
 * is built in one struct literal with the debug fields gated by config.
 * The lockdep class name is the stringified lock name, like C's #lockname.
 */
#[doc(hidden)]
#[macro_export]
macro_rules! __raw_spin_lock_value {
    ($wait_type_inner:expr, $lock_type:expr, $($lockname:tt)+) => {
        raw_spinlock {
            raw_lock: __ARCH_SPIN_LOCK_UNLOCKED!(),
            #[cfg(CONFIG_DEBUG_SPINLOCK)]
            magic: SPINLOCK_MAGIC,
            #[cfg(CONFIG_DEBUG_SPINLOCK)]
            owner_cpu: -1i32 as ::core::ffi::c_uint,
            #[cfg(CONFIG_DEBUG_SPINLOCK)]
            owner: SPINLOCK_OWNER_INIT,
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            dep_map: lockdep_map {
                name: concat!(stringify!($($lockname)+), "\0").as_ptr().cast(),
                wait_type_inner: $wait_type_inner as _,
                lock_type: $lock_type as _,
                // SAFETY: the remaining lockdep fields are zero-initialized in C.
                ..unsafe { ::core::mem::zeroed() }
            },
        }
    };
}

#[macro_export]
macro_rules! __RAW_SPIN_LOCK_INITIALIZER {
    ($($lockname:tt)+) => {
        __raw_spin_lock_value!(LD_WAIT_SPIN, 0, $($lockname)+)
    };
}

#[macro_export]
macro_rules! __RAW_SPIN_LOCK_UNLOCKED {
    ($($lockname:tt)+) => {
        __RAW_SPIN_LOCK_INITIALIZER!($($lockname)+)
    };
}

#[macro_export]
macro_rules! DEFINE_RAW_SPINLOCK {
    ($x:ident) => {
        static mut $x: raw_spinlock_t = __RAW_SPIN_LOCK_UNLOCKED!($x);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
