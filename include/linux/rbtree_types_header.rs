/* SPDX-License-Identifier: GPL-2.0-or-later */

/// Native binding identity is required for both layout and KCFI type IDs.
pub use kernel::bindings::{rb_node, rb_node_linked, rb_root, rb_root_cached, rb_root_linked};

/* The alignment might seem pointless, but allegedly CRIS needs it. */
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
/*
 * Leftmost tree with links. This would allow a trivial rb_rightmost update,
 * but that has been omitted due to the lack of users.
 */

// Also compiled for genuine i686: never assume 64-bit alignment or field order.
const _: () = {
    let word = core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<rb_node>() == 3 * word);
    assert!(core::mem::align_of::<rb_node>() == core::mem::align_of::<usize>());
    assert!(core::mem::offset_of!(rb_node, rb_right) == word);
    assert!(core::mem::offset_of!(rb_node, rb_left) == 2 * word);
    assert!(core::mem::offset_of!(rb_node_linked, prev) == 3 * word);
    assert!(core::mem::offset_of!(rb_node_linked, next) == 4 * word);
    assert!(core::mem::size_of::<rb_node_linked>() == 5 * word);
    assert!(core::mem::size_of::<rb_root>() == word);
    assert!(core::mem::size_of::<rb_root_cached>() == 2 * word);
    assert!(core::mem::size_of::<rb_root_linked>() == 2 * word);
};

/// RB_ROOT from the original Linux rbtree interface.
pub const RB_ROOT: rb_root = rb_root {
    rb_node: ::core::ptr::null_mut(),
};

/// RB_ROOT_CACHED from the original Linux rbtree interface.
pub const RB_ROOT_CACHED: rb_root_cached = rb_root_cached {
    rb_root: rb_root {
        rb_node: ::core::ptr::null_mut(),
    },
    rb_leftmost: ::core::ptr::null_mut(),
};

/// RB_ROOT_LINKED from the original Linux rbtree interface.
pub const RB_ROOT_LINKED: rb_root_linked = rb_root_linked {
    rb_root: rb_root {
        rb_node: ::core::ptr::null_mut(),
    },
    rb_leftmost: ::core::ptr::null_mut(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
