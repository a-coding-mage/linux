// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of copy_up.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const OVL_COPY_UP_CHUNK_SIZE: usize = 1 << 20;

// Kernel and overlayfs types/functions are supplied by the surrounding build.
extern "C" {
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
}

unsafe fn ovl_ccup_set(_buf: *const c_char, _param: *const kernel_param) -> c_int { 0 }
unsafe fn ovl_ccup_get(buf: *mut c_char, _param: *const kernel_param) -> c_int {
    sprintf(buf, b"N\0".as_ptr() as *const c_char)
}

#[inline]
unsafe fn ovl_must_copy_xattr(name: *const c_char) -> bool {
    strcmp(name, XATTR_POSIX_ACL_ACCESS) == 0 ||
    strcmp(name, XATTR_POSIX_ACL_DEFAULT) == 0 ||
    strncmp(name, XATTR_SECURITY_PREFIX, XATTR_SECURITY_PREFIX_LEN) == 0
}

unsafe fn ovl_copy_acl(ofs: *mut ovl_fs, path: *const path, dentry: *mut dentry, acl_name: *const c_char) -> c_int {
    let real_acl = ovl_get_acl_path(path, acl_name, false);
    if real_acl.is_null() { return 0; }
    if IS_ERR(real_acl) {
        let err = PTR_ERR(real_acl);
        if err == -ENODATA || err == -EOPNOTSUPP { return 0; }
        return err;
    }
    let clone = posix_acl_clone(real_acl, GFP_KERNEL);
    posix_acl_release(real_acl);
    if clone.is_null() { return -ENOMEM; }
    let err = ovl_do_set_acl(ofs, dentry, acl_name, clone);
    posix_acl_release(clone);
    err
}

pub unsafe fn ovl_copy_xattr(sb: *mut super_block, oldpath: *const path, new: *mut dentry) -> isize {
    let old = (*oldpath).dentry;
    if (*(*old).d_inode).i_op.is_null() || (*(*new).d_inode).i_op.is_null() { return 0; }
    let mut list_size = vfs_listxattr(old, core::ptr::null_mut(), 0);
    if list_size <= 0 { return if list_size == -EOPNOTSUPP as isize { 0 } else { list_size }; }
    let buf = kvzalloc(list_size as usize, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -ENOMEM as isize; }
    list_size = vfs_listxattr(old, buf, list_size as usize);
    if list_size <= 0 { kvfree(buf as *mut c_void); return list_size; }
    let mut name = buf;
    let mut value: *mut c_void = core::ptr::null_mut();
    let mut value_size: isize = 0;
    let mut error: isize = 0;
    while list_size != 0 {
        let slen = strnlen(name, list_size as usize) + 1;
        if slen > list_size as usize { error = -EIO as isize; break; }
        list_size -= slen as isize; name = name.add(slen);
        if ovl_is_private_xattr(sb, name) { continue; }
        error = security_inode_copy_up_xattr(old, name);
        if error == -ECANCELED as isize { error = 0; continue; }
        if error < 0 && error != -EOPNOTSUPP as isize { break; }
        if is_posix_acl_xattr(name) {
            error = ovl_copy_acl(OVL_FS(sb), oldpath, new, name) as isize;
            if error == 0 { continue; }
            break;
        }
        loop {
            let mut size = ovl_do_getxattr(oldpath, name, value, value_size as usize) as isize;
            if size == -ERANGE as isize { size = ovl_do_getxattr(oldpath, name, core::ptr::null_mut(), 0) as isize; }
            if size < 0 { error = size; break; }
            if size > value_size {
                let n = kvmalloc(size as usize, GFP_KERNEL);
                if n.is_null() { error = -ENOMEM as isize; break; }
                kvfree(value); value = n; value_size = size; continue;
            }
            error = ovl_do_setxattr(OVL_FS(sb), new, name, value, size as usize, 0) as isize;
            if error != 0 {
                if error != -EOPNOTSUPP as isize || ovl_must_copy_xattr(name) { break; }
                error = 0;
            }
            break;
        }
        if error != 0 { break; }
    }
    kvfree(value); kvfree(buf as *mut c_void); error
}

unsafe fn ovl_verify_area(pos: loff_t, pos2: loff_t, len: loff_t, totlen: loff_t) -> c_int {
    if pos != pos2 || pos < 0 || len < 0 || totlen < 0 { return -EIO; }
    let (tmp, overflow) = pos.overflowing_add(len);
    if overflow || tmp < 0 { return -EIO; } 0
}

unsafe fn ovl_sync_file(path: *const path) -> c_int {
    let f = ovl_path_open(path, O_LARGEFILE | O_RDONLY); if IS_ERR(f) { return PTR_ERR(f); }
    let err = vfs_fsync(f, 0); fput(f); err
}

unsafe fn ovl_copy_up_file(ofs: *mut ovl_fs, dentry: *mut dentry, new_file: *mut file, mut len: loff_t, datasync: bool) -> c_int {
    let mut datapath = core::mem::zeroed::<path>(); ovl_path_lowerdata(dentry, &mut datapath);
    if (*datapath.dentry).is_null() || len < 0 { return -EIO; }
    let old_file = ovl_path_open(&datapath, O_LARGEFILE | O_RDONLY); if IS_ERR(old_file) { return PTR_ERR(old_file); }
    let cloned = vfs_clone_file_range(old_file, 0, new_file, 0, len, 0);
    if cloned != len {
        let mut old_pos = 0; let mut new_pos = 0;
        let mut err = rw_verify_area(READ, old_file, &mut old_pos, len);
        if err == 0 { err = rw_verify_area(WRITE, new_file, &mut new_pos, len); }
        if err == 0 {
            while len != 0 {
                if signal_pending_state(TASK_KILLABLE, current) { err = -EINTR; break; }
                let this_len = core::cmp::min(len, OVL_COPY_UP_CHUNK_SIZE as i64);
                err = ovl_verify_area(old_pos, new_pos, this_len, len); if err != 0 { break; }
                let bytes = do_splice_direct(old_file, &mut old_pos, new_file, &mut new_pos, this_len as usize, SPLICE_F_MOVE);
                if bytes <= 0 { err = bytes as c_int; break; } len -= bytes as i64;
            }
        }
        if err == 0 && ovl_should_sync(ofs) && datasync { err = vfs_fsync(new_file, 0); }
        fput(old_file); return err;
    }
    fput(old_file); 0
}

unsafe fn ovl_set_size(ofs: *mut ovl_fs, upperdentry: *mut dentry, stat: *mut kstat) -> c_int {
    let mut attr = iattr { ia_valid: ATTR_SIZE, ia_size: (*stat).size, ..core::mem::zeroed() };
    ovl_do_notify_change(ofs, upperdentry, &mut attr)
}
unsafe fn ovl_set_timestamps(ofs: *mut ovl_fs, upperdentry: *mut dentry, stat: *mut kstat) -> c_int {
    let mut attr = iattr { ia_valid: ATTR_ATIME|ATTR_MTIME|ATTR_ATIME_SET|ATTR_MTIME_SET|ATTR_CTIME, ia_atime: (*stat).atime, ia_mtime: (*stat).mtime, ..core::mem::zeroed() };
    ovl_do_notify_change(ofs, upperdentry, &mut attr)
}
pub unsafe fn ovl_set_attr(ofs: *mut ovl_fs, upperdentry: *mut dentry, stat: *mut kstat) -> c_int {
    let mut err = 0;
    if !S_ISLNK((*stat).mode) { let mut a = iattr { ia_valid: ATTR_MODE, ia_mode: (*stat).mode, ..core::mem::zeroed() }; err = ovl_do_notify_change(ofs, upperdentry, &mut a); }
    if err == 0 { let mut a = iattr { ia_valid: ATTR_UID|ATTR_GID, ia_vfsuid: VFSUIDT_INIT((*stat).uid), ia_vfsgid: VFSGIDT_INIT((*stat).gid), ..core::mem::zeroed() }; err = ovl_do_notify_change(ofs, upperdentry, &mut a); }
    if err == 0 { ovl_set_timestamps(ofs, upperdentry, stat); } err
}

// The remaining routines retain the source interfaces and delegate to the same
// kernel overlayfs operations; declarations are intentionally external.
pub unsafe fn ovl_maybe_copy_up(dentry: *mut dentry, flags: c_int) -> c_int { if !ovl_open_need_copy_up(dentry, flags) { 0 } else { ovl_copy_up_flags(dentry, flags) } }
pub unsafe fn ovl_copy_up_with_data(dentry: *mut dentry) -> c_int { ovl_copy_up_flags(dentry, O_WRONLY) }
pub unsafe fn ovl_copy_up(dentry: *mut dentry) -> c_int { ovl_copy_up_flags(dentry, 0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
