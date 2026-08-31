// SPDX-License-Identifier: GPL-2.0-or-later
/*
  Red Black Trees
  (C) 1999  Andrea Arcangeli <andrea@suse.de>
  (C) 2002  David Woodhouse <dwmw2@infradead.org>
  (C) 2012  Michel Lespinasse <walken@google.com>


  linux/lib/rbtree.c
*/

// C includes translated as external dependencies:
// #include <linux/rbtree_augmented.h>
// #include <linux/export.h>

use core::ptr;

pub const RB_RED: i32 = 0;
pub const RB_BLACK: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_node {
    pub __rb_parent_color: usize,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_augment_callbacks {
    pub propagate: Option<unsafe extern "C" fn(node: *mut rb_node, stop: *mut rb_node)>,
    pub copy: Option<unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node)>,
    pub rotate: Option<unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node)>,
}

extern "C" {
    fn __rb_erase_augmented(
        node: *mut rb_node,
        root: *mut rb_root,
        augment: *const rb_augment_callbacks,
    ) -> *mut rb_node;
}

#[inline]
unsafe fn WRITE_ONCE<T: Copy>(dst: *mut T, val: T) {
    ptr::write_volatile(dst, val);
}

#[inline]
unsafe fn rb_parent(rb: *const rb_node) -> *mut rb_node {
    ((*rb).__rb_parent_color & !3usize) as *mut rb_node
}

#[inline]
unsafe fn rb_set_parent(rb: *mut rb_node, p: *mut rb_node) {
    (*rb).__rb_parent_color = ((*rb).__rb_parent_color & 3usize) | p as usize;
}

#[inline]
unsafe fn rb_set_parent_color(rb: *mut rb_node, p: *mut rb_node, color: i32) {
    (*rb).__rb_parent_color = p as usize | color as usize;
}

#[inline]
unsafe fn rb_is_red(rb: *const rb_node) -> bool {
    ((*rb).__rb_parent_color & RB_BLACK as usize) == RB_RED as usize
}

#[inline]
unsafe fn rb_is_black(rb: *const rb_node) -> bool {
    ((*rb).__rb_parent_color & RB_BLACK as usize) == RB_BLACK as usize
}

#[inline]
unsafe fn RB_EMPTY_NODE(node: *const rb_node) -> bool {
    (*node).__rb_parent_color == node as usize
}

#[inline]
unsafe fn __rb_change_child(
    old: *mut rb_node,
    new: *mut rb_node,
    parent: *mut rb_node,
    root: *mut rb_root,
) {
    if !parent.is_null() {
        if (*parent).rb_left == old {
            WRITE_ONCE(&mut (*parent).rb_left, new);
        } else {
            WRITE_ONCE(&mut (*parent).rb_right, new);
        }
    } else {
        WRITE_ONCE(&mut (*root).rb_node, new);
    }
}

/*
 * red-black trees properties:  https://en.wikipedia.org/wiki/Rbtree
 *
 *  1) A node is either red or black
 *  2) The root is black
 *  3) All leaves (NULL) are black
 *  4) Both children of every red node are black
 *  5) Every simple path from root to leaves contains the same number
 *     of black nodes.
 *
 *  4 and 5 give the O(log n) guarantee, since 4 implies you cannot have two
 *  consecutive red nodes in a path and every red node is therefore followed by
 *  a black. So if B is the number of black nodes on every simple path (as per
 *  5), then the longest possible path due to 4 is 2B.
 *
 *  We shall indicate color with case, where black nodes are uppercase and red
 *  nodes will be lowercase. Unknown color nodes shall be drawn as red within
 *  parentheses and have some accompanying text comment.
 */

/*
 * Notes on lockless lookups:
 *
 * All stores to the tree structure (rb_left and rb_right) must be done using
 * WRITE_ONCE(). And we must not inadvertently cause (temporary) loops in the
 * tree structure as seen in program order.
 *
 * These two requirements will allow lockless iteration of the tree -- not
 * correct iteration mind you, tree rotations are not atomic so a lookup might
 * miss entire subtrees.
 *
 * But they do guarantee that any such traversal will only see valid elements
 * and that it will indeed complete -- does not get stuck in a loop.
 *
 * It also guarantees that if the lookup returns an element it is the 'correct'
 * one. But not returning an element does _NOT_ mean it's not present.
 *
 * NOTE:
 *
 * Stores to __rb_parent_color are not important for simple lookups so those
 * are left undone as of now. Nor did I check for loops involving parent
 * pointers.
 */

#[inline]
unsafe fn rb_set_black(rb: *mut rb_node) {
    (*rb).__rb_parent_color = (*rb).__rb_parent_color.wrapping_add(RB_BLACK as usize);
}

#[inline]
unsafe fn rb_red_parent(red: *mut rb_node) -> *mut rb_node {
    (*red).__rb_parent_color as *mut rb_node
}

/*
 * Helper function for rotations:
 * - old's parent and color get assigned to new
 * - old gets assigned new as a parent and 'color' as a color.
 */
#[inline]
unsafe fn __rb_rotate_set_parents(
    old: *mut rb_node,
    new: *mut rb_node,
    root: *mut rb_root,
    color: i32,
) {
    let parent = rb_parent(old);
    (*new).__rb_parent_color = (*old).__rb_parent_color;
    rb_set_parent_color(old, new, color);
    __rb_change_child(old, new, parent, root);
}

#[inline]
unsafe fn __rb_insert(
    mut node: *mut rb_node,
    root: *mut rb_root,
    augment_rotate: unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node),
) {
    let mut parent = rb_red_parent(node);
    let mut gparent: *mut rb_node;
    let mut tmp: *mut rb_node;

    loop {
        /*
         * Loop invariant: node is red.
         */
        if parent.is_null() {
            /*
             * The inserted node is root. Either this is the
             * first node, or we recursed at Case 1 below and
             * are no longer violating 4).
             */
            rb_set_parent_color(node, ptr::null_mut(), RB_BLACK);
            break;
        }

        /*
         * If there is a black parent, we are done.
         * Otherwise, take some corrective action as,
         * per 4), we don't want a red root or two
         * consecutive red nodes.
         */
        if rb_is_black(parent) {
            break;
        }

        gparent = rb_red_parent(parent);

        tmp = (*gparent).rb_right;
        if parent != tmp {
            /* parent == gparent->rb_left */
            if !tmp.is_null() && rb_is_red(tmp) {
                /*
                 * Case 1 - node's uncle is red (color flips).
                 *
                 *       G            g
                 *      / \          / \
                 *     p   u  -->   P   U
                 *    /            /
                 *   n            n
                 *
                 * However, since g's parent might be red, and
                 * 4) does not allow this, we need to recurse
                 * at g.
                 */
                rb_set_parent_color(tmp, gparent, RB_BLACK);
                rb_set_parent_color(parent, gparent, RB_BLACK);
                node = gparent;
                parent = rb_parent(node);
                rb_set_parent_color(node, parent, RB_RED);
                continue;
            }

            tmp = (*parent).rb_right;
            if node == tmp {
                /*
                 * Case 2 - node's uncle is black and node is
                 * the parent's right child (left rotate at parent).
                 *
                 *      G             G
                 *     / \           / \
                 *    p   U  -->    n   U
                 *     \           /
                 *      n         p
                 *
                 * This still leaves us in violation of 4), the
                 * continuation into Case 3 will fix that.
                 */
                tmp = (*node).rb_left;
                WRITE_ONCE(&mut (*parent).rb_right, tmp);
                WRITE_ONCE(&mut (*node).rb_left, parent);
                if !tmp.is_null() {
                    rb_set_parent_color(tmp, parent, RB_BLACK);
                }
                rb_set_parent_color(parent, node, RB_RED);
                augment_rotate(parent, node);
                parent = node;
                tmp = (*node).rb_right;
            }

            /*
             * Case 3 - node's uncle is black and node is
             * the parent's left child (right rotate at gparent).
             *
             *        G           P
             *       / \         / \
             *      p   U  -->  n   g
             *     /                 \
             *    n                   U
             */
            WRITE_ONCE(&mut (*gparent).rb_left, tmp); /* == parent->rb_right */
            WRITE_ONCE(&mut (*parent).rb_right, gparent);
            if !tmp.is_null() {
                rb_set_parent_color(tmp, gparent, RB_BLACK);
            }
            __rb_rotate_set_parents(gparent, parent, root, RB_RED);
            augment_rotate(gparent, parent);
            break;
        } else {
            tmp = (*gparent).rb_left;
            if !tmp.is_null() && rb_is_red(tmp) {
                /* Case 1 - color flips */
                rb_set_parent_color(tmp, gparent, RB_BLACK);
                rb_set_parent_color(parent, gparent, RB_BLACK);
                node = gparent;
                parent = rb_parent(node);
                rb_set_parent_color(node, parent, RB_RED);
                continue;
            }

            tmp = (*parent).rb_left;
            if node == tmp {
                /* Case 2 - right rotate at parent */
                tmp = (*node).rb_right;
                WRITE_ONCE(&mut (*parent).rb_left, tmp);
                WRITE_ONCE(&mut (*node).rb_right, parent);
                if !tmp.is_null() {
                    rb_set_parent_color(tmp, parent, RB_BLACK);
                }
                rb_set_parent_color(parent, node, RB_RED);
                augment_rotate(parent, node);
                parent = node;
                tmp = (*node).rb_left;
            }

            /* Case 3 - left rotate at gparent */
            WRITE_ONCE(&mut (*gparent).rb_right, tmp); /* == parent->rb_left */
            WRITE_ONCE(&mut (*parent).rb_left, gparent);
            if !tmp.is_null() {
                rb_set_parent_color(tmp, gparent, RB_BLACK);
            }
            __rb_rotate_set_parents(gparent, parent, root, RB_RED);
            augment_rotate(gparent, parent);
            break;
        }
    }
}

/*
 * Inline version for rb_erase() use - we want to be able to inline
 * and eliminate the dummy_rotate callback there
 */
#[inline]
unsafe fn ____rb_erase_color(
    mut parent: *mut rb_node,
    root: *mut rb_root,
    augment_rotate: unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node),
) {
    let mut node: *mut rb_node = ptr::null_mut();
    let mut sibling: *mut rb_node;
    let mut tmp1: *mut rb_node;
    let mut tmp2: *mut rb_node;

    loop {
        /*
         * Loop invariants:
         * - node is black (or NULL on first iteration)
         * - node is not the root (parent is not NULL)
         * - All leaf paths going through parent and node have a
         *   black node count that is 1 lower than other leaf paths.
         */
        sibling = (*parent).rb_right;
        if node != sibling {
            /* node == parent->rb_left */
            if rb_is_red(sibling) {
                /*
                 * Case 1 - left rotate at parent
                 *
                 *     P               S
                 *    / \             / \
                 *   N   s    -->    p   Sr
                 *      / \         / \
                 *     Sl  Sr      N   Sl
                 */
                tmp1 = (*sibling).rb_left;
                WRITE_ONCE(&mut (*parent).rb_right, tmp1);
                WRITE_ONCE(&mut (*sibling).rb_left, parent);
                rb_set_parent_color(tmp1, parent, RB_BLACK);
                __rb_rotate_set_parents(parent, sibling, root, RB_RED);
                augment_rotate(parent, sibling);
                sibling = tmp1;
            }
            tmp1 = (*sibling).rb_right;
            if tmp1.is_null() || rb_is_black(tmp1) {
                tmp2 = (*sibling).rb_left;
                if tmp2.is_null() || rb_is_black(tmp2) {
                    /*
                     * Case 2 - sibling color flip
                     * (p could be either color here)
                     *
                     *    (p)           (p)
                     *    / \           / \
                     *   N   S    -->  N   s
                     *      / \           / \
                     *     Sl  Sr        Sl  Sr
                     *
                     * This leaves us violating 5) which
                     * can be fixed by flipping p to black
                     * if it was red, or by recursing at p.
                     * p is red when coming from Case 1.
                     */
                    rb_set_parent_color(sibling, parent, RB_RED);
                    if rb_is_red(parent) {
                        rb_set_black(parent);
                    } else {
                        node = parent;
                        parent = rb_parent(node);
                        if !parent.is_null() {
                            continue;
                        }
                    }
                    break;
                }
                /*
                 * Case 3 - right rotate at sibling
                 * (p could be either color here)
                 *
                 *   (p)           (p)
                 *   / \           / \
                 *  N   S    -->  N   sl
                 *     / \             \
                 *    sl  Sr            S
                 *                       \
                 *                        Sr
                 *
                 * Note: p might be red, and then both
                 * p and sl are red after rotation(which
                 * breaks property 4). This is fixed in
                 * Case 4 (in __rb_rotate_set_parents()
                 *         which set sl the color of p
                 *         and set p RB_BLACK)
                 *
                 *   (p)            (sl)
                 *   / \            /  \
                 *  N   sl   -->   P    S
                 *       \        /      \
                 *        S      N        Sr
                 *         \
                 *          Sr
                 */
                tmp1 = (*tmp2).rb_right;
                WRITE_ONCE(&mut (*sibling).rb_left, tmp1);
                WRITE_ONCE(&mut (*tmp2).rb_right, sibling);
                WRITE_ONCE(&mut (*parent).rb_right, tmp2);
                if !tmp1.is_null() {
                    rb_set_parent_color(tmp1, sibling, RB_BLACK);
                }
                augment_rotate(sibling, tmp2);
                tmp1 = sibling;
                sibling = tmp2;
            }
            /*
             * Case 4 - left rotate at parent + color flips
             * (p and sl could be either color here.
             *  After rotation, p becomes black, s acquires
             *  p's color, and sl keeps its color)
             *
             *      (p)             (s)
             *      / \             / \
             *     N   S     -->   P   Sr
             *        / \         / \
             *      (sl) sr      N  (sl)
             */
            tmp2 = (*sibling).rb_left;
            WRITE_ONCE(&mut (*parent).rb_right, tmp2);
            WRITE_ONCE(&mut (*sibling).rb_left, parent);
            rb_set_parent_color(tmp1, sibling, RB_BLACK);
            if !tmp2.is_null() {
                rb_set_parent(tmp2, parent);
            }
            __rb_rotate_set_parents(parent, sibling, root, RB_BLACK);
            augment_rotate(parent, sibling);
            break;
        } else {
            sibling = (*parent).rb_left;
            if rb_is_red(sibling) {
                /* Case 1 - right rotate at parent */
                tmp1 = (*sibling).rb_right;
                WRITE_ONCE(&mut (*parent).rb_left, tmp1);
                WRITE_ONCE(&mut (*sibling).rb_right, parent);
                rb_set_parent_color(tmp1, parent, RB_BLACK);
                __rb_rotate_set_parents(parent, sibling, root, RB_RED);
                augment_rotate(parent, sibling);
                sibling = tmp1;
            }
            tmp1 = (*sibling).rb_left;
            if tmp1.is_null() || rb_is_black(tmp1) {
                tmp2 = (*sibling).rb_right;
                if tmp2.is_null() || rb_is_black(tmp2) {
                    /* Case 2 - sibling color flip */
                    rb_set_parent_color(sibling, parent, RB_RED);
                    if rb_is_red(parent) {
                        rb_set_black(parent);
                    } else {
                        node = parent;
                        parent = rb_parent(node);
                        if !parent.is_null() {
                            continue;
                        }
                    }
                    break;
                }
                /* Case 3 - left rotate at sibling */
                tmp1 = (*tmp2).rb_left;
                WRITE_ONCE(&mut (*sibling).rb_right, tmp1);
                WRITE_ONCE(&mut (*tmp2).rb_left, sibling);
                WRITE_ONCE(&mut (*parent).rb_left, tmp2);
                if !tmp1.is_null() {
                    rb_set_parent_color(tmp1, sibling, RB_BLACK);
                }
                augment_rotate(sibling, tmp2);
                tmp1 = sibling;
                sibling = tmp2;
            }
            /* Case 4 - right rotate at parent + color flips */
            tmp2 = (*sibling).rb_right;
            WRITE_ONCE(&mut (*parent).rb_left, tmp2);
            WRITE_ONCE(&mut (*sibling).rb_right, parent);
            rb_set_parent_color(tmp1, sibling, RB_BLACK);
            if !tmp2.is_null() {
                rb_set_parent(tmp2, parent);
            }
            __rb_rotate_set_parents(parent, sibling, root, RB_BLACK);
            augment_rotate(parent, sibling);
            break;
        }
    }
}

/* Non-inline version for rb_erase_augmented() use */
#[no_mangle]
pub unsafe extern "C" fn __rb_erase_color(
    parent: *mut rb_node,
    root: *mut rb_root,
    augment_rotate: unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node),
) {
    ____rb_erase_color(parent, root, augment_rotate);
}

/*
 * Non-augmented rbtree manipulation functions.
 *
 * We use dummy augmented callbacks here, and have the compiler optimize them
 * out of the rb_insert_color() and rb_erase() function definitions.
 */

#[inline]
unsafe extern "C" fn dummy_propagate(_node: *mut rb_node, _stop: *mut rb_node) {}
#[inline]
unsafe extern "C" fn dummy_copy(_old: *mut rb_node, _new: *mut rb_node) {}
#[inline]
unsafe extern "C" fn dummy_rotate(_old: *mut rb_node, _new: *mut rb_node) {}

static dummy_callbacks: rb_augment_callbacks = rb_augment_callbacks {
    propagate: Some(dummy_propagate),
    copy: Some(dummy_copy),
    rotate: Some(dummy_rotate),
};

#[no_mangle]
pub unsafe extern "C" fn rb_insert_color(node: *mut rb_node, root: *mut rb_root) {
    __rb_insert(node, root, dummy_rotate);
}

#[no_mangle]
pub unsafe extern "C" fn rb_erase(node: *mut rb_node, root: *mut rb_root) {
    let rebalance: *mut rb_node;
    rebalance = __rb_erase_augmented(node, root, &dummy_callbacks);
    if !rebalance.is_null() {
        ____rb_erase_color(rebalance, root, dummy_rotate);
    }
}

/*
 * Augmented rbtree manipulation functions.
 *
 * This instantiates the same __always_inline functions as in the non-augmented
 * case, but this time with user-defined callbacks.
 */

#[no_mangle]
pub unsafe extern "C" fn __rb_insert_augmented(
    node: *mut rb_node,
    root: *mut rb_root,
    augment_rotate: unsafe extern "C" fn(old: *mut rb_node, new: *mut rb_node),
) {
    __rb_insert(node, root, augment_rotate);
}

/*
 * This function returns the first node (in sort order) of the tree.
 */
#[no_mangle]
pub unsafe extern "C" fn rb_first(root: *const rb_root) -> *mut rb_node {
    let mut n: *mut rb_node;

    n = (*root).rb_node;
    if n.is_null() {
        return ptr::null_mut();
    }
    while !(*n).rb_left.is_null() {
        n = (*n).rb_left;
    }
    n
}

#[no_mangle]
pub unsafe extern "C" fn rb_last(root: *const rb_root) -> *mut rb_node {
    let mut n: *mut rb_node;

    n = (*root).rb_node;
    if n.is_null() {
        return ptr::null_mut();
    }
    while !(*n).rb_right.is_null() {
        n = (*n).rb_right;
    }
    n
}

#[no_mangle]
pub unsafe extern "C" fn rb_next(mut node: *const rb_node) -> *mut rb_node {
    let mut parent: *mut rb_node;

    if RB_EMPTY_NODE(node) {
        return ptr::null_mut();
    }

    /*
     * If we have a right-hand child, go down and then left as far
     * as we can.
     */
    if !(*node).rb_right.is_null() {
        node = (*node).rb_right;
        while !(*node).rb_left.is_null() {
            node = (*node).rb_left;
        }
        return node as *mut rb_node;
    }

    /*
     * No right-hand children. Everything down and left is smaller than us,
     * so any 'next' node must be in the general direction of our parent.
     * Go up the tree; any time the ancestor is a right-hand child of its
     * parent, keep going up. First time it's a left-hand child of its
     * parent, said parent is our 'next' node.
     */
    loop {
        parent = rb_parent(node);
        if parent.is_null() || node != (*parent).rb_right {
            break;
        }
        node = parent;
    }

    parent
}

#[no_mangle]
pub unsafe extern "C" fn rb_prev(mut node: *const rb_node) -> *mut rb_node {
    let mut parent: *mut rb_node;

    if RB_EMPTY_NODE(node) {
        return ptr::null_mut();
    }

    /*
     * If we have a left-hand child, go down and then right as far
     * as we can.
     */
    if !(*node).rb_left.is_null() {
        node = (*node).rb_left;
        while !(*node).rb_right.is_null() {
            node = (*node).rb_right;
        }
        return node as *mut rb_node;
    }

    /*
     * No left-hand children. Go up till we find an ancestor which
     * is a right-hand child of its parent.
     */
    loop {
        parent = rb_parent(node);
        if parent.is_null() || node != (*parent).rb_left {
            break;
        }
        node = parent;
    }

    parent
}

#[no_mangle]
pub unsafe extern "C" fn rb_replace_node(
    victim: *mut rb_node,
    new: *mut rb_node,
    root: *mut rb_root,
) {
    let parent: *mut rb_node = rb_parent(victim);

    /* Copy the pointers/colour from the victim to the replacement */
    *new = *victim;

    /* Set the surrounding nodes to point to the replacement */
    if !(*victim).rb_left.is_null() {
        rb_set_parent((*victim).rb_left, new);
    }
    if !(*victim).rb_right.is_null() {
        rb_set_parent((*victim).rb_right, new);
    }
    __rb_change_child(victim, new, parent, root);
}

unsafe fn rb_left_deepest_node(mut node: *const rb_node) -> *mut rb_node {
    loop {
        if !(*node).rb_left.is_null() {
            node = (*node).rb_left;
        } else if !(*node).rb_right.is_null() {
            node = (*node).rb_right;
        } else {
            return node as *mut rb_node;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rb_next_postorder(node: *const rb_node) -> *mut rb_node {
    let parent: *const rb_node;
    if node.is_null() {
        return ptr::null_mut();
    }
    parent = rb_parent(node);

    /* If we're sitting on node, we've already seen our children */
    if !parent.is_null() && node == (*parent).rb_left && !(*parent).rb_right.is_null() {
        /*
         * If we are the parent's left node, go to the parent's right
         * node then all the way down to the left
         */
        rb_left_deepest_node((*parent).rb_right)
    } else {
        /*
         * Otherwise we are the parent's right node, and the parent
         * should be next
         */
        parent as *mut rb_node
    }
}

#[no_mangle]
pub unsafe extern "C" fn rb_first_postorder(root: *const rb_root) -> *mut rb_node {
    if (*root).rb_node.is_null() {
        return ptr::null_mut();
    }

    rb_left_deepest_node((*root).rb_node)
}
