/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/local_lock_internal.h>
// The symbols referenced below are supplied by that dependency.

/// local_lock_init - Runtime initialize a lock instance
/// @lock: The lock variable
#[macro_export]
macro_rules! local_lock_init {
    ($lock:expr) => { __local_lock_init($lock) };
}

/// local_lock - Acquire a per CPU local lock
#[macro_export]
macro_rules! local_lock {
    ($lock:expr) => { __local_lock(__this_cpu_local_lock($lock)) };
}

/// local_lock_irq - Acquire a per CPU local lock and disable interrupts
#[macro_export]
macro_rules! local_lock_irq {
    ($lock:expr) => { __local_lock_irq(__this_cpu_local_lock($lock)) };
}

/// local_lock_irqsave - Acquire a per CPU local lock, save and disable interrupts
#[macro_export]
macro_rules! local_lock_irqsave {
    ($lock:expr, $flags:expr) => {
        __local_lock_irqsave(__this_cpu_local_lock($lock), $flags)
    };
}

/// local_unlock - Release a per CPU local lock
#[macro_export]
macro_rules! local_unlock {
    ($lock:expr) => { __local_unlock(__this_cpu_local_lock($lock)) };
}

/// local_unlock_irq - Release a per CPU local lock and enable interrupts
#[macro_export]
macro_rules! local_unlock_irq {
    ($lock:expr) => { __local_unlock_irq(__this_cpu_local_lock($lock)) };
}

/// local_unlock_irqrestore - Release a per CPU local lock and restore interrupt flags
#[macro_export]
macro_rules! local_unlock_irqrestore {
    ($lock:expr, $flags:expr) => {
        __local_unlock_irqrestore(__this_cpu_local_lock($lock), $flags)
    };
}

/// local_trylock_init - Runtime initialize a lock instance
#[macro_export]
macro_rules! local_trylock_init {
    ($lock:expr) => { __local_trylock_init($lock) };
}

/// local_trylock - Try to acquire a per CPU local lock.
/// This can be used in any context such as NMI or HARDIRQ. Due to locking
/// constraints it always fails to acquire the lock in those contexts on PREEMPT_RT.
#[macro_export]
macro_rules! local_trylock {
    ($lock:expr) => { __local_trylock(__this_cpu_local_lock($lock)) };
}

#[macro_export]
macro_rules! local_lock_is_locked {
    ($lock:expr) => { __local_lock_is_locked($lock) };
}

/// Try to acquire a per CPU local lock, saving and disabling interrupts if acquired.
#[macro_export]
macro_rules! local_trylock_irqsave {
    ($lock:expr, $flags:expr) => {
        __local_trylock_irqsave(__this_cpu_local_lock($lock), $flags)
    };
}

#[macro_export]
macro_rules! local_lock_nested_bh {
    ($lock:expr) => { __local_lock_nested_bh(__this_cpu_local_lock($lock)) };
}

#[macro_export]
macro_rules! local_unlock_nested_bh {
    ($lock:expr) => { __local_unlock_nested_bh(__this_cpu_local_lock($lock)) };
}

// C lock-guard declarations. Their implementations and attribute semantics are
// supplied by the kernel lock-guard infrastructure.
// DEFINE_LOCK_GUARD_1(local_lock, local_lock_t __percpu,
//     local_lock(_T->lock), local_unlock(_T->lock))
// DEFINE_LOCK_GUARD_1(local_lock_irq, local_lock_t __percpu,
//     local_lock_irq(_T->lock), local_unlock_irq(_T->lock))
// DEFINE_LOCK_GUARD_1(local_lock_irqsave, local_lock_t __percpu,
//     local_lock_irqsave(_T->lock, _T->flags),
//     local_unlock_irqrestore(_T->lock, _T->flags), unsigned long flags)
// DEFINE_LOCK_GUARD_1(local_lock_nested_bh, local_lock_t __percpu,
//     local_lock_nested_bh(_T->lock), local_unlock_nested_bh(_T->lock))
// DEFINE_LOCK_GUARD_1(local_lock_init, local_lock_t, local_lock_init(_T->lock), /* */)
// DEFINE_LOCK_GUARD_1(local_trylock_init, local_trylock_t,
//     local_trylock_init(_T->lock), /* */)

// DECLARE_LOCK_GUARD_1_ATTRS and WITH_LOCK_GUARD_1_ATTRS are C kernel
// infrastructure declarations; preserve their constructor definitions here.
// class_local_lock_constructor(_T) = WITH_LOCK_GUARD_1_ATTRS(local_lock, _T)
// class_local_lock_irq_constructor(_T) = WITH_LOCK_GUARD_1_ATTRS(local_lock_irq, _T)
// class_local_lock_irqsave_constructor(_T) = WITH_LOCK_GUARD_1_ATTRS(local_lock_irqsave, _T)
// class_local_lock_nested_bh_constructor(_T) = WITH_LOCK_GUARD_1_ATTRS(local_lock_nested_bh, _T)
// class_local_lock_init_constructor(_T) = WITH_LOCK_GUARD_1_ATTRS(local_lock_init, _T)
// class_local_trylock_init_constructor(_T) = WITH_LOCK_GUARD_1_ATTRS(local_trylock_init, _T)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
