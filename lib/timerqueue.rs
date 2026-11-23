// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Generic Timer-queue
 *
 *  Manages a simple queue of timers, ordered by expiration time.
 *  Uses rbtrees for quick list adds and expiration.
 *
 *  NOTE: All of the following functions need to be serialized
 *  to avoid races. No locking is done by this library code.
 */

// Dependencies: linux/bug.h, linux/timerqueue.h, linux/rbtree.h.

use core::mem::offset_of;
use core::ptr::{addr_of, addr_of_mut, null_mut, read_volatile};
use kernel::bindings::{
    rb_erase, rb_insert_color, rb_next, rb_node, rb_node_linked, rb_root, rb_root_cached,
    rb_root_linked, timerqueue_head, timerqueue_linked_head, timerqueue_linked_node, timerqueue_node,
};
use kernel::ffi::c_ulong;

extern "C" {
    fn lupos_timerqueue_warn_add_linked_node();
    fn lupos_timerqueue_warn_del_empty_node();
}

/// RB_EMPTY_NODE(node).
///
/// # Safety
/// `node` points to a valid `struct rb_node`.
unsafe fn rb_empty_node(node: *const rb_node) -> bool {
    // SAFETY: node is valid.
    unsafe { (*node).__rb_parent_color == node as c_ulong }
}

/// RB_CLEAR_NODE(node).
///
/// # Safety
/// `node` points to a valid `struct rb_node`.
unsafe fn rb_clear_node(node: *mut rb_node) {
    // SAFETY: node is valid.
    unsafe { (*node).__rb_parent_color = node as c_ulong };
}

/// rb_link_node(node, parent, rb_link).
///
/// # Safety
/// `node` is a new node and `rb_link` the child slot of `parent`.
unsafe fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node) {
    // SAFETY: forwarded caller contract.
    unsafe {
        (*node).__rb_parent_color = parent as c_ulong;
        (*node).rb_left = null_mut();
        (*node).rb_right = null_mut();

        *rb_link = node;
    }
}

/// __node_2_tq(n)->expires.
///
/// # Safety
/// `n` is the `node` member of a `struct timerqueue_node`.
unsafe fn tq_expires(n: *const rb_node) -> i64 {
    // SAFETY: rb_entry() of the embedded node.
    unsafe { (*n.byte_sub(offset_of!(timerqueue_node, node)).cast::<timerqueue_node>()).expires }
}

/// __node_2_tq_linked(n)->expires.
///
/// # Safety
/// `n` is the `node.node` member of a `struct timerqueue_linked_node`.
unsafe fn tq_linked_expires(n: *const rb_node) -> i64 {
    // SAFETY: rb_entry() to the rb_node_linked, then container_of() to the
    // timerqueue_linked_node.
    unsafe {
        let linked = n.byte_sub(offset_of!(rb_node_linked, node)).cast::<rb_node_linked>();
        (*linked.byte_sub(offset_of!(timerqueue_linked_node, node)).cast::<timerqueue_linked_node>()).expires
    }
}

/**
 * timerqueue_add - Adds timer to timerqueue.
 *
 * @head: head of timerqueue
 * @node: timer node to be added
 *
 * Adds the timer node to the timerqueue, sorted by the node's expires
 * value. Returns true if the newly added timer is the first expiring timer in
 * the queue.
 */
/// # Safety
/// `head` is a valid timerqueue and `node` an unqueued timer node.
#[no_mangle]
pub unsafe extern "C" fn timerqueue_add(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool {
    // SAFETY: the tree is serialized by the caller, as the C API requires.
    unsafe {
        let node = addr_of_mut!((*node).node);
        let tree: *mut rb_root_cached = addr_of_mut!((*head).rb_root);

        /* Make sure we don't add nodes that are already added */
        if !rb_empty_node(node) {
            lupos_timerqueue_warn_add_linked_node();
        }

        // rb_add_cached(&node->node, &head->rb_root, __timerqueue_less)
        let mut link: *mut *mut rb_node = addr_of_mut!((*tree).rb_root.rb_node);
        let mut parent: *mut rb_node = null_mut();
        let mut leftmost = true;

        while !(*link).is_null() {
            parent = *link;
            if tq_expires(node) < tq_expires(parent) {
                link = addr_of_mut!((*parent).rb_left);
            } else {
                link = addr_of_mut!((*parent).rb_right);
                leftmost = false;
            }
        }

        rb_link_node(node, parent, link);
        // rb_insert_color_cached(node, tree, leftmost)
        if leftmost {
            (*tree).rb_leftmost = node;
        }
        rb_insert_color(node, addr_of_mut!((*tree).rb_root));

        leftmost
    }
}
// EXPORT_SYMBOL_GPL(timerqueue_add);

/**
 * timerqueue_del - Removes a timer from the timerqueue.
 *
 * @head: head of timerqueue
 * @node: timer node to be removed
 *
 * Removes the timer node from the timerqueue. Returns false if the queue is
 * empty after removal.
 */
/// # Safety
/// `head` is a valid timerqueue and `node` a timer node queued on it.
#[no_mangle]
pub unsafe extern "C" fn timerqueue_del(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool {
    // SAFETY: the tree is serialized by the caller, as the C API requires.
    unsafe {
        let node = addr_of_mut!((*node).node);
        let root: *mut rb_root_cached = addr_of_mut!((*head).rb_root);

        if rb_empty_node(node) {
            lupos_timerqueue_warn_del_empty_node();
        }

        // rb_erase_cached(&node->node, &head->rb_root)
        if (*root).rb_leftmost == node {
            (*root).rb_leftmost = rb_next(node);
        }
        rb_erase(node, addr_of_mut!((*root).rb_root));
        rb_clear_node(node);

        // !RB_EMPTY_ROOT(&head->rb_root.rb_root)
        let rb: *const rb_root = addr_of!((*root).rb_root);
        !read_volatile(addr_of!((*rb).rb_node)).is_null()
    }
}
// EXPORT_SYMBOL_GPL(timerqueue_del);

/**
 * timerqueue_iterate_next - Returns the timer after the provided timer
 *
 * @node: Pointer to a timer.
 *
 * Provides the timer that is after the given node. This is used, when
 * necessary, to iterate through the list of timers in a timer list
 * without modifying the list.
 */
/// # Safety
/// `node` is NULL or a timer node queued on a timerqueue.
#[no_mangle]
pub unsafe extern "C" fn timerqueue_iterate_next(node: *mut timerqueue_node) -> *mut timerqueue_node {
    if node.is_null() {
        return null_mut();
    }

    // SAFETY: node is queued; rb_next() returns a sibling node or NULL.
    unsafe {
        let next = rb_next(addr_of!((*node).node));
        if next.is_null() {
            return null_mut();
        }
        next.byte_sub(offset_of!(timerqueue_node, node)).cast::<timerqueue_node>()
    }
}
// EXPORT_SYMBOL_GPL(timerqueue_iterate_next);

/// # Safety
/// `head` is a valid linked timerqueue and `node` an unqueued linked node.
#[no_mangle]
pub unsafe extern "C" fn timerqueue_linked_add(
    head: *mut timerqueue_linked_head,
    node: *mut timerqueue_linked_node,
) -> bool {
    // SAFETY: the tree is serialized by the caller, as the C API requires.
    unsafe {
        let lnode: *mut rb_node_linked = addr_of_mut!((*node).node);
        let tree: *mut rb_root_linked = addr_of_mut!((*head).rb_root);
        let node = addr_of_mut!((*lnode).node);

        // __rb_add(&node->node, &tree->rb_root, __tq_linked_less,
        //          rb_link_linked_node)
        let mut link: *mut *mut rb_node = addr_of_mut!((*tree).rb_root.rb_node);
        let mut parent: *mut rb_node = null_mut();

        while !(*link).is_null() {
            parent = *link;
            if tq_linked_expires(node) < tq_linked_expires(parent) {
                link = addr_of_mut!((*parent).rb_left);
            } else {
                link = addr_of_mut!((*parent).rb_right);
            }
        }

        // rb_link_linked_node(node, parent, link)
        if !parent.is_null() {
            let nnew = lnode;
            let npar = parent.byte_sub(offset_of!(rb_node_linked, node)).cast::<rb_node_linked>();

            if link == addr_of_mut!((*parent).rb_left) {
                (*nnew).prev = (*npar).prev;
                (*nnew).next = npar;
                (*npar).prev = nnew;
                if !(*nnew).prev.is_null() {
                    (*(*nnew).prev).next = nnew;
                }
            } else {
                (*nnew).next = (*npar).next;
                (*nnew).prev = npar;
                (*npar).next = nnew;
                if !(*nnew).next.is_null() {
                    (*(*nnew).next).prev = nnew;
                }
            }
        }
        rb_link_node(node, parent, link);
        rb_insert_color(node, addr_of_mut!((*tree).rb_root));

        if (*lnode).prev.is_null() {
            (*tree).rb_leftmost = lnode;
        }
        (*lnode).prev.is_null()
    }
}
// EXPORT_SYMBOL_GPL(timerqueue_linked_add);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
