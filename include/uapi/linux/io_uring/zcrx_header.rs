/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
/*
 * Header file for the io_uring zerocopy receive (zcrx) interface.
 *
 * Copyright (C) 2026 Pavel Begunkov
 * Copyright (C) 2026 David Wei
 * Copyright (C) Meta Platforms, Inc.
 */

/* Dependency equivalent of <linux/types.h>: __u32 and __u64 are u32 and u64. */

/* Zero copy receive refill queue entry */
#[repr(C)]
pub struct io_uring_zcrx_rqe {
    pub off: u64,
    pub len: u32,
    pub __pad: u32,
}

#[repr(C)]
pub struct io_uring_zcrx_cqe {
    pub off: u64,
    pub __pad: u64,
}

/* The bit from which area id is encoded into offsets */
pub const IORING_ZCRX_AREA_SHIFT: u32 = 48;
pub const IORING_ZCRX_AREA_MASK: u64 = !((1u64 << IORING_ZCRX_AREA_SHIFT) - 1);

#[repr(C)]
pub struct io_uring_zcrx_offsets {
    pub head: u32,
    pub tail: u32,
    pub rqes: u32,
    pub __resv2: u32,
    pub __resv: [u64; 2],
}

#[repr(u32)]
pub enum io_uring_zcrx_area_flags {
    IORING_ZCRX_AREA_DMABUF = 1,
}

#[repr(C)]
pub struct io_uring_zcrx_area_reg {
    pub addr: u64,
    pub len: u64,
    pub rq_area_token: u64,
    pub flags: u32,
    pub dmabuf_fd: u32,
    pub __resv2: [u64; 2],
}

#[repr(u32)]
pub enum zcrx_reg_flags {
    ZCRX_REG_IMPORT = 1,

    /*
     * Register a zcrx instance without a net device. All data will be
     * copied. The refill queue entries might not be automatically
     * consumed and need to be flushed, see ZCRX_CTRL_FLUSH_RQ.
     */
    ZCRX_REG_NODEV = 2,
}

#[repr(u32)]
pub enum zcrx_features {
    /*
     * The user can ask for the desired rx page size by passing the
     * value in struct io_uring_zcrx_ifq_reg::rx_buf_len.
     */
    ZCRX_FEATURE_RX_PAGE_SIZE = 1 << 0,
    ZCRX_FEATURE_EVENT = 1 << 1,
}

#[repr(u32)]
pub enum zcrx_event_type {
    ZCRX_EVENT_ALLOC_FAIL,
    ZCRX_EVENT_COPY,

    __ZCRX_EVENT_TYPE_LAST,
}

#[repr(u32)]
pub enum zcrx_event_desc_flags {
    /* If set, stats_offset holds a valid offset to a zcrx_stats struct */
    ZCRX_EVENT_DESC_FLAG_STATS = 1 << 0,
}

#[repr(C)]
pub struct zcrx_stats {
    pub copy_count: u64, /* cumulative copy-fallback CQEs */
    pub copy_bytes: u64, /* cumulative bytes copied */
}

#[repr(C)]
pub struct zcrx_event_desc {
    pub user_data: u64,
    pub type_mask: u32,
    pub flags: u32, /* see enum zcrx_event_desc_flags */
    pub stats_offset: u64, /* offset from the beginning of refill ring region for stats */
    pub __resv2: [u64; 9],
}

/*
 * Argument for IORING_REGISTER_ZCRX_IFQ
 */
#[repr(C)]
pub struct io_uring_zcrx_ifq_reg {
    pub if_idx: u32,
    pub if_rxq: u32,
    pub rq_entries: u32,
    pub flags: u32,
    pub area_ptr: u64, /* pointer to struct io_uring_zcrx_area_reg */
    pub region_ptr: u64, /* struct io_uring_region_desc * */
    pub offsets: io_uring_zcrx_offsets,
    pub zcrx_id: u32,
    pub rx_buf_len: u32,
    pub event_desc: u64, /* see struct zcrx_event_desc */
    pub __resv: [u64; 2],
}

#[repr(u32)]
pub enum zcrx_ctrl_op {
    ZCRX_CTRL_FLUSH_RQ,
    ZCRX_CTRL_EXPORT,
    ZCRX_CTRL_ARM_EVENT,
    ZCRX_CTRL_ADD_AREA,

    __ZCRX_CTRL_LAST,
}

#[repr(C)]
pub struct zcrx_ctrl_flush_rq {
    pub __resv: [u64; 6],
}

#[repr(C)]
pub struct zcrx_ctrl_export {
    pub zcrx_fd: u32,
    pub __resv1: [u32; 11],
}

#[repr(C)]
pub struct zcrx_ctrl_arm_event {
    pub event_type: u32, /* see enum zcrx_event_type */
    pub __resv: [u32; 11],
}

#[repr(C)]
pub struct zcrx_ctrl_add_area {
    pub area_ptr: u64, /* pointer to struct io_uring_zcrx_area_reg */
    pub __resv: [u64; 5],
}

#[repr(C)]
pub union zcrx_ctrl_payload {
    pub zc_export: zcrx_ctrl_export,
    pub zc_flush: zcrx_ctrl_flush_rq,
    pub zc_arm_event: zcrx_ctrl_arm_event,
    pub zc_area: zcrx_ctrl_add_area,
}

#[repr(C)]
pub struct zcrx_ctrl {
    pub zcrx_id: u32,
    pub op: u32, /* see enum zcrx_ctrl_op */
    pub __resv: [u64; 2],
    pub payload: zcrx_ctrl_payload,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
