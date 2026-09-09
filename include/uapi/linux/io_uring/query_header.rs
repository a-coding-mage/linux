/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
/*
 * Header file for the io_uring query interface.
 *
 * Copyright (C) 2026 Pavel Begunkov <asml.silence@gmail.com>
 * Copyright (C) Meta Platforms, Inc.
 */

#[repr(C)]
pub struct io_uring_query_hdr {
    pub next_entry: u64,
    pub query_data: u64,
    pub query_op: u32,
    pub size: u32,
    pub result: i32,
    pub __resv: [u32; 3],
}

pub const IO_URING_QUERY_OPCODES: u32 = 0;
pub const IO_URING_QUERY_ZCRX: u32 = 1;
pub const IO_URING_QUERY_SCQ: u32 = 2;
pub const IO_URING_QUERY_ZCRX_EVENT: u32 = 3;
pub const __IO_URING_QUERY_MAX: u32 = 4;

/* Doesn't require a ring */
#[repr(C)]
pub struct io_uring_query_opcode {
    /* The number of supported IORING_OP_* opcodes */
    pub nr_request_opcodes: u32,
    /* The number of supported IORING_[UN]REGISTER_* opcodes */
    pub nr_register_opcodes: u32,
    /* Bitmask of all supported IORING_FEAT_* flags */
    pub feature_flags: u64,
    /* Bitmask of all supported IORING_SETUP_* flags */
    pub ring_setup_flags: u64,
    /* Bitmask of all supported IORING_ENTER_** flags */
    pub enter_flags: u64,
    /* Bitmask of all supported IOSQE_* flags */
    pub sqe_flags: u64,
    /* The number of available query opcodes */
    pub nr_query_opcodes: u32,
    pub __pad: u32,
}

#[repr(C)]
pub struct io_uring_query_zcrx {
    /* Bitmask of supported ZCRX_REG_* flags, */
    pub register_flags: u64,
    /* Bitmask of all supported IORING_ZCRX_AREA_* flags */
    pub area_flags: u64,
    /* The number of supported ZCRX_CTRL_* opcodes */
    pub nr_ctrl_opcodes: u32,
    /* Bitmask of ZCRX_FEATURE_* indicating which features are available */
    pub features: u32,
    /* The refill ring header size */
    pub rq_hdr_size: u32,
    /* The alignment for the header */
    pub rq_hdr_alignment: u32,
    pub __resv2: u64,
}

#[repr(C)]
pub struct io_uring_query_zcrx_event {
    /* Bitmask of supported ZCRX_EVENT_* flags */
    pub event_flags: u32,
    /* Size of zcrx_stats */
    pub stats_size: u32,
    /* Required alignment for the stats struct within the region (ie stats_offset) */
    pub stats_off_alignment: u32,
    pub __resv1: u32,
    pub __resv2: [u64; 4],
}

#[repr(C)]
pub struct io_uring_query_scq {
    /* The SQ/CQ rings header size */
    pub hdr_size: u64,
    /* The alignment for the header */
    pub hdr_alignment: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
