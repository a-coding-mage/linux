/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 */

/*
 * This file contains defines, structures, etc. that are used
 * to communicate between kernel and user code.
 */

// Linux UAPI header dependencies: <linux/types.h>, <rdma/rdma_user_ioctl.h>

/* This version number is given to the driver by the user code during
 * initialization in the spu_userversion field of hfi1_user_info. */
pub const HFI1_USER_SWMAJOR: u32 = 6;
pub const HFI1_USER_SWMINOR: u32 = 3;
pub const HFI1_SWMAJOR_SHIFT: u32 = 16;

pub const HFI1_CAP_DMA_RTAIL: u64 = 1u64 << 0;
pub const HFI1_CAP_SDMA: u64 = 1u64 << 1;
pub const HFI1_CAP_SDMA_AHG: u64 = 1u64 << 2;
pub const HFI1_CAP_EXTENDED_PSN: u64 = 1u64 << 3;
pub const HFI1_CAP_HDRSUPP: u64 = 1u64 << 4;
pub const HFI1_CAP_TID_RDMA: u64 = 1u64 << 5;
pub const HFI1_CAP_USE_SDMA_HEAD: u64 = 1u64 << 6;
pub const HFI1_CAP_MULTI_PKT_EGR: u64 = 1u64 << 7;
pub const HFI1_CAP_NODROP_RHQ_FULL: u64 = 1u64 << 8;
pub const HFI1_CAP_NODROP_EGR_FULL: u64 = 1u64 << 9;
pub const HFI1_CAP_TID_UNMAP: u64 = 1u64 << 10;
pub const HFI1_CAP_PRINT_UNIMPL: u64 = 1u64 << 11;
pub const HFI1_CAP_ALLOW_PERM_JKEY: u64 = 1u64 << 12;
pub const HFI1_CAP_NO_INTEGRITY: u64 = 1u64 << 13;
pub const HFI1_CAP_PKEY_CHECK: u64 = 1u64 << 14;
pub const HFI1_CAP_STATIC_RATE_CTRL: u64 = 1u64 << 15;
pub const HFI1_CAP_OPFN: u64 = 1u64 << 16;
pub const HFI1_CAP_SDMA_HEAD_CHECK: u64 = 1u64 << 17;
pub const HFI1_CAP_EARLY_CREDIT_RETURN: u64 = 1u64 << 18;
pub const HFI1_CAP_AIP: u64 = 1u64 << 19;

pub const HFI1_RCVHDR_ENTSIZE_2: u64 = 1u64 << 0;
pub const HFI1_RCVHDR_ENTSIZE_16: u64 = 1u64 << 1;
pub const HFI1_RCVDHR_ENTSIZE_32: u64 = 1u64 << 2;

pub const _HFI1_EVENT_FROZEN_BIT: u32 = 0;
pub const _HFI1_EVENT_LINKDOWN_BIT: u32 = 1;
pub const _HFI1_EVENT_LID_CHANGE_BIT: u32 = 2;
pub const _HFI1_EVENT_LMC_CHANGE_BIT: u32 = 3;
pub const _HFI1_EVENT_SL2VL_CHANGE_BIT: u32 = 4;
pub const _HFI1_EVENT_TID_MMU_NOTIFY_BIT: u32 = 5;
pub const _HFI1_MAX_EVENT_BIT: u32 = _HFI1_EVENT_TID_MMU_NOTIFY_BIT;

pub const HFI1_EVENT_FROZEN: u64 = 1u64 << _HFI1_EVENT_FROZEN_BIT;
pub const HFI1_EVENT_LINKDOWN: u64 = 1u64 << _HFI1_EVENT_LINKDOWN_BIT;
pub const HFI1_EVENT_LID_CHANGE: u64 = 1u64 << _HFI1_EVENT_LID_CHANGE_BIT;
pub const HFI1_EVENT_LMC_CHANGE: u64 = 1u64 << _HFI1_EVENT_LMC_CHANGE_BIT;
pub const HFI1_EVENT_SL2VL_CHANGE: u64 = 1u64 << _HFI1_EVENT_SL2VL_CHANGE_BIT;
pub const HFI1_EVENT_TID_MMU_NOTIFY: u64 = 1u64 << _HFI1_EVENT_TID_MMU_NOTIFY_BIT;

pub const HFI1_STATUS_INITTED: u32 = 0x1;
pub const HFI1_STATUS_CHIP_PRESENT: u32 = 0x20;
pub const HFI1_STATUS_IB_READY: u32 = 0x40;
pub const HFI1_STATUS_IB_CONF: u32 = 0x80;
pub const HFI1_STATUS_HWERROR: u32 = 0x200;

pub const HFI1_MAX_SHARED_CTXTS: u32 = 8;
pub const HFI1_POLL_TYPE_ANYRCV: u32 = 0x0;
pub const HFI1_POLL_TYPE_URGENT: u32 = 0x1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hfi1_sdma_comp_state { FREE = 0, QUEUED, COMPLETE, ERROR }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_sdma_comp_entry { pub status: u32, pub errcode: u32 }

#[repr(C)]
pub struct hfi1_status {
    pub dev: u64,
    pub port: u64,
    pub freezemsg: [core::ffi::c_char; 0],
}

pub const HFI1_SDMA_REQ_VERSION_MASK: u16 = 0xF;
pub const HFI1_SDMA_REQ_VERSION_SHIFT: u16 = 0x0;
pub const HFI1_SDMA_REQ_OPCODE_MASK: u16 = 0xF;
pub const HFI1_SDMA_REQ_OPCODE_SHIFT: u16 = 0x4;
pub const HFI1_SDMA_REQ_IOVCNT_MASK: u16 = 0xFF;
pub const HFI1_SDMA_REQ_IOVCNT_SHIFT: u16 = 0x8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sdma_req_opcode { EXPECTED = 0, EAGER }

#[repr(C, packed)]
pub struct sdma_req_info {
    pub ctrl: u16,
    pub npkts: u16,
    pub fragsize: u16,
    pub comp_idx: u16,
}

#[repr(C, packed)]
pub struct hfi1_kdeth_header {
    pub ver_tid_offset: u32,
    pub jkey: u16,
    pub hcrc: u16,
    pub swdata: [u32; 7],
}

#[repr(C, packed)]
pub struct hfi1_pkt_header {
    pub pbc: [u16; 4],
    pub lrh: [u16; 4],
    pub bth: [u32; 3],
    pub kdeth: hfi1_kdeth_header,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hfi1_ureg {
    ur_rcvhdrtail = 0,
    ur_rcvhdrhead = 1,
    ur_rcvegrindextail = 2,
    ur_rcvegrindexhead = 3,
    ur_rcvegroffsettail = 4,
    ur_maxreg,
    ur_rcvtidflowtable = 256,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
