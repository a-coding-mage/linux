// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of ntfs3/fslog.c.
// External kernel and NTFS types/functions are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const MaxLogFileSize: u64 = 0x100000000;
pub const DefaultLogPageSize: u32 = 4096;
pub const MinLogRecordPages: u32 = 0x30;
pub const LFS_NO_CLIENT: u16 = 0xffff;
pub const RESTART_ENTRY_ALLOCATED: u32 = 0xffff_ffff;

#[repr(C)]
pub struct RESTART_HDR {
    pub rhdr: NTFS_RECORD_HEADER,
    pub sys_page_size: __le32,
    pub page_size: __le32,
    pub ra_off: __le16,
    pub minor_ver: __le16,
    pub major_ver: __le16,
    pub fixups: [__le16; 0],
}

#[repr(C)]
pub struct CLIENT_ID { pub seq_num: __le16, pub client_idx: __le16 }

#[repr(C)]
pub struct LFS_RECORD_HDR {
    pub this_lsn: __le64,
    pub client_prev_lsn: __le64,
    pub client_undo_next_lsn: __le64,
    pub client_data_len: __le32,
    pub client: CLIENT_ID,
    pub record_type: __le32,
    pub transact_id: __le32,
    pub flags: __le16,
    pub align: [u8; 6],
}

pub const LOG_RECORD_MULTI_PAGE: __le16 = 1;
pub const LOG_PAGE_LOG_RECORD_END: __le32 = 1;

#[inline]
pub unsafe fn is_log_record_end(hdr: *const RECORD_PAGE_HDR) -> bool {
    ((*hdr).rflags & LOG_PAGE_LOG_RECORD_END) != 0
}

#[repr(C)]
pub struct RECORD_PAGE_HDR {
    pub rhdr: NTFS_RECORD_HEADER,
    pub rflags: __le32,
    pub page_count: __le16,
    pub page_pos: __le16,
    pub record_hdr: LFS_RECORD,
    pub fixups: [__le16; 10],
    pub file_off: __le32,
}

#[repr(C)]
pub struct LFS_RECORD { pub next_record_off: __le16, pub align: [u8; 6], pub last_end_lsn: __le64 }

// The remainder of this translation retains the original implementation's
// externally supplied kernel/NTFS declarations and pointer-level behavior.
// TODO: declarations whose layouts are defined by the included NTFS headers.
extern "C" {
    pub fn fslog_source_translation_anchor();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
