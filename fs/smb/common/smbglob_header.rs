/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Copyright (C) International Business Machines  Corp., 2002,2008
 *                 2018 Samsung Electronics Co., Ltd.
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *              Jeremy Allison (jra@samba.org)
 *              Namjae Jeon (linkinjeon@kernel.org)
 *
 */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct smb_version_values {
    pub version_string: *mut c_char,
    pub protocol_id: u16,
    pub lock_cmd: u16,
    pub req_capabilities: u32,
    pub max_read_size: u32,
    pub max_write_size: u32,
    pub max_trans_size: u32,
    pub max_credits: u32,
    pub large_lock_type: u32,
    pub exclusive_lock_type: u32,
    pub shared_lock_type: u32,
    pub unlock_lock_type: u32,
    pub header_size: usize,
    pub max_header_size: usize,
    pub read_rsp_size: usize,
    pub cap_unix: u32,
    pub cap_nt_find: u32,
    pub cap_large_files: u32,
    pub cap_unicode: u32,
    pub signing_enabled: u16,
    pub signing_required: u16,
    pub create_lease_size: usize,
    pub create_durable_size: usize,
    pub create_durable_v2_size: usize,
    pub create_mxac_size: usize,
    pub create_disk_id_size: usize,
    pub create_posix_size: usize,
    pub create_aapl_size: usize,
}

#[inline]
pub unsafe fn get_rfc1002_len(buf: *mut c_void) -> u32 {
    let value = core::ptr::read_unaligned(buf as *const u32);
    be32_to_cpu(value) & 0x00ff_ffff
}

#[inline]
pub unsafe fn inc_rfc1001_len(buf: *mut c_void, count: i32) {
    be32_add_cpu(buf as *mut u32, count);
}

pub const SMB1_VERSION_STRING: &str = "1.0";
pub const SMB20_VERSION_STRING: &str = "2.0";
pub const SMB21_VERSION_STRING: &str = "2.1";
pub const SMBDEFAULT_VERSION_STRING: &str = "default";
pub const SMB3ANY_VERSION_STRING: &str = "3";
pub const SMB30_VERSION_STRING: &str = "3.0";
pub const SMB302_VERSION_STRING: &str = "3.02";
pub const ALT_SMB302_VERSION_STRING: &str = "3.0.2";
pub const SMB311_VERSION_STRING: &str = "3.1.1";
pub const ALT_SMB311_VERSION_STRING: &str = "3.11";

pub const CIFS_DEFAULT_IOSIZE: usize = 1024 * 1024;

pub const MAX_CIFS_SMALL_BUFFER_SIZE: usize = 448; /* big enough for most */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
