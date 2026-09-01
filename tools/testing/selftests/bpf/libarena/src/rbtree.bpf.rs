// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/*
 * Copyright (c) 2025-2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2025-2026 Emil Tsalapatis <emil@etsalapatis.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;
type u8 = u8;
type u64 = u64;
type uint64_t = u64;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const EALREADY: c_int = 114;
const ENOMEM: c_int = 12;

const RB_MAXLVL_PRINT: usize = 64;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum rbtree_alloc {
    RB_ALLOC = 0,
    RB_NOALLOC = 1,
}

use rbtree_alloc::{RB_ALLOC, RB_NOALLOC};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum rbtree_insert_mode {
    RB_DEFAULT = 0,
    RB_UPDATE = 1,
    RB_DUPLICATE = 2,
}

use rbtree_insert_mode::{RB_DEFAULT, RB_DUPLICATE, RB_UPDATE};

#[repr(C)]
pub struct rbnode {
    pub key: u64,
    pub value: u64,
    pub parent: *mut rbnode,
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub is_red: bool,
}

#[repr(C)]
pub struct rbtree {
    pub alloc: rbtree_alloc,
    pub insert: rbtree_insert_mode,
    pub root: *mut rbnode,
}

unsafe extern "C" {
    static can_loop: bool;

    fn arena_malloc(size: size_t) -> *mut c_void;
    fn arena_free(ptr: *mut c_void);
    fn arena_subprog_init();
    fn arena_stderr(fmt: *const c_char, ...);
    fn barrier();
}

#[inline]
unsafe fn child(node: *mut rbnode, dir: c_int) -> *mut rbnode {
    if dir == 0 {
        unsafe { (*node).left }
    } else {
        unsafe { (*node).right }
    }
}

#[inline]
unsafe fn set_child(node: *mut rbnode, dir: c_int, value: *mut rbnode) {
    if dir == 0 {
        unsafe { (*node).left = value };
    } else {
        unsafe { (*node).right = value };
    }
}

#[inline]
fn unlikely(v: bool) -> bool {
    v
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_create(
    alloc: rbtree_alloc,
    insert: rbtree_insert_mode,
) -> *mut rbtree {
    let rbtree: *mut rbtree;

    rbtree = unsafe { arena_malloc(core::mem::size_of::<rbtree>()) as *mut rbtree };
    if unlikely(rbtree.is_null()) {
        return core::ptr::null_mut();
    }

    /*
     * RB_UPDATE overwrites existing values in the nodes, but RB_NOALLOC
     * trees manage the tree nodes directly (including holding pointers
     * to them). Disallow mixing the two modes to avoid dealing with
     * unintuitive semantics.
     */
    if alloc == RB_NOALLOC && insert == RB_UPDATE {
        unsafe {
            arena_stderr(c"WARNING: Cannot combine RB_NOALLOC and RB_UPDATE".as_ptr());
            arena_free(rbtree as *mut c_void);
        }
        return core::ptr::null_mut();
    }

    unsafe {
        (*rbtree).alloc = alloc;
        (*rbtree).insert = insert;
        (*rbtree).root = core::ptr::null_mut();
    }

    rbtree
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_destroy(rbtree: *mut rbtree) -> c_int {
    let mut ret: c_int = 0;

    unsafe { arena_subprog_init() };

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unsafe { (*rbtree).alloc == RB_NOALLOC } {
        /*
         * We cannot do anything about RB_NOALLOC nodes. The whole
         * point of RB_NOALLOC is that the nodes are directly owned
         * by the caller that allocates and inserts them. We could
         * unilaterally grab all nodes and free them anyway, but that
         * would almost certainly cause UAF as the callers keep accessing
         * the now freed nodes. Throw an error instead.
         */
        if unsafe { !(*rbtree).root.is_null() } {
            unsafe { arena_stderr(c"WARNING: Destroying RB_NOALLOC tree with > 0 nodes".as_ptr()) };
            return -EBUSY;
        }

    } else {
        while unsafe { !(*rbtree).root.is_null() && can_loop } {
            ret = unsafe { rb_remove(rbtree, (*(*rbtree).root).key) };
            if ret != 0 {
                break;
            }
        }
    }

    unsafe { arena_free(rbtree as *mut c_void) };
    ret
}

#[inline]
unsafe fn rbnode_dir(node: *mut rbnode) -> c_int {
    /* Arbitrarily choose a direction for the root. */
    if unlikely(unsafe { (*node).parent.is_null() }) {
        return 0;
    }

    if unsafe { (*(*node).parent).left == node } { 0 } else { 1 }
}

/*
 * The __noinline is to prevent inlining from bloating the add
 * remove calls, in turn causing register splits and increasing
 * stack usage above what is permitted.
 */
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbnode_rotate(
    rbtree: *mut rbtree,
    node: *mut rbnode,
    dir: c_int,
) -> c_int {
    let tmp: *mut rbnode;
    let parent: *mut rbnode;
    let mut parentdir: c_int = 0;

    parent = unsafe { (*node).parent };
    if !parent.is_null() {
        parentdir = unsafe { rbnode_dir(node) };
    }

    /* If we're doing a root change, are we the root? */
    if unlikely(parent.is_null() && unsafe { (*rbtree).root != node }) {
        return -EINVAL;
    }

    /*
     * Does the node we're turning into the root into exist?
     * Note that the new root is on the opposite side of the
     * rotation's direction.
     */
    tmp = unsafe { child(node, 1 - dir) };
    if unlikely(tmp.is_null()) {
        return -EINVAL;
    }

    /* Steal the closest child of the new root. */
    unsafe { set_child(node, 1 - dir, child(tmp, dir)) };
    if unsafe { !child(node, 1 - dir).is_null() } {
        unsafe { (*child(node, 1 - dir)).parent = node };
    }

    /* Put the node below the new root.*/
    unsafe {
        set_child(tmp, dir, node);
        (*node).parent = tmp;

        (*tmp).parent = parent;
        if !parent.is_null() {
            set_child(parent, parentdir, tmp);
        } else {
            (*rbtree).root = tmp;
        }
    }

    0
}

unsafe fn rbnode_find(subtree: *mut rbnode, key: u64) -> *mut rbnode {
    let mut node: *mut rbnode = subtree;
    let mut dir: c_int;

    if subtree.is_null() {
        return core::ptr::null_mut();
    }

    while unsafe { can_loop } {
        if unsafe { (*node).key == key } {
            break;
        }

        dir = if unsafe { key < (*node).key } { 0 } else { 1 };

        if unsafe { child(node, dir).is_null() } {
            break;
        }

        node = unsafe { child(node, dir) };
    }

    node
}

unsafe fn rbnode_least_upper_bound(subtree: *mut rbnode, key: uint64_t) -> *mut rbnode {
    let mut node: *mut rbnode = subtree;
    let mut dir: c_int;

    if subtree.is_null() {
        return core::ptr::null_mut();
    }

    while unsafe { can_loop } {
        dir = if unsafe { key <= (*node).key } { 0 } else { 1 };

        if unsafe { child(node, dir).is_null() } {
            break;
        }

        node = unsafe { child(node, dir) };
    }

    node
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_find(rbtree: *mut rbtree, key: u64, value: *mut u64) -> c_int {
    let node: *mut rbnode;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unlikely(value.is_null()) {
        return -EINVAL;
    }

    node = unsafe { rbnode_find((*rbtree).root, key) };
    if node.is_null() || unsafe { (*node).key != key } {
        return -ENOENT;
    }

    unsafe { *value = (*node).value };

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_node_alloc(key: u64, value: u64) -> *mut rbnode {
    let mut rbnode: *mut rbnode = core::ptr::null_mut();

    rbnode = unsafe { arena_malloc(core::mem::size_of::<rbnode>()) as *mut rbnode };
    if rbnode.is_null() {
        return core::ptr::null_mut();
    }

    /*
     * WARNING: The order of assignments is weird on purpose.
     * See comment in rb_insert_node() for more context.
     * TL;DR: Prevent consecutive 0 assignments from being
     * promoted into an unverifiable memset by the compiler.
     */

    unsafe {
        (*rbnode).key = key;
        (*rbnode).parent = core::ptr::null_mut();
        (*rbnode).value = value;
        (*rbnode).left = core::ptr::null_mut();
        (*rbnode).is_red = true;
        (*rbnode).right = core::ptr::null_mut();
    }

    rbnode
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_node_free(rbnode: *mut rbnode) {
    unsafe { arena_free(rbnode as *mut c_void) };
}

unsafe fn rb_node_insert(rbtree: *mut rbtree, node: *mut rbnode) -> c_int {
    let mut grandparent: *mut rbnode;
    let mut parent: *mut rbnode = unsafe { (*rbtree).root };
    let key: u64 = unsafe { (*node).key };
    let mut uncle: *mut rbnode;
    let mut dir: c_int;
    let ret: c_int;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if parent.is_null() {
        unsafe { (*rbtree).root = node };
        return 0;
    }

    if unsafe { (*rbtree).insert != RB_DUPLICATE } {
        parent = unsafe { rbnode_find(parent, key) };
    } else {
        parent = unsafe { rbnode_least_upper_bound(parent, key) };
    }

    if unsafe { key == (*parent).key && (*rbtree).insert != RB_DUPLICATE } {
        if unsafe { (*rbtree).insert == RB_UPDATE } {
            /*
             * Replace the old node with the new one.
             * Free up the old node.
             */
            ret = unsafe { rbnode_replace(rbtree, parent, node) };
            if ret != 0 {
                return ret;
            }

            if unsafe { (*rbtree).alloc == RB_ALLOC } {
                unsafe { rb_node_free(parent) };
            }

            return 0;
        }

        /* Otherwise it's RB_DEFAULT. */
        return -EALREADY;
    }

    unsafe { (*node).parent = parent };
    /* Also works if key == parent->key. */
    if unsafe { key <= (*parent).key } {
        unsafe { (*parent).left = node };
    } else {
        unsafe { (*parent).right = node };
    }

    while unsafe { can_loop } {
        parent = unsafe { (*node).parent };
        if parent.is_null() {
            return 0;
        }

        if unsafe { !(*parent).is_red } {
            return 0;
        }

        grandparent = unsafe { (*parent).parent };
        if grandparent.is_null() {
            unsafe { (*parent).is_red = false };
            return 0;
        }

        dir = unsafe { rbnode_dir(parent) };
        uncle = unsafe { child(grandparent, 1 - dir) };

        if uncle.is_null() || unsafe { !(*uncle).is_red } {
            if unsafe { node == child(parent, 1 - dir) } {
                unsafe { rbnode_rotate(rbtree, parent, dir) };
                node = parent;
                parent = unsafe { child(grandparent, dir) };
            }

            unsafe { rbnode_rotate(rbtree, grandparent, 1 - dir) };
            unsafe {
                (*parent).is_red = false;
                (*grandparent).is_red = true;
            }

            return 0;
        }

        /* Uncle is red. */

        unsafe {
            (*parent).is_red = false;
            (*uncle).is_red = false;
            (*grandparent).is_red = true;
        }

        node = grandparent;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_insert_node(rbtree: *mut rbtree, node: *mut rbnode) -> c_int {
    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unlikely(unsafe { (*rbtree).alloc == RB_ALLOC }) {
        return -EINVAL;
    }

    unsafe { (*node).left = core::ptr::null_mut() };

    /*
     * Workaround to break an optimization that causes
     * verification failures on some compilers. Assignments
     * of the kind
     *
     * *(r0 + 0) = 0;
     * *(r0 + 8) = 0;
     * *(r0 + 16) = 0;
     *
     * get promoted into a memset, and that in turn is not
     * handled properly for arena memory by LLVM 21 and GCC 15.
     * Add a barrier for now to prevent the assignments from being fused.
     */
    unsafe { barrier() };

    unsafe {
        (*node).parent = core::ptr::null_mut();
        (*node).right = core::ptr::null_mut();

        (*node).is_red = true;
    }

    unsafe { rb_node_insert(rbtree, node) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_insert(rbtree: *mut rbtree, key: u64, value: u64) -> c_int {
    let node: *mut rbnode;
    let ret: c_int;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unlikely(unsafe { (*rbtree).alloc != RB_ALLOC }) {
        return -EINVAL;
    }

    node = unsafe { rb_node_alloc(key, value) };
    if node.is_null() {
        return -ENOMEM;
    }

    ret = unsafe { rb_node_insert(rbtree, node) };
    if ret != 0 {
        unsafe { rb_node_free(node) };
        return ret;
    }

    0
}

#[inline]
unsafe fn rbnode_least(mut subtree: *mut rbnode) -> *mut rbnode {
    while unsafe { !(*subtree).left.is_null() && can_loop } {
        subtree = unsafe { (*subtree).left };
    }

    subtree
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_least(rbtree: *mut rbtree, key: *mut u64, value: *mut u64) -> c_int {
    let least: *mut rbnode;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unsafe { (*rbtree).root.is_null() } {
        return -ENOENT;
    }

    least = unsafe { rbnode_least((*rbtree).root) };
    if !key.is_null() {
        unsafe { *key = (*least).key };
    }
    if !value.is_null() {
        unsafe { *value = (*least).value };
    }

    0
}

/*
 * If we are referencing ourselves, a and b have a parent-child relation,
 * and we should be pointing at the other node instead.
 */
#[inline]
unsafe fn rbnode_fixup_pointers(a: *mut rbnode, b: *mut rbnode) {
    if unsafe { (*a).left == a } {
        unsafe { (*a).left = b };
    }
    if unsafe { (*a).right == a } {
        unsafe { (*a).right = b };
    }
    if unsafe { (*a).parent == a } {
        unsafe { (*a).parent = b };
    }
}

#[inline]
unsafe fn rbnode_swap_values(a: *mut rbnode, b: *mut rbnode) {
    let mut tmpnode: *mut rbnode;
    let mut tmp: u64;

    /* Swap the pointers. */
    tmp = unsafe { (*a).is_red as u64 };
    unsafe { (*a).is_red = (*b).is_red };
    unsafe { (*b).is_red = tmp != 0 };

    tmpnode = unsafe { (*a).left };
    unsafe { (*a).left = (*b).left };
    unsafe { (*b).left = tmpnode };

    tmpnode = unsafe { (*a).right };
    unsafe { (*a).right = (*b).right };
    unsafe { (*b).right = tmpnode };

    tmpnode = unsafe { (*a).parent };
    unsafe { (*a).parent = (*b).parent };
    unsafe { (*b).parent = tmpnode };

    /* Account for the nodes being parent and child. */
    unsafe {
        rbnode_fixup_pointers(b, a);
        rbnode_fixup_pointers(a, b);
    }
}

#[inline]
unsafe fn rbnode_adjust_neighbors(rbtree: *mut rbtree, node: *mut rbnode, dir: c_int) {
    if unsafe { !(*node).left.is_null() } {
        unsafe { (*(*node).left).parent = node };
    }
    if unsafe { !(*node).right.is_null() } {
        unsafe { (*(*node).right).parent = node };
    }

    if unsafe { !(*node).parent.is_null() } {
        unsafe { set_child((*node).parent, dir, node) };
        return;
    }

    unsafe { (*rbtree).root = node };
}

/*
 * Directly replace an existing node with a replacement. The replacement node
 * should not already be in the tree.
 */
unsafe fn rbnode_replace(
    rbtree: *mut rbtree,
    existing: *mut rbnode,
    replacement: *mut rbnode,
) -> c_int {
    let mut dir: c_int = 0;

    if unlikely(unsafe {
        !(*replacement).parent.is_null()
            || !(*replacement).left.is_null()
            || !(*replacement).right.is_null()
    }) {
        return -EINVAL;
    }

    if unsafe { !(*existing).parent.is_null() } {
        dir = unsafe { rbnode_dir(existing) };
    }

    unsafe {
        (*replacement).is_red = (*existing).is_red;
        (*replacement).left = (*existing).left;
        (*replacement).right = (*existing).right;
        (*replacement).parent = (*existing).parent;
    }

    /* Fix up the new node's neighbors. */
    unsafe { rbnode_adjust_neighbors(rbtree, replacement, dir) };

    0
}

/*
 * Switch two nodes in the tree in place. This is useful during node deletion.
 * This is more involved than switching the values of the two nodes because we
 * must update all tree pointers.
 */
unsafe fn rbnode_switch(rbtree: *mut rbtree, a: *mut rbnode, b: *mut rbnode) {
    let mut adir: c_int = 0;
    let mut bdir: c_int = 0;

    /*
     * Store the direction in the parent because we will not
     * be able to recompute it once we start swapping values.
     */
    if unsafe { !(*a).parent.is_null() } {
        adir = unsafe { rbnode_dir(a) };
    }

    if unsafe { !(*b).parent.is_null() } {
        bdir = unsafe { rbnode_dir(b) };
    }

    unsafe { rbnode_swap_values(a, b) };

    /*
     * Fix up the pointers from the children/parent to the
     * new nodes.
     */
    unsafe {
        rbnode_adjust_neighbors(rbtree, a, bdir);
        rbnode_adjust_neighbors(rbtree, b, adir);
    }
}

#[inline]
unsafe fn rbnode_remove_node_single_child(
    rbtree: *mut rbtree,
    node: *mut rbnode,
    free: bool,
) -> c_int {
    let child_node: *mut rbnode;
    let dir: c_int;

    if unlikely(unsafe { (*node).is_red }) {
        unsafe { arena_stderr(c"Node unexpectedly red\n".as_ptr()) };
        return -EINVAL;
    }

    child_node = if unsafe { !(*node).left.is_null() } {
        unsafe { (*node).left }
    } else {
        unsafe { (*node).right }
    };
    if unlikely(unsafe { !(*child_node).is_red }) {
        unsafe { arena_stderr(c"Only child is black\n".as_ptr()) };
        return -EINVAL;
    }

    /*
     * Since it's the immediate child, we can just
     * remove the parent.
     */
    unsafe { (*child_node).parent = (*node).parent };

    if unsafe { !(*node).parent.is_null() } {
        dir = unsafe { rbnode_dir(node) };
        unsafe { set_child((*node).parent, dir, child_node) };
    } else {
        unsafe { (*rbtree).root = child_node };
    }

    /* Color the child black. */
    unsafe { (*child_node).is_red = false };

    /* Only free if called from rb_remove. */
    if free {
        unsafe { rb_node_free(node) };
    }

    0
}

#[inline]
unsafe fn rbnode_has_red_children(node: *mut rbnode) -> bool {
    if unsafe { !(*node).left.is_null() && (*(*node).left).is_red } {
        return true;
    }

    unsafe { !(*node).right.is_null() && (*(*node).right).is_red }
}

unsafe fn rb_node_remove(rbtree: *mut rbtree, mut node: *mut rbnode) -> c_int {
    let mut parent: *mut rbnode;
    let mut sibling: *mut rbnode;
    let mut close_nephew: *mut rbnode;
    let mut distant_nephew: *mut rbnode;
    let free: bool = unsafe { (*rbtree).alloc == RB_ALLOC };
    let replace: *mut rbnode;
    let initial: *mut rbnode;
    let is_red: bool;
    let mut dir: c_int;

    /* Both children present, replace with next largest key. */
    if unsafe { !(*node).left.is_null() && !(*node).right.is_null() } {
        /*
         * Swap the node itself instead of just the
         * key/value pair to account for nodes embedded
         * in other structs.
         */

        replace = unsafe { rbnode_least((*node).right) };
        unsafe { rbnode_switch(rbtree, replace, node) };

        /*
         * FALLTHROUGH: We moved the node we are removing to
         * the leftmost position of the subtree. We can now
         * remove it as if it was always where we moved it to.
         */
    }

    initial = node;

    /* Only one child present, replace with child and paint it black. */
    if unsafe { (*node).left.is_null() != (*node).right.is_null() } {
        return unsafe { rbnode_remove_node_single_child(rbtree, node, free) };
    }

    /* (!node->left && !node->right) */

    parent = unsafe { (*node).parent };
    if parent.is_null() {
        /* Check that we're _actually_ the root. */
        if unsafe { (*rbtree).root == node } {
            unsafe { (*rbtree).root = core::ptr::null_mut() };
        } else {
            unsafe {
                arena_stderr(
                    c"WARNING: Attempting to remove detached node from rbtree\n".as_ptr(),
                )
            };
        }

        if free {
            unsafe { rb_node_free(node) };
        }
        return 0;
    }

    dir = unsafe { rbnode_dir(node) };
    unsafe { set_child(parent, dir, core::ptr::null_mut()) };
    is_red = unsafe { (*node).is_red };

    if free {
        unsafe { rb_node_free(node) };
    }

    /* If we removed a red node, we did not unbalance the tree.*/
    if is_red {
        return 0;
    }

    sibling = unsafe { child(parent, 1 - dir) };
    if unlikely(sibling.is_null()) {
        unsafe { arena_stderr(c"rbtree: removed black node has no sibling\n".as_ptr()) };
        return -EINVAL;
    }

    /*
     * We removed a black node, causing a change in path
     * weight. Start rebalancing. The invariant is that
     * all paths going through the node are shortened
     * by one, and the current node is black.
     */
    while unsafe { can_loop } {
        /* Balancing reached the root, there can be no imbalance. */
        if parent.is_null() {
            return 0;
        }

        /*
         * We already determined the dir, either above or
         * at the end of the loop.
         */

        /*
         * If we have no sibling, the tree was
         * already unbalanced.
         */
        sibling = unsafe { child(parent, 1 - dir) };
        if unlikely(sibling.is_null()) {
            unsafe { arena_stderr(c"rbtree: removed black node has no sibling\n".as_ptr()) };
            return -EINVAL;
        }

        /* Sibling is red, turn it into the grandparent. */
        if unsafe { (*sibling).is_red } {
            /*
             * Sibling is red. Transform the tree to turn
             * the sibling into the parent's position, and
             * repaint them. This does not balance the tree
             * but makes it so we know the sibling is black
             * and so can use the transformations to balance.
             */
            unsafe { rbnode_rotate(rbtree, parent, dir) };
            unsafe {
                (*parent).is_red = true;
                (*sibling).is_red = false;
            }

            /* Our new sibling is now the close nephew. */
            sibling = unsafe { child(parent, 1 - dir) };
            /* If sibling has any red siblings, break out. */
            if unsafe { rbnode_has_red_children(sibling) } {
                break;
            }

            /* We can repaint the sibling and parent, we're done. */
            unsafe {
                (*sibling).is_red = true;
                (*parent).is_red = false;
            }

            return 0;
        }

        /* Sibling guaranteed to be black. If it has red children, break out. */
        if unsafe { rbnode_has_red_children(sibling) } {
            break;
        }

        /*
         * Both sibling and children are black. If parent is red, swap
         * colors with the sibling. Otherwise
         */
        if unsafe { (*parent).is_red } {
            unsafe {
                (*parent).is_red = false;
                (*sibling).is_red = true;
            }
            return 0;
        }

        /*
         * Parent, sibling, and all its children are black. Repaint the sibling.
         * This shortens the paths through it, so pop up a level in the
         * tree and repeat the balancing.
         */
        unsafe { (*sibling).is_red = true };
        node = parent;
        parent = unsafe { (*node).parent };
        dir = unsafe { rbnode_dir(node) };
    }

    if node != initial {
        dir = unsafe { rbnode_dir(node) };
        parent = unsafe { (*node).parent };
        sibling = unsafe { child(parent, 1 - dir) };
    }
    /*
     * Almost there. We know between the parent, sibling,
     * and nephews only one or two of the nephews are red. If
     * it is the close one, rotate it to the sibling position,
     * paint it black, and paint the previous sibling red.
     */

    close_nephew = unsafe { child(sibling, dir) };
    distant_nephew = unsafe { child(sibling, 1 - dir) };

    /*
     * If the distant red nephew is not red, rotate
     * and repaint. We need the distant nephew
     * to be red. We know the close nephew is red
     * because at least one of them are, so the
     * distant one is black if it exists.
     */
    if distant_nephew.is_null() || unsafe { !(*distant_nephew).is_red } {
        unsafe { rbnode_rotate(rbtree, sibling, 1 - dir) };
        unsafe {
            (*sibling).is_red = true;
            (*close_nephew).is_red = false;
        }
        distant_nephew = sibling;
        sibling = close_nephew;
    }

    /*
     * We now know it's the distant nephew that's red.
     * Rotate the sibling into our parent's position
     * and paint both black.
     */

    unsafe { rbnode_rotate(rbtree, parent, dir) };
    unsafe {
        (*sibling).is_red = (*parent).is_red;
        (*parent).is_red = false;
        (*distant_nephew).is_red = false;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_remove_node(rbtree: *mut rbtree, node: *mut rbnode) -> c_int {
    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unlikely(unsafe { (*rbtree).alloc == RB_ALLOC }) {
        return -EINVAL;
    }

    unsafe { rb_node_remove(rbtree, node) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_remove(rbtree: *mut rbtree, key: u64) -> c_int {
    let node: *mut rbnode;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unlikely(unsafe { (*rbtree).alloc != RB_ALLOC }) {
        return -EINVAL;
    }

    if unsafe { (*rbtree).root.is_null() } {
        return -ENOENT;
    }

    node = unsafe { rbnode_find((*rbtree).root, key) };
    if node.is_null() || unsafe { (*node).key != key } {
        return -ENOENT;
    }

    unsafe { rb_node_remove(rbtree, node) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_pop(rbtree: *mut rbtree, key: *mut u64, value: *mut u64) -> c_int {
    let node: *mut rbnode;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unsafe { (*rbtree).root.is_null() } {
        return -ENOENT;
    }

    if unsafe { (*rbtree).alloc != RB_ALLOC } {
        return -EINVAL;
    }

    node = unsafe { rbnode_least((*rbtree).root) };
    if unlikely(node.is_null()) {
        return -ENOENT;
    }

    if !key.is_null() {
        unsafe { *key = (*node).key };
    }
    if !value.is_null() {
        unsafe { *value = (*node).value };
    }

    unsafe { rb_node_remove(rbtree, node) }
}

#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbnode_print(depth: size_t, rbn: *mut rbnode) {
    unsafe {
        arena_stderr(
            c"[DEPTH %d] %p (%s)\n PARENT %p".as_ptr(),
            depth,
            rbn,
            if (*rbn).is_red {
                c"red".as_ptr()
            } else {
                c"black".as_ptr()
            },
            (*rbn).parent,
        );
        arena_stderr(
            c"\tKV (%ld, %ld)\n LEFT %p RIGHT %p]\n".as_ptr(),
            (*rbn).key,
            (*rbn).value,
            (*rbn).left,
            (*rbn).right,
        );
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum rb_print_state {
    RB_NONE_VISITED,
    RB_LEFT_VISITED,
    RB_RIGHT_VISITED,
}

use rb_print_state::{RB_LEFT_VISITED, RB_NONE_VISITED, RB_RIGHT_VISITED};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_print_next_state(
    rbnode: *mut rbnode,
    mut state: rb_print_state,
    next: *mut u64,
) -> rb_print_state {
    if unlikely(next.is_null()) {
        return RB_NONE_VISITED;
    }

    match state {
        RB_NONE_VISITED => {
            if unsafe { !(*rbnode).left.is_null() } {
                unsafe { *next = (*rbnode).left as u64 };
                state = RB_LEFT_VISITED;
            } else if unsafe { !(*rbnode).right.is_null() } {
                unsafe { *next = (*rbnode).right as u64 };
                state = RB_RIGHT_VISITED;
            } else {
                unsafe { *next = 0 };
                state = RB_RIGHT_VISITED;
            }
        }
        RB_LEFT_VISITED => {
            if unsafe { !(*rbnode).right.is_null() } {
                unsafe { *next = (*rbnode).right as u64 };
                state = RB_RIGHT_VISITED;
            } else {
                unsafe { *next = 0 };
                state = RB_RIGHT_VISITED;
            }
        }
        _ => {
            unsafe { *next = 0 };
            state = RB_RIGHT_VISITED;
        }
    }

    state
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_print_pop_up(
    rbnodep: *mut *mut rbnode,
    depthp: *mut u8,
    stack: *mut [rb_print_state; RB_MAXLVL_PRINT],
    state: *mut rb_print_state,
) -> c_int {
    let mut rbnode: *mut rbnode;
    let mut depth: u8;
    let mut j: c_int;

    if unlikely(rbnodep.is_null() || depthp.is_null() || stack.is_null() || state.is_null()) {
        return -EINVAL;
    }

    rbnode = unsafe { *rbnodep };
    depth = unsafe { core::ptr::read_volatile(depthp) };

    j = 0;
    while j < RB_MAXLVL_PRINT as c_int && unsafe { can_loop } {
        if unsafe { *state != RB_RIGHT_VISITED } {
            break;
        }

        depth = depth.wrapping_sub(1);
        if (depth as c_int) < 0 || depth as usize >= RB_MAXLVL_PRINT {
            break;
        }

        unsafe {
            *state = (*stack)[depth as usize % RB_MAXLVL_PRINT];
            rbnode = (*rbnode).parent;
        }
        j += 1;
    }

    unsafe {
        *rbnodep = rbnode;
        *depthp = depth;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_print(rbtree: *mut rbtree) -> c_int {
    let mut stack: [rb_print_state; RB_MAXLVL_PRINT] = [RB_NONE_VISITED; RB_MAXLVL_PRINT];
    let mut rbnode: *mut rbnode = unsafe { (*rbtree).root };
    let mut state: rb_print_state;
    let mut next: *mut rbnode;
    let mut next_addr: u64 = 0;
    let mut depth: u8;
    let mut ret: c_int;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    depth = 0;
    state = RB_NONE_VISITED;

    unsafe { arena_stderr(c"=== RB TREE START ===\n".as_ptr()) };

    if unsafe { (*rbtree).root.is_null() } {
        unsafe { arena_stderr(c"=== RB TREE END ===\n".as_ptr()) };
        return 0;
    }

    /* Even with can_loop, the verifier doesn't like infinite loops. */
    while unsafe { can_loop } {
        if state == RB_NONE_VISITED {
            unsafe { rbnode_print(depth as size_t, rbnode) };
        }

        /* Find which child to traverse next. */
        state = unsafe { rb_print_next_state(rbnode, state, &mut next_addr) };
        next = next_addr as *mut rbnode;

        /* Child found. Store the node state and go on. */
        if !next.is_null() {
            if (depth as c_int) < 0 || depth as usize >= RB_MAXLVL_PRINT {
                return 0;
            }

            stack[depth as usize] = state;
            depth = depth.wrapping_add(1);

            rbnode = next;
            state = RB_NONE_VISITED;

            continue;
        }

        /* Otherwise, go as far up as possible. */
        ret = unsafe { rb_print_pop_up(&mut rbnode, &mut depth, &mut stack, &mut state) };
        if ret != 0 {
            return -EINVAL;
        }

        if (depth as c_int) < 0 || depth as usize >= RB_MAXLVL_PRINT {
            unsafe { arena_stderr(c"=== RB TREE END (depth %d\n)===".as_ptr(), depth) };
            return 0;
        }
    }

    unsafe { arena_stderr(c"=== RB TREE END ===\n".as_ptr()) };

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rb_integrity_check(rbtree: *mut rbtree) -> c_int {
    let mut stack: [rb_print_state; RB_MAXLVL_PRINT] = [RB_NONE_VISITED; RB_MAXLVL_PRINT];
    let mut rbnode: *mut rbnode = unsafe { (*rbtree).root };
    let mut state: rb_print_state;
    let mut next: *mut rbnode;
    let mut next_addr: u64 = 0;
    let mut depth: u8;
    let mut ret: c_int;

    if unlikely(rbtree.is_null()) {
        return -EINVAL;
    }

    if unsafe { (*rbtree).root.is_null() } {
        return 0;
    }

    depth = 0;
    state = RB_NONE_VISITED;

    /* Even with can_loop, the verifier doesn't like infinite loops. */
    while unsafe { can_loop } {
        if unsafe {
            !(*rbnode).parent.is_null()
                && (*(*rbnode).parent).left != rbnode
                && (*(*rbnode).parent).right != rbnode
        } {
            unsafe {
                arena_stderr(
                    c"WARNING: Inconsistent tree. Parent %p has no child %p\n".as_ptr(),
                    (*rbnode).parent,
                    rbnode,
                )
            };
            return -EINVAL;
        }

        if unsafe { (*rbnode).parent == rbnode } {
            unsafe {
                arena_stderr(
                    c"WARNING: Inconsistent tree, node %p is its own parent\n".as_ptr(),
                    rbnode,
                )
            };
            return -EINVAL;
        }

        if unsafe { (*rbnode).left == rbnode } {
            unsafe {
                arena_stderr(
                    c"WARNING: Inconsistent tree, node %p is its own left child\n".as_ptr(),
                    rbnode,
                )
            };
            return -EINVAL;
        }

        if unsafe { (*rbnode).right == rbnode } {
            unsafe {
                arena_stderr(
                    c"WARNING: Inconsistent tree, node %p is its own right child\n".as_ptr(),
                    rbnode,
                )
            };
            return -EINVAL;
        }

        if unsafe { (*rbnode).is_red } {
            if unsafe { !(*rbnode).left.is_null() && (*(*rbnode).left).is_red } {
                unsafe {
                    arena_stderr(
                        c"WARNING: Inconsistent tree. Parent has %p has red child %p\n".as_ptr(),
                        rbnode,
                        (*rbnode).left,
                    )
                };
                return -EINVAL;
            }
            if unsafe { !(*rbnode).right.is_null() && (*(*rbnode).right).is_red } {
                unsafe {
                    arena_stderr(
                        c"WARNING: Inconsistent tree. Parent has %p has red child %p\n".as_ptr(),
                        rbnode,
                        (*rbnode).right,
                    )
                };
                return -EINVAL;
            }
        } else if unsafe {
            !(*rbnode).parent.is_null()
                && child((*rbnode).parent, 1 - rbnode_dir(rbnode)).is_null()
        } {
            unsafe {
                arena_stderr(
                    c"WARNING: Inconsistent tree. Black node %p has no sibling\n".as_ptr(),
                    rbnode,
                )
            };
            return -EINVAL;
        }

        /* Find which child to traverse next. */
        state = unsafe { rb_print_next_state(rbnode, state, &mut next_addr) };
        next = next_addr as *mut rbnode;

        /* Child found. Store the node state and go on. */
        if !next.is_null() {
            if (depth as c_int) < 0 || depth as usize >= RB_MAXLVL_PRINT {
                return 0;
            }

            stack[depth as usize] = state;
            depth = depth.wrapping_add(1);

            rbnode = next;
            state = RB_NONE_VISITED;

            continue;
        }

        /* Otherwise, go as far up as possible. */
        ret = unsafe { rb_print_pop_up(&mut rbnode, &mut depth, &mut stack, &mut state) };
        if ret != 0 {
            return -EINVAL;
        }

        if (depth as c_int) < 0 || depth as usize >= RB_MAXLVL_PRINT {
            return 0;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
