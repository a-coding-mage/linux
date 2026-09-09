/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Unix network namespace
 */

// Dependency supplied by the surrounding translation unit:
// use the corresponding Rust definitions for `spinlock_t` and `hlist_head`.

#[repr(C)]
pub struct unix_table {
    pub locks: *mut spinlock_t,
    pub buckets: *mut hlist_head,
}

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netns_unix {
    pub table: unix_table,
    pub sysctl_max_dgram_qlen: i32,
    pub ctl: *mut ctl_table_header,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
