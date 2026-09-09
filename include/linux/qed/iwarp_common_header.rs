/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

// Dependency intent: declarations from <linux/qed/rdma_common.h> are supplied
// by the surrounding translation unit.

/************************/
/* IWARP FW CONSTANTS    */
/************************/

pub const IWARP_ACTIVE_MODE: i32 = 0;
pub const IWARP_PASSIVE_MODE: i32 = 1;

pub const IWARP_SHARED_QUEUE_PAGE_SIZE: u32 = 0x8000;
pub const IWARP_SHARED_QUEUE_PAGE_RQ_PBL_OFFSET: u32 = 0x4000;
pub const IWARP_SHARED_QUEUE_PAGE_RQ_PBL_MAX_SIZE: u32 = 0x1000;
pub const IWARP_SHARED_QUEUE_PAGE_SQ_PBL_OFFSET: u32 = 0x5000;
pub const IWARP_SHARED_QUEUE_PAGE_SQ_PBL_MAX_SIZE: u32 = 0x3000;

pub const IWARP_REQ_MAX_INLINE_DATA_SIZE: i32 = 128;
pub const IWARP_REQ_MAX_SINGLE_SQ_WQE_SIZE: i32 = 176;

pub const IWARP_MAX_QPS: u32 = 64 * 1024;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
