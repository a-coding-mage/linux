/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *
 *   Author(s): Long Li <longli@microsoft.com>
 */

/* The C header is conditional on CONFIG_CIFS_SMB_DIRECT. */

#[cfg(CONFIG_CIFS_SMB_DIRECT)]
pub unsafe fn cifs_rdma_enabled(server: *mut TCP_Server_Info) -> bool {
    (*server).rdma
}

#[cfg(CONFIG_CIFS_SMB_DIRECT)]
extern "C" {
    pub static mut rdma_readwrite_threshold: ::core::ffi::c_int;
    pub static mut smbd_max_frmr_depth: ::core::ffi::c_int;
    pub static mut smbd_keep_alive_interval: ::core::ffi::c_int;
    pub static mut smbd_max_receive_size: ::core::ffi::c_int;
    pub static mut smbd_max_fragmented_recv_size: ::core::ffi::c_int;
    pub static mut smbd_max_send_size: ::core::ffi::c_int;
    pub static mut smbd_send_credit_target: ::core::ffi::c_int;
    pub static mut smbd_receive_credit_max: ::core::ffi::c_int;
}

#[cfg(CONFIG_CIFS_SMB_DIRECT)]
#[repr(C)]
pub struct smbd_connection {
    pub socket: *mut smbdirect_socket,
}

#[cfg(CONFIG_CIFS_SMB_DIRECT)]
extern "C" {
    /* Create a SMBDirect session */
    pub fn smbd_get_connection(
        server: *mut TCP_Server_Info,
        dstaddr: *mut sockaddr,
    ) -> *mut smbd_connection;

    pub fn smbd_get_parameters(
        conn: *mut smbd_connection,
    ) -> *const smbdirect_socket_parameters;

    /* Reconnect SMBDirect session */
    pub fn smbd_reconnect(server: *mut TCP_Server_Info) -> ::core::ffi::c_int;
    /* Destroy SMBDirect session */
    pub fn smbd_destroy(server: *mut TCP_Server_Info);

    /* Interface for carrying upper layer I/O through send/recv */
    pub fn smbd_recv(info: *mut smbd_connection, msg: *mut msghdr) -> ::core::ffi::c_int;
    pub fn smbd_send(
        server: *mut TCP_Server_Info,
        num_rqst: ::core::ffi::c_int,
        rqst: *mut smb_rqst,
    ) -> ::core::ffi::c_int;

    /* Interfaces to register and deregister MR for RDMA read/write */
    pub fn smbd_register_mr(
        info: *mut smbd_connection,
        iter: *mut iov_iter,
        writing: bool,
        need_invalidate: bool,
    ) -> *mut smbdirect_mr_io;
    pub fn smbd_mr_fill_buffer_descriptor(
        mr: *mut smbdirect_mr_io,
        v1: *mut smbdirect_buffer_descriptor_v1,
    );
    pub fn smbd_deregister_mr(mr: *mut smbdirect_mr_io);

    pub fn smbd_debug_proc_show(server: *mut TCP_Server_Info, m: *mut seq_file);
}

#[cfg(not(CONFIG_CIFS_SMB_DIRECT))]
pub unsafe fn cifs_rdma_enabled(_server: *mut TCP_Server_Info) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_CIFS_SMB_DIRECT))]
#[repr(C)]
pub struct smbd_connection {}

#[cfg(not(CONFIG_CIFS_SMB_DIRECT))]
pub unsafe fn smbd_get_connection(
    _server: *mut TCP_Server_Info,
    _dstaddr: *mut sockaddr,
) -> *mut ::core::ffi::c_void {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_CIFS_SMB_DIRECT))]
pub unsafe fn smbd_reconnect(_server: *mut TCP_Server_Info) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(CONFIG_CIFS_SMB_DIRECT))]
pub unsafe fn smbd_destroy(_server: *mut TCP_Server_Info) {}

#[cfg(not(CONFIG_CIFS_SMB_DIRECT))]
pub unsafe fn smbd_recv(
    _info: *mut smbd_connection,
    _msg: *mut msghdr,
) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(CONFIG_CIFS_SMB_DIRECT))]
pub unsafe fn smbd_send(
    _server: *mut TCP_Server_Info,
    _num_rqst: ::core::ffi::c_int,
    _rqst: *mut smb_rqst,
) -> ::core::ffi::c_int {
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
