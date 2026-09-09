/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]
pub type __u8=u8; pub type __s8=i8; pub type __le16=u16; pub type __le32=u32; pub type __le64=u64; pub type bdaddr_t=[u8;6];
macro_rules! flex { ($t:ty) => { [$t;0] }; }
#[repr(C,packed)] pub struct mgmt_hdr{pub opcode:__le16,pub index:__le16,pub len:__le16}
#[repr(C,packed)] pub struct mgmt_tlv{pub r#type:__le16,pub length:__u8,pub value:[__u8;0]}
#[repr(C,packed)] pub struct mgmt_addr_info{pub bdaddr:bdaddr_t,pub r#type:__u8}
pub const MGMT_INDEX_NONE:u16=0xffff; pub const MGMT_ADDR_INFO_SIZE:usize=7;
pub const MGMT_STATUS_SUCCESS:u8=0; pub const MGMT_STATUS_UNKNOWN_COMMAND:u8=1; pub const MGMT_STATUS_NOT_CONNECTED:u8=2; pub const MGMT_STATUS_FAILED:u8=3; pub const MGMT_STATUS_CONNECT_FAILED:u8=4; pub const MGMT_STATUS_AUTH_FAILED:u8=5; pub const MGMT_STATUS_NOT_PAIRED:u8=6; pub const MGMT_STATUS_NO_RESOURCES:u8=7; pub const MGMT_STATUS_TIMEOUT:u8=8; pub const MGMT_STATUS_ALREADY_CONNECTED:u8=9; pub const MGMT_STATUS_BUSY:u8=10; pub const MGMT_STATUS_REJECTED:u8=11; pub const MGMT_STATUS_NOT_SUPPORTED:u8=12; pub const MGMT_STATUS_INVALID_PARAMS:u8=13; pub const MGMT_STATUS_DISCONNECTED:u8=14; pub const MGMT_STATUS_NOT_POWERED:u8=15; pub const MGMT_STATUS_CANCELLED:u8=16; pub const MGMT_STATUS_INVALID_INDEX:u8=17; pub const MGMT_STATUS_RFKILLED:u8=18; pub const MGMT_STATUS_ALREADY_PAIRED:u8=19; pub const MGMT_STATUS_PERMISSION_DENIED:u8=20;
pub const MGMT_MAX_NAME_LENGTH:usize=249; pub const MGMT_MAX_SHORT_NAME_LENGTH:usize=11;
#[repr(C,packed)] pub struct mgmt_rp_read_version{pub version:__u8,pub revision:__le16}
#[repr(C,packed)] pub struct mgmt_rp_read_commands{pub num_commands:__le16,pub num_events:__le16,pub opcodes:flex!(__le16)}
#[repr(C,packed)] pub struct mgmt_rp_read_index_list{pub num_controllers:__le16,pub index:flex!(__le16)}
#[repr(C,packed)] pub struct mgmt_rp_read_info{pub bdaddr:bdaddr_t,pub version:__u8,pub manufacturer:__le16,pub supported_settings:__le32,pub current_settings:__le32,pub dev_class:[__u8;3],pub name:[__u8;MGMT_MAX_NAME_LENGTH],pub short_name:[__u8;MGMT_MAX_SHORT_NAME_LENGTH]}
#[repr(C,packed)] pub struct mgmt_mode{pub val:__u8}
#[repr(C,packed)] pub struct mgmt_cp_set_discoverable{pub val:__u8,pub timeout:__le16}
#[repr(C,packed)] pub struct mgmt_cp_set_dev_class{pub major:__u8,pub minor:__u8}
#[repr(C,packed)] pub struct mgmt_cp_set_local_name{pub name:[__u8;MGMT_MAX_NAME_LENGTH],pub short_name:[__u8;MGMT_MAX_SHORT_NAME_LENGTH]}
#[repr(C,packed)] pub struct mgmt_cp_add_uuid{pub uuid:[__u8;16],pub svc_hint:__u8}
#[repr(C,packed)] pub struct mgmt_cp_remove_uuid{pub uuid:[__u8;16]}
#[repr(C,packed)] pub struct mgmt_link_key_info{pub addr:mgmt_addr_info,pub r#type:__u8,pub val:[__u8;16],pub pin_len:__u8}
#[repr(C,packed)] pub struct mgmt_ltk_info{pub addr:mgmt_addr_info,pub r#type:__u8,pub initiator:__u8,pub enc_size:__u8,pub ediv:__le16,pub rand:__le64,pub val:[__u8;16]}
#[repr(C,packed)] pub struct mgmt_cp_load_link_keys{pub debug_keys:__u8,pub key_count:__le16,pub keys:flex!(mgmt_link_key_info)}
#[repr(C,packed)] pub struct mgmt_cp_load_long_term_keys{pub key_count:__le16,pub keys:flex!(mgmt_ltk_info)}
#[repr(C,packed)] pub struct mgmt_cp_disconnect{pub addr:mgmt_addr_info}
#[repr(C,packed)] pub struct mgmt_rp_disconnect{pub addr:mgmt_addr_info}
#[repr(C,packed)] pub struct mgmt_rp_get_connections{pub conn_count:__le16,pub addr:flex!(mgmt_addr_info)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
