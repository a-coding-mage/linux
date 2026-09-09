/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause) */
/* This file is part of the Emulex RoCE Device Driver for
 * RoCE (RDMA over Converged Ethernet) adapters.
 * Copyright (C) 2012-2015 Emulex. All rights reserved.
 * EMULEX and SLI are trademarks of Emulex.
 * www.emulex.com
 *
 * This software is available under a choice of one of two licenses.
 * You may choose to be licensed under the terms of the GNU General Public
 * License (GPL) Version 2, available from the file COPYING in the main
 * directory of this source tree, or the BSD license below:
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 *
 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
 * BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 * OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Contact Information:
 * linux-drivers@emulex.com
 *
 * Emulex
 * 3333 Susan Street
 * Costa Mesa, CA 92626
 */

pub const OCRDMA_ABI_VERSION: u32 = 2;
pub const OCRDMA_BE_ROCE_ABI_VERSION: u32 = 1;
/* user kernel communication data structures. */

#[repr(C)]
pub struct ocrdma_alloc_ucontext_resp {
    pub dev_id: u32,
    pub wqe_size: u32,
    pub max_inline_data: u32,
    pub dpp_wqe_size: u32,
    pub ah_tbl_page: u64,
    pub ah_tbl_len: u32,
    pub rqe_size: u32,
    pub fw_ver: [u8; 32],
    /* for future use/new features in progress */
    pub rsvd1: u64,
    pub rsvd2: u64,
}

#[repr(C)]
pub struct ocrdma_alloc_pd_ureq {
    pub rsvd: [u32; 2],
}

#[repr(C)]
pub struct ocrdma_alloc_pd_uresp {
    pub id: u32,
    pub dpp_enabled: u32,
    pub dpp_page_addr_hi: u32,
    pub dpp_page_addr_lo: u32,
    pub rsvd: [u32; 2],
}

#[repr(C)]
pub struct ocrdma_create_cq_ureq {
    pub dpp_cq: u32,
    pub rsvd: u32, /* pad */
}

pub const MAX_CQ_PAGES: usize = 8;
#[repr(C)]
pub struct ocrdma_create_cq_uresp {
    pub cq_id: u32,
    pub page_size: u32,
    pub num_pages: u32,
    pub max_hw_cqe: u32,
    pub page_addr: [u64; MAX_CQ_PAGES],
    pub db_page_addr: u64,
    pub db_page_size: u32,
    pub phase_change: u32,
    /* for future use/new features in progress */
    pub rsvd1: u64,
    pub rsvd2: u64,
}

pub const MAX_QP_PAGES: usize = 8;
pub const MAX_UD_AV_PAGES: usize = 8;

#[repr(C)]
pub struct ocrdma_create_qp_ureq {
    pub enable_dpp_cq: u8,
    pub rsvd: u8,
    pub dpp_cq_id: u16,
    pub rsvd1: u32, /* pad */
}

#[repr(C)]
pub struct ocrdma_create_qp_uresp {
    pub qp_id: u16,
    pub sq_dbid: u16,
    pub rq_dbid: u16,
    pub resv0: u16, /* pad */
    pub sq_page_size: u32,
    pub rq_page_size: u32,
    pub num_sq_pages: u32,
    pub num_rq_pages: u32,
    pub sq_page_addr: [u64; MAX_QP_PAGES],
    pub rq_page_addr: [u64; MAX_QP_PAGES],
    pub db_page_addr: u64,
    pub db_page_size: u32,
    pub dpp_credit: u32,
    pub dpp_offset: u32,
    pub num_wqe_allocated: u32,
    pub num_rqe_allocated: u32,
    pub db_sq_offset: u32,
    pub db_rq_offset: u32,
    pub db_shift: u32,
    pub rsvd: [u64; 11],
}

#[repr(C)]
pub struct ocrdma_create_srq_uresp {
    pub rq_dbid: u16,
    pub resv0: u16, /* pad */
    pub resv1: u32,
    pub rq_page_size: u32,
    pub num_rq_pages: u32,
    pub rq_page_addr: [u64; MAX_QP_PAGES],
    pub db_page_addr: u64,
    pub db_page_size: u32,
    pub num_rqe_allocated: u32,
    pub db_rq_offset: u32,
    pub db_shift: u32,
    pub rsvd2: u64,
    pub rsvd3: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
