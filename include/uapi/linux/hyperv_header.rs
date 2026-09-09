/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Translated from hyperv.h. The original include and header guard are omitted.
 */

pub const UTIL_FW_MINOR: u32 = 0;
pub const UTIL_WS2K8_FW_MAJOR: u32 = 1;
pub const UTIL_WS2K8_FW_VERSION: u32 = (UTIL_WS2K8_FW_MAJOR << 16) | UTIL_FW_MINOR;
pub const UTIL_FW_MAJOR: u32 = 3;
pub const UTIL_FW_VERSION: u32 = (UTIL_FW_MAJOR << 16) | UTIL_FW_MINOR;

pub const VSS_OP_REGISTER: u32 = 128;
pub const VSS_OP_REGISTER1: u32 = 129;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hv_vss_op {
    VSS_OP_CREATE = 0,
    VSS_OP_DELETE,
    VSS_OP_HOT_BACKUP,
    VSS_OP_GET_DM_INFO,
    VSS_OP_BU_COMPLETE,
    VSS_OP_FREEZE,
    VSS_OP_THAW,
    VSS_OP_AUTO_RECOVER,
    VSS_OP_COUNT,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_vss_hdr { pub operation: u8, pub reserved: [u8; 7] }

pub const VSS_HBU_NO_AUTO_RECOVERY: u32 = 0x00000005;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_vss_check_feature { pub flags: u32 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_vss_check_dm_info { pub flags: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_vss_msg_hdr { pub vss_hdr: hv_vss_hdr, pub error: i32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_vss_msg_body { pub vss_cf: hv_vss_check_feature, pub dm_info: hv_vss_check_dm_info }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_vss_msg { pub hdr: hv_vss_msg_hdr, pub body: hv_vss_msg_body }

pub const FCOPY_VERSION_0: u32 = 0;
pub const FCOPY_VERSION_1: u32 = 1;
pub const FCOPY_CURRENT_VERSION: u32 = FCOPY_VERSION_1;
pub const W_MAX_PATH: usize = 260;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hv_fcopy_op { START_FILE_COPY = 0, WRITE_TO_FILE, COMPLETE_FCOPY, CANCEL_FCOPY }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_fcopy_hdr { pub operation: u32, pub service_id0: [u8; 16], pub service_id1: [u8; 16] }
pub const OVER_WRITE: u32 = 0x1;
pub const CREATE_PATH: u32 = 0x2;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_start_fcopy { pub hdr: hv_fcopy_hdr, pub file_name: [u16; W_MAX_PATH], pub path_name: [u16; W_MAX_PATH], pub copy_flags: u32, pub file_size: u64 }
pub const DATA_FRAGMENT: usize = 6 * 1024;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_do_fcopy { pub hdr: hv_fcopy_hdr, pub pad: u32, pub offset: u64, pub size: u32, pub data: [u8; DATA_FRAGMENT] }

pub const HV_KVP_EXCHANGE_MAX_VALUE_SIZE: usize = 2048;
pub const HV_KVP_EXCHANGE_MAX_KEY_SIZE: usize = 512;
pub const REG_SZ: u32 = 1;
pub const REG_U32: u32 = 4;
pub const REG_U64: u32 = 8;
pub const KVP_OP_REGISTER: u32 = 4;
pub const KVP_OP_REGISTER1: u32 = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hv_kvp_exchg_op { KVP_OP_GET = 0, KVP_OP_SET, KVP_OP_DELETE, KVP_OP_ENUMERATE, KVP_OP_GET_IP_INFO, KVP_OP_SET_IP_INFO, KVP_OP_COUNT }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum hv_kvp_exchg_pool { KVP_POOL_EXTERNAL = 0, KVP_POOL_GUEST, KVP_POOL_AUTO, KVP_POOL_AUTO_EXTERNAL, KVP_POOL_AUTO_INTERNAL, KVP_POOL_COUNT }

pub const HV_S_OK: u32 = 0x00000000;
pub const HV_E_FAIL: u32 = 0x80004005;
pub const HV_S_CONT: u32 = 0x80070103;
pub const HV_ERROR_NOT_SUPPORTED: u32 = 0x80070032;
pub const HV_ERROR_MACHINE_LOCKED: u32 = 0x800704F7;
pub const HV_ERROR_DEVICE_NOT_CONNECTED: u32 = 0x8007048F;
pub const HV_INVALIDARG: u32 = 0x80070057;
pub const HV_GUID_NOTFOUND: u32 = 0x80041002;
pub const HV_ERROR_ALREADY_EXISTS: u32 = 0x80070050;
pub const HV_ERROR_DISK_FULL: u32 = 0x80070070;
pub const ADDR_FAMILY_NONE: u8 = 0x00;
pub const ADDR_FAMILY_IPV4: u8 = 0x01;
pub const ADDR_FAMILY_IPV6: u8 = 0x02;
pub const MAX_ADAPTER_ID_SIZE: usize = 128;
pub const MAX_IP_ADDR_SIZE: usize = 1024;
pub const MAX_GATEWAY_SIZE: usize = 512;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_kvp_ipaddr_value { pub adapter_id: [u16; MAX_ADAPTER_ID_SIZE], pub addr_family: u8, pub dhcp_enabled: u8, pub ip_addr: [u16; MAX_IP_ADDR_SIZE], pub sub_net: [u16; MAX_IP_ADDR_SIZE], pub gate_way: [u16; MAX_GATEWAY_SIZE], pub dns_addr: [u16; MAX_IP_ADDR_SIZE] }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_kvp_hdr { pub operation: u8, pub pool: u8, pub pad: u16 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union hv_kvp_exchg_msg_value_union { pub value: [u8; HV_KVP_EXCHANGE_MAX_VALUE_SIZE], pub value_u32: u32, pub value_u64: u64 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_kvp_exchg_msg_value { pub value_type: u32, pub key_size: u32, pub value_size: u32, pub key: [u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE], pub value: hv_kvp_exchg_msg_value_union }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_enumerate { pub index: u32, pub data: hv_kvp_exchg_msg_value }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_get { pub data: hv_kvp_exchg_msg_value }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_set { pub data: hv_kvp_exchg_msg_value }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_delete { pub key_size: u32, pub key: [u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_register { pub version: [u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE] }
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_kvp_msg_hdr_union { pub kvp_hdr: hv_kvp_hdr, pub error: i32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_kvp_msg_body_union { pub kvp_get: hv_kvp_msg_get, pub kvp_set: hv_kvp_msg_set, pub kvp_delete: hv_kvp_msg_delete, pub kvp_enum_data: hv_kvp_msg_enumerate, pub kvp_ip_val: hv_kvp_ipaddr_value, pub kvp_register: hv_kvp_register }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg { pub hdr: hv_kvp_msg_hdr_union, pub body: hv_kvp_msg_body_union }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hv_kvp_ip_msg { pub operation: u8, pub pool: u8, pub kvp_ip_val: hv_kvp_ipaddr_value }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
