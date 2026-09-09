/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2025, Stefan Metzmacher
 */

// Translated from the Linux SMB-DIRECT header.  External kernel/RDMA types
// and functions are supplied by other dependencies.

#[repr(C, packed)]
pub struct smbdirect_buffer_descriptor_v1 {
    pub offset: __le64,
    pub token: __le32,
    pub length: __le32,
}

#[repr(C, packed)]
pub struct smbdirect_socket_parameters {
    pub flags: __u64,
    pub resolve_addr_timeout_msec: __u32,
    pub resolve_route_timeout_msec: __u32,
    pub rdma_connect_timeout_msec: __u32,
    pub negotiate_timeout_msec: __u32,
    pub initiator_depth: __u16,     /* limited to U8_MAX */
    pub responder_resources: __u16, /* limited to U8_MAX */
    pub recv_credit_max: __u16,
    pub send_credit_target: __u16,
    pub max_send_size: __u32,
    pub max_fragmented_send_size: __u32,
    pub max_recv_size: __u32,
    pub max_fragmented_recv_size: __u32,
    pub max_read_write_size: __u32,
    pub max_frmr_depth: __u32,
    pub keepalive_interval_msec: __u32,
    pub keepalive_timeout_msec: __u32,
}

pub const SMBDIRECT_FLAG_PORT_RANGE_ONLY_IB: __u64 = 0x1;
pub const SMBDIRECT_FLAG_PORT_RANGE_ONLY_IW: __u64 = 0x2;
pub const SMBDIRECT_FLAG_PORT_RANGE_MASK: __u64 =
    SMBDIRECT_FLAG_PORT_RANGE_ONLY_IB | SMBDIRECT_FLAG_PORT_RANGE_ONLY_IW;

#[repr(C)]
pub struct smbdirect_socket {
    _private: [u8; 0],
}
#[repr(C)]
pub struct smbdirect_send_batch {
    _private: [u8; 0],
}
#[repr(C)]
pub struct smbdirect_mr_io {
    _private: [u8; 0],
}

pub const SMBDIRECT_LOG_ERR: u32 = 0x0;
pub const SMBDIRECT_LOG_INFO: u32 = 0x1;
pub const SMBDIRECT_LOG_OUTGOING: u32 = 0x1;
pub const SMBDIRECT_LOG_INCOMING: u32 = 0x2;
pub const SMBDIRECT_LOG_READ: u32 = 0x4;
pub const SMBDIRECT_LOG_WRITE: u32 = 0x8;
pub const SMBDIRECT_LOG_RDMA_SEND: u32 = 0x10;
pub const SMBDIRECT_LOG_RDMA_RECV: u32 = 0x20;
pub const SMBDIRECT_LOG_KEEP_ALIVE: u32 = 0x40;
pub const SMBDIRECT_LOG_RDMA_EVENT: u32 = 0x80;
pub const SMBDIRECT_LOG_RDMA_MR: u32 = 0x100;
pub const SMBDIRECT_LOG_RDMA_RW: u32 = 0x200;
pub const SMBDIRECT_LOG_NEGOTIATE: u32 = 0x400;

#[repr(C)]
pub union smbdirect_send_batch_storage {
    pub __msg_list: list_head,
    pub __space: [__aligned_u64; 5],
}

extern "C" {
    pub fn smbdirect_netdev_rdma_capable_node_type(netdev: *mut net_device) -> u8;
    pub fn smbdirect_frwr_is_supported(attrs: *const ib_device_attr) -> bool;
    pub fn smbdirect_socket_create_kern(net: *mut net, _sc: *mut *mut smbdirect_socket) -> i32;
    pub fn smbdirect_socket_create_accepting(id: *mut rdma_cm_id, _sc: *mut *mut smbdirect_socket) -> i32;
    pub fn smbdirect_socket_set_initial_parameters(sc: *mut smbdirect_socket, sp: *const smbdirect_socket_parameters) -> i32;
    pub fn smbdirect_socket_get_current_parameters(sc: *mut smbdirect_socket) -> *const smbdirect_socket_parameters;
    pub fn smbdirect_socket_set_kernel_settings(sc: *mut smbdirect_socket, poll_ctx: ib_poll_context, gfp_mask: gfp_t) -> i32;
    pub fn smbdirect_socket_set_logging(sc: *mut smbdirect_socket, private_ptr: *mut core::ffi::c_void,
        needed: Option<unsafe extern "C" fn(*mut smbdirect_socket, *mut core::ffi::c_void, u32, u32) -> bool>,
        vaprintf: Option<unsafe extern "C" fn(*mut smbdirect_socket, *const core::ffi::c_char, u32, *mut core::ffi::c_void, u32, u32, *mut va_format)>);
    pub fn smbdirect_connection_is_connected(sc: *mut smbdirect_socket) -> bool;
    pub fn smbdirect_connection_wait_for_connected(sc: *mut smbdirect_socket) -> i32;
    pub fn smbdirect_socket_bind(sc: *mut smbdirect_socket, addr: *mut sockaddr) -> i32;
    pub fn smbdirect_socket_shutdown(sc: *mut smbdirect_socket);
    pub fn smbdirect_socket_release(sc: *mut smbdirect_socket);
    pub fn smbdirect_connection_send_batch_flush(sc: *mut smbdirect_socket, batch: *mut smbdirect_send_batch, is_last: bool) -> i32;
    pub fn smbdirect_init_send_batch_storage(storage: *mut smbdirect_send_batch_storage, need_invalidate_rkey: bool, remote_key: u32) -> *mut smbdirect_send_batch;
    pub fn smbdirect_connection_send_single_iter(sc: *mut smbdirect_socket, batch: *mut smbdirect_send_batch, iter: *mut iov_iter, flags: u32, remaining_data_length: u32) -> i32;
    pub fn smbdirect_connection_send_wait_zero_pending(sc: *mut smbdirect_socket) -> i32;
    pub fn smbdirect_connection_send_iter(sc: *mut smbdirect_socket, iter: *mut iov_iter, flags: u32, need_invalidate: bool, remote_key: u32) -> i32;
    pub fn smbdirect_connection_recvmsg(sc: *mut smbdirect_socket, msg: *mut msghdr, flags: u32) -> i32;
    pub fn smbdirect_connect(sc: *mut smbdirect_socket, dst: *const sockaddr) -> i32;
    pub fn smbdirect_connect_sync(sc: *mut smbdirect_socket, dst: *const sockaddr) -> i32;
    pub fn smbdirect_socket_listen(sc: *mut smbdirect_socket, backlog: i32) -> i32;
    pub fn smbdirect_socket_accept(lsc: *mut smbdirect_socket, timeo: i64, arg: *mut proto_accept_arg) -> *mut smbdirect_socket;
    pub fn smbdirect_connection_rdma_xmit(sc: *mut smbdirect_socket, buf: *mut core::ffi::c_void, buf_len: usize, desc: *mut smbdirect_buffer_descriptor_v1, desc_len: usize, is_read: bool) -> i32;
    pub fn smbdirect_connection_register_mr_io(sc: *mut smbdirect_socket, iter: *mut iov_iter, writing: bool, need_invalidate: bool) -> *mut smbdirect_mr_io;
    pub fn smbdirect_mr_io_fill_buffer_descriptor(mr: *mut smbdirect_mr_io, v1: *mut smbdirect_buffer_descriptor_v1);
    pub fn smbdirect_connection_deregister_mr_io(mr: *mut smbdirect_mr_io);
    pub fn smbdirect_connection_legacy_debug_proc_show(sc: *mut smbdirect_socket, rdma_readwrite_threshold: u32, m: *mut seq_file);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
