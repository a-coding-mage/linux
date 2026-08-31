/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
  Red Black Trees
  (C) 1999  Andrea Arcangeli <andrea@suse.de>
  (C) 2002  David Woodhouse <dwmw2@infradead.org>
  (C) 2012  Michel Lespinasse <walken@google.com>


  tools/linux/include/linux/rbtree_augmented.h

  Copied from:
  linux/include/linux/rbtree_augmented.h
*/

/* Dependencies from the C header:
 * #include <linux/compiler.h>
 * #include <linux/rbtree.h>
 */

use core::ffi::c_ulong;

/*
 * Please note - only struct rb_augment_callbacks and the prototypes for
 * rb_insert_augmented() and rb_erase_augmented() are intended to be public.
 * The rest are implementation details you are not expected to depend on.
 *
 * See Documentation/core-api/rbtree.rst for documentation and samples.
 */

#[repr(C)]
pub struct rb_augment_callbacks {
    pub propagate: Option<unsafe extern "C" fn(node: *mut rb_node, stop: *mut rb_node)>,
    pub copy: Option<unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node)>,
    pub rotate: Option<unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node)>,
}

unsafe extern "C" {
    pub fn __rb_insert_augmented(
        node: *mut rb_node,
        root: *mut rb_root,
        augment_rotate: Option<unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node)>,
    );
}

/*
 * Fixup the rbtree and update the augmented information when rebalancing.
 *
 * On insertion, the user must update the augmented information on the path
 * leading to the inserted node, then call rb_link_node() as usual and
 * rb_insert_augmented() instead of the usual rb_insert_color() call.
 * If rb_insert_augmented() rebalances the rbtree, it will callback into
 * a user provided function to update the augmented information on the
 * affected subtrees.
 */
#[inline]
pub unsafe fn rb_insert_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    augment: *const rb_augment_callbacks,
) {
    unsafe {
        __rb_insert_augmented(node, root, (*augment).rotate);
    }
}

#[inline]
pub unsafe fn rb_insert_augmented_cached(
    node: *mut rb_node,
    root: *mut rb_root_cached,
    newleft: bool,
    augment: *const rb_augment_callbacks,
) {
    unsafe {
        if newleft {
            (*root).rb_leftmost = node;
        }
        rb_insert_augmented(node, &mut (*root).rb_root, augment);
    }
}

/*
 * Template for declaring augmented rbtree callbacks (generic case)
 *
 * RBSTATIC:    'static' or empty
 * RBNAME:      name of the rb_augment_callbacks structure
 * RBSTRUCT:    struct type of the tree nodes
 * RBFIELD:     name of struct rb_node field within RBSTRUCT
 * RBAUGMENTED: name of field within RBSTRUCT holding data for subtree
 * RBCOMPUTE:   name of function that recomputes the RBAUGMENTED data
 *
 * C macro RB_DECLARE_CALLBACKS expands to:
 * - RBNAME_propagate(rb, stop): walk parents with rb_parent(&node->RBFIELD)
 *   until stop or RBCOMPUTE(node, true) requests exit.
 * - RBNAME_copy(rb_old, rb_new): copy old->RBAUGMENTED to new->RBAUGMENTED.
 * - RBNAME_rotate(rb_old, rb_new): copy old augmented data to new, then
 *   recompute old with RBCOMPUTE(old, false).
 * - RBSTATIC const struct rb_augment_callbacks RBNAME with those callbacks.
 *
 * This token-pasting, field-name, and type-parameter macro has no direct
 * file-local Rust item equivalent without the caller's RBSTRUCT/RBFIELD.
 */

/*
 * Template for declaring augmented rbtree callbacks,
 * computing RBAUGMENTED scalar as max(RBCOMPUTE(node)) for all subtree nodes.
 *
 * RBSTATIC:    'static' or empty
 * RBNAME:      name of the rb_augment_callbacks structure
 * RBSTRUCT:    struct type of the tree nodes
 * RBFIELD:     name of struct rb_node field within RBSTRUCT
 * RBTYPE:      type of the RBAUGMENTED field
 * RBAUGMENTED: name of RBTYPE field within RBSTRUCT holding data for subtree
 * RBCOMPUTE:   name of function that returns the per-node RBTYPE scalar
 *
 * C macro RB_DECLARE_CALLBACKS_MAX expands to a RBNAME_compute_max helper
 * over node->RBFIELD.rb_left and node->RBFIELD.rb_right, then invokes
 * RB_DECLARE_CALLBACKS with RBNAME_compute_max.
 */

pub const RB_RED: i32 = 0;
pub const RB_BLACK: i32 = 1;

#[inline]
pub unsafe fn __rb_parent(pc: c_ulong) -> *mut rb_node {
    (pc & !3) as *mut rb_node
}

#[inline]
pub fn __rb_color(pc: c_ulong) -> c_ulong {
    pc & 1
}

#[inline]
pub fn __rb_is_black(pc: c_ulong) -> c_ulong {
    __rb_color(pc)
}

#[inline]
pub fn __rb_is_red(pc: c_ulong) -> bool {
    __rb_color(pc) == 0
}

#[inline]
pub unsafe fn rb_color(rb: *const rb_node) -> c_ulong {
    unsafe { __rb_color((*rb).__rb_parent_color) }
}

#[inline]
pub unsafe fn rb_is_red(rb: *const rb_node) -> bool {
    unsafe { __rb_is_red((*rb).__rb_parent_color) }
}

#[inline]
pub unsafe fn rb_is_black(rb: *const rb_node) -> c_ulong {
    unsafe { __rb_is_black((*rb).__rb_parent_color) }
}

#[inline]
pub unsafe fn rb_set_parent(rb: *mut rb_node, p: *mut rb_node) {
    unsafe {
        (*rb).__rb_parent_color = rb_color(rb) + p as c_ulong;
    }
}

#[inline]
pub unsafe fn rb_set_parent_color(rb: *mut rb_node, p: *mut rb_node, color: i32) {
    unsafe {
        (*rb).__rb_parent_color = p as c_ulong + color as c_ulong;
    }
}

#[inline]
pub unsafe fn __rb_change_child(
    old: *mut rb_node,
    new: *mut rb_node,
    parent: *mut rb_node,
    root: *mut rb_root,
) {
    unsafe {
        if !parent.is_null() {
            if (*parent).rb_left == old {
                core::ptr::write_volatile(&mut (*parent).rb_left, new);
            } else {
                core::ptr::write_volatile(&mut (*parent).rb_right, new);
            }
        } else {
            core::ptr::write_volatile(&mut (*root).rb_node, new);
        }
    }
}

unsafe extern "C" {
    pub fn __rb_erase_color(
        parent: *mut rb_node,
        root: *mut rb_root,
        augment_rotate: Option<unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node)>,
    );
}

#[inline(always)]
pub unsafe fn __rb_erase_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    augment: *const rb_augment_callbacks,
) -> *mut rb_node {
    unsafe {
        let mut child: *mut rb_node = (*node).rb_right;
        let mut tmp: *mut rb_node = (*node).rb_left;
        let mut parent: *mut rb_node;
        let rebalance: *mut rb_node;
        let mut pc: c_ulong;

        if tmp.is_null() {
            /*
             * Case 1: node to erase has no more than 1 child (easy!)
             *
             * Note that if there is one child it must be red due to 5)
             * and node must be black due to 4). We adjust colors locally
             * so as to bypass __rb_erase_color() later on.
             */
            pc = (*node).__rb_parent_color;
            parent = __rb_parent(pc);
            __rb_change_child(node, child, parent, root);
            if !child.is_null() {
                (*child).__rb_parent_color = pc;
                rebalance = core::ptr::null_mut();
            } else {
                rebalance = if __rb_is_black(pc) != 0 {
                    parent
                } else {
                    core::ptr::null_mut()
                };
            }
            tmp = parent;
        } else if child.is_null() {
            /* Still case 1, but this time the child is node->rb_left */
            pc = (*node).__rb_parent_color;
            (*tmp).__rb_parent_color = pc;
            parent = __rb_parent(pc);
            __rb_change_child(node, tmp, parent, root);
            rebalance = core::ptr::null_mut();
            tmp = parent;
        } else {
            let mut successor: *mut rb_node = child;
            let child2: *mut rb_node;

            tmp = (*child).rb_left;
            if tmp.is_null() {
                /*
                 * Case 2: node's successor is its right child
                 *
                 *    (n)          (s)
                 *    / \          / \
                 *  (x) (s)  ->  (x) (c)
                 *        \
                 *        (c)
                 */
                parent = successor;
                child2 = (*successor).rb_right;

                ((*augment).copy.expect("rb_augment_callbacks.copy"))(node, successor);
            } else {
                /*
                 * Case 3: node's successor is leftmost under
                 * node's right child subtree
                 *
                 *    (n)          (s)
                 *    / \          / \
                 *  (x) (y)  ->  (x) (y)
                 *      /            /
                 *    (p)          (p)
                 *    /            /
                 *  (s)          (c)
                 *    \
                 *    (c)
                 */
                loop {
                    parent = successor;
                    successor = tmp;
                    tmp = (*tmp).rb_left;
                    if tmp.is_null() {
                        break;
                    }
                }
                child2 = (*successor).rb_right;
                core::ptr::write_volatile(&mut (*parent).rb_left, child2);
                core::ptr::write_volatile(&mut (*successor).rb_right, child);
                rb_set_parent(child, successor);

                ((*augment).copy.expect("rb_augment_callbacks.copy"))(node, successor);
                ((*augment).propagate.expect("rb_augment_callbacks.propagate"))(parent, successor);
            }

            tmp = (*node).rb_left;
            core::ptr::write_volatile(&mut (*successor).rb_left, tmp);
            rb_set_parent(tmp, successor);

            pc = (*node).__rb_parent_color;
            tmp = __rb_parent(pc);
            __rb_change_child(node, successor, tmp, root);

            if !child2.is_null() {
                (*successor).__rb_parent_color = pc;
                rb_set_parent_color(child2, parent, RB_BLACK);
                rebalance = core::ptr::null_mut();
            } else {
                let pc2: c_ulong = (*successor).__rb_parent_color;
                (*successor).__rb_parent_color = pc;
                rebalance = if __rb_is_black(pc2) != 0 {
                    parent
                } else {
                    core::ptr::null_mut()
                };
            }
            tmp = successor;
        }

        ((*augment).propagate.expect("rb_augment_callbacks.propagate"))(tmp, core::ptr::null_mut());
        rebalance
    }
}

#[inline(always)]
pub unsafe fn rb_erase_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    augment: *const rb_augment_callbacks,
) {
    unsafe {
        let rebalance: *mut rb_node = __rb_erase_augmented(node, root, augment);
        if !rebalance.is_null() {
            __rb_erase_color(rebalance, root, (*augment).rotate);
        }
    }
}

#[inline(always)]
pub unsafe fn rb_erase_augmented_cached(
    node: *mut rb_node,
    root: *mut rb_root_cached,
    augment: *const rb_augment_callbacks,
) {
    unsafe {
        if (*root).rb_leftmost == node {
            (*root).rb_leftmost = rb_next(node);
        }
        rb_erase_augmented(node, &mut (*root).rb_root, augment);
    }
}
