/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Anna Schumaker <Anna.Schumaker@Netapp.com>
 */

/* Dependency: declarations from <linux/xattr.h> are supplied elsewhere. */

/*
 * FIXME:  four LAYOUTSTATS calls per compound at most! Do we need to support
 * more? Need to consider not to pre-alloc too much for a compound.
 */
pub const PNFS_LAYOUTSTATS_MAXDEV: i32 = 4;
pub const READ_PLUS_SCRATCH_SIZE: i32 = 16;

/* nfs4.2proc.c */
/* Preserves the C CONFIG_NFS_V4_2 conditional. */
#[cfg(feature = "CONFIG_NFS_V4_2")]
extern "C" {
    pub fn nfs42_proc_allocate(file: *mut file, offset: loff_t, len: loff_t) -> i32;
    pub fn nfs42_proc_copy(
        src: *mut file,
        src_pos: loff_t,
        dst: *mut file,
        dst_pos: loff_t,
        len: size_t,
        server: *mut nl4_server,
        stateid: *mut nfs4_stateid,
        sync: bool,
    ) -> ssize_t;
    pub fn nfs42_proc_deallocate(file: *mut file, offset: loff_t, len: loff_t) -> i32;
    pub fn nfs42_proc_zero_range(file: *mut file, offset: loff_t, len: loff_t) -> i32;
    pub fn nfs42_proc_llseek(file: *mut file, offset: loff_t, whence: i32) -> loff_t;
    pub fn nfs42_proc_layoutstats_generic(
        server: *mut nfs_server,
        data: *mut nfs42_layoutstat_data,
    ) -> i32;
    pub fn nfs42_proc_clone(
        src: *mut file,
        dst: *mut file,
        src_pos: loff_t,
        dst_pos: loff_t,
        len: loff_t,
    ) -> i32;
    pub fn nfs42_proc_layouterror(
        lseg: *mut pnfs_layout_segment,
        errors: *const nfs42_layout_error,
        n: size_t,
    ) -> i32;
    pub fn nfs42_proc_copy_notify(
        src: *mut file,
        dst: *mut file,
        res: *mut nfs42_copy_notify_res,
    ) -> i32;

    pub fn nfs42_proc_getxattr(
        inode: *mut inode,
        name: *const c_char,
        buf: *mut c_void,
        buflen: size_t,
    ) -> ssize_t;
    pub fn nfs42_proc_setxattr(
        inode: *mut inode,
        name: *const c_char,
        buf: *const c_void,
        buflen: size_t,
        flags: i32,
    ) -> i32;
    pub fn nfs42_proc_listxattrs(
        inode: *mut inode,
        buf: *mut c_void,
        buflen: size_t,
        cookiep: *mut u64,
        eofp: *mut bool,
    ) -> ssize_t;
    pub fn nfs42_proc_removexattr(inode: *mut inode, name: *const c_char) -> i32;
}

/* The following inline functions are available when CONFIG_NFS_V4_2 is set. */
#[cfg(feature = "CONFIG_NFS_V4_2")]
#[inline]
pub unsafe fn nfs42_files_from_same_server(input: *mut file, output: *mut file) -> bool {
    let c_in = (*NFS_SERVER(file_inode(input))).nfs_client;
    let c_out = (*NFS_SERVER(file_inode(output))).nfs_client;

    nfs4_check_serverowner_major_id((*c_in).cl_serverowner, (*c_out).cl_serverowner)
}

/*
 * Maximum XDR buffer size needed for a listxattr buffer of buflen size.
 *
 * The upper boundary is a buffer with all 1-byte sized attribute names.
 * They would be 7 bytes long in the eventual buffer ("user.x\0"), and
 * 8 bytes long XDR-encoded.
 *
 * Include the trailing eof word as well and make the result a multiple
 * of 4 bytes.
 */
#[cfg(feature = "CONFIG_NFS_V4_2")]
#[inline]
pub fn nfs42_listxattr_xdrsize(buflen: u32) -> u32 {
    let size = 8 * buflen / (XATTR_USER_PREFIX_LEN + 2) + 4;

    (size + 3) & !3
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
