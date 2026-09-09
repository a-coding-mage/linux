/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/* QLogic qedr NIC Driver
 * Copyright (c) 2015-2016  QLogic Corporation
 *
 * This software is available to you under a choice of one of two
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
 *        disclaimer in the documentation and /or other materials
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

pub const QEDR_ABI_VERSION: u32 = 8;

#[repr(i32)]
pub enum qedr_alloc_ucontext_flags {
    QEDR_ALLOC_UCTX_EDPM_MODE = 1 << 0,
    QEDR_ALLOC_UCTX_DB_REC = 1 << 1,
    QEDR_SUPPORT_DPM_SIZES = 1 << 2,
}

#[repr(C)]
pub struct qedr_alloc_ucontext_req {
    pub context_flags: u32,
    pub reserved: u32,
}

pub const QEDR_LDPM_MAX_SIZE: u32 = 8192;
pub const QEDR_EDPM_TRANS_SIZE: u32 = 64;
pub const QEDR_EDPM_MAX_SIZE: u32 = ROCE_REQ_MAX_INLINE_DATA_SIZE;

#[repr(i32)]
pub enum qedr_rdma_dpm_type {
    QEDR_DPM_TYPE_NONE = 0,
    QEDR_DPM_TYPE_ROCE_ENHANCED = 1 << 0,
    QEDR_DPM_TYPE_ROCE_LEGACY = 1 << 1,
    QEDR_DPM_TYPE_IWARP_LEGACY = 1 << 2,
    QEDR_DPM_TYPE_ROCE_EDPM_MODE = 1 << 3,
    QEDR_DPM_SIZES_SET = 1 << 4,
}

#[repr(C)]
pub struct qedr_alloc_ucontext_resp {
    pub db_pa: u64,
    pub db_size: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_srq_wr: u32,
    pub sges_per_send_wr: u32,
    pub sges_per_recv_wr: u32,
    pub sges_per_srq_wr: u32,
    pub max_cqes: u32,
    pub dpm_flags: u8,
    pub wids_enabled: u8,
    pub wid_count: u16,
    pub ldpm_limit_size: u16,
    pub edpm_trans_size: u8,
    pub reserved: u8,
    pub edpm_limit_size: u16,
    pub padding: [u8; 6],
}

#[repr(C)]
pub struct qedr_alloc_pd_ureq {
    pub rsvd1: u64,
}

#[repr(C)]
pub struct qedr_alloc_pd_uresp {
    pub pd_id: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct qedr_create_cq_ureq {
    pub addr: u64,
    pub len: u64,
}

#[repr(C)]
pub struct qedr_create_cq_uresp {
    pub db_offset: u32,
    pub icid: u16,
    pub reserved: u16,
    pub db_rec_addr: u64,
}

#[repr(C)]
pub struct qedr_create_qp_ureq {
    pub qp_handle_hi: u32,
    pub qp_handle_lo: u32,
    /* SQ */
    /* user space virtual address of SQ buffer */
    pub sq_addr: u64,
    /* length of SQ buffer */
    pub sq_len: u64,
    /* RQ */
    /* user space virtual address of RQ buffer */
    pub rq_addr: u64,
    /* length of RQ buffer */
    pub rq_len: u64,
}

#[repr(C)]
pub struct qedr_create_qp_uresp {
    pub qp_id: u32,
    pub atomic_supported: u32,
    /* SQ */
    pub sq_db_offset: u32,
    pub sq_icid: u16,
    /* RQ */
    pub rq_db_offset: u32,
    pub rq_icid: u16,
    pub rq_db2_offset: u32,
    pub reserved: u32,
    /* address of SQ doorbell recovery user entry */
    pub sq_db_rec_addr: u64,
    /* address of RQ doorbell recovery user entry */
    pub rq_db_rec_addr: u64,
}

#[repr(C)]
pub struct qedr_create_srq_ureq {
    /* user space virtual address of producer pair */
    pub prod_pair_addr: u64,
    /* user space virtual address of SRQ buffer */
    pub srq_addr: u64,
    /* length of SRQ buffer */
    pub srq_len: u64,
}

#[repr(C)]
pub struct qedr_create_srq_uresp {
    pub srq_id: u16,
    pub reserved0: u16,
    pub reserved1: u32,
}

/* doorbell recovery entry allocated and populated by userspace doorbelling
 * entities and mapped to kernel. Kernel uses this to register doorbell
 * information with doorbell drop recovery mechanism.
 */
#[repr(C)]
pub struct qedr_user_db_rec {
    pub db_data: u64, /* doorbell data */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
