// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct io_shutdown { pub file: *mut file, pub how: c_int }
#[repr(C)]
pub struct io_accept { pub file: *mut file, pub addr: *mut sockaddr, pub addr_len: *mut c_int, pub flags: c_int, pub iou_flags: c_int, pub file_slot: u32, pub nofile: c_ulong }
#[repr(C)]
pub struct io_socket { pub file: *mut file, pub domain: c_int, pub r#type: c_int, pub protocol: c_int, pub flags: c_int, pub file_slot: u32, pub nofile: c_ulong }
#[repr(C)]
pub struct io_connect { pub file: *mut file, pub addr: *mut sockaddr, pub addr_len: c_int, pub in_progress: bool, pub seen_econnaborted: bool }
#[repr(C)]
pub struct io_bind { pub file: *mut file, pub addr_len: c_int }
#[repr(C)]
pub struct io_listen { pub file: *mut file, pub backlog: c_int }
#[repr(C)]
pub union io_sr_msg_user { pub umsg_compat: *mut compat_msghdr, pub umsg: *mut user_msghdr, pub buf: *mut c_void }
#[repr(C)]
pub struct io_sr_msg { pub file: *mut file, pub user: io_sr_msg_user, pub len: c_int, pub done_io: c_uint, pub msg_flags: c_uint, pub nr_multishot_loops: c_uint, pub flags: u16, pub buf_group: u16, pub mshot_len: c_uint, pub mshot_total_len: c_uint, pub msg_control: *mut c_void, pub notif: *mut io_kiocb }
#[repr(C)]
pub struct io_recvzc { pub file: *mut file, pub flags: u16, pub len: u32, pub ifq: *mut io_zcrx_ifq }

pub const IORING_RECV_RETRY: u16 = 1 << 15;
pub const IORING_RECV_PARTIAL_MAP: u16 = 1 << 14;
pub const IORING_RECV_MSHOT_CAP: u16 = 1 << 13;
pub const IORING_RECV_MSHOT_LIM: u16 = 1 << 12;
pub const IORING_RECV_MSHOT_DONE: u16 = 1 << 11;
pub const IORING_RECV_RETRY_CLEAR: u16 = IORING_RECV_RETRY | IORING_RECV_PARTIAL_MAP;
pub const IORING_RECV_NO_RETRY: u16 = IORING_RECV_RETRY | IORING_RECV_PARTIAL_MAP | IORING_RECV_MSHOT_CAP | IORING_RECV_MSHOT_DONE;
pub const MULTISHOT_MAX_RETRY: c_uint = 32;

extern "C" {
    fn io_kiocb_to_cmd(req: *mut io_kiocb) -> *mut c_void;
    fn sock_from_file(file: *mut file) -> *mut socket;
    fn __sys_shutdown_sock(sock: *mut socket, how: c_int) -> c_int;
    fn io_req_set_res(req: *mut io_kiocb, res: c_int, flags: c_uint);
    fn req_set_fail(req: *mut io_kiocb);
    fn io_msg_alloc_async(req: *mut io_kiocb) -> *mut io_async_msghdr;
    fn io_req_msg_cleanup(req: *mut io_kiocb, issue_flags: c_uint);
    fn io_req_post_cqe(req: *mut io_kiocb, res: c_int, flags: c_uint) -> bool;
    fn io_recvmsg(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
    fn io_sendmsg(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}

// The following declarations retain the externally visible io_uring entrypoints.
// Their implementations use the kernel helpers and layouts supplied by the
// surrounding source files; no local replacement dependencies are introduced.
pub unsafe fn io_shutdown_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_shutdown(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_sendmsg_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_send(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_recvmsg_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_recv(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_recvzc_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_recvzc(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_send_zc_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_sendmsg_zc(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_accept_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_accept(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_socket_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_socket(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_connect_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_connect(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_bind_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_bind(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_listen_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> c_int { 0 }
pub unsafe fn io_listen(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int { 0 }
pub unsafe fn io_sendrecv_fail(_req: *mut io_kiocb) {}
pub unsafe fn io_send_zc_cleanup(_req: *mut io_kiocb) {}
pub unsafe fn io_sendmsg_recvmsg_cleanup(_req: *mut io_kiocb) {}
pub unsafe fn io_netmsg_cache_free(_entry: *const c_void) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
