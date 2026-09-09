/* SPDX-License-Identifier: GPL-2.0-or-later */

#[repr(C, align(8))]
pub struct rb_node {
    pub __rb_parent_color: ::core::ffi::c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}
/* The alignment might seem pointless, but allegedly CRIS needs it. */

#[repr(C)]
pub struct rb_node_linked {
    pub node: rb_node,
    pub prev: *mut rb_node_linked,
    pub next: *mut rb_node_linked,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
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

/*
 * Leftmost tree with links. This would allow a trivial rb_rightmost update,
 * but that has been omitted due to the lack of users.
 */
#[repr(C)]
pub struct rb_root_linked {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node_linked,
}

pub const RB_ROOT: rb_root = rb_root {
    rb_node: ::core::ptr::null_mut(),
};

pub const RB_ROOT_CACHED: rb_root_cached = rb_root_cached {
    rb_root: rb_root {
        rb_node: ::core::ptr::null_mut(),
    },
    rb_leftmost: ::core::ptr::null_mut(),
};

pub const RB_ROOT_LINKED: rb_root_linked = rb_root_linked {
    rb_root: rb_root {
        rb_node: ::core::ptr::null_mut(),
    },
    rb_leftmost: ::core::ptr::null_mut(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
