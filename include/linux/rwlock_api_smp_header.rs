// Translation of linux/rwlock_api_smp.h.
// Required types, functions, macros, and configuration symbols are supplied externally.

extern "C" {
    pub fn _raw_read_lock(lock: *mut rwlock_t);
    pub fn _raw_write_lock(lock: *mut rwlock_t);
    pub fn _raw_write_lock_nested(lock: *mut rwlock_t, subclass: ::core::ffi::c_int);
    pub fn _raw_read_lock_bh(lock: *mut rwlock_t);
    pub fn _raw_write_lock_bh(lock: *mut rwlock_t);
    pub fn _raw_read_lock_irq(lock: *mut rwlock_t);
    pub fn _raw_write_lock_irq(lock: *mut rwlock_t);
    pub fn _raw_read_lock_irqsave(lock: *mut rwlock_t) -> ::core::ffi::c_ulong;
    pub fn _raw_write_lock_irqsave(lock: *mut rwlock_t) -> ::core::ffi::c_ulong;
    pub fn _raw_read_trylock(lock: *mut rwlock_t) -> ::core::ffi::c_int;
    pub fn _raw_write_trylock(lock: *mut rwlock_t) -> ::core::ffi::c_int;
    pub fn _raw_read_unlock(lock: *mut rwlock_t);
    pub fn _raw_write_unlock(lock: *mut rwlock_t);
    pub fn _raw_read_unlock_bh(lock: *mut rwlock_t);
    pub fn _raw_write_unlock_bh(lock: *mut rwlock_t);
    pub fn _raw_read_unlock_irq(lock: *mut rwlock_t);
    pub fn _raw_write_unlock_irq(lock: *mut rwlock_t);
    pub fn _raw_read_unlock_irqrestore(lock: *mut rwlock_t, flags: ::core::ffi::c_ulong);
    pub fn _raw_write_unlock_irqrestore(lock: *mut rwlock_t, flags: ::core::ffi::c_ulong);
}

// #ifdef CONFIG_INLINE_* aliases are build-time macro substitutions in C:
// _raw_read_lock => __raw_read_lock, _raw_write_lock => __raw_write_lock,
// _raw_read_lock_bh => __raw_read_lock_bh, _raw_write_lock_bh => __raw_write_lock_bh,
// _raw_read_lock_irq => __raw_read_lock_irq, _raw_write_lock_irq => __raw_write_lock_irq,
// _raw_read_lock_irqsave => __raw_read_lock_irqsave,
// _raw_write_lock_irqsave => __raw_write_lock_irqsave,
// _raw_read_trylock => __raw_read_trylock, _raw_write_trylock => __raw_write_trylock,
// _raw_read_unlock => __raw_read_unlock, _raw_write_unlock => __raw_write_unlock,
// _raw_read_unlock_bh => __raw_read_unlock_bh, _raw_write_unlock_bh => __raw_write_unlock_bh,
// _raw_read_unlock_irq => __raw_read_unlock_irq,
// _raw_write_unlock_irq => __raw_write_unlock_irq,
// _raw_read_unlock_irqrestore => __raw_read_unlock_irqrestore,
// _raw_write_unlock_irqrestore => __raw_write_unlock_irqrestore.

pub unsafe fn __raw_read_trylock(lock: *mut rwlock_t) -> ::core::ffi::c_int {
    preempt_disable();
    if do_raw_read_trylock(lock) != 0 {
        rwlock_acquire_read(&mut (*lock).dep_map, 0, 1, _RET_IP!());
        return 1;
    }
    preempt_enable();
    0
}

pub unsafe fn __raw_write_trylock(lock: *mut rwlock_t) -> ::core::ffi::c_int {
    preempt_disable();
    if do_raw_write_trylock(lock) != 0 {
        rwlock_acquire(&mut (*lock).dep_map, 0, 1, _RET_IP!());
        return 1;
    }
    preempt_enable();
    0
}

pub unsafe fn _raw_write_trylock_irqsave(lock: *mut rwlock_t, flags: *mut ::core::ffi::c_ulong) -> bool {
    local_irq_save(flags);
    if _raw_write_trylock(lock) != 0 { true } else {
        local_irq_restore(*flags);
        false
    }
}

// #if !defined(CONFIG_GENERIC_LOCKBREAK) || defined(CONFIG_DEBUG_LOCK_ALLOC)
pub unsafe fn __raw_read_lock(lock: *mut rwlock_t) {
    preempt_disable();
    rwlock_acquire_read(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_read_trylock, do_raw_read_lock);
}

pub unsafe fn __raw_read_lock_irqsave(lock: *mut rwlock_t) -> ::core::ffi::c_ulong {
    let mut flags = 0;
    local_irq_save(&mut flags);
    preempt_disable();
    rwlock_acquire_read(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_read_trylock, do_raw_read_lock);
    flags
}

pub unsafe fn __raw_read_lock_irq(lock: *mut rwlock_t) {
    local_irq_disable(); preempt_disable();
    rwlock_acquire_read(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_read_trylock, do_raw_read_lock);
}

pub unsafe fn __raw_read_lock_bh(lock: *mut rwlock_t) {
    __local_bh_disable_ip(_RET_IP!(), SOFTIRQ_LOCK_OFFSET);
    rwlock_acquire_read(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_read_trylock, do_raw_read_lock);
}

pub unsafe fn __raw_write_lock_irqsave(lock: *mut rwlock_t) -> ::core::ffi::c_ulong {
    let mut flags = 0;
    local_irq_save(&mut flags); preempt_disable();
    rwlock_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_write_trylock, do_raw_write_lock);
    flags
}

pub unsafe fn __raw_write_lock_irq(lock: *mut rwlock_t) {
    local_irq_disable(); preempt_disable();
    rwlock_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_write_trylock, do_raw_write_lock);
}

pub unsafe fn __raw_write_lock_bh(lock: *mut rwlock_t) {
    __local_bh_disable_ip(_RET_IP!(), SOFTIRQ_LOCK_OFFSET);
    rwlock_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_write_trylock, do_raw_write_lock);
}

pub unsafe fn __raw_write_lock(lock: *mut rwlock_t) {
    preempt_disable(); rwlock_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_write_trylock, do_raw_write_lock);
}

pub unsafe fn __raw_write_lock_nested(lock: *mut rwlock_t, subclass: ::core::ffi::c_int) {
    preempt_disable(); rwlock_acquire(&mut (*lock).dep_map, subclass, 0, _RET_IP!());
    LOCK_CONTENDED!(lock, do_raw_write_trylock, do_raw_write_lock);
}
// #endif

pub unsafe fn __raw_write_unlock(lock: *mut rwlock_t) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_write_unlock(lock); preempt_enable();
}
pub unsafe fn __raw_read_unlock(lock: *mut rwlock_t) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_read_unlock(lock); preempt_enable();
}
pub unsafe fn __raw_read_unlock_irqrestore(lock: *mut rwlock_t, flags: ::core::ffi::c_ulong) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_read_unlock(lock); local_irq_restore(flags); preempt_enable();
}
pub unsafe fn __raw_read_unlock_irq(lock: *mut rwlock_t) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_read_unlock(lock); local_irq_enable(); preempt_enable();
}
pub unsafe fn __raw_read_unlock_bh(lock: *mut rwlock_t) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_read_unlock(lock); __local_bh_enable_ip(_RET_IP!(), SOFTIRQ_LOCK_OFFSET);
}
pub unsafe fn __raw_write_unlock_irqrestore(lock: *mut rwlock_t, flags: ::core::ffi::c_ulong) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_write_unlock(lock); local_irq_restore(flags); preempt_enable();
}
pub unsafe fn __raw_write_unlock_irq(lock: *mut rwlock_t) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_write_unlock(lock); local_irq_enable(); preempt_enable();
}
pub unsafe fn __raw_write_unlock_bh(lock: *mut rwlock_t) {
    rwlock_release(&mut (*lock).dep_map, _RET_IP!()); do_raw_write_unlock(lock); __local_bh_enable_ip(_RET_IP!(), SOFTIRQ_LOCK_OFFSET);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
