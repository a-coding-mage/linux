/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * UUID/GUID definition
 *
 * Copyright (C) 2010, 2016 Intel Corp.
 *	Huang Ying <ying.huang@intel.com>
 */

// Dependency supplied by the surrounding translation unit: linux/string.h.

pub const UUID_SIZE: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guid_t {
    pub b: [__u8; UUID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uuid_t {
    pub b: [__u8; UUID_SIZE],
}

#[macro_export]
macro_rules! GUID_INIT {
    ($a:expr, $b:expr, $c:expr, $d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr, $d6:expr, $d7:expr) => {
        $crate::guid_t { b: [
            (($a) & 0xff), ((($a) >> 8) & 0xff), ((($a) >> 16) & 0xff), ((($a) >> 24) & 0xff),
            (($b) & 0xff), ((($b) >> 8) & 0xff),
            (($c) & 0xff), ((($c) >> 8) & 0xff),
            $d0, $d1, $d2, $d3, $d4, $d5, $d6, $d7,
        ]}
    };
}

#[macro_export]
macro_rules! UUID_INIT {
    ($a:expr, $b:expr, $c:expr, $d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr, $d6:expr, $d7:expr) => {
        $crate::uuid_t { b: [
            ((($a) >> 24) & 0xff), ((($a) >> 16) & 0xff), ((($a) >> 8) & 0xff), (($a) & 0xff),
            ((($b) >> 8) & 0xff), (($b) & 0xff),
            ((($c) >> 8) & 0xff), (($c) & 0xff),
            $d0, $d1, $d2, $d3, $d4, $d5, $d6, $d7,
        ]}
    };
}

/* The length of a UUID string ("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")
 * not including trailing NUL. */
pub const UUID_STRING_LEN: usize = 36;

extern "C" {
    pub static guid_null: guid_t;
    pub static uuid_null: uuid_t;

    pub fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
    pub fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn guid_equal(u1: *const guid_t, u2: *const guid_t) -> bool {
    memcmp(u1.cast(), u2.cast(), core::mem::size_of::<guid_t>()) == 0
}

#[inline]
pub unsafe fn guid_copy(dst: *mut guid_t, src: *const guid_t) {
    memcpy(dst.cast(), src.cast(), core::mem::size_of::<guid_t>());
}

#[inline]
pub unsafe fn import_guid(dst: *mut guid_t, src: *const __u8) {
    memcpy(dst.cast(), src.cast(), core::mem::size_of::<guid_t>());
}

#[inline]
pub unsafe fn export_guid(dst: *mut __u8, src: *const guid_t) {
    memcpy(dst.cast(), src.cast(), core::mem::size_of::<guid_t>());
}

#[inline]
pub unsafe fn guid_is_null(guid: *const guid_t) -> bool {
    guid_equal(guid, &guid_null)
}

#[inline]
pub unsafe fn uuid_equal(u1: *const uuid_t, u2: *const uuid_t) -> bool {
    memcmp(u1.cast(), u2.cast(), core::mem::size_of::<uuid_t>()) == 0
}

#[inline]
pub unsafe fn uuid_copy(dst: *mut uuid_t, src: *const uuid_t) {
    memcpy(dst.cast(), src.cast(), core::mem::size_of::<uuid_t>());
}

#[inline]
pub unsafe fn import_uuid(dst: *mut uuid_t, src: *const __u8) {
    memcpy(dst.cast(), src.cast(), core::mem::size_of::<uuid_t>());
}

#[inline]
pub unsafe fn export_uuid(dst: *mut __u8, src: *const uuid_t) {
    memcpy(dst.cast(), src.cast(), core::mem::size_of::<uuid_t>());
}

#[inline]
pub unsafe fn uuid_is_null(uuid: *const uuid_t) -> bool {
    uuid_equal(uuid, &uuid_null)
}

extern "C" {
    pub fn generate_random_uuid(uuid: *mut core::ffi::c_uchar);
    pub fn generate_random_guid(guid: *mut core::ffi::c_uchar);
    pub fn guid_gen(u: *mut guid_t);
    pub fn uuid_gen(u: *mut uuid_t);
    pub fn uuid_is_valid(uuid: *const core::ffi::c_char) -> bool;

    pub static guid_index: [u8; 16];
    pub static uuid_index: [u8; 16];

    pub fn guid_parse(uuid: *const core::ffi::c_char, u: *mut guid_t) -> i32;
    pub fn uuid_parse(uuid: *const core::ffi::c_char, u: *mut uuid_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
