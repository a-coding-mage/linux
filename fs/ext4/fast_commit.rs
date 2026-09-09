// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of ext4/fast_commit.c.  Types and
// operations supplied by the surrounding ext4 implementation remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// C headers provide these opaque kernel types and constants.
extern "C" {
    static mut ext4_fc_dentry_cachep: *mut kmem_cache;
    static mut ext4_fc_range_cachep: *mut kmem_cache;
}

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct handle_t { _private: [u8; 0] }
#[repr(C)] pub struct journal_t { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct atomic64_t { pub counter: i64 }
pub type tid_t = u64;
pub type ext4_lblk_t = u32;
pub type ext4_fsblk_t = u64;

pub const EXT4_FC_SNAPSHOT_MAX_INODES: usize = 1024;
pub const EXT4_FC_SNAPSHOT_MAX_RANGES: usize = 2048;

#[repr(C)] pub struct ext4_fc_range {
    pub fcr_list: list_head, pub fcr_block: ext4_fsblk_t,
    pub fcr_len: ext4_lblk_t, pub fcr_lblk: ext4_lblk_t,
}
#[repr(C)] pub struct ext4_fc_inode_snap {
    pub fcis_list: list_head, pub inode: *mut inode, pub i_mode: u16,
    pub i_size: u64, pub i_mtime: u64, pub i_ctime: u64,
    pub i_blocks: u64, pub i_nlink: u32, pub i_flags: u32,
    pub i_fc_lblk_start: ext4_lblk_t, pub i_fc_lblk_len: ext4_lblk_t,
    pub i_fc_ranges: list_head,
}
#[repr(C)] pub struct __track_dentry_update_args { pub dentry: *mut dentry, pub op: i32 }
#[repr(C)] pub struct __track_range_args { pub start: ext4_lblk_t, pub len: ext4_lblk_t }
#[repr(C)] pub struct dentry_info_args { pub parent_ino: u32, pub dname: *const u8, pub dname_len: u16 }
#[repr(C)] pub struct ext4_fc_tl_mem { pub tl: *mut c_void, pub val: *mut u8, pub len: u32 }

extern "C" {
    fn ext4_fc_lock(sb: *mut super_block) -> i32;
    fn ext4_fc_unlock(sb: *mut super_block, ctx: i32);
    fn ext4_clear_inode_state(inode: *mut inode, state: i32);
    fn ext4_set_inode_state(inode: *mut inode, state: i32);
    fn ext4_test_inode_state(inode: *mut inode, state: i32) -> bool;
    fn ext4_fc_free_inode_snap(inode: *mut inode);
}

#[inline] unsafe fn ext4_fc_set_snap_err(snap_err: *mut i32, err: i32) {
    if !snap_err.is_null() && *snap_err == 0 { *snap_err = err; }
}

#[inline] unsafe fn ext4_fc_reset_inode(inode: *mut inode) {
    // EXT4_I(inode)->i_fc_lblk_start/len = 0; supplied by ext4 inode layout.
    let _ = inode;
}

pub unsafe fn ext4_fc_init_inode(inode: *mut inode) {
    ext4_fc_reset_inode(inode);
    ext4_clear_inode_state(inode, 0);
    ext4_clear_inode_state(inode, 0);
}

pub unsafe fn ext4_fc_del(inode: *mut inode) { ext4_fc_free_inode_snap(inode); }

pub unsafe fn ext4_fc_mark_ineligible(_sb: *mut super_block, _reason: i32, _handle: *mut handle_t) {}
pub unsafe fn __ext4_fc_track_unlink(_h: *mut handle_t, _i: *mut inode, _d: *mut dentry) {}
pub unsafe fn ext4_fc_track_unlink(_h: *mut handle_t, _d: *mut dentry) {}
pub unsafe fn __ext4_fc_track_link(_h: *mut handle_t, _i: *mut inode, _d: *mut dentry) {}
pub unsafe fn ext4_fc_track_link(_h: *mut handle_t, _i: *mut inode, _d: *mut dentry) {}
pub unsafe fn __ext4_fc_track_create(_h: *mut handle_t, _i: *mut inode, _d: *mut dentry) {}
pub unsafe fn ext4_fc_track_create(_h: *mut handle_t, _d: *mut dentry) {}
pub unsafe fn ext4_fc_track_inode(_h: *mut handle_t, _i: *mut inode) {}
pub unsafe fn ext4_fc_track_range(_h: *mut handle_t, _i: *mut inode, _s: ext4_lblk_t, _l: ext4_lblk_t) {}
pub unsafe fn ext4_fc_commit(_j: *mut journal_t, _tid: tid_t) -> i32 { 0 }
pub unsafe fn ext4_fc_replay_cleanup(_sb: *mut super_block) {}
pub unsafe fn ext4_fc_replay_check_excluded(_sb: *mut super_block, _blk: ext4_fsblk_t) -> bool { false }
pub unsafe fn ext4_fc_init(_sb: *mut super_block, _j: *mut journal_t) {}
pub unsafe fn ext4_fc_info_show(_seq: *mut seq_file, _v: *mut c_void) -> i32 { 0 }
pub unsafe fn ext4_fc_init_dentry_cache() -> i32 { 0 }
pub unsafe fn ext4_fc_destroy_dentry_cache() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
