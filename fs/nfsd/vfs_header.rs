/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
 */

// C dependencies: linux/fs.h, linux/posix_acl.h, and nfsfh.h.

pub const NFSD_MAY_NOP: u32 = 0;
pub const NFSD_MAY_EXEC: u32 = 0x001;
pub const NFSD_MAY_WRITE: u32 = 0x002;
pub const NFSD_MAY_READ: u32 = 0x004;
pub const NFSD_MAY_SATTR: u32 = 0x008;
pub const NFSD_MAY_TRUNC: u32 = 0x010;
pub const NFSD_MAY_NLM: u32 = 0x020;
pub const NFSD_MAY_MASK: u32 = 0x03f;

pub const NFSD_MAY_OWNER_OVERRIDE: u32 = 0x040;
pub const NFSD_MAY_LOCAL_ACCESS: u32 = 0x080;
pub const NFSD_MAY_BYPASS_GSS_ON_ROOT: u32 = 0x100;
pub const NFSD_MAY_NOT_BREAK_LEASE: u32 = 0x200;
pub const NFSD_MAY_BYPASS_GSS: u32 = 0x400;
pub const NFSD_MAY_READ_IF_EXEC: u32 = 0x800;
pub const NFSD_MAY_64BIT_COOKIE: u32 = 0x1000;
pub const NFSD_MAY_LOCALIO: u32 = 0x2000;
pub const NFSD_MAY_CREATE: u32 = NFSD_MAY_EXEC | NFSD_MAY_WRITE;
pub const NFSD_MAY_REMOVE: u32 = NFSD_MAY_EXEC | NFSD_MAY_WRITE | NFSD_MAY_TRUNC;

pub struct nfsd_file;

pub type nfsd_filldir_t = Option<unsafe extern "C" fn(
    *mut core::ffi::c_void, *const core::ffi::c_char, core::ffi::c_int,
    loff_t, u64, u32,
)>;

#[repr(C)]
pub struct readdir_cd {
    pub err: __be32,
}

#[repr(C)]
pub struct nfsd_attrs {
    pub na_iattr: *mut iattr,
    pub na_seclabel: *mut xdr_netobj,
    pub na_pacl: *mut posix_acl,
    pub na_dpacl: *mut posix_acl,
    pub na_labelerr: core::ffi::c_int,
    pub na_dpaclerr: core::ffi::c_int,
    pub na_paclerr: core::ffi::c_int,
}

#[inline]
pub unsafe fn nfsd_attrs_free(attrs: *mut nfsd_attrs) {
    posix_acl_release((*attrs).na_pacl);
    posix_acl_release((*attrs).na_dpacl);
}

#[inline]
pub unsafe fn nfsd_attrs_valid(attrs: *mut nfsd_attrs) -> bool {
    let iap = (*attrs).na_iattr;
    (*iap).ia_valid != 0
        || (!(*attrs).na_seclabel.is_null() && (*(*attrs).na_seclabel).len != 0)
        || !(*attrs).na_pacl.is_null()
        || !(*attrs).na_dpacl.is_null()
}

// These declarations are present when CONFIG_NFSD_V4 is enabled.
#[cfg(CONFIG_NFSD_V4)]
extern "C" {
    pub fn nfsd4_vfs_fallocate(rqstp: *mut svc_rqst, fhp: *mut svc_fh, file: *mut file,
        offset: u64, len: u64, mode: core::ffi::c_int) -> __be32;
    pub fn nfsd4_clone_file_range(rqstp: *mut svc_rqst, nf_src: *mut nfsd_file,
        src_pos: u64, nf_dst: *mut nfsd_file, dst_pos: u64, count: u64, sync: bool) -> __be32;
    pub fn nfsd_getxattr(rqstp: *mut svc_rqst, fhp: *mut svc_fh, name: *mut core::ffi::c_char,
        bufp: *mut *mut core::ffi::c_void, lenp: *mut core::ffi::c_int) -> __be32;
    pub fn nfsd_listxattr(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
        bufp: *mut *mut core::ffi::c_char, lenp: *mut core::ffi::c_int) -> __be32;
    pub fn nfsd_removexattr(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
        name: *mut core::ffi::c_char) -> __be32;
    pub fn nfsd_setxattr(rqstp: *mut svc_rqst, fhp: *mut svc_fh, name: *mut core::ffi::c_char,
        buf: *mut core::ffi::c_void, len: u32, flags: u32) -> __be32;
}

extern "C" {
    pub fn nfserrno(errno: core::ffi::c_int) -> __be32;
    pub fn nfsd_cross_mnt(rqstp: *mut svc_rqst, dpp: *mut *mut dentry,
        expp: *mut *mut svc_export) -> core::ffi::c_int;
    pub fn nfsd_lookup(rqstp: *mut svc_rqst, fhp: *mut svc_fh, name: *const core::ffi::c_char,
        len: u32, resfhp: *mut svc_fh) -> __be32;
    pub fn nfsd_lookup_dentry(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
        name: *const core::ffi::c_char, len: u32, expp: *mut *mut svc_export,
        dpp: *mut *mut dentry) -> __be32;
    pub fn nfsd_setattr(rqstp: *mut svc_rqst, fhp: *mut svc_fh, attrs: *mut nfsd_attrs,
        time: *const timespec64) -> __be32;
    pub fn nfsd_mountpoint(dentry: *mut dentry, exp: *mut svc_export) -> core::ffi::c_int;
    pub fn nfsd_create_locked(rqstp: *mut svc_rqst, fhp: *mut svc_fh, attrs: *mut nfsd_attrs,
        type_: core::ffi::c_int, rdev: dev_t, res: *mut svc_fh) -> __be32;
    pub fn nfsd_create(rqstp: *mut svc_rqst, fhp: *mut svc_fh, name: *mut core::ffi::c_char,
        len: core::ffi::c_int, attrs: *mut nfsd_attrs, type_: core::ffi::c_int,
        rdev: dev_t, res: *mut svc_fh) -> __be32;
    pub fn nfsd_access(rqstp: *mut svc_rqst, fhp: *mut svc_fh, ac: *mut u32, supported: *mut u32) -> __be32;
    pub fn nfsd_create_setattr(rqstp: *mut svc_rqst, fhp: *mut svc_fh, resfhp: *mut svc_fh, iap: *mut nfsd_attrs) -> __be32;
    pub fn nfsd_commit(rqst: *mut svc_rqst, fhp: *mut svc_fh, nf: *mut nfsd_file, offset: u64, count: u32, verf: *mut __be32) -> __be32;
    pub fn nfsd_open_break_lease(inode: *mut inode, flags: core::ffi::c_int) -> core::ffi::c_int;
    pub fn nfsd_open(rqstp: *mut svc_rqst, fhp: *mut svc_fh, type_: umode_t, flags: core::ffi::c_int, filp: *mut *mut file) -> __be32;
    pub fn nfsd_open_verified(fhp: *mut svc_fh, type_: umode_t, may_flags: core::ffi::c_int, filp: *mut *mut file) -> core::ffi::c_int;
    pub fn nfsd_readlink(rqstp: *mut svc_rqst, fhp: *mut svc_fh, buf: *mut core::ffi::c_char, len: *mut core::ffi::c_int) -> __be32;
    pub fn nfsd_symlink(rqstp: *mut svc_rqst, fhp: *mut svc_fh, name: *mut core::ffi::c_char,
        len: core::ffi::c_int, path: *mut core::ffi::c_char, attrs: *mut nfsd_attrs,
        res: *mut svc_fh) -> __be32;
    pub fn nfsd_link(rqstp: *mut svc_rqst, fhp: *mut svc_fh, name: *mut core::ffi::c_char,
        len: core::ffi::c_int, res: *mut svc_fh) -> __be32;
    pub fn nfsd_copy_file_range(src: *mut file, src_pos: u64, dst: *mut file,
        dst_pos: u64, count: u64) -> isize;
    pub fn nfsd_rename(rqstp: *mut svc_rqst, ffhp: *mut svc_fh, fname: *mut core::ffi::c_char,
        flen: core::ffi::c_int, tfhp: *mut svc_fh, tname: *mut core::ffi::c_char,
        tlen: core::ffi::c_int) -> __be32;
    pub fn nfsd_unlink(rqstp: *mut svc_rqst, fhp: *mut svc_fh, type_: core::ffi::c_int,
        name: *mut core::ffi::c_char, len: core::ffi::c_int) -> __be32;
    pub fn nfsd_readdir(rqstp: *mut svc_rqst, fhp: *mut svc_fh, offset: *mut u64,
        cd: *mut readdir_cd, filldir: nfsd_filldir_t) -> __be32;
    pub fn nfsd_statfs(rqstp: *mut svc_rqst, fhp: *mut svc_fh, stat: *mut kstatfs,
        access: core::ffi::c_int) -> __be32;
    pub fn nfsd_get_case_info(dentry: *mut dentry, case_insensitive: *mut bool,
        case_preserving: *mut bool) -> core::ffi::c_int;
    pub fn nfsd_permission(cred: *mut svc_cred, exp: *mut svc_export,
        dentry: *mut dentry, acc: core::ffi::c_int) -> __be32;
    pub fn nfsd_filp_close(fp: *mut file);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
