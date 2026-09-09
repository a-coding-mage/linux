/* SPDX-License-Identifier: GPL-2.0 */

/* Flags shared between flags/xflags. */
pub const FS_COMMON_FL: u32 =
    FS_SYNC_FL | FS_IMMUTABLE_FL | FS_APPEND_FL |
    FS_NODUMP_FL | FS_NOATIME_FL | FS_DAX_FL |
    FS_PROJINHERIT_FL | FS_VERITY_FL;

pub const FS_XFLAG_COMMON: u32 =
    FS_XFLAG_SYNC | FS_XFLAG_IMMUTABLE | FS_XFLAG_APPEND |
    FS_XFLAG_NODUMP | FS_XFLAG_NOATIME | FS_XFLAG_DAX |
    FS_XFLAG_PROJINHERIT | FS_XFLAG_VERITY;

/* Read-only inode flags. */
pub const FS_XFLAG_RDONLY_MASK: u32 =
    FS_XFLAG_PREALLOC | FS_XFLAG_HASATTR | FS_XFLAG_VERITY |
    FS_XFLAG_CASEFOLD | FS_XFLAG_CASENONPRESERVING;

/* Flags to indicate valid value of fsx_ fields. */
pub const FS_XFLAG_VALUES_MASK: u32 = FS_XFLAG_EXTSIZE | FS_XFLAG_COWEXTSIZE;

/* Flags for directories. */
pub const FS_XFLAG_DIRONLY_MASK: u32 =
    FS_XFLAG_RTINHERIT | FS_XFLAG_NOSYMLINKS | FS_XFLAG_EXTSZINHERIT;

/* Misc settable flags. */
pub const FS_XFLAG_MISC_MASK: u32 =
    FS_XFLAG_REALTIME | FS_XFLAG_NODEFRAG | FS_XFLAG_FILESTREAM;

pub const FS_XFLAGS_MASK: u32 =
    FS_XFLAG_COMMON | FS_XFLAG_RDONLY_MASK | FS_XFLAG_VALUES_MASK |
    FS_XFLAG_DIRONLY_MASK | FS_XFLAG_MISC_MASK;

/*
 * Merged interface for miscellaneous file attributes.  `flags` originates
 * from ext* and `fsx_flags` from xfs.  There is some overlap between the two,
 * which is handled by the VFS helpers.
 */
#[repr(C)]
pub struct file_kattr {
    pub flags: u32,
    pub fsx_xflags: u32,
    pub fsx_extsize: u32,
    pub fsx_nextents: u32,
    pub fsx_projid: u32,
    pub fsx_cowextsize: u32,
    /* C bitfields: flags_valid:1 and fsx_valid:1. */
    pub flags_valid: bool,
    pub fsx_valid: bool,
}

extern "C" {
    pub fn copy_fsxattr_to_user(
        fa: *const file_kattr,
        ufa: *mut fsxattr,
    ) -> ::core::ffi::c_int;

    pub fn fileattr_fill_xflags(fa: *mut file_kattr, xflags: u32);
    pub fn fileattr_fill_flags(fa: *mut file_kattr, flags: u32);

    pub fn vfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> ::core::ffi::c_int;
    pub fn vfs_fileattr_set(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> ::core::ffi::c_int;
    pub fn ioctl_getflags(file: *mut file, argp: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ioctl_setflags(file: *mut file, argp: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ioctl_fsgetxattr(file: *mut file, argp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn ioctl_fssetxattr(file: *mut file, argp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}

/* External kernel types supplied by other headers. */
extern "C" {
    pub type fsxattr;
    pub type dentry;
    pub type mnt_idmap;
    pub type file;
}

/**
 * Check for extended flags/attributes not represented in `flags`.
 */
#[inline]
pub unsafe fn fileattr_has_fsx(fa: *const file_kattr) -> bool {
    (*fa).fsx_valid &&
        (((*fa).fsx_xflags & !FS_XFLAG_COMMON) != 0 ||
         (*fa).fsx_extsize != 0 || (*fa).fsx_projid != 0 ||
         (*fa).fsx_cowextsize != 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
