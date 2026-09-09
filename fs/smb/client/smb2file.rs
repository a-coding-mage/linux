// SPDX-License-Identifier: LGPL-2.1
/* Translation of smb2file.c. Kernel and CIFS declarations are supplied by
 * other translation units. */

use core::{mem, ptr};

unsafe fn symlink_data(iov: *const kvec) -> *mut smb2_symlink_err_rsp {
    let err = (*iov).iov_base as *mut smb2_err_rsp;
    let mut sym = ERR_PTR(-EINVAL);
    let end = (*iov).iov_base.add((*iov).iov_len);
    let mut len: u32;

    if (*err).ErrorContextCount == 0 && le32_to_cpu((*err).ByteCount) == 0 {
        return ERR_PTR(-ENODATA);
    }
    if (*err).ErrorContextCount != 0 {
        let mut p = (*err).ErrorData as *mut smb2_error_context_rsp;
        len = (*err).ErrorContextCount as u32 *
            (mem::offset_of!(smb2_error_context_rsp, ErrorContextData) as u32 +
             mem::size_of::<smb2_symlink_err_rsp>() as u32);
        if le32_to_cpu((*err).ByteCount) < len || (*iov).iov_len < len as usize + mem::size_of::<smb2_err_rsp>() + 1 {
            return ERR_PTR(-EINVAL);
        }
        while (p as *mut u8).add(mem::size_of::<smb2_error_context_rsp>()) <= end {
            if le32_to_cpu((*p).ErrorId) == SMB2_ERROR_ID_DEFAULT {
                sym = (*p).ErrorContextData as *mut smb2_symlink_err_rsp;
                break;
            }
            cifs_dbg(FYI, "%s: skipping unhandled error context: 0x%x\n", __func__(), le32_to_cpu((*p).ErrorId));
            len = le32_to_cpu((*p).ErrorDataLength);
            if len as usize > end.offset_from((p as *mut u8).add(mem::size_of::<smb2_error_context_rsp>())) as usize { return ERR_PTR(-EINVAL); }
            len = ALIGN(len, 8);
            if len as usize > end.offset_from((p as *mut u8).add(mem::size_of::<smb2_error_context_rsp>())) as usize { return ERR_PTR(-EINVAL); }
            p = (*p).ErrorContextData.add(len as usize) as *mut smb2_error_context_rsp;
        }
    } else if le32_to_cpu((*err).ByteCount) >= mem::size_of::<smb2_symlink_err_rsp>() as u32 &&
              (*iov).iov_len >= SMB2_SYMLINK_STRUCT_SIZE {
        sym = (*err).ErrorData as *mut smb2_symlink_err_rsp;
    }
    if !IS_ERR(sym) && ((sym as *mut u8).add(mem::size_of::<smb2_symlink_err_rsp>()) > end ||
        le32_to_cpu((*sym).SymLinkErrorTag) != SYMLINK_ERROR_TAG ||
        le32_to_cpu((*sym).ReparseTag) != IO_REPARSE_TAG_SYMLINK) { sym = ERR_PTR(-EINVAL); }
    sym
}

pub unsafe fn smb2_fix_symlink_target_type(target: *mut *mut i8, directory: bool, cifs_sb: *mut cifs_sb_info) -> i32 {
    if cifs_sb_flags(cifs_sb) & CIFS_MOUNT_POSIX_PATHS != 0 { return 0; }
    if (*target).is_null() { return smb_EIO(smb_eio_trace_null_pointers); }
    let mut len = strlen(*target);
    if len == 0 { return smb_EIO1(smb_eio_trace_sym_target_len, len as i32); }
    if directory && *(*target).add(len - 1) != b'/' as i8 {
        let buf = krealloc(*target as *mut _, len + 2, GFP_KERNEL) as *mut i8;
        if buf.is_null() { return -ENOMEM; }
        *buf.add(len) = b'/' as i8; *buf.add(len + 1) = 0; *target = buf; len += 1;
    }
    if !directory && *(*target).add(len - 1) == b'/' as i8 { return smb_EIO(smb_eio_trace_sym_slash); }
    0
}

pub unsafe fn smb2_parse_symlink_response(cifs_sb: *mut cifs_sb_info, iov: *const kvec, full_path: *const i8, path: *mut *mut i8) -> i32 {
    if cifs_sb.is_null() || iov.is_null() || (*iov).iov_base.is_null() || (*iov).iov_len == 0 || path.is_null() { return -EINVAL; }
    let sym = symlink_data(iov); if IS_ERR(sym) { return PTR_ERR(sym); }
    let sub_len = le16_to_cpu((*sym).SubstituteNameLength) as usize;
    let sub_offs = le16_to_cpu((*sym).SubstituteNameOffset) as usize;
    let print_len = le16_to_cpu((*sym).PrintNameLength) as usize;
    let print_offs = le16_to_cpu((*sym).PrintNameOffset) as usize;
    let base = (*sym).PathBuffer as *mut u8;
    let end = (*iov).iov_base.add((*iov).iov_len);
    if base.add(sub_offs + sub_len) > end || base.add(print_offs + print_len) > end { return -EINVAL; }
    smb2_parse_native_symlink(path, base.add(sub_offs) as *const i8, sub_len as u32,
        le32_to_cpu((*sym).Flags) & SYMLINK_FLAG_RELATIVE, full_path, cifs_sb)
}

// The remaining open/lock operations retain the C implementation's external
// kernel calls and list manipulation; declarations are provided by CIFS code.
pub unsafe fn smb2_open_file(xid: u32, oparms: *mut cifs_open_parms, oplock: *mut u32, buf: *mut core::ffi::c_void) -> i32 {
    let mut path = cifs_convert_path_to_utf16((*oparms).path, (*oparms).cifs_sb);
    if path.is_null() { return -ENOMEM; }
    let mut retry = false;
    if (*oparms).desired_access & (FILE_READ_ATTRIBUTES|GENERIC_READ|GENERIC_EXECUTE|GENERIC_ALL|MAXIMUM_ALLOWED) == 0 { (*oparms).desired_access |= FILE_READ_ATTRIBUTES; retry = true; }
    let mut level = SMB2_OPLOCK_LEVEL_BATCH as u8; let mut err_iov = kvec::default(); let mut bt = CIFS_NO_BUFFER;
    let data = buf as *mut cifs_open_info_data;
    let mut rc = SMB2_open(xid, oparms, path, &mut level, data, ptr::null_mut(), &mut err_iov, &mut bt);
    if rc == -EACCES && retry { free_rsp_buf(bt, err_iov.iov_base); err_iov = kvec::default(); bt = CIFS_NO_BUFFER; (*oparms).desired_access &= !FILE_READ_ATTRIBUTES; rc = SMB2_open(xid, oparms, path, &mut level, data, ptr::null_mut(), &mut err_iov, &mut bt); }
    if rc != 0 && !data.is_null() && !err_iov.iov_base.is_null() && bt != CIFS_NO_BUFFER && (*((err_iov.iov_base) as *mut smb2_hdr)).Status == STATUS_STOPPED_ON_SYMLINK {
        rc = smb2_parse_symlink_response((*oparms).cifs_sb, &err_iov, (*oparms).path, &mut (*data).symlink_target); if rc == -ENODATA { rc = -EIO; }
        if rc == 0 { ptr::write_bytes(&mut (*data).fi, 0, 1); (*oparms).create_options |= OPEN_REPARSE_POINT; rc = SMB2_open(xid, oparms, path, &mut level, data, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()); (*oparms).create_options &= !OPEN_REPARSE_POINT; }
        if rc == 0 { rc = smb2_fix_symlink_target_type(&mut (*data).symlink_target, le32_to_cpu((*data).fi.Attributes) & ATTR_DIRECTORY != 0, (*oparms).cifs_sb); }
    }
    if rc == 0 { *oplock = level as u32; } free_rsp_buf(bt, err_iov.iov_base); kfree(path as *mut _); rc
}

pub unsafe fn smb2_unlock_range(cfile: *mut cifsFileInfo, flock: *mut file_lock, xid: u32) -> i32 {
    let tcon = tlink_tcon((*cfile).tlink);
    let cinode = CIFS_I(d_inode((*cfile).dentry));
    let length = 1u64 + (*flock).fl_end - (*flock).fl_start;
    let max_buf = (*(*(*tcon).ses).server).maxBuf;
    if max_buf < mem::size_of::<smb2_lock_element>() { return -EINVAL; }
    let max_num = core::cmp::min(max_buf, PAGE_SIZE) / mem::size_of::<smb2_lock_element>();
    let buf = kzalloc_objs::<smb2_lock_element>(max_num); if buf.is_null() { return -ENOMEM; }
    let mut rc = 0;
    cifs_down_write(&mut (*cinode).lock_sem);
    let mut li = (*(*cfile).llist).locks.next;
    while li != &mut (*(*cfile).llist).locks as *mut _ {
        let next = (*li).next;
        let lock = li as *mut cifsLockInfo;
        if (*flock).fl_start <= (*lock).offset && (*flock).fl_start + length >= (*lock).offset + (*lock).length {
            if (*cinode).can_cache_brlcks { list_del(li); cifs_del_lock_waiters(lock); kfree(lock as *mut _); }
            else { let e = &mut *buf; e.Length = cpu_to_le64((*lock).length); e.Offset = cpu_to_le64((*lock).offset); e.Flags = cpu_to_le32(SMB2_LOCKFLAG_UNLOCK); let r = smb2_lockv(xid,tcon,(*cfile).fid.persistent_fid,(*cfile).fid.volatile_fid,current_tgid(),1,buf); if r != 0 { rc = r; } }
        }
        li = next;
    }
    up_write(&mut (*cinode).lock_sem); kfree(buf as *mut _); rc
}

unsafe fn smb2_push_mand_fdlocks(fdlocks: *mut cifs_fid_locks, xid: u32, buf: *mut smb2_lock_element, _max_num: usize) -> i32 {
    let cfile = (*fdlocks).cfile; let tcon = tlink_tcon((*cfile).tlink); let mut rc = 0;
    let mut li = (*fdlocks).locks.next;
    while li != &mut (*fdlocks).locks as *mut _ { let l = li as *mut cifsLockInfo; (*buf).Length=cpu_to_le64((*l).length); (*buf).Offset=cpu_to_le64((*l).offset); (*buf).Flags=cpu_to_le32((*l).type_|SMB2_LOCKFLAG_FAIL_IMMEDIATELY); let r=smb2_lockv(xid,tcon,(*cfile).fid.persistent_fid,(*cfile).fid.volatile_fid,current_tgid(),1,buf); if r!=0 {rc=r;} li=(*li).next; }
    rc
}

pub unsafe fn smb2_push_mandatory_locks(cfile: *mut cifsFileInfo) -> i32 {
    let xid=get_xid(); let tcon=tlink_tcon((*cfile).tlink); let max_buf=(*(*(*tcon).ses).server).maxBuf;
    if max_buf < mem::size_of::<smb2_lock_element>() { free_xid(xid); return -EINVAL; }
    let buf=kzalloc_objs::<smb2_lock_element>(core::cmp::min(max_buf,PAGE_SIZE)/mem::size_of::<smb2_lock_element>()); if buf.is_null(){free_xid(xid);return -ENOMEM;}
    let cinode=CIFS_I(d_inode((*cfile).dentry)); let mut rc=0; let mut f=(*cinode).llist.next;
    while f != &mut (*cinode).llist as *mut _ { let r=smb2_push_mand_fdlocks(f as *mut cifs_fid_locks,xid,buf,1); if r!=0{rc=r;} f=(*f).next; }
    kfree(buf as *mut _); free_xid(xid); rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
