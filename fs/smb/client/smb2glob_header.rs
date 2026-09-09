/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Definitions for various global variables and structures
 *
 *   Copyright (C) International Business Machines  Corp., 2002, 2011
 *                 Etersoft, 2012
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *              Jeremy Allison (jra@samba.org)
 *              Pavel Shilovsky (pshilovsky@samba.org) 2012
 *
 */

/*
 *****************************************************************
 * Constants go here
 *****************************************************************
 */

/*
 * Identifiers for functions that use the open, operation, close pattern
 * in smb2inode.c:smb2_compound_op()
 */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum smb2_compound_ops {
    SMB2_OP_SET_DELETE = 1,
    SMB2_OP_SET_INFO,
    SMB2_OP_QUERY_INFO,
    SMB2_OP_QUERY_DIR,
    SMB2_OP_MKDIR,
    SMB2_OP_RENAME,
    SMB2_OP_HARDLINK,
    SMB2_OP_SET_EOF,
    SMB2_OP_UNLINK,
    SMB2_OP_POSIX_QUERY_INFO,
    SMB2_OP_SET_REPARSE,
    SMB2_OP_GET_REPARSE,
    SMB2_OP_QUERY_WSL_EA,
    SMB2_OP_OPEN_QUERY,
}

/* Used when constructing chained read requests. */
pub const CHAINED_REQUEST: i32 = 1;
pub const START_OF_CHAIN: i32 = 2;
pub const END_OF_CHAIN: i32 = 4;
pub const RELATED_REQUEST: i32 = 8;

/*
 *****************************************************************
 * Struct definitions go here
 *****************************************************************
 */

#[repr(C)]
pub struct status_to_posix_error {
    pub smb2_status: u32,
    pub posix_error: i32,
    pub status_string: *mut core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
