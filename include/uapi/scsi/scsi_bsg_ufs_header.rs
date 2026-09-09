/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * UFS Transport SGIO v4 BSG Message Support
 *
 * Copyright (C) 2011-2013 Samsung India Software Operations
 * Copyright (C) 2018 Western Digital Corporation
 */

/* This file is intended to be included by both kernel and user space. */

pub const UFS_CDB_SIZE: usize = 16;
/* uic commands are 4DW long, per UFSHCI V2.1 paragraph 5.6.1 */
pub const UIC_CMD_SIZE: usize = core::mem::size_of::<u32>() * 4;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UfsBsgMsgCode {
    UPIU_TRANSACTION_UIC_CMD = 0x1F,
    UPIU_TRANSACTION_ARPMB_CMD,
}

/* UFS RPMB Request Message Types */
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UfsRpmbOpType {
    UFS_RPMB_WRITE_KEY = 0x01,
    UFS_RPMB_READ_CNT = 0x02,
    UFS_RPMB_WRITE = 0x03,
    UFS_RPMB_READ = 0x04,
    UFS_RPMB_READ_RESP = 0x05,
    UFS_RPMB_SEC_CONF_WRITE = 0x06,
    UFS_RPMB_SEC_CONF_READ = 0x07,
    UFS_RPMB_PURGE_ENABLE = 0x08,
    UFS_RPMB_PURGE_STATUS_READ = 0x09,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtPupiHeaderDwords {
    pub dword_0: u32,
    pub dword_1: u32,
    pub dword_2: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UtPupiHeaderFields {
    pub transaction_code: u8,
    pub flags: u8,
    pub lun: u8,
    pub task_tag: u8,
    /* On big endian: iid occupies the high nibble; on little endian, the low nibble. */
    pub iid_and_command_set_type: u8,
    pub tm_function_or_query_function: u8,
    pub response: u8,
    pub status: u8,
    pub ehs_length: u8,
    pub device_information: u8,
    pub data_segment_length: u16,
}

#[repr(C)]
pub union UtPupiHeader {
    pub dwords: UtPupiHeaderDwords,
    pub fields: UtPupiHeaderFields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UtPupiQueryFunction {
    pub tm_function: u8,
    pub query_function: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtpUpiuQuery {
    pub opcode: u8,
    pub idn: u8,
    pub index: u8,
    pub selector: u8,
    pub reserved_osf: u16,
    pub length: u16,
    pub value: u32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtpUpiuQueryV4_0 {
    pub opcode: u8,
    pub idn: u8,
    pub index: u8,
    pub selector: u8,
    pub osf3: u8,
    pub osf4: u8,
    pub osf5: u16,
    pub osf6: u32,
    pub osf7: u32,
    /* private: */
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtpUpiuCmd {
    pub exp_data_transfer_len: u32,
    pub cdb: [u8; UFS_CDB_SIZE],
}

#[repr(C)]
pub union UtpUpiuReqPayload {
    pub sc: UtpUpiuCmd,
    pub qr: UtpUpiuQuery,
    pub uc: UtpUpiuQuery,
}

#[repr(C)]
pub struct UtpUpiuReq {
    pub header: UtPupiHeader,
    pub payload: UtpUpiuReqPayload,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UfsArpmbMeta {
    pub req_resp_type: u16,
    pub nonce: [u8; 16],
    pub write_counter: u32,
    pub addr_lun: u16,
    pub block_count: u16,
    pub result: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UfsEhs {
    pub length: u8,
    pub ehs_type: u8,
    pub ehssub_type: u16,
    pub meta: UfsArpmbMeta,
    pub mac_key: [u8; 32],
}

/* request (CDB) structure of the sg_io_v4 */
#[repr(C)]
pub struct UfsBsgRequest {
    pub msgcode: u32,
    pub upiu_req: UtpUpiuReq,
}

/* response (request sense data) structure of the sg_io_v4 */
#[repr(C)]
pub struct UfsBsgReply {
    /*
     * The completion result. Result exists in two forms:
     * if negative, it is an -Exxx system errno value. There will
     * be no further reply information supplied.
     * else, it's the 4-byte scsi error result, with driver, host,
     * msg and status fields. The per-msgcode reply structure
     * will contain valid data.
     */
    pub result: i32,

    /* If there was reply_payload, how much was received? */
    pub reply_payload_rcv_len: u32,

    pub upiu_rsp: UtpUpiuReq,
}

#[repr(C)]
pub struct UfsRpmbRequest {
    pub bsg_request: UfsBsgRequest,
    pub ehs_req: UfsEhs,
}

#[repr(C)]
pub struct UfsRpmbReply {
    pub bsg_reply: UfsBsgReply,
    pub ehs_rsp: UfsEhs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
