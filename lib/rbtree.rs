// SPDX-License-Identifier: GPL-2.0-or-later
/*
  Red Black Trees
  (C) 1999  Andrea Arcangeli <andrea@suse.de>
  (C) 2002  David Woodhouse <dwmw2@infradead.org>
  (C) 2012  Michel Lespinasse <walken@google.com>


  linux/lib/rbtree.c
*/

// The original C permits null callbacks when no rotation is reached. The actual
// binding wrapper preserves that representation and the enclosing C KCFI type.
#[path = "../include/linux/rbtree_augmented_header.rs"]
/// augmented from the original Linux rbtree interface.
pub mod augmented;
#[path = "../include/linux/rbtree_header.rs"]
/// declarations from the original Linux rbtree interface.
pub mod declarations;
pub use declarations::{rb_first, rb_last};
use augmented::{
    __rb_change_child, __rb_change_child_rcu, __rb_erase_augmented, rb_augment_callbacks,
    rb_is_black, rb_is_red, rb_set_parent, rb_set_parent_color, RB_BLACK, RB_RED,
};
use declarations::{
    rb_clear_linked_node, rb_empty_node, rb_node, rb_node_linked, rb_parent, rb_root,
    rb_root_linked, write_once,
};

#[inline]
unsafe fn rb_set_black(rb: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        (*rb).__rb_parent_color = (*rb).__rb_parent_color.wrapping_add(RB_BLACK as _);
    }
}
#[inline]
unsafe fn rb_red_parent(red: *mut rb_node) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { (*red).__rb_parent_color as *mut rb_node }
}

#[inline]
unsafe fn __rb_rotate_set_parents(
    old: *mut rb_node,
    new: *mut rb_node,
    root: *mut rb_root,
    color: i32,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let parent = rb_parent(old);
        (*new).__rb_parent_color = (*old).__rb_parent_color;
        rb_set_parent_color(old, new, color);
        __rb_change_child(old, new, parent, root);
    }
}

unsafe fn __rb_insert(
    mut node: *mut rb_node,
    root: *mut rb_root,
    augment_rotate: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut parent = rb_red_parent(node);
        loop {
            if parent.is_null() {
                rb_set_parent_color(node, core::ptr::null_mut(), RB_BLACK);
                break;
            }
            if rb_is_black(parent) {
                break;
            }
            let gparent = rb_red_parent(parent);
            let mut tmp = (*gparent).rb_right;
            if parent != tmp {
                if !tmp.is_null() && rb_is_red(tmp) {
                    rb_set_parent_color(tmp, gparent, RB_BLACK);
                    rb_set_parent_color(parent, gparent, RB_BLACK);
                    node = gparent;
                    parent = rb_parent(node);
                    rb_set_parent_color(node, parent, RB_RED);
                    continue;
                }
                tmp = (*parent).rb_right;
                if node == tmp {
                    tmp = (*node).rb_left;
                    write_once(core::ptr::addr_of_mut!((*parent).rb_right), tmp);
                    write_once(core::ptr::addr_of_mut!((*node).rb_left), parent);
                    if !tmp.is_null() {
                        rb_set_parent_color(tmp, parent, RB_BLACK)
                    }
                    rb_set_parent_color(parent, node, RB_RED);
                    augment_rotate.unwrap_unchecked()(parent, node);
                    parent = node;
                    tmp = (*node).rb_right;
                }
                write_once(core::ptr::addr_of_mut!((*gparent).rb_left), tmp);
                write_once(core::ptr::addr_of_mut!((*parent).rb_right), gparent);
                if !tmp.is_null() {
                    rb_set_parent_color(tmp, gparent, RB_BLACK)
                }
                __rb_rotate_set_parents(gparent, parent, root, RB_RED);
                augment_rotate.unwrap_unchecked()(gparent, parent);
                break;
            } else {
                tmp = (*gparent).rb_left;
                if !tmp.is_null() && rb_is_red(tmp) {
                    rb_set_parent_color(tmp, gparent, RB_BLACK);
                    rb_set_parent_color(parent, gparent, RB_BLACK);
                    node = gparent;
                    parent = rb_parent(node);
                    rb_set_parent_color(node, parent, RB_RED);
                    continue;
                }
                tmp = (*parent).rb_left;
                if node == tmp {
                    tmp = (*node).rb_right;
                    write_once(core::ptr::addr_of_mut!((*parent).rb_left), tmp);
                    write_once(core::ptr::addr_of_mut!((*node).rb_right), parent);
                    if !tmp.is_null() {
                        rb_set_parent_color(tmp, parent, RB_BLACK)
                    }
                    rb_set_parent_color(parent, node, RB_RED);
                    augment_rotate.unwrap_unchecked()(parent, node);
                    parent = node;
                    tmp = (*node).rb_left;
                }
                write_once(core::ptr::addr_of_mut!((*gparent).rb_right), tmp);
                write_once(core::ptr::addr_of_mut!((*parent).rb_left), gparent);
                if !tmp.is_null() {
                    rb_set_parent_color(tmp, gparent, RB_BLACK)
                }
                __rb_rotate_set_parents(gparent, parent, root, RB_RED);
                augment_rotate.unwrap_unchecked()(gparent, parent);
                break;
            }
        }
    }
}

unsafe fn ____rb_erase_color(
    mut parent: *mut rb_node,
    root: *mut rb_root,
    augment_rotate: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut node: *mut rb_node = core::ptr::null_mut();
        loop {
            let mut sibling = (*parent).rb_right;
            if node != sibling {
                if rb_is_red(sibling) {
                    let tmp1 = (*sibling).rb_left;
                    write_once(core::ptr::addr_of_mut!((*parent).rb_right), tmp1);
                    write_once(core::ptr::addr_of_mut!((*sibling).rb_left), parent);
                    rb_set_parent_color(tmp1, parent, RB_BLACK);
                    __rb_rotate_set_parents(parent, sibling, root, RB_RED);
                    augment_rotate.unwrap_unchecked()(parent, sibling);
                    sibling = tmp1;
                }
                let mut tmp1 = (*sibling).rb_right;
                if tmp1.is_null() || rb_is_black(tmp1) {
                    let tmp2 = (*sibling).rb_left;
                    if tmp2.is_null() || rb_is_black(tmp2) {
                        rb_set_parent_color(sibling, parent, RB_RED);
                        if rb_is_red(parent) {
                            rb_set_black(parent)
                        } else {
                            node = parent;
                            parent = rb_parent(node);
                            if !parent.is_null() {
                                continue;
                            }
                        }
                        break;
                    }
                    tmp1 = (*tmp2).rb_right;
                    write_once(core::ptr::addr_of_mut!((*sibling).rb_left), tmp1);
                    write_once(core::ptr::addr_of_mut!((*tmp2).rb_right), sibling);
                    write_once(core::ptr::addr_of_mut!((*parent).rb_right), tmp2);
                    if !tmp1.is_null() {
                        rb_set_parent_color(tmp1, sibling, RB_BLACK)
                    }
                    augment_rotate.unwrap_unchecked()(sibling, tmp2);
                    tmp1 = sibling;
                    sibling = tmp2;
                }
                let tmp2 = (*sibling).rb_left;
                write_once(core::ptr::addr_of_mut!((*parent).rb_right), tmp2);
                write_once(core::ptr::addr_of_mut!((*sibling).rb_left), parent);
                rb_set_parent_color(tmp1, sibling, RB_BLACK);
                if !tmp2.is_null() {
                    rb_set_parent(tmp2, parent)
                }
                __rb_rotate_set_parents(parent, sibling, root, RB_BLACK);
                augment_rotate.unwrap_unchecked()(parent, sibling);
                break;
            } else {
                sibling = (*parent).rb_left;
                if rb_is_red(sibling) {
                    let tmp1 = (*sibling).rb_right;
                    write_once(core::ptr::addr_of_mut!((*parent).rb_left), tmp1);
                    write_once(core::ptr::addr_of_mut!((*sibling).rb_right), parent);
                    rb_set_parent_color(tmp1, parent, RB_BLACK);
                    __rb_rotate_set_parents(parent, sibling, root, RB_RED);
                    augment_rotate.unwrap_unchecked()(parent, sibling);
                    sibling = tmp1
                }
                let mut tmp1 = (*sibling).rb_left;
                if tmp1.is_null() || rb_is_black(tmp1) {
                    let tmp2 = (*sibling).rb_right;
                    if tmp2.is_null() || rb_is_black(tmp2) {
                        rb_set_parent_color(sibling, parent, RB_RED);
                        if rb_is_red(parent) {
                            rb_set_black(parent)
                        } else {
                            node = parent;
                            parent = rb_parent(node);
                            if !parent.is_null() {
                                continue;
                            }
                        }
                        break;
                    }
                    tmp1 = (*tmp2).rb_left;
                    write_once(core::ptr::addr_of_mut!((*sibling).rb_right), tmp1);
                    write_once(core::ptr::addr_of_mut!((*tmp2).rb_left), sibling);
                    write_once(core::ptr::addr_of_mut!((*parent).rb_left), tmp2);
                    if !tmp1.is_null() {
                        rb_set_parent_color(tmp1, sibling, RB_BLACK)
                    }
                    augment_rotate.unwrap_unchecked()(sibling, tmp2);
                    tmp1 = sibling;
                    sibling = tmp2
                }
                let tmp2 = (*sibling).rb_right;
                write_once(core::ptr::addr_of_mut!((*parent).rb_left), tmp2);
                write_once(core::ptr::addr_of_mut!((*sibling).rb_right), parent);
                rb_set_parent_color(tmp1, sibling, RB_BLACK);
                if !tmp2.is_null() {
                    rb_set_parent(tmp2, parent)
                }
                __rb_rotate_set_parents(parent, sibling, root, RB_BLACK);
                augment_rotate.unwrap_unchecked()(parent, sibling);
                break;
            }
        }
    }
}

#[no_mangle]
/// __rb_erase_color follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn __rb_erase_color(
    parent: *mut rb_node,
    root: *mut rb_root,
    augment_rotate: kernel::bindings::RbAugmentRotate,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { ____rb_erase_color(parent, root, augment_rotate.into_option()) }
}
unsafe extern "C" fn dummy_propagate(_: *mut rb_node, _: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
}
unsafe extern "C" fn dummy_copy(_: *mut rb_node, _: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
}
unsafe extern "C" fn dummy_rotate(_: *mut rb_node, _: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
}
static DUMMY_CALLBACKS: rb_augment_callbacks = rb_augment_callbacks {
    propagate: Some(dummy_propagate),
    copy: Some(dummy_copy),
    rotate: Some(dummy_rotate),
};

#[no_mangle]
/// rb_insert_color follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_insert_color(node: *mut rb_node, root: *mut rb_root) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { __rb_insert(node, root, Some(dummy_rotate)) }
}
#[no_mangle]
/// rb_erase follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_erase(node: *mut rb_node, root: *mut rb_root) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let rebalance = __rb_erase_augmented(node, root, &DUMMY_CALLBACKS);
        if !rebalance.is_null() {
            ____rb_erase_color(rebalance, root, Some(dummy_rotate))
        }
    }
}
#[no_mangle]
/// rb_erase_linked follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_erase_linked(
    node: *mut rb_node_linked,
    root: *mut rb_root_linked,
) -> bool {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if (*node).prev.is_null() {
            (*root).rb_leftmost = (*node).next
        } else {
            (*(*node).prev).next = (*node).next
        }
        if !(*node).next.is_null() {
            (*(*node).next).prev = (*node).prev
        }
        rb_erase(core::ptr::addr_of_mut!((*node).node), core::ptr::addr_of_mut!((*root).rb_root));
        rb_clear_linked_node(node);
        !(*root).rb_leftmost.is_null()
    }
}
#[no_mangle]
/// __rb_insert_augmented follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn __rb_insert_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    rotate: kernel::bindings::RbAugmentRotate,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { __rb_insert(node, root, rotate.into_option()) }
}

#[no_mangle]
/// rb_next follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_next(mut node: *const rb_node) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if rb_empty_node(node) {
            return core::ptr::null_mut();
        }
        if !(*node).rb_right.is_null() {
            let mut n = (*node).rb_right;
            while !(*n).rb_left.is_null() {
                n = (*n).rb_left
            }
            return n;
        }
        let mut parent;
        while {
            parent = rb_parent(node);
            !parent.is_null() && node == (*parent).rb_right
        } {
            node = parent
        }
        parent
    }
}
#[no_mangle]
/// rb_prev follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_prev(mut node: *const rb_node) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if rb_empty_node(node) {
            return core::ptr::null_mut();
        }
        if !(*node).rb_left.is_null() {
            let mut n = (*node).rb_left;
            while !(*n).rb_right.is_null() {
                n = (*n).rb_right
            }
            return n;
        }
        let mut parent;
        while {
            parent = rb_parent(node);
            !parent.is_null() && node == (*parent).rb_left
        } {
            node = parent
        }
        parent
    }
}
#[no_mangle]
/// rb_replace_node follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_replace_node(
    victim: *mut rb_node,
    new: *mut rb_node,
    root: *mut rb_root,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let parent = rb_parent(victim);
        core::ptr::copy(victim, new, 1);
        if !(*victim).rb_left.is_null() {
            rb_set_parent((*victim).rb_left, new)
        }
        if !(*victim).rb_right.is_null() {
            rb_set_parent((*victim).rb_right, new)
        }
        __rb_change_child(victim, new, parent, root)
    }
}
#[no_mangle]
/// rb_replace_node_rcu follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_replace_node_rcu(
    victim: *mut rb_node,
    new: *mut rb_node,
    root: *mut rb_root,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let parent = rb_parent(victim);
        core::ptr::copy(victim, new, 1);
        if !(*victim).rb_left.is_null() {
            rb_set_parent((*victim).rb_left, new)
        }
        if !(*victim).rb_right.is_null() {
            rb_set_parent((*victim).rb_right, new)
        }
        __rb_change_child_rcu(victim, new, parent, root)
    }
}
unsafe fn rb_left_deepest_node(mut node: *const rb_node) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        loop {
            if !(*node).rb_left.is_null() {
                node = (*node).rb_left
            } else if !(*node).rb_right.is_null() {
                node = (*node).rb_right
            } else {
                return node as *mut rb_node;
            }
        }
    }
}
#[no_mangle]
/// rb_next_postorder follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_next_postorder(node: *const rb_node) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if node.is_null() {
            return core::ptr::null_mut();
        }
        let parent = rb_parent(node);
        if !parent.is_null() && node == (*parent).rb_left && !(*parent).rb_right.is_null() {
            rb_left_deepest_node((*parent).rb_right)
        } else {
            parent
        }
    }
}
#[no_mangle]
/// rb_first_postorder follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe extern "C" fn rb_first_postorder(root: *const rb_root) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if (*root).rb_node.is_null() {
            core::ptr::null_mut()
        } else {
            rb_left_deepest_node((*root).rb_node)
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
