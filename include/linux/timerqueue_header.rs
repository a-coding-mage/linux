/* SPDX-License-Identifier: GPL-2.0 */

// The following types and tree operations are supplied by the corresponding
// Linux rbtree and timerqueue_types translations.
use crate::{
    rb_entry_safe, rb_erase_linked, rb_first_cached, rb_root_cached, rb_node,
    timerqueue_head, timerqueue_linked_head, timerqueue_linked_node,
    timerqueue_node, RB_CLEAR_LINKED_NODE, RB_CLEAR_NODE, RB_EMPTY_LINKED_NODE,
    RB_EMPTY_NODE, RB_ROOT_CACHED, RB_ROOT_LINKED,
};

extern "C" {
    pub fn timerqueue_add(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool;
    pub fn timerqueue_del(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool;
    pub fn timerqueue_iterate_next(node: *mut timerqueue_node) -> *mut timerqueue_node;
    pub fn timerqueue_linked_add(
        head: *mut timerqueue_linked_head,
        node: *mut timerqueue_linked_node,
    ) -> bool;
}

/**
 * timerqueue_getnext - Returns the timer with the earliest expiration time
 *
 * @head: head of timerqueue
 *
 * Returns a pointer to the timer node that has the earliest expiration time.
 */
#[inline]
pub unsafe fn timerqueue_getnext(head: *mut timerqueue_head) -> *mut timerqueue_node {
    let leftmost: *mut rb_node = rb_first_cached(&(*head).rb_root);

    rb_entry_safe(leftmost, timerqueue_node, node)
}

#[inline]
pub unsafe fn timerqueue_init(node: *mut timerqueue_node) {
    RB_CLEAR_NODE(&mut (*node).node);
}

#[inline]
pub unsafe fn timerqueue_node_queued(node: *mut timerqueue_node) -> bool {
    !RB_EMPTY_NODE(&(*node).node)
}

#[inline]
pub unsafe fn timerqueue_init_head(head: *mut timerqueue_head) {
    (*head).rb_root = RB_ROOT_CACHED;
}

/* Timer queues with linked nodes */

#[inline(always)]
pub unsafe fn timerqueue_linked_first(
    head: *mut timerqueue_linked_head,
) -> *mut timerqueue_linked_node {
    rb_entry_safe((*head).rb_root.rb_leftmost, timerqueue_linked_node, node)
}

#[inline(always)]
pub unsafe fn timerqueue_linked_next(
    node: *mut timerqueue_linked_node,
) -> *mut timerqueue_linked_node {
    rb_entry_safe((*node).node.next, timerqueue_linked_node, node)
}

#[inline(always)]
pub unsafe fn timerqueue_linked_prev(
    node: *mut timerqueue_linked_node,
) -> *mut timerqueue_linked_node {
    rb_entry_safe((*node).node.prev, timerqueue_linked_node, node)
}

#[inline(always)]
pub unsafe fn timerqueue_linked_del(
    head: *mut timerqueue_linked_head,
    node: *mut timerqueue_linked_node,
) -> bool {
    rb_erase_linked(&mut (*node).node, &mut (*head).rb_root)
}

#[inline(always)]
pub unsafe fn timerqueue_linked_init(node: *mut timerqueue_linked_node) {
    RB_CLEAR_LINKED_NODE(&mut (*node).node);
}

#[inline(always)]
pub unsafe fn timerqueue_linked_node_queued(node: *mut timerqueue_linked_node) -> bool {
    !RB_EMPTY_LINKED_NODE(&(*node).node)
}

#[inline(always)]
pub unsafe fn timerqueue_linked_init_head(head: *mut timerqueue_linked_head) {
    (*head).rb_root = RB_ROOT_LINKED;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
