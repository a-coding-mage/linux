// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the corresponding Linux/io_uring headers are
// intentionally referenced here rather than redefined.

#[repr(C)]
pub struct io_async_msghdr {
    #[cfg(feature = "CONFIG_NET")]
    pub vec: iou_vec,

    #[cfg(feature = "CONFIG_NET")]
    pub namelen: ::std::os::raw::c_int,
    #[cfg(feature = "CONFIG_NET")]
    pub fast_iov: iovec,
    #[cfg(feature = "CONFIG_NET")]
    pub controllen: __kernel_size_t,
    #[cfg(feature = "CONFIG_NET")]
    pub payloadlen: __kernel_size_t,
    #[cfg(feature = "CONFIG_NET")]
    pub uaddr: *mut sockaddr,
    #[cfg(feature = "CONFIG_NET")]
    pub msg: msghdr,
    #[cfg(feature = "CONFIG_NET")]
    pub addr: sockaddr_storage,
}

#[cfg(feature = "CONFIG_NET")]
extern "C" {
    pub fn io_shutdown_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_shutdown(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_sendmsg_recvmsg_cleanup(req: *mut io_kiocb);
    pub fn io_sendmsg_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_sendmsg(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_send(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_recvmsg_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_recvmsg(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn io_recv(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_sendrecv_fail(req: *mut io_kiocb);

    pub fn io_accept_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_accept(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_socket_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_socket(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn io_socket_bpf_populate(bctx: *mut io_uring_bpf_ctx, req: *mut io_kiocb);
    pub fn io_connect_bpf_populate(bctx: *mut io_uring_bpf_ctx, req: *mut io_kiocb);

    pub fn io_connect_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_connect(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_sendmsg_zc(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn io_send_zc_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_send_zc_cleanup(req: *mut io_kiocb);

    pub fn io_bind_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_bind(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_listen_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_listen(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;

    pub fn io_netmsg_cache_free(entry: *const ::std::ffi::c_void);
}

#[cfg(not(feature = "CONFIG_NET"))]
pub unsafe fn io_netmsg_cache_free(_entry: *const ::std::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_NET"))]
pub unsafe fn io_socket_bpf_populate(_bctx: *mut io_uring_bpf_ctx, _req: *mut io_kiocb) {}

#[cfg(not(feature = "CONFIG_NET"))]
pub unsafe fn io_connect_bpf_populate(_bctx: *mut io_uring_bpf_ctx, _req: *mut io_kiocb) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
