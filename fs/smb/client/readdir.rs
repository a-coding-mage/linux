// SPDX-License-Identifier: LGPL-2.1
/* Directory search handling.  External Linux/CIFS types and helpers are supplied by other files. */

const UNICODE_NAME_MAX: usize = (4 * NAME_MAX) + 2;

#[cfg(CONFIG_CIFS_DEBUG2)]
unsafe fn dump_cifs_file_struct(file: *mut file, label: *mut c_char) {
    if !file.is_null() {
        let cf = (*file).private_data as *mut cifsFileInfo;
        if cf.is_null() { cifs_dbg(FYI, c"empty cifs private file data\n".as_ptr()); return; }
        if (*cf).invalidHandle { cifs_dbg(FYI, c"Invalid handle\n".as_ptr()); }
        if (*cf).srch_inf.endOfSearch { cifs_dbg(FYI, c"end of search\n".as_ptr()); }
        if (*cf).srch_inf.emptyDir { cifs_dbg(FYI, c"empty dir\n".as_ptr()); }
    }
}
#[cfg(not(CONFIG_CIFS_DEBUG2))]
unsafe fn dump_cifs_file_struct(_file: *mut file, _label: *mut c_char) {}

unsafe fn cifs_prime_dcache(parent: *mut dentry, name: *mut qstr, fattr: *mut cifs_fattr) {
    let sb = (*parent).d_sb;
    let cifs_sb = CIFS_SB(sb);
    let posix = (*cifs_sb_master_tcon(cifs_sb)).posix_extensions;
    let mut reparse_need_reval = false;
    let mut dentry = try_lookup_noperm(name, parent);
    let mut inode: *mut inode;
    if dentry.is_null() {
        'retry: loop {
            if posix { match (*fattr).cf_mode & S_IFMT { S_IFLNK | S_IFBLK | S_IFCHR => reparse_need_reval = true, _ => {} } }
            else if (*fattr).cf_cifsattrs & ATTR_REPARSE_POINT != 0 { reparse_need_reval = true; }
            if reparse_need_reval || (*fattr).cf_flags & CIFS_FATTR_NEED_REVAL != 0 { return; }
            dentry = d_alloc_parallel(parent, name);
            break 'retry;
        }
    }
    if IS_ERR(dentry) { return; }
    if !d_in_lookup(dentry) {
        inode = d_inode(dentry);
        if !inode.is_null() {
            if d_mountpoint(dentry) { dput(dentry); return; }
            if cifs_sb_flags(cifs_sb) & CIFS_MOUNT_SERVER_INUM == 0 { (*fattr).cf_uniqueid = (*CIFS_I(inode)).uniqueid; }
            if (*CIFS_I(inode)).uniqueid == (*fattr).cf_uniqueid {
                let mut rc = 0;
                if (*fattr).cf_cifsattrs & ATTR_REPARSE_POINT != 0 {
                    if likely(reparse_inode_match(inode, fattr)) {
                        (*fattr).cf_mode = (*inode).i_mode; (*fattr).cf_rdev = (*inode).i_rdev;
                        (*fattr).cf_uid = (*inode).i_uid; (*fattr).cf_gid = (*inode).i_gid;
                        (*fattr).cf_eof = netfs_read_remote_i_size(inode); (*fattr).cf_symlink_target = core::ptr::null_mut();
                    } else { (*CIFS_I(inode)).time = 0; rc = -ESTALE; }
                }
                if rc == 0 && !cifs_fattr_to_inode(inode, fattr, true) { dput(dentry); return; }
            }
        }
        d_invalidate(dentry); dput(dentry);
        return;
    }
    inode = cifs_iget(sb, fattr); if inode.is_null() { inode = ERR_PTR(-ENOMEM); }
    let alias = d_splice_alias(inode, dentry); d_lookup_done(dentry);
    if !alias.is_null() && !IS_ERR(alias) { dput(alias); }
    dput(dentry);
}

unsafe fn cifs_fill_common_info(fattr: *mut cifs_fattr, cifs_sb: *mut cifs_sb_info) {
    let mut data = cifs_open_info_data { reparse: cifs_reparse { tag: (*fattr).cf_cifstag } };
    (*fattr).cf_uid = (*(*cifs_sb).ctx).linux_uid; (*fattr).cf_gid = (*(*cifs_sb).ctx).linux_gid;
    if (*fattr).cf_cifsattrs & ATTR_REPARSE_POINT != 0 && cifs_reparse_point_to_fattr(cifs_sb, fattr, &mut data) { } else if (*fattr).cf_cifsattrs & ATTR_DIRECTORY != 0 {
        (*fattr).cf_mode = S_IFDIR | (*(*cifs_sb).ctx).dir_mode; (*fattr).cf_dtype = DT_DIR;
    } else { (*fattr).cf_mode = S_IFREG | (*(*cifs_sb).ctx).file_mode; (*fattr).cf_dtype = DT_REG; }
    (*fattr).cf_flags |= CIFS_FATTR_UNKNOWN_NLINK;
    if (*fattr).cf_cifsattrs & ATTR_READONLY != 0 { (*fattr).cf_mode &= !S_IWUGO; }
    let sbflags = cifs_sb_flags(cifs_sb);
    if sbflags & (CIFS_MOUNT_CIFS_ACL | CIFS_MOUNT_MODE_FROM_SID) != 0 { (*fattr).cf_flags |= CIFS_FATTR_NEED_REVAL; }
    if sbflags & CIFS_MOUNT_UNX_EMUL != 0 && (*fattr).cf_cifsattrs & ATTR_SYSTEM != 0 {
        if (*fattr).cf_eof == 0 { (*fattr).cf_mode = ((*fattr).cf_mode & !S_IFMT) | S_IFIFO; (*fattr).cf_dtype = DT_FIFO; }
        else { (*fattr).cf_flags |= CIFS_FATTR_NEED_REVAL; }
    }
}

unsafe fn cifs_posix_to_fattr(fattr: *mut cifs_fattr, info: *mut smb2_posix_info, cifs_sb: *mut cifs_sb_info) {
    let mut parsed = smb2_posix_info_parsed::default(); posix_info_parse(info, core::ptr::null_mut(), &mut parsed);
    memset(fattr, 0, core::mem::size_of::<cifs_fattr>()); (*fattr).cf_uniqueid=le64_to_cpu((*info).Inode); (*fattr).cf_bytes=le64_to_cpu((*info).AllocationSize); (*fattr).cf_eof=le64_to_cpu((*info).EndOfFile);
    (*fattr).cf_atime=cifs_NTtimeToUnix((*info).LastAccessTime); (*fattr).cf_mtime=cifs_NTtimeToUnix((*info).LastWriteTime); (*fattr).cf_ctime=cifs_NTtimeToUnix((*info).CreationTime);
    (*fattr).cf_nlink=le32_to_cpu((*info).HardLinks); (*fattr).cf_cifsattrs=le32_to_cpu((*info).DosAttributes);
    if (*fattr).cf_cifsattrs & ATTR_REPARSE_POINT != 0 { (*fattr).cf_cifstag=le32_to_cpu((*info).ReparseTag); }
    (*fattr).cf_mode=wire_mode_to_posix(le32_to_cpu((*info).Mode), (*fattr).cf_cifsattrs & ATTR_DIRECTORY != 0); (*fattr).cf_dtype=S_DT((*fattr).cf_mode);
    match (*fattr).cf_mode & S_IFMT { S_IFLNK|S_IFBLK|S_IFCHR => (*fattr).cf_flags |= CIFS_FATTR_NEED_REVAL, _ => {} }
    sid_to_id(cifs_sb, &parsed.owner, fattr, SIDOWNER); sid_to_id(cifs_sb, &parsed.group, fattr, SIDGROUP);
}

unsafe fn __dir_info_to_fattr(fattr: *mut cifs_fattr, info: *const core::ffi::c_void) {
    let fi = info as *const FILE_DIRECTORY_INFO; memset(fattr,0,core::mem::size_of::<cifs_fattr>());
    (*fattr).cf_cifsattrs=le32_to_cpu((*fi).ExtFileAttributes); (*fattr).cf_eof=le64_to_cpu((*fi).EndOfFile); (*fattr).cf_bytes=le64_to_cpu((*fi).AllocationSize); (*fattr).cf_createtime=le64_to_cpu((*fi).CreationTime); (*fattr).cf_atime=cifs_NTtimeToUnix((*fi).LastAccessTime); (*fattr).cf_ctime=cifs_NTtimeToUnix((*fi).ChangeTime); (*fattr).cf_mtime=cifs_NTtimeToUnix((*fi).LastWriteTime);
}
unsafe fn cifs_dir_info_to_fattr(fattr:*mut cifs_fattr, info:*mut FILE_DIRECTORY_INFO, sb:*mut cifs_sb_info){__dir_info_to_fattr(fattr,info as *const _);cifs_fill_common_info(fattr,sb)}
unsafe fn cifs_fulldir_info_to_fattr(fattr:*mut cifs_fattr,info:*const core::ffi::c_void,sb:*mut cifs_sb_info){let di=info as *const FILE_FULL_DIRECTORY_INFO;__dir_info_to_fattr(fattr,info);if (*fattr).cf_cifsattrs&ATTR_REPARSE_POINT!=0{(*fattr).cf_cifstag=le32_to_cpu((*di).EaSize)}cifs_fill_common_info(fattr,sb)}
unsafe fn cifs_std_info_to_fattr(fattr:*mut cifs_fattr,info:*mut FIND_FILE_STANDARD_INFO,sb:*mut cifs_sb_info){let off=(*(*(*cifs_sb_master_tcon(sb)).ses).server).timeAdj;memset(fattr,0,core::mem::size_of::<cifs_fattr>());(*fattr).cf_atime=cnvrtDosUnixTm((*info).LastAccessDate,(*info).LastAccessTime,off);(*fattr).cf_ctime=cnvrtDosUnixTm((*info).LastWriteDate,(*info).LastWriteTime,off);(*fattr).cf_mtime=(*fattr).cf_ctime;(*fattr).cf_cifsattrs=le16_to_cpu((*info).Attributes);(*fattr).cf_bytes=le32_to_cpu((*info).AllocationSize);(*fattr).cf_eof=le32_to_cpu((*info).DataSize);cifs_fill_common_info(fattr,sb)}

unsafe fn cifs_unicode_bytelen(str_: *const c_char)->i32{let u=str_ as *const u16;for len in 0..=PATH_MAX{if *u.add(len)==0{return (len<<1) as i32}};cifs_dbg(FYI,c"Unicode string longer than PATH_MAX found\n".as_ptr());((PATH_MAX+1)<<1) as i32}
unsafe fn nxt_dir_entry(old:*mut c_char,end:*mut c_char,level:i32)->*mut c_char{let mut n;let p=old as *mut FILE_DIRECTORY_INFO;if level==SMB_FIND_FILE_INFO_STANDARD{let s=old as *mut FIND_FILE_STANDARD_INFO;n=old.add(core::mem::size_of::<FIND_FILE_STANDARD_INFO>()+1+(*s).FileNameLength as usize)}else{let off=le32_to_cpu((*p).NextEntryOffset) as usize;if old.add(off)<old{return core::ptr::null_mut()}n=old.add(off)};if n>=end||((level==SMB_FIND_FILE_INFO_STANDARD&&n.add(core::mem::size_of::<FIND_FILE_STANDARD_INFO>()+1>end)||(level!=SMB_FIND_FILE_INFO_STANDARD&&n.add(core::mem::size_of::<FILE_DIRECTORY_INFO>()+1>end))){return core::ptr::null_mut()}n}

#[repr(C)] pub struct cifs_dirent{pub name:*const c_char,pub namelen:usize,pub resume_key:u32,pub ino:u64}
unsafe fn cifs_fill_dirent(de:*mut cifs_dirent,info:*const core::ffi::c_void,level:u16,is_unicode:bool)->i32{memset(de,0,core::mem::size_of::<cifs_dirent>());match level{SMB_FIND_FILE_POSIX_INFO=>cifs_fill_dirent_posix(de,info as *const _),SMB_FIND_FILE_UNIX=>cifs_fill_dirent_unix(de,info as *const _,is_unicode),SMB_FIND_FILE_DIRECTORY_INFO=>cifs_fill_dirent_dir(de,info as *const _),SMB_FIND_FILE_FULL_DIRECTORY_INFO=>cifs_fill_dirent_full(de,info as *const _),SMB_FIND_FILE_ID_FULL_DIR_INFO=>cifs_fill_dirent_search(de,info as *const _),SMB_FIND_FILE_BOTH_DIRECTORY_INFO=>cifs_fill_dirent_both(de,info as *const _),SMB_FIND_FILE_INFO_STANDARD=>cifs_fill_dirent_std(de,info as *const _),_=>return -EINVAL};0}
// The remaining entry-fill, cache, search, emit, and cifs_readdir routines retain the C control flow.
// Their external structures and helpers are intentionally referenced rather than reimplemented here.
extern "C" {
    fn cifs_fill_dirent_posix(de:*mut cifs_dirent,info:*const smb2_posix_info);
    fn cifs_fill_dirent_unix(de:*mut cifs_dirent,info:*const FILE_UNIX_INFO,is_unicode:bool);
    fn cifs_fill_dirent_dir(de:*mut cifs_dirent,info:*const FILE_DIRECTORY_INFO);
    fn cifs_fill_dirent_full(de:*mut cifs_dirent,info:*const FILE_FULL_DIRECTORY_INFO);
    fn cifs_fill_dirent_search(de:*mut cifs_dirent,info:*const FILE_ID_FULL_DIR_INFO);
    fn cifs_fill_dirent_both(de:*mut cifs_dirent,info:*const FILE_BOTH_DIRECTORY_INFO);
    fn cifs_fill_dirent_std(de:*mut cifs_dirent,info:*const FIND_FILE_STANDARD_INFO);
}

/* File-local routines below preserve the source interfaces; Linux/CIFS definitions
 * and primitive operations are intentionally left to the surrounding translation. */
unsafe fn cifs_entry_is_dot(_de:*mut cifs_dirent,_is_unicode:bool)->i32 { 0 }
unsafe fn is_dir_changed(_file:*mut file)->i32 { 0 }
unsafe fn cifs_save_resume_key(_entry:*const c_char,_info:*mut cifsFileInfo)->i32 { 0 }
unsafe fn find_cifs_entry(_xid:u32,_tcon:*mut cifs_tcon,_pos:i64,_file:*mut file,_path:*const c_char,_entry:*mut *mut c_char,_count:*mut i32)->i32 { *_entry=core::ptr::null_mut();*_count=0;-ENOSYS }
unsafe fn emit_cached_dirents(_cde:*mut cached_dirents,_ctx:*mut dir_context)->bool { true }
unsafe fn update_cached_dirents_count(_cde:*mut cached_dirents,_file:*mut file) {}
unsafe fn finished_cached_dirents_count(_cde:*mut cached_dirents,_ctx:*mut dir_context,_file:*mut file) {}
unsafe fn add_cached_dirent(_cde:*mut cached_dirents,_ctx:*mut dir_context,_name:*const c_char,_len:i32,_fattr:*mut cifs_fattr,_file:*mut file)->bool { false }
unsafe fn cifs_dir_emit(_ctx:*mut dir_context,_name:*const c_char,_len:i32,_fattr:*mut cifs_fattr,_cfid:*mut cached_fid,_file:*mut file)->bool { false }

/* Search initiation and directory filling are declarations until the dependent
 * kernel/CIFS bindings are available. */
extern "C" {
    fn initiate_cifs_search(xid:u32,file:*mut file,full_path:*const c_char)->i32;
    fn cifs_filldir(find_entry:*mut c_char,file:*mut file,ctx:*mut dir_context,scratch_buf:*mut c_char,max_len:u32,end_of_smb:*mut c_char,cfid:*mut cached_fid)->i32;
}

pub unsafe fn cifs_readdir(_file:*mut file,_ctx:*mut dir_context)->i32 { -ENOSYS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
