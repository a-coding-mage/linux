/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Red Black Trees -- translation of linux/include/linux/rbtree.h */

/*
  Red Black Trees
  (C) 1999  Andrea Arcangeli <andrea@suse.de>

  linux/include/linux/rbtree.h

  To use rbtrees you'll have to implement your own insert and search cores.
  This will avoid us to use callbacks and to drop drammatically performances.
  I know it's not the cleaner way,  but in C (not in C++) to get
  performances and genericity...

  See Documentation/core-api/rbtree.rst for documentation and samples.
*/
#[path = "rbtree_types_header.rs"]
/// types from the original Linux rbtree interface.
pub mod types;
pub use types::*;

/// Pointer-sized LKMM WRITE_ONCE, retaining order between structural stores.
#[inline]
/// write_once follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn write_once(slot: *mut *mut rb_node, value: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        kernel::sync::atomic::atomic_store(slot, value, kernel::sync::atomic::Relaxed);
    }
}

/// Publish initialized links with a real release store (RCU writer side).
#[inline]
/// publish follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn publish(slot: *mut *mut rb_node, value: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        kernel::sync::atomic::atomic_store(slot, value, kernel::sync::atomic::Release);
    }
}

/// Clear both linked-list pointers and the tree membership marker.
#[inline]
/// rb_clear_linked_node follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_clear_linked_node(node: *mut rb_node_linked) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        rb_clear_node(core::ptr::addr_of_mut!((*node).node));
        (*node).prev = core::ptr::null_mut();
        (*node).next = core::ptr::null_mut();
    }
}

#[inline]
/// rb_parent follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_parent(r: *const rb_node) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { ((*r).__rb_parent_color & !3 as usize) as *mut rb_node }
}

/* rb_entry and rb_entry_safe depend on the source-language container_of/typeof
 * facilities; retain them as macros for callers providing those facilities. */
#[macro_export]
/// Corresponding rbtree header macro.
macro_rules! rb_entry {
    ($ptr:expr, $ty:ty, $member:ident) => {
        (($ptr) as *mut u8)
            .wrapping_sub(core::mem::offset_of!($ty, $member))
            .cast::<$ty>()
    };
}

#[inline]
/// rb_empty_root follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_empty_root(root: *const rb_root) -> bool {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        kernel::sync::atomic::atomic_load(
            core::ptr::addr_of!((*root).rb_node).cast_mut(),
            kernel::sync::atomic::Relaxed,
        ).is_null()
    }
}

#[inline]
/// rb_empty_node follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_empty_node(node: *const rb_node) -> bool {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { (*node).__rb_parent_color == node as usize }
}

#[inline]
/// rb_clear_node follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_clear_node(node: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        (*node).__rb_parent_color = node as usize;
    }
}

extern "C" {
    /// Original C ABI declaration for rb_insert_color.
    pub fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    /// Original C ABI declaration for rb_erase.
    pub fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    /// Original C ABI declaration for rb_erase_linked.
    pub fn rb_erase_linked(node: *mut rb_node_linked, root: *mut rb_root_linked) -> bool;
    /// Original C ABI declaration for rb_next.
    pub fn rb_next(node: *const rb_node) -> *mut rb_node;
    /// Original C ABI declaration for rb_prev.
    pub fn rb_prev(node: *const rb_node) -> *mut rb_node;
    /// Original C ABI declaration for rb_first_postorder.
    pub fn rb_first_postorder(root: *const rb_root) -> *mut rb_node;
    /// Original C ABI declaration for rb_next_postorder.
    pub fn rb_next_postorder(node: *const rb_node) -> *mut rb_node;
    /// Original C ABI declaration for rb_replace_node.
    pub fn rb_replace_node(victim: *mut rb_node, new: *mut rb_node, root: *mut rb_root);
    /// Original C ABI declaration for rb_replace_node_rcu.
    pub fn rb_replace_node_rcu(victim: *mut rb_node, new: *mut rb_node, root: *mut rb_root);
}

#[inline]
/// rb_first follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_first(root: *const rb_root) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut n = (*root).rb_node;
        if n.is_null() {
            return core::ptr::null_mut();
        }
        while !(*n).rb_left.is_null() {
            n = (*n).rb_left;
        }
        n
    }
}

#[inline]
/// rb_last follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_last(root: *const rb_root) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut n = (*root).rb_node;
        if n.is_null() {
            return core::ptr::null_mut();
        }
        while !(*n).rb_right.is_null() {
            n = (*n).rb_right;
        }
        n
    }
}

#[inline]
/// rb_link_node follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        (*node).__rb_parent_color = parent as usize;
        (*node).rb_left = core::ptr::null_mut();
        (*node).rb_right = core::ptr::null_mut();
        *link = node;
    }
}

#[inline]
/// rb_link_node_rcu follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_link_node_rcu(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        (*node).__rb_parent_color = parent as usize;
        (*node).rb_left = core::ptr::null_mut();
        (*node).rb_right = core::ptr::null_mut();
        publish(link, node);
    }
}

#[inline]
/// rb_first_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { (*root).rb_leftmost }
}

#[inline]
/// rb_insert_color_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_insert_color_cached(
    node: *mut rb_node,
    root: *mut rb_root_cached,
    leftmost: bool,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if leftmost {
            (*root).rb_leftmost = node;
        }
        rb_insert_color(node, core::ptr::addr_of_mut!((*root).rb_root));
    }
}

#[inline]
/// rb_erase_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut leftmost = core::ptr::null_mut();
        if (*root).rb_leftmost == node {
            leftmost = rb_next(node);
            (*root).rb_leftmost = leftmost;
        }
        rb_erase(node, core::ptr::addr_of_mut!((*root).rb_root));
        leftmost
    }
}

#[inline]
/// rb_replace_node_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_replace_node_cached(
    victim: *mut rb_node,
    new: *mut rb_node,
    root: *mut rb_root_cached,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if (*root).rb_leftmost == victim {
            (*root).rb_leftmost = new;
        }
        rb_replace_node(victim, new, core::ptr::addr_of_mut!((*root).rb_root));
    }
}

/* The insertion/search helpers below are direct Rust equivalents of the
 * corresponding inline C helpers. */
#[inline]
/// rb_add_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_add_cached(
    node: *mut rb_node,
    tree: *mut rb_root_cached,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut link = core::ptr::addr_of_mut!((*tree).rb_root.rb_node);
        let mut parent = core::ptr::null_mut();
        let mut leftmost = true;
        while !(*link).is_null() {
            parent = *link;
            if less.unwrap_unchecked()(node, parent) {
                link = core::ptr::addr_of_mut!((*parent).rb_left);
            } else {
                link = core::ptr::addr_of_mut!((*parent).rb_right);
                leftmost = false;
            }
        }
        rb_link_node(node, parent, link);
        rb_insert_color_cached(node, tree, leftmost);
        if leftmost {
            node
        } else {
            core::ptr::null_mut()
        }
    }
}

#[inline]
/// rb_find follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_find(
    key: *const core::ffi::c_void,
    tree: *const rb_root,
    cmp: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32>,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut node = (*tree).rb_node;
        while !node.is_null() {
            let c = cmp.unwrap_unchecked()(key, node);
            if c < 0 {
                node = (*node).rb_left;
            } else if c > 0 {
                node = (*node).rb_right;
            } else {
                return node;
            }
        }
        core::ptr::null_mut()
    }
}

#[inline]
/// rb_find_add follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_find_add(
    node: *mut rb_node,
    tree: *mut rb_root,
    cmp: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> i32>,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut link = core::ptr::addr_of_mut!((*tree).rb_node);
        let mut parent = core::ptr::null_mut();
        while !(*link).is_null() {
            parent = *link;
            let c = cmp.unwrap_unchecked()(node, parent);
            if c < 0 {
                link = core::ptr::addr_of_mut!((*parent).rb_left);
            } else if c > 0 {
                link = core::ptr::addr_of_mut!((*parent).rb_right);
            } else {
                return parent;
            }
        }
        rb_link_node(node, parent, link);
        rb_insert_color(node, tree);
        core::ptr::null_mut()
    }
}

#[inline]
/// rb_find_add_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_find_add_cached(
    node: *mut rb_node,
    tree: *mut rb_root_cached,
    cmp: Option<unsafe extern "C" fn(*const rb_node, *const rb_node) -> i32>,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut link = core::ptr::addr_of_mut!((*tree).rb_root.rb_node);
        let mut parent = core::ptr::null_mut();
        let mut leftmost = true;
        while !(*link).is_null() {
            parent = *link;
            let c = cmp.unwrap_unchecked()(node, parent);
            if c < 0 {
                link = core::ptr::addr_of_mut!((*parent).rb_left);
            } else if c > 0 {
                link = core::ptr::addr_of_mut!((*parent).rb_right);
                leftmost = false;
            } else {
                return parent;
            }
        }
        rb_link_node(node, parent, link);
        rb_insert_color_cached(node, tree, leftmost);
        core::ptr::null_mut()
    }
}

#[inline]
/// rb_find_first follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_find_first(
    key: *const core::ffi::c_void,
    tree: *const rb_root,
    cmp: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32>,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut node = (*tree).rb_node;
        let mut found = core::ptr::null_mut();
        while !node.is_null() {
            let c = cmp.unwrap_unchecked()(key, node);
            if c <= 0 {
                if c == 0 {
                    found = node;
                }
                node = (*node).rb_left;
            } else {
                node = (*node).rb_right;
            }
        }
        found
    }
}

#[inline]
/// rb_next_match follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_next_match(
    key: *const core::ffi::c_void,
    mut node: *mut rb_node,
    cmp: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32>,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        node = rb_next(node);
        if !node.is_null() && cmp.unwrap_unchecked()(key, node) != 0 {
            core::ptr::null_mut()
        } else {
            node
        }
    }
}

#[inline]
/// rb_link_noop follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_link_noop(_n: *mut rb_node, _p: *mut rb_node, _l: *mut *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
}

#[inline]
/// rb_add follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_add(
    node: *mut rb_node,
    tree: *mut rb_root,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut link = core::ptr::addr_of_mut!((*tree).rb_node);
        let mut parent = core::ptr::null_mut();
        while !(*link).is_null() {
            parent = *link;
            if less.unwrap_unchecked()(node, parent) {
                link = core::ptr::addr_of_mut!((*parent).rb_left);
            } else {
                link = core::ptr::addr_of_mut!((*parent).rb_right);
            }
        }
        rb_link_node(node, parent, link);
        rb_insert_color(node, tree);
    }
}

#[macro_export]
/// Corresponding rbtree header macro.
macro_rules! RB_EMPTY_NODE {
    ($node:expr) => {
        (*$node).__rb_parent_color == ($node as usize)
    };
}
#[macro_export]
/// Corresponding rbtree header macro.
macro_rules! RB_CLEAR_NODE {
    ($node:expr) => {
        (*$node).__rb_parent_color = ($node as usize)
    };
}
#[macro_export]
/// Corresponding rbtree header macro.
macro_rules! RB_EMPTY_ROOT {
    ($root:expr) => {
        kernel::sync::atomic::atomic_load(
            core::ptr::addr_of!((*$root).rb_node).cast_mut(),
            kernel::sync::atomic::Relaxed,
        ).is_null()
    };
}

/// Iterate matches, executing the supplied body for each current node.
#[macro_export]
/// Corresponding rbtree header macro.
macro_rules! rb_for_each {
    ($node:ident, $key:expr, $tree:expr, $cmp:expr, $body:block) => {
        let mut first_iteration = true;
        let mut $node = core::ptr::null_mut();
        loop {
            $node = if first_iteration {
                first_iteration = false;
                rb_find_first($key, $tree, $cmp)
            } else {
                rb_next_match($key, $node, $cmp)
            };
            if $node.is_null() {
                break;
            }
            $body
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/// Maintain the in-order links before linking a new tree node.
///
/// # Safety
/// Nodes must be embedded in initialized rb_node_linked objects.
pub unsafe extern "C" fn rb_link_linked_node(
    node: *mut rb_node,
    parent: *mut rb_node,
    link: *mut *mut rb_node,
) {
    // SAFETY: The caller supplies valid linked nodes, with node at offset zero.
    unsafe {
        if parent.is_null() {
            return;
        }
        let new = node.cast::<rb_node_linked>();
        let par = parent.cast::<rb_node_linked>();
        if link == core::ptr::addr_of_mut!((*parent).rb_left) {
            (*new).prev = (*par).prev;
            (*new).next = par;
            (*par).prev = new;
            if !(*new).prev.is_null() {
                (*(*new).prev).next = new;
            }
        } else {
            (*new).next = (*par).next;
            (*new).prev = par;
            (*par).next = new;
            if !(*new).next.is_null() {
                (*(*new).next).prev = new;
            }
        }
    }
}

/// Insert with a pre-link operation, in the original callback order.
///
/// # Safety
/// Tree and callbacks must meet the C insertion contract.
pub unsafe fn __rb_add(
    node: *mut rb_node,
    tree: *mut rb_root,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
    linkop: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node, *mut *mut rb_node)>,
) {
    // SAFETY: Valid links and callbacks at each reached call are required.
    unsafe {
        let mut link = core::ptr::addr_of_mut!((*tree).rb_node);
        let mut parent = core::ptr::null_mut();
        while !(*link).is_null() {
            parent = *link;
            link = if less.unwrap_unchecked()(node, parent) {
                core::ptr::addr_of_mut!((*parent).rb_left)
            } else {
                core::ptr::addr_of_mut!((*parent).rb_right)
            };
        }
        linkop.unwrap_unchecked()(node, parent, link);
        rb_link_node(node, parent, link);
        rb_insert_color(node, tree);
    }
}

/// Insert into a linked tree; returns whether the node is the new leftmost.
///
/// # Safety
/// Node must have initialized prev/next links and not already be in the tree.
pub unsafe fn rb_add_linked(
    node: *mut rb_node_linked,
    tree: *mut rb_root_linked,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
) -> bool {
    // SAFETY: The caller provides a valid linked tree and comparator.
    unsafe {
        __rb_add(
            core::ptr::addr_of_mut!((*node).node),
            core::ptr::addr_of_mut!((*tree).rb_root),
            less,
            Some(rb_link_linked_node),
        );
        if (*node).prev.is_null() {
            (*tree).rb_leftmost = node;
        }
        (*node).prev.is_null()
    }
}

/// Search or insert, publishing the new node with a release store.
///
/// # Safety
/// The caller serializes writers and provides valid nodes and comparator.
pub unsafe fn rb_find_add_rcu(
    node: *mut rb_node,
    tree: *mut rb_root,
    cmp: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> i32>,
) -> *mut rb_node {
    // SAFETY: Valid tree links and callback at each reached call are required.
    unsafe {
        let mut link = core::ptr::addr_of_mut!((*tree).rb_node);
        let mut parent = core::ptr::null_mut();
        while !(*link).is_null() {
            parent = *link;
            let c = cmp.unwrap_unchecked()(node, parent);
            if c == 0 {
                return parent;
            }
            link = if c < 0 {
                core::ptr::addr_of_mut!((*parent).rb_left)
            } else {
                core::ptr::addr_of_mut!((*parent).rb_right)
            };
        }
        rb_link_node_rcu(node, parent, link);
        rb_insert_color(node, tree);
        core::ptr::null_mut()
    }
}

/// RCU lookup using kernel LKMM atomics, interoperable with C RCU publication.
///
/// Root sampling uses READ_ONCE. Child loads are acquire (stronger than the C
/// dependency-only loads), never Rust core atomics mixed with non-atomic stores.
///
/// # Safety
/// Caller must hold the appropriate RCU read-side lifetime protection.
pub unsafe fn rb_find_rcu(
    key: *const core::ffi::c_void,
    tree: *const rb_root,
    cmp: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32>,
) -> *mut rb_node {
    // SAFETY: Links remain allocated throughout the protected traversal.
    unsafe {
        let mut node = kernel::sync::atomic::atomic_load(
            core::ptr::addr_of!((*tree).rb_node).cast_mut(),
            kernel::sync::atomic::Relaxed,
        );
        while !node.is_null() {
            let c = cmp.unwrap_unchecked()(key, node);
            if c == 0 {
                return node;
            }
            let link = if c < 0 {
                core::ptr::addr_of_mut!((*node).rb_left)
            } else {
                core::ptr::addr_of_mut!((*node).rb_right)
            };
            node = kernel::sync::atomic::atomic_load(link, kernel::sync::atomic::Acquire);
        }
        core::ptr::null_mut()
    }
}

/// Recover an enclosing object from its embedded node.
#[macro_export]
macro_rules! rb_entry_safe {
    ($ptr:expr, $ty:ty, $member:ident) => {{
        let ptr = $ptr;
        if ptr.is_null() {
            core::ptr::null_mut::<$ty>()
        } else {
            (ptr as *mut u8)
                .wrapping_sub(core::mem::offset_of!($ty, $member))
                .cast::<$ty>()
        }
    }};
}

/// Visit postorder with the next object computed before the body may free pos.
#[macro_export]
macro_rules! rbtree_postorder_for_each_entry_safe {
    ($pos:ident, $next:ident, $root:expr, $ty:ty, $field:ident, $body:block) => {{
        let mut cursor = $crate::rb_entry_safe!(rb_first_postorder($root), $ty, $field);
        while !cursor.is_null() {
            let $pos = cursor;
            let $next = $crate::rb_entry_safe!(
                rb_next_postorder(core::ptr::addr_of!((*$pos).$field)),
                $ty,
                $field
            );
            cursor = $next;
            $body
        }
    }};
}

/// Membership marker for a linked node.
#[macro_export]
macro_rules! RB_EMPTY_LINKED_NODE {
    ($node:expr) => {
        (*$node).node.__rb_parent_color == core::ptr::addr_of!((*$node).node) as usize
    };
}

/// Reset the membership marker and both linked-list links.
#[macro_export]
macro_rules! RB_CLEAR_LINKED_NODE {
    ($node:expr) => {{
        let node = $node;
        (*node).node.__rb_parent_color = core::ptr::addr_of!((*node).node) as usize;
        (*node).prev = core::ptr::null_mut();
        (*node).next = core::ptr::null_mut();
    }};
}
