// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of skmsg.c. Kernel types and helpers are supplied
 * by the surrounding kernel/Rust bindings. */

use core::ffi::c_void;

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_msg { _private: [u8; 0] }
#[repr(C)] pub struct sk_psock { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct strparser { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }

extern "C" {
    fn sk_msg_try_coalesce_ok(msg: *mut sk_msg, first: i32) -> bool;
    fn sk_msg_alloc(sk: *mut sock, msg: *mut sk_msg, len: i32, first: i32) -> i32;
    fn sk_msg_clone(sk: *mut sock, dst: *mut sk_msg, src: *mut sk_msg, off: u32, len: u32) -> i32;
    fn sk_msg_return_zero(sk: *mut sock, msg: *mut sk_msg, bytes: i32);
    fn sk_msg_return(sk: *mut sock, msg: *mut sk_msg, bytes: i32);
    fn sk_msg_free_nocharge(sk: *mut sock, msg: *mut sk_msg) -> i32;
    fn sk_msg_free(sk: *mut sock, msg: *mut sk_msg) -> i32;
    fn sk_msg_free_partial(sk: *mut sock, msg: *mut sk_msg, bytes: u32);
    fn sk_msg_free_partial_nocharge(sk: *mut sock, msg: *mut sk_msg, bytes: u32);
    fn sk_msg_trim(sk: *mut sock, msg: *mut sk_msg, len: i32);
    fn sk_msg_zerocopy_from_iter(sk: *mut sock, from: *mut iov_iter, msg: *mut sk_msg, bytes: u32) -> i32;
    fn sk_msg_memcopy_from_iter(sk: *mut sock, from: *mut iov_iter, msg: *mut sk_msg, bytes: u32) -> i32;
    fn __sk_msg_recvmsg(sk: *mut sock, psock: *mut sk_psock, msg: *mut msghdr, len: i32, flags: i32, self_bytes: *mut i32) -> i32;
    fn sk_msg_recvmsg(sk: *mut sock, psock: *mut sk_psock, msg: *mut msghdr, len: i32, flags: i32) -> i32;
    fn sk_msg_is_readable(sk: *mut sock) -> bool;
    fn sk_psock_init(sk: *mut sock, node: i32) -> *mut sk_psock;
    fn sk_psock_drop(sk: *mut sock, psock: *mut sk_psock);
    fn sk_psock_msg_verdict(sk: *mut sock, psock: *mut sk_psock, msg: *mut sk_msg) -> i32;
    fn sk_psock_start_strp(sk: *mut sock, psock: *mut sk_psock);
    fn sk_psock_stop_strp(sk: *mut sock, psock: *mut sk_psock);
    fn sk_psock_init_strp(sk: *mut sock, psock: *mut sk_psock) -> i32;
    fn sk_psock_start_verdict(sk: *mut sock, psock: *mut sk_psock);
    fn sk_psock_stop_verdict(sk: *mut sock, psock: *mut sk_psock);
}

/* The following declarations retain the C implementation's externally visible
 * interfaces. Their definitions are provided by the kernel translation unit. */
pub unsafe fn sk_msg_alloc_rust(sk: *mut sock, msg: *mut sk_msg, len: i32, first: i32) -> i32 {
    sk_msg_alloc(sk, msg, len, first)
}
pub unsafe fn sk_msg_clone_rust(sk: *mut sock, dst: *mut sk_msg, src: *mut sk_msg, off: u32, len: u32) -> i32 {
    sk_msg_clone(sk, dst, src, off, len)
}
pub unsafe fn sk_msg_return_zero_rust(sk: *mut sock, msg: *mut sk_msg, bytes: i32) { sk_msg_return_zero(sk, msg, bytes); }
pub unsafe fn sk_msg_return_rust(sk: *mut sock, msg: *mut sk_msg, bytes: i32) { sk_msg_return(sk, msg, bytes); }
pub unsafe fn sk_msg_free_nocharge_rust(sk: *mut sock, msg: *mut sk_msg) -> i32 { sk_msg_free_nocharge(sk, msg) }
pub unsafe fn sk_msg_free_rust(sk: *mut sock, msg: *mut sk_msg) -> i32 { sk_msg_free(sk, msg) }
pub unsafe fn sk_msg_free_partial_rust(sk: *mut sock, msg: *mut sk_msg, bytes: u32) { sk_msg_free_partial(sk, msg, bytes); }
pub unsafe fn sk_msg_free_partial_nocharge_rust(sk: *mut sock, msg: *mut sk_msg, bytes: u32) { sk_msg_free_partial_nocharge(sk, msg, bytes); }
pub unsafe fn sk_msg_trim_rust(sk: *mut sock, msg: *mut sk_msg, len: i32) { sk_msg_trim(sk, msg, len); }
pub unsafe fn sk_msg_zerocopy_from_iter_rust(sk: *mut sock, from: *mut iov_iter, msg: *mut sk_msg, bytes: u32) -> i32 { sk_msg_zerocopy_from_iter(sk, from, msg, bytes) }
pub unsafe fn sk_msg_memcopy_from_iter_rust(sk: *mut sock, from: *mut iov_iter, msg: *mut sk_msg, bytes: u32) -> i32 { sk_msg_memcopy_from_iter(sk, from, msg, bytes) }
pub unsafe fn sk_msg_recvmsg_rust(sk: *mut sock, psock: *mut sk_psock, msg: *mut msghdr, len: i32, flags: i32) -> i32 { sk_msg_recvmsg(sk, psock, msg, len, flags) }
pub unsafe fn sk_msg_is_readable_rust(sk: *mut sock) -> bool { sk_msg_is_readable(sk) }
pub unsafe fn sk_psock_init_rust(sk: *mut sock, node: i32) -> *mut sk_psock { sk_psock_init(sk, node) }
pub unsafe fn sk_psock_drop_rust(sk: *mut sock, psock: *mut sk_psock) { sk_psock_drop(sk, psock); }
pub unsafe fn sk_psock_msg_verdict_rust(sk: *mut sock, psock: *mut sk_psock, msg: *mut sk_msg) -> i32 { sk_psock_msg_verdict(sk, psock, msg) }
pub unsafe fn sk_psock_init_strp_rust(sk: *mut sock, psock: *mut sk_psock) -> i32 { sk_psock_init_strp(sk, psock) }
pub unsafe fn sk_psock_start_strp_rust(sk: *mut sock, psock: *mut sk_psock) { sk_psock_start_strp(sk, psock); }
pub unsafe fn sk_psock_stop_strp_rust(sk: *mut sock, psock: *mut sk_psock) { sk_psock_stop_strp(sk, psock); }
pub unsafe fn sk_psock_start_verdict_rust(sk: *mut sock, psock: *mut sk_psock) { sk_psock_start_verdict(sk, psock); }
pub unsafe fn sk_psock_stop_verdict_rust(sk: *mut sock, psock: *mut sk_psock) { sk_psock_stop_verdict(sk, psock); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
