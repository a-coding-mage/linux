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

#[repr(C)]
pub struct rb_augment_callbacks {
    pub propagate: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>,
    pub copy: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>,
    pub rotate: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>,
}

extern "C" {
    pub fn __rb_insert_augmented(node: *mut rb_node, root: *mut rb_root,
        augment_rotate: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>);
    pub fn __rb_erase_color(parent: *mut rb_node, root: *mut rb_root,
        augment_rotate: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>);
}

#[inline]
pub unsafe fn rb_insert_augmented(node: *mut rb_node, root: *mut rb_root,
    augment: *const rb_augment_callbacks) {
    __rb_insert_augmented(node, root, (*augment).rotate);
}

#[inline]
pub unsafe fn rb_insert_augmented_cached(node: *mut rb_node, root: *mut rb_root_cached,
    newleft: bool, augment: *const rb_augment_callbacks) {
    if newleft { (*root).rb_leftmost = node; }
    rb_insert_augmented(node, &mut (*root).rb_root, augment);
}

#[inline(always)]
pub unsafe fn rb_add_augmented_cached(node: *mut rb_node, tree: *mut rb_root_cached,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
    augment: *const rb_augment_callbacks) -> *mut rb_node {
    let mut link: *mut *mut rb_node = &mut (*tree).rb_root.rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut leftmost = true;
    while !(*link).is_null() {
        parent = *link;
        if less.unwrap()(node, parent) { link = &mut (*parent).rb_left; }
        else { link = &mut (*parent).rb_right; leftmost = false; }
    }
    rb_link_node(node, parent, link);
    (*augment).propagate.unwrap()(parent, core::ptr::null_mut());
    rb_insert_augmented_cached(node, tree, leftmost, augment);
    if leftmost { node } else { core::ptr::null_mut() }
}

/* Templates for augmented callback declarations. */
#[macro_export]
macro_rules! RB_DECLARE_CALLBACKS {
    ($staticness:ident, $name:ident, $struct:ty, $field:ident, $augmented:ident, $compute:ident) => {
        #[inline] unsafe fn $name _propagate(rb: *mut rb_node, stop: *mut rb_node) {
            let mut rb = rb;
            while rb != stop {
                let node: *mut $struct = rb_entry!(rb, $struct, $field);
                if $compute(node, true) { break; }
                rb = rb_parent(&mut (*node).$field);
            }
        }
        #[inline] unsafe fn $name _copy(old_rb: *mut rb_node, new_rb: *mut rb_node) {
            let old: *mut $struct = rb_entry!(old_rb, $struct, $field);
            let new: *mut $struct = rb_entry!(new_rb, $struct, $field);
            (*new).$augmented = (*old).$augmented;
        }
        unsafe fn $name _rotate(old_rb: *mut rb_node, new_rb: *mut rb_node) {
            let old: *mut $struct = rb_entry!(old_rb, $struct, $field);
            let new: *mut $struct = rb_entry!(new_rb, $struct, $field);
            (*new).$augmented = (*old).$augmented;
            $compute(old, false);
        }
    };
}

#[macro_export]
macro_rules! RB_DECLARE_CALLBACKS_MAX {
    ($staticness:ident, $name:ident, $struct:ty, $field:ident, $rbtype:ty, $augmented:ident, $compute:ident) => {
        #[inline] unsafe fn $name _compute_max(node: *mut $struct, exit: bool) -> bool {
            let mut max: $rbtype = $compute(node);
            if !(*node).$field.rb_left.is_null() { let child = rb_entry!((*node).$field.rb_left, $struct, $field); if (*child).$augmented > max { max = (*child).$augmented; } }
            if !(*node).$field.rb_right.is_null() { let child = rb_entry!((*node).$field.rb_right, $struct, $field); if (*child).$augmented > max { max = (*child).$augmented; } }
            if exit && (*node).$augmented == max { return true; }
            (*node).$augmented = max; false
        }
        RB_DECLARE_CALLBACKS!($staticness, $name, $struct, $field, $augmented, $name _compute_max);
    };
}

pub const RB_RED: i32 = 0;
pub const RB_BLACK: i32 = 1;

#[inline] pub unsafe fn __rb_parent(pc: usize) -> *mut rb_node { (pc & !3) as *mut rb_node }
#[inline] pub fn __rb_color(pc: usize) -> usize { pc & 1 }
#[inline] pub fn __rb_is_black(pc: usize) -> bool { __rb_color(pc) != 0 }
#[inline] pub fn __rb_is_red(pc: usize) -> bool { __rb_color(pc) == 0 }
#[inline] pub unsafe fn rb_color(rb: *mut rb_node) -> usize { __rb_color((*rb).__rb_parent_color) }
#[inline] pub unsafe fn rb_is_red(rb: *mut rb_node) -> bool { __rb_is_red((*rb).__rb_parent_color) }
#[inline] pub unsafe fn rb_is_black(rb: *mut rb_node) -> bool { __rb_is_black((*rb).__rb_parent_color) }

#[inline] pub unsafe fn rb_set_parent(rb: *mut rb_node, p: *mut rb_node) { (*rb).__rb_parent_color = rb_color(rb).wrapping_add(p as usize); }
#[inline] pub unsafe fn rb_set_parent_color(rb: *mut rb_node, p: *mut rb_node, color: i32) { (*rb).__rb_parent_color = (p as usize).wrapping_add(color as usize); }

#[inline] pub unsafe fn __rb_change_child(old: *mut rb_node, new: *mut rb_node, parent: *mut rb_node, root: *mut rb_root) {
    if !parent.is_null() { if (*parent).rb_left == old { (*parent).rb_left = new; } else { (*parent).rb_right = new; } } else { (*root).rb_node = new; }
}
#[inline] pub unsafe fn __rb_change_child_rcu(old: *mut rb_node, new: *mut rb_node, parent: *mut rb_node, root: *mut rb_root) {
    __rb_change_child(old, new, parent, root);
}

#[inline(always)]
pub unsafe fn __rb_erase_augmented(node: *mut rb_node, root: *mut rb_root, augment: *const rb_augment_callbacks) -> *mut rb_node {
    let mut child = (*node).rb_right;
    let mut tmp = (*node).rb_left;
    let parent: *mut rb_node;
    let rebalance: *mut rb_node;
    let pc: usize;
    if tmp.is_null() {
        pc = (*node).__rb_parent_color; parent = __rb_parent(pc); __rb_change_child(node, child, parent, root);
        if !child.is_null() { (*child).__rb_parent_color = pc; rebalance = core::ptr::null_mut(); }
        else { rebalance = if __rb_is_black(pc) { parent } else { core::ptr::null_mut() }; }
        tmp = parent;
    } else if child.is_null() {
        (*tmp).__rb_parent_color = (*node).__rb_parent_color; pc = (*node).__rb_parent_color; parent = __rb_parent(pc); __rb_change_child(node, tmp, parent, root); rebalance = core::ptr::null_mut(); tmp = parent;
    } else {
        let mut successor = child; let child2: *mut rb_node;
        tmp = (*child).rb_left;
        if tmp.is_null() { parent = successor; child2 = (*successor).rb_right; (*augment).copy.unwrap()(node, successor); }
        else { loop { parent = successor; successor = tmp; tmp = (*tmp).rb_left; if tmp.is_null() { break; } } child2 = (*successor).rb_right; (*parent).rb_left = child2; (*successor).rb_right = child; rb_set_parent(child, successor); (*augment).copy.unwrap()(node, successor); (*augment).propagate.unwrap()(parent, successor); }
        tmp = (*node).rb_left; (*successor).rb_left = tmp; rb_set_parent(tmp, successor); pc = (*node).__rb_parent_color; tmp = __rb_parent(pc); __rb_change_child(node, successor, tmp, root);
        if !child2.is_null() { rb_set_parent_color(child2, parent, RB_BLACK); rebalance = core::ptr::null_mut(); } else { rebalance = if rb_is_black(successor) { parent } else { core::ptr::null_mut() }; }
        (*successor).__rb_parent_color = pc; tmp = successor;
    }
    (*augment).propagate.unwrap()(tmp, core::ptr::null_mut()); rebalance
}

#[inline(always)] pub unsafe fn rb_erase_augmented(node: *mut rb_node, root: *mut rb_root, augment: *const rb_augment_callbacks) { let rebalance = __rb_erase_augmented(node, root, augment); if !rebalance.is_null() { __rb_erase_color(rebalance, root, (*augment).rotate); } }
#[inline(always)] pub unsafe fn rb_erase_augmented_cached(node: *mut rb_node, root: *mut rb_root_cached, augment: *const rb_augment_callbacks) { if (*root).rb_leftmost == node { (*root).rb_leftmost = rb_next(node); } rb_erase_augmented(node, &mut (*root).rb_root, augment); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
