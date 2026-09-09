// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * Copyright (c) 2022-2024 Oracle.
 * All rights reserved.
 */

// Translated from xfs_handle.h. C header guards and include directives are
// omitted; dependent declarations are supplied by other translation units.

#[allow(non_camel_case_types)]
struct file;
#[allow(non_camel_case_types)]
struct xfs_fsop_attrlist_handlereq;
#[allow(non_camel_case_types)]
struct xfs_fsop_handlereq;
#[allow(non_camel_case_types)]
struct inode;
#[allow(non_camel_case_types)]
struct xfs_inode;
#[allow(non_camel_case_types)]
struct xfs_attrlist_cursor;
#[allow(non_camel_case_types)]
struct xfs_getparents;
#[allow(non_camel_case_types)]
struct xfs_getparents_by_handle;

unsafe extern "C" {
    fn xfs_attrlist_by_handle(
        parfilp: *mut file,
        p: *mut xfs_fsop_attrlist_handlereq,
    ) -> i32;

    fn xfs_attrmulti_by_handle(
        parfilp: *mut file,
        arg: *mut core::ffi::c_void,
    ) -> i32;

    fn xfs_find_handle(cmd: u32, hreq: *mut xfs_fsop_handlereq) -> i32;
    fn xfs_open_by_handle(parfilp: *mut file, hreq: *mut xfs_fsop_handlereq) -> i32;
    fn xfs_readlink_by_handle(parfilp: *mut file, hreq: *mut xfs_fsop_handlereq) -> i32;

    fn xfs_ioc_attrmulti_one(
        parfilp: *mut file,
        inode: *mut inode,
        opcode: u32,
        uname: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        len: *mut u32,
        flags: u32,
    ) -> i32;

    fn xfs_ioc_attr_list(
        dp: *mut xfs_inode,
        ubuf: *mut core::ffi::c_void,
        bufsize: usize,
        flags: i32,
        ucursor: *mut xfs_attrlist_cursor,
    ) -> i32;

    fn xfs_handle_to_dentry(
        parfilp: *mut file,
        uhandle: *mut core::ffi::c_void,
        hlen: u32,
    ) -> *mut dentry;

    fn xfs_ioc_getparents(file: *mut file, arg: *mut xfs_getparents) -> i32;
    fn xfs_ioc_getparents_by_handle(
        file: *mut file,
        arg: *mut xfs_getparents_by_handle,
    ) -> i32;
}

#[allow(non_camel_case_types)]
struct dentry;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
