/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/types.h> and the surrounding kernel sources.
pub enum ctl_table_header {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum vsock_net_mode {
    VSOCK_NET_MODE_GLOBAL,
    VSOCK_NET_MODE_LOCAL,
}

#[repr(C)]
pub struct netns_vsock {
    pub sysctl_hdr: *mut ctl_table_header,

    /* protected by the vsock_table_lock in af_vsock.c */
    pub port: u32,

    pub mode: vsock_net_mode,
    pub child_ns_mode: vsock_net_mode,

    /* 0 = unlocked, 1 = locked to global, 2 = locked to local */
    pub child_ns_mode_locked: i32,

    pub g2h_fallback: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
