/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Translated from aie4_msg_priv.h. Linux sizes/types dependencies are represented
// by equivalent fixed-width Rust types below.

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Aie4MsgOpcode {
    AIE4_MSG_OP_SUSPEND = 0x10003,
    AIE4_MSG_OP_ATTACH_WORK_BUFFER = 0x1000D,
    AIE4_MSG_OP_CREATE_VFS = 0x20001,
    AIE4_MSG_OP_DESTROY_VFS = 0x20002,
    AIE4_MSG_OP_CREATE_PARTITION = 0x30001,
    AIE4_MSG_OP_DESTROY_PARTITION = 0x30002,
    AIE4_MSG_OP_CREATE_HW_CONTEXT = 0x30003,
    AIE4_MSG_OP_DESTROY_HW_CONTEXT = 0x30004,
    AIE4_MSG_OP_AIE_TILE_INFO = 0x30006,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Aie4MsgStatus {
    AIE4_MSG_STATUS_SUCCESS = 0x0,
    AIE4_MSG_STATUS_ERROR = 0x1,
    AIE4_MSG_STATUS_NOTSUPP = 0x2,
    MAX_AIE4_MSG_STATUS_CODE = 0x4,
}

#[repr(C, packed)]
pub struct Aie4MsgSuspendReq { pub rsvd: u32 }

#[repr(C, packed)]
pub struct Aie4MsgSuspendResp { pub status: Aie4MsgStatus }

#[repr(C, packed)]
pub struct Aie4MsgCreateVfsReq { pub vf_cnt: u32 }

#[repr(C, packed)]
pub struct Aie4MsgCreateVfsResp { pub status: Aie4MsgStatus }

#[repr(C, packed)]
pub struct Aie4MsgDestroyVfsReq { pub rsvd: u32 }

#[repr(C, packed)]
pub struct Aie4MsgDestroyVfsResp { pub status: Aie4MsgStatus }

#[repr(C, packed)]
pub struct Aie4MsgCreatePartitionReq {
    pub partition_col_start: u32,
    pub partition_col_count: u32,
}

#[repr(C, packed)]
pub struct Aie4MsgCreatePartitionResp {
    pub status: Aie4MsgStatus,
    pub partition_id: u32,
}

#[repr(C, packed)]
pub struct Aie4MsgDestroyPartitionReq { pub partition_id: u32 }

#[repr(C, packed)]
pub struct Aie4MsgDestroyPartitionResp { pub status: Aie4MsgStatus }

#[repr(C, packed)]
pub struct Aie4MsgCreateHwContextReq {
    pub partition_id: u32,
    pub request_num_tiles: u32,
    pub hsa_addr_high: u32,
    pub hsa_addr_low: u32,
    pub pasid: u32,
    pub priority_band: u32,
}

pub const AIE4_MSG_PASID: u32 = (1u32 << 20) - 1;
pub const AIE4_MSG_PASID_VLD: u32 = 1u32 << 31;

#[repr(C, packed)]
pub struct Aie4MsgCreateHwContextResp {
    pub status: Aie4MsgStatus,
    pub hw_context_id: u32,
    pub doorbell_offset: u32,
    pub job_complete_msix_idx: u32,
}

#[repr(C, packed)]
pub struct Aie4MsgDestroyHwContextReq {
    pub hw_context_id: u32,
    pub resvd1: u32,
}

#[repr(C, packed)]
pub struct Aie4MsgDestroyHwContextResp { pub status: Aie4MsgStatus }

#[repr(C, packed)]
pub struct Aie4TileInfo {
    pub size: u32,
    pub major: u16,
    pub minor: u16,
    pub cols: u16,
    pub rows: u16,
    pub core_rows: u16,
    pub mem_rows: u16,
    pub shim_rows: u16,
    pub core_row_start: u16,
    pub mem_row_start: u16,
    pub shim_row_start: u16,
    pub core_dma_channels: u16,
    pub mem_dma_channels: u16,
    pub shim_dma_channels: u16,
    pub core_locks: u16,
    pub mem_locks: u16,
    pub shim_locks: u16,
    pub core_events: u16,
    pub mem_events: u16,
    pub shim_events: u16,
    pub resvd: u16,
}

#[repr(C, packed)]
pub struct Aie4MsgAie4TileInfoReq { pub resvd: u32 }

#[repr(C, packed)]
pub struct Aie4MsgAie4TileInfoResp {
    pub status: Aie4MsgStatus,
    pub info: Aie4TileInfo,
}

pub const AIE4_WORK_BUFFER_MIN_SIZE: usize = 4 * 1024 * 1024;

#[repr(C, packed)]
pub struct Aie4MsgAttachWorkBufferReq {
    pub buff_addr: u64,
    pub reserved: u32,
    pub buff_size: u32,
}

#[repr(C, packed)]
pub struct Aie4MsgAttachWorkBufferResp { pub status: Aie4MsgStatus }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
