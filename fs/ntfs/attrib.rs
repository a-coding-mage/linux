// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust-facing translation of ntfs/attrib.c.
//
// The concrete NTFS layouts, endian helpers, locking primitives, allocator,
// and kernel operations are supplied by the surrounding translation unit.
// This file intentionally keeps the C ABI and pointer-oriented interfaces.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct ntfs_inode { _private: [u8; 0] }
#[repr(C)]
pub struct ntfs_volume { _private: [u8; 0] }
#[repr(C)]
pub struct mft_record { _private: [u8; 0] }
#[repr(C)]
pub struct attr_record { _private: [u8; 0] }
#[repr(C)]
pub struct runlist { _private: [u8; 0] }
#[repr(C)]
pub struct runlist_element { _private: [u8; 0] }
#[repr(C)]
pub struct ntfs_attr_search_ctx { _private: [u8; 0] }
#[repr(C)]
pub struct attr_list_entry { _private: [u8; 0] }
#[repr(C)]
pub struct attr_def { _private: [u8; 0] }

pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;
pub type s64 = i64;
pub type u8_ = u8;

#[no_mangle]
pub static mut AT_UNNAMED: [__le16; 1] = [0];
pub const NTFS_ATTR_READALL_MAX_SIZE: usize = 64 * 1024;
pub const NTFS_VCN_DELETE_MARK: s64 = -2;

extern "C" {
    pub fn ntfs_map_runlist_nolock(ni: *mut ntfs_inode, vcn: s64,
        ctx: *mut ntfs_attr_search_ctx) -> c_int;
    pub fn ntfs_map_runlist(ni: *mut ntfs_inode, vcn: s64) -> c_int;
    pub fn ntfs_attr_vcn_to_rl(ni: *mut ntfs_inode, vcn: s64,
        lcn: *mut s64) -> *mut runlist_element;
    pub fn ntfs_attr_vcn_to_lcn_nolock(ni: *mut ntfs_inode, vcn: s64,
        write_locked: bool) -> s64;
    pub fn ntfs_attr_find_vcn_nolock(ni: *mut ntfs_inode, vcn: s64,
        ctx: *mut ntfs_attr_search_ctx) -> *mut runlist_element;
    pub fn ntfs_attr_lookup(ty: __le32, name: *const __le16, name_len: u32,
        ic: u32, lowest_vcn: s64, val: *const u8, val_len: u32,
        ctx: *mut ntfs_attr_search_ctx) -> c_int;
    pub fn ntfs_attr_list_entry_is_valid(ale: *const attr_list_entry,
        end: *const u8) -> bool;
    pub fn ntfs_attr_list_is_valid(start: *const u8, size: s64) -> bool;
    pub fn load_attribute_list(base: *mut ntfs_inode, start: *mut u8,
        size: s64) -> c_int;
    pub fn ntfs_attr_size_bounds_check(vol: *const ntfs_volume, ty: __le32,
        size: s64) -> c_int;
    pub fn ntfs_attr_can_be_resident(vol: *const ntfs_volume, ty: __le32) -> c_int;
    pub fn ntfs_attr_record_resize(m: *mut mft_record, a: *mut attr_record,
        new_size: u32) -> c_int;
    pub fn ntfs_resident_attr_value_resize(m: *mut mft_record,
        a: *mut attr_record, new_size: u32) -> c_int;
    pub fn ntfs_attr_make_non_resident(ni: *mut ntfs_inode, data_size: u32) -> c_int;
    pub fn ntfs_attr_set(ni: *mut ntfs_inode, ofs: s64, cnt: s64, val: u8) -> c_int;
    pub fn ntfs_attr_set_initialized_size(ni: *mut ntfs_inode, new_size: s64) -> c_int;
    pub fn ntfs_attr_record_rm(ctx: *mut ntfs_attr_search_ctx) -> c_int;
    pub fn ntfs_attr_add(ni: *mut ntfs_inode, ty: __le32, name: *mut __le16,
        name_len: u8, val: *mut u8, size: s64) -> c_int;
    pub fn ntfs_attr_open(ni: *mut ntfs_inode, ty: __le32, name: *mut __le16,
        name_len: u32) -> c_int;
    pub fn ntfs_attr_close(ni: *mut ntfs_inode);
    pub fn ntfs_attr_map_whole_runlist(ni: *mut ntfs_inode) -> c_int;
    pub fn ntfs_attr_record_move_to(ctx: *mut ntfs_attr_search_ctx,
        ni: *mut ntfs_inode) -> c_int;
    pub fn ntfs_attr_record_move_away(ctx: *mut ntfs_attr_search_ctx,
        extra: c_int) -> c_int;
    pub fn ntfs_attr_update_mapping_pairs(ni: *mut ntfs_inode,
        from_vcn: s64) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
