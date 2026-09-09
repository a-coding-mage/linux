/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) International Business Machines Corp., 2006
 *
 * Author: Artem Bityutskiy (Битюцкий Артём)
 */

// C dependencies: <linux/ioctl.h>, <linux/types.h>, <linux/scatterlist.h>,
// and <mtd/ubi-user.h>.

/* All volumes/LEBs */
pub const UBI_ALL: i32 = -1;

/* Maximum number of scatter gather list entries. */
pub const UBI_MAX_SG_COUNT: usize = 64;

/* enum ubi_open_mode - UBI volume open mode constants. */
pub const UBI_READONLY: i32 = 1;
pub const UBI_READWRITE: i32 = 2;
pub const UBI_EXCLUSIVE: i32 = 3;
pub const UBI_METAONLY: i32 = 4;

#[repr(C)]
pub struct ubi_volume_info {
    pub ubi_num: ::core::ffi::c_int,
    pub vol_id: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
    pub used_bytes: ::core::ffi::c_longlong,
    pub used_ebs: ::core::ffi::c_int,
    pub vol_type: ::core::ffi::c_int,
    pub corrupted: ::core::ffi::c_int,
    pub upd_marker: ::core::ffi::c_int,
    pub alignment: ::core::ffi::c_int,
    pub usable_leb_size: ::core::ffi::c_int,
    pub name_len: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
    pub cdev: dev_t,
    pub dev: *mut device,
}

#[repr(C)]
pub struct ubi_sgl {
    pub list_pos: ::core::ffi::c_int,
    pub page_pos: ::core::ffi::c_int,
    pub sg: [scatterlist; UBI_MAX_SG_COUNT],
}

/** Initialize an UBI scatter gather list data structure. */
#[inline]
pub unsafe fn ubi_sgl_init(usgl: *mut ubi_sgl) {
    (*usgl).list_pos = 0;
    (*usgl).page_pos = 0;
}

#[repr(C)]
pub struct ubi_device_info {
    pub ubi_num: ::core::ffi::c_int,
    pub leb_size: ::core::ffi::c_int,
    pub leb_start: ::core::ffi::c_int,
    pub min_io_size: ::core::ffi::c_int,
    pub max_write_size: ::core::ffi::c_int,
    pub ro_mode: ::core::ffi::c_int,
    pub cdev: dev_t,
}

/* Volume notification types. */
pub const UBI_VOLUME_ADDED: ::core::ffi::c_int = 0;
pub const UBI_VOLUME_REMOVED: ::core::ffi::c_int = 1;
pub const UBI_VOLUME_RESIZED: ::core::ffi::c_int = 2;
pub const UBI_VOLUME_RENAMED: ::core::ffi::c_int = 3;
pub const UBI_VOLUME_SHUTDOWN: ::core::ffi::c_int = 4;
pub const UBI_VOLUME_UPDATED: ::core::ffi::c_int = 5;

#[repr(C)]
pub struct ubi_notification {
    pub di: ubi_device_info,
    pub vi: ubi_volume_info,
}

/* UBI descriptor given to users when they open UBI volumes. */
#[repr(C)]
pub struct ubi_volume_desc {
    _private: [u8; 0],
}

extern "C" {
    pub fn ubi_get_device_info(ubi_num: ::core::ffi::c_int, di: *mut ubi_device_info) -> ::core::ffi::c_int;
    pub fn ubi_get_volume_info(desc: *mut ubi_volume_desc, vi: *mut ubi_volume_info);
    pub fn ubi_open_volume(ubi_num: ::core::ffi::c_int, vol_id: ::core::ffi::c_int, mode: ::core::ffi::c_int) -> *mut ubi_volume_desc;
    pub fn ubi_open_volume_nm(ubi_num: ::core::ffi::c_int, name: *const ::core::ffi::c_char, mode: ::core::ffi::c_int) -> *mut ubi_volume_desc;
    pub fn ubi_open_volume_path(pathname: *const ::core::ffi::c_char, mode: ::core::ffi::c_int) -> *mut ubi_volume_desc;
    pub fn ubi_register_volume_notifier(nb: *mut notifier_block, ignore_existing: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_unregister_volume_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn ubi_close_volume(desc: *mut ubi_volume_desc);
    pub fn ubi_leb_read(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int, buf: *mut ::core::ffi::c_char, offset: ::core::ffi::c_int, len: ::core::ffi::c_int, check: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_leb_read_sg(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int, sgl: *mut ubi_sgl, offset: ::core::ffi::c_int, len: ::core::ffi::c_int, check: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_leb_write(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int, buf: *const ::core::ffi::c_void, offset: ::core::ffi::c_int, len: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_leb_change(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int, buf: *const ::core::ffi::c_void, len: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_leb_erase(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_leb_unmap(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_leb_map(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_is_mapped(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ubi_sync(ubi_num: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn ubi_read(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int, buf: *mut ::core::ffi::c_char, offset: ::core::ffi::c_int, len: ::core::ffi::c_int) -> ::core::ffi::c_int {
    ubi_leb_read(desc, lnum, buf, offset, len, 0)
}

#[inline]
pub unsafe fn ubi_read_sg(desc: *mut ubi_volume_desc, lnum: ::core::ffi::c_int, sgl: *mut ubi_sgl, offset: ::core::ffi::c_int, len: ::core::ffi::c_int) -> ::core::ffi::c_int {
    ubi_leb_read_sg(desc, lnum, sgl, offset, len, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
