/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for attribute handling in NTFS Linux kernel driver.
 *
 * Copyright (c) 2001-2005 Anton Altaparmakov
 * Copyright (c) 2002 Richard Russon
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// Dependencies supplied by the translated ntfs and dir headers are intentionally
// referenced here rather than redefined.

extern "C" {
    pub static mut AT_UNNAMED: [__le16; 0];

    #[link_name = "ntfs_map_runlist_nolock"]
    pub fn ntfs_map_runlist_nolock(ni: *mut ntfs_inode, vcn: s64, ctx: *mut ntfs_attr_search_ctx) -> ::core::ffi::c_int;
    pub fn ntfs_map_runlist(ni: *mut ntfs_inode, vcn: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_vcn_to_lcn_nolock(ni: *mut ntfs_inode, vcn: s64, write_locked: bool) -> s64;
    pub fn ntfs_attr_find_vcn_nolock(ni: *mut ntfs_inode, vcn: s64, ctx: *mut ntfs_attr_search_ctx) -> *mut runlist_element;
    pub fn __ntfs_attr_find_vcn_nolock(runlist: *mut runlist, vcn: s64) -> *mut runlist_element;
    pub fn ntfs_attr_map_whole_runlist(ni: *mut ntfs_inode) -> ::core::ffi::c_int;
    pub fn ntfs_attr_lookup(type_: __le32, name: *const __le16, name_len: u32, ic: u32,
        lowest_vcn: s64, val: *const u8, val_len: u32, ctx: *mut ntfs_attr_search_ctx) -> ::core::ffi::c_int;
    pub fn ntfs_attr_list_entry_is_valid(ale: *const attr_list_entry, al_end: *const u8) -> bool;
    pub fn ntfs_attr_list_is_valid(al_start: *const u8, size: s64) -> bool;
    pub fn load_attribute_list(base_ni: *mut ntfs_inode, al_start: *mut u8, size: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_reinit_search_ctx(ctx: *mut ntfs_attr_search_ctx);
    pub fn ntfs_attr_get_search_ctx(ni: *mut ntfs_inode, mrec: *mut mft_record) -> *mut ntfs_attr_search_ctx;
    pub fn ntfs_attr_put_search_ctx(ctx: *mut ntfs_attr_search_ctx);
    pub fn ntfs_attr_size_bounds_check(vol: *const ntfs_volume, type_: __le32, size: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_can_be_resident(vol: *const ntfs_volume, type_: __le32) -> ::core::ffi::c_int;
    pub fn ntfs_attr_map_cluster(ni: *mut ntfs_inode, vcn_start: s64, lcn_start: *mut s64,
        lcn_count: *mut s64, max_clu_count: s64, balloc: *mut bool, update_mp: bool, skip_holes: bool) -> ::core::ffi::c_int;
    pub fn ntfs_attr_record_resize(m: *mut mft_record, a: *mut attr_record, new_size: u32) -> ::core::ffi::c_int;
    pub fn ntfs_resident_attr_value_resize(m: *mut mft_record, a: *mut attr_record, new_size: u32) -> ::core::ffi::c_int;
    pub fn ntfs_attr_make_non_resident(ni: *mut ntfs_inode, data_size: u32) -> ::core::ffi::c_int;
    pub fn ntfs_attr_set(ni: *mut ntfs_inode, ofs: s64, cnt: s64, val: u8) -> ::core::ffi::c_int;
    pub fn ntfs_attr_set_initialized_size(ni: *mut ntfs_inode, new_size: loff_t) -> ::core::ffi::c_int;
    pub fn ntfs_attr_open(ni: *mut ntfs_inode, type_: __le32, name: *mut __le16, name_len: u32) -> ::core::ffi::c_int;
    pub fn ntfs_attr_close(n: *mut ntfs_inode);
    pub fn ntfs_attr_fallocate(ni: *mut ntfs_inode, start: loff_t, byte_len: loff_t, keep_size: bool) -> ::core::ffi::c_int;
    pub fn ntfs_non_resident_attr_insert_range(ni: *mut ntfs_inode, start_vcn: s64, len: s64) -> ::core::ffi::c_int;
    pub fn ntfs_non_resident_attr_collapse_range(ni: *mut ntfs_inode, start_vcn: s64, len: s64) -> ::core::ffi::c_int;
    pub fn ntfs_non_resident_attr_punch_hole(ni: *mut ntfs_inode, start_vcn: s64, len: s64) -> ::core::ffi::c_int;
    pub fn __ntfs_attr_truncate_vfs(ni: *mut ntfs_inode, newsize: s64, i_size: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_expand(ni: *mut ntfs_inode, newsize: s64, prealloc_size: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_truncate_i(ni: *mut ntfs_inode, newsize: s64, holes: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ntfs_attr_truncate(ni: *mut ntfs_inode, newsize: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_rm(ni: *mut ntfs_inode) -> ::core::ffi::c_int;
    pub fn ntfs_attr_exist(ni: *mut ntfs_inode, type_: __le32, name: *mut __le16, name_len: u32) -> ::core::ffi::c_int;
    pub fn ntfs_attr_remove(ni: *mut ntfs_inode, type_: __le32, name: *mut __le16, name_len: u32) -> ::core::ffi::c_int;
    pub fn ntfs_attr_record_rm(ctx: *mut ntfs_attr_search_ctx) -> ::core::ffi::c_int;
    pub fn ntfs_attr_record_move_to(ctx: *mut ntfs_attr_search_ctx, ni: *mut ntfs_inode) -> ::core::ffi::c_int;
    pub fn ntfs_attr_add(ni: *mut ntfs_inode, type_: __le32, name: *mut __le16, name_len: u8, val: *mut u8, size: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_record_move_away(ctx: *mut ntfs_attr_search_ctx, extra: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ntfs_attr_name_get(vol: *const ntfs_volume, uname: *const __le16, uname_len: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    pub fn ntfs_attr_name_free(name: *mut *mut u8);
    pub fn ntfs_attr_readall(ni: *mut ntfs_inode, type_: __le32, name: *mut __le16, name_len: u32, data_size: *mut s64) -> *mut ::core::ffi::c_void;
    pub fn ntfs_resident_attr_record_add(ni: *mut ntfs_inode, type_: __le32, name: *mut __le16, name_len: u8, val: *mut u8, size: u32, flags: __le16) -> ::core::ffi::c_int;
    pub fn ntfs_attr_update_mapping_pairs(ni: *mut ntfs_inode, from_vcn: s64) -> ::core::ffi::c_int;
    pub fn ntfs_attr_vcn_to_rl(ni: *mut ntfs_inode, vcn: s64, lcn: *mut s64) -> *mut runlist_element;
}

#[repr(C)]
pub struct ntfs_attr_search_ctx {
    pub mrec: *mut mft_record,
    pub mapped_mrec: bool,
    pub attr: *mut attr_record,
    pub is_first: bool,
    pub ntfs_ino: *mut ntfs_inode,
    pub al_entry: *mut attr_list_entry,
    pub base_ntfs_ino: *mut ntfs_inode,
    pub base_mrec: *mut mft_record,
    pub mapped_base_mrec: bool,
    pub base_attr: *mut attr_record,
}

pub const HOLES_NO: u32 = 0;
pub const HOLES_OK: u32 = 1;

#[inline]
pub unsafe fn ntfs_attr_size(a: *const attr_record) -> s64 {
    if !(*a).non_resident {
        le32_to_cpu((*a).data.resident.value_length) as s64
    } else {
        le64_to_cpu((*a).data.non_resident.data_size) as s64
    }
}

#[inline]
pub unsafe fn ntfs_attrs_walk(ctx: *mut ntfs_attr_search_ctx) -> ::core::ffi::c_int {
    ntfs_attr_lookup(AT_UNUSED, core::ptr::null(), 0, CASE_SENSITIVE, 0,
        core::ptr::null(), 0, ctx)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
