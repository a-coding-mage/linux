/*
 * Copyright (c) 2005 Cisco Systems.  All rights reserved.
 *
 * This software is available under the GNU General Public License (GPL)
 * Version 2 or the OpenIB.org BSD license.
 *
 * Structures and constants for the SCSI RDMA Protocol (SRP), based on
 * draft Revision 16a of the SRP standard.
 */

// Dependencies corresponding to <linux/types.h> and <scsi/scsi.h> are
// supplied by the surrounding translation unit.

pub const SRP_LOGIN_REQ: u32 = 0x00;
pub const SRP_TSK_MGMT: u32 = 0x01;
pub const SRP_CMD: u32 = 0x02;
pub const SRP_I_LOGOUT: u32 = 0x03;
pub const SRP_LOGIN_RSP: u32 = 0xc0;
pub const SRP_RSP: u32 = 0xc1;
pub const SRP_LOGIN_REJ: u32 = 0xc2;
pub const SRP_T_LOGOUT: u32 = 0x80;
pub const SRP_CRED_REQ: u32 = 0x81;
pub const SRP_AER_REQ: u32 = 0x82;
pub const SRP_CRED_RSP: u32 = 0x41;
pub const SRP_AER_RSP: u32 = 0x42;

pub const SRP_BUF_FORMAT_DIRECT: u32 = 1 << 1;
pub const SRP_BUF_FORMAT_INDIRECT: u32 = 1 << 2;
pub const SRP_NO_DATA_DESC: u32 = 0;
pub const SRP_DATA_DESC_DIRECT: u32 = 1;
pub const SRP_DATA_DESC_INDIRECT: u32 = 2;
pub const SRP_DATA_DESC_IMM: u32 = 3; // new in SRP2

pub const SRP_TSK_ABORT_TASK: u32 = 0x01;
pub const SRP_TSK_ABORT_TASK_SET: u32 = 0x02;
pub const SRP_TSK_CLEAR_TASK_SET: u32 = 0x04;
pub const SRP_TSK_LUN_RESET: u32 = 0x08;
pub const SRP_TSK_CLEAR_ACA: u32 = 0x40;

#[repr(u32)]
pub enum srp_login_rej_reason {
    SRP_LOGIN_REJ_UNABLE_ESTABLISH_CHANNEL = 0x00010000,
    SRP_LOGIN_REJ_INSUFFICIENT_RESOURCES = 0x00010001,
    SRP_LOGIN_REJ_REQ_IT_IU_LENGTH_TOO_LARGE = 0x00010002,
    SRP_LOGIN_REJ_UNABLE_ASSOCIATE_CHANNEL = 0x00010003,
    SRP_LOGIN_REJ_UNSUPPORTED_DESCRIPTOR_FMT = 0x00010004,
    SRP_LOGIN_REJ_MULTI_CHANNEL_UNSUPPORTED = 0x00010005,
    SRP_LOGIN_REJ_CHANNEL_LIMIT_REACHED = 0x00010006,
}

pub const SRP_REV10_IB_IO_CLASS: u32 = 0xff00;
pub const SRP_REV16A_IB_IO_CLASS: u32 = 0x0100;

#[repr(C)]
pub struct srp_direct_buf { pub va: __be64, pub key: __be32, pub len: __be32 }

#[repr(C, packed)]
pub struct srp_indirect_buf {
    pub table_desc: srp_direct_buf,
    pub len: __be32,
    pub desc_list: [srp_direct_buf; 0],
}

#[repr(C)]
pub struct srp_imm_buf { pub len: __be32 }

pub const SRP_MULTICHAN_SINGLE: u32 = 0;
pub const SRP_MULTICHAN_MULTI: u32 = 1;
pub const SRP_IMMED_REQUESTED: u32 = 0x80; // new in SRP2

#[repr(C)]
pub struct srp_login_req {
    pub opcode: u8, pub reserved1: [u8; 7], pub tag: u64,
    pub req_it_iu_len: __be32, pub reserved2: [u8; 4], pub req_buf_fmt: __be16,
    pub req_flags: u8, pub reserved3: [u8; 1], pub imm_data_offset: __be16,
    pub reserved4: [u8; 2], pub initiator_port_id: [u8; 16], pub target_port_id: [u8; 16],
}

#[repr(C)]
pub struct srp_login_req_rdma {
    pub tag: u64, pub req_buf_fmt: __be16, pub req_flags: u8, pub opcode: u8,
    pub req_it_iu_len: __be32, pub initiator_port_id: [u8; 16],
    pub target_port_id: [u8; 16], pub imm_data_offset: __be16, pub reserved: [u8; 6],
}

pub const SRP_LOGIN_RSP_MULTICHAN_NO_CHAN: u32 = 0x0;
pub const SRP_LOGIN_RSP_MULTICHAN_TERMINATED: u32 = 0x1;
pub const SRP_LOGIN_RSP_MULTICHAN_MAINTAINED: u32 = 0x2;
pub const SRP_LOGIN_RSP_IMMED_SUPP: u32 = 0x80; // new in SRP2

#[repr(C, packed)]
pub struct srp_login_rsp {
    pub opcode: u8, pub reserved1: [u8; 3], pub req_lim_delta: __be32, pub tag: u64,
    pub max_it_iu_len: __be32, pub max_ti_iu_len: __be32, pub buf_fmt: __be16,
    pub rsp_flags: u8, pub reserved2: [u8; 25],
}

#[repr(C)]
pub struct srp_login_rej {
    pub opcode: u8, pub reserved1: [u8; 3], pub reason: __be32, pub tag: u64,
    pub reserved2: [u8; 8], pub buf_fmt: __be16, pub reserved3: [u8; 6],
}

#[repr(C)] pub struct srp_i_logout { pub opcode: u8, pub reserved: [u8; 7], pub tag: u64 }
#[repr(C)] pub struct srp_t_logout { pub opcode: u8, pub sol_not: u8, pub reserved: [u8; 2], pub reason: __be32, pub tag: u64 }

#[repr(C)]
pub struct srp_tsk_mgmt {
    pub opcode: u8, pub sol_not: u8, pub reserved1: [u8; 6], pub tag: u64,
    pub reserved2: [u8; 4], pub lun: scsi_lun, pub reserved3: [u8; 2],
    pub tsk_mgmt_func: u8, pub reserved4: u8, pub task_tag: u64, pub reserved5: [u8; 8],
}

#[repr(C)]
pub struct srp_cmd {
    pub opcode: u8, pub sol_not: u8, pub reserved1: [u8; 3], pub buf_fmt: u8,
    pub data_out_desc_cnt: u8, pub data_in_desc_cnt: u8, pub tag: u64,
    pub reserved2: [u8; 4], pub lun: scsi_lun, pub reserved3: u8, pub task_attr: u8,
    pub reserved4: u8, pub add_cdb_len: u8, pub cdb: [u8; 16], pub add_data: [u8; 0],
}

pub const SRP_RSP_FLAG_RSPVALID: u32 = 1 << 0;
pub const SRP_RSP_FLAG_SNSVALID: u32 = 1 << 1;
pub const SRP_RSP_FLAG_DOOVER: u32 = 1 << 2;
pub const SRP_RSP_FLAG_DOUNDER: u32 = 1 << 3;
pub const SRP_RSP_FLAG_DIOVER: u32 = 1 << 4;
pub const SRP_RSP_FLAG_DIUNDER: u32 = 1 << 5;

#[repr(C, packed)]
pub struct srp_rsp {
    pub opcode: u8, pub sol_not: u8, pub reserved1: [u8; 2], pub req_lim_delta: __be32,
    pub tag: u64, pub reserved2: [u8; 2], pub flags: u8, pub status: u8,
    pub data_out_res_cnt: __be32, pub data_in_res_cnt: __be32, pub sense_data_len: __be32,
    pub resp_data_len: __be32, pub data: [u8; 0],
}

#[repr(C)] pub struct srp_cred_req { pub opcode: u8, pub sol_not: u8, pub reserved: [u8; 2], pub req_lim_delta: __be32, pub tag: u64 }
#[repr(C)] pub struct srp_cred_rsp { pub opcode: u8, pub reserved: [u8; 7], pub tag: u64 }

#[repr(C, packed)]
pub struct srp_aer_req {
    pub opcode: u8, pub sol_not: u8, pub reserved: [u8; 2], pub req_lim_delta: __be32,
    pub tag: u64, pub reserved2: u32, pub lun: scsi_lun, pub sense_data_len: __be32,
    pub reserved3: u32, pub sense_data: [u8; 0],
}

#[repr(C)] pub struct srp_aer_rsp { pub opcode: u8, pub reserved: [u8; 7], pub tag: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
