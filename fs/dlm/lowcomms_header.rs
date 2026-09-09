/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2009 Red Hat, Inc.  All rights reserved.
**
*******************************************************************************
******************************************************************************/

// Dependency declarations from "dlm_internal.h" are supplied externally.

use core::ffi::c_void;
use core::mem::size_of;

pub const DLM_MIDCOMMS_OPT_LEN: usize = size_of::<dlm_opts>();
pub const DLM_MAX_APP_BUFSIZE: usize = DLM_MAX_SOCKET_BUFSIZE - DLM_MIDCOMMS_OPT_LEN;

pub const CONN_HASH_SIZE: i32 = 32;

/* This is deliberately very simple because most clusters have simple
 * sequential nodeids, so we should be able to go straight to a connection
 * struct in the array
 */
#[inline]
pub fn nodeid_hash(nodeid: i32) -> i32 {
    nodeid & (CONN_HASH_SIZE - 1)
}

/* check if dlm is running */
extern "C" {
    pub fn dlm_lowcomms_is_running() -> bool;

    pub fn dlm_lowcomms_start() -> i32;
    pub fn dlm_lowcomms_shutdown();
    pub fn dlm_lowcomms_shutdown_node(nodeid: i32, force: bool);
    pub fn dlm_lowcomms_stop();
    pub fn dlm_lowcomms_init();
    pub fn dlm_lowcomms_exit();
    pub fn dlm_lowcomms_close(nodeid: i32) -> i32;
    pub fn dlm_lowcomms_new_msg(
        nodeid: i32,
        len: i32,
        ppc: *mut *mut u8,
        cb: Option<unsafe extern "C" fn(data: *mut c_void)>,
        data: *mut c_void,
    ) -> *mut dlm_msg;
    pub fn dlm_lowcomms_commit_msg(msg: *mut dlm_msg);
    pub fn dlm_lowcomms_put_msg(msg: *mut dlm_msg);
    pub fn dlm_lowcomms_resend_msg(msg: *mut dlm_msg) -> i32;
    pub fn dlm_lowcomms_connect_node(nodeid: i32) -> i32;
    pub fn dlm_lowcomms_nodes_set_mark(nodeid: i32, mark: u32) -> i32;
    pub fn dlm_lowcomms_addr(nodeid: i32, addr: *mut sockaddr_storage) -> i32;
    pub fn dlm_midcomms_receive_done(nodeid: i32);
    pub fn dlm_lowcomms_writequeue_cache_create() -> *mut kmem_cache;
    pub fn dlm_lowcomms_msg_cache_create() -> *mut kmem_cache;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
