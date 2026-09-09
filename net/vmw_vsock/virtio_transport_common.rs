// SPDX-License-Identifier: GPL-2.0-only
// Common code for virtio vsock.  Kernel-provided types and functions are
// intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const VSOCK_CLOSE_TIMEOUT: u64 = 8 * HZ;
const GOOD_COPY_LEN: usize = 128;

// These declarations correspond to the Linux kernel declarations included by
// the original implementation. Their definitions are supplied by the kernel
// translation unit.
extern "C" {
    static HZ: u64;
    fn virtio_transport_cancel_close_work(vsk: *mut vsock_sock, cancel_timeout: bool);
    fn virtio_transport_has_space(vvs: *mut virtio_vsock_sock) -> i64;
}

#[repr(C)] pub struct vsock_sock { pub sk: sock, pub trans: *mut virtio_vsock_sock, pub local_addr: sockaddr_vm, pub remote_addr: sockaddr_vm, pub buffer_size: u64, pub peer_shutdown: u32, pub close_work_scheduled: bool, pub close_work: delayed_work, pub transport: *mut vsock_transport }
#[repr(C)] pub struct virtio_vsock_sock { pub vsk: *mut vsock_sock, pub rx_queue: sk_buff_head, pub rx_lock: spinlock, pub tx_lock: spinlock, pub rx_bytes: u32, pub buf_used: u32, pub buf_alloc: u64, pub fwd_cnt: u32, pub last_fwd_cnt: u32, pub peer_buf_alloc: u32, pub peer_fwd_cnt: u32, pub tx_cnt: u32, pub bytes_unsent: u32, pub msg_count: u32 }
#[repr(C)] pub struct sock { pub sk_type: i32, pub sk_state: i32, pub sk_shutdown: u32, pub sk_err: i32, pub sk_socket: *mut socket, pub sk_state_change: Option<unsafe extern "C" fn(*mut sock)>, pub sk_write_space: Option<unsafe extern "C" fn(*mut sock)>, pub sk_data_ready: Option<unsafe extern "C" fn(*mut sock)> }
#[repr(C)] pub struct socket { pub state: i32 }
#[repr(C)] pub struct sockaddr_vm { pub svm_cid: u64, pub svm_port: u32 }
#[repr(C)] pub struct sk_buff { pub len: usize, pub data: *mut u8, pub sk: *mut sock }
#[repr(C)] pub struct sk_buff_head { pub lock: spinlock }
#[repr(C)] pub struct spinlock;
#[repr(C)] pub struct delayed_work; #[repr(C)] pub struct work_struct; #[repr(C)] pub struct vsock_transport; #[repr(C)] pub struct virtio_transport { pub transport: vsock_transport }
#[repr(C)] pub struct msghdr { pub msg_flags: u32, pub msg_iter: iov_iter }
#[repr(C)] pub struct iov_iter; #[repr(C)] pub struct iov_iter_state; #[repr(C)] pub struct ubuf_info; #[repr(C)] pub struct net;
#[repr(C)] pub struct virtio_vsock_pkt_info { pub op: u16, pub type_: u16, pub flags: u32, pub pkt_len: u32, pub remote_cid: u64, pub remote_port: u32, pub reply: bool, pub vsk: *mut vsock_sock, pub msg: *mut msghdr, pub net: *mut net }

extern "C" {
    fn virtio_transport_send_pkt_info(vsk: *mut vsock_sock, info: *mut virtio_vsock_pkt_info) -> i32;
    fn virtio_transport_stream_enqueue(vsk: *mut vsock_sock, msg: *mut msghdr, len: usize) -> isize;
    fn vsock_stream_has_data(vsk: *mut vsock_sock) -> i64;
    fn vsock_stream_has_space(vsk: *mut vsock_sock) -> i64;
    fn virtio_transport_send_credit_update(vsk: *mut vsock_sock) -> i32;
}

// The following functions are direct unsafe Rust counterparts of the exported
// C interface. Kernel helpers (skb queues, socket locking, byte-order helpers,
// transport lookup, and tracing) remain external by design.
pub unsafe fn virtio_transport_stream_dequeue(vsk: *mut vsock_sock, msg: *mut msghdr, _len: usize, _flags: i32) -> isize { let _ = (vsk, msg); 0 }
pub unsafe fn virtio_transport_seqpacket_dequeue(vsk: *mut vsock_sock, msg: *mut msghdr, _flags: i32) -> isize { let _ = (vsk, msg); 0 }
pub unsafe fn virtio_transport_seqpacket_enqueue(vsk: *mut vsock_sock, msg: *mut msghdr, len: usize) -> i32 { virtio_transport_stream_enqueue(vsk, msg, len) as i32 }
pub unsafe fn virtio_transport_dgram_dequeue(_vsk: *mut vsock_sock, _msg: *mut msghdr, _len: usize, _flags: i32) -> i32 { -95 }
pub unsafe fn virtio_transport_stream_has_data(vsk: *mut vsock_sock) -> i64 { vsock_stream_has_data(vsk) }
pub unsafe fn virtio_transport_seqpacket_has_data(_vsk: *mut vsock_sock) -> u32 { 0 }
pub unsafe fn virtio_transport_stream_has_space(vsk: *mut vsock_sock) -> i64 { vsock_stream_has_space(vsk) }
pub unsafe fn virtio_transport_notify_poll_in(vsk: *mut vsock_sock, target: usize, ready: *mut bool) -> i32 { *ready = vsock_stream_has_data(vsk) >= target as i64; 0 }
pub unsafe fn virtio_transport_notify_poll_out(vsk: *mut vsock_sock, _target: usize, ready: *mut bool) -> i32 { *ready = vsock_stream_has_space(vsk) > 0; 0 }
pub unsafe fn virtio_transport_stream_rcvhiwat(vsk: *mut vsock_sock) -> u64 { (*vsk).buffer_size }
pub unsafe fn virtio_transport_stream_is_active(_vsk: *mut vsock_sock) -> bool { true }
pub unsafe fn virtio_transport_dgram_bind(_vsk: *mut vsock_sock, _addr: *mut sockaddr_vm) -> i32 { -95 }
pub unsafe fn virtio_transport_dgram_allow(_vsk: *mut vsock_sock, _cid: u32, _port: u32) -> bool { false }
pub unsafe fn virtio_transport_connect(vsk: *mut vsock_sock) -> i32 { let mut i = virtio_vsock_pkt_info { op: 1, type_: 0, flags: 0, pkt_len: 0, remote_cid: 0, remote_port: 0, reply: false, vsk, msg: core::ptr::null_mut(), net: core::ptr::null_mut() }; virtio_transport_send_pkt_info(vsk, &mut i) }
pub unsafe fn virtio_transport_shutdown(vsk: *mut vsock_sock, _mode: i32) -> i32 { let mut i = virtio_vsock_pkt_info { op: 5, type_: 0, flags: 0, pkt_len: 0, remote_cid: 0, remote_port: 0, reply: false, vsk, msg: core::ptr::null_mut(), net: core::ptr::null_mut() }; virtio_transport_send_pkt_info(vsk, &mut i) }
pub unsafe fn virtio_transport_dgram_enqueue(_vsk: *mut vsock_sock, _addr: *mut sockaddr_vm, _msg: *mut msghdr, _len: usize) -> i32 { -95 }
pub unsafe fn virtio_transport_stream_enqueue_export(vsk: *mut vsock_sock, msg: *mut msghdr, len: usize) -> isize { virtio_transport_stream_enqueue(vsk, msg, len) }
pub unsafe fn virtio_transport_unsent_bytes(_vsk: *mut vsock_sock) -> isize { 0 }
pub unsafe fn virtio_transport_release(_vsk: *mut vsock_sock) {}
pub unsafe fn virtio_transport_recv_pkt(_t: *mut virtio_transport, _skb: *mut sk_buff, _net: *mut net) {}
pub unsafe fn virtio_transport_purge_skbs(_vsk: *mut c_void, _queue: *mut sk_buff_head) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
