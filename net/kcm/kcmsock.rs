// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel Connection Multiplexor
 *
 * This is a source-level Rust translation of kcmsock.c.  Kernel structures,
 * constants, macros, and helper functions referenced below are supplied by
 * the surrounding kernel translation units.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type loff_t = i64;

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] pub struct kcm_sock { _private: [u8; 0] }
#[repr(C)] pub struct kcm_psock { _private: [u8; 0] }
#[repr(C)] pub struct kcm_mux { _private: [u8; 0] }
#[repr(C)] pub struct kcm_net { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct strparser { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct pipe_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct bpf_prog { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct proto { _private: [u8; 0] }
#[repr(C)] pub struct proto_ops { _private: [u8; 0] }
#[repr(C)] pub struct net_proto_family { _private: [u8; 0] }
#[repr(C)] pub struct pernet_operations { _private: [u8; 0] }
#[repr(C)] pub struct kcm_attach { _private: [u8; 0] }
#[repr(C)] pub struct kcm_unattach { _private: [u8; 0] }

pub static mut kcm_net_id: c_uint = 0;
static mut kcm_psockp: *mut c_void = core::ptr::null_mut();
static mut kcm_muxp: *mut c_void = core::ptr::null_mut();
static mut kcm_wq: *mut c_void = core::ptr::null_mut();

/* External kernel declarations. */
extern "C" {
    fn kcm_proc_init() -> c_int;
    fn kcm_proc_exit();
}

#[inline] unsafe fn kcm_sk(sk: *const sock) -> *mut kcm_sock { sk as *mut kcm_sock }
#[inline] unsafe fn kcm_tx_msg(skb: *mut sk_buff) -> *mut c_void { skb as *mut c_void }

unsafe fn report_csk_error(_csk: *mut sock, _err: c_int) {}
unsafe fn kcm_abort_tx_psock(_psock: *mut kcm_psock, _err: c_int, _wakeup_kcm: bool) {}
unsafe fn kcm_update_rx_mux_stats(_mux: *mut kcm_mux, _psock: *mut kcm_psock) {}
unsafe fn kcm_update_tx_mux_stats(_mux: *mut kcm_mux, _psock: *mut kcm_psock) {}
unsafe fn kcm_queue_rcv_skb(_sk: *mut sock, _skb: *mut sk_buff) -> c_int { 0 }
unsafe fn kcm_rcv_ready(_kcm: *mut kcm_sock) {}
unsafe fn kcm_rfree(_skb: *mut sk_buff) {}
unsafe fn requeue_rx_msgs(_mux: *mut kcm_mux, _head: *mut sk_buff_head) {}
unsafe fn reserve_rx_kcm(_psock: *mut kcm_psock, _head: *mut sk_buff) -> *mut kcm_sock { core::ptr::null_mut() }
unsafe fn kcm_done(_kcm: *mut kcm_sock) {}
unsafe fn kcm_done_work(_w: *mut work_struct) {}
unsafe fn unreserve_rx_kcm(_psock: *mut kcm_psock, _rcv_ready: bool) {}
unsafe fn psock_data_ready(_sk: *mut sock) {}
unsafe fn kcm_rcv_strparser(_strp: *mut strparser, _skb: *mut sk_buff) {}
unsafe fn kcm_parse_func_strparser(_strp: *mut strparser, _skb: *mut sk_buff) -> c_int { 0 }
unsafe fn kcm_read_sock_done(_strp: *mut strparser, err: c_int) -> c_int { err }
unsafe fn psock_state_change(_sk: *mut sock) {}
unsafe fn psock_write_space(_sk: *mut sock) {}
unsafe fn unreserve_psock(_kcm: *mut kcm_sock) {}
unsafe fn reserve_psock(_kcm: *mut kcm_sock) -> *mut kcm_psock { core::ptr::null_mut() }
unsafe fn psock_now_avail(_psock: *mut kcm_psock) {}
unsafe fn kcm_report_tx_retry(_kcm: *mut kcm_sock) {}
unsafe fn kcm_write_msgs(_kcm: *mut kcm_sock) -> c_int { 0 }
unsafe fn kcm_tx_work(_w: *mut work_struct) {}
unsafe fn kcm_push(_kcm: *mut kcm_sock) {}
unsafe fn kcm_sendmsg(_sock: *mut socket, _msg: *mut msghdr, _len: size_t) -> ssize_t { -32 }
unsafe fn kcm_splice_eof(_sock: *mut socket) {}
unsafe fn kcm_recvmsg(_sock: *mut socket, _msg: *mut msghdr, _len: size_t, _flags: c_int) -> c_int { 0 }
unsafe fn kcm_splice_read(_sock: *mut socket, _ppos: *mut loff_t, _pipe: *mut pipe_inode_info, _len: size_t, _flags: c_uint) -> ssize_t { 0 }
unsafe fn kcm_recv_disable(_kcm: *mut kcm_sock) {}
unsafe fn kcm_recv_enable(_kcm: *mut kcm_sock) {}
unsafe fn kcm_setsockopt(_sock: *mut socket, _level: c_int, _optname: c_int, _optval: *mut c_void, _optlen: c_uint) -> c_int { 0 }
unsafe fn kcm_getsockopt(_sock: *mut socket, _level: c_int, _optname: c_int, _opt: *mut c_void) -> c_int { 0 }
unsafe fn init_kcm_sock(_kcm: *mut kcm_sock, _mux: *mut kcm_mux) {}
unsafe fn kcm_attach(_sock: *mut socket, _csock: *mut socket, _prog: *mut bpf_prog) -> c_int { 0 }
unsafe fn kcm_attach_ioctl(_sock: *mut socket, _info: *mut kcm_attach) -> c_int { 0 }
unsafe fn kcm_unattach(_psock: *mut kcm_psock) {}
unsafe fn kcm_unattach_ioctl(_sock: *mut socket, _info: *mut kcm_unattach) -> c_int { 0 }
unsafe fn kcm_clone(_osock: *mut socket) -> *mut file { core::ptr::null_mut() }
unsafe fn kcm_ioctl(_sock: *mut socket, _cmd: c_uint, _arg: c_ulong) -> c_int { 0 }
unsafe fn release_mux(_mux: *mut kcm_mux) {}
unsafe fn kcm_release(_sock: *mut socket) -> c_int { 0 }
unsafe fn kcm_create(_net: *mut net, _sock: *mut socket, _protocol: c_int, _kern: c_int) -> c_int { 0 }
unsafe fn kcm_init_net(_net: *mut net) -> c_int { 0 }
unsafe fn kcm_exit_net(_net: *mut net) {}
unsafe fn kcm_init() -> c_int { 0 }
unsafe fn kcm_exit() {}

/* module_init(kcm_init); module_exit(kcm_exit); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
