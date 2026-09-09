/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2005 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

/* C header guard: __MIDCOMMS_DOT_H__ */

use core::ffi::{c_char, c_int, c_uchar, c_ulong, c_void};

/* Opaque types supplied by the surrounding C/Rust translation unit. */
#[repr(C)]
pub struct midcomms_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_mhandle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr_storage {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

extern "C" {
    pub fn dlm_validate_incoming_buffer(nodeid: c_int, buf: *mut c_uchar, len: c_int) -> c_int;
    pub fn dlm_process_incoming_buffer(
        nodeid: c_int,
        buf: *mut c_uchar,
        buflen: c_int,
    ) -> c_int;
    pub fn dlm_midcomms_get_mhandle(
        nodeid: c_int,
        len: c_int,
        ppc: *mut *mut c_char,
    ) -> *mut dlm_mhandle;
    pub fn dlm_midcomms_commit_mhandle(
        mh: *mut dlm_mhandle,
        name: *const c_void,
        namelen: c_int,
    );
    pub fn dlm_midcomms_addr(nodeid: c_int, addr: *mut sockaddr_storage) -> c_int;
    pub fn dlm_midcomms_version_wait();
    pub fn dlm_midcomms_close(nodeid: c_int) -> c_int;
    pub fn dlm_midcomms_start() -> c_int;
    pub fn dlm_midcomms_stop();
    pub fn dlm_midcomms_init();
    pub fn dlm_midcomms_exit();
    pub fn dlm_midcomms_shutdown();
    pub fn dlm_midcomms_add_member(nodeid: c_int);
    pub fn dlm_midcomms_remove_member(nodeid: c_int);
    pub fn dlm_midcomms_unack_msg_resend(nodeid: c_int);
    pub fn dlm_midcomms_state(node: *mut midcomms_node) -> *const c_char;
    pub fn dlm_midcomms_flags(node: *mut midcomms_node) -> c_ulong;
    pub fn dlm_midcomms_send_queue_cnt(node: *mut midcomms_node) -> c_int;
    pub fn dlm_midcomms_version(node: *mut midcomms_node) -> u32;
    pub fn dlm_midcomms_rawmsg_send(
        node: *mut midcomms_node,
        buf: *mut c_void,
        buflen: c_int,
    ) -> c_int;
    pub fn dlm_midcomms_cache_create() -> *mut kmem_cache;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
