/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2011 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C header guard: __CONFIG_DOT_H__

pub const DLM_MAX_SOCKET_BUFSIZE: i32 = 4096;

#[repr(C)]
pub struct dlm_config_node {
    pub nodeid: i32,
    pub weight: i32,
    pub gone: bool,
    pub new: i32,
    pub comm_seq: u32,
    pub release_recover: u32,
}

// `rhashtable_params` is supplied by an external dependency.
extern "C" {
    pub static dlm_rhash_rsb_params: rhashtable_params;
}

pub const DLM_MAX_ADDR_COUNT: i32 = 8;

pub const DLM_PROTO_TCP: i32 = 0;
pub const DLM_PROTO_SCTP: i32 = 1;

#[repr(C)]
pub struct dlm_config_info {
    pub ci_tcp_port: __be16,
    pub ci_buffer_size: u32,
    pub ci_rsbtbl_size: u32,
    pub ci_recover_timer: u32,
    pub ci_toss_secs: u32,
    pub ci_scan_secs: u32,
    pub ci_log_debug: u32,
    pub ci_log_info: u32,
    pub ci_protocol: u32,
    pub ci_mark: u32,
    pub ci_new_rsb_count: u32,
    pub ci_recover_callbacks: u32,
    pub ci_cluster_name: [core::ffi::c_char; DLM_LOCKSPACE_LEN],
}

extern "C" {
    pub static mut dlm_config: dlm_config_info;

    pub fn dlm_config_init() -> i32;
    pub fn dlm_config_exit();
    pub fn dlm_config_nodes(
        lsname: *mut core::ffi::c_char,
        nodes_out: *mut *mut dlm_config_node,
        count_out: *mut i32,
    ) -> i32;
    pub fn dlm_comm_seq(nodeid: i32, seq: *mut u32, locked: bool) -> i32;
    pub fn dlm_our_nodeid() -> i32;
    pub fn dlm_our_addr(addr: *mut sockaddr_storage, num: i32) -> i32;
}

// `__be16`, `DLM_LOCKSPACE_LEN`, `rhashtable_params`, and `sockaddr_storage`
// are supplied by external dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
