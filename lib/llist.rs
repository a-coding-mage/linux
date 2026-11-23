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

// Dependencies: linux/llist.h, linux/export.h.

use core::ptr::{addr_of, addr_of_mut, null_mut, read_volatile};
use kernel::bindings::{llist_head, llist_node};

extern "C" {
    fn lupos_llist_load_acquire(first: *mut *mut llist_node) -> *mut llist_node;
    fn lupos_llist_try_cmpxchg(
        first: *mut *mut llist_node,
        old: *mut *mut llist_node,
        new: *mut llist_node,
    ) -> bool;
}

/**
 * llist_del_first - delete the first entry of lock-less list
 * @head:	the head for your lock-less list
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
/// Deletes and returns the newest entry, or NULL for an empty list.
///
/// # Safety
/// `head` points to a valid `struct llist_head`; the caller is the only
/// concurrent `llist_del_first()` consumer, as the C contract requires.
#[no_mangle]
pub unsafe extern "C" fn llist_del_first(head: *mut llist_head) -> *mut llist_node {
    // SAFETY: head is a live list head; entries reachable from it stay valid
    // while this single consumer owns removal, as in the C implementation.
    unsafe {
        let first = addr_of_mut!((*head).first);
        let mut entry = lupos_llist_load_acquire(first);
        loop {
            if entry.is_null() {
                return null_mut();
            }
            let next = read_volatile(addr_of!((*entry).next));
            // A failed try_cmpxchg() reloads `entry` with the current head.
            if lupos_llist_try_cmpxchg(first, &mut entry, next) {
                break;
            }
        }

        entry
    }
}
// EXPORT_SYMBOL_GPL(llist_del_first);

/**
 * llist_del_first_this - delete given entry of lock-less list if it is first
 * @head:	the head for your lock-less list
 * @this:	a list entry.
 *
 * If head of the list is given entry, delete and return %true else
 * return %false.
 *
 * Multiple callers can safely call this concurrently with multiple
 * llist_add() callers, providing all the callers offer a different @this.
 */
/// Deletes `this` when it is the first entry.
///
/// # Safety
/// `head` points to a valid `struct llist_head`; `this` is the caller's
/// entry, distinct from those of other concurrent callers.
#[no_mangle]
pub unsafe extern "C" fn llist_del_first_this(
    head: *mut llist_head,
    this: *mut llist_node,
) -> bool {
    // SAFETY: head is a live list head; `this` is only dereferenced while it
    // is observed as the first entry, as in the C implementation.
    unsafe {
        let first = addr_of_mut!((*head).first);

        /* acquire ensures orderig wrt try_cmpxchg() is llist_del_first() */
        let mut entry = lupos_llist_load_acquire(first);
        loop {
            if entry != this {
                return false;
            }
            let next = read_volatile(addr_of!((*entry).next));
            if lupos_llist_try_cmpxchg(first, &mut entry, next) {
                break;
            }
        }

        true
    }
}
// EXPORT_SYMBOL_GPL(llist_del_first_this);

/**
 * llist_reverse_order - reverse order of a llist chain
 * @head:	first item of the list to be reversed
 *
 * Reverse the order of a chain of llist entries and return the
 * new first entry.
 */
/// Reverses a detached chain and returns its new first entry.
///
/// # Safety
/// `head` is NULL or the first node of a NULL-terminated chain owned by the
/// caller.
#[no_mangle]
pub unsafe extern "C" fn llist_reverse_order(mut head: *mut llist_node) -> *mut llist_node {
    let mut new_head: *mut llist_node = null_mut();

    while !head.is_null() {
        let tmp = head;
        // SAFETY: every non-NULL node of the caller-owned chain is valid.
        unsafe {
            head = (*head).next;
            (*tmp).next = new_head;
        }
        new_head = tmp;
    }

    new_head
}
// EXPORT_SYMBOL_GPL(llist_reverse_order);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
