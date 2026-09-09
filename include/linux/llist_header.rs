/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Lock-less NULL terminated single linked list
 *
 * Translation of the C header.  Atomic primitives and container_of/offsetof
 * are supplied by the surrounding kernel translation.
 */

#[repr(C)]
pub struct llist_head {
    pub first: *mut llist_node,
}

#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
}

#[macro_export]
macro_rules! LLIST_HEAD_INIT {
    ($name:ident) => { llist_head { first: core::ptr::null_mut() } };
}

#[macro_export]
macro_rules! LLIST_HEAD {
    ($name:ident) => { let mut $name: llist_head = LLIST_HEAD_INIT!($name); };
}

#[inline]
pub unsafe fn init_llist_head(list: *mut llist_head) {
    (*list).first = core::ptr::null_mut();
}

#[inline]
pub unsafe fn init_llist_node(node: *mut llist_node) {
    core::ptr::write_volatile(&mut (*node).next, node);
}

#[inline]
pub unsafe fn llist_on_list(node: *const llist_node) -> bool {
    core::ptr::read_volatile(&(*node).next) != node as *mut llist_node
}

/* llist_entry maps to the surrounding translation's container_of facility. */
#[macro_export]
macro_rules! llist_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

/* Equivalent to ((uintptr_t)(ptr) + offsetof(typeof(*(ptr)), member) != 0). */
#[macro_export]
macro_rules! member_address_is_nonnull {
    ($ptr:expr, $member:ident) => {
        (($ptr as usize) + core::mem::offset_of!(_, $member) != 0)
    };
}

#[macro_export]
macro_rules! llist_for_each {
    ($pos:ident, $node:expr) => {
        for $pos in core::iter::successors(Some($node), |p| unsafe {
            if p.is_null() { None } else { Some((*p).next) }
        })
    };
}

#[macro_export]
macro_rules! llist_for_each_safe {
    ($pos:ident, $n:ident, $node:expr) => {
        for $pos in core::iter::successors(Some($node), |p| unsafe {
            if p.is_null() { None } else { $n = (*p).next; Some($n) }
        })
    };
}

/* The entry traversal macros retain the C container_of-based calling shape. */
#[macro_export]
macro_rules! llist_for_each_entry {
    ($pos:ident, $node:expr, $member:ident) => {
        for $pos in llist_entry!($node, _, $member) {
            if !member_address_is_nonnull!($pos, $member) { break; }
        }
    };
}

#[macro_export]
macro_rules! llist_for_each_entry_safe {
    ($pos:ident, $n:ident, $node:expr, $member:ident) => {
        for $pos in llist_entry!($node, _, $member) {
            if !member_address_is_nonnull!($pos, $member) { break; }
            $n = llist_entry!((*$pos).$member.next, _, $member);
        }
    };
}

#[inline]
pub unsafe fn llist_empty(head: *const llist_head) -> bool {
    core::ptr::read_volatile(&(*head).first).is_null()
}

#[inline]
pub unsafe fn llist_next(node: *mut llist_node) -> *mut llist_node {
    core::ptr::read_volatile(&(*node).next)
}

#[inline]
pub unsafe fn llist_add_batch(
    new_first: *mut llist_node,
    new_last: *mut llist_node,
    head: *mut llist_head,
) -> bool {
    let mut first = core::ptr::read_volatile(&(*head).first);
    loop {
        (*new_last).next = first;
        /* try_cmpxchg(&head->first, &first, new_first) */
        if (*head).first == first {
            (*head).first = new_first;
            break;
        }
        first = core::ptr::read_volatile(&(*head).first);
    }
    first.is_null()
}

#[inline]
pub unsafe fn __llist_add_batch(
    new_first: *mut llist_node,
    new_last: *mut llist_node,
    head: *mut llist_head,
) -> bool {
    (*new_last).next = (*head).first;
    (*head).first = new_first;
    (*new_last).next.is_null()
}

#[inline]
pub unsafe fn llist_add(new_node: *mut llist_node, head: *mut llist_head) -> bool {
    llist_add_batch(new_node, new_node, head)
}

#[inline]
pub unsafe fn __llist_add(new_node: *mut llist_node, head: *mut llist_head) -> bool {
    __llist_add_batch(new_node, new_node, head)
}

#[inline]
pub unsafe fn llist_del_all(head: *mut llist_head) -> *mut llist_node {
    let first = (*head).first;
    (*head).first = core::ptr::null_mut();
    first
}

#[inline]
pub unsafe fn __llist_del_all(head: *mut llist_head) -> *mut llist_node {
    let first = (*head).first;
    (*head).first = core::ptr::null_mut();
    first
}

extern "C" {
    pub fn llist_del_first(head: *mut llist_head) -> *mut llist_node;
}

#[inline]
pub unsafe fn llist_del_first_init(head: *mut llist_head) -> *mut llist_node {
    let n = llist_del_first(head);
    if !n.is_null() {
        init_llist_node(n);
    }
    n
}

extern "C" {
    pub fn llist_del_first_this(head: *mut llist_head, this: *mut llist_node) -> bool;
    pub fn llist_reverse_order(head: *mut llist_node) -> *mut llist_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
