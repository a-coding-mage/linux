/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2015 Mellanox Technologies. All rights reserved.
 */

// C header dependencies (__le16 and u8) are supplied by the surrounding
// translation unit.

pub const NVME_RDMA_IP_PORT: u32 = 4420;

pub const NVME_RDMA_MAX_QUEUE_SIZE: u32 = 256;
pub const NVME_RDMA_MAX_METADATA_QUEUE_SIZE: u32 = 128;
pub const NVME_RDMA_DEFAULT_QUEUE_SIZE: u32 = 128;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nvme_rdma_cm_fmt {
    NVME_RDMA_CM_FMT_1_0 = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nvme_rdma_cm_status {
    NVME_RDMA_CM_INVALID_LEN = 0x01,
    NVME_RDMA_CM_INVALID_RECFMT = 0x02,
    NVME_RDMA_CM_INVALID_QID = 0x03,
    NVME_RDMA_CM_INVALID_HSQSIZE = 0x04,
    NVME_RDMA_CM_INVALID_HRQSIZE = 0x05,
    NVME_RDMA_CM_NO_RSC = 0x06,
    NVME_RDMA_CM_INVALID_IRD = 0x07,
    NVME_RDMA_CM_INVALID_ORD = 0x08,
    NVME_RDMA_CM_INVALID_CNTLID = 0x09,
}

pub unsafe fn nvme_rdma_cm_msg(status: nvme_rdma_cm_status) -> *const core::ffi::c_char {
    match status {
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_LEN => b"invalid length\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_RECFMT => b"invalid record format\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_QID => b"invalid queue ID\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_HSQSIZE => b"invalid host SQ size\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_HRQSIZE => b"invalid host RQ size\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_NO_RSC => b"resource not found\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_IRD => b"invalid IRD\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_ORD => b"Invalid ORD\0".as_ptr() as *const core::ffi::c_char,
        nvme_rdma_cm_status::NVME_RDMA_CM_INVALID_CNTLID => b"invalid controller ID\0".as_ptr() as *const core::ffi::c_char,
        _ => b"unrecognized reason\0".as_ptr() as *const core::ffi::c_char,
    }
}

/**
 * struct nvme_rdma_cm_req - rdma connect request
 *
 * @recfmt:        format of the RDMA Private Data
 * @qid:           queue Identifier for the Admin or I/O Queue
 * @hrqsize:       host receive queue size to be created
 * @hsqsize:       host send queue size to be created
 */
#[repr(C)]
pub struct nvme_rdma_cm_req {
    pub recfmt: __le16,
    pub qid: __le16,
    pub hrqsize: __le16,
    pub hsqsize: __le16,
    pub cntlid: __le16,
    pub rsvd: [u8; 22],
}

/**
 * struct nvme_rdma_cm_rep - rdma connect reply
 *
 * @recfmt:        format of the RDMA Private Data
 * @crqsize:       controller receive queue size
 */
#[repr(C)]
pub struct nvme_rdma_cm_rep {
    pub recfmt: __le16,
    pub crqsize: __le16,
    pub rsvd: [u8; 28],
}

/**
 * struct nvme_rdma_cm_rej - rdma connect reject
 *
 * @recfmt:        format of the RDMA Private Data
 * @sts:           error status for the associated connect request
 */
#[repr(C)]
pub struct nvme_rdma_cm_rej {
    pub recfmt: __le16,
    pub sts: __le16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
