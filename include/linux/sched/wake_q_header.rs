/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Wake-queues are lists of tasks with a pending wakeup, whose
 * callers have already marked the task as woken internally,
 * and can thus carry on. A common use case is being able to
 * do the wakeups once the corresponding user lock as been
 * released.
 *
 * We hold reference to each task in the list across the wakeup,
 * thus guaranteeing that the memory is still valid by the time
 * the actual wakeups are performed in wake_up_q().
 *
 * One per task suffices, because there's never a need for a task to be
 * in two wake queues simultaneously; it is forbidden to abandon a task
 * in a wake queue (a call to wake_up_q() _must_ follow), so if a task is
 * already in a wake queue, the wakeup will happen soon and the second
 * waker can just skip it.
 *
 * The DEFINE_WAKE_Q macro declares and initializes the list head.
 * wake_up_q() does NOT reinitialize the list; it's expected to be
 * called near the end of a function. Otherwise, the list can be
 * re-initialized for later re-use by wake_q_init().
 *
 * NOTE that this can cause spurious wakeups. schedule() callers
 * must ensure the call is done inside a loop, confirming that the
 * wakeup condition has in fact occurred.
 *
 * NOTE that there is no guarantee the wakeup will happen any later than the
 * wake_q_add() location. Therefore task must be ready to be woken at the
 * location of the wake_q_add().
 */

#[repr(C)]
pub struct wake_q_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wake_q_head {
    pub first: *mut wake_q_node,
    pub lastp: *mut *mut wake_q_node,
}

pub const WAKE_Q_TAIL: *mut wake_q_node = 0x01usize as *mut wake_q_node;

pub const fn wake_q_head_initializer(name: &wake_q_head) -> wake_q_head {
    wake_q_head {
        first: WAKE_Q_TAIL,
        lastp: &name.first as *const *mut wake_q_node as *mut *mut wake_q_node,
    }
}

pub unsafe fn wake_q_init(head: *mut wake_q_head) {
    (*head).first = WAKE_Q_TAIL;
    (*head).lastp = &mut (*head).first;
}

pub unsafe fn wake_q_empty(head: *mut wake_q_head) -> bool {
    (*head).first == WAKE_Q_TAIL
}

extern "C" {
    pub fn wake_q_add(head: *mut wake_q_head, task: *mut task_struct);
    pub fn wake_q_add_safe(head: *mut wake_q_head, task: *mut task_struct);
    pub fn wake_up_q(head: *mut wake_q_head);

    pub fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    pub fn raw_spin_unlock_irq(lock: *mut raw_spinlock_t);
    pub fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: u64);
    /* C guard(preempt)() is an RAII preemption guard; supplied externally. */
    pub fn guard_preempt();
}

/* Spin unlock helpers to unlock and call wake_up_q with preempt disabled. */
pub unsafe fn raw_spin_unlock_wake(
    lock: *mut raw_spinlock_t,
    wake_q: *mut wake_q_head,
) {
    guard_preempt();
    raw_spin_unlock(lock);
    if !wake_q.is_null() {
        wake_up_q(wake_q);
        wake_q_init(wake_q);
    }
}

pub unsafe fn raw_spin_unlock_irq_wake(
    lock: *mut raw_spinlock_t,
    wake_q: *mut wake_q_head,
) {
    guard_preempt();
    raw_spin_unlock_irq(lock);
    if !wake_q.is_null() {
        wake_up_q(wake_q);
        wake_q_init(wake_q);
    }
}

pub unsafe fn raw_spin_unlock_irqrestore_wake(
    lock: *mut raw_spinlock_t,
    flags: u64,
    wake_q: *mut wake_q_head,
) {
    guard_preempt();
    raw_spin_unlock_irqrestore(lock, flags);
    if !wake_q.is_null() {
        wake_up_q(wake_q);
        wake_q_init(wake_q);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
