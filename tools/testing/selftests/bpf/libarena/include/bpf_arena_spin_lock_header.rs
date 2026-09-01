// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/*
 * Rust translation of bpf_arena_spin_lock.h.
 *
 * Original C dependencies:
 *   #include <vmlinux.h>
 *   #include <bpf/bpf_helpers.h>
 *   #include <bpf_atomic.h>
 *
 * The original declarations below are enabled only when both
 * ENABLE_ATOMICS_TESTS and __BPF_FEATURE_ADDR_SPACE_CAST are defined. The BPF
 * __arena address-space qualifier has no direct stable Rust spelling here, so
 * arena pointers are represented as raw pointers.
 */

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const EBUSY: i32 = 16;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const EOPNOTSUPP: i32 = 95;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const ETIMEDOUT: i32 = 110;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
extern "C" {
    pub static CONFIG_NR_CPUS: core::ffi::c_ulong;
}

/*
 * Typically, we'd just rely on the definition in vmlinux.h for qspinlock, but
 * PowerPC overrides the definition to define lock->val as u32 instead of
 * atomic_t, leading to compilation errors.  Import a local definition below so
 * that we don't depend on the vmlinux.h version.
 */

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[repr(C)]
pub struct __qspinlock {
    pub u: __qspinlock_union,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[repr(C)]
pub union __qspinlock_union {
    pub val: atomic_t,
    #[cfg(target_endian = "little")]
    pub bytes: __qspinlock_bytes_le,
    #[cfg(target_endian = "little")]
    pub words: __qspinlock_words_le,
    #[cfg(target_endian = "big")]
    pub words: __qspinlock_words_be,
    #[cfg(target_endian = "big")]
    pub bytes: __qspinlock_bytes_be,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST, target_endian = "little"))]
#[repr(C)]
pub struct __qspinlock_bytes_le {
    pub locked: u8,
    pub pending: u8,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST, target_endian = "little"))]
#[repr(C)]
pub struct __qspinlock_words_le {
    pub locked_pending: u16,
    pub tail: u16,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST, target_endian = "big"))]
#[repr(C)]
pub struct __qspinlock_words_be {
    pub tail: u16,
    pub locked_pending: u16,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST, target_endian = "big"))]
#[repr(C)]
pub struct __qspinlock_bytes_be {
    pub reserved: [u8; 2],
    pub pending: u8,
    pub locked: u8,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub type arena_spinlock_t = __qspinlock;
/* FIXME: Using typedef causes CO-RE relocation error */
/* typedef struct qspinlock arena_spinlock_t; */

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[repr(C)]
pub struct arena_mcs_spinlock {
    pub next: *mut arena_mcs_spinlock,
    pub locked: i32,
    pub count: i32,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[repr(C)]
pub struct arena_qnode {
    pub mcs: arena_mcs_spinlock,
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_MAX_NODES: i32 = 4;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_PENDING_LOOPS: i32 = 1;

/*
 * Bitfields in the atomic value:
 *
 *  0- 7: locked byte
 *     8: pending
 *  9-15: not used
 * 16-17: tail index
 * 18-31: tail cpu (+1)
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_MAX_CPUS: usize = 1024;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_LOCKED_OFFSET: u32 = 0;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_LOCKED_BITS: u32 = 8;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_LOCKED_MASK: u32 = ((1u32 << _Q_LOCKED_BITS) - 1) << _Q_LOCKED_OFFSET;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_PENDING_OFFSET: u32 = _Q_LOCKED_OFFSET + _Q_LOCKED_BITS;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_PENDING_BITS: u32 = 8;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_PENDING_MASK: u32 = ((1u32 << _Q_PENDING_BITS) - 1) << _Q_PENDING_OFFSET;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_IDX_OFFSET: u32 = _Q_PENDING_OFFSET + _Q_PENDING_BITS;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_IDX_BITS: u32 = 2;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_IDX_MASK: u32 = ((1u32 << _Q_TAIL_IDX_BITS) - 1) << _Q_TAIL_IDX_OFFSET;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_CPU_OFFSET: u32 = _Q_TAIL_IDX_OFFSET + _Q_TAIL_IDX_BITS;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_CPU_BITS: u32 = 32 - _Q_TAIL_CPU_OFFSET;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_CPU_MASK: u32 = ((1u32 << _Q_TAIL_CPU_BITS) - 1) << _Q_TAIL_CPU_OFFSET;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_OFFSET: u32 = _Q_TAIL_IDX_OFFSET;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_TAIL_MASK: u32 = _Q_TAIL_IDX_MASK | _Q_TAIL_CPU_MASK;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_LOCKED_VAL: u32 = 1u32 << _Q_LOCKED_OFFSET;
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_PENDING_VAL: u32 = 1u32 << _Q_PENDING_OFFSET;

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
extern "C" {
    pub static mut qnodes: [[arena_qnode; _Q_MAX_NODES as usize]; _Q_MAX_CPUS];
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn encode_tail(cpu: i32, idx: i32) -> u32 {
    let mut tail: u32;

    tail = ((cpu + 1) as u32) << _Q_TAIL_CPU_OFFSET;
    tail |= (idx as u32) << _Q_TAIL_IDX_OFFSET; /* assume < 4 */

    tail
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn decode_tail(tail: u32) -> *mut arena_mcs_spinlock {
    let cpu: u32 = (tail >> _Q_TAIL_CPU_OFFSET).wrapping_sub(1);
    let idx: u32 = (tail & _Q_TAIL_IDX_MASK) >> _Q_TAIL_IDX_OFFSET;

    &mut qnodes[cpu as usize][idx as usize].mcs as *mut arena_mcs_spinlock
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn grab_mcs_node(base: *mut arena_mcs_spinlock, idx: i32) -> *mut arena_mcs_spinlock {
    &mut (*(base as *mut arena_qnode).offset(idx as isize)).mcs as *mut arena_mcs_spinlock
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
pub const _Q_LOCKED_PENDING_MASK: u32 = _Q_LOCKED_MASK | _Q_PENDING_MASK;

/**
 * xchg_tail - Put in the new queue tail code word & retrieve previous one
 * @lock : Pointer to queued spinlock structure
 * @tail : The new queue tail code word
 * Return: The previous queue tail code word
 *
 * xchg(lock, tail)
 *
 * p,*,* -> n,*,* ; prev = xchg(lock, node)
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn xchg_tail(lock: *mut arena_spinlock_t, tail: u32) -> u32 {
    let mut old: u32;
    let mut new: u32;

    old = atomic_read(&mut (*lock).u.val);
    loop {
        new = (old & _Q_LOCKED_PENDING_MASK) | tail;
        /*
         * We can use relaxed semantics since the caller ensures that
         * the MCS node is properly initialized before updating the
         * tail.
         */
        /* These loops are not expected to stall, but we still need to
         * prove to the verifier they will terminate eventually.
         */
        if cond_break() {
            bpf_printk(
                b"RUNTIME ERROR: %s unexpected cond_break exit!!!\0".as_ptr(),
                b"xchg_tail\0".as_ptr(),
            );
            return old;
        }
        if atomic_try_cmpxchg_relaxed(&mut (*lock).u.val, &mut old, new) {
            break;
        }
    }

    old
}

/**
 * clear_pending - clear the pending bit.
 * @lock: Pointer to queued spinlock structure
 *
 * *,1,* -> *,0,*
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn clear_pending(lock: *mut arena_spinlock_t) {
    core::ptr::write_volatile(&mut (*lock).u.bytes.pending, 0);
}

/**
 * clear_pending_set_locked - take ownership and clear the pending bit.
 * @lock: Pointer to queued spinlock structure
 *
 * *,1,0 -> *,0,1
 *
 * Lock stealing is not allowed if this function is used.
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn clear_pending_set_locked(lock: *mut arena_spinlock_t) {
    core::ptr::write_volatile(&mut (*lock).u.words.locked_pending, _Q_LOCKED_VAL as u16);
}

/**
 * set_locked - Set the lock bit and own the lock
 * @lock: Pointer to queued spinlock structure
 *
 * *,*,0 -> *,0,1
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn set_locked(lock: *mut arena_spinlock_t) {
    core::ptr::write_volatile(&mut (*lock).u.bytes.locked, _Q_LOCKED_VAL as u8);
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn arena_fetch_set_pending_acquire(lock: *mut arena_spinlock_t) -> u32 {
    let mut old: u32;
    let mut new: u32;

    old = atomic_read(&mut (*lock).u.val);
    loop {
        new = old | _Q_PENDING_VAL;
        /*
         * These loops are not expected to stall, but we still need to
         * prove to the verifier they will terminate eventually.
         */
        if cond_break() {
            bpf_printk(
                b"RUNTIME ERROR: %s unexpected cond_break exit!!!\0".as_ptr(),
                b"arena_fetch_set_pending_acquire\0".as_ptr(),
            );
            return old;
        }
        if atomic_try_cmpxchg_acquire(&mut (*lock).u.val, &mut old, new) {
            break;
        }
    }

    old
}

/**
 * arena_spin_trylock - try to acquire the queued spinlock
 * @lock : Pointer to queued spinlock structure
 * Return: 1 if lock acquired, 0 if failed
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn arena_spin_trylock(lock: *mut arena_spinlock_t) -> i32 {
    let mut val: i32 = atomic_read(&mut (*lock).u.val);

    if unlikely(val != 0) {
        return 0;
    }

    likely(atomic_try_cmpxchg_acquire(
        &mut (*lock).u.val,
        &mut val,
        _Q_LOCKED_VAL as i32,
    )) as i32
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline(never)]
pub unsafe fn arena_spin_lock_slowpath(lock: *mut arena_spinlock_t, mut val: u32) -> i32 {
    let mut prev: *mut arena_mcs_spinlock;
    let mut next: *mut arena_mcs_spinlock;
    let mut node0: *mut arena_mcs_spinlock;
    let mut node: *mut arena_mcs_spinlock;
    let mut ret: i32 = -ETIMEDOUT;
    let mut old: u32;
    let mut tail: u32;
    let mut idx: i32;

    /*
     * Wait for in-progress pending->locked hand-overs with a bounded
     * number of spins so that we guarantee forward progress.
     *
     * 0,1,0 -> 0,0,1
     */
    if val == _Q_PENDING_VAL {
        let mut cnt: i32 = _Q_PENDING_LOOPS;
        val = atomic_cond_read_relaxed_label(
            &mut (*lock).u.val,
            |VAL: u32| (VAL != _Q_PENDING_VAL) || {
                let old_cnt = cnt;
                cnt -= 1;
                old_cnt == 0
            },
            "release_err",
        );
        if val == _Q_PENDING_VAL {
            return ret;
        }
    }

    /*
     * If we observe any contention; queue.
     */
    if (val & !_Q_LOCKED_MASK) == 0 {
        /*
         * trylock || pending
         *
         * 0,0,* -> 0,1,* -> 0,0,1 pending, trylock
         */
        val = arena_fetch_set_pending_acquire(lock);

        /*
         * If we observe contention, there is a concurrent locker.
         *
         * Undo and queue; our setting of PENDING might have made the
         * n,0,0 -> 0,0,0 transition fail and it will now be waiting
         * on @next to become !NULL.
         */
        if unlikely((val & !_Q_LOCKED_MASK) != 0) {
            /* Undo PENDING if we set it. */
            if (val & _Q_PENDING_MASK) == 0 {
                clear_pending(lock);
            }
        } else {
            /*
             * We're pending, wait for the owner to go away.
             *
             * 0,1,1 -> *,1,0
             *
             * this wait loop must be a load-acquire such that we match the
             * store-release that clears the locked bit and create lock
             * sequentiality; this is because not all
             * clear_pending_set_locked() implementations imply full
             * barriers.
             */
            if (val & _Q_LOCKED_MASK) != 0 {
                let locked = smp_cond_load_acquire_label(
                    &mut (*lock).u.bytes.locked,
                    |VAL: u8| VAL == 0,
                    "release_err",
                );
                if locked != 0 {
                    return ret;
                }
            }

            /*
             * take ownership and clear the pending bit.
             *
             * 0,1,0 -> 0,0,1
             */
            clear_pending_set_locked(lock);
            return 0;
        }
    }

    /*
     * End of pending bit optimistic spinning and beginning of MCS
     * queuing.
     */
    node0 = &mut qnodes[bpf_get_smp_processor_id() as usize][0].mcs as *mut arena_mcs_spinlock;
    idx = (*node0).count;
    (*node0).count += 1;
    tail = encode_tail(bpf_get_smp_processor_id(), idx);

    /*
     * 4 nodes are allocated based on the assumption that there will not be
     * nested NMIs taking spinlocks. That may not be true in some
     * architectures even though the chance of needing more than 4 nodes
     * will still be extremely unlikely. When that happens, we simply return
     * an error. Original qspinlock has a trylock fallback in this case.
     */
    if unlikely(idx >= _Q_MAX_NODES) {
        ret = -EBUSY;
        barrier();
        (*node0).count -= 1;
        return ret;
    }

    node = grab_mcs_node(node0, idx);

    /*
     * Ensure that we increment the head node->count before initialising
     * the actual node. If the compiler is kind enough to reorder these
     * stores, then an IRQ could overwrite our assignments.
     */
    barrier();

    (*node).locked = 0;
    (*node).next = core::ptr::null_mut();

    /*
     * We touched a (possibly) cold cacheline in the per-cpu queue node;
     * attempt the trylock once more in the hope someone let go while we
     * weren't watching.
     */
    if arena_spin_trylock(lock) != 0 {
        barrier();
        (*node0).count -= 1;
        return 0;
    }

    /*
     * Ensure that the initialisation of @node is complete before we
     * publish the updated tail via xchg_tail() and potentially link
     * @node into the waitqueue via WRITE_ONCE(prev->next, node) below.
     */
    smp_wmb();

    /*
     * Publish the updated tail.
     * We have already touched the queueing cacheline; don't bother with
     * pending stuff.
     *
     * p,*,* -> n,*,*
     */
    old = xchg_tail(lock, tail);
    next = core::ptr::null_mut();

    /*
     * if there was a previous node; link it and wait until reaching the
     * head of the waitqueue.
     */
    if (old & _Q_TAIL_MASK) != 0 {
        prev = decode_tail(old);

        /* Link @node into the waitqueue. */
        core::ptr::write_volatile(&mut (*prev).next, node);

        let node_locked = arch_mcs_spin_lock_contended_label(&mut (*node).locked, "release_node_err");
        if node_locked == 0 {
            barrier();
            (*node0).count -= 1;
            return ret;
        }

        /*
         * While waiting for the MCS lock, the next pointer may have
         * been set by another lock waiter. We cannot prefetch here
         * due to lack of equivalent instruction in BPF ISA.
         */
        next = core::ptr::read_volatile(&(*node).next);
    }

    /*
     * we're at the head of the waitqueue, wait for the owner & pending to
     * go away.
     *
     * *,x,y -> *,0,0
     *
     * this wait loop must use a load-acquire such that we match the
     * store-release that clears the locked bit and create lock
     * sequentiality; this is because the set_locked() function below
     * does not imply a full barrier.
     */
    val = atomic_cond_read_acquire_label(
        &mut (*lock).u.val,
        |VAL: u32| (VAL & _Q_LOCKED_PENDING_MASK) == 0,
        "release_node_err",
    );
    if (val & _Q_LOCKED_PENDING_MASK) != 0 {
        barrier();
        (*node0).count -= 1;
        return ret;
    }

    /*
     * claim the lock:
     *
     * n,0,0 -> 0,0,1 : lock, uncontended
     * *,*,0 -> *,*,1 : lock, contended
     *
     * If the queue head is the only one in the queue (lock value == tail)
     * and nobody is pending, clear the tail code and grab the lock.
     * Otherwise, we only need to grab the lock.
     */

    /*
     * In the PV case we might already have _Q_LOCKED_VAL set, because
     * of lock stealing; therefore we must also allow:
     *
     * n,0,1 -> 0,0,1
     *
     * Note: at this point: (val & _Q_PENDING_MASK) == 0, because of the
     *       above wait condition, therefore any concurrent setting of
     *       PENDING will make the uncontended transition fail.
     */
    if (val & _Q_TAIL_MASK) == tail {
        if atomic_try_cmpxchg_relaxed(&mut (*lock).u.val, &mut val, _Q_LOCKED_VAL) {
            barrier();
            (*node0).count -= 1;
            return 0; /* No contention */
        }
    }

    /*
     * Either somebody is queued behind us or _Q_PENDING_VAL got set
     * which will then detect the remaining tail and queue behind us
     * ensuring we'll see a @next.
     */
    set_locked(lock);

    /*
     * contended path; wait for next if not observed yet, release.
     */
    if next.is_null() {
        next = smp_cond_load_relaxed_label(&mut (*node).next, |VAL: *mut arena_mcs_spinlock| !VAL.is_null(), "release_node_err");
        if next.is_null() {
            barrier();
            (*node0).count -= 1;
            return ret;
        }
    }

    arch_mcs_spin_unlock_contended(&mut (*next).locked);

    /*
     * release the node
     *
     * Doing a normal dec vs this_cpu_dec is fine. An upper context always
     * decrements count it incremented before returning, thus we're fine.
     * For contexts interrupting us, they either observe our dec or not.
     * Just ensure the compiler doesn't reorder this statement, as a
     * this_cpu_dec implicitly implied that.
     */
    barrier();
    (*node0).count -= 1;
    0
}

/**
 * arena_spin_lock - acquire a queued spinlock
 * @lock: Pointer to queued spinlock structure
 *
 * On error, returned value will be negative.
 * On success, zero is returned.
 *
 * The return value _must_ be tested against zero for success,
 * instead of checking it against negative, for passing the
 * BPF verifier.
 *
 * The user should do:
 *	if (arena_spin_lock(...) != 0) // failure
 *		or
 *	if (arena_spin_lock(...) == 0) // success
 *		or
 *	if (arena_spin_lock(...)) // failure
 *		or
 *	if (!arena_spin_lock(...)) // success
 * instead of:
 *	if (arena_spin_lock(...) < 0) // failure
 *
 * The return value can still be inspected later.
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn arena_spin_lock(lock: *mut arena_spinlock_t) -> i32 {
    let mut val: i32 = 0;

    if CONFIG_NR_CPUS > 1024 {
        return -EOPNOTSUPP;
    }

    bpf_preempt_disable();
    if likely(atomic_try_cmpxchg_acquire(
        &mut (*lock).u.val,
        &mut val,
        _Q_LOCKED_VAL as i32,
    )) {
        return 0;
    }

    val = arena_spin_lock_slowpath(lock, val as u32);
    /* FIXME: bpf_assert_range(-MAX_ERRNO, 0) once we have it working for all cases. */
    if val != 0 {
        bpf_preempt_enable();
    }
    val
}

/**
 * arena_spin_unlock - release a queued spinlock
 * @lock : Pointer to queued spinlock structure
 */
#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn arena_spin_unlock(lock: *mut arena_spinlock_t) {
    /*
     * unlock() needs release semantics:
     */
    smp_store_release(&mut (*lock).u.bytes.locked, 0);
    bpf_preempt_enable();
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn arena_spin_lock_irqsave(lock: *mut arena_spinlock_t, flags: *mut u64) -> i32 {
    let __ret: i32;
    bpf_local_irq_save(flags);
    __ret = arena_spin_lock(lock);
    if __ret != 0 {
        bpf_local_irq_restore(flags);
    }
    __ret
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[inline]
pub unsafe fn arena_spin_unlock_irqrestore(lock: *mut arena_spinlock_t, flags: *mut u64) {
    arena_spin_unlock(lock);
    bpf_local_irq_restore(flags);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
