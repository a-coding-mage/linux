/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2020 Samsung Electronics Co., Ltd.
 *   Author(s): Namjae Jeon <linkinjeon@kernel.org>
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct ndr {
    pub data: *mut c_char,
    pub offset: c_int,
    pub length: c_int,
}

pub const NDR_NTSD_OFFSETOF: c_int = 0xA0;

// Opaque types supplied by the corresponding C dependencies.
pub struct xattr_dos_attrib;
pub struct mnt_idmap;
pub struct inode;
pub struct xattr_smb_acl;
pub struct xattr_ntacl;

extern "C" {
    pub fn ndr_encode_dos_attr(n: *mut ndr, da: *mut xattr_dos_attrib) -> c_int;
    pub fn ndr_decode_dos_attr(n: *mut ndr, da: *mut xattr_dos_attrib) -> c_int;
    pub fn ndr_encode_posix_acl(
        n: *mut ndr,
        idmap: *mut mnt_idmap,
        inode: *mut inode,
        acl: *mut xattr_smb_acl,
        def_acl: *mut xattr_smb_acl,
    ) -> c_int;
    pub fn ndr_encode_v4_ntacl(n: *mut ndr, acl: *mut xattr_ntacl) -> c_int;
    pub fn ndr_encode_v3_ntacl(n: *mut ndr, acl: *mut xattr_ntacl) -> c_int;
    pub fn ndr_decode_v4_ntacl(n: *mut ndr, acl: *mut xattr_ntacl) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
