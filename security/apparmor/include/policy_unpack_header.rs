// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor policy loading interface function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

use core::ffi::c_void;
use core::ffi::c_char;

pub struct list_head;
pub struct kref;
pub struct dentry;
pub struct work_struct;
pub struct aa_ns;
pub struct aa_common_ref;
pub struct aa_profile;

#[repr(C)]
pub struct aa_load_ent {
    pub list: list_head,
    pub new: *mut aa_profile,
    pub old: *mut aa_profile,
    pub rename: *mut aa_profile,
    pub ns_name: *const c_char,
}

#[inline]
pub unsafe fn aa_get_i_loaddata(data: *mut aa_loaddata) -> *mut aa_loaddata {
    if !data.is_null() { kref_get(&mut (*data).count.count); }
    data
}

#[inline]
pub unsafe fn aa_get_profile_loaddata(data: *mut aa_loaddata) -> *mut aa_loaddata {
    if !data.is_null() { kref_get(&mut (*data).pcount); }
    data
}

#[inline]
pub unsafe fn aa_get_profile_loaddata_not0(data: *mut aa_loaddata) -> *mut aa_loaddata {
    if !data.is_null() && kref_get_unless_zero(&mut (*data).pcount) { return data; }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn aa_put_i_loaddata(data: *mut aa_loaddata) {
    if !data.is_null() { kref_put(&mut (*data).count.count, aa_loaddata_kref); }
}

#[inline]
pub unsafe fn aa_put_profile_loaddata(data: *mut aa_loaddata) {
    if !data.is_null() { kref_put(&mut (*data).pcount, aa_ploaddata_kref); }
}

extern "C" {
    pub fn aa_load_ent_free(ent: *mut aa_load_ent);
    pub fn aa_load_ent_alloc() -> *mut aa_load_ent;
}

pub const PACKED_FLAG_HAT: u32 = 1;
pub const PACKED_FLAG_DEBUG1: u32 = 2;
pub const PACKED_FLAG_DEBUG2: u32 = 4;
pub const PACKED_MODE_ENFORCE: u32 = 0;
pub const PACKED_MODE_COMPLAIN: u32 = 1;
pub const PACKED_MODE_KILL: u32 = 2;
pub const PACKED_MODE_UNCONFINED: u32 = 3;
pub const PACKED_MODE_USER: u32 = 4;

pub const AAFS_LOADDATA_ABI: usize = 0;
pub const AAFS_LOADDATA_REVISION: usize = 1;
pub const AAFS_LOADDATA_HASH: usize = 2;
pub const AAFS_LOADDATA_DATA: usize = 3;
pub const AAFS_LOADDATA_COMPRESSED_SIZE: usize = 4;
pub const AAFS_LOADDATA_DIR: usize = 5;
pub const AAFS_LOADDATA_NDENTS: usize = 6;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum aa_code {
    AA_U8 = 0, AA_U16, AA_U32, AA_U64, AA_NAME, AA_STRING, AA_BLOB,
    AA_STRUCT, AA_STRUCTEND, AA_LIST, AA_LISTEND, AA_ARRAY, AA_ARRAYEND,
}

#[repr(C)]
pub struct aa_ext {
    pub start: *mut c_void,
    pub end: *mut c_void,
    pub pos: *mut c_void,
    pub version: u32,
}

#[repr(C)]
pub struct aa_loaddata {
    pub count: aa_common_ref,
    pub pcount: kref,
    pub list: list_head,
    pub work: work_struct,
    pub dents: [*mut dentry; 6],
    pub ns: *mut aa_ns,
    pub name: *mut c_char,
    pub size: usize,
    pub compressed_size: usize,
    pub revision: i64,
    pub abi: i32,
    pub hash: *mut u8,
    pub data: *mut c_char,
}

extern "C" {
    pub fn aa_unpack(udata: *mut aa_loaddata, lh: *mut list_head,
        ns: *mut *const c_char, compressed_data: *mut c_char,
        compressed_size: usize) -> i32;
    pub fn __aa_loaddata_update(data: *mut aa_loaddata, revision: i64);
    pub fn aa_rawdata_eq(l: *mut aa_loaddata, r: *mut aa_loaddata) -> bool;
    pub fn aa_loaddata_kref(kref: *mut kref);
    pub fn aa_ploaddata_kref(kref: *mut kref);
    pub fn aa_loaddata_alloc(size: usize) -> *mut aa_loaddata;
}

extern "C" {
    fn kref_get(kref: *mut kref);
    fn kref_get_unless_zero(kref: *mut kref) -> bool;
    fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref));
}

// CONFIG_KUNIT declarations are available when the kernel unit-test condition is enabled.
#[cfg(any(test, feature = "kunit"))]
extern "C" {
    pub fn aa_inbounds(e: *mut aa_ext, size: usize) -> bool;
    pub fn aa_unpack_u16_chunk(e: *mut aa_ext, chunk: *mut *mut c_char) -> usize;
    pub fn aa_unpack_X(e: *mut aa_ext, code: aa_code) -> bool;
    pub fn aa_unpack_nameX(e: *mut aa_ext, code: aa_code, name: *const c_char) -> bool;
    pub fn aa_unpack_u32(e: *mut aa_ext, data: *mut u32, name: *const c_char) -> bool;
    pub fn aa_unpack_u64(e: *mut aa_ext, data: *mut u64, name: *const c_char) -> bool;
    pub fn aa_unpack_array(e: *mut aa_ext, name: *const c_char, size: *mut u16) -> bool;
    pub fn aa_unpack_blob(e: *mut aa_ext, blob: *mut *mut c_char, name: *const c_char) -> usize;
    pub fn aa_unpack_str(e: *mut aa_ext, string: *mut *const c_char, name: *const c_char) -> i32;
    pub fn aa_unpack_strdup(e: *mut aa_ext, string: *mut *mut c_char, name: *const c_char) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
