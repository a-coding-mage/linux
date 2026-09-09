/* Translation of linux/spinlock_api_smp.h. */

/* The following declarations are supplied by the surrounding kernel crate. */
extern "C" {
    pub fn in_lock_functions(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn _raw_spin_lock(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_lock_nested(lock: *mut raw_spinlock_t, subclass: ::core::ffi::c_int);
    pub fn _raw_spin_lock_nest_lock(lock: *mut raw_spinlock_t, map: *mut lockdep_map);
    pub fn _raw_spin_lock_bh(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_lock_irq(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_lock_irq_disable(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_lock_irqsave(lock: *mut raw_spinlock_t) -> ::core::ffi::c_ulong;
    pub fn _raw_spin_lock_irqsave_nested(lock: *mut raw_spinlock_t, subclass: ::core::ffi::c_int)
        -> ::core::ffi::c_ulong;
    pub fn _raw_spin_trylock(lock: *mut raw_spinlock_t) -> ::core::ffi::c_int;
    pub fn _raw_spin_trylock_bh(lock: *mut raw_spinlock_t) -> ::core::ffi::c_int;
    pub fn _raw_spin_unlock(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock_bh(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock_irq(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock_irq_enable(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: ::core::ffi::c_ulong);
}

/* assert_raw_spin_locked(x) expands to BUG_ON(!raw_spin_is_locked(x)). */

/* Configuration-controlled macro aliases are retained by these notes:
 * _raw_spin_{lock,lock_bh,lock_irq,lock_irq_disable,lock_irqsave,trylock,
 * trylock_bh,unlock,unlock_bh,unlock_irq,unlock_irq_enable,unlock_irqrestore}
 * may alias their corresponding __raw_spin_* implementation according to the
 * CONFIG_INLINE_* and CONFIG_UNINLINE_SPIN_UNLOCK build conditions.
 */

/* !CONFIG_GENERIC_LOCKBREAK || CONFIG_DEBUG_LOCK_ALLOC */
#[inline]
pub unsafe fn __raw_spin_trylock(lock: *mut raw_spinlock_t) -> ::core::ffi::c_int {
    preempt_disable();
    if do_raw_spin_trylock(lock) != 0 {
        spin_acquire(&mut (*lock).dep_map, 0, 1, _RET_IP_());
        return 1;
    }
    preempt_enable();
    0
}

#[inline(always)]
pub unsafe fn _raw_spin_trylock_irq(lock: *mut raw_spinlock_t) -> bool {
    local_irq_disable();
    if __raw_spin_trylock(lock) != 0 { true } else {
        local_irq_enable();
        false
    }
}

#[inline(always)]
pub unsafe fn _raw_spin_trylock_irq_disable(lock: *mut raw_spinlock_t) -> bool {
    local_interrupt_disable();
    if __raw_spin_trylock(lock) != 0 { true } else {
        local_interrupt_enable();
        false
    }
}

#[inline(always)]
pub unsafe fn _raw_spin_trylock_irqsave(
    lock: *mut raw_spinlock_t, flags: *mut ::core::ffi::c_ulong,
) -> bool {
    local_irq_save(flags);
    if __raw_spin_trylock(lock) != 0 { true } else {
        local_irq_restore(flags);
        false
    }
}

#[inline]
pub unsafe fn __raw_spin_lock_irqsave(lock: *mut raw_spinlock_t) -> ::core::ffi::c_ulong {
    let mut flags = 0;
    local_irq_save(&mut flags);
    preempt_disable();
    spin_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP_());
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
    flags
}

#[inline]
pub unsafe fn __raw_spin_lock_irq(lock: *mut raw_spinlock_t) {
    local_irq_disable();
    preempt_disable();
    spin_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP_());
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
}

#[inline]
pub unsafe fn __raw_spin_lock_irq_disable(lock: *mut raw_spinlock_t) {
    local_interrupt_disable();
    preempt_disable();
    spin_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP_());
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
}

#[inline]
pub unsafe fn __raw_spin_lock_bh(lock: *mut raw_spinlock_t) {
    __local_bh_disable_ip(_RET_IP_(), SOFTIRQ_LOCK_OFFSET);
    spin_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP_());
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
}

#[inline]
pub unsafe fn __raw_spin_lock(lock: *mut raw_spinlock_t) {
    preempt_disable();
    spin_acquire(&mut (*lock).dep_map, 0, 0, _RET_IP_());
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
}

#[inline]
pub unsafe fn __raw_spin_unlock(lock: *mut raw_spinlock_t) {
    spin_release(&mut (*lock).dep_map, _RET_IP_());
    do_raw_spin_unlock(lock);
    preempt_enable();
}

#[inline]
pub unsafe fn __raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: ::core::ffi::c_ulong) {
    spin_release(&mut (*lock).dep_map, _RET_IP_());
    do_raw_spin_unlock(lock);
    local_irq_restore(flags);
    preempt_enable();
}

#[inline]
pub unsafe fn __raw_spin_unlock_irq(lock: *mut raw_spinlock_t) {
    spin_release(&mut (*lock).dep_map, _RET_IP_());
    do_raw_spin_unlock(lock);
    local_irq_enable();
    preempt_enable();
}

#[inline]
pub unsafe fn __raw_spin_unlock_irq_enable(lock: *mut raw_spinlock_t) {
    spin_release(&mut (*lock).dep_map, _RET_IP_());
    do_raw_spin_unlock(lock);
    local_interrupt_enable();
    preempt_enable();
}

#[inline]
pub unsafe fn __raw_spin_unlock_bh(lock: *mut raw_spinlock_t) {
    spin_release(&mut (*lock).dep_map, _RET_IP_());
    do_raw_spin_unlock(lock);
    __local_bh_enable_ip(_RET_IP_(), SOFTIRQ_LOCK_OFFSET);
}

#[inline]
pub unsafe fn __raw_spin_trylock_bh(lock: *mut raw_spinlock_t) -> ::core::ffi::c_int {
    __local_bh_disable_ip(_RET_IP_(), SOFTIRQ_LOCK_OFFSET);
    if do_raw_spin_trylock(lock) != 0 {
        spin_acquire(&mut (*lock).dep_map, 0, 1, _RET_IP_());
        return 1;
    }
    __local_bh_enable_ip(_RET_IP_(), SOFTIRQ_LOCK_OFFSET);
    0
}

/* PREEMPT_RT has its own rwlock implementation; otherwise include the SMP rwlock API. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
