/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Descending-priority-sorted double-linked list.
 *
 * This list is a priority-sorted list of nodes; each node has a priority
 * from INT_MIN (highest) to INT_MAX (lowest).  Addition is O(K), removal is
 * O(1), and changing priority is O(K), where K is the number of RT priority
 * levels used in the system.
 *
 * No locking is done; that is up to the caller.
 */

// Dependencies supplied by the corresponding list, container_of, plist_types,
// and bug-warning headers are intentionally left external.

#[macro_export]
macro_rules! PLIST_HEAD_INIT {
    ($head:expr) => {
        plist_head { node_list: LIST_HEAD_INIT!($head.node_list) }
    };
}

#[macro_export]
macro_rules! PLIST_HEAD {
    ($head:ident) => {
        let mut $head = PLIST_HEAD_INIT!($head);
    };
}

#[macro_export]
macro_rules! PLIST_NODE_INIT {
    ($node:expr, $__prio:expr) => {
        plist_node {
            prio: $__prio,
            prio_list: LIST_HEAD_INIT!($node.prio_list),
            node_list: LIST_HEAD_INIT!($node.node_list),
        }
    };
}

#[inline]
pub unsafe fn plist_head_init(head: *mut plist_head) {
    INIT_LIST_HEAD!(&mut (*head).node_list);
}

#[inline]
pub unsafe fn plist_node_init(node: *mut plist_node, prio: core::ffi::c_int) {
    (*node).prio = prio;
    INIT_LIST_HEAD!(&mut (*node).prio_list);
    INIT_LIST_HEAD!(&mut (*node).node_list);
}

extern "C" {
    pub fn plist_add(node: *mut plist_node, head: *mut plist_head);
    pub fn plist_del(node: *mut plist_node, head: *mut plist_head);
    pub fn plist_requeue(node: *mut plist_node, head: *mut plist_head);
}

#[macro_export]
macro_rules! plist_for_each {
    ($pos:ident, $head:expr) => {
        list_for_each_entry!($pos, &mut (*$head).node_list, node_list)
    };
}

#[macro_export]
macro_rules! plist_for_each_continue {
    ($pos:ident, $head:expr) => {
        list_for_each_entry_continue!($pos, &mut (*$head).node_list, node_list)
    };
}

#[macro_export]
macro_rules! plist_for_each_safe {
    ($pos:ident, $n:ident, $head:expr) => {
        list_for_each_entry_safe!($pos, $n, &mut (*$head).node_list, node_list)
    };
}

#[macro_export]
macro_rules! plist_for_each_entry {
    ($pos:ident, $head:expr, $mem:ident) => {
        list_for_each_entry!($pos, &mut (*$head).node_list, $mem.node_list)
    };
}

#[macro_export]
macro_rules! plist_for_each_entry_continue {
    ($pos:ident, $head:expr, $m:ident) => {
        list_for_each_entry_continue!($pos, &mut (*$head).node_list, $m.node_list)
    };
}

#[macro_export]
macro_rules! plist_for_each_entry_safe {
    ($pos:ident, $n:ident, $head:expr, $m:ident) => {
        list_for_each_entry_safe!($pos, $n, &mut (*$head).node_list, $m.node_list)
    };
}

#[inline]
pub unsafe fn plist_head_empty(head: *const plist_head) -> core::ffi::c_int {
    list_empty!(&(*head).node_list)
}

#[inline]
pub unsafe fn plist_node_empty(node: *const plist_node) -> core::ffi::c_int {
    list_empty!(&(*node).node_list)
}

#[macro_export]
macro_rules! plist_first_entry {
    ($head:expr, $ty:ty, $member:ident) => {
        container_of!(plist_first($head), $ty, $member)
    };
}

#[macro_export]
macro_rules! plist_last_entry {
    ($head:expr, $ty:ty, $member:ident) => {
        container_of!(plist_last($head), $ty, $member)
    };
}

#[macro_export]
macro_rules! plist_next {
    ($pos:expr) => { list_next_entry!($pos, node_list) };
}

#[macro_export]
macro_rules! plist_prev {
    ($pos:expr) => { list_prev_entry!($pos, node_list) };
}

#[inline]
pub unsafe fn plist_first(head: *const plist_head) -> *mut plist_node {
    list_entry!((*head).node_list.next, plist_node, node_list)
}

#[inline]
pub unsafe fn plist_last(head: *const plist_head) -> *mut plist_node {
    list_entry!((*head).node_list.prev, plist_node, node_list)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
