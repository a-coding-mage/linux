/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
  Red Black Trees
  (C) 1999  Andrea Arcangeli <andrea@suse.de>
  (C) 2002  David Woodhouse <dwmw2@infradead.org>
  (C) 2012  Michel Lespinasse <walken@google.com>

  linux/include/linux/rbtree_augmented.h
*/

/* Dependencies: linux/compiler.h, linux/rbtree.h, linux/rcupdate.h. */

/* Only rb_augment_callbacks and the public operation prototypes are intended
 * to be public; the remainder are implementation details. */

use super::declarations::*;
/// Actual C callback structure, including its nullable fields.
pub use kernel::bindings::rb_augment_callbacks;

/// Actual bindings retain nullable rotation arguments and the original outer CFI.
pub use kernel::bindings::{__rb_erase_color, __rb_insert_augmented, RbAugmentRotate};

#[inline]
/// rb_insert_augmented follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_insert_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    augment: *const rb_augment_callbacks,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        __rb_insert_augmented(node, root, RbAugmentRotate::from_option((*augment).rotate));
    }
}

#[inline]
/// rb_insert_augmented_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_insert_augmented_cached(
    node: *mut rb_node,
    root: *mut rb_root_cached,
    newleft: bool,
    augment: *const rb_augment_callbacks,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if newleft {
            (*root).rb_leftmost = node;
        }
        rb_insert_augmented(node, core::ptr::addr_of_mut!((*root).rb_root), augment);
    }
}

#[inline(always)]
/// rb_add_augmented_cached follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_add_augmented_cached(
    node: *mut rb_node,
    tree: *mut rb_root_cached,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
    augment: *const rb_augment_callbacks,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let mut link: *mut *mut rb_node = core::ptr::addr_of_mut!((*tree).rb_root.rb_node);
        let mut parent: *mut rb_node = core::ptr::null_mut();
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
        (*augment).propagate.unwrap_unchecked()(parent, core::ptr::null_mut());
        rb_insert_augmented_cached(node, tree, leftmost, augment);
        if leftmost {
            node
        } else {
            core::ptr::null_mut()
        }
    }
}

/// Define the table and its original callable helper operations.
///
/// Rust callers supply explicit helper identifiers instead of C token pasting:
/// `RB_DECLARE_CALLBACKS!(pub NAME, [propagate, copy, rotate], Type, node, max, compute)`.
#[macro_export]
macro_rules! RB_DECLARE_CALLBACKS {
    ($vis:vis $name:ident, [$propagate:ident, $copy:ident, $rotate:ident],
     $struct:ty, $field:ident, $augmented:ident, $compute:expr) => {
            /// Propagate the aggregate along the original parent chain.
            ///
            /// # Safety
            /// Nodes and the stop point satisfy the original augmented callback contract.
            $vis unsafe extern "C" fn $propagate(mut rb: *mut rb_node, stop: *mut rb_node) {
                // SAFETY: Original augmented callback traversal contract.
                unsafe {
                    while rb != stop {
                        let node = rb_entry!(rb, $struct, $field);
                        if ($compute)(node, true) { break; }
                        rb = rb_parent(core::ptr::addr_of!((*node).$field));
                    }
                }
            }
            /// Copy the aggregate between original embedded nodes.
            ///
            /// # Safety
            /// Both nodes embed the declared field and the old aggregate is initialized.
            $vis unsafe extern "C" fn $copy(old_rb: *mut rb_node, new_rb: *mut rb_node) {
                // SAFETY: Both nodes embed the declared field.
                unsafe {
                    let old = rb_entry!(old_rb, $struct, $field);
                    let new = rb_entry!(new_rb, $struct, $field);
                    (*new).$augmented = (*old).$augmented;
                }
            }
            /// Update aggregates after an original tree rotation.
            ///
            /// # Safety
            /// Nodes satisfy the original rotation callback contract.
            $vis unsafe extern "C" fn $rotate(old_rb: *mut rb_node, new_rb: *mut rb_node) {
                // SAFETY: Rotation callback receives the original subtree roots.
                unsafe {
                    $copy(old_rb, new_rb);
                    let old = rb_entry!(old_rb, $struct, $field);
                    ($compute)(old, false);
                }
            }
        /// Original callback table with stable storage and named helper functions.
        $vis static $name: rb_augment_callbacks = rb_augment_callbacks {
            propagate: Some($propagate), copy: Some($copy), rotate: Some($rotate)
        };
    };
}

/// Define maximum-aggregate callbacks and their named compute helper.
///
/// The explicit helper list is `[propagate, copy, rotate, compute_max]`.
#[macro_export]
macro_rules! RB_DECLARE_CALLBACKS_MAX {
    ($vis:vis $name:ident, [$propagate:ident, $copy:ident, $rotate:ident, $compute_max:ident],
     $struct:ty, $field:ident, $rbtype:ty, $augmented:ident, $compute:expr) => {
            /// Recompute a subtree maximum, retaining the original early-exit rule.
            ///
            /// # Safety
            /// The node and all non-null children satisfy the original aggregate contract.
            $vis unsafe extern "C" fn $compute_max(node: *mut $struct, exit: bool) -> bool {
                // SAFETY: The node and each nonnull child have the declared layout.
                unsafe {
                    let mut max: $rbtype = ($compute)(node);
                    for rb in [(*node).$field.rb_left, (*node).$field.rb_right] {
                        if !rb.is_null() {
                            let child = rb_entry!(rb, $struct, $field);
                            if (*child).$augmented > max { max = (*child).$augmented; }
                        }
                    }
                    if exit && (*node).$augmented == max { return true; }
                    (*node).$augmented = max;
                    false
                }
            }
        $crate::RB_DECLARE_CALLBACKS!($vis $name, [$propagate, $copy, $rotate],
                                    $struct, $field, $augmented, $compute_max);
    };
}

/// RB_RED from the original Linux rbtree interface.
pub const RB_RED: i32 = 0;
/// RB_BLACK from the original Linux rbtree interface.
pub const RB_BLACK: i32 = 1;

#[inline]
/// __rb_parent follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn __rb_parent(pc: usize) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    (pc & !3) as *mut rb_node
}
#[inline]
/// __rb_color follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub fn __rb_color(pc: usize) -> usize {
    pc & 1
}
#[inline]
/// __rb_is_black follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub fn __rb_is_black(pc: usize) -> bool {
    __rb_color(pc) != 0
}
#[inline]
/// __rb_is_red follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub fn __rb_is_red(pc: usize) -> bool {
    __rb_color(pc) == 0
}
#[inline]
/// rb_color follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_color(rb: *const rb_node) -> usize {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { __rb_color((*rb).__rb_parent_color) }
}
#[inline]
/// rb_is_red follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_is_red(rb: *const rb_node) -> bool {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { __rb_is_red((*rb).__rb_parent_color) }
}
#[inline]
/// rb_is_black follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_is_black(rb: *const rb_node) -> bool {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe { __rb_is_black((*rb).__rb_parent_color) }
}

#[inline]
/// rb_set_parent follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_set_parent(rb: *mut rb_node, p: *mut rb_node) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        (*rb).__rb_parent_color = rb_color(rb).wrapping_add(p as usize);
    }
}
#[inline]
/// rb_set_parent_color follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_set_parent_color(rb: *mut rb_node, p: *mut rb_node, color: i32) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        (*rb).__rb_parent_color = (p as usize).wrapping_add(color as usize);
    }
}

#[inline]
/// __rb_change_child follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn __rb_change_child(
    old: *mut rb_node,
    new: *mut rb_node,
    parent: *mut rb_node,
    root: *mut rb_root,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if !parent.is_null() {
            if (*parent).rb_left == old {
                write_once(core::ptr::addr_of_mut!((*parent).rb_left), new);
            } else {
                write_once(core::ptr::addr_of_mut!((*parent).rb_right), new);
            }
        } else {
            write_once(core::ptr::addr_of_mut!((*root).rb_node), new);
        }
    }
}
#[inline]
/// __rb_change_child_rcu follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn __rb_change_child_rcu(
    old: *mut rb_node,
    new: *mut rb_node,
    parent: *mut rb_node,
    root: *mut rb_root,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let slot = if parent.is_null() {
            core::ptr::addr_of_mut!((*root).rb_node)
        } else if (*parent).rb_left == old {
            core::ptr::addr_of_mut!((*parent).rb_left)
        } else {
            core::ptr::addr_of_mut!((*parent).rb_right)
        };
        publish(slot, new);
    }
}

#[inline(always)]
/// __rb_erase_augmented follows the corresponding Linux rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn __rb_erase_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    augment: *const rb_augment_callbacks,
) -> *mut rb_node {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let child = (*node).rb_right;
        let mut tmp = (*node).rb_left;
        let mut parent: *mut rb_node;
        let rebalance: *mut rb_node;
        let pc: usize;
        if tmp.is_null() {
            pc = (*node).__rb_parent_color;
            parent = __rb_parent(pc);
            __rb_change_child(node, child, parent, root);
            if !child.is_null() {
                (*child).__rb_parent_color = pc;
                rebalance = core::ptr::null_mut();
            } else {
                rebalance = if __rb_is_black(pc) {
                    parent
                } else {
                    core::ptr::null_mut()
                };
            }
            tmp = parent;
        } else if child.is_null() {
            (*tmp).__rb_parent_color = (*node).__rb_parent_color;
            pc = (*node).__rb_parent_color;
            parent = __rb_parent(pc);
            __rb_change_child(node, tmp, parent, root);
            rebalance = core::ptr::null_mut();
            tmp = parent;
        } else {
            let mut successor = child;
            let child2: *mut rb_node;
            tmp = (*child).rb_left;
            if tmp.is_null() {
                parent = successor;
                child2 = (*successor).rb_right;
                (*augment).copy.unwrap_unchecked()(node, successor);
            } else {
                loop {
                    parent = successor;
                    successor = tmp;
                    tmp = (*tmp).rb_left;
                    if tmp.is_null() {
                        break;
                    }
                }
                child2 = (*successor).rb_right;
                write_once(core::ptr::addr_of_mut!((*parent).rb_left), child2);
                write_once(core::ptr::addr_of_mut!((*successor).rb_right), child);
                rb_set_parent(child, successor);
                (*augment).copy.unwrap_unchecked()(node, successor);
                (*augment).propagate.unwrap_unchecked()(parent, successor);
            }
            tmp = (*node).rb_left;
            write_once(core::ptr::addr_of_mut!((*successor).rb_left), tmp);
            rb_set_parent(tmp, successor);
            pc = (*node).__rb_parent_color;
            tmp = __rb_parent(pc);
            __rb_change_child(node, successor, tmp, root);
            if !child2.is_null() {
                rb_set_parent_color(child2, parent, RB_BLACK);
                rebalance = core::ptr::null_mut();
            } else {
                rebalance = if rb_is_black(successor) {
                    parent
                } else {
                    core::ptr::null_mut()
                };
            }
            (*successor).__rb_parent_color = pc;
            tmp = successor;
        }
        (*augment).propagate.unwrap_unchecked()(tmp, core::ptr::null_mut());
        rebalance
    }
}

#[inline(always)]
/// rb_erase_augmented follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_erase_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    augment: *const rb_augment_callbacks,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        let rebalance = __rb_erase_augmented(node, root, augment);
        if !rebalance.is_null() {
            __rb_erase_color(
                rebalance,
                root,
                RbAugmentRotate::from_option((*augment).rotate),
            );
        }
    }
}
#[inline(always)]
/// rb_erase_augmented_cached follows the original C rbtree contract.
///
/// # Safety
/// Pointers and callbacks must satisfy the original C preconditions.
pub unsafe fn rb_erase_augmented_cached(
    node: *mut rb_node,
    root: *mut rb_root_cached,
    augment: *const rb_augment_callbacks,
) {
    // SAFETY: The caller guarantees the original C pointer and callback contracts.
    unsafe {
        if (*root).rb_leftmost == node {
            (*root).rb_leftmost = rb_next(node);
        }
        rb_erase_augmented(node, core::ptr::addr_of_mut!((*root).rb_root), augment);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
