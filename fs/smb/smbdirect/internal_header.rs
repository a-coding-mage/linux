/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (c) 2025, Stefan Metzmacher
 */

/* Translated from __FS_SMB_COMMON_SMBDIRECT_INTERNAL_H__. */

pub const DEFAULT_SYMBOL_NAMESPACE: &str = "SMBDIRECT";

/* KBUILD_MODNAME ": " fmt */
#[inline]
pub fn pr_fmt(fmt: &str) -> String {
    format!("{}: {}", KBUILD_MODNAME, fmt)
}

#[repr(C)]
pub struct smbdirect_module_state {
    pub mutex: mutex,
    pub workqueues: smbdirect_module_state_workqueues,
    pub devices: smbdirect_module_state_devices,
}

#[repr(C)]
pub struct smbdirect_module_state_workqueues {
    pub accept: *mut workqueue_struct,
    pub connect: *mut workqueue_struct,
    pub idle: *mut workqueue_struct,
    pub refill: *mut workqueue_struct,
    pub immediate: *mut workqueue_struct,
    pub cleanup: *mut workqueue_struct,
}

#[repr(C)]
pub struct smbdirect_module_state_devices {
    pub lock: rwlock_t,
    pub list: list_head,
}

unsafe extern "C" {
    pub static mut smbdirect_globals: smbdirect_module_state;

    pub fn smbdirect_socket_init_new(
        net: *mut net,
        sc: *mut smbdirect_socket,
    ) -> ::std::ffi::c_int;

    pub fn smbdirect_socket_init_accepting(
        id: *mut rdma_cm_id,
        sc: *mut smbdirect_socket,
    ) -> ::std::ffi::c_int;

    pub fn __smbdirect_socket_schedule_cleanup(
        sc: *mut smbdirect_socket,
        macro_name: *const ::std::ffi::c_char,
        lvl: ::std::ffi::c_uint,
        func: *const ::std::ffi::c_char,
        line: ::std::ffi::c_uint,
        error: ::std::ffi::c_int,
        force_status: *mut smbdirect_socket_status,
    );

    pub fn smbdirect_socket_destroy_sync(sc: *mut smbdirect_socket);

    pub fn smbdirect_socket_wait_for_credits(
        sc: *mut smbdirect_socket,
        expected_status: smbdirect_socket_status,
        unexpected_errno: ::std::ffi::c_int,
        waitq: *mut wait_queue_head_t,
        total_credits: *mut atomic_t,
        needed: ::std::ffi::c_int,
    ) -> ::std::ffi::c_int;

    pub fn smbdirect_connection_rdma_established(sc: *mut smbdirect_socket);
    pub fn smbdirect_connection_negotiation_done(sc: *mut smbdirect_socket);
    pub fn smbdirect_connection_create_qp(sc: *mut smbdirect_socket) -> ::std::ffi::c_int;
    pub fn smbdirect_connection_destroy_qp(sc: *mut smbdirect_socket);
    pub fn smbdirect_connection_create_mem_pools(sc: *mut smbdirect_socket) -> ::std::ffi::c_int;
    pub fn smbdirect_connection_destroy_mem_pools(sc: *mut smbdirect_socket);
    pub fn smbdirect_connection_alloc_send_io(sc: *mut smbdirect_socket) -> *mut smbdirect_send_io;
    pub fn smbdirect_connection_free_send_io(msg: *mut smbdirect_send_io);
    pub fn smbdirect_connection_get_recv_io(sc: *mut smbdirect_socket) -> *mut smbdirect_recv_io;
    pub fn smbdirect_connection_put_recv_io(msg: *mut smbdirect_recv_io);
    pub fn smbdirect_connection_reassembly_append_recv_io(
        sc: *mut smbdirect_socket,
        msg: *mut smbdirect_recv_io,
        data_length: u32,
    );
    pub fn smbdirect_connection_reassembly_first_recv_io(
        sc: *mut smbdirect_socket,
    ) -> *mut smbdirect_recv_io;
    pub fn smbdirect_connection_negotiate_rdma_resources(
        sc: *mut smbdirect_socket,
        peer_initiator_depth: u8,
        peer_responder_resources: u8,
        param: *const rdma_conn_param,
    );
    pub fn smbdirect_connection_idle_timer_work(work: *mut work_struct);
    pub fn smbdirect_connection_grant_recv_credits(sc: *mut smbdirect_socket) -> u16;
    pub fn smbdirect_connection_post_send_wr(
        sc: *mut smbdirect_socket,
        wr: *mut ib_send_wr,
    ) -> ::std::ffi::c_int;
    pub fn smbdirect_connection_post_recv_io(msg: *mut smbdirect_recv_io) -> ::std::ffi::c_int;
    pub fn smbdirect_connection_recv_io_done(cq: *mut ib_cq, wc: *mut ib_wc);
    pub fn smbdirect_connection_recv_io_refill(sc: *mut smbdirect_socket) -> ::std::ffi::c_int;
    pub fn smbdirect_connection_create_mr_list(sc: *mut smbdirect_socket) -> ::std::ffi::c_int;
    pub fn smbdirect_connection_destroy_mr_list(sc: *mut smbdirect_socket);
    pub fn smbdirect_accept_connect_request(
        sc: *mut smbdirect_socket,
        param: *const rdma_conn_param,
    ) -> ::std::ffi::c_int;
    pub fn smbdirect_accept_negotiate_finish(sc: *mut smbdirect_socket, ntstatus: u32);
    pub fn smbdirect_devices_init() -> ::std::ffi::c_int;
    pub fn smbdirect_devices_exit();
}

#[repr(C)]
pub struct smbdirect_device {
    pub list: list_head,
    pub ib_dev: *mut ib_device,
    /* copy of ib_dev->name, in order to print renames */
    pub ib_name: [::std::ffi::c_char; IB_DEVICE_NAME_MAX as usize],
}

#[macro_export]
macro_rules! smbdirect_socket_schedule_cleanup {
    ($sc:expr, $error:expr) => {
        unsafe {
            $crate::__smbdirect_socket_schedule_cleanup(
                $sc,
                b"smbdirect_socket_schedule_cleanup\0".as_ptr() as *const _,
                SMBDIRECT_LOG_ERR,
                b"<caller>\0".as_ptr() as *const _,
                line!(),
                $error,
                ::std::ptr::null_mut(),
            )
        }
    };
}

#[macro_export]
macro_rules! smbdirect_socket_schedule_cleanup_lvl {
    ($sc:expr, $lvl:expr, $error:expr) => {
        unsafe {
            $crate::__smbdirect_socket_schedule_cleanup(
                $sc,
                b"smbdirect_socket_schedule_cleanup_lvl\0".as_ptr() as *const _,
                $lvl,
                b"<caller>\0".as_ptr() as *const _,
                line!(),
                $error,
                ::std::ptr::null_mut(),
            )
        }
    };
}

#[macro_export]
macro_rules! smbdirect_socket_schedule_cleanup_status {
    ($sc:expr, $lvl:expr, $error:expr, $status:expr) => {{
        let mut force_status = $status;
        unsafe {
            $crate::__smbdirect_socket_schedule_cleanup(
                $sc,
                b"smbdirect_socket_schedule_cleanup_status\0".as_ptr() as *const _,
                $lvl,
                b"<caller>\0".as_ptr() as *const _,
                line!(),
                $error,
                &mut force_status,
            )
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
