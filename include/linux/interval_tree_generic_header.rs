/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
  Interval Trees
  (C) 2012  Michel Lespinasse <walken@google.com>

  Rust translation of include/linux/interval_tree_generic.h
*/

/*
 * Template for implementing interval trees.
 * The red-black-tree types and helpers used below are supplied externally.
 */

macro_rules! INTERVAL_TREE_DEFINE {
    ($itstruct:ty, $itrb:ident, $ittype:ty, $itsubtree:ident,
     $itstart:expr, $itlast:expr, $itstatic:ident, $itprefix:ident) => {
        $itstatic fn $itprefix##_augment(node: *mut $itstruct) {
            /* RB_DECLARE_CALLBACKS_MAX(...): external augmented-rbtree callback. */
            unsafe {
                let _ = node;
            }
        }

        $itstatic unsafe fn $itprefix##_insert(
            node: *mut $itstruct,
            root: *mut rb_root_cached,
        ) {
            let mut link: *mut *mut rb_node = &mut (*root).rb_root.rb_node;
            let mut rb_parent: *mut rb_node = core::ptr::null_mut();
            let start: $ittype = $itstart(node);
            let last: $ittype = $itlast(node);
            let mut leftmost = true;

            while !(*link).is_null() {
                rb_parent = *link;
                let parent: *mut $itstruct = rb_entry(rb_parent);
                if (*parent).$itsubtree < last {
                    (*parent).$itsubtree = last;
                }
                if start < $itstart(parent) {
                    link = &mut (*parent).$itrb.rb_left;
                } else {
                    link = &mut (*parent).$itrb.rb_right;
                    leftmost = false;
                }
            }

            (*node).$itsubtree = last;
            rb_link_node(&mut (*node).$itrb, rb_parent, link);
            rb_insert_augmented_cached(
                &mut (*node).$itrb,
                root,
                leftmost,
                $itprefix##_augment,
            );
        }

        $itstatic unsafe fn $itprefix##_remove(
            node: *mut $itstruct,
            root: *mut rb_root_cached,
        ) {
            rb_erase_augmented_cached(&mut (*node).$itrb, root, $itprefix##_augment);
        }

        /* Iterate over intervals intersecting [start; last]. */
        $itstatic unsafe fn $itprefix##_subtree_search(
            mut node: *mut $itstruct,
            start: $ittype,
            last: $ittype,
        ) -> *mut $itstruct {
            loop {
                /* Loop invariant: start <= node->ITSUBTREE. */
                if !(*node).$itrb.rb_left.is_null() {
                    let left: *mut $itstruct = rb_entry((*node).$itrb.rb_left);
                    if start <= (*left).$itsubtree {
                        node = left;
                        continue;
                    }
                }
                if $itstart(node) <= last {
                    if start <= $itlast(node) {
                        return node;
                    }
                    node = rb_entry((*node).$itrb.rb_right);
                    continue;
                }
                return core::ptr::null_mut();
            }
        }

        $itstatic unsafe fn $itprefix##_iter_first(
            root: *mut rb_root_cached,
            start: $ittype,
            last: $ittype,
        ) -> *mut $itstruct {
            if (*root).rb_root.rb_node.is_null() {
                return core::ptr::null_mut();
            }
            let node: *mut $itstruct = rb_entry((*root).rb_root.rb_node);
            if (*node).$itsubtree < start {
                return core::ptr::null_mut();
            }
            let leftmost: *mut $itstruct = rb_entry((*root).rb_leftmost);
            if $itstart(leftmost) > last {
                return core::ptr::null_mut();
            }
            $itprefix##_subtree_search(node, start, last)
        }

        $itstatic unsafe fn $itprefix##_iter_next(
            mut node: *mut $itstruct,
            start: $ittype,
            last: $ittype,
        ) -> *mut $itstruct {
            let mut rb = (*node).$itrb.rb_right;
            loop {
                if !rb.is_null() {
                    let right: *mut $itstruct = rb_entry(rb);
                    if start <= (*right).$itsubtree {
                        return $itprefix##_subtree_search(right, start, last);
                    }
                }
                loop {
                    rb = rb_parent(&mut (*node).$itrb);
                    if rb.is_null() {
                        return core::ptr::null_mut();
                    }
                    let prev = &mut (*node).$itrb as *mut rb_node;
                    node = rb_entry(rb);
                    rb = (*node).$itrb.rb_right;
                    if prev != rb {
                        break;
                    }
                }
                if last < $itstart(node) {
                    return core::ptr::null_mut();
                } else if start <= $itlast(node) {
                    return node;
                }
            }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
