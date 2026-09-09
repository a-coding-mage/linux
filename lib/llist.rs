// SPDX-License-Identifier: GPL-2.0-only
/*
 * Lock-less NULL terminated single linked list
 *
 * The basic atomic operation of this list is cmpxchg on long.  On
 * architectures that don't have NMI-safe cmpxchg implementation, the
 * list can NOT be used in NMI handlers.  So code that uses the list in
 * an NMI handler should depend on CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG.
 *
 * Copyright 2010,2011 Intel Corp.
 *   Author: Huang Ying <ying.huang@intel.com>
 */

// Types are supplied by the corresponding llist header in the surrounding
// translation unit.
#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
}

#[repr(C)]
pub struct llist_head {
    pub first: *mut llist_node,
}

/**
 * llist_del_first - delete the first entry of lock-less list
 * @head: the head for your lock-less list
 *
 * If list is empty, return NULL, otherwise, return the first entry
 * deleted, this is the newest added one.
 *
 * Only one llist_del_first user can be used simultaneously with
 * multiple llist_add users without lock.  Because otherwise
 * llist_del_first, llist_add, llist_add (or llist_del_all, llist_add,
 * llist_add) sequence in another user may change @head->first->next,
 * but keep @head->first.  If multiple consumers are needed, please
 * use llist_del_all or use lock between consumers.
 */
#[no_mangle]
pub unsafe extern "C" fn llist_del_first(head: *mut llist_head) -> *mut llist_node {
    use core::sync::atomic::{AtomicPtr, Ordering};

    let first = &*(core::ptr::addr_of!((*head).first) as *const AtomicPtr<llist_node>);
    let mut entry = first.load(Ordering::Acquire);
    loop {
        if entry.is_null() {
            return core::ptr::null_mut();
        }
        let next = core::ptr::read_volatile(core::ptr::addr_of!((*entry).next));
        match first.compare_exchange(entry, next, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return entry,
            Err(actual) => entry = actual,
        }
    }
}

/**
 * llist_del_first_this - delete given entry of lock-less list if it is first
 * @head: the head for your lock-less list
 * @this: a list entry.
 *
 * If head of the list is given entry, delete and return %true else
 * return %false.
 *
 * Multiple callers can safely call this concurrently with multiple
 * llist_add() callers, providing all the callers offer a different @this.
 */
#[no_mangle]
pub unsafe extern "C" fn llist_del_first_this(
    head: *mut llist_head,
    this: *mut llist_node,
) -> bool {
    use core::sync::atomic::{AtomicPtr, Ordering};

    /* acquire ensures orderig wrt try_cmpxchg() is llist_del_first() */
    let first = &*(core::ptr::addr_of!((*head).first) as *const AtomicPtr<llist_node>);
    let mut entry = first.load(Ordering::Acquire);
    loop {
        if entry != this {
            return false;
        }
        let next = core::ptr::read_volatile(core::ptr::addr_of!((*entry).next));
        match first.compare_exchange(entry, next, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return true,
            Err(actual) => entry = actual,
        }
    }
}

/**
 * llist_reverse_order - reverse order of a llist chain
 * @head: first item of the list to be reversed
 *
 * Reverse the order of a chain of llist entries and return the
 * new first entry.
 */
#[no_mangle]
pub unsafe extern "C" fn llist_reverse_order(mut head: *mut llist_node) -> *mut llist_node {
    let mut new_head: *mut llist_node = core::ptr::null_mut();

    while !head.is_null() {
        let tmp = head;
        head = (*head).next;
        (*tmp).next = new_head;
        new_head = tmp;
    }

    new_head
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
