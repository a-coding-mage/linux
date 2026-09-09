/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Red Black Trees -- translation of linux/include/linux/rbtree.h */

/* Types and helpers are supplied by the corresponding Linux bindings. */

#[inline]
pub unsafe fn rb_parent(r: *mut rb_node) -> *mut rb_node {
    ((*r).__rb_parent_color & !3usize) as *mut rb_node
}

/* rb_entry and rb_entry_safe depend on the source-language container_of/typeof
 * facilities; retain them as macros for callers providing those facilities. */
#[macro_export]
macro_rules! rb_entry {
    ($ptr:expr, $ty:ty, $member:ident) => { container_of!($ptr, $ty, $member) };
}

#[inline]
pub unsafe fn rb_empty_root(root: *const rb_root) -> bool { (*root).rb_node == core::ptr::null_mut() }

#[inline]
pub unsafe fn rb_empty_node(node: *const rb_node) -> bool {
    (*node).__rb_parent_color == node as usize
}

#[inline]
pub unsafe fn rb_clear_node(node: *mut rb_node) { (*node).__rb_parent_color = node as usize; }

extern "C" {
    pub fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    pub fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    pub fn rb_erase_linked(node: *mut rb_node_linked, root: *mut rb_root_linked) -> bool;
    pub fn rb_next(node: *const rb_node) -> *mut rb_node;
    pub fn rb_prev(node: *const rb_node) -> *mut rb_node;
    pub fn rb_first_postorder(root: *const rb_root) -> *mut rb_node;
    pub fn rb_next_postorder(node: *const rb_node) -> *mut rb_node;
    pub fn rb_replace_node(victim: *mut rb_node, new: *mut rb_node, root: *mut rb_root);
    pub fn rb_replace_node_rcu(victim: *mut rb_node, new: *mut rb_node, root: *mut rb_root);
}

#[inline]
pub unsafe fn rb_first(root: *const rb_root) -> *mut rb_node {
    let mut n = (*root).rb_node;
    if n.is_null() { return core::ptr::null_mut(); }
    while !(*n).rb_left.is_null() { n = (*n).rb_left; }
    n
}

#[inline]
pub unsafe fn rb_last(root: *const rb_root) -> *mut rb_node {
    let mut n = (*root).rb_node;
    if n.is_null() { return core::ptr::null_mut(); }
    while !(*n).rb_right.is_null() { n = (*n).rb_right; }
    n
}

#[inline]
pub unsafe fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node) {
    (*node).__rb_parent_color = parent as usize;
    (*node).rb_left = core::ptr::null_mut();
    (*node).rb_right = core::ptr::null_mut();
    *link = node;
}

#[inline]
pub unsafe fn rb_link_node_rcu(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node) {
    (*node).__rb_parent_color = parent as usize;
    (*node).rb_left = core::ptr::null_mut();
    (*node).rb_right = core::ptr::null_mut();
    *link = node;
}

#[inline]
pub unsafe fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node { (*root).rb_leftmost }

#[inline]
pub unsafe fn rb_insert_color_cached(node: *mut rb_node, root: *mut rb_root_cached, leftmost: bool) {
    if leftmost { (*root).rb_leftmost = node; }
    rb_insert_color(node, &mut (*root).rb_root);
}

#[inline]
pub unsafe fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached) -> *mut rb_node {
    let mut leftmost = core::ptr::null_mut();
    if (*root).rb_leftmost == node {
        leftmost = rb_next(node);
        (*root).rb_leftmost = leftmost;
    }
    rb_erase(node, &mut (*root).rb_root);
    leftmost
}

#[inline]
pub unsafe fn rb_replace_node_cached(victim: *mut rb_node, new: *mut rb_node, root: *mut rb_root_cached) {
    if (*root).rb_leftmost == victim { (*root).rb_leftmost = new; }
    rb_replace_node(victim, new, &mut (*root).rb_root);
}

/* The insertion/search helpers below are direct Rust equivalents of the
 * corresponding inline C helpers. */
#[inline]
pub unsafe fn rb_add_cached(node: *mut rb_node, tree: *mut rb_root_cached,
    less: unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool) -> *mut rb_node {
    let mut link = &mut (*tree).rb_root.rb_node as *mut *mut rb_node;
    let mut parent = core::ptr::null_mut();
    let mut leftmost = true;
    while !(*link).is_null() { parent = *link; if less(node, parent) { link = &mut (*parent).rb_left; } else { link = &mut (*parent).rb_right; leftmost = false; } }
    rb_link_node(node, parent, link); rb_insert_color_cached(node, tree, leftmost);
    if leftmost { node } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn rb_find(key: *const core::ffi::c_void, tree: *const rb_root,
    cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32) -> *mut rb_node {
    let mut node = (*tree).rb_node;
    while !node.is_null() { let c = cmp(key, node); if c < 0 { node = (*node).rb_left; } else if c > 0 { node = (*node).rb_right; } else { return node; } }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn rb_find_add(node: *mut rb_node, tree: *mut rb_root,
    cmp: unsafe extern "C" fn(*mut rb_node, *const rb_node) -> i32) -> *mut rb_node {
    let mut link = &mut (*tree).rb_node as *mut *mut rb_node;
    let mut parent = core::ptr::null_mut();
    while !(*link).is_null() { parent = *link; let c = cmp(node, parent); if c < 0 { link = &mut (*parent).rb_left; } else if c > 0 { link = &mut (*parent).rb_right; } else { return parent; } }
    rb_link_node(node, parent, link); rb_insert_color(node, tree); core::ptr::null_mut()
}

#[inline]
pub unsafe fn rb_find_add_cached(node: *mut rb_node, tree: *mut rb_root_cached,
    cmp: unsafe extern "C" fn(*const rb_node, *const rb_node) -> i32) -> *mut rb_node {
    let mut link = &mut (*tree).rb_root.rb_node as *mut *mut rb_node;
    let mut parent = core::ptr::null_mut(); let mut leftmost = true;
    while !(*link).is_null() { parent = *link; let c = cmp(node, parent); if c < 0 { link = &mut (*parent).rb_left; } else if c > 0 { link = &mut (*parent).rb_right; leftmost = false; } else { return parent; } }
    rb_link_node(node, parent, link); rb_insert_color_cached(node, tree, leftmost); core::ptr::null_mut()
}

#[inline]
pub unsafe fn rb_find_first(key: *const core::ffi::c_void, tree: *const rb_root,
    cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32) -> *mut rb_node {
    let mut node = (*tree).rb_node; let mut found = core::ptr::null_mut();
    while !node.is_null() { let c = cmp(key, node); if c <= 0 { if c == 0 { found = node; } node = (*node).rb_left; } else { node = (*node).rb_right; } }
    found
}

#[inline]
pub unsafe fn rb_next_match(key: *const core::ffi::c_void, mut node: *mut rb_node,
    cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const rb_node) -> i32) -> *mut rb_node {
    node = rb_next(node); if !node.is_null() && cmp(key, node) != 0 { core::ptr::null_mut() } else { node }
}

#[inline]
pub unsafe fn rb_link_noop(_n: *mut rb_node, _p: *mut rb_node, _l: *mut *mut rb_node) {}

#[inline]
pub unsafe fn rb_add(node: *mut rb_node, tree: *mut rb_root,
    less: unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool) {
    let mut link = &mut (*tree).rb_node as *mut *mut rb_node;
    let mut parent = core::ptr::null_mut();
    while !(*link).is_null() { parent = *link; if less(node, parent) { link = &mut (*parent).rb_left; } else { link = &mut (*parent).rb_right; } }
    rb_link_node(node, parent, link); rb_insert_color(node, tree);
}

#[macro_export]
macro_rules! RB_EMPTY_NODE { ($node:expr) => { unsafe { (*$node).__rb_parent_color == ($node as usize) } }; }
#[macro_export]
macro_rules! RB_CLEAR_NODE { ($node:expr) => { unsafe { (*$node).__rb_parent_color = ($node as usize) } }; }
#[macro_export]
macro_rules! RB_EMPTY_ROOT { ($root:expr) => { unsafe { (*$root).rb_node.is_null() } }; }

/* C iteration macros are retained as Rust macro placeholders for callers. */
#[macro_export]
macro_rules! rb_for_each { ($node:ident, $key:expr, $tree:expr, $cmp:expr) => {
    let mut $node = unsafe { rb_find_first($key, $tree, $cmp) };
    while !$node.is_null() { /* caller body */ $node = unsafe { rb_next_match($key, $node, $cmp) }; }
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
