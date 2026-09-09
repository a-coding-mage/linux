// SPDX-License-Identifier: GPL-2.0
/*
 * Rust translation of spinlock.c.  Kernel configuration conditionals and
 * externally supplied kernel declarations are preserved as comments.
 */

// CONFIG_MMIOWB: per-CPU __mmiowb_state is supplied by the kernel build.
// The __lock_function inlines are emitted only when
// !CONFIG_GENERIC_LOCKBREAK || CONFIG_DEBUG_LOCK_ALLOC.

#[allow(non_snake_case, non_camel_case_types, dead_code)]
unsafe fn __raw_spin_lock_irq_disable(lock: *mut raw_spinlock_t) {
    loop {
        preempt_disable();
        local_interrupt_disable();
        if do_raw_spin_trylock(lock) != 0 {
            break;
        }
        local_interrupt_enable();
        preempt_enable();
        arch_spin_relax(unsafe { &mut (*lock).raw_lock });
    }
}

unsafe fn __raw_spin_lock(lock: *mut raw_spinlock_t) {
    loop {
        preempt_disable();
        if do_raw_spin_trylock(lock) != 0 { break; }
        preempt_enable();
        arch_spin_relax(&mut (*lock).raw_lock);
    }
}
unsafe fn __raw_spin_lock_irqsave(lock: *mut raw_spinlock_t) -> c_ulong {
    loop {
        preempt_disable();
        let mut flags: c_ulong = 0;
        local_irq_save(&mut flags);
        if do_raw_spin_trylock(lock) != 0 { return flags; }
        local_irq_restore(flags);
        preempt_enable();
        arch_spin_relax(&mut (*lock).raw_lock);
    }
}
unsafe fn __raw_spin_lock_irq(lock: *mut raw_spinlock_t) { __raw_spin_lock_irqsave(lock); }
unsafe fn __raw_spin_lock_bh(lock: *mut raw_spinlock_t) {
    let flags = __raw_spin_lock_irqsave(lock);
    local_bh_disable();
    local_irq_restore(flags);
}

// The following wrappers correspond to the C noinline/exported entry points.
#[inline(never)]
pub unsafe fn _raw_spin_trylock(lock: *mut raw_spinlock_t) -> i32 { __raw_spin_trylock(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_trylock_bh(lock: *mut raw_spinlock_t) -> i32 { __raw_spin_trylock_bh(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_lock(lock: *mut raw_spinlock_t) { __raw_spin_lock(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_lock_irqsave(lock: *mut raw_spinlock_t) -> c_ulong { __raw_spin_lock_irqsave(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_lock_irq(lock: *mut raw_spinlock_t) { __raw_spin_lock_irq(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_lock_irq_disable(lock: *mut raw_spinlock_t) { __raw_spin_lock_irq_disable(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_lock_bh(lock: *mut raw_spinlock_t) { __raw_spin_lock_bh(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_unlock(lock: *mut raw_spinlock_t) { __raw_spin_unlock(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong) { __raw_spin_unlock_irqrestore(lock, flags) }
#[inline(never)]
pub unsafe fn _raw_spin_unlock_irq(lock: *mut raw_spinlock_t) { __raw_spin_unlock_irq(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_unlock_irq_enable(lock: *mut raw_spinlock_t) { __raw_spin_unlock_irq_enable(lock) }
#[inline(never)]
pub unsafe fn _raw_spin_unlock_bh(lock: *mut raw_spinlock_t) { __raw_spin_unlock_bh(lock) }

// BUILD_LOCK_OPS(read, rwlock, __acquires_shared) and
// BUILD_LOCK_OPS(write, rwlock, __acquires), omitted on CONFIG_PREEMPT_RT.
unsafe fn __raw_read_lock(lock: *mut rwlock_t) {
    loop {
        preempt_disable();
        if do_raw_read_trylock(lock) != 0 { break; }
        preempt_enable();
        arch_read_relax(&mut (*lock).raw_lock);
    }
}
unsafe fn __raw_write_lock(lock: *mut rwlock_t) {
    loop {
        preempt_disable();
        if do_raw_write_trylock(lock) != 0 { break; }
        preempt_enable();
        arch_write_relax(&mut (*lock).raw_lock);
    }
}

// !CONFIG_PREEMPT_RT: rwlock entry points.
#[inline(never)]
pub unsafe fn _raw_read_trylock(lock: *mut rwlock_t) -> i32 { __raw_read_trylock(lock) }
#[inline(never)]
pub unsafe fn _raw_read_lock(lock: *mut rwlock_t) { __raw_read_lock(lock) }
#[inline(never)]
pub unsafe fn _raw_read_lock_irqsave(lock: *mut rwlock_t) -> c_ulong { __raw_read_lock_irqsave(lock) }
#[inline(never)]
pub unsafe fn _raw_read_lock_irq(lock: *mut rwlock_t) { __raw_read_lock_irq(lock) }
#[inline(never)]
pub unsafe fn _raw_read_lock_bh(lock: *mut rwlock_t) { __raw_read_lock_bh(lock) }
#[inline(never)]
pub unsafe fn _raw_read_unlock(lock: *mut rwlock_t) { __raw_read_unlock(lock) }
#[inline(never)]
pub unsafe fn _raw_read_unlock_irqrestore(lock: *mut rwlock_t, flags: c_ulong) { __raw_read_unlock_irqrestore(lock, flags) }
#[inline(never)]
pub unsafe fn _raw_read_unlock_irq(lock: *mut rwlock_t) { __raw_read_unlock_irq(lock) }
#[inline(never)]
pub unsafe fn _raw_read_unlock_bh(lock: *mut rwlock_t) { __raw_read_unlock_bh(lock) }
#[inline(never)]
pub unsafe fn _raw_write_trylock(lock: *mut rwlock_t) -> i32 { __raw_write_trylock(lock) }
#[inline(never)]
pub unsafe fn _raw_write_lock(lock: *mut rwlock_t) { __raw_write_lock(lock) }
#[inline(never)]
pub unsafe fn _raw_write_lock_irqsave(lock: *mut rwlock_t) -> c_ulong { __raw_write_lock_irqsave(lock) }
#[inline(never)]
pub unsafe fn _raw_write_lock_irq(lock: *mut rwlock_t) { __raw_write_lock_irq(lock) }
#[inline(never)]
pub unsafe fn _raw_write_lock_bh(lock: *mut rwlock_t) { __raw_write_lock_bh(lock) }
#[inline(never)]
pub unsafe fn _raw_write_unlock(lock: *mut rwlock_t) { __raw_write_unlock(lock) }
#[inline(never)]
pub unsafe fn _raw_write_unlock_irqrestore(lock: *mut rwlock_t, flags: c_ulong) { __raw_write_unlock_irqrestore(lock, flags) }
#[inline(never)]
pub unsafe fn _raw_write_unlock_irq(lock: *mut rwlock_t) { __raw_write_unlock_irq(lock) }
#[inline(never)]
pub unsafe fn _raw_write_unlock_bh(lock: *mut rwlock_t) { __raw_write_unlock_bh(lock) }

pub unsafe fn _raw_write_lock_nested(lock: *mut rwlock_t, subclass: i32) {
    __raw_write_lock_nested(lock, subclass);
}

pub unsafe fn in_lock_functions(addr: c_ulong) -> i32 {
    extern "C" {
        static __lock_text_start: u8;
        static __lock_text_end: u8;
    }
    let start = &__lock_text_start as *const u8 as c_ulong;
    let end = &__lock_text_end as *const u8 as c_ulong;
    (addr >= start && addr < end) as i32
}

// CONFIG_DEBUG_LOCK_ALLOC entry points.
pub unsafe fn _raw_spin_lock_nested(lock: *mut raw_spinlock_t, subclass: i32) {
    preempt_disable();
    spin_acquire(&mut (*lock).dep_map, subclass, 0, _RET_IP_);
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
}

pub unsafe fn _raw_spin_lock_irqsave_nested(lock: *mut raw_spinlock_t, subclass: i32) -> c_ulong {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    preempt_disable();
    spin_acquire(&mut (*lock).dep_map, subclass, 0, _RET_IP_);
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
    flags
}

pub unsafe fn _raw_spin_lock_nest_lock(lock: *mut raw_spinlock_t, nest_lock: *mut lockdep_map) {
    preempt_disable();
    spin_acquire_nest(&mut (*lock).dep_map, 0, 0, nest_lock, _RET_IP_);
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
}

// CONFIG_PROVE_LOCKING && CONFIG_PREEMPT_RT
pub unsafe fn lockdep_assert_in_softirq_func() { lockdep_assert_in_softirq(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
