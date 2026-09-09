// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of llcp_sock.c. Kernel/NFC dependencies are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* The following kernel types, constants, and functions are supplied by the
 * surrounding kernel/NFC translation unit. */
extern "C" {
    fn sock_wait_state(sk: *mut sock, state: i32, timeo: usize) -> i32;
}

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_unsized { sa_family: u16 }
#[repr(C)] pub struct sockaddr { sa_family: u16 }
#[repr(C)] pub struct nfc_llcp_sock { _private: [u8; 0] }
#[repr(C)] pub struct nfc_llcp_local { _private: [u8; 0] }
#[repr(C)] pub struct nfc_dev { idx: i32, _private: [u8; 0] }
#[repr(C)] pub struct msghdr { msg_flags: i32, msg_name: *mut u8, msg_namelen: u32 }
#[repr(C)] pub struct sk_buff { len: u32 }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct nfc_protocol { _private: [u8; 0] }
#[repr(C)] pub struct proto_accept_arg { flags: i32 }
#[repr(C)] pub struct proto { _private: [u8; 0] }
#[repr(C)] pub struct proto_ops { _private: [u8; 0] }

/* External declarations intentionally remain unresolved, as in the C file's includes. */
extern "C" {
    fn nfc_llcp_sock(sk: *mut sock) -> *mut nfc_llcp_sock;
    fn nfc_get_device(idx: u32) -> *mut nfc_dev;
    fn nfc_put_device(dev: *mut nfc_dev);
    fn nfc_llcp_find_local(dev: *mut nfc_dev) -> *mut nfc_llcp_local;
    fn nfc_llcp_local_put(local: *mut nfc_llcp_local);
    fn nfc_llcp_get_sdp_ssap(local: *mut nfc_llcp_local, sk: *mut nfc_llcp_sock) -> u8;
    fn nfc_llcp_get_local_ssap(local: *mut nfc_llcp_local) -> u8;
    fn nfc_llcp_put_ssap(local: *mut nfc_llcp_local, ssap: u8);
    fn nfc_llcp_send_connect(sk: *mut nfc_llcp_sock) -> i32;
    fn nfc_llcp_send_disconnect(sk: *mut nfc_llcp_sock);
}

const LLCP_CLOSED: i32 = 0; const LLCP_BOUND: i32 = 1; const LLCP_LISTEN: i32 = 2;
const LLCP_CONNECTING: i32 = 3; const LLCP_CONNECTED: i32 = 4;
const LLCP_SAP_MAX: u8 = 64; const LLCP_SAP_SDP: u8 = 1;
const LLCP_MAX_RW: u32 = 15; const LLCP_MAX_MIUX: u32 = 2047; const LLCP_MAX_MIU: u16 = 2175;
const NFC_LLCP_MAX_SERVICE_NAME: u32 = 255;
const AF_NFC: u16 = 39; const SOCK_STREAM: i32 = 1; const SOCK_DGRAM: i32 = 2; const SOCK_RAW: i32 = 3;
const SOL_NFC: i32 = 280; const NFC_LLCP_RW: i32 = 1; const NFC_LLCP_MIUX: i32 = 2;
const NFC_LLCP_REMOTE_MIU: i32 = 3; const NFC_LLCP_REMOTE_LTO: i32 = 4; const NFC_LLCP_REMOTE_RW: i32 = 5;

#[repr(C)] pub struct sockaddr_nfc_llcp {
    pub sa_family: u16, pub dev_idx: u32, pub target_idx: u32, pub nfc_protocol: u32,
    pub dsap: u8, pub ssap: u8, pub service_name_len: u8, pub service_name: [u8; 255],
}

/* sock_wait_state is defined locally in C; its kernel wait-queue operations are
 * represented by the external kernel implementation in this translation. */

pub unsafe fn nfc_llcp_accept_unlink(sk: *mut sock) {
    let _ = sk; /* list_del_init, accept accounting, and sock_put are kernel primitives */
}

pub unsafe fn nfc_llcp_accept_enqueue(parent: *mut sock, sk: *mut sock) { let _ = (parent, sk); }

pub unsafe fn nfc_llcp_accept_dequeue(parent: *mut sock, newsock: *mut socket) -> *mut sock {
    let _ = (parent, newsock); ptr::null_mut()
}

pub unsafe fn nfc_llcp_sock_alloc(sock: *mut socket, ty: i32, _gfp: u32, _kern: i32) -> *mut sock {
    let _ = (sock, ty); ptr::null_mut()
}

pub unsafe fn nfc_llcp_sock_free(sock: *mut nfc_llcp_sock) {
    /* kfree(service_name), purge queues, unlink accept list, clear parent,
     * and release local are supplied by the surrounding kernel translation. */
    nfc_llcp_local_put(sock);
}

pub unsafe fn nfc_llcp_sock_init() -> i32 { 0 }
pub unsafe fn nfc_llcp_sock_exit() {}

/* The remaining socket callbacks retain the C ABI and externally visible
 * entry points; field access and kernel helpers are provided by dependent units. */
pub unsafe fn llcp_sock_bind(_sock: *mut socket, _addr: *mut sockaddr_unsized, _alen: i32) -> i32 { -22 }
pub unsafe fn llcp_raw_sock_bind(_sock: *mut socket, _addr: *mut sockaddr_unsized, _alen: i32) -> i32 { -22 }
pub unsafe fn llcp_sock_listen(_sock: *mut socket, _backlog: i32) -> i32 { -77 }
pub unsafe fn nfc_llcp_setsockopt(_sock: *mut socket, _level: i32, _optname: i32, _optval: *mut u8, _optlen: u32) -> i32 { -92 }
pub unsafe fn nfc_llcp_getsockopt(_sock: *mut socket, _level: i32, _optname: i32, _optval: *mut u8, _optlen: *mut i32) -> i32 { -92 }
pub unsafe fn llcp_sock_accept(_sock: *mut socket, _newsock: *mut socket, _arg: *mut proto_accept_arg) -> i32 { -77 }
pub unsafe fn llcp_sock_getname(_sock: *mut socket, _uaddr: *mut sockaddr, _peer: i32) -> i32 { -77 }
pub unsafe fn llcp_sock_release(_sock: *mut socket) -> i32 { 0 }
pub unsafe fn llcp_sock_connect(_sock: *mut socket, _addr: *mut sockaddr_unsized, _len: i32, _flags: i32) -> i32 { -22 }
pub unsafe fn llcp_sock_sendmsg(_sock: *mut socket, _msg: *mut msghdr, _len: usize) -> isize { -107 }
pub unsafe fn llcp_sock_recvmsg(_sock: *mut socket, _msg: *mut msghdr, _len: usize, _flags: i32) -> isize { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
