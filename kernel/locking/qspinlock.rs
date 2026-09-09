// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Queued spinlock
 *
 * (C) Copyright 2013-2015 Hewlett-Packard Development Company, L.P.
 * (C) Copyright 2013-2014,2018 Red Hat, Inc.
 * (C) Copyright 2015 Intel Corp.
 * (C) Copyright 2015 Hewlett-Packard Enterprise Development LP
 *
 * Authors: Waiman Long <longman@redhat.com>
 *          Peter Zijlstra <peterz@infradead.org>
 */

// C headers and kernel configuration supplied by other translation units.

/*
 * The basic principle of a queue-based spinlock can best be understood
 * by studying a classic queue-based spinlock implementation called the
 * MCS lock. A copy of the original MCS lock paper ("Algorithms for Scalable
 * Synchronization on Shared-Memory Multiprocessors by Mellor-Crummey and
 * Scott") is available at
 *
 * https://bugzilla.kernel.org/show_bug.cgi?id=206115
 */

// Per-CPU queue node structures; the concrete definitions are external.
static mut qnodes: [qnode; _Q_MAX_NODES] = [qnode::default(); _Q_MAX_NODES];

#[inline(always)]
unsafe fn __pv_init_node(_node: *mut mcs_spinlock) {}
#[inline(always)]
unsafe fn __pv_wait_node(_node: *mut mcs_spinlock, _prev: *mut mcs_spinlock) {}
#[inline(always)]
unsafe fn __pv_kick_node(_lock: *mut qspinlock, _node: *mut mcs_spinlock) {}
#[inline(always)]
unsafe fn __pv_wait_head_or_lock(_lock: *mut qspinlock, _node: *mut mcs_spinlock) -> u32 { 0 }

#[inline(always)]
unsafe fn pv_enabled() -> bool { false }

#[cfg(any())]
pub unsafe fn queued_spin_release_traced(lock: *mut qspinlock) {
    if queued_spin_is_contended(lock) {
        trace_call__contended_release(lock);
    }
    queued_spin_release(lock);
}

/**
 * queued_spin_lock_slowpath - acquire the queued spinlock
 * @lock: Pointer to queued spinlock structure
 * @val: Current value of the queued spinlock 32-bit word
 */
pub unsafe fn queued_spin_lock_slowpath(lock: *mut qspinlock, mut val: u32) {
    let mut prev: *mut mcs_spinlock;
    let mut next: *mut mcs_spinlock;
    let mut node: *mut mcs_spinlock;
    let mut old: u32;
    let mut tail: u32;
    let mut idx: i32;

    // BUILD_BUG_ON(CONFIG_NR_CPUS >= (1U << _Q_TAIL_CPU_BITS));

    if pv_enabled() {
        goto_pv_queue(lock, val);
        return;
    }

    if virt_spin_lock(lock) {
        return;
    }

    if val == _Q_PENDING_VAL {
        let mut cnt = _Q_PENDING_LOOPS;
        val = atomic_cond_read_relaxed(&mut (*lock).val, || {
            val != _Q_PENDING_VAL || { cnt -= 1; cnt < 0 }
        });
    }

    if val & !_Q_LOCKED_MASK != 0 {
        goto_queue(lock, val);
        return;
    }

    val = queued_fetch_set_pending_acquire(lock);
    if val & !_Q_LOCKED_MASK != 0 {
        if val & _Q_PENDING_MASK == 0 {
            clear_pending(lock);
        }
        goto_queue(lock, val);
        return;
    }

    if val & _Q_LOCKED_MASK != 0 {
        smp_cond_load_acquire(&(*lock).locked, || !val_nonzero());
    }
    clear_pending_set_locked(lock);
    lockevent_inc(lock_pending);
    return;

    unsafe fn goto_pv_queue(lock: *mut qspinlock, _val: u32) {
        goto_queue(lock, _val)
    }

    unsafe fn goto_queue(lock: *mut qspinlock, _val: u32) {
        lockevent_inc(lock_slowpath);
        let head = this_cpu_ptr(&mut qnodes[0].mcs);
        let idx = (*head).count;
        (*head).count += 1;
        let tail = encode_tail(smp_processor_id(), idx);
        trace_contention_begin(lock, LCB_F_SPIN);

        if idx >= _Q_MAX_NODES {
            lockevent_inc(lock_no_node);
            while !queued_spin_trylock(lock) { cpu_relax(); }
            trace_contention_end(lock, 0);
            (*head).count -= 1;
            return;
        }

        let node = grab_mcs_node(head, idx);
        lockevent_cond_inc(lock_use_node2 + idx - 1, idx);
        compiler_barrier();
        (*node).locked = 0;
        (*node).next = core::ptr::null_mut();
        __pv_init_node(node);

        if queued_spin_trylock(lock) {
            trace_contention_end(lock, 0);
            (*head).count -= 1;
            return;
        }
        smp_wmb();
        let old = xchg_tail(lock, tail);
        let mut next: *mut mcs_spinlock = core::ptr::null_mut();

        if old & _Q_TAIL_MASK != 0 {
            let prev = decode_tail(old, qnodes.as_mut_ptr());
            core::ptr::write_volatile(&mut (*prev).next, node);
            __pv_wait_node(node, prev);
            arch_mcs_spin_lock_contended(&mut (*node).locked);
            next = core::ptr::read_volatile(&(*node).next);
            if !next.is_null() { prefetchw(next); }
        }

        let val = __pv_wait_head_or_lock(lock, node);
        let val = if val != 0 { val } else {
            atomic_cond_read_acquire(&mut (*lock).val, || !(val_nonzero() & _Q_LOCKED_PENDING_MASK != 0))
        };

        if val & _Q_TAIL_MASK == tail {
            let mut expected = val;
            if atomic_try_cmpxchg_relaxed(&mut (*lock).val, &mut expected, _Q_LOCKED_VAL) {
                trace_contention_end(lock, 0);
                (*head).count -= 1;
                return;
            }
        }
        set_locked(lock);
        if next.is_null() { next = smp_cond_load_relaxed(&mut (*node).next, || !val_is_null()); }
        arch_mcs_spin_unlock_contended(&mut (*next).locked);
        __pv_kick_node(lock, next);
        trace_contention_end(lock, 0);
        (*head).count -= 1;
    }
}

// The paravirtual generated variant is enabled by CONFIG_PARAVIRT_SPINLOCKS
// in the kernel build and is supplied by qspinlock_paravirt.h/qspinlock.c.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
