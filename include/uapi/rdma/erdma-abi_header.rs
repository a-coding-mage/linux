/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * Copyright (c) 2020-2022, Alibaba Group.
 */

// Translated from the Linux UAPI header. `__aligned_u64` is represented by
// `u64`; under `repr(C)`, this preserves its native 64-bit alignment.

pub const ERDMA_ABI_VERSION: u32 = 1;

#[repr(C)]
pub struct erdma_ureq_create_cq {
    pub db_record_va: u64,
    pub qbuf_va: u64,
    pub qbuf_len: u32,
    pub rsvd0: u32,
}

#[repr(C)]
pub struct erdma_uresp_create_cq {
    pub cq_id: u32,
    pub num_cqe: u32,
}

#[repr(C)]
pub struct erdma_ureq_create_qp {
    pub db_record_va: u64,
    pub qbuf_va: u64,
    pub qbuf_len: u32,
    pub rsvd0: u32,
}

#[repr(C)]
pub struct erdma_uresp_create_qp {
    pub qp_id: u32,
    pub num_sqe: u32,
    pub num_rqe: u32,
    pub rq_offset: u32,
}

#[repr(C)]
pub struct erdma_uresp_alloc_ctx {
    pub dev_id: u32,
    pub pad: u32,
    pub sdb_type: u32,
    pub sdb_offset: u32,
    pub sdb: u64,
    pub rdb: u64,
    pub cdb: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
