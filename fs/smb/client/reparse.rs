// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024 Paulo Alcantara <pc@manguebit.com>
 */

// Kernel/CIFS dependencies supplied by the surrounding translation unit.

unsafe extern "C" {
    fn mknod_nfs(xid: u32, inode: *mut inode, dentry: *mut dentry, tcon: *mut cifs_tcon,
                 full_path: *const i8, mode: umode_t, dev: dev_t, symname: *const i8) -> i32;
    fn mknod_wsl(xid: u32, inode: *mut inode, dentry: *mut dentry, tcon: *mut cifs_tcon,
                 full_path: *const i8, mode: umode_t, dev: dev_t, symname: *const i8) -> i32;
    fn create_native_symlink(xid: u32, inode: *mut inode, dentry: *mut dentry, tcon: *mut cifs_tcon,
                             full_path: *const i8, symname: *const i8) -> i32;
}

pub unsafe fn create_reparse_symlink(xid: u32, inode: *mut inode, dentry: *mut dentry,
                                     tcon: *mut cifs_tcon, full_path: *const i8,
                                     symname: *const i8) -> i32 {
    match cifs_symlink_type(CIFS_SB((*inode).i_sb)) {
        CIFS_SYMLINK_TYPE_NATIVE => create_native_symlink(xid, inode, dentry, tcon, full_path, symname),
        CIFS_SYMLINK_TYPE_NFS => mknod_nfs(xid, inode, dentry, tcon, full_path, S_IFLNK, 0, symname),
        CIFS_SYMLINK_TYPE_WSL => mknod_wsl(xid, inode, dentry, tcon, full_path, S_IFLNK, 0, symname),
        _ => -EOPNOTSUPP,
    }
}

unsafe fn detect_directory_symlink_target(cifs_sb: *mut cifs_sb_info, xid: u32,
                                          full_path: *const i8, symname: *const i8,
                                          directory: *mut bool) -> i32 {
    let sep = CIFS_DIR_SEP(cifs_sb);
    let basename = kbasename(symname);
    let basename_len = strlen(basename);
    if basename_len == 0 || (basename_len == 1 && *basename == b'.' as i8) ||
       (basename_len == 2 && *basename == b'.' as i8 && *basename.add(1) == b'.' as i8) {
        *directory = true;
        return 0;
    }
    if *symname == b'/' as i8 { return 0; }

    let full_path_len = strlen(full_path);
    let symname_len = strlen(symname);
    let tlink = cifs_sb_tlink(cifs_sb);
    if IS_ERR(tlink) { return PTR_ERR(tlink); }
    let resolved_path = kzalloc(full_path_len + symname_len + 1, GFP_KERNEL) as *mut i8;
    if resolved_path.is_null() { cifs_put_tlink(tlink); return -ENOMEM; }
    memcpy(resolved_path as *mut _, full_path as *const _, full_path_len + 1);
    let mut path_sep = strrchr(resolved_path, sep);
    if !path_sep.is_null() { path_sep = path_sep.add(1); } else { path_sep = resolved_path; }
    memcpy(path_sep as *mut _, symname as *const _, symname_len + 1);
    if sep == b'\\' as i8 { convert_delimiter(path_sep, sep); }

    let tcon = tlink_tcon(tlink);
    let mut oparms = CIFS_OPARMS(cifs_sb, tcon, resolved_path, FILE_READ_ATTRIBUTES, FILE_OPEN, 0, ACL_NO_MODE);
    let mut fid = cifs_fid::default();
    oparms.fid = &mut fid;
    let mut oplock = 0u32;
    oparms.create_options = cifs_create_options(cifs_sb, CREATE_NOT_FILE | OPEN_REPARSE_POINT);
    let mut open_rc = (*tcon).ses.server.ops.open(xid, &mut oparms, &mut oplock, core::ptr::null_mut());
    if open_rc == 0 {
        *directory = true;
        (*tcon).ses.server.ops.close(xid, tcon, &mut fid);
    } else if open_rc == -ENOTDIR { *directory = false;
    } else if open_rc != -ENOENT {
        oparms.create_options = cifs_create_options(cifs_sb, CREATE_NOT_DIR | OPEN_REPARSE_POINT);
        open_rc = (*tcon).ses.server.ops.open(xid, &mut oparms, &mut oplock, core::ptr::null_mut());
        if open_rc == 0 { *directory = false; (*tcon).ses.server.ops.close(xid, tcon, &mut fid); }
        else if open_rc == -EISDIR { *directory = true; }
    }
    kfree(resolved_path as *mut _);
    cifs_put_tlink(tlink);
    0
}

unsafe fn create_native_socket(xid: u32, inode: *mut inode, dentry: *mut dentry,
                               tcon: *mut cifs_tcon, full_path: *const i8) -> i32 {
    let mut buf = reparse_data_buffer { ReparseTag: cpu_to_le32(IO_REPARSE_TAG_AF_UNIX), ReparseDataLength: cpu_to_le16(0), ..core::mem::zeroed() };
    let mut data = cifs_open_info_data { reparse_point: true, reparse: reparse_info { tag: IO_REPARSE_TAG_AF_UNIX, buf: &mut buf }, ..core::mem::zeroed() };
    let mut iov = kvec { iov_base: &mut buf as *mut _ as *mut _, iov_len: core::mem::size_of_val(&buf) };
    let new = (*tcon).ses.server.ops.create_reparse_inode(&mut data, (*inode).i_sb, xid, tcon, full_path, false, &mut iov, core::ptr::null_mut());
    let rc = if !IS_ERR(new) { d_instantiate(dentry, new); 0 } else { PTR_ERR(new) };
    cifs_free_open_info(&mut data); rc
}

pub unsafe fn mknod_reparse(xid: u32, inode: *mut inode, dentry: *mut dentry,
                            tcon: *mut cifs_tcon, full_path: *const i8,
                            mode: umode_t, dev: dev_t) -> i32 {
    let ctx = CIFS_SB((*inode).i_sb).ctx;
    if S_ISSOCK(mode) && !(*ctx).nonativesocket && (*ctx).reparse_type != CIFS_REPARSE_TYPE_NONE {
        return create_native_socket(xid, inode, dentry, tcon, full_path);
    }
    match (*ctx).reparse_type {
        CIFS_REPARSE_TYPE_NFS => mknod_nfs(xid, inode, dentry, tcon, full_path, mode, dev, core::ptr::null()),
        CIFS_REPARSE_TYPE_WSL => mknod_wsl(xid, inode, dentry, tcon, full_path, mode, dev, core::ptr::null()),
        _ => -EOPNOTSUPP,
    }
}

pub unsafe fn smb2_parse_native_symlink(target: *mut *mut i8, buf: *const i8, len: u32,
                                        relative: bool, full_path: *const i8,
                                        cifs_sb: *mut cifs_sb_info) -> i32 {
    if len == 0 || len % 2 != 0 { return smb_EIO1(smb_eio_trace_reparse_native_nul, len); }
    let ulen = UniStrnlen(buf as *const _, len as usize / 2);
    if ulen != len as usize / 2 { return smb_EIO2(smb_eio_trace_reparse_native_nul, ulen as _, len); }
    let smb_target = cifs_strndup_from_utf16(buf, len, true, (*cifs_sb).local_nls);
    if smb_target.is_null() { return -ENOMEM; }
    let sep = CIFS_DIR_SEP(cifs_sb);
    let mut linux_target = smb_target;
    if sep == b'\\' as i8 { convert_delimiter(linux_target, b'/'); }
    *target = linux_target;
    0
}

pub unsafe fn parse_reparse_point(buf: *mut reparse_data_buffer, _plen: u32,
                                  _cifs_sb: *mut cifs_sb_info, _full_path: *const i8,
                                  data: *mut cifs_open_info_data) -> i32 {
    (*data).reparse.buf = buf;
    match le32_to_cpu((*buf).ReparseTag) {
        IO_REPARSE_TAG_NFS | IO_REPARSE_TAG_SYMLINK | IO_REPARSE_TAG_LX_SYMLINK => 0,
        IO_REPARSE_TAG_AF_UNIX | IO_REPARSE_TAG_LX_FIFO | IO_REPARSE_TAG_LX_CHR | IO_REPARSE_TAG_LX_BLK => {
            if le16_to_cpu((*buf).ReparseDataLength) != 0 { return -EIO; } 0
        }
        _ => -EOPNOTSUPP,
    }
}

pub unsafe fn smb2_get_reparse_point_buffer(rsp_iov: *const kvec, plen: *mut u32) -> *mut reparse_data_buffer {
    let io = (*rsp_iov).iov_base as *mut smb2_ioctl_rsp;
    *plen = le32_to_cpu((*io).OutputCount);
    (io as *mut u8).add(le32_to_cpu((*io).OutputOffset) as usize) as *mut reparse_data_buffer
}

pub unsafe fn cifs_reparse_point_to_fattr(cifs_sb: *mut cifs_sb_info, fattr: *mut cifs_fattr,
                                          data: *mut cifs_open_info_data) -> bool {
    let tag = (*data).reparse.tag;
    match tag {
        IO_REPARSE_TAG_LX_SYMLINK => (*fattr).cf_mode |= S_IFLNK,
        IO_REPARSE_TAG_LX_FIFO => (*fattr).cf_mode |= S_IFIFO,
        IO_REPARSE_TAG_AF_UNIX => (*fattr).cf_mode |= S_IFSOCK,
        IO_REPARSE_TAG_LX_CHR => (*fattr).cf_mode |= S_IFCHR,
        IO_REPARSE_TAG_LX_BLK => (*fattr).cf_mode |= S_IFBLK,
        IO_REPARSE_TAG_NFS => return posix_reparse_to_fattr(cifs_sb, fattr, data),
        0 | IO_REPARSE_TAG_SYMLINK => (*fattr).cf_mode |= S_IFLNK,
        _ => {
            if (*fattr).cf_cifsattrs & ATTR_DIRECTORY == 0 { return false; }
            if !IS_REPARSE_TAG_NAME_SURROGATE(tag) && tag != IO_REPARSE_TAG_INTERNAL { return false; }
            (*fattr).cf_mode = S_IFDIR | 0o711;
        }
    }
    (*fattr).cf_dtype = S_DT((*fattr).cf_mode); true
}

unsafe fn posix_reparse_to_fattr(_cifs_sb: *mut cifs_sb_info, fattr: *mut cifs_fattr,
                                 data: *mut cifs_open_info_data) -> bool {
    let buf = (*data).reparse.buf as *mut reparse_nfs_data_buffer;
    if buf.is_null() { return true; }
    match le64_to_cpu((*buf).InodeType) {
        NFS_SPECFILE_CHR => (*fattr).cf_mode |= S_IFCHR,
        NFS_SPECFILE_BLK => (*fattr).cf_mode |= S_IFBLK,
        NFS_SPECFILE_FIFO => (*fattr).cf_mode |= S_IFIFO,
        NFS_SPECFILE_SOCK => (*fattr).cf_mode |= S_IFSOCK,
        NFS_SPECFILE_LNK => (*fattr).cf_mode |= S_IFLNK,
        _ => return false,
    } true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
