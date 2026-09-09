/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * iSCSI over TCP/IP Data-Path lib
 *
 * Copyright (C) 2008 Mike Christie
 * Copyright (C) 2008 Red Hat, Inc.  All rights reserved.
 * maintained by open-iscsi@googlegroups.com
 */

// Dependency declarations supplied by the surrounding translation unit.

pub type iscsi_segment_done_fn_t = unsafe extern "C" fn(
    *mut iscsi_tcp_conn,
    *mut iscsi_segment,
) -> ::core::ffi::c_int;

#[repr(C)]
pub struct iscsi_segment {
    pub data: *mut u8,
    pub size: ::core::ffi::c_uint,
    pub copied: ::core::ffi::c_uint,
    pub total_size: ::core::ffi::c_uint,
    pub total_copied: ::core::ffi::c_uint,

    pub crcp: *mut u32,
    pub padbuf: [u8; ISCSI_PAD_LEN as usize],
    pub recv_digest: [u8; ISCSI_DIGEST_SIZE as usize],
    pub digest: [u8; ISCSI_DIGEST_SIZE as usize],
    pub digest_len: ::core::ffi::c_uint,

    pub sg: *mut scatterlist,
    pub sg_mapped: *mut ::core::ffi::c_void,
    pub sg_offset: ::core::ffi::c_uint,
    pub atomic_mapped: bool,

    pub done: Option<iscsi_segment_done_fn_t>,
}

/* Socket connection receive helper */
#[repr(C)]
pub struct iscsi_tcp_recv {
    pub hdr: *mut iscsi_hdr,
    pub segment: iscsi_segment,

    /* Allocate buffer for BHS + AHS */
    pub hdr_buf: [u32; 64],

    /* copied and flipped values */
    pub datalen: ::core::ffi::c_int,
}

#[repr(C)]
pub struct iscsi_tcp_conn {
    pub iscsi_conn: *mut iscsi_conn,
    pub dd_data: *mut ::core::ffi::c_void,
    pub stop_stage: ::core::ffi::c_int, /* conn_stop() flag:
                                         * stop to recover,
                                         * stop to terminate */
    /* control data */
    pub r#in: iscsi_tcp_recv, /* TCP receive context */
    /* CRC32C (Rx) LLD should set this if they do not offload */
    pub rx_crcp: *mut u32,
}

#[repr(C)]
pub struct iscsi_tcp_task {
    pub exp_datasn: u32, /* expected target's R2TSN/DataSN */
    pub data_offset: ::core::ffi::c_int,
    pub r2t: *mut iscsi_r2t_info, /* in progress solict R2T */
    pub r2tpool: iscsi_pool,
    pub r2tqueue: kfifo,
    pub dd_data: *mut ::core::ffi::c_void,
    pub pool2queue: spinlock_t,
    pub queue2pool: spinlock_t,
}

pub const ISCSI_TCP_SEGMENT_DONE: ::core::ffi::c_uint = 0;
pub const ISCSI_TCP_SKB_DONE: ::core::ffi::c_uint = 1;
pub const ISCSI_TCP_CONN_ERR: ::core::ffi::c_uint = 2;
pub const ISCSI_TCP_SUSPENDED: ::core::ffi::c_uint = 3;

unsafe extern "C" {
    pub fn iscsi_tcp_hdr_recv_prep(tcp_conn: *mut iscsi_tcp_conn);
    pub fn iscsi_tcp_recv_skb(
        conn: *mut iscsi_conn,
        skb: *mut sk_buff,
        offset: ::core::ffi::c_uint,
        offloaded: bool,
        status: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn iscsi_tcp_cleanup_task(task: *mut iscsi_task);
    pub fn iscsi_tcp_task_init(task: *mut iscsi_task) -> ::core::ffi::c_int;
    pub fn iscsi_tcp_task_xmit(task: *mut iscsi_task) -> ::core::ffi::c_int;

    /* segment helpers */
    pub fn iscsi_tcp_recv_segment_is_hdr(tcp_conn: *mut iscsi_tcp_conn)
        -> ::core::ffi::c_int;
    pub fn iscsi_tcp_segment_done(
        tcp_conn: *mut iscsi_tcp_conn,
        segment: *mut iscsi_segment,
        recv: ::core::ffi::c_int,
        copied: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn iscsi_tcp_segment_unmap(segment: *mut iscsi_segment);

    pub fn iscsi_segment_init_linear(
        segment: *mut iscsi_segment,
        data: *mut ::core::ffi::c_void,
        size: usize,
        done: Option<iscsi_segment_done_fn_t>,
        crcp: *mut u32,
    );
    pub fn iscsi_segment_seek_sg(
        segment: *mut iscsi_segment,
        sg_list: *mut scatterlist,
        sg_count: ::core::ffi::c_uint,
        offset: ::core::ffi::c_uint,
        size: usize,
        done: Option<iscsi_segment_done_fn_t>,
        crcp: *mut u32,
    ) -> ::core::ffi::c_int;

    /* digest helpers */
    pub fn iscsi_tcp_dgst_header(
        hdr: *const ::core::ffi::c_void,
        hdrlen: usize,
        digest: *mut u8,
    );
    pub fn iscsi_tcp_conn_setup(
        cls_session: *mut iscsi_cls_session,
        dd_data_size: ::core::ffi::c_int,
        conn_idx: u32,
    ) -> *mut iscsi_cls_conn;
    pub fn iscsi_tcp_conn_teardown(cls_conn: *mut iscsi_cls_conn);

    /* misc helpers */
    pub fn iscsi_tcp_r2tpool_alloc(session: *mut iscsi_session) -> ::core::ffi::c_int;
    pub fn iscsi_tcp_r2tpool_free(session: *mut iscsi_session);
    pub fn iscsi_tcp_set_max_r2t(
        conn: *mut iscsi_conn,
        buf: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn iscsi_tcp_conn_get_stats(
        cls_conn: *mut iscsi_cls_conn,
        stats: *mut iscsi_stats,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
