/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2016 Hisilicon Limited.
 *
 * This software is available under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Dependency: Linux __u8, __u32, and __aligned_u64 map to u8, u32, and u64.

#[repr(C)]
pub struct hns_roce_ib_create_cq {
    pub buf_addr: u64,
    pub db_addr: u64,
    pub cqe_size: u32,
    pub reserved: u32,
}

pub const HNS_ROCE_CQ_FLAG_RECORD_DB: i32 = 1 << 0;

#[repr(C)]
pub struct hns_roce_ib_create_cq_resp {
    pub cqn: u64, // Only 32 bits used, 64 for compat
    pub cap_flags: u64,
}

pub const HNS_ROCE_SRQ_CAP_RECORD_DB: i32 = 1 << 0;
pub const HNS_ROCE_RSP_SRQ_CAP_RECORD_DB: i32 = 1 << 0;

#[repr(C)]
pub struct hns_roce_ib_create_srq {
    pub buf_addr: u64,
    pub db_addr: u64,
    pub que_addr: u64,
    pub req_cap_flags: u32, // Use enum hns_roce_srq_cap_flags
    pub reserved: u32,
}

#[repr(C)]
pub struct hns_roce_ib_create_srq_resp {
    pub srqn: u32,
    pub cap_flags: u32, // Use enum hns_roce_srq_cap_flags
}

pub const HNS_ROCE_CREATE_QP_FLAGS_DCQCN: i32 = 0;
pub const HNS_ROCE_CREATE_QP_FLAGS_LDCP: i32 = 1;
pub const HNS_ROCE_CREATE_QP_FLAGS_HC3: i32 = 2;
pub const HNS_ROCE_CREATE_QP_FLAGS_DIP: i32 = 3;

pub const HNS_ROCE_CREATE_QP_MASK_CONGEST_TYPE: i32 = 1 << 0;

#[repr(C)]
pub struct hns_roce_ib_create_qp {
    pub buf_addr: u64,
    pub db_addr: u64,
    pub log_sq_bb_count: u8,
    pub log_sq_stride: u8,
    pub sq_no_prefetch: u8,
    pub reserved: [u8; 5],
    pub sdb_addr: u64,
    pub comp_mask: u64, // Use enum hns_roce_create_qp_comp_mask
    pub create_flags: u64,
    pub cong_type_flags: u64,
}

pub const HNS_ROCE_QP_CAP_RQ_RECORD_DB: i32 = 1 << 0;
pub const HNS_ROCE_QP_CAP_SQ_RECORD_DB: i32 = 1 << 1;
pub const HNS_ROCE_QP_CAP_OWNER_DB: i32 = 1 << 2;
pub const HNS_ROCE_QP_CAP_DIRECT_WQE: i32 = 1 << 5;

#[repr(C)]
pub struct hns_roce_ib_create_qp_resp {
    pub cap_flags: u64,
    pub dwqe_mmap_key: u64,
}

#[repr(C)]
pub struct hns_roce_ib_modify_qp_resp {
    pub tc_mode: u8,
    pub priority: u8,
    pub reserved: [u8; 6],
}

pub const HNS_ROCE_EXSGE_FLAGS: i32 = 1 << 0;
pub const HNS_ROCE_RQ_INLINE_FLAGS: i32 = 1 << 1;
pub const HNS_ROCE_CQE_INLINE_FLAGS: i32 = 1 << 2;

pub const HNS_ROCE_RSP_EXSGE_FLAGS: i32 = 1 << 0;
pub const HNS_ROCE_RSP_RQ_INLINE_FLAGS: i32 = 1 << 1;
pub const HNS_ROCE_RSP_CQE_INLINE_FLAGS: i32 = 1 << 2;

#[repr(C)]
pub struct hns_roce_ib_alloc_ucontext_resp {
    pub qp_tab_size: u32,
    pub cqe_size: u32,
    pub srq_tab_size: u32,
    pub reserved: u32,
    pub config: u32,
    pub max_inline_data: u32,
    pub congest_type: u8,
    pub reserved0: [u8; 7],
}

#[repr(C)]
pub struct hns_roce_ib_alloc_ucontext {
    pub config: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct hns_roce_ib_alloc_pd_resp {
    pub pdn: u32,
}

#[repr(C)]
pub struct hns_roce_ib_create_ah_resp {
    pub dmac: [u8; 6],
    pub priority: u8,
    pub tc_mode: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
