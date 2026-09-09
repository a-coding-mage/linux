/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024, SUSE LLC
 *
 * Authors: Enzo Matsumiya <ematsumiya@suse.de>
 *
 * This file implements I/O compression support for SMB2 messages (SMB 3.1.1 only).
 * See compress/ for implementation details of each algorithm.
 *
 * References:
 * MS-SMB2 "3.1.4.4 Compressing the Message" - for compression details
 * MS-SMB2 "3.1.5.3 Decompressing the Chained Message" - for decompression details
 * MS-XCA - for details of the supported algorithms
 */

// C header dependencies: linux/uio.h, linux/kernel.h, ../common/smb2pdu.h,
// ../common/compress/compress.h, and cifsglob.h.

pub const SMB_COMPRESS_HDR_LEN: usize = 16;
pub const SMB_COMPRESS_PAYLOAD_HDR_LEN: usize = 8;
// SMB_COMPRESS_MIN_LEN is PAGE_SIZE in the supplied platform dependencies.
pub const SMB_COMPRESS_MIN_LEN: usize = PAGE_SIZE;

#[repr(C)]
pub enum TCP_Server_Info {}

#[repr(C)]
pub enum smb_rqst {}

#[repr(C)]
pub enum cifs_tcon {}

#[cfg(feature = "CONFIG_CIFS_COMPRESSION")]
pub type compress_send_fn = unsafe extern "C" fn(
    server: *mut TCP_Server_Info,
    flags: core::ffi::c_int,
    rq: *mut smb_rqst,
) -> core::ffi::c_int;

#[cfg(feature = "CONFIG_CIFS_COMPRESSION")]
extern "C" {
    pub fn smb_compress(
        server: *mut TCP_Server_Info,
        rq: *mut smb_rqst,
        send_fn: compress_send_fn,
    ) -> core::ffi::c_int;

    pub fn should_compress(
        tcon: *const cifs_tcon,
        rq: *const smb_rqst,
    ) -> bool;
}

#[cfg(not(feature = "CONFIG_CIFS_COMPRESSION"))]
#[inline]
pub unsafe fn smb_compress(
    _unused1: *mut core::ffi::c_void,
    _unused2: *mut core::ffi::c_void,
    _unused3: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_CIFS_COMPRESSION"))]
#[inline]
pub unsafe fn should_compress(
    _unused1: *mut core::ffi::c_void,
    _unused2: *mut core::ffi::c_void,
) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
