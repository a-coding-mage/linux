// SPDX-License-Identifier: LGPL-2.1
/* vfs operations that deal with io control */

// Kernel and CIFS declarations are supplied by the surrounding translation unit.

unsafe fn cifs_ioctl_query_info(xid: u32, filep: *mut file, p: c_ulong) -> c_long {
    let inode = file_inode(filep);
    let cifs_sb = CIFS_SB((*inode).i_sb);
    let tcon = cifs_sb_master_tcon(cifs_sb);
    let dentry = (*filep).f_path.dentry;
    let mut path: *const u8;
    let page = alloc_dentry_path();
    let mut utf16_path: *mut le16 = core::ptr::null_mut();
    let mut root_path: le16 = 0;
    let mut rc: c_int = 0;

    path = build_path_from_dentry(dentry, page);
    if IS_ERR(path) { free_dentry_path(page); return PTR_ERR(path) as c_long; }
    cifs_dbg(FYI, "%s %s\n", "cifs_ioctl_query_info", path);
    if *path == 0 { utf16_path = &mut root_path; }
    else {
        utf16_path = cifs_convert_path_to_utf16(path.add(1), cifs_sb);
        if utf16_path.is_null() { rc = -ENOMEM; goto ici_exit; }
    }
    if !(*(*(*tcon).ses).server).ops.ioctl_query_info.is_none() {
        rc = (*(*(*tcon).ses).server).ops.ioctl_query_info.unwrap()(xid, tcon, cifs_sb, utf16_path,
            if (*filep).private_data.is_null() { 1 } else { 0 }, p);
    } else { rc = -EOPNOTSUPP; }
ici_exit:
    if utf16_path != &mut root_path { kfree(utf16_path); }
    free_dentry_path(page);
    rc as c_long
}

unsafe fn cifs_set_compression_by_path(xid: u32, filep: *mut file, tcon: *mut cifs_tcon,
                                       compression_state: u16) -> c_int {
    let inode = file_inode(filep);
    let cifs_sb = CIFS_SB((*inode).i_sb);
    let server = (*(*tcon).ses).server;
    let mut oparms: cifs_open_parms;
    let mut data: cifs_open_info_data = core::mem::zeroed();
    let mut tmp_cfile: *mut cifsFileInfo = core::ptr::null_mut();
    let mut fid: cifs_fid = core::mem::zeroed();
    let page = alloc_dentry_path();
    let mut oplock: u32 = 0;
    let mut rc: c_int;
    if (*server).ops.open.is_none() || (*server).ops.close.is_none() || (*server).ops.query_file_info.is_none() { return -EOPNOTSUPP; }
    if cifs_sb_flags(cifs_sb) & CIFS_MOUNT_SERVER_INUM == 0 || (*cifs_sb).mnt_cifs_serverino_autodisabled { return -EOPNOTSUPP; }
    if d_unhashed((*filep).f_path.dentry) { return -ESTALE; }
    let full_path = build_path_from_dentry((*filep).f_path.dentry, page);
    if IS_ERR(full_path) { free_dentry_path(page); return PTR_ERR(full_path) as c_int; }
    oparms = CIFS_OPARMS(cifs_sb, tcon, full_path, FILE_WRITE_DATA | FILE_READ_ATTRIBUTES, FILE_OPEN, 0, ACL_NO_MODE);
    oparms.fid = &mut fid;
    rc = (*server).ops.open.unwrap()(xid, &mut oparms, &mut oplock, core::ptr::null_mut());
    if rc != 0 { goto out; }
    tmp_cfile = kzalloc_obj();
    if tmp_cfile.is_null() { rc = -ENOMEM; goto close; }
    (*tmp_cfile).fid = fid;
    rc = (*server).ops.query_file_info.unwrap()(xid, tcon, tmp_cfile, &mut data);
    if rc != 0 { goto close; }
    let uniqueid = le64_to_cpu(data.fi.IndexNumber);
    if uniqueid != (*CIFS_I(inode)).uniqueid { rc = -ESTALE; goto close; }
    rc = (*server).ops.set_compression.unwrap()(xid, tcon, tmp_cfile, compression_state);
close:
    (*server).ops.close.unwrap()(xid, tcon, &mut fid);
    kfree(tmp_cfile);
    cifs_free_open_info(&mut data);
out:
    free_dentry_path(page); rc
}

unsafe fn cifs_ioctl_set_compression(xid: u32, filep: *mut file, tcon: *mut cifs_tcon,
                                     cfile: *mut cifsFileInfo, compression_state: u16) -> c_int {
    let inode = file_inode(filep);
    if (*(*tcon).ses).server.ops.set_compression.is_none() { return -EOPNOTSUPP; }
    if !cfile.is_null() && (*cfile).fid.access & FILE_WRITE_DATA != 0 {
        let rc = (*(*(*tcon).ses).server).ops.set_compression.unwrap()(xid, tcon, cfile, compression_state);
        if rc != -EACCES { return rc; }
    }
    let mut wfile: *mut cifsFileInfo = core::ptr::null_mut();
    let mut rc = cifs_get_writable_file(CIFS_I(inode), FIND_FSUID_ONLY, &mut wfile);
    if rc == 0 { let wtcon = tlink_tcon((*wfile).tlink); rc = (*(*(*wtcon).ses).server).ops.set_compression.unwrap()(xid, wtcon, wfile, compression_state); cifsFileInfo_put(wfile); if rc != -EACCES { return rc; } }
    else if rc != -EBADF { return rc; }
    cifs_set_compression_by_path(xid, filep, tcon, compression_state)
}

unsafe fn cifs_ioctl_copychunk(xid: u32, dst_file: *mut file, srcfd: c_ulong) -> c_long {
    if (*dst_file).f_mode & FMODE_WRITE == 0 { return -EINVAL as c_long; }
    let mut rc = mnt_want_write_file(dst_file); if rc != 0 { return rc as c_long; }
    let src_file = fdget(srcfd); if src_file.is_null() { rc = -EBADF; goto out_drop_write; }
    if (*(*src_file).f_op).unlocked_ioctl != Some(cifs_ioctl) { rc = -EBADF; goto out_drop_write; }
    let src_inode = file_inode(src_file); rc = -EINVAL;
    if S_ISDIR((*src_inode).i_mode) { goto out_drop_write; }
    rc = cifs_file_copychunk_range(xid, src_file, 0, dst_file, 0, (*src_inode).i_size, 0);
    if rc > 0 { rc = 0; }
out_drop_write:
    mnt_drop_write_file(dst_file); rc as c_long
}

unsafe fn smb_mnt_get_tcon_info(tcon: *mut cifs_tcon, arg: *mut c_void) -> c_long {
    let mut info: smb_mnt_tcon_info = core::mem::zeroed(); info.tid = (*tcon).tid; info.session_id = (*(*tcon).ses).Suid;
    if copy_to_user(arg, &info, core::mem::size_of::<smb_mnt_tcon_info>()) != 0 { -EFAULT as c_long } else { 0 }
}

unsafe fn smb_mnt_get_fsinfo(_xid: u32, tcon: *mut cifs_tcon, arg: *mut c_void) -> c_long {
    let fsinf: *mut smb_mnt_fs_info = kzalloc_obj(); if fsinf.is_null() { return -ENOMEM as c_long; }
    (*fsinf).version=1; (*fsinf).protocol_id=(*(*(*tcon).ses).server).vals.protocol_id; (*fsinf).tcon_flags=(*tcon).Flags;
    (*fsinf).device_characteristics=le32_to_cpu((*tcon).fsDevInfo.DeviceCharacteristics); (*fsinf).device_type=le32_to_cpu((*tcon).fsDevInfo.DeviceType);
    (*fsinf).fs_attributes=le32_to_cpu((*tcon).fsAttrInfo.Attributes); (*fsinf).max_path_component=le32_to_cpu((*tcon).fsAttrInfo.MaxPathNameComponentLength);
    (*fsinf).vol_serial_number=(*tcon).vol_serial_number; (*fsinf).vol_create_time=le64_to_cpu((*tcon).vol_create_time); (*fsinf).share_flags=(*tcon).share_flags;
    (*fsinf).share_caps=le32_to_cpu((*tcon).capabilities); (*fsinf).sector_flags=(*tcon).ss_flags; (*fsinf).optimal_sector_size=(*tcon).perf_sector_size;
    (*fsinf).max_bytes_chunk=(*tcon).max_bytes_chunk; (*fsinf).maximal_access=(*tcon).maximal_access; (*fsinf).cifs_posix_caps=le64_to_cpu((*tcon).fsUnixInfo.Capability);
    let rc = if copy_to_user(arg, fsinf, core::mem::size_of::<smb_mnt_fs_info>()) != 0 { -EFAULT } else { 0 }; kfree(fsinf); rc as c_long
}

// The remaining ioctl dispatch is a direct low-level translation; declarations referenced below are supplied externally.
pub unsafe fn cifs_ioctl(filep: *mut file, command: u32, arg: c_ulong) -> c_long {
    let inode = file_inode(filep); let mut rc: c_long = -ENOTTY as c_long; let xid=get_xid();
    let pSMBFile=(*filep).private_data as *mut cifsFileInfo;
    if pSMBFile.is_null() { trace_smb3_ioctl(xid,0,command); } else { trace_smb3_ioctl(xid,(*pSMBFile).fid.persistent_fid,command); }
    match command {
        CIFS_IOC_COPYCHUNK_FILE => { rc=cifs_ioctl_copychunk(xid,filep,arg); }
        CIFS_QUERY_INFO => { rc=cifs_ioctl_query_info(xid,filep,arg); }
        CIFS_IOC_GET_MNT_INFO => { if !pSMBFile.is_null() { rc=smb_mnt_get_fsinfo(xid,tlink_tcon((*pSMBFile).tlink),arg as *mut c_void); } }
        CIFS_IOC_GET_TCON_INFO => { let sb=CIFS_SB((*inode).i_sb); let l=cifs_sb_tlink(sb); if !IS_ERR(l) { rc=smb_mnt_get_tcon_info(tlink_tcon(l),arg as *mut c_void); cifs_put_tlink(l); } else { rc=PTR_ERR(l) as c_long; } }
        CIFS_IOC_SHUTDOWN => { rc=cifs_shutdown((*inode).i_sb,arg) as c_long; }
        _ => { cifs_dbg(FYI,"unsupported ioctl\n"); trace_smb3_unsupported_ioctl(xid,if pSMBFile.is_null(){0}else{(*pSMBFile).fid.persistent_fid},command); }
    }
    free_xid(xid); rc
}

unsafe fn cifs_shutdown(sb: *mut super_block, arg: c_ulong) -> c_int {
    let sbi=CIFS_SB(sb); if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let mut flags:u32=0; if get_user(&mut flags,arg as *mut u32)!=0 { return -EFAULT; }
    let tlink=cifs_sb_tlink(sbi); if IS_ERR(tlink) { return PTR_ERR(tlink) as c_int; } let tcon=tlink_tcon(tlink);
    trace_smb3_shutdown_enter(flags,(*tcon).tid);
    if flags>CIFS_GOING_FLAGS_NOLOGFLUSH { cifs_put_tlink(tlink); return -EINVAL; }
    if cifs_forced_shutdown(sbi) { trace_smb3_shutdown_done(flags,(*tcon).tid); cifs_put_tlink(tlink); return 0; }
    match flags { CIFS_GOING_FLAGS_DEFAULT => { cifs_put_tlink(tlink); -EINVAL }, CIFS_GOING_FLAGS_LOGFLUSH|CIFS_GOING_FLAGS_NOLOGFLUSH => { atomic_or(CIFS_MOUNT_SHUTDOWN,&mut (*sbi).mnt_cifs_flags); trace_smb3_shutdown_done(flags,(*tcon).tid); cifs_put_tlink(tlink); 0 }, _ => { trace_smb3_shutdown_err(-EINVAL,flags,(*tcon).tid); cifs_put_tlink(tlink); -EINVAL } }
}

unsafe fn cifs_dump_full_key(tcon:*mut cifs_tcon, input:*mut smb3_full_key_debug_info)->c_int {
    if !smb3_encryption_required(tcon) { return -EOPNOTSUPP; }
    let mut out:smb3_full_key_debug_info=core::mem::zeroed(); if copy_from_user(&mut out,input,core::mem::size_of_val(&out))!=0{return -EINVAL;}
    let ses=(*tcon).ses; let cipher=le16_to_cpu((*(*ses).server).cipher_type);
    match cipher { SMB2_ENCRYPTION_AES128_CCM|SMB2_ENCRYPTION_AES128_GCM=>{out.session_key_length=CIFS_SESS_KEY_SIZE;out.server_in_key_length=SMB3_GCM128_CRYPTKEY_SIZE;out.server_out_key_length=SMB3_GCM128_CRYPTKEY_SIZE;}, SMB2_ENCRYPTION_AES256_CCM|SMB2_ENCRYPTION_AES256_GCM=>{out.session_key_length=(*ses).auth_key.len;out.server_in_key_length=SMB3_GCM256_CRYPTKEY_SIZE;out.server_out_key_length=SMB3_GCM256_CRYPTKEY_SIZE;}, _=>return -EOPNOTSUPP }
    if out.in_size < core::mem::size_of_val(&out)+out.session_key_length+out.server_in_key_length+out.server_out_key_length{return -ENOBUFS;}
    out.session_id=(*ses).Suid;out.cipher_type=cipher;if copy_to_user(input,&out,core::mem::size_of_val(&out))!=0{return -EINVAL;}
    let mut end=input.data;if copy_to_user(end,(*ses).auth_key.response,out.session_key_length)!=0{return -EINVAL;}end=end.add(out.session_key_length);
    if copy_to_user(end,(*ses).smb3encryptionkey,out.server_in_key_length)!=0{return -EINVAL;}end=end.add(out.server_in_key_length);
    if copy_to_user(end,(*ses).smb3decryptionkey,out.server_out_key_length)!=0{return -EINVAL;}0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
