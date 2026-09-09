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

// CONFIG_DEBUG_LOCK_ALLOC controls the C initializer macros below.
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export]
macro_rules! RAW_SPIN_DEP_MAP_INIT {
    ($lockname:ident) => {
        .dep_map = lockdep_map {
            .name = stringify!($lockname),
            .wait_type_inner = LD_WAIT_SPIN,
        }
    };
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export]
macro_rules! SPIN_DEP_MAP_INIT {
    ($lockname:ident) => {
        .dep_map = lockdep_map {
            .name = stringify!($lockname),
            .wait_type_inner = LD_WAIT_CONFIG,
        }
    };
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[macro_export]
macro_rules! LOCAL_SPIN_DEP_MAP_INIT {
    ($lockname:ident) => {
        .dep_map = lockdep_map {
            .name = stringify!($lockname),
            .wait_type_inner = LD_WAIT_CONFIG,
            .lock_type = LD_LOCK_PERCPU,
        }
    };
}

#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
#[macro_export]
macro_rules! RAW_SPIN_DEP_MAP_INIT { ($lockname:ident) => {}; }
#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
#[macro_export]
macro_rules! SPIN_DEP_MAP_INIT { ($lockname:ident) => {}; }
#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
#[macro_export]
macro_rules! LOCAL_SPIN_DEP_MAP_INIT { ($lockname:ident) => {}; }

#[cfg(CONFIG_DEBUG_SPINLOCK)]
#[macro_export]
macro_rules! SPIN_DEBUG_INIT {
    ($lockname:ident) => {
        .magic = SPINLOCK_MAGIC,
        .owner_cpu = (-1i32) as ::core::ffi::c_uint,
        .owner = SPINLOCK_OWNER_INIT,
    };
}

#[cfg(not(CONFIG_DEBUG_SPINLOCK))]
#[macro_export]
macro_rules! SPIN_DEBUG_INIT { ($lockname:ident) => {}; }

#[macro_export]
macro_rules! __RAW_SPIN_LOCK_INITIALIZER {
    ($lockname:ident) => {
        raw_spinlock {
            .raw_lock = __ARCH_SPIN_LOCK_UNLOCKED,
            SPIN_DEBUG_INIT!($lockname)
            RAW_SPIN_DEP_MAP_INIT!($lockname)
        }
    };
}

#[macro_export]
macro_rules! __RAW_SPIN_LOCK_UNLOCKED {
    ($lockname:ident) => {
        __RAW_SPIN_LOCK_INITIALIZER!($lockname)
    };
}

#[macro_export]
macro_rules! DEFINE_RAW_SPINLOCK {
    ($x:ident) => {
        static mut $x: raw_spinlock_t = __RAW_SPIN_LOCK_UNLOCKED!($x);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
