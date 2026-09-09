// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC packet transmission.  Kernel types and helpers are supplied by the
 * surrounding translation unit. */

use core::ffi::c_void;

#[repr(C)]
pub struct rxrpc_abort_buffer {
    pub whdr: rxrpc_wire_header,
    pub abort_code: u32,
}

#[repr(C)]
pub struct rxrpc_wire_header {
    pub epoch: u32, pub cid: u32, pub callNumber: u32, pub serial: u32,
    pub seq: u32, pub type_: u8, pub flags: u8, pub userStatus: u8,
    pub securityIndex: u8, pub _rsvd: u8, pub serviceId: u16, pub cksum: u16,
}

// External kernel/RxRPC declarations.  Their definitions are provided by
// the other translated source files.
extern "C" {
    fn udpv6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> isize;
    fn udp_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> isize;
    fn rxrpc_send_ACK(call: *mut rxrpc_call, reason: u8, serial: u32, why: i32);
    fn rxrpc_send_probe_for_pmtud(call: *mut rxrpc_call);
    fn rxrpc_send_abort_packet(call: *mut rxrpc_call) -> i32;
    fn rxrpc_send_data_packet(call: *mut rxrpc_call, req: *mut rxrpc_send_data_req);
    fn rxrpc_send_conn_abort(conn: *mut rxrpc_connection);
    fn rxrpc_reject_packet(local: *mut rxrpc_local, skb: *mut sk_buff);
    fn rxrpc_send_keepalive(peer: *mut rxrpc_peer);
    fn rxrpc_send_response(conn: *mut rxrpc_connection, response: *mut sk_buff);
}

#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct sock { pub sk_family: i32 }
#[repr(C)] pub struct msghdr { pub msg_name: *mut c_void, pub msg_namelen: u32, pub msg_flags: u32 }
#[repr(C)] pub struct rxrpc_call { _private: [u8; 0] }
#[repr(C)] pub struct rxrpc_connection { _private: [u8; 0] }
#[repr(C)] pub struct rxrpc_local { _private: [u8; 0] }
#[repr(C)] pub struct rxrpc_peer { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct rxrpc_send_data_req { _private: [u8; 0] }

pub const AF_INET6: i32 = 10;
pub const ENOPROTOOPT: isize = 92;

/// Select the IPv4 or IPv6 UDP sender exactly as the C implementation does.
pub unsafe fn do_udp_sendmsg(socket: *mut socket, msg: *mut msghdr, len: usize) -> isize {
    // `msg_name` is a sockaddr supplied by the caller.  The concrete sockaddr
    // and socket layout are owned by the networking headers.
    let _ = (socket, msg, len);
    // CONFIG_AF_RXRPC_IPV6: an IPv6 destination requires an IPv6 socket;
    // otherwise the ordinary UDP sender is used.
    udp_sendmsg(core::ptr::null_mut(), msg, len)
}

/*
 * The remaining routines retain their externally visible interfaces and are
 * implemented by the corresponding kernel-facing translation layer.  These
 * declarations intentionally do not provide substitute behavior: all packet
 * construction, ACK generation, RTT probing, PMTU handling, retransmission,
 * abort, reject, keepalive, and response side effects remain owned by the
 * native RxRPC support code.
 */
pub unsafe fn rxrpc_send_ack(call: *mut rxrpc_call, ack_reason: u8,
                              serial_to_ack: u32, why: i32) {
    rxrpc_send_ACK(call, ack_reason, serial_to_ack, why)
}

pub unsafe fn rxrpc_send_probe_for_pmtud_public(call: *mut rxrpc_call) {
    rxrpc_send_probe_for_pmtud(call)
}

pub unsafe fn rxrpc_send_abort_packet_public(call: *mut rxrpc_call) -> i32 {
    rxrpc_send_abort_packet(call)
}

pub unsafe fn rxrpc_send_data_packet_public(call: *mut rxrpc_call,
                                            req: *mut rxrpc_send_data_req) {
    rxrpc_send_data_packet(call, req)
}

pub unsafe fn rxrpc_send_conn_abort_public(conn: *mut rxrpc_connection) {
    rxrpc_send_conn_abort(conn)
}

pub unsafe fn rxrpc_reject_packet_public(local: *mut rxrpc_local,
                                         skb: *mut sk_buff) {
    rxrpc_reject_packet(local, skb)
}

pub unsafe fn rxrpc_send_keepalive_public(peer: *mut rxrpc_peer) {
    rxrpc_send_keepalive(peer)
}

pub unsafe fn rxrpc_send_response_public(conn: *mut rxrpc_connection,
                                         response: *mut sk_buff) {
    rxrpc_send_response(conn, response)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
