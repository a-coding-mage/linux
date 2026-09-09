/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2005 Topspin Communications.  All rights reserved.
 * Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
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

/* Linux header guard: MTHCA_ABI_USER_H */

/* Dependency equivalent: linux/types.h */

/*
 * Increment this value if any changes that break userspace ABI
 * compatibility are made.
 */
pub const MTHCA_UVERBS_ABI_VERSION: u32 = 1;

/*
 * Make sure that all structs defined in this file remain laid out so
 * that they pack the same way on 32-bit and 64-bit architectures (to
 * avoid incompatibility between 32-bit userspace and 64-bit kernels).
 * In particular do not use pointer types -- pass pointers in __u64
 * instead.
 */
#[repr(C)]
pub struct mthca_alloc_ucontext_resp {
    pub qp_tab_size: u32,
    pub uarc_size: u32,
}

#[repr(C)]
pub struct mthca_alloc_pd_resp {
    pub pdn: u32,
    pub reserved: u32,
}

/*
 * Mark the memory region with a DMA attribute that causes
 * in-flight DMA to be flushed when the region is written to:
 */
pub const MTHCA_MR_DMASYNC: u32 = 0x1;

#[repr(C)]
pub struct mthca_reg_mr {
    pub mr_attrs: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mthca_create_cq {
    pub lkey: u32,
    pub pdn: u32,
    pub arm_db_page: u64,
    pub set_db_page: u64,
    pub arm_db_index: u32,
    pub set_db_index: u32,
}

#[repr(C)]
pub struct mthca_create_cq_resp {
    pub cqn: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mthca_resize_cq {
    pub lkey: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mthca_create_srq {
    pub lkey: u32,
    pub db_index: u32,
    pub db_page: u64,
}

#[repr(C)]
pub struct mthca_create_srq_resp {
    pub srqn: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct mthca_create_qp {
    pub lkey: u32,
    pub reserved: u32,
    pub sq_db_page: u64,
    pub rq_db_page: u64,
    pub sq_db_index: u32,
    pub rq_db_index: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
