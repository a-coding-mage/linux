// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008 Intel Corporation
 * Author: Matthew Wilcox <willy@linux.intel.com>
 *
 * This file implements counting semaphores.
 * A counting semaphore may be acquired 'n' times before sleeping.
 * See mutex.c for single-acquisition sleeping locks which enforce
 * rules which allow code to be debugged more easily.
 */

/*
 * Some notes on the implementation:
 *
 * The spinlock controls access to the other members of the semaphore.
 * down_trylock() and up() can be called from interrupt context, so we
 * have to disable interrupts when taking the lock.  It turns out various
 * parts of the kernel expect to be able to use down() on a semaphore in
 * interrupt context when they know it will succeed, so we have to use
 * irqsave variants for down(), down_interruptible() and down_killable()
 * too.
 *
 * The ->count variable represents how many more tasks can acquire this
 * semaphore.  If it's zero, there may be waiters.
 */

// External kernel types, functions, constants, and macros are supplied by other files.

extern "C" {
    fn __down(sem: *mut semaphore);
    fn __down_interruptible(sem: *mut semaphore) -> i32;
    fn __down_killable(sem: *mut semaphore) -> i32;
    fn __down_timeout(sem: *mut semaphore, timeout: i64) -> i32;
    fn __up(sem: *mut semaphore, wake_q: *mut wake_q_head);
}

#[repr(C)]
pub struct semaphore {
    pub lock: raw_spinlock,
    pub count: i32,
    pub first_waiter: *mut semaphore_waiter,
    pub last_holder: usize,
}

#[repr(C)]
pub struct raw_spinlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wake_q_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct semaphore_waiter {
    pub list: list_head,
    pub task: *mut task_struct,
    pub up: bool,
}

#[inline]
unsafe fn hung_task_sem_set_holder(_sem: *mut semaphore) {}

#[inline]
unsafe fn hung_task_sem_clear_if_holder(_sem: *mut semaphore) {}

pub unsafe fn sem_last_holder(_sem: *mut semaphore) -> usize {
    0usize
}

#[inline]
unsafe fn __sem_acquire(sem: *mut semaphore) {
    (*sem).count -= 1;
    hung_task_sem_set_holder(sem);
}

/// down - acquire the semaphore
/// @sem: the semaphore to be acquired
///
/// Acquires the semaphore.  If no more tasks are allowed to acquire the
/// semaphore, calling this function will put the task to sleep until the
/// semaphore is released.
///
/// Use of this function is deprecated, please use down_interruptible() or
/// down_killable() instead.
pub unsafe fn down(sem: *mut semaphore) {
    let mut flags: usize = 0;

    might_sleep();
    raw_spin_lock_irqsave(&mut (*sem).lock, &mut flags);
    if (*sem).count > 0 {
        __sem_acquire(sem);
    } else {
        __down(sem);
    }
    raw_spin_unlock_irqrestore(&mut (*sem).lock, flags);
}

/// down_interruptible - acquire the semaphore unless interrupted
pub unsafe fn down_interruptible(sem: *mut semaphore) -> i32 {
    let mut flags: usize = 0;
    let mut result: i32 = 0;

    might_sleep();
    raw_spin_lock_irqsave(&mut (*sem).lock, &mut flags);
    if (*sem).count > 0 {
        __sem_acquire(sem);
    } else {
        result = __down_interruptible(sem);
    }
    raw_spin_unlock_irqrestore(&mut (*sem).lock, flags);
    result
}

/// down_killable - acquire the semaphore unless killed
pub unsafe fn down_killable(sem: *mut semaphore) -> i32 {
    let mut flags: usize = 0;
    let mut result: i32 = 0;

    might_sleep();
    raw_spin_lock_irqsave(&mut (*sem).lock, &mut flags);
    if (*sem).count > 0 {
        __sem_acquire(sem);
    } else {
        result = __down_killable(sem);
    }
    raw_spin_unlock_irqrestore(&mut (*sem).lock, flags);
    result
}

/// down_trylock - try to acquire the semaphore, without waiting
pub unsafe fn down_trylock(sem: *mut semaphore) -> i32 {
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*sem).lock, &mut flags);
    let count = (*sem).count - 1;
    if count >= 0 {
        __sem_acquire(sem);
    }
    raw_spin_unlock_irqrestore(&mut (*sem).lock, flags);
    (count < 0) as i32
}

/// down_timeout - acquire the semaphore within a specified time
pub unsafe fn down_timeout(sem: *mut semaphore, timeout: i64) -> i32 {
    let mut flags: usize = 0;
    let mut result: i32 = 0;

    might_sleep();
    raw_spin_lock_irqsave(&mut (*sem).lock, &mut flags);
    if (*sem).count > 0 {
        __sem_acquire(sem);
    } else {
        result = __down_timeout(sem, timeout);
    }
    raw_spin_unlock_irqrestore(&mut (*sem).lock, flags);
    result
}

/// up - release the semaphore
pub unsafe fn up(sem: *mut semaphore) {
    let mut flags: usize = 0;
    let mut wake_q = wake_q_head { _private: [] };

    raw_spin_lock_irqsave(&mut (*sem).lock, &mut flags);
    hung_task_sem_clear_if_holder(sem);

    if (*sem).first_waiter.is_null() {
        (*sem).count += 1;
    } else {
        __up(sem, &mut wake_q);
    }

    if trace_contended_release_enabled() && !wake_q_empty(&wake_q) {
        trace_call__contended_release(sem);
    }

    raw_spin_unlock_irqrestore(&mut (*sem).lock, flags);
    if !wake_q_empty(&wake_q) {
        wake_up_q(&mut wake_q);
    }
}

#[inline]
unsafe fn sem_del_waiter(sem: *mut semaphore, waiter: *mut semaphore_waiter) {
    if list_empty(&mut (*waiter).list) {
        (*sem).first_waiter = core::ptr::null_mut();
        return;
    }

    if (*sem).first_waiter == waiter {
        (*sem).first_waiter = list_first_entry(&mut (*waiter).list);
    }
    list_del(&mut (*waiter).list);
}

#[inline]
unsafe fn ___down_common(sem: *mut semaphore, state: i64, mut timeout: i64) -> i32 {
    let mut waiter = semaphore_waiter {
        list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
        task: core::ptr::null_mut(),
        up: false,
    };
    let first = (*sem).first_waiter;

    if !first.is_null() {
        list_add_tail(&mut waiter.list, &mut (*first).list);
    } else {
        init_list_head(&mut waiter.list);
        (*sem).first_waiter = &mut waiter;
    }
    waiter.task = current();
    waiter.up = false;

    loop {
        if signal_pending_state(state, current()) != 0 {
            sem_del_waiter(sem, &mut waiter);
            return -4;
        }
        if timeout <= 0 {
            sem_del_waiter(sem, &mut waiter);
            return -62;
        }
        set_current_state(state);
        raw_spin_unlock_irq(&mut (*sem).lock);
        timeout = schedule_timeout(timeout);
        raw_spin_lock_irq(&mut (*sem).lock);
        if waiter.up {
            hung_task_sem_set_holder(sem);
            return 0;
        }
    }
}

#[inline]
unsafe fn __down_common(sem: *mut semaphore, state: i64, timeout: i64) -> i32 {
    hung_task_set_blocker(sem, 0);
    trace_contention_begin(sem, 0);
    let ret = ___down_common(sem, state, timeout);
    trace_contention_end(sem, ret);
    hung_task_clear_blocker();
    ret
}

unsafe fn __down(sem: *mut semaphore) {
    __down_common(sem, 0,  i64::MAX);
}

unsafe fn __down_interruptible(sem: *mut semaphore) -> i32 {
    __down_common(sem, 1, i64::MAX)
}

unsafe fn __down_killable(sem: *mut semaphore) -> i32 {
    __down_common(sem, 2, i64::MAX)
}

unsafe fn __down_timeout(sem: *mut semaphore, timeout: i64) -> i32 {
    __down_common(sem, 0, timeout)
}

unsafe fn __up(sem: *mut semaphore, wake_q: *mut wake_q_head) {
    let waiter = (*sem).first_waiter;
    sem_del_waiter(sem, waiter);
    (*waiter).up = true;
    wake_q_add(wake_q, (*waiter).task);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
