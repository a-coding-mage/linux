/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
  Interval Trees
  (C) 2012  Michel Lespinasse <walken@google.com>


  include/linux/interval_tree_generic.h
*/

/* Depends on the Rust translation of <linux/rbtree_augmented.h>. */

/*
 * Template for implementing interval trees
 *
 * ITSTRUCT:   struct type of the interval tree nodes
 * ITRB:       name of struct rb_node field within ITSTRUCT
 * ITTYPE:     type of the interval endpoints
 * ITSUBTREE:  name of ITTYPE field within ITSTRUCT holding last-in-subtree
 * ITSTART(n): start endpoint of ITSTRUCT node n
 * ITLAST(n):  last endpoint of ITSTRUCT node n
 * ITSTATIC:   'static' or empty
 * ITPREFIX:   prefix to use for the inline tree definitions
 *
 * Note - before using this, please consider if generic version
 * (interval_tree.h) would work for you...
 */

#[macro_export]
macro_rules! INTERVAL_TREE_DEFINE {
    (
        $ITSTRUCT:ty,
        $ITRB:ident,
        $ITTYPE:ty,
        $ITSUBTREE:ident,
        $ITSTART:ident,
        $ITLAST:ident,
        $ITSTATIC:vis,
        $ITPREFIX:ident
    ) => {
        ::paste::paste! {
            /* Callbacks for augmented rbtree insert and remove */

            RB_DECLARE_CALLBACKS_MAX!(
                pub(crate),
                [<$ITPREFIX _augment>],
                $ITSTRUCT,
                $ITRB,
                $ITTYPE,
                $ITSUBTREE,
                $ITLAST
            );

            /* Insert / remove interval nodes from the tree */

            $ITSTATIC unsafe fn [<$ITPREFIX _insert>](
                node: *mut $ITSTRUCT,
                root: *mut rb_root_cached,
            ) {
                let mut link: *mut *mut rb_node = &mut (*root).rb_root.rb_node;
                let mut rb_parent: *mut rb_node = core::ptr::null_mut();
                let start: $ITTYPE = $ITSTART(node);
                let last: $ITTYPE = $ITLAST(node);
                let mut parent: *mut $ITSTRUCT;
                let mut leftmost: bool = true;

                while !(*link).is_null() {
                    rb_parent = *link;
                    parent = rb_entry!(rb_parent, $ITSTRUCT, $ITRB);
                    if (*parent).$ITSUBTREE < last {
                        (*parent).$ITSUBTREE = last;
                    }
                    if start < $ITSTART(parent) {
                        link = &mut (*parent).$ITRB.rb_left;
                    } else {
                        link = &mut (*parent).$ITRB.rb_right;
                        leftmost = false;
                    }
                }

                (*node).$ITSUBTREE = last;
                rb_link_node(&mut (*node).$ITRB, rb_parent, link);
                rb_insert_augmented_cached(
                    &mut (*node).$ITRB,
                    root,
                    leftmost,
                    &[<$ITPREFIX _augment>],
                );
            }

            $ITSTATIC unsafe fn [<$ITPREFIX _remove>](
                node: *mut $ITSTRUCT,
                root: *mut rb_root_cached,
            ) {
                rb_erase_augmented_cached(&mut (*node).$ITRB, root, &[<$ITPREFIX _augment>]);
            }

            /*
             * Iterate over intervals intersecting [start;last]
             *
             * Note that a node's interval intersects [start;last] iff:
             *   Cond1: ITSTART(node) <= last
             * and
             *   Cond2: start <= ITLAST(node)
             */

            $ITSTATIC unsafe fn [<$ITPREFIX _subtree_search>](
                mut node: *mut $ITSTRUCT,
                start: $ITTYPE,
                last: $ITTYPE,
            ) -> *mut $ITSTRUCT {
                loop {
                    /*
                     * Loop invariant: start <= node->ITSUBTREE
                     * (Cond2 is satisfied by one of the subtree nodes)
                     */
                    if !(*node).$ITRB.rb_left.is_null() {
                        let left: *mut $ITSTRUCT =
                            rb_entry!((*node).$ITRB.rb_left, $ITSTRUCT, $ITRB);
                        if start <= (*left).$ITSUBTREE {
                            /*
                             * Some nodes in left subtree satisfy Cond2.
                             * Iterate to find the leftmost such node N.
                             * If it also satisfies Cond1, that's the
                             * match we are looking for. Otherwise, there
                             * is no matching interval as nodes to the
                             * right of N can't satisfy Cond1 either.
                             */
                            node = left;
                            continue;
                        }
                    }
                    if $ITSTART(node) <= last {
                        /* Cond1 */
                        if start <= $ITLAST(node) {
                            /* Cond2 */
                            return node; /* node is leftmost match */
                        }
                        node = rb_entry!((*node).$ITRB.rb_right, $ITSTRUCT, $ITRB);
                        continue;
                    }
                    return core::ptr::null_mut(); /* No match */
                }
            }

            $ITSTATIC unsafe fn [<$ITPREFIX _iter_first>](
                root: *mut rb_root_cached,
                start: $ITTYPE,
                last: $ITTYPE,
            ) -> *mut $ITSTRUCT {
                let mut node: *mut $ITSTRUCT;
                let leftmost: *mut $ITSTRUCT;

                if (*root).rb_root.rb_node.is_null() {
                    return core::ptr::null_mut();
                }

                /*
                 * Fastpath range intersection/overlap between A: [a0, a1] and
                 * B: [b0, b1] is given by:
                 *
                 *         a0 <= b1 && b0 <= a1
                 *
                 *  ... where A holds the lock range and B holds the smallest
                 * 'start' and largest 'last' in the tree. For the later, we
                 * rely on the root node, which by augmented interval tree
                 * property, holds the largest value in its last-in-subtree.
                 * This allows mitigating some of the tree walk overhead for
                 * for non-intersecting ranges, maintained and consulted in O(1).
                 */
                node = rb_entry!((*root).rb_root.rb_node, $ITSTRUCT, $ITRB);
                if (*node).$ITSUBTREE < start {
                    return core::ptr::null_mut();
                }

                leftmost = rb_entry!((*root).rb_leftmost, $ITSTRUCT, $ITRB);
                if $ITSTART(leftmost) > last {
                    return core::ptr::null_mut();
                }

                [<$ITPREFIX _subtree_search>](node, start, last)
            }

            $ITSTATIC unsafe fn [<$ITPREFIX _iter_next>](
                mut node: *mut $ITSTRUCT,
                start: $ITTYPE,
                last: $ITTYPE,
            ) -> *mut $ITSTRUCT {
                let mut rb: *mut rb_node = (*node).$ITRB.rb_right;
                let mut prev: *mut rb_node;

                loop {
                    /*
                     * Loop invariants:
                     *   Cond1: ITSTART(node) <= last
                     *   rb == node->ITRB.rb_right
                     *
                     * First, search right subtree if suitable
                     */
                    if !rb.is_null() {
                        let right: *mut $ITSTRUCT = rb_entry!(rb, $ITSTRUCT, $ITRB);
                        if start <= (*right).$ITSUBTREE {
                            return [<$ITPREFIX _subtree_search>](right, start, last);
                        }
                    }

                    /* Move up the tree until we come from a node's left child */
                    loop {
                        rb = rb_parent(&mut (*node).$ITRB);
                        if rb.is_null() {
                            return core::ptr::null_mut();
                        }
                        prev = &mut (*node).$ITRB;
                        node = rb_entry!(rb, $ITSTRUCT, $ITRB);
                        rb = (*node).$ITRB.rb_right;
                        if prev != rb {
                            break;
                        }
                    }

                    /* Check if the node intersects [start;last] */
                    if last < $ITSTART(node) {
                        /* !Cond1 */
                        return core::ptr::null_mut();
                    } else if start <= $ITLAST(node) {
                        /* Cond2 */
                        return node;
                    }
                }
            }
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
