// SPDX-License-Identifier: GPL-2.0-only
/*
 * fence-chain: chain fences together in a timeline
 *
 * Copyright (C) 2018 Advanced Micro Devices, Inc.
 * Authors:
 *	Christian König <christian.koenig@amd.com>
 */

// Dependency declarations supplied by the surrounding kernel translation.

unsafe fn dma_fence_chain_enable_signaling(fence: *mut dma_fence) -> bool;

/**
 * dma_fence_chain_get_prev - use RCU to get a reference to the previous fence
 * @chain: chain node to get the previous node from
 *
 * Use dma_fence_get_rcu_safe to get a reference to the previous fence of the
 * chain node.
 */
unsafe fn dma_fence_chain_get_prev(chain: *mut dma_fence_chain) -> *mut dma_fence {
    let prev: *mut dma_fence;

    rcu_read_lock();
    prev = dma_fence_get_rcu_safe(unsafe { &mut (*chain).prev });
    rcu_read_unlock();
    prev
}

/**
 * dma_fence_chain_walk - chain walking function
 * @fence: current chain node
 *
 * Walk the chain to the next node. Returns the next fence or NULL if we are at
 * the end of the chain. Garbage collects chain nodes which are already
 * signaled.
 */
unsafe fn dma_fence_chain_walk(mut fence: *mut dma_fence) -> *mut dma_fence {
    let chain = to_dma_fence_chain(fence);
    if chain.is_null() {
        dma_fence_put(fence);
        return core::ptr::null_mut();
    }

    let mut prev: *mut dma_fence;
    while {
        prev = dma_fence_chain_get_prev(chain);
        !prev.is_null()
    } {
        let prev_chain = to_dma_fence_chain(prev);
        let replacement: *mut dma_fence;
        if !prev_chain.is_null() {
            if !dma_fence_is_signaled((*prev_chain).fence) {
                break;
            }
            replacement = dma_fence_chain_get_prev(prev_chain);
        } else {
            if !dma_fence_is_signaled(prev) {
                break;
            }
            replacement = core::ptr::null_mut();
        }

        let tmp = unrcu_pointer(cmpxchg(
            &mut (*chain).prev,
            rcu_initializer(prev),
            rcu_initializer(replacement),
        ));
        if tmp == prev {
            dma_fence_put(tmp);
        } else {
            dma_fence_put(replacement);
        }
        dma_fence_put(prev);
    }

    dma_fence_put(fence);
    prev
}

/**
 * dma_fence_chain_find_seqno - find fence chain node by seqno
 * @pfence: pointer to the chain node where to start
 * @seqno: the sequence number to search for
 *
 * Advance the fence pointer to the chain node which will signal this sequence
 * number. If no sequence number is provided then this is a no-op.
 *
 * Returns EINVAL if the fence is not a chain node or the sequence number has
 * not yet advanced far enough.
 */
unsafe fn dma_fence_chain_find_seqno(pfence: *mut *mut dma_fence, seqno: u64) -> i32 {
    if seqno == 0 {
        return 0;
    }

    let chain = to_dma_fence_chain(*pfence);
    if chain.is_null() || (*chain).base.seqno < seqno {
        return -EINVAL;
    }

    // dma_fence_chain_for_each(*pfence, &chain->base)
    while {
        let current = *pfence;
        if (*current).context != (*chain).base.context
            || (*to_dma_fence_chain(current)).prev_seqno < seqno
        {
            break;
        }
        // The macro advances *pfence to the next chain node.
        *pfence = dma_fence_chain_walk(current);
        !(*pfence).is_null()
    } {}
    dma_fence_put(&mut (*chain).base);
    0
}

unsafe fn dma_fence_chain_get_driver_name(_fence: *mut dma_fence) -> *const u8 {
    b"dma_fence_chain\0".as_ptr()
}

unsafe fn dma_fence_chain_get_timeline_name(_fence: *mut dma_fence) -> *const u8 {
    b"unbound\0".as_ptr()
}

unsafe fn dma_fence_chain_irq_work(work: *mut irq_work) {
    let chain = container_of(work, dma_fence_chain_work_member());

    /* Try to rearm the callback */
    if !dma_fence_chain_enable_signaling(&mut (*chain).base) {
        /* Ok, we are done. No more unsignaled fences left */
        dma_fence_signal(&mut (*chain).base);
    }
    dma_fence_put(&mut (*chain).base);
}

unsafe fn dma_fence_chain_cb(f: *mut dma_fence, cb: *mut dma_fence_cb) {
    let chain = container_of(cb, dma_fence_chain_cb_member());
    init_irq_work(&mut (*chain).work, dma_fence_chain_irq_work);
    irq_work_queue(&mut (*chain).work);
    dma_fence_put(f);
}

unsafe fn dma_fence_chain_enable_signaling(fence: *mut dma_fence) -> bool {
    let head = to_dma_fence_chain(fence);

    dma_fence_get(&mut (*head).base);
    // dma_fence_chain_for_each(fence, &head->base)
    let mut current = fence;
    while !current.is_null() {
        let f = dma_fence_chain_contained(current);
        dma_fence_get(f);
        if dma_fence_add_callback(f, &mut (*head).cb, dma_fence_chain_cb) == 0 {
            dma_fence_put(current);
            return true;
        }
        dma_fence_put(f);
        current = dma_fence_chain_walk(current);
    }
    dma_fence_put(&mut (*head).base);
    false
}

unsafe fn dma_fence_chain_signaled(fence: *mut dma_fence) -> bool {
    let mut current = fence;
    while !current.is_null() {
        let f = dma_fence_chain_contained(current);
        if !dma_fence_is_signaled(f) {
            dma_fence_put(current);
            return false;
        }
        current = dma_fence_chain_walk(current);
    }
    true
}

unsafe fn dma_fence_chain_release(fence: *mut dma_fence) {
    let chain = to_dma_fence_chain(fence);
    let mut prev: *mut dma_fence;

    /* Manually unlink the chain as much as possible to avoid recursion
     * and potential stack overflow.
     */
    while {
        prev = rcu_dereference_protected((*chain).prev, true);
        !prev.is_null()
    } {
        if kref_read(&(*prev).refcount) > 1 {
            break;
        }
        let prev_chain = to_dma_fence_chain(prev);
        if prev_chain.is_null() {
            break;
        }
        /* No need for atomic operations since we hold the last
         * reference to prev_chain.
         */
        (*chain).prev = (*prev_chain).prev;
        rcu_init_pointer(&mut (*prev_chain).prev, core::ptr::null_mut());
        dma_fence_put(prev);
    }
    dma_fence_put(prev);

    dma_fence_put((*chain).fence);
    dma_fence_free(fence);
}

unsafe fn dma_fence_chain_set_deadline(mut fence: *mut dma_fence, deadline: ktime_t) {
    while !fence.is_null() {
        let f = dma_fence_chain_contained(fence);
        dma_fence_set_deadline(f, deadline);
        fence = dma_fence_chain_walk(fence);
    }
}

const dma_fence_chain_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(dma_fence_chain_get_driver_name),
    get_timeline_name: Some(dma_fence_chain_get_timeline_name),
    enable_signaling: Some(dma_fence_chain_enable_signaling),
    signaled: Some(dma_fence_chain_signaled),
    release: Some(dma_fence_chain_release),
    set_deadline: Some(dma_fence_chain_set_deadline),
};

/**
 * dma_fence_chain_init - initialize a fence chain
 * @chain: the chain node to initialize
 * @prev: the previous fence
 * @fence: the current fence
 * @seqno: the sequence number to use for the fence chain
 *
 * Initialize a new chain node and either start a new chain or add the node to
 * the existing chain of the previous fence.
 */
unsafe fn dma_fence_chain_init(
    chain: *mut dma_fence_chain,
    prev: *mut dma_fence,
    fence: *mut dma_fence,
    mut seqno: u64,
) {
    static mut DMA_FENCE_CHAIN_LOCK_KEY: lock_class_key = lock_class_key {};
    let prev_chain = to_dma_fence_chain(prev);
    let context: u64;

    rcu_assign_pointer(&mut (*chain).prev, prev);
    (*chain).fence = fence;
    (*chain).prev_seqno = 0;

    /* Try to reuse the context of the previous chain node. */
    if !prev_chain.is_null() && __dma_fence_is_later(prev, seqno, (*prev).seqno) {
        context = (*prev).context;
        (*chain).prev_seqno = (*prev).seqno;
    } else {
        context = dma_fence_context_alloc(1);
        /* Make sure that we always have a valid sequence number. */
        if !prev_chain.is_null() {
            seqno = core::cmp::max((*prev).seqno, seqno);
        }
    }

    dma_fence_init64(&mut (*chain).base, &dma_fence_chain_ops, core::ptr::null_mut(), context, seqno);

    /*
     * dma_fence_chain_enable_signaling() is invoked while holding
     * chain->base.inline_lock and may call dma_fence_add_callback()
     * on the underlying fences, which takes their inline_lock.
     *
     * Since both locks share the same lockdep class, this legitimate
     * nesting confuses lockdep and triggers a recursive locking
     * warning. Assign a separate lockdep class to the chain lock
     * to model this hierarchy correctly.
     */
    lockdep_set_class(&mut (*chain).base.inline_lock, &DMA_FENCE_CHAIN_LOCK_KEY);

    /*
     * Chaining dma_fence_chain container together is only allowed through
     * the prev fence and not through the contained fence.
     *
     * The correct way of handling this is to flatten out the fence
     * structure into a dma_fence_array by the caller instead.
     */
    WARN_ON(dma_fence_is_chain(fence));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
