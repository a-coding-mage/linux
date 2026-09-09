/* SPDX-License-Identifier: GPL-2.0-or-later */
/*****************************************************************************/
/* srp.h -- SCSI RDMA Protocol definitions                                   */
/*                                                                           */
/* Written By: Colin Devilbis, IBM Corporation                               */
/*                                                                           */
/* Copyright (C) 2003 IBM Corporation                                        */
/*                                                                           */
/* This file contains structures and definitions for IBM RPA (RS/6000        */
/* platform architecture) implementation of the SRP (SCSI RDMA Protocol)     */
/* standard.  SRP is used on IBM iSeries and pSeries platforms to send SCSI  */
/* commands between logical partitions.                                      */
/*****************************************************************************/

use core::mem::ManuallyDrop;

pub const SRP_VERSION: &str = "16.a";
pub const SRP_MAX_IU_LEN: usize = 256;
pub const SRP_MAX_LOC_LEN: usize = 32;

/* Types supplied by the SRP dependency. */
extern "Rust" {
    type srp_login_req;
    type srp_login_rsp;
    type srp_login_rej;
    type srp_i_logout;
    type srp_t_logout;
    type srp_tsk_mgmt;
    type srp_cmd;
    type srp_rsp;
}

#[repr(C)]
pub union srp_iu {
    pub login_req: ManuallyDrop<srp_login_req>,
    pub login_rsp: ManuallyDrop<srp_login_rsp>,
    pub login_rej: ManuallyDrop<srp_login_rej>,
    pub i_logout: ManuallyDrop<srp_i_logout>,
    pub t_logout: ManuallyDrop<srp_t_logout>,
    pub tsk_mgmt: ManuallyDrop<srp_tsk_mgmt>,
    pub cmd: ManuallyDrop<srp_cmd>,
    pub rsp: ManuallyDrop<srp_rsp>,
    pub reserved: [u8; SRP_MAX_IU_LEN],
}

#[repr(i32)]
pub enum viosrp_crq_headers { VIOSRP_CRQ_FREE = 0x00, VIOSRP_CRQ_CMD_RSP = 0x80, VIOSRP_CRQ_INIT_RSP = 0xC0, VIOSRP_CRQ_XPORT_EVENT = 0xFF }
#[repr(i32)]
pub enum viosrp_crq_init_formats { VIOSRP_CRQ_INIT = 0x01, VIOSRP_CRQ_INIT_COMPLETE = 0x02 }
#[repr(i32)]
pub enum viosrp_crq_formats { VIOSRP_SRP_FORMAT = 0x01, VIOSRP_MAD_FORMAT = 0x02, VIOSRP_OS400_FORMAT = 0x03, VIOSRP_AIX_FORMAT = 0x04, VIOSRP_LINUX_FORMAT = 0x05, VIOSRP_INLINE_FORMAT = 0x06 }
#[repr(i32)]
pub enum viosrp_crq_status { VIOSRP_OK = 0x0, VIOSRP_NONRECOVERABLE_ERR = 0x1, VIOSRP_VIOLATES_MAX_XFER = 0x2, VIOSRP_PARTNER_PANIC = 0x3, VIOSRP_DEVICE_BUSY = 0x8, VIOSRP_ADAPTER_FAIL = 0x10, VIOSRP_OK2 = 0x99 }

#[repr(C)]
pub union viosrp_crq_high {
    pub high: u64,
    pub fields: viosrp_crq_fields,
}
#[repr(C)]
pub struct viosrp_crq_fields { pub valid: u8, pub format: u8, pub reserved: u8, pub status: u8, pub timeout: u16, pub IU_length: u16 }
#[repr(C)]
pub struct viosrp_crq { pub high: viosrp_crq_high, pub IU_data_ptr: u64 }

#[repr(i32)]
pub enum viosrp_mad_types { VIOSRP_EMPTY_IU_TYPE = 0x01, VIOSRP_ERROR_LOG_TYPE = 0x02, VIOSRP_ADAPTER_INFO_TYPE = 0x03, VIOSRP_CAPABILITIES_TYPE = 0x05, VIOSRP_ENABLE_FAST_FAIL = 0x08 }
#[repr(i32)]
pub enum viosrp_mad_status { VIOSRP_MAD_SUCCESS = 0x00, VIOSRP_MAD_NOT_SUPPORTED = 0xF1, VIOSRP_MAD_FAILED = 0xF7 }
#[repr(i32)]
pub enum viosrp_capability_type { MIGRATION_CAPABILITIES = 0x01, RESERVATION_CAPABILITIES = 0x02 }
#[repr(i32)]
pub enum viosrp_capability_support { SERVER_DOES_NOT_SUPPORTS_CAP = 0x0, SERVER_SUPPORTS_CAP = 0x01, SERVER_CAP_DATA = 0x02 }
#[repr(i32)]
pub enum viosrp_reserve_type { CLIENT_RESERVE_SCSI_2 = 0x01 }
#[repr(i32)]
pub enum viosrp_capability_flag { CLIENT_MIGRATED = 0x01, CLIENT_RECONNECT = 0x02, CAP_LIST_SUPPORTED = 0x04, CAP_LIST_DATA = 0x08 }

#[repr(C)] pub struct mad_common { pub type_: u32, pub status: u16, pub length: u16, pub tag: u64 }
#[repr(C)] pub struct viosrp_empty_iu { pub common: mad_common, pub buffer: u64, pub port: u32 }
#[repr(C)] pub struct viosrp_error_log { pub common: mad_common, pub buffer: u64 }
#[repr(C)] pub struct viosrp_adapter_info { pub common: mad_common, pub buffer: u64 }
#[repr(C)] pub struct viosrp_fast_fail { pub common: mad_common }
#[repr(C)] pub struct viosrp_capabilities { pub common: mad_common, pub buffer: u64 }
#[repr(C)] pub struct mad_capability_common { pub cap_type: u32, pub length: u16, pub server_support: u16 }
#[repr(C)] pub struct mad_reserve_cap { pub common: mad_capability_common, pub type_: u32 }
#[repr(C)] pub struct mad_migration_cap { pub common: mad_capability_common, pub ecl: u32 }
#[repr(C)] pub struct capabilities { pub flags: u32, pub name: [i8; SRP_MAX_LOC_LEN], pub loc: [i8; SRP_MAX_LOC_LEN], pub migration: mad_migration_cap, pub reserve: mad_reserve_cap }

#[repr(C)] pub union mad_iu { pub empty_iu: ManuallyDrop<viosrp_empty_iu>, pub error_log: ManuallyDrop<viosrp_error_log>, pub adapter_info: ManuallyDrop<viosrp_adapter_info>, pub fast_fail: ManuallyDrop<viosrp_fast_fail>, pub capabilities: ManuallyDrop<viosrp_capabilities> }
#[repr(C)] pub union viosrp_iu { pub srp: ManuallyDrop<srp_iu>, pub mad: ManuallyDrop<mad_iu> }

#[repr(C)] pub struct mad_adapter_info_data { pub srp_version: [i8; 8], pub partition_name: [i8; 96], pub partition_number: u32, pub mad_version: u32, pub os_type: u32, pub port_max_txu: [u32; 8] }

pub const SRP_MAD_VERSION_1: u32 = 1;
pub const SRP_MAD_OS_LINUX: u32 = 2;
pub const SRP_MAD_OS_AIX: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
