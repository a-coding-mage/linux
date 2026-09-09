/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Resilient Queued Spin Lock defines
 *
 * (C) Copyright 2024-2025 Meta Platforms, Inc. and affiliates.
 *
 * Authors: Kumar Kartikeya Dwivedi <memxor@gmail.com>
 */

// Dependency equivalent of: #include "../locking/qspinlock.h"

/*
 * try_cmpxchg_tail - Return result of cmpxchg of tail word with a new value
 * @lock: Pointer to queued spinlock structure
 * @tail: The tail to compare against
 * @new_tail: The new queue tail code word
 * Return: Bool to indicate whether the cmpxchg operation succeeded
 *
 * This is used by the head of the wait queue to clean up the queue.
 * Provides relaxed ordering, since observers only rely on initialized
 * state of the node which was made visible through the xchg_tail operation,
 * i.e. through the smp_wmb preceding xchg_tail.
 *
 * We avoid using 16-bit cmpxchg, which is not available on all architectures.
 */
#[inline(always)]
pub unsafe fn try_cmpxchg_tail(lock: *mut qspinlock, tail: u32, new_tail: u32) -> bool {
    let mut old: u32;
    old = unsafe { atomic_read(&(*lock).val) };
    loop {
        /*
         * Is the tail part we compare to already stale? Fail.
         */
        if (old & _Q_TAIL_MASK) != tail {
            return false;
        }
        /*
         * Encode latest locked/pending state for new tail.
         */
        let new = (old & _Q_LOCKED_PENDING_MASK) | new_tail;
        if unsafe { atomic_try_cmpxchg_relaxed(&(*lock).val, &mut old, new) } {
            break;
        }
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
