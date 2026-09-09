/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/include/linux/sunrpc/xprtsock.h
 *
 *  Declarations for the RPC transport socket provider.
 */

// C header guard: _LINUX_SUNRPC_XPRTSOCK_H

extern "C" {
    pub fn init_socket_xprt() -> ::core::ffi::c_int;
    pub fn cleanup_socket_xprt();
}

pub const RPC_MIN_RESVPORT: ::core::ffi::c_uint = 1u32;
pub const RPC_MAX_RESVPORT: ::core::ffi::c_uint = 65535u32;
pub const RPC_DEF_MIN_RESVPORT: ::core::ffi::c_uint = 665u32;
pub const RPC_DEF_MAX_RESVPORT: ::core::ffi::c_uint = 1023u32;

#[repr(C)]
pub struct SockXprt {
    pub xprt: rpc_xprt,

    /*
     * Network layer
     */
    pub sock: *mut socket,
    pub inet: *mut sock,
    pub file: *mut file,

    /*
     * State of TCP reply receive
     */
    pub recv: SockXprtRecv,

    /*
     * State of TCP transmit queue
     */
    pub xmit: SockXprtXmit,

    /*
     * Connection of transports
     */
    pub sock_state: ::core::ffi::c_ulong,
    pub connect_worker: delayed_work,
    pub error_worker: work_struct,
    pub recv_worker: work_struct,
    pub recv_mutex: mutex,
    pub handshake_done: completion,
    pub srcaddr: sockaddr_storage,
    pub srcport: u16,
    pub xprt_err: ::core::ffi::c_int,
    pub clnt: *mut rpc_clnt,

    /*
     * UDP socket buffer size parameters
     */
    pub rcvsize: usize,
    pub sndsize: usize,

    pub tcp_timeout: rpc_timeout,

    /*
     * Saved socket callback addresses
     */
    pub old_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    pub old_state_change: Option<unsafe extern "C" fn(*mut sock)>,
    pub old_write_space: Option<unsafe extern "C" fn(*mut sock)>,
    pub old_error_report: Option<unsafe extern "C" fn(*mut sock)>,
}

#[repr(C)]
pub struct SockXprtRecv {
    pub fraghdr: __be32,
    pub xid: __be32,
    pub calldir: __be32,
    pub offset: u32,
    pub len: u32,
    pub copied: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct SockXprtXmit {
    pub offset: u32,
}

/*
 * TCP RPC flags
 */
pub const XPRT_SOCK_CONNECTING: ::core::ffi::c_uint = 1u32;
pub const XPRT_SOCK_DATA_READY: ::core::ffi::c_uint = 2u32;
pub const XPRT_SOCK_UPD_TIMEOUT: ::core::ffi::c_uint = 3u32;
pub const XPRT_SOCK_WAKE_ERROR: ::core::ffi::c_uint = 4u32;
pub const XPRT_SOCK_WAKE_WRITE: ::core::ffi::c_uint = 5u32;
pub const XPRT_SOCK_WAKE_PENDING: ::core::ffi::c_uint = 6u32;
pub const XPRT_SOCK_WAKE_DISCONNECT: ::core::ffi::c_uint = 7u32;
pub const XPRT_SOCK_CONNECT_SENT: ::core::ffi::c_uint = 8u32;
pub const XPRT_SOCK_NOSPACE: ::core::ffi::c_uint = 9u32;
pub const XPRT_SOCK_IGNORE_RECV: ::core::ffi::c_uint = 10u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
