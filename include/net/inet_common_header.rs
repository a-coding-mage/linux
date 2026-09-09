/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations corresponding to the Linux networking headers included by the
 * original C header are supplied by other translated units. */

use core::ffi::{c_int, c_uint, c_ulong, c_uchar, c_ushort, c_void};

#[repr(C)]
pub struct proto_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct msghdr { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct page { _private: [u8; 0] }
#[repr(C)]
pub struct socket { _private: [u8; 0] }
#[repr(C)]
pub struct sockaddr_unsized { _private: [u8; 0] }
#[repr(C)]
pub struct sockaddr { _private: [u8; 0] }
#[repr(C)]
pub struct proto_accept_arg { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }

/* The translated inline function accesses this member as in the C definition. */
#[repr(C)]
pub struct sock {
    pub sk_socket: *mut socket,
}

pub type size_t = usize;
pub type u32 = core::ffi::c_uint;
pub type netdev_features_t = u64;

extern "C" {
    pub static inet_stream_ops: proto_ops;
    pub static inet_dgram_ops: proto_ops;

    pub fn inet_release(sock: *mut socket) -> c_int;
    pub fn inet_stream_connect(sock: *mut socket, uaddr: *mut sockaddr_unsized,
                                addr_len: c_int, flags: c_int) -> c_int;
    pub fn __inet_stream_connect(sock: *mut socket, uaddr: *mut sockaddr_unsized,
                                 addr_len: c_int, flags: c_int,
                                 is_sendmsg: c_int) -> c_int;
    pub fn inet_dgram_connect(sock: *mut socket, uaddr: *mut sockaddr_unsized,
                              addr_len: c_int, flags: c_int) -> c_int;
    pub fn inet_accept(sock: *mut socket, newsock: *mut socket,
                       arg: *mut proto_accept_arg) -> c_int;
    pub fn __inet_accept(sock: *mut socket, newsock: *mut socket,
                         newsk: *mut sock);
    pub fn inet_send_prepare(sk: *mut sock) -> c_int;
    pub fn inet_sendmsg(sock: *mut socket, msg: *mut msghdr, size: size_t) -> c_int;
    pub fn inet_splice_eof(sock: *mut socket);
    pub fn inet_recvmsg(sock: *mut socket, msg: *mut msghdr, size: size_t,
                        flags: c_int) -> c_int;
    pub fn inet_shutdown(sock: *mut socket, how: c_int) -> c_int;
    pub fn inet_listen(sock: *mut socket, backlog: c_int) -> c_int;
    pub fn __inet_listen_sk(sk: *mut sock, backlog: c_int) -> c_int;
    pub fn inet_sock_destruct(sk: *mut sock);
    pub fn inet_bind(sock: *mut socket, uaddr: *mut sockaddr_unsized,
                     addr_len: c_int) -> c_int;
    pub fn inet_bind_sk(sk: *mut sock, uaddr: *mut sockaddr_unsized,
                        addr_len: c_int) -> c_int;
    pub fn __inet_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized,
                       addr_len: c_int, flags: u32) -> c_int;
    pub fn inet_getname(sock: *mut socket, uaddr: *mut sockaddr,
                        peer: c_int) -> c_int;
    pub fn inet_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int;
    pub fn inet_ctl_sock_create(sk: *mut *mut sock, family: c_ushort,
                                type_: c_ushort, protocol: c_uchar,
                                net: *mut net) -> c_int;
    pub fn inet_recv_error(sk: *mut sock, msg: *mut msghdr, len: c_int) -> c_int;
    pub fn inet_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn inet_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int;
    pub fn inet_gso_segment(skb: *mut sk_buff,
                            features: netdev_features_t) -> *mut sk_buff;
    pub fn sock_release(sock: *mut socket);
}

/* Don't allocate port at this moment, defer to connect. */
pub const BIND_FORCE_ADDRESS_NO_PORT: u32 = 1 << 0;
/* Grab and release socket lock. */
pub const BIND_WITH_LOCK: u32 = 1 << 1;
/* Called from BPF program. */
pub const BIND_FROM_BPF: u32 = 1 << 2;
/* Skip CAP_NET_BIND_SERVICE check. */
pub const BIND_NO_CAP_NET_BIND_SERVICE: u32 = 1 << 3;

#[inline]
pub unsafe fn inet_ctl_sock_destroy(sk: *mut sock) {
    if !sk.is_null() {
        sock_release((*sk).sk_socket);
    }
}

/* The C macro depends on INDIRECT_CALL_2, unlikely, gro_recursion_inc_test,
 * NAPI_GRO_CB, and the callback's surrounding translation unit. */
#[macro_export]
macro_rules! indirect_call_gro_receive {
    ($f2:expr, $f1:expr, $cb:expr, $head:expr, $skb:expr) => {{
        if unsafe { $crate::gro_recursion_inc_test($skb) } {
            unsafe { $crate::napi_gro_cb($skb) }.flush |= 1;
            core::ptr::null_mut()
        } else {
            unsafe { $cb($head, $skb) }
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
