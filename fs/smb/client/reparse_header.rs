/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024 Paulo Alcantara <pc@manguebit.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/fs.h, linux/stat.h, linux/uidgid.h, fs_context.h, cifsglob.h,
// and ../common/smbfsctl.h.

pub const REPARSE_SYM_PATH_MAX: usize = 4060;

/*
 * Used only by cifs.ko to ignore reparse points from files when client or
 * server doesn't support FSCTL_GET_REPARSE_POINT.
 */
pub const IO_REPARSE_TAG_INTERNAL: u32 = !0u32;

pub unsafe fn reparse_mkdev(ptr: *mut core::ffi::c_void) -> dev_t {
    let v = le64_to_cpu(*(ptr as *const __le64));
    MKDEV(v & 0xffff_ffff, v >> 32)
}

pub unsafe fn wsl_make_kuid(cifs_sb: *mut cifs_sb_info, ptr: *mut core::ffi::c_void) -> kuid_t {
    let uid: u32 = le32_to_cpu(*(ptr as *const __le32));

    if cifs_sb_flags(cifs_sb) & CIFS_MOUNT_OVERR_UID != 0 {
        return (*(*cifs_sb).ctx).linux_uid;
    }
    make_kuid(current_user_ns(), uid)
}

pub unsafe fn wsl_make_kgid(cifs_sb: *mut cifs_sb_info, ptr: *mut core::ffi::c_void) -> kgid_t {
    let gid: u32 = le32_to_cpu(*(ptr as *const __le32));

    if cifs_sb_flags(cifs_sb) & CIFS_MOUNT_OVERR_GID != 0 {
        return (*(*cifs_sb).ctx).linux_gid;
    }
    make_kgid(current_user_ns(), gid)
}

pub fn reparse_mode_nfs_type(mode: mode_t) -> u64 {
    match mode & S_IFMT {
        S_IFLNK => NFS_SPECFILE_LNK,
        S_IFBLK => NFS_SPECFILE_BLK,
        S_IFCHR => NFS_SPECFILE_CHR,
        S_IFIFO => NFS_SPECFILE_FIFO,
        S_IFSOCK => NFS_SPECFILE_SOCK,
        _ => 0,
    }
}

pub fn reparse_mode_wsl_tag(mode: mode_t) -> u32 {
    match mode & S_IFMT {
        S_IFLNK => IO_REPARSE_TAG_LX_SYMLINK,
        S_IFBLK => IO_REPARSE_TAG_LX_BLK,
        S_IFCHR => IO_REPARSE_TAG_LX_CHR,
        S_IFIFO => IO_REPARSE_TAG_LX_FIFO,
        S_IFSOCK => IO_REPARSE_TAG_AF_UNIX,
        _ => 0,
    }
}

/*
 * Match a reparse point inode if reparse tag and ctime haven't changed.
 *
 * Windows Server updates ctime of reparse points when their data have changed.
 * The server doesn't allow changing reparse tags from existing reparse points,
 * though it's worth checking.
 */
pub unsafe fn reparse_inode_match(
    inode: *mut inode,
    fattr: *mut cifs_fattr,
) -> bool {
    let cinode = CIFS_I(inode);
    let ctime = inode_get_ctime(inode);

    /*
     * Do not match reparse tags when client or server doesn't support
     * FSCTL_GET_REPARSE_POINT.  @fattr->cf_cifstag should contain correct
     * reparse tag from query dir response but the client won't be able to
     * read the reparse point data anyway.  This spares us a revalidation.
     */
    if (*cinode).reparse_tag != IO_REPARSE_TAG_INTERNAL
        && (*cinode).reparse_tag != (*fattr).cf_cifstag
    {
        return false;
    }
    ((*cinode).cifsAttrs & ATTR_REPARSE_POINT) != 0
        && timespec64_equal(&ctime, &(*fattr).cf_ctime)
}

pub unsafe fn cifs_open_data_attrs(data: *const cifs_open_info_data) -> u32 {
    if (*data).contains_posix_file_info {
        return le32_to_cpu((*data).posix_fi.DosAttributes);
    }

    le32_to_cpu((*data).fi.Attributes)
}

pub unsafe fn cifs_open_data_reparse(data: *mut cifs_open_info_data) -> bool {
    let mut attrs = cifs_open_data_attrs(data);

    if (*data).contains_posix_file_info {
        let fi = &mut (*data).posix_fi;

        if (*data).reparse_point {
            attrs |= ATTR_REPARSE_POINT;
            fi.DosAttributes = cpu_to_le32(attrs);
        }
    } else {
        let fi = &mut (*data).fi;

        if (*data).reparse_point {
            attrs |= ATTR_REPARSE_POINT;
            fi.Attributes = cpu_to_le32(attrs);
        }
    }

    (attrs & ATTR_REPARSE_POINT) != 0
}

extern "C" {
    pub fn cifs_reparse_point_to_fattr(
        cifs_sb: *mut cifs_sb_info,
        fattr: *mut cifs_fattr,
        data: *mut cifs_open_info_data,
    ) -> bool;
    pub fn create_reparse_symlink(
        xid: c_uint,
        inode: *mut inode,
        dentry: *mut dentry,
        tcon: *mut cifs_tcon,
        full_path: *const c_char,
        symname: *const c_char,
    ) -> c_int;
    pub fn mknod_reparse(
        xid: c_uint,
        inode: *mut inode,
        dentry: *mut dentry,
        tcon: *mut cifs_tcon,
        full_path: *const c_char,
        mode: umode_t,
        dev: dev_t,
    ) -> c_int;
    pub fn smb2_get_reparse_point_buffer(
        rsp_iov: *const kvec,
        plen: *mut u32,
    ) -> *mut reparse_data_buffer;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
