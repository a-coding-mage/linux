/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ocfs2_lockid.h
 *
 * Defines OCFS2 lockid bits.
 *
 * Copyright (C) 2002, 2005 Oracle.  All rights reserved.
 */

/* lock ids are made up in the following manner:
 * name[0]     --> type
 * name[1-6]   --> 6 pad characters, reserved for now
 * name[7-22]  --> block number, expressed in hex as 16 chars
 * name[23-30] --> i_generation, expressed in hex 8 chars
 * name[31]    --> '\0' */
pub const OCFS2_LOCK_ID_MAX_LEN: usize = 32;
pub const OCFS2_LOCK_ID_PAD: &str = "000000";

pub const OCFS2_DENTRY_LOCK_INO_START: i32 = 18;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ocfs2_lock_type {
    OCFS2_LOCK_TYPE_META = 0,
    OCFS2_LOCK_TYPE_DATA,
    OCFS2_LOCK_TYPE_SUPER,
    OCFS2_LOCK_TYPE_RENAME,
    OCFS2_LOCK_TYPE_RW,
    OCFS2_LOCK_TYPE_DENTRY,
    OCFS2_LOCK_TYPE_OPEN,
    OCFS2_LOCK_TYPE_FLOCK,
    OCFS2_LOCK_TYPE_QINFO,
    OCFS2_LOCK_TYPE_NFS_SYNC,
    OCFS2_LOCK_TYPE_ORPHAN_SCAN,
    OCFS2_LOCK_TYPE_REFCOUNT,
    OCFS2_LOCK_TYPE_TRIM_FS,
    OCFS2_NUM_LOCK_TYPES,
}

pub fn ocfs2_lock_type_char(type_: ocfs2_lock_type) -> std::os::raw::c_char {
    match type_ {
        ocfs2_lock_type::OCFS2_LOCK_TYPE_META => b'M' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_DATA => b'D' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_SUPER => b'S' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_RENAME => b'R' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_RW => b'W' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_DENTRY => b'N' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_OPEN => b'O' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_FLOCK => b'F' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_QINFO => b'Q' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_NFS_SYNC => b'Y' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_ORPHAN_SCAN => b'P' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_REFCOUNT => b'T' as std::os::raw::c_char,
        ocfs2_lock_type::OCFS2_LOCK_TYPE_TRIM_FS => b'I' as std::os::raw::c_char,
        _ => 0,
    }
}

/* Need to differentiate from [R]ename.. serializing writes is the
 * important job it does, anyway. */
pub static mut ocfs2_lock_type_strings: [*mut std::os::raw::c_char; 13] = [
    b"Meta\0".as_ptr() as *mut std::os::raw::c_char,
    b"Data\0".as_ptr() as *mut std::os::raw::c_char,
    b"Super\0".as_ptr() as *mut std::os::raw::c_char,
    b"Rename\0".as_ptr() as *mut std::os::raw::c_char,
    b"Write/Read\0".as_ptr() as *mut std::os::raw::c_char,
    b"Dentry\0".as_ptr() as *mut std::os::raw::c_char,
    b"Open\0".as_ptr() as *mut std::os::raw::c_char,
    b"Flock\0".as_ptr() as *mut std::os::raw::c_char,
    b"Quota\0".as_ptr() as *mut std::os::raw::c_char,
    b"NFSSync\0".as_ptr() as *mut std::os::raw::c_char,
    b"OrphanScan\0".as_ptr() as *mut std::os::raw::c_char,
    b"Refcount\0".as_ptr() as *mut std::os::raw::c_char,
    b"TrimFs\0".as_ptr() as *mut std::os::raw::c_char,
];

pub unsafe fn ocfs2_lock_type_string(type_: ocfs2_lock_type) -> *const std::os::raw::c_char {
    /* Under __KERNEL__, the C source applies BUG_ON(type >= OCFS2_NUM_LOCK_TYPES). */
    ocfs2_lock_type_strings[type_ as usize] as *const std::os::raw::c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
