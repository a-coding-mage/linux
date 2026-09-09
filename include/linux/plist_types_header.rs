/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency provided by linux/types.h in the original header:
// struct list_head;

#[repr(C)]
pub struct plist_head {
    pub node_list: list_head,
}

#[repr(C)]
pub struct plist_node {
    pub prio: i32,
    pub prio_list: list_head,
    pub node_list: list_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
