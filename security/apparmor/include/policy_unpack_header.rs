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

// External types from other modules
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

// The AppArmor interface treats data as a type byte followed by the
// actual data.  The interface has the notion of a named entry
// which has a name (AA_NAME typecode followed by name string) followed by
// the entries typecode and data.  Named types allow for optional
// elements and extensions to be added and tested for without breaking
// backwards compatibility.

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum aa_code {
    AA_U8 = 0,
    AA_U16 = 1,
    AA_U32 = 2,
    AA_U64 = 3,
    AA_NAME = 4,
    AA_STRING = 5,
    AA_BLOB = 6,
    AA_STRUCT = 7,
    AA_STRUCTEND = 8,
    AA_LIST = 9,
    AA_LISTEND = 10,
    AA_ARRAY = 11,
    AA_ARRAYEND = 12,
}

// aa_ext is the read of the buffer containing the serialized profile. The
// data is copied into a kernel buffer in apparmorfs and then handed off to
// the unpack routines.
#[repr(C)]
pub struct aa_ext {
    pub start: *mut c_void,
    pub end: *mut c_void,
    pub pos: *mut c_void,
    pub version: u32,
}

// struct aa_loaddata - buffer of policy raw_data set
// @count: inode/filesystem refcount - use aa_get_i_loaddata()
// @pcount: profile refcount - use aa_get_profile_loaddata()
// @list: list the loaddata is on
// @work: used to do a delayed cleanup
// @dents: refs to dents created in aafs
// @ns: the namespace this loaddata was loaded into
// @name:
// @size: the size of the data that was loaded
// @compressed_size: the size of the data when it is compressed
// @revision: unique revision count that this data was loaded as
// @abi: the abi number the loaddata uses
// @hash: a hash of the loaddata, used to help dedup data
//
// There is no loaddata ref for being on ns->rawdata_list, so
// @ns->lock must be held when walking the list. Dentries and
// inode opens hold refs on @count; profiles hold refs on @pcount.
// When the last @pcount drops, do_ploaddata_rmfs() removes the
// fs entries and drops the associated @count ref.
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
    // Pointer to payload. If @compressed_size > 0, then this is the
    // compressed version of the payload, else it is the uncompressed
    // version (with the size indicated by @size).
    pub data: *mut c_char,
}

extern "C" {
    pub fn aa_unpack(
        udata: *mut aa_loaddata,
        lh: *mut list_head,
        ns: *mut *const c_char,
        compressed_data: *mut c_char,
        compressed_size: usize,
    ) -> i32;
}

// aa_get_i_loaddata - get a reference count from a counted data reference
// @data: reference to get a count on
//
// Returns: pointer to reference
// Requires: @data to have a valid reference count on it. It is a bug
//           if the race to reap can be encountered when it is used.
#[inline]
pub fn aa_get_i_loaddata(data: *mut aa_loaddata) -> *mut aa_loaddata {
    if !data.is_null() {
        unsafe {
            kref_get(&mut (*data).count.count);
        }
    }
    data
}

// aa_get_profile_loaddata - get a profile reference count on loaddata
// @data: reference to get a count on
//
// Returns: pointer to reference
// Requires: @data to have a valid reference count on it.
#[inline]
pub fn aa_get_profile_loaddata(data: *mut aa_loaddata) -> *mut aa_loaddata {
    if !data.is_null() {
        unsafe {
            kref_get(&mut (*data).pcount);
        }
    }
    data
}

// aa_get_profile_loaddata_not0 - get a profile reference count if not zero
// @data: reference to get a count on
//
// Like aa_get_profile_loaddata(), but safe to call on an entry that may
// be on a list (e.g. ns->rawdata_list) where the last pcount has already
// dropped and the deferred cleanup has not yet run.
//
// Returns: pointer to reference, or NULL if @data is NULL or its
//          profile refcount has already reached zero.
#[inline]
pub fn aa_get_profile_loaddata_not0(data: *mut aa_loaddata) -> *mut aa_loaddata {
    if !data.is_null() && unsafe { kref_get_unless_zero(&mut (*data).pcount) } {
        return data;
    }
    core::ptr::null_mut()
}

extern "C" {
    pub fn __aa_loaddata_update(data: *mut aa_loaddata, revision: i64);
    pub fn aa_rawdata_eq(l: *mut aa_loaddata, r: *mut aa_loaddata) -> bool;
    pub fn aa_loaddata_kref(kref: *mut kref);
    pub fn aa_ploaddata_kref(kref: *mut kref);
    pub fn aa_loaddata_alloc(size: usize) -> *mut aa_loaddata;
}

#[inline]
pub fn aa_put_i_loaddata(data: *mut aa_loaddata) {
    if !data.is_null() {
        unsafe {
            kref_put(&mut (*data).count.count, aa_loaddata_kref);
        }
    }
}

#[inline]
pub fn aa_put_profile_loaddata(data: *mut aa_loaddata) {
    if !data.is_null() {
        unsafe {
            kref_put(&mut (*data).pcount, aa_ploaddata_kref);
        }
    }
}

// Build condition: IS_ENABLED(CONFIG_KUNIT) indicates kernel unit test configuration
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

// External kref_get and kref_put functions for reference counting
extern "C" {
    fn kref_get(kref: *mut kref);
    fn kref_get_unless_zero(kref: *mut kref) -> bool;
    fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref));
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
