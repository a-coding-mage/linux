/* SPDX-License-Identifier: GPL-2.0-only */
/* Faithful Rust declaration translation of ubifs.h. External kernel types and
 * constants are intentionally left as dependencies of the surrounding crate. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const UBIFS_VERSION: u32 = 1;
pub const UBIFS_SUPER_MAGIC: u32 = 0x24051905;
pub const SQNUM_WARN_WATERMARK: u64 = 0xFFFFFFFF00000000;
pub const SQNUM_WATERMARK: u64 = 0xFFFFFFFFFF000000;
pub const MIN_INDEX_LEBS: i32 = 2;
pub const LPT_HEAP_SZ: usize = 256;
pub const MAX_INUM: u32 = 0xFFFFFFFF;
pub const NONDATA_JHEADS_CNT: i32 = 2;
pub const LPROPS_NC: u32 = 0x80000001;
pub const OLD_ZNODE_AGE: i32 = 20;
pub const YOUNG_ZNODE_AGE: i32 = 5;
pub const BOTTOM_UP_HEIGHT: usize = 64;
pub const UBIFS_MAX_BULK_READ: usize = 32;

pub const DIRTY_ZNODE: i32 = 0;
pub const COW_ZNODE: i32 = 1;
pub const OBSOLETE_ZNODE: i32 = 2;
pub const COMMIT_RESTING: i32 = 0;
pub const COMMIT_BACKGROUND: i32 = 1;
pub const COMMIT_REQUIRED: i32 = 2;
pub const COMMIT_RUNNING_BACKGROUND: i32 = 3;
pub const COMMIT_RUNNING_REQUIRED: i32 = 4;
pub const COMMIT_BROKEN: i32 = 5;
pub const SCANNED_GARBAGE: i32 = 0;
pub const SCANNED_EMPTY_SPACE: i32 = -1;
pub const SCANNED_A_NODE: i32 = -2;
pub const SCANNED_A_CORRUPT_NODE: i32 = -3;
pub const SCANNED_A_BAD_PAD_NODE: i32 = -4;
pub const DIRTY_CNODE: i32 = 0;
pub const OBSOLETE_CNODE: i32 = 1;
pub const COW_CNODE: i32 = 2;
pub const LTAB_DIRTY: i32 = 1;
pub const LSAVE_DIRTY: i32 = 2;
pub const LEB_FREED: i32 = 0;
pub const LEB_FREED_IDX: i32 = 1;
pub const LEB_RETAINED: i32 = 2;
pub const ASSACT_REPORT: i32 = 0;
pub const ASSACT_RO: i32 = 1;
pub const ASSACT_PANIC: i32 = 2;
pub const LPROPS_UNCAT: i32 = 0;
pub const LPROPS_DIRTY: i32 = 1;
pub const LPROPS_DIRTY_IDX: i32 = 2;
pub const LPROPS_FREE: i32 = 3;
pub const LPROPS_HEAP_CNT: i32 = 3;
pub const LPROPS_EMPTY: i32 = 4;
pub const LPROPS_FREEABLE: i32 = 5;
pub const LPROPS_FRDI_IDX: i32 = 6;
pub const LPROPS_CAT_MASK: i32 = 15;
pub const LPROPS_TAKEN: i32 = 16;
pub const LPROPS_INDEX: i32 = 32;
pub const LPT_SCAN_CONTINUE: i32 = 0;
pub const LPT_SCAN_ADD: i32 = 1;
pub const LPT_SCAN_STOP: i32 = 2;

#[repr(C)]
pub union ubifs_key {
    pub u8: [u8; UBIFS_SK_LEN],
    pub u32: [u32; UBIFS_SK_LEN / 4],
    pub u64: [u64; UBIFS_SK_LEN / 8],
    pub j32: [__le32; UBIFS_SK_LEN / 4],
}

#[repr(C)] pub struct ubifs_old_idx { pub rb: rb_node, pub lnum: i32, pub offs: i32 }
#[repr(C)] pub struct ubifs_scan_node { pub list: list_head, pub key: ubifs_key, pub sqnum: u64, pub type_: i32, pub offs: i32, pub len: i32, pub node: *mut c_void }
#[repr(C)] pub struct ubifs_scan_leb { pub lnum: i32, pub nodes_cnt: i32, pub nodes: list_head, pub endpt: i32, pub buf: *mut c_void }
#[repr(C)] pub struct ubifs_gced_idx_leb { pub list: list_head, pub lnum: i32, pub unmap: i32 }
#[repr(C)] pub struct ubifs_unclean_leb { pub list: list_head, pub lnum: i32, pub endpt: i32 }
#[repr(C)] pub struct ubifs_lprops { pub free: i32, pub dirty: i32, pub flags: i32, pub lnum: i32, pub list_or_hpos: ubifs_lprops_union }
#[repr(C)] pub union ubifs_lprops_union { pub list: list_head, pub hpos: i32 }
#[repr(C)] pub struct ubifs_lpt_lprops { pub free: i32, pub dirty: i32, pub tgc: u32, pub cmt: u32 }
#[repr(C)] pub struct ubifs_lp_stats { pub empty_lebs: i32, pub taken_empty_lebs: i32, pub idx_lebs: i32, pub total_free: i64, pub total_dirty: i64, pub total_used: i64, pub total_dead: i64, pub total_dark: i64 }
#[repr(C)] pub struct ubifs_node_range { pub len_or_min_len: ubifs_node_range_union, pub max_len: i32 }
#[repr(C)] pub union ubifs_node_range_union { pub len: i32, pub min_len: i32 }
#[repr(C)] pub struct ubifs_mount_opts { pub unmount_mode: u32, pub bulk_read: u32, pub chk_data_crc: u32, pub override_compr: u32, pub compr_type: u32 }
#[repr(C)] pub struct ubifs_stats_info { pub magic_errors: u32, pub node_errors: u32, pub crc_errors: u32 }

/* The remaining declarations retain the C ABI through opaque dependency types. */
extern "C" {
    pub static mut ubifs_infos: list_head;
    pub static mut ubifs_infos_lock: spinlock_t;
    pub static mut ubifs_clean_zn_cnt: atomic_long_t;
}

pub type ubifs_lpt_scan_callback = unsafe extern "C" fn(*mut ubifs_info, *const ubifs_lprops, i32, *mut c_void) -> i32;

/* External kernel declarations supplied by the translated dependency headers. */
extern "C" {
    pub fn ubifs_ro_mode(c: *mut ubifs_info, err: i32);
    pub fn ubifs_leb_read(c: *const ubifs_info, lnum: i32, buf: *mut c_void, offs: i32, len: i32, even_ebadmsg: i32) -> i32;
    pub fn ubifs_leb_write(c: *mut ubifs_info, lnum: i32, buf: *const c_void, offs: i32, len: i32) -> i32;
    pub fn ubifs_budget_space(c: *mut ubifs_info, req: *mut ubifs_budget_req) -> i32;
}

use core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
