// SPDX-License-Identifier: GPL-2.0-only
// This header is intended to be included through linux/spinlock.h.

#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
extern "C" {
    pub fn __rt_rwlock_init(
        rwlock: *mut rwlock_t,
        name: *const core::ffi::c_char,
        key: *mut lock_class_key,
    );
}

#[cfg(not(feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
#[inline(always)]
pub unsafe fn __rt_rwlock_init(
    _rwlock: *mut rwlock_t,
    _name: *mut core::ffi::c_char,
    _key: *mut lock_class_key,
) {
}

// C macro rwlock_init(rwl): initializes the rwbase and registers a lock key.
// The call-site expression is retained through the macro argument stringification.
#[macro_export]
macro_rules! rwlock_init {
    ($rwl:expr) => {{
        static mut __KEY: lock_class_key = lock_class_key::default();
        unsafe {
            init_rwbase_rt(&mut (*($rwl)).rwbase);
            __rt_rwlock_init(
                $rwl,
                concat!(stringify!($rwl), "\0").as_ptr() as *const core::ffi::c_char,
                &mut __KEY,
            );
        }
    }};
}

extern "C" {
    pub fn rt_read_lock(rwlock: *mut rwlock_t);
    pub fn rt_read_trylock(rwlock: *mut rwlock_t) -> core::ffi::c_int;
    pub fn rt_read_unlock(rwlock: *mut rwlock_t);
    pub fn rt_write_lock(rwlock: *mut rwlock_t);
    pub fn rt_write_lock_nested(rwlock: *mut rwlock_t, subclass: core::ffi::c_int);
    pub fn rt_write_trylock(rwlock: *mut rwlock_t) -> core::ffi::c_int;
    pub fn rt_write_unlock(rwlock: *mut rwlock_t);
}

#[inline(always)]
pub unsafe fn read_lock(rwlock: *mut rwlock_t) {
    rt_read_lock(rwlock);
}

#[inline(always)]
pub unsafe fn read_lock_bh(rwlock: *mut rwlock_t) {
    local_bh_disable();
    rt_read_lock(rwlock);
}

#[inline(always)]
pub unsafe fn read_lock_irq(rwlock: *mut rwlock_t) {
    rt_read_lock(rwlock);
}

#[macro_export]
macro_rules! read_lock_irqsave {
    ($lock:expr, $flags:expr) => {{
        rt_read_lock($lock);
        $flags = 0;
    }};
}

#[inline(always)]
pub unsafe fn read_trylock(lock: *mut rwlock_t) -> bool {
    rt_read_trylock(lock) != 0
}

#[inline(always)]
pub unsafe fn read_unlock(rwlock: *mut rwlock_t) {
    rt_read_unlock(rwlock);
}

#[inline(always)]
pub unsafe fn read_unlock_bh(rwlock: *mut rwlock_t) {
    rt_read_unlock(rwlock);
    local_bh_enable();
}

#[inline(always)]
pub unsafe fn read_unlock_irq(rwlock: *mut rwlock_t) {
    rt_read_unlock(rwlock);
}

#[inline(always)]
pub unsafe fn read_unlock_irqrestore(rwlock: *mut rwlock_t, _flags: libc::c_ulong) {
    rt_read_unlock(rwlock);
}

#[inline(always)]
pub unsafe fn write_lock(rwlock: *mut rwlock_t) {
    rt_write_lock(rwlock);
}

#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
#[inline(always)]
pub unsafe fn write_lock_nested(rwlock: *mut rwlock_t, subclass: core::ffi::c_int) {
    rt_write_lock_nested(rwlock, subclass);
}

#[cfg(not(feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
#[macro_export]
macro_rules! write_lock_nested {
    ($lock:expr, $subclass:expr) => {{
        let _ = $subclass;
        unsafe { rt_write_lock($lock) }
    }};
}

#[inline(always)]
pub unsafe fn write_lock_bh(rwlock: *mut rwlock_t) {
    local_bh_disable();
    rt_write_lock(rwlock);
}

#[inline(always)]
pub unsafe fn write_lock_irq(rwlock: *mut rwlock_t) {
    rt_write_lock(rwlock);
}

#[macro_export]
macro_rules! write_lock_irqsave {
    ($lock:expr, $flags:expr) => {{
        rt_write_lock($lock);
        $flags = 0;
    }};
}

#[inline(always)]
pub unsafe fn write_trylock(lock: *mut rwlock_t) -> bool {
    rt_write_trylock(lock) != 0
}

#[inline(always)]
pub unsafe fn _write_trylock_irqsave(rwlock: *mut rwlock_t, flags: *mut libc::c_ulong) -> bool {
    *flags = 0;
    rt_write_trylock(rwlock) != 0
}

#[macro_export]
macro_rules! write_trylock_irqsave {
    ($lock:expr, $flags:expr) => {{
        _write_trylock_irqsave($lock, &mut $flags)
    }};
}

#[inline(always)]
pub unsafe fn write_unlock(rwlock: *mut rwlock_t) {
    rt_write_unlock(rwlock);
}

#[inline(always)]
pub unsafe fn write_unlock_bh(rwlock: *mut rwlock_t) {
    rt_write_unlock(rwlock);
    local_bh_enable();
}

#[inline(always)]
pub unsafe fn write_unlock_irq(rwlock: *mut rwlock_t) {
    rt_write_unlock(rwlock);
}

#[inline(always)]
pub unsafe fn write_unlock_irqrestore(rwlock: *mut rwlock_t, _flags: libc::c_ulong) {
    rt_write_unlock(rwlock);
}

#[inline(always)]
pub unsafe fn rwlock_is_contended<T>(_lock: *mut T) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
