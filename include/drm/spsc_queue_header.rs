/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation:
// atomic_long_t, atomic_t, preempt_disable, preempt_enable, atomic_long_set,
// atomic_set, atomic_read, atomic_inc, atomic_dec, atomic_long_xchg,
// atomic_long_cmpxchg, smp_mb__after_atomic, smp_wmb, smp_rmb, READ_ONCE,
// WRITE_ONCE, and unlikely.

/** SPSC lockless queue */

#[repr(C)]
pub struct spsc_node {
    /* Stores spsc_node* */
    pub next: *mut spsc_node,
}

#[repr(C)]
pub struct spsc_queue {
    pub head: *mut spsc_node,
    /* atomic pointer to struct spsc_node* */
    pub tail: atomic_long_t,
    pub job_count: atomic_t,
}

#[inline]
pub unsafe fn spsc_queue_init(queue: *mut spsc_queue) {
    (*queue).head = core::ptr::null_mut();
    atomic_long_set(&mut (*queue).tail, (&mut (*queue).head as *mut *mut spsc_node) as isize);
    atomic_set(&mut (*queue).job_count, 0);
}

#[inline]
pub unsafe fn spsc_queue_peek(queue: *mut spsc_queue) -> *mut spsc_node {
    (*queue).head
}

#[inline]
pub unsafe fn spsc_queue_count(queue: *mut spsc_queue) -> i32 {
    atomic_read(&(*queue).job_count)
}

#[inline]
pub unsafe fn spsc_queue_push(queue: *mut spsc_queue, node: *mut spsc_node) -> bool {
    let tail: *mut *mut spsc_node;

    (*node).next = core::ptr::null_mut();

    preempt_disable();

    atomic_inc(&mut (*queue).job_count);
    smp_mb__after_atomic();

    tail = atomic_long_xchg(
        &mut (*queue).tail,
        (&mut (*node).next as *mut *mut spsc_node) as isize,
    ) as *mut *mut spsc_node;
    WRITE_ONCE(&mut *tail, node);

    /*
     * In case of first element verify new node will be visible to the consumer
     * thread when we ping the kernel thread that there is new work to do.
     */
    smp_wmb();

    preempt_enable();

    tail == &mut (*queue).head
}

#[inline]
pub unsafe fn spsc_queue_pop(queue: *mut spsc_queue) -> *mut spsc_node {
    let next: *mut spsc_node;
    let node: *mut spsc_node;

    /* Verify reading from memory and not the cache */
    smp_rmb();

    node = READ_ONCE(&(*queue).head);

    if node.is_null() {
        return core::ptr::null_mut();
    }

    next = READ_ONCE(&(*node).next);
    WRITE_ONCE(&mut (*queue).head, next);

    if next.is_null() {
        /* slowpath for the last element in the queue */

        if atomic_long_cmpxchg(
            &mut (*queue).tail,
            (&mut (*node).next as *mut *mut spsc_node) as isize,
            (&mut (*queue).head as *mut *mut spsc_node) as isize,
        ) != ((&mut (*node).next as *mut *mut spsc_node) as isize) {
            /* Updating tail failed wait for new next to appear */
            loop {
                smp_rmb();
                (*queue).head = READ_ONCE(&(*node).next);
                if !(*queue).head.is_null() {
                    break;
                }
            }
        }
    }

    atomic_dec(&mut (*queue).job_count);
    node
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
