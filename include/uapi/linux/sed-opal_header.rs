/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright © 2016 Intel Corporation
 *
 * Authors:
 *    Rafael Antognolli <rafael.antognolli@intel.com>
 *    Scott  Bauer      <scott.bauer@intel.com>
 */

// Dependency: Linux __u8/__u16/__u32/__u64 types are represented by Rust's
// fixed-width unsigned integer types below. ioctl helpers (_IO, _IOW, _IOR)
// are supplied by the surrounding ABI bindings.

pub const OPAL_KEY_MAX: usize = 256;
pub const OPAL_MAX_LRS: usize = 9;

#[repr(u32)]
pub enum opal_mbr { OPAL_MBR_ENABLE = 0x0, OPAL_MBR_DISABLE = 0x01 }
#[repr(u32)]
pub enum opal_mbr_done_flag { OPAL_MBR_NOT_DONE = 0x0, OPAL_MBR_DONE = 0x01 }
#[repr(u32)]
pub enum opal_user {
    OPAL_ADMIN1 = 0x0, OPAL_USER1 = 0x01, OPAL_USER2 = 0x02,
    OPAL_USER3 = 0x03, OPAL_USER4 = 0x04, OPAL_USER5 = 0x05,
    OPAL_USER6 = 0x06, OPAL_USER7 = 0x07, OPAL_USER8 = 0x08,
    OPAL_USER9 = 0x09,
}
#[repr(u32)]
pub enum opal_lock_state { OPAL_RO = 0x01, OPAL_RW = 0x02, OPAL_LK = 0x04 }
#[repr(u32)]
pub enum opal_lock_flags { OPAL_SAVE_FOR_LOCK = 0x01 }
#[repr(u32)]
pub enum opal_key_type { OPAL_INCLUDED = 0, OPAL_KEYRING }

#[repr(C)]
pub struct opal_key { pub lr: u8, pub key_len: u8, pub key_type: u8, pub __align: [u8; 5], pub key: [u8; OPAL_KEY_MAX] }

#[repr(u32)]
pub enum opal_revert_lsp_opts { OPAL_PRESERVE = 0x01 }

#[repr(C)]
pub struct opal_lr_act { pub key: opal_key, pub sum: u32, pub num_lrs: u8, pub lr: [u8; OPAL_MAX_LRS], pub align: [u8; 2] }
#[repr(C)]
pub struct opal_lr_react { pub key: opal_key, pub new_admin_key: opal_key, pub num_lrs: u8, pub lr: [u8; OPAL_MAX_LRS], pub range_policy: u8, pub entire_table: u8, pub align: [u8; 4] }
#[repr(C)]
pub struct opal_session_info { pub sum: u32, pub who: u32, pub opal_key: opal_key }
#[repr(C)]
pub struct opal_user_lr_setup { pub range_start: u64, pub range_length: u64, pub RLE: u32, pub WLE: u32, pub session: opal_session_info }
#[repr(C)]
pub struct opal_lr_status { pub session: opal_session_info, pub range_start: u64, pub range_length: u64, pub RLE: u32, pub WLE: u32, pub l_state: u32, pub align: [u8; 4] }
#[repr(C)]
pub struct opal_sum_ranges { pub key: opal_key, pub num_lrs: u8, pub lr: [u8; OPAL_MAX_LRS], pub range_policy: u8, pub align: [u8; 5] }
#[repr(C)]
pub struct opal_lock_unlock { pub session: opal_session_info, pub l_state: u32, pub flags: u16, pub __align: [u8; 2] }
#[repr(C)]
pub struct opal_new_pw { pub session: opal_session_info, pub new_user_pw: opal_session_info }
#[repr(C)]
pub struct opal_mbr_data { pub key: opal_key, pub enable_disable: u8, pub __align: [u8; 7] }
#[repr(C)]
pub struct opal_mbr_done { pub key: opal_key, pub done_flag: u8, pub __align: [u8; 7] }
#[repr(C)]
pub struct opal_shadow_mbr { pub key: opal_key, pub data: u64, pub offset: u64, pub size: u64 }

#[repr(u32)]
pub enum opal_table_ops { OPAL_READ_TABLE, OPAL_WRITE_TABLE }
pub const OPAL_UID_LENGTH: usize = 8;
#[repr(C)]
pub struct opal_read_write_table { pub key: opal_key, pub data: u64, pub table_uid: [u8; OPAL_UID_LENGTH], pub offset: u64, pub size: u64, pub flags: u64, pub priv_: u64 }
pub const OPAL_TABLE_READ: u64 = 1 << (OPAL_READ_TABLE as u32);
pub const OPAL_TABLE_WRITE: u64 = 1 << (OPAL_WRITE_TABLE as u32);

pub const OPAL_FL_SUPPORTED: u32 = 0x00000001;
pub const OPAL_FL_LOCKING_SUPPORTED: u32 = 0x00000002;
pub const OPAL_FL_LOCKING_ENABLED: u32 = 0x00000004;
pub const OPAL_FL_LOCKED: u32 = 0x00000008;
pub const OPAL_FL_MBR_ENABLED: u32 = 0x00000010;
pub const OPAL_FL_MBR_DONE: u32 = 0x00000020;
pub const OPAL_FL_SUM_SUPPORTED: u32 = 0x00000040;

#[repr(C)] pub struct opal_status { pub flags: u32, pub reserved: u32 }
#[repr(C)] pub struct opal_geometry { pub align: u8, pub logical_block_size: u32, pub alignment_granularity: u64, pub lowest_aligned_lba: u64, pub __align: [u8; 3] }
#[repr(C)] pub struct opal_discovery { pub data: u64, pub size: u64 }
#[repr(C)] pub struct opal_revert_lsp { pub key: opal_key, pub options: u32, pub __pad: u32 }

// ioctl constants retain the source macros and depend on the surrounding ABI's
// _IO/_IOW/_IOR definitions.
pub const IOC_OPAL_SAVE: _ = _IOW('p', 220, opal_lock_unlock);
pub const IOC_OPAL_LOCK_UNLOCK: _ = _IOW('p', 221, opal_lock_unlock);
pub const IOC_OPAL_TAKE_OWNERSHIP: _ = _IOW('p', 222, opal_key);
pub const IOC_OPAL_ACTIVATE_LSP: _ = _IOW('p', 223, opal_lr_act);
pub const IOC_OPAL_SET_PW: _ = _IOW('p', 224, opal_new_pw);
pub const IOC_OPAL_ACTIVATE_USR: _ = _IOW('p', 225, opal_session_info);
pub const IOC_OPAL_REVERT_TPR: _ = _IOW('p', 226, opal_key);
pub const IOC_OPAL_LR_SETUP: _ = _IOW('p', 227, opal_user_lr_setup);
pub const IOC_OPAL_ADD_USR_TO_LR: _ = _IOW('p', 228, opal_lock_unlock);
pub const IOC_OPAL_ENABLE_DISABLE_MBR: _ = _IOW('p', 229, opal_mbr_data);
pub const IOC_OPAL_ERASE_LR: _ = _IOW('p', 230, opal_session_info);
pub const IOC_OPAL_SECURE_ERASE_LR: _ = _IOW('p', 231, opal_session_info);
pub const IOC_OPAL_PSID_REVERT_TPR: _ = _IOW('p', 232, opal_key);
pub const IOC_OPAL_MBR_DONE: _ = _IOW('p', 233, opal_mbr_done);
pub const IOC_OPAL_WRITE_SHADOW_MBR: _ = _IOW('p', 234, opal_shadow_mbr);
pub const IOC_OPAL_GENERIC_TABLE_RW: _ = _IOW('p', 235, opal_read_write_table);
pub const IOC_OPAL_GET_STATUS: _ = _IOR('p', 236, opal_status);
pub const IOC_OPAL_GET_LR_STATUS: _ = _IOW('p', 237, opal_lr_status);
pub const IOC_OPAL_GET_GEOMETRY: _ = _IOR('p', 238, opal_geometry);
pub const IOC_OPAL_DISCOVERY: _ = _IOW('p', 239, opal_discovery);
pub const IOC_OPAL_REVERT_LSP: _ = _IOW('p', 240, opal_revert_lsp);
pub const IOC_OPAL_SET_SID_PW: _ = _IOW('p', 241, opal_new_pw);
pub const IOC_OPAL_REACTIVATE_LSP: _ = _IOW('p', 242, opal_lr_react);
pub const IOC_OPAL_LR_SET_START_LEN: _ = _IOW('p', 243, opal_user_lr_setup);
pub const IOC_OPAL_ENABLE_DISABLE_LR: _ = _IOW('p', 244, opal_user_lr_setup);
pub const IOC_OPAL_GET_SUM_STATUS: _ = _IOW('p', 245, opal_sum_ranges);
pub const IOC_OPAL_STACK_RESET: _ = _IO('p', 246);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
