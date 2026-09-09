/* SPDX-License-Identifier: GPL-2.0 */
/*
 * File: linux/nfsacl.h
 *
 * (C) 2003 Andreas Gruenbacher <agruen@suse.de>
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/posix_acl.h, linux/sunrpc/xdr.h, and uapi/linux/nfsacl.h.

/* Maximum number of ACL entries over NFS */
pub const NFS_ACL_MAX_ENTRIES: usize = 1024;

pub const NFSACL_MAXWORDS: usize = 2 * (2 + 3 * NFS_ACL_MAX_ENTRIES);

// Build-time page constants are supplied by the target environment.
pub const NFSACL_MAXPAGES: usize =
    (2 * (8 + 12 * NFS_ACL_MAX_ENTRIES) + PAGE_SIZE - 1) >> PAGE_SHIFT;

pub const NFS_ACL_MAX_ENTRIES_INLINE: usize = 5;
pub const NFS_ACL_INLINE_BUFSIZE: usize =
    (2 * (2 + 3 * NFS_ACL_MAX_ENTRIES_INLINE)) << 2;

pub unsafe fn nfsacl_size(
    acl_access: *mut posix_acl,
    acl_default: *mut posix_acl,
) -> u32 {
    let mut w: u32 = 16;
    w = w.wrapping_add(
        core::cmp::max(
            if !acl_access.is_null() {
                (*acl_access).a_count as i32
            } else {
                3
            },
            4,
        ) as u32
            * 12,
    );
    if !acl_default.is_null() {
        w = w.wrapping_add(
            core::cmp::max((*acl_default).a_count as i32, 4) as u32 * 12,
        );
    }
    w
}

unsafe extern "C" {
    pub fn nfsacl_encode(
        buf: *mut xdr_buf,
        base: u32,
        inode: *mut inode,
        acl: *mut posix_acl,
        encode_entries: i32,
        typeflag: i32,
    ) -> i32;

    pub fn nfsacl_decode(
        buf: *mut xdr_buf,
        base: u32,
        aclcnt: *mut u32,
        pacl: *mut *mut posix_acl,
    ) -> i32;

    pub fn nfs_stream_decode_acl(
        xdr: *mut xdr_stream,
        aclcnt: *mut u32,
        pacl: *mut *mut posix_acl,
    ) -> bool;

    pub fn nfs_stream_encode_acl(
        xdr: *mut xdr_stream,
        inode: *mut inode,
        acl: *mut posix_acl,
        encode_entries: i32,
        typeflag: i32,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
