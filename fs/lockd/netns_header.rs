/* SPDX-License-Identifier: GPL-2.0 */

// The declarations supplied by the Linux kernel headers included by netns.h
// are external dependencies of this translation.

#[repr(C)]
pub struct lockd_net {
    pub nlmsvc_users: ::core::ffi::c_uint,
    pub next_gc: ::core::ffi::c_ulong,
    pub nrhosts: ::core::ffi::c_ulong,
    pub gracetime: u32,
    pub tcp_port: u16,
    pub udp_port: u16,

    pub grace_period_end: delayed_work,
    pub lockd_manager: lock_manager,

    pub nsm_handles: list_head,
}

extern "C" {
    pub static mut lockd_net_id: ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
