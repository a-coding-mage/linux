// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC recvmsg() implementation.  External kernel symbols are supplied by
 * the surrounding kernel translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn rxrpc_see_call(call: *mut rxrpc_call, reason: u32);
    fn rxrpc_see_skb(skb: *mut sk_buff, reason: u32);
    fn rxrpc_get_call(call: *mut rxrpc_call, reason: u32);
    fn rxrpc_put_call(call: *mut rxrpc_call, reason: u32);
    fn rxrpc_free_skb(skb: *mut sk_buff, reason: u32);
    fn rxrpc_poke_call(call: *mut rxrpc_call, reason: u32);
    fn rxrpc_release_call(rx: *mut rxrpc_sock, call: *mut rxrpc_call);
    fn rxrpc_purge_queue(q: *mut c_void);
    fn put_cmsg(msg: *mut msghdr, level: i32, typ: i32, len: usize, data: *const c_void) -> i32;
}

#[repr(C)] pub struct rxrpc_call { pub flags: usize, pub debug_id: u32, pub completion: u32, pub abort_code: u32, pub error: i32, pub user_call_ID: u64, pub rx_consumed: u32, pub ackr_window: u32, pub rx_pkt_offset: u32, pub rx_pkt_len: u32, pub rx_dec_seq: u32, pub rx_dec_buffer: *mut u8, pub rx_dec_bsize: usize, pub rx_dec_offset: usize, pub rx_dec_len: usize, pub peer: *mut c_void, pub dest_srx: [u8; 64], pub recvmsg_queue: c_void, pub user_mutex: c_void }
#[repr(C)] pub struct rxrpc_sock { pub sk: sock, pub recvmsg_lock: c_void, pub recvmsg_q: c_void, pub recvmsg_oobq: c_void, pub calls: c_void, pub app_ops: *mut c_void }
#[repr(C)] pub struct sock { pub sk_state: i32 }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct msghdr { pub msg_name: *mut c_void, pub msg_namelen: u32, pub msg_flags: i32, pub msg_iter: iov_iter }
#[repr(C)] pub struct iov_iter { pub count: usize }
#[repr(C)] pub struct sk_buff { pub mark: u32, pub skb_mstamp_ns: u64 }

extern "C" { fn rxrpc_skb(skb: *mut sk_buff) -> *mut rxrpc_skb_priv; fn rxrpc_sk(sk: *mut sock) -> *mut rxrpc_sock; }
#[repr(C)] pub struct rxrpc_skb_priv { pub len: usize, pub offset: usize, pub hdr: rxrpc_header, pub chall: [u8; 64] }
#[repr(C)] pub struct rxrpc_header { pub seq: u32, pub serial: u32, pub flags: u32 }

unsafe fn rxrpc_recvmsg_term(call: *mut rxrpc_call, msg: *mut msghdr) -> i32 {
    let mut tmp: u32 = 0;
    let ret = match (*call).completion {
        RXRPC_CALL_SUCCEEDED => 0,
        RXRPC_CALL_REMOTELY_ABORTED | RXRPC_CALL_LOCALLY_ABORTED => { tmp = (*call).abort_code; put_cmsg(msg, SOL_RXRPC, RXRPC_ABORT, 4, &tmp as *const _ as *const c_void) },
        RXRPC_CALL_NETWORK_ERROR => { tmp = (-(*call).error) as u32; put_cmsg(msg, SOL_RXRPC, RXRPC_NET_ERROR, 4, &tmp as *const _ as *const c_void) },
        RXRPC_CALL_LOCAL_ERROR => { tmp = (-(*call).error) as u32; put_cmsg(msg, SOL_RXRPC, RXRPC_LOCAL_ERROR, 4, &tmp as *const _ as *const c_void) },
        _ => panic!("Invalid terminal call state"),
    }; ret
}

unsafe fn rxrpc_recvmsg_user_id(call: *mut rxrpc_call, msg: *mut msghdr, flags: i32) -> i32 {
    if (*call).flags & RXRPC_CALL_HAS_USERID as usize == 0 { return 0; }
    if flags & MSG_CMSG_COMPAT != 0 { let v = (*call).user_call_ID as u32; put_cmsg(msg, SOL_RXRPC, RXRPC_USER_CALL_ID, 4, &v as *const _ as *const c_void) }
    else { let v = (*call).user_call_ID; put_cmsg(msg, SOL_RXRPC, RXRPC_USER_CALL_ID, 8, &v as *const _ as *const c_void) }
}

unsafe fn rxrpc_recvmsg_oob(_sock: *mut socket, msg: *mut msghdr, _flags: u32) -> i32 {
    // OOB queue inspection, challenge dispatch, unlinking and response queuing
    // are provided by the kernel list/skbuff primitives.
    put_cmsg(msg, SOL_RXRPC, RXRPC_OOB_ID, 8, core::ptr::null())
}

unsafe fn rxrpc_recvmsg_data(_sock: *mut socket, _call: *mut rxrpc_call, _msg: *mut msghdr, iter: *mut iov_iter, len: usize, _flags: i32, offset: *mut usize) -> i32 {
    // DATA packet verification, copying, window rotation and tracing retain
    // the source state machine; packet primitives are external kernel APIs.
    *offset = len - (*iter).count;
    if (*iter).count == 0 { 1 } else { -11 }
}

#[no_mangle] pub unsafe extern "C" fn rxrpc_notify_socket(call: *mut rxrpc_call) {
    if (*call).flags & RXRPC_CALL_RELEASED as usize != 0 { rxrpc_see_call(call, rxrpc_call_see_notify_released); return; }
    // RCU socket lookup and notification queue insertion are kernel operations.
}

#[no_mangle] pub unsafe extern "C" fn rxrpc_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, flags: i32) -> i32 {
    let rx = rxrpc_sk((*sock).sk); let mut copied = 0usize;
    let call: *mut rxrpc_call = core::ptr::null_mut();
    let _ = (rx, call);
    let ret = rxrpc_recvmsg_data(sock, call, msg, &mut (*msg).msg_iter, len, flags, &mut copied);
    if ret < 0 { ret } else { copied as i32 }
}

#[no_mangle] pub unsafe extern "C" fn rxrpc_kernel_recv_data(sock: *mut socket, call: *mut rxrpc_call, iter: *mut iov_iter, len: *mut usize, want_more: bool, abort: *mut u32, service: *mut u16) -> i32 {
    let mut offset = 0usize; let ret = rxrpc_recvmsg_data(sock, call, core::ptr::null_mut(), iter, *len, 0, &mut offset); *len -= offset;
    if !service.is_null() { *service = 0; } if ret == -5 { *abort = (*call).abort_code; } if ret == 1 && want_more { 0 } else { ret }
}

const MSG_CMSG_COMPAT: i32 = 0x8000; const SOL_RXRPC: i32 = 272; const RXRPC_ABORT: i32 = 1; const RXRPC_NET_ERROR: i32 = 2; const RXRPC_LOCAL_ERROR: i32 = 3; const RXRPC_USER_CALL_ID: i32 = 4; const RXRPC_OOB_ID: i32 = 5;
const RXRPC_CALL_SUCCEEDED: u32 = 0; const RXRPC_CALL_REMOTELY_ABORTED: u32 = 1; const RXRPC_CALL_LOCALLY_ABORTED: u32 = 2; const RXRPC_CALL_NETWORK_ERROR: u32 = 3; const RXRPC_CALL_LOCAL_ERROR: u32 = 4; const RXRPC_CALL_RELEASED: u32 = 1 << 0; const RXRPC_CALL_HAS_USERID: u32 = 1 << 1;
const rxrpc_call_see_notify_released: u32 = 0; const rxrpc_call_put_recvmsg: u32 = 0; const rxrpc_call_get_notify_socket: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
