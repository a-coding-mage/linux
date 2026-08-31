/* SPDX-License-Identifier: GPL-2.0-or-later */
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

/* C header guard and includes omitted. Original dependencies:
 * <linux/kernel.h> for container_of, READ_ONCE, bool, and __always_inline.
 * <linux/stddef.h> for NULL and offsetof-style support.
 */

use core::ffi::c_void;
use core::ptr;

#[repr(C, align(8))]
pub struct rb_node {
    pub __rb_parent_color: libc::c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}
/* The alignment might seem pointless, but allegedly CRIS needs it */

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[inline]
pub unsafe fn rb_parent(r: *const rb_node) -> *mut rb_node {
    unsafe { ((*r).__rb_parent_color & !3) as *mut rb_node }
}

pub const RB_ROOT: rb_root = rb_root {
    rb_node: ptr::null_mut(),
};

/* rb_entry(ptr, type, member) maps to container_of(ptr, type, member).
 * It requires the external container_of facility and cannot be represented as a
 * file-local Rust function over arbitrary field names.
 */

#[inline]
pub unsafe fn RB_EMPTY_ROOT(root: *const rb_root) -> bool {
    unsafe { core::ptr::read_volatile(&(*root).rb_node).is_null() }
}

/* 'empty' nodes are nodes that are known not to be inserted in an rbtree */
#[inline]
pub unsafe fn RB_EMPTY_NODE(node: *const rb_node) -> bool {
    unsafe { (*node).__rb_parent_color == node as libc::c_ulong }
}

#[inline]
pub unsafe fn RB_CLEAR_NODE(node: *mut rb_node) {
    unsafe {
        (*node).__rb_parent_color = node as libc::c_ulong;
    }
}

unsafe extern "C" {
    pub fn rb_insert_color(arg1: *mut rb_node, arg2: *mut rb_root);
    pub fn rb_erase(arg1: *mut rb_node, arg2: *mut rb_root);

    /* Find logical next and previous nodes in a tree */
    pub fn rb_next(arg1: *const rb_node) -> *mut rb_node;
    pub fn rb_prev(arg1: *const rb_node) -> *mut rb_node;
    pub fn rb_first(arg1: *const rb_root) -> *mut rb_node;
    pub fn rb_last(arg1: *const rb_root) -> *mut rb_node;

    /* Postorder iteration - always visit the parent after its children */
    pub fn rb_first_postorder(arg1: *const rb_root) -> *mut rb_node;
    pub fn rb_next_postorder(arg1: *const rb_node) -> *mut rb_node;

    /* Fast replacement of a single node without remove/rebalance/add/rebalance */
    pub fn rb_replace_node(victim: *mut rb_node, new: *mut rb_node, root: *mut rb_root);
}

#[inline]
pub unsafe fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node) {
    unsafe {
        (*node).__rb_parent_color = parent as libc::c_ulong;
        (*node).rb_right = ptr::null_mut();
        (*node).rb_left = (*node).rb_right;

        *rb_link = node;
    }
}

/* rb_entry_safe(ptr, type, member) evaluates ptr once and returns
 * rb_entry(ptr, type, member) or NULL. It depends on rb_entry/container_of and
 * C typeof/member-name macro semantics.
 */

/**
 * rbtree_postorder_for_each_entry_safe - iterate in post-order over rb_root of
 * given type allowing the backing memory of @pos to be invalidated
 *
 * @pos:	the 'type *' to use as a loop cursor.
 * @n:		another 'type *' to use as temporary storage
 * @root:	'rb_root *' of the rbtree.
 * @field:	the name of the rb_node field within 'type'.
 *
 * rbtree_postorder_for_each_entry_safe() provides a similar guarantee as
 * list_for_each_entry_safe() and allows the iteration to continue independent
 * of changes to @pos by the body of the loop.
 *
 * Note, however, that it cannot handle other modifications that re-order the
 * rbtree it is iterating over. This includes calling rb_erase() on @pos, as
 * rb_erase() may rebalance the tree, causing us to miss some nodes.
 */
/* rbtree_postorder_for_each_entry_safe(pos, n, root, field) is a C for-loop
 * macro over arbitrary containing types and field names. It relies on typeof,
 * statement expressions, and rb_entry_safe/container_of.
 */

#[inline]
pub unsafe fn rb_erase_init(n: *mut rb_node, root: *mut rb_root) {
    unsafe {
        rb_erase(n, root);
        RB_CLEAR_NODE(n);
    }
}

/*
 * Leftmost-cached rbtrees.
 *
 * We do not cache the rightmost node based on footprint
 * size vs number of potential users that could benefit
 * from O(1) rb_last(). Just not worth it, users that want
 * this feature can always implement the logic explicitly.
 * Furthermore, users that want to cache both pointers may
 * find it a bit asymmetric, but that's ok.
 */
#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

pub const RB_ROOT_CACHED: rb_root_cached = rb_root_cached {
    rb_root: rb_root {
        rb_node: ptr::null_mut(),
    },
    rb_leftmost: ptr::null_mut(),
};

/* Same as rb_first(), but O(1) */
#[inline]
pub unsafe fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node {
    unsafe { (*root).rb_leftmost }
}

#[inline]
pub unsafe fn rb_insert_color_cached(
    node: *mut rb_node,
    root: *mut rb_root_cached,
    leftmost: bool,
) {
    unsafe {
        if leftmost {
            (*root).rb_leftmost = node;
        }
        rb_insert_color(node, &mut (*root).rb_root);
    }
}

#[inline]
pub unsafe fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached) {
    unsafe {
        if (*root).rb_leftmost == node {
            (*root).rb_leftmost = rb_next(node);
        }
        rb_erase(node, &mut (*root).rb_root);
    }
}

#[inline]
pub unsafe fn rb_replace_node_cached(
    victim: *mut rb_node,
    new: *mut rb_node,
    root: *mut rb_root_cached,
) {
    unsafe {
        if (*root).rb_leftmost == victim {
            (*root).rb_leftmost = new;
        }
        rb_replace_node(victim, new, &mut (*root).rb_root);
    }
}

/*
 * The below helper functions use 2 operators with 3 different
 * calling conventions. The operators are related like:
 *
 *	comp(a->key,b) < 0  := less(a,b)
 *	comp(a->key,b) > 0  := less(b,a)
 *	comp(a->key,b) == 0 := !less(a,b) && !less(b,a)
 *
 * If these operators define a partial order on the elements we make no
 * guarantee on which of the elements matching the key is found. See
 * rb_find().
 *
 * The reason for this is to allow the find() interface without requiring an
 * on-stack dummy object, which might not be feasible due to object size.
 */

/**
 * rb_add_cached() - insert @node into the leftmost cached tree @tree
 * @node: node to insert
 * @tree: leftmost cached tree to insert @node into
 * @less: operator defining the (partial) node order
 */
#[inline]
pub unsafe fn rb_add_cached(
    node: *mut rb_node,
    tree: *mut rb_root_cached,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
) {
    unsafe {
        let mut link: *mut *mut rb_node = &mut (*tree).rb_root.rb_node;
        let mut parent: *mut rb_node = ptr::null_mut();
        let mut leftmost: bool = true;

        while !(*link).is_null() {
            parent = *link;
            if less.expect("non-null function pointer")(node, parent) {
                link = &mut (*parent).rb_left;
            } else {
                link = &mut (*parent).rb_right;
                leftmost = false;
            }
        }

        rb_link_node(node, parent, link);
        rb_insert_color_cached(node, tree, leftmost);
    }
}

/**
 * rb_add() - insert @node into @tree
 * @node: node to insert
 * @tree: tree to insert @node into
 * @less: operator defining the (partial) node order
 */
#[inline]
pub unsafe fn rb_add(
    node: *mut rb_node,
    tree: *mut rb_root,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
) {
    unsafe {
        let mut link: *mut *mut rb_node = &mut (*tree).rb_node;
        let mut parent: *mut rb_node = ptr::null_mut();

        while !(*link).is_null() {
            parent = *link;
            if less.expect("non-null function pointer")(node, parent) {
                link = &mut (*parent).rb_left;
            } else {
                link = &mut (*parent).rb_right;
            }
        }

        rb_link_node(node, parent, link);
        rb_insert_color(node, tree);
    }
}

/**
 * rb_find_add() - find equivalent @node in @tree, or add @node
 * @node: node to look-for / insert
 * @tree: tree to search / modify
 * @cmp: operator defining the node order
 *
 * Returns the rb_node matching @node, or NULL when no match is found and @node
 * is inserted.
 */
#[inline]
pub unsafe fn rb_find_add(
    node: *mut rb_node,
    tree: *mut rb_root,
    cmp: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> libc::c_int>,
) -> *mut rb_node {
    unsafe {
        let mut link: *mut *mut rb_node = &mut (*tree).rb_node;
        let mut parent: *mut rb_node = ptr::null_mut();
        let mut c: libc::c_int;

        while !(*link).is_null() {
            parent = *link;
            c = cmp.expect("non-null function pointer")(node, parent);

            if c < 0 {
                link = &mut (*parent).rb_left;
            } else if c > 0 {
                link = &mut (*parent).rb_right;
            } else {
                return parent;
            }
        }

        rb_link_node(node, parent, link);
        rb_insert_color(node, tree);
        ptr::null_mut()
    }
}

/**
 * rb_find() - find @key in tree @tree
 * @key: key to match
 * @tree: tree to search
 * @cmp: operator defining the node order
 *
 * Returns the rb_node matching @key or NULL.
 */
#[inline]
pub unsafe fn rb_find(
    key: *const c_void,
    tree: *const rb_root,
    cmp: Option<unsafe extern "C" fn(*const c_void, *const rb_node) -> libc::c_int>,
) -> *mut rb_node {
    unsafe {
        let mut node: *mut rb_node = (*tree).rb_node;

        while !node.is_null() {
            let c: libc::c_int = cmp.expect("non-null function pointer")(key, node);

            if c < 0 {
                node = (*node).rb_left;
            } else if c > 0 {
                node = (*node).rb_right;
            } else {
                return node;
            }
        }

        ptr::null_mut()
    }
}

/**
 * rb_find_first() - find the first @key in @tree
 * @key: key to match
 * @tree: tree to search
 * @cmp: operator defining node order
 *
 * Returns the leftmost node matching @key, or NULL.
 */
#[inline]
pub unsafe fn rb_find_first(
    key: *const c_void,
    tree: *const rb_root,
    cmp: Option<unsafe extern "C" fn(*const c_void, *const rb_node) -> libc::c_int>,
) -> *mut rb_node {
    unsafe {
        let mut node: *mut rb_node = (*tree).rb_node;
        let mut match_: *mut rb_node = ptr::null_mut();

        while !node.is_null() {
            let c: libc::c_int = cmp.expect("non-null function pointer")(key, node);

            if c <= 0 {
                if c == 0 {
                    match_ = node;
                }
                node = (*node).rb_left;
            } else if c > 0 {
                node = (*node).rb_right;
            }
        }

        match_
    }
}

/**
 * rb_next_match() - find the next @key in @tree
 * @key: key to match
 * @tree: tree to search
 * @cmp: operator defining node order
 *
 * Returns the next node matching @key, or NULL.
 */
#[inline]
pub unsafe fn rb_next_match(
    key: *const c_void,
    mut node: *mut rb_node,
    cmp: Option<unsafe extern "C" fn(*const c_void, *const rb_node) -> libc::c_int>,
) -> *mut rb_node {
    unsafe {
        node = rb_next(node);
        if !node.is_null() && cmp.expect("non-null function pointer")(key, node) != 0 {
            node = ptr::null_mut();
        }
        node
    }
}

/**
 * rb_for_each() - iterates a subtree matching @key
 * @node: iterator
 * @key: key to match
 * @tree: tree to search
 * @cmp: operator defining node order
 */
/* rb_for_each(node, key, tree, cmp) is a C for-loop macro:
 * for ((node) = rb_find_first((key), (tree), (cmp));
 *      (node); (node) = rb_next_match((key), (node), (cmp)))
 */
