// SPDX-License-Identifier: GPL-2.0-only
// C header guard: __LINUX_SPINLOCK_RT_H
// This header is intended to be included from spinlock.h (__LINUX_INSIDE_SPINLOCK_H).

#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
extern "C" {
    pub fn __rt_spin_lock_init(
        lock: *mut spinlock_t,
        name: *const ::std::os::raw::c_char,
        key: *mut lock_class_key,
        percpu: bool,
    );
}

#[cfg(not(feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
#[inline]
pub unsafe fn __rt_spin_lock_init(
    _lock: *mut spinlock_t,
    _name: *const ::std::os::raw::c_char,
    _key: *mut lock_class_key,
    _percpu: bool,
) {
}

#[inline]
pub unsafe fn __spin_lock_init(
    slock: *mut spinlock_t,
    name: *const ::std::os::raw::c_char,
    key: *mut lock_class_key,
    percpu: bool,
) {
    rt_mutex_base_init(&mut (*slock).lock);
    __rt_spin_lock_init(slock, name, key, percpu);
}

#[inline]
pub unsafe fn _spin_lock_init(slock: *mut spinlock_t, percpu: bool) {
    static mut __KEY: lock_class_key = lock_class_key { _unused: 0 };
    // C macro stringification of slock is not representable from a pointer alone.
    __spin_lock_init(slock, ::std::ptr::null(), &raw mut __KEY, percpu);
}

#[inline]
pub unsafe fn spin_lock_init(slock: *mut spinlock_t) {
    _spin_lock_init(slock, false);
}

#[inline]
pub unsafe fn local_spin_lock_init(slock: *mut spinlock_t) {
    _spin_lock_init(slock, true);
}

extern "C" {
    pub fn rt_spin_lock(lock: *mut spinlock_t);
    pub fn rt_spin_lock_nested(lock: *mut spinlock_t, subclass: ::std::os::raw::c_int);
    pub fn rt_spin_lock_nest_lock(lock: *mut spinlock_t, nest_lock: *mut lockdep_map);
    pub fn rt_spin_unlock(lock: *mut spinlock_t);
    pub fn rt_spin_lock_unlock(lock: *mut spinlock_t);
    pub fn rt_spin_trylock_bh(lock: *mut spinlock_t) -> ::std::os::raw::c_int;
    pub fn rt_spin_trylock(lock: *mut spinlock_t) -> ::std::os::raw::c_int;
}

#[inline]
pub unsafe fn spin_lock(lock: *mut spinlock_t) {
    rt_spin_lock(lock);
}

#[cfg(feature = "CONFIG_LOCKDEP")]
#[inline]
pub unsafe fn __spin_lock_nested(lock: *mut spinlock_t, subclass: ::std::os::raw::c_int) {
    rt_spin_lock_nested(lock, subclass);
}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[inline]
pub unsafe fn __spin_lock_nested(lock: *mut spinlock_t, subclass: ::std::os::raw::c_int) {
    let _ = subclass;
    spin_lock(lock);
}

#[cfg(feature = "CONFIG_LOCKDEP")]
#[inline]
pub unsafe fn __spin_lock_nest_lock(lock: *mut spinlock_t, nest_lock: *mut lockdep_map) {
    rt_spin_lock_nest_lock(lock, nest_lock);
}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[inline]
pub unsafe fn __spin_lock_nest_lock(lock: *mut spinlock_t, subclass: ::std::os::raw::c_int) {
    let _ = subclass;
    spin_lock(lock);
}

#[inline]
pub unsafe fn spin_lock_nested(lock: *mut spinlock_t, subclass: ::std::os::raw::c_int) {
    __spin_lock_nested(lock, subclass);
}

#[inline]
pub unsafe fn spin_lock_nest_lock(lock: *mut spinlock_t, nest_lock: *mut lockdep_map) {
    __spin_lock_nest_lock(lock, nest_lock);
}

#[inline]
pub unsafe fn spin_lock_irqsave_nested(
    lock: *mut spinlock_t,
    flags: *mut ::std::os::raw::c_ulong,
    subclass: ::std::os::raw::c_int,
) {
    *flags = 0;
    __spin_lock_nested(lock, subclass);
}

#[inline]
pub unsafe fn spin_lock_bh(lock: *mut spinlock_t) {
    // Investigate: Drop bh when blocking ?
    local_bh_disable();
    rt_spin_lock(lock);
}

#[inline]
pub unsafe fn spin_lock_irq(lock: *mut spinlock_t) { rt_spin_lock(lock); }

#[inline]
pub unsafe fn spin_lock_irq_disable(lock: *mut spinlock_t) { rt_spin_lock(lock); }

#[inline]
pub unsafe fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut ::std::os::raw::c_ulong) {
    *flags = 0;
    spin_lock(lock);
}

#[inline]
pub unsafe fn spin_unlock(lock: *mut spinlock_t) { rt_spin_unlock(lock); }

#[inline]
pub unsafe fn spin_unlock_bh(lock: *mut spinlock_t) {
    rt_spin_unlock(lock);
    local_bh_enable();
}

#[inline]
pub unsafe fn spin_unlock_irq(lock: *mut spinlock_t) { rt_spin_unlock(lock); }

#[inline]
pub unsafe fn spin_unlock_irq_enable(lock: *mut spinlock_t) { rt_spin_unlock(lock); }

#[inline]
pub unsafe fn spin_unlock_irqrestore(
    lock: *mut spinlock_t,
    _flags: ::std::os::raw::c_ulong,
) { rt_spin_unlock(lock); }

#[inline]
pub unsafe fn spin_trylock(lock: *mut spinlock_t) -> ::std::os::raw::c_int {
    rt_spin_trylock(lock)
}

#[inline]
pub unsafe fn spin_trylock_irq_disable(lock: *mut spinlock_t) -> ::std::os::raw::c_int {
    rt_spin_trylock(lock)
}

#[inline]
pub unsafe fn spin_trylock_bh(lock: *mut spinlock_t) -> ::std::os::raw::c_int {
    rt_spin_trylock_bh(lock)
}

#[inline]
pub unsafe fn spin_trylock_irq(lock: *mut spinlock_t) -> ::std::os::raw::c_int {
    rt_spin_trylock(lock)
}

#[inline]
pub unsafe fn _spin_trylock_irqsave(
    lock: *mut spinlock_t,
    flags: *mut ::std::os::raw::c_ulong,
) -> bool {
    *flags = 0;
    rt_spin_trylock(lock) != 0
}

#[inline]
pub unsafe fn spin_trylock_irqsave(
    lock: *mut spinlock_t,
    flags: *mut ::std::os::raw::c_ulong,
) -> bool {
    _spin_trylock_irqsave(lock, flags)
}

#[inline]
pub unsafe fn spin_is_contended(lock: *mut spinlock_t) -> ::std::os::raw::c_int {
    let _ = lock;
    0
}

#[inline]
pub unsafe fn spin_is_locked(lock: *mut spinlock_t) -> ::std::os::raw::c_int {
    rt_mutex_base_is_locked(&mut (*lock).lock)
}

#[inline]
pub unsafe fn assert_spin_locked(lock: *mut spinlock_t) {
    if spin_is_locked(lock) == 0 {
        BUG_ON(true);
    }
}

// Dependency supplied by linux/rwlock_rt.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
