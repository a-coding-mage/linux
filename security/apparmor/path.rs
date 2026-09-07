// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor function for pathnames
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Linux kernel includes (provided via external dependencies):
// #include <linux/magic.h>
// #include <linux/mount.h>
// #include <linux/namei.h>
// #include <linux/nsproxy.h>
// #include <linux/path.h>
// #include <linux/sched.h>
// #include <linux/slab.h>
// #include <linux/fs_struct.h>

// AppArmor internal includes (provided via external dependencies):
// #include "include/apparmor.h"
// #include "include/path.h"
// #include "include/policy.h"

use std::ptr;

// Error codes
const ENAMETOOLONG: i32 = -36;
const EACCES: i32 = -13;
const ENOENT: i32 = -2;

// Path flags (from include/path.h)
extern "C" {
    static aa_g_path_max: i32;
}

// External kernel structures (opaque in this translation)
#[repr(C)]
pub struct path {
    pub mnt: *mut MountStruct,
    pub dentry: *mut DentryStruct,
}

#[repr(C)]
pub struct MountStruct {
    pub mnt_flags: u32,
}

#[repr(C)]
pub struct DentryStruct {
    pub d_sb: *mut SuperBlock,
}

#[repr(C)]
pub struct SuperBlock {
    pub s_magic: u32,
}

// External kernel constants and functions
const PATH_CONNECT_PATH: i32 = 1;
const PATH_CHROOT_REL: i32 = 2;
const PATH_CHROOT_NSCONNECT: i32 = 4;
const PATH_IS_DIR: i32 = 8;
const PATH_MEDIATE_DELETED: i32 = 16;
const PATH_DELEGATE_DELETED: i32 = 32;
const MNT_INTERNAL: u32 = 0x4000;
const PROC_SUPER_MAGIC: u32 = 0x9fa0;

extern "C" {
    fn dentry_path(dentry: *const DentryStruct, buf: *mut u8, buflen: i32) -> *mut u8;
    fn __d_path(
        path: *const path,
        root: *const path,
        buf: *mut u8,
        buflen: i32,
    ) -> *mut u8;
    fn d_absolute_path(path: *const path, buf: *mut u8, buflen: i32) -> *mut u8;
    fn dentry_path_raw(dentry: *const DentryStruct, buf: *mut u8, buflen: i32) -> *mut u8;
    fn d_unlinked(dentry: *const DentryStruct) -> i32;
    fn d_is_positive(dentry: *const DentryStruct) -> i32;
    fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    fn strlen(s: *const u8) -> usize;
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn our_mnt(mnt: *const MountStruct) -> i32;
    fn get_fs_root(fs: *const (), root: *mut path);
    fn path_put(path: *const path);
}

// modified from dcache.c
unsafe fn prepend(
    buffer: &mut *mut u8,
    buflen: i32,
    str: *const u8,
    namelen: i32,
) -> i32 {
    let new_buflen = buflen - namelen;
    if new_buflen < 0 {
        return ENAMETOOLONG;
    }
    *buffer = (*buffer).offset(-(namelen as isize));
    memcpy(*buffer, str, namelen as usize);
    0
}

const CHROOT_NSCONNECT: i32 = PATH_CHROOT_REL | PATH_CHROOT_NSCONNECT;

// Helper to check if pointer is an error (Linux kernel macro pattern)
fn is_err(ptr: *const u8) -> bool {
    (ptr as isize) < 0
}

// Helper to get error from pointer (Linux kernel macro pattern)
fn ptr_err(ptr: *const u8) -> i32 {
    (ptr as isize) as i32
}

// Helper to check if pointer is null or error
fn is_err_or_null(ptr: *const u8) -> bool {
    ptr.is_null() || is_err(ptr)
}

/* If the path is not connected to the expected root,
 * check if it is a sysctl and handle specially else remove any
 * leading / that __d_path may have returned.
 * Unless
 *     specifically directed to connect the path,
 * OR
 *     if in a chroot and doing chroot relative paths and the path
 *     resolves to the namespace root (would be connected outside
 *     of chroot) and specifically directed to connect paths to
 *     namespace root.
 */
unsafe fn disconnect(
    path: *const path,
    buf: *mut u8,
    name: &mut *mut u8,
    flags: i32,
    disconnected: *const u8,
) -> i32 {
    let mut error = 0;

    if (flags & PATH_CONNECT_PATH) == 0
        && !(((flags & CHROOT_NSCONNECT) == CHROOT_NSCONNECT) && our_mnt((*path).mnt) != 0)
    {
        /* disconnected path, don't return pathname starting
         * with '/'
         */
        error = EACCES;
        if **name == b'/' as u8 {
            *name = (*name).offset(1);
        }
    } else {
        if **name != b'/' as u8 {
            /* CONNECT_PATH with missing root */
            error = prepend(name, *name as isize as i32 - buf as isize as i32, b"/\0".as_ptr(), 1);
        }
        if error == 0 && !disconnected.is_null() {
            error = prepend(
                name,
                *name as isize as i32 - buf as isize as i32,
                disconnected,
                strlen(disconnected) as i32,
            );
        }
    }

    error
}

/**
 * d_namespace_path - lookup a name associated with a given path
 * @path: path to lookup  (NOT NULL)
 * @buf:  buffer to store path to  (NOT NULL)
 * @name: Returns - pointer for start of path name with in @buf (NOT NULL)
 * @flags: flags controlling path lookup
 * @disconnected: string to prefix to disconnected paths
 *
 * Handle path name lookup.
 *
 * Returns: %0 else error code if path lookup fails
 *          When no error the path name is returned in @name which points to
 *          a position in @buf
 */
unsafe fn d_namespace_path(
    path: *const path,
    buf: *mut u8,
    name: &mut *mut u8,
    flags: i32,
    disconnected: *const u8,
) -> i32 {
    let mut res: *mut u8;
    let mut error = 0;
    let mut connected = 1;
    let isdir = if (flags & PATH_IS_DIR) != 0 { 1 } else { 0 };
    let buflen = aa_g_path_max - isdir;

    if (*(*path).mnt).mnt_flags & MNT_INTERNAL != 0 {
        /* it's not mounted anywhere */
        res = dentry_path((*path).dentry, buf, buflen);
        *name = res;
        if is_err(res) {
            *name = buf;
            return ptr_err(res);
        }
        if (*(*(*path).dentry).d_sb).s_magic == PROC_SUPER_MAGIC
            && strncmp(*name, b"/sys/\0".as_ptr(), 5) == 0
        {
            /* TODO: convert over to using a per namespace
             * control instead of hard coded /proc
            */
            error = prepend(name, *name as isize as i32 - buf as isize as i32, b"/proc\0".as_ptr(), 5);
        } else {
            error = disconnect(path, buf, name, flags, disconnected);
        }

        /* Append "/" to directory paths and reterminate string, except for
         * root "/" which already ends in a slash.
         */
        if error == 0 && isdir != 0 {
            let is_root = *(*name) == b'/' as u8 && *(*name).offset(1) == b'\0' as u8;

            if !is_root {
                *(buf.offset((aa_g_path_max - 2) as isize)) = b'/' as u8;
                *(buf.offset((aa_g_path_max - 1) as isize)) = b'\0' as u8;
            }
        }
        return error;
    }

    /* resolve paths relative to chroot?*/
    if (flags & PATH_CHROOT_REL) != 0 {
        let mut root: path = std::mem::zeroed();
        get_fs_root((*path).mnt as *const (), &mut root);
        res = __d_path(path, &root, buf, buflen);
        path_put(&root);
    } else {
        res = d_absolute_path(path, buf, buflen);
        if our_mnt((*path).mnt) == 0 {
            connected = 0;
        }
    }

    /* handle error conditions - and still allow a partial path to
     * be returned.
     */
    if is_err_or_null(res) {
        if ptr_err(res) == ENAMETOOLONG {
            error = ENAMETOOLONG;
            *name = buf;
            return error;
        }
        connected = 0;
        res = dentry_path_raw((*path).dentry, buf, buflen);
        if is_err(res) {
            error = ptr_err(res);
            *name = buf;
            return error;
        }
    } else if our_mnt((*path).mnt) == 0 {
        connected = 0;
    }

    *name = res;

    if connected == 0 {
        error = disconnect(path, buf, name, flags, disconnected);
    }

    /* Handle two cases:
     * 1. A deleted dentry && profile is not allowing mediation of deleted
     * 2. On some filesystems, newly allocated dentries appear to the
     *    security_path hooks as a deleted dentry except without an inode
     *    allocated.
     */
    if d_unlinked((*path).dentry) != 0
        && d_is_positive((*path).dentry) != 0
        && ((flags & (PATH_MEDIATE_DELETED | PATH_DELEGATE_DELETED)) == 0)
    {
        error = ENOENT;
    }

    /* Append "/" to directory paths and reterminate string, except for
     * root "/" which already ends in a slash.
     */
    if error == 0 && isdir != 0 {
        let is_root = *(*name) == b'/' as u8 && *(*name).offset(1) == b'\0' as u8;

        if !is_root {
            *(buf.offset((aa_g_path_max - 2) as isize)) = b'/' as u8;
            *(buf.offset((aa_g_path_max - 1) as isize)) = b'\0' as u8;
        }
    }

    error
}

/**
 * aa_path_name - get the pathname to a buffer ensure dir / is appended
 * @path: path the file  (NOT NULL)
 * @flags: flags controlling path name generation
 * @buffer: buffer to put name in (NOT NULL)
 * @name: Returns - the generated path name if !error (NOT NULL)
 * @info: Returns - information on why the path lookup failed (MAYBE NULL)
 * @disconnected: string to prepend to disconnected paths
 *
 * @name is a pointer to the beginning of the pathname (which usually differs
 * from the beginning of the buffer), or NULL.  If there is an error @name
 * may contain a partial or invalid name that can be used for audit purposes,
 * but it can not be used for mediation.
 *
 * We need PATH_IS_DIR to indicate whether the file is a directory or not
 * because the file may not yet exist, and so we cannot check the inode's
 * file type.
 *
 * Returns: %0 else error code if could retrieve name
 */
pub unsafe fn aa_path_name(
    path: *const path,
    flags: i32,
    buffer: *mut u8,
    name: &mut *const u8,
    info: *mut *const u8,
    disconnected: *const u8,
) -> i32 {
    let mut str: *mut u8 = ptr::null_mut();
    let error = d_namespace_path(path, buffer, &mut str, flags, disconnected);

    if !info.is_null() && error != 0 {
        if error == ENOENT {
            *info = b"Failed name lookup - deleted entry\0".as_ptr();
        } else if error == EACCES {
            *info = b"Failed name lookup - disconnected path\0".as_ptr();
        } else if error == ENAMETOOLONG {
            *info = b"Failed name lookup - name too long\0".as_ptr();
        } else {
            *info = b"Failed name lookup\0".as_ptr();
        }
    }

    *name = str as *const u8;

    error
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
