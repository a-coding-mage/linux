/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Houlong Wei <houlong.wei@mediatek.com>
 *
 */

/* GCE HW thread priority */
pub const CMDQ_THR_PRIO_LOWEST: u32 = 0;
pub const CMDQ_THR_PRIO_HIGHEST: u32 = 1;

/* GCE SUBSYS */
pub const SUBSYS_1400XXXX: u32 = 1;
pub const SUBSYS_1401XXXX: u32 = 2;
pub const SUBSYS_1402XXXX: u32 = 3;

/* GCE HW EVENT */
pub const CMDQ_EVENT_DISP_OVL0_SOF: u32 = 11;
pub const CMDQ_EVENT_DISP_OVL1_SOF: u32 = 12;
pub const CMDQ_EVENT_DISP_RDMA0_SOF: u32 = 13;
pub const CMDQ_EVENT_DISP_RDMA1_SOF: u32 = 14;
pub const CMDQ_EVENT_DISP_RDMA2_SOF: u32 = 15;
pub const CMDQ_EVENT_DISP_WDMA0_SOF: u32 = 16;
pub const CMDQ_EVENT_DISP_WDMA1_SOF: u32 = 17;
pub const CMDQ_EVENT_DISP_OVL0_EOF: u32 = 39;
pub const CMDQ_EVENT_DISP_OVL1_EOF: u32 = 40;
pub const CMDQ_EVENT_DISP_RDMA0_EOF: u32 = 41;
pub const CMDQ_EVENT_DISP_RDMA1_EOF: u32 = 42;
pub const CMDQ_EVENT_DISP_RDMA2_EOF: u32 = 43;
pub const CMDQ_EVENT_DISP_WDMA0_EOF: u32 = 44;
pub const CMDQ_EVENT_DISP_WDMA1_EOF: u32 = 45;
pub const CMDQ_EVENT_MUTEX0_STREAM_EOF: u32 = 53;
pub const CMDQ_EVENT_MUTEX1_STREAM_EOF: u32 = 54;
pub const CMDQ_EVENT_MUTEX2_STREAM_EOF: u32 = 55;
pub const CMDQ_EVENT_MUTEX3_STREAM_EOF: u32 = 56;
pub const CMDQ_EVENT_MUTEX4_STREAM_EOF: u32 = 57;
pub const CMDQ_EVENT_DISP_RDMA0_UNDERRUN: u32 = 63;
pub const CMDQ_EVENT_DISP_RDMA1_UNDERRUN: u32 = 64;
pub const CMDQ_EVENT_DISP_RDMA2_UNDERRUN: u32 = 65;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
