/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2009-2010 Chelsio, Inc. All rights reserved.
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

// Dependency equivalent of <linux/types.h>: __u32 is u32 and __aligned_u64 is u64.

pub const C4IW_UVERBS_ABI_VERSION: u32 = 3;

/*
 * Make sure that all structs defined in this file remain laid out so
 * that they pack the same way on 32-bit and 64-bit architectures (to
 * avoid incompatibility between 32-bit userspace and 64-bit kernels).
 * In particular do not use pointer types -- pass pointers in __aligned_u64
 * instead.
 */

pub const C4IW_64B_CQE: u32 = 1 << 0;

#[repr(C)]
pub struct c4iw_create_cq {
    pub flags: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct c4iw_create_cq_resp {
    pub key: u64,
    pub gts_key: u64,
    pub memsize: u64,
    pub cqid: u32,
    pub size: u32,
    pub qid_mask: u32,
    pub flags: u32,
}

pub const C4IW_QPF_ONCHIP: u32 = 1 << 0;
pub const C4IW_QPF_WRITE_W_IMM: u32 = 1 << 1;

#[repr(C)]
pub struct c4iw_create_qp_resp {
    pub ma_sync_key: u64,
    pub sq_key: u64,
    pub rq_key: u64,
    pub sq_db_gts_key: u64,
    pub rq_db_gts_key: u64,
    pub sq_memsize: u64,
    pub rq_memsize: u64,
    pub sqid: u32,
    pub rqid: u32,
    pub sq_size: u32,
    pub rq_size: u32,
    pub qid_mask: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct c4iw_create_srq_resp {
    pub srq_key: u64,
    pub srq_db_gts_key: u64,
    pub srq_memsize: u64,
    pub srqid: u32,
    pub srq_size: u32,
    pub rqt_abs_idx: u32,
    pub qid_mask: u32,
    pub flags: u32,
    pub reserved: u32, /* explicit padding */
}

/* HW supports SRQ_LIMIT_REACHED event */
pub const T4_SRQ_LIMIT_SUPPORT: u32 = 1 << 0;

#[repr(C)]
pub struct c4iw_alloc_ucontext_resp {
    pub status_page_key: u64,
    pub status_page_size: u32,
    pub reserved: u32, /* explicit padding (optional for i386) */
}

#[repr(C)]
pub struct c4iw_alloc_pd_resp {
    pub pdid: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
