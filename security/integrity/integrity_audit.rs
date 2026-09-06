// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 IBM Corporation
 * Author: Mimi Zohar <zohar@us.ibm.com>
 *
 * File: integrity_audit.rs
 *	Audit calls for the integrity subsystem
 */

// Requires kernel dependencies: linux/fs.h, linux/gfp.h, linux/audit.h, integrity.h

use std::os::raw::{c_char, c_int};

// Opaque kernel structures
#[repr(C)]
pub struct audit_buffer {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct audit_context {
    _opaque: [u8; 0],
}

// Inode and superblock structures from linux/fs.h
#[repr(C)]
pub struct super_block {
    pub s_id: [u8; 32],
    // ... other fields omitted
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    pub i_ino: u64,
    // ... other fields omitted
}

const TASK_COMM_LEN: usize = 16;
const GFP_KERNEL: u32 = 0x10;

// External kernel functions
extern "C" {
    fn audit_log_start(
        ctx: *mut audit_context,
        flags: u32,
        msg_type: c_int,
    ) -> *mut audit_buffer;

    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...) -> c_int;
    fn audit_log_untrustedstring(ab: *mut audit_buffer, data: *const u8) -> c_int;
    fn audit_log_task_context(ab: *mut audit_buffer) -> c_int;
    fn audit_log_end(ab: *mut audit_buffer);

    fn audit_context() -> *mut audit_context;
    fn task_pid_nr(t: *const std::ffi::c_void) -> u32;
    fn current_uid() -> u32;
    fn from_kuid(ns: *const std::ffi::c_void, uid: u32) -> u32;
    fn audit_get_loginuid(t: *const std::ffi::c_void) -> u32;
    fn audit_get_sessionid(t: *const std::ffi::c_void) -> u32;
    fn get_task_comm(buf: *mut u8, t: *const std::ffi::c_void) -> *mut u8;
    fn kstrtoul(s: *const c_char, base: c_int, res: *mut u64) -> c_int;

    static current: *const std::ffi::c_void;
    static init_user_ns: std::ffi::c_void;
}

static mut INTEGRITY_AUDIT_INFO: c_int = 0;

// ima_audit_setup - enable informational auditing messages
unsafe extern "C" fn integrity_audit_setup(str_param: *mut c_char) -> c_int {
    let mut audit: u64 = 0;

    if kstrtoul(str_param, 0, &mut audit) == 0 {
        INTEGRITY_AUDIT_INFO = if audit != 0 { 1 } else { 0 };
    }
    1
}

// __setup("integrity_audit=", integrity_audit_setup);
// Note: Linux kernel __setup() macro cannot be represented in pure Rust.

pub unsafe extern "C" fn integrity_audit_msg(
    audit_msgno: c_int,
    inode: *mut inode,
    fname: *const u8,
    op: *const c_char,
    cause: *const c_char,
    result: c_int,
    audit_info: c_int,
) {
    integrity_audit_message(audit_msgno, inode, fname, op, cause, result, audit_info, 0);
}

pub unsafe extern "C" fn integrity_audit_message(
    audit_msgno: c_int,
    inode: *mut inode,
    fname: *const u8,
    op: *const c_char,
    cause: *const c_char,
    result: c_int,
    audit_info: c_int,
    errno: c_int,
) {
    let mut name: [u8; TASK_COMM_LEN] = [0; TASK_COMM_LEN];

    if INTEGRITY_AUDIT_INFO == 0 && audit_info == 1 {
        /* Skip info messages */
        return;
    }

    let ab = audit_log_start(audit_context(), GFP_KERNEL, audit_msgno);
    if ab.is_null() {
        return;
    }

    audit_log_format(
        ab,
        b"pid=%d uid=%u auid=%u ses=%u\0".as_ptr() as *const c_char,
        task_pid_nr(current),
        from_kuid(&init_user_ns as *const _ as *const std::ffi::c_void, current_uid()),
        from_kuid(&init_user_ns as *const _ as *const std::ffi::c_void, audit_get_loginuid(current)),
        audit_get_sessionid(current),
    );
    audit_log_task_context(ab);
    audit_log_format(
        ab,
        b" op=%s cause=%s comm=\0".as_ptr() as *const c_char,
        op,
        cause,
    );
    audit_log_untrustedstring(ab, get_task_comm(name.as_mut_ptr(), current));
    if !fname.is_null() {
        audit_log_format(ab, b" name=\0".as_ptr() as *const c_char);
        audit_log_untrustedstring(ab, fname);
    }
    if !inode.is_null() {
        audit_log_format(ab, b" dev=\0".as_ptr() as *const c_char);
        audit_log_untrustedstring(ab, (*(*inode).i_sb).s_id.as_ptr());
        audit_log_format(
            ab,
            b" ino=%llu\0".as_ptr() as *const c_char,
            (*inode).i_ino,
        );
    }
    audit_log_format(
        ab,
        b" res=%d errno=%d\0".as_ptr() as *const c_char,
        (result == 0) as c_int,
        errno,
    );
    audit_log_end(ab);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
