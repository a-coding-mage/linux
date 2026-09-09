// Translated from rwlock.h. The original include guard and direct-include
// diagnostic are intentionally omitted; this file remains dependent on the
// surrounding spinlock definitions.

#[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
extern "C" {
    pub fn __rwlock_init(
        lock: *mut rwlock_t,
        name: *const ::core::ffi::c_char,
        key: *mut lock_class_key,
    );
}

#[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
#[macro_export]
macro_rules! rwlock_init {
    ($lock:expr) => {{
        static mut __KEY: lock_class_key = unsafe { ::core::mem::zeroed() };
        unsafe {
            $crate::__rwlock_init(
                ($lock),
                concat!(stringify!($lock), "\0").as_ptr() as *const ::core::ffi::c_char,
                &mut __KEY,
            );
        }
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[macro_export]
macro_rules! rwlock_init {
    ($lock:expr) => {{
        unsafe { *($lock) = __RW_LOCK_UNLOCKED!($lock); }
    }};
}

#[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
extern "C" {
    pub fn do_raw_read_lock(lock: *mut rwlock_t);
    pub fn do_raw_read_trylock(lock: *mut rwlock_t) -> ::core::ffi::c_int;
    pub fn do_raw_read_unlock(lock: *mut rwlock_t);
    pub fn do_raw_write_lock(lock: *mut rwlock_t);
    pub fn do_raw_write_trylock(lock: *mut rwlock_t) -> ::core::ffi::c_int;
    pub fn do_raw_write_unlock(lock: *mut rwlock_t);
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[macro_export]
macro_rules! do_raw_read_lock {
    ($rwlock:expr) => {{
        unsafe { __acquire_shared!(lock); arch_read_lock(&mut (*($rwlock)).raw_lock); }
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[inline]
pub unsafe fn do_raw_read_trylock(rwlock: *mut rwlock_t) -> ::core::ffi::c_int {
    arch_read_trylock(&mut (*rwlock).raw_lock)
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[macro_export]
macro_rules! do_raw_read_unlock {
    ($rwlock:expr) => {{
        unsafe { arch_read_unlock(&mut (*($rwlock)).raw_lock); __release_shared!(lock); }
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[macro_export]
macro_rules! do_raw_write_lock {
    ($rwlock:expr) => {{
        unsafe { __acquire!(lock); arch_write_lock(&mut (*($rwlock)).raw_lock); }
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[inline]
pub unsafe fn do_raw_write_trylock(rwlock: *mut rwlock_t) -> ::core::ffi::c_int {
    arch_write_trylock(&mut (*rwlock).raw_lock)
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[macro_export]
macro_rules! do_raw_write_unlock {
    ($rwlock:expr) => {{
        unsafe { arch_write_unlock(&mut (*($rwlock)).raw_lock); __release!(lock); }
    }};
}

#[macro_export]
macro_rules! read_trylock { ($lock:expr) => { _raw_read_trylock!($lock) }; }
#[macro_export]
macro_rules! write_trylock { ($lock:expr) => { _raw_write_trylock!($lock) }; }
#[macro_export]
macro_rules! write_lock { ($lock:expr) => { _raw_write_lock!($lock) }; }
#[macro_export]
macro_rules! read_lock { ($lock:expr) => { _raw_read_lock!($lock) }; }

// CONFIG_DEBUG_LOCK_ALLOC selects the nested locking variant.
#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
#[macro_export]
macro_rules! write_lock_nested { ($lock:expr, $subclass:expr) => { _raw_write_lock_nested!($lock, $subclass) }; }
#[cfg(not(feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
#[macro_export]
macro_rules! write_lock_nested { ($lock:expr, $subclass:expr) => { _raw_write_lock!($lock) }; }

// When neither CONFIG_SMP nor CONFIG_DEBUG_SPINLOCK is enabled, the raw
// irqsave operations have the alternate flags argument semantics.
#[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
#[macro_export]
macro_rules! read_lock_irqsave { ($lock:expr, $flags:expr) => {{ typecheck!(unsigned long, $flags); $flags = _raw_read_lock_irqsave!($lock); }}; }
#[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
#[macro_export]
macro_rules! write_lock_irqsave { ($lock:expr, $flags:expr) => {{ typecheck!(unsigned long, $flags); $flags = _raw_write_lock_irqsave!($lock); }}; }
#[cfg(not(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK")))]
#[macro_export]
macro_rules! read_lock_irqsave { ($lock:expr, $flags:expr) => {{ typecheck!(unsigned long, $flags); _raw_read_lock_irqsave!($lock, $flags); }}; }
#[cfg(not(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK")))]
#[macro_export]
macro_rules! write_lock_irqsave { ($lock:expr, $flags:expr) => {{ typecheck!(unsigned long, $flags); _raw_write_lock_irqsave!($lock, $flags); }}; }

#[macro_export] macro_rules! read_lock_irq { ($lock:expr) => { _raw_read_lock_irq!($lock) }; }
#[macro_export] macro_rules! read_lock_bh { ($lock:expr) => { _raw_read_lock_bh!($lock) }; }
#[macro_export] macro_rules! write_lock_irq { ($lock:expr) => { _raw_write_lock_irq!($lock) }; }
#[macro_export] macro_rules! write_lock_bh { ($lock:expr) => { _raw_write_lock_bh!($lock) }; }
#[macro_export] macro_rules! read_unlock { ($lock:expr) => { _raw_read_unlock!($lock) }; }
#[macro_export] macro_rules! write_unlock { ($lock:expr) => { _raw_write_unlock!($lock) }; }
#[macro_export] macro_rules! read_unlock_irq { ($lock:expr) => { _raw_read_unlock_irq!($lock) }; }
#[macro_export] macro_rules! write_unlock_irq { ($lock:expr) => { _raw_write_unlock_irq!($lock) }; }
#[macro_export] macro_rules! read_unlock_irqrestore { ($lock:expr, $flags:expr) => {{ typecheck!(unsigned long, $flags); _raw_read_unlock_irqrestore!($lock, $flags); }}; }
#[macro_export] macro_rules! read_unlock_bh { ($lock:expr) => { _raw_read_unlock_bh!($lock) }; }
#[macro_export] macro_rules! write_unlock_irqrestore { ($lock:expr, $flags:expr) => {{ typecheck!(unsigned long, $flags); _raw_write_unlock_irqrestore!($lock, $flags); }}; }
#[macro_export] macro_rules! write_unlock_bh { ($lock:expr) => { _raw_write_unlock_bh!($lock) }; }
#[macro_export] macro_rules! write_trylock_irqsave { ($lock:expr, $flags:expr) => { _raw_write_trylock_irqsave!($lock, &mut $flags) }; }

// CONFIG-dependent arch_rwlock_is_contended definition.
#[cfg(feature = "arch_rwlock_is_contended")]
#[macro_export]
macro_rules! rwlock_is_contended { ($lock:expr) => { arch_rwlock_is_contended!(&mut (*($lock)).raw_lock) }; }
#[cfg(not(feature = "arch_rwlock_is_contended"))]
#[macro_export]
macro_rules! rwlock_is_contended { ($lock:expr) => {{ let _ = &$lock; 0 }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
