/* SPDX-License-Identifier: GPL-2.0 */
// Translated from list_types.h; the C header guard is not needed in Rust.

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
