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

// The following types and operations are supplied by the surrounding kernel
// implementation; the declarations here preserve the C interfaces used below.
#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_node_linked {
    pub node: rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timerqueue_node {
    pub node: rb_node,
    pub expires: u64,
}

#[repr(C)]
pub struct timerqueue_head {
    pub rb_root: rb_root_cached,
}

#[repr(C)]
pub struct timerqueue_linked_node {
    pub node: rb_node_linked,
    pub expires: u64,
}

#[repr(C)]
pub struct timerqueue_linked_head {
    pub rb_root: rb_root_cached,
}

extern "C" {
    fn rb_add_cached(
        node: *mut rb_node,
        root: *mut rb_root_cached,
        less: unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool,
    ) -> bool;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_add_linked(
        node: *mut rb_node_linked,
        root: *mut rb_root_cached,
        less: unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool,
    ) -> bool;
    fn warn_on_once(condition: bool) -> bool;
    fn rb_empty_node(node: *const rb_node) -> bool;
    fn rb_clear_node(node: *mut rb_node);
    fn rb_empty_root(root: *const rb_root) -> bool;
}

unsafe extern "C" fn __timerqueue_less(a: *mut rb_node, b: *const rb_node) -> bool {
    (*(a as *const timerqueue_node)).expires < (*(b as *const timerqueue_node)).expires
}

/// Adds timer to timerqueue.
pub unsafe fn timerqueue_add(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool {
    // Make sure we don't add nodes that are already added
    let _ = warn_on_once(!rb_empty_node(&(*node).node));

    rb_add_cached(&mut (*node).node, &mut (*head).rb_root, __timerqueue_less)
}

/// Removes a timer from the timerqueue.
pub unsafe fn timerqueue_del(head: *mut timerqueue_head, node: *mut timerqueue_node) -> bool {
    let _ = warn_on_once(rb_empty_node(&(*node).node));

    rb_erase_cached(&mut (*node).node, &mut (*head).rb_root);
    rb_clear_node(&mut (*node).node);

    !rb_empty_root(&(*head).rb_root.rb_root)
}

/// Returns the timer after the provided timer.
pub unsafe fn timerqueue_iterate_next(node: *mut timerqueue_node) -> *mut timerqueue_node {
    let next: *mut rb_node;

    if node.is_null() {
        return core::ptr::null_mut();
    }
    next = rb_next(&mut (*node).node);
    if next.is_null() {
        return core::ptr::null_mut();
    }
    next as *mut timerqueue_node
}

unsafe extern "C" fn __tq_linked_less(a: *mut rb_node, b: *const rb_node) -> bool {
    (*(a as *const timerqueue_linked_node)).expires
        < (*(b as *const timerqueue_linked_node)).expires
}

pub unsafe fn timerqueue_linked_add(
    head: *mut timerqueue_linked_head,
    node: *mut timerqueue_linked_node,
) -> bool {
    rb_add_linked(
        &mut (*node).node,
        &mut (*head).rb_root,
        __tq_linked_less,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
