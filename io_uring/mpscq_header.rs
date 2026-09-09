/* SPDX-License-Identifier: GPL-2.0 */

/*
 * mpscq - lockless multi-producer, single-consumer FIFO queue
 *
 * Rust translation of mpscq.h. The queue layout and pointer operations retain
 * the C interface and synchronization behavior.
 */

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
}

#[repr(C)]
pub struct mpscq {
    pub tail: *mut llist_node,
    pub stub: llist_node,
}

#[inline]
pub unsafe fn mpscq_init(q: *mut mpscq, headp: *mut *mut llist_node) {
    (*q).tail = &mut (*q).stub;
    *headp = &mut (*q).stub;
    (*q).stub.next = ptr::null_mut();
}

#[inline]
pub unsafe fn mpscq_empty(q: *mut mpscq) -> bool {
    ptr::read_volatile(&(*q).tail) == &mut (*q).stub
}

#[inline]
pub unsafe fn mpscq_push(q: *mut mpscq, node: *mut llist_node) -> bool {
    (*node).next = ptr::null_mut();
    /* xchg() is a full barrier in the source implementation. */
    let tail = &(*q).tail as *const *mut llist_node as *const AtomicPtr<llist_node>;
    let prev = (&*tail).swap(node, Ordering::SeqCst);
    ptr::write_volatile(&mut (*prev).next, node);
    prev == &mut (*q).stub
}

#[inline]
pub unsafe fn mpscq_pop(
    q: *mut mpscq,
    headp: *mut *mut llist_node,
) -> *mut llist_node {
    let mut head = *headp;
    if head == &mut (*q).stub {
        head = ptr::read_volatile(&(*head).next);
        if head.is_null() {
            return ptr::null_mut();
        }
        (*q).stub.next = ptr::null_mut();
        *headp = head;
    }

    let next = ptr::read_volatile(&(*head).next);
    if !next.is_null() {
        *headp = next;
        return head;
    }

    /* try_cmpxchg(&q->tail, &head, &q->stub) */
    let tail = &(*q).tail as *const *mut llist_node as *const AtomicPtr<llist_node>;
    if (&*tail)
        .compare_exchange(head, &mut (*q).stub, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        *headp = &mut (*q).stub;
        return head;
    }
    ptr::null_mut()
}

#[inline]
pub unsafe fn mpscq_pop_emptied(q: *mut mpscq, head: *mut llist_node) -> bool {
    head == &mut (*q).stub
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
