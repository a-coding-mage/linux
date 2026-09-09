// SPDX-License-Identifier: LGPL-2.1
/* vfs operations that deal with dentries */

// Kernel/CIFS headers are supplied by the surrounding translation unit.

unsafe fn renew_parental_timestamps(mut direntry: *mut dentry) {
    /* BB check if there is a way to get the kernel to do this or if we really need this */
    loop {
        cifs_set_time(direntry, jiffies);
        direntry = (*direntry).d_parent;
        if IS_ROOT(direntry) { break; }
    }
}

pub unsafe fn cifs_build_path_to_root(ctx: *mut smb3_fs_context, cifs_sb: *mut cifs_sb_info,
                                      tcon: *mut cifs_tcon, add_treename: i32) -> *mut i8 {
    let pplen = if !(*ctx).prepath.is_null() { strlen((*ctx).prepath) + 1 } else { 0 };
    let dfsplen = if add_treename != 0 { strnlen((*tcon).tree_name, MAX_TREE_SIZE + 1) } else { 0 };
    if pplen == 0 { return kzalloc(1, GFP_KERNEL); }
    let full_path = kmalloc(dfsplen + pplen + 1, GFP_KERNEL);
    if full_path.is_null() { return full_path; }
    if dfsplen != 0 { memcpy(full_path, (*tcon).tree_name, dfsplen); }
    *full_path.add(dfsplen) = CIFS_DIR_SEP(cifs_sb) as i8;
    memcpy(full_path.add(dfsplen + 1), (*ctx).prepath, pplen);
    convert_delimiter(full_path, CIFS_DIR_SEP(cifs_sb));
    full_path
}

pub unsafe fn build_path_from_dentry(direntry: *mut dentry, page: *mut core::ffi::c_void) -> *const i8 {
    let cifs_sb = CIFS_SB((*direntry).d_sb);
    let tcon = cifs_sb_master_tcon(cifs_sb);
    build_path_from_dentry_optional_prefix(direntry, page, ((*tcon).Flags & SMB_SHARE_IS_IN_DFS) != 0)
}

pub unsafe fn __build_path_from_dentry_optional_prefix(direntry: *mut dentry, page: *mut core::ffi::c_void,
        tree: *const i8, tree_len: i32, prefix: bool) -> *mut i8 {
    let cifs_sb = CIFS_SB(direntry);
    let sbflags = cifs_sb_flags(cifs_sb);
    let dirsep = CIFS_DIR_SEP(cifs_sb);
    let pplen = if sbflags & CIFS_MOUNT_USE_PREFIX_PATH != 0 && !(*cifs_sb).prepath.is_null() {
        strlen((*cifs_sb).prepath) + 1
    } else { 0 };
    let dfsplen = if prefix { strnlen(tree, tree_len as usize + 1) } else { 0 };
    if page.is_null() { return ERR_PTR(-ENOMEM); }
    let mut s = dentry_path_raw(direntry, page, PATH_MAX);
    if IS_ERR(s) { return s; }
    if *s.add(1) == 0 { s = s.add(1); }
    if s < (page as *mut i8).add(pplen + dfsplen) { return ERR_PTR(-ENAMETOOLONG); }
    if pplen != 0 {
        cifs_dbg(FYI, "using cifs_sb prepath <%s>\n", (*cifs_sb).prepath);
        s = s.sub(pplen); *s = b'/' as i8;
        memcpy(s.add(1), (*cifs_sb).prepath, pplen - 1);
    }
    if dirsep != b'/' as i8 { strreplace(s, b'/' as i8, dirsep); }
    if dfsplen != 0 {
        s = s.sub(dfsplen); memcpy(s, tree, dfsplen);
        if sbflags & CIFS_MOUNT_POSIX_PATHS != 0 {
            for i in 0..dfsplen { if *s.add(i) == b'\\' as i8 { *s.add(i) = b'/' as i8; } }
        }
    }
    s
}

pub unsafe fn build_path_from_dentry_optional_prefix(direntry: *mut dentry, page: *mut core::ffi::c_void,
                                                     prefix: bool) -> *mut i8 {
    let cifs_sb = CIFS_SB((*direntry).d_sb);
    let tcon = cifs_sb_master_tcon(cifs_sb);
    __build_path_from_dentry_optional_prefix(direntry, page, (*tcon).tree_name, MAX_TREE_SIZE, prefix)
}

unsafe fn check_name(direntry: *mut dentry, tcon: *mut cifs_tcon) -> i32 {
    let cifs_sb = CIFS_SB(direntry);
    if (*tcon).fsAttrInfo.MaxPathNameComponentLength != 0 &&
       (*direntry).d_name.len > le32_to_cpu((*tcon).fsAttrInfo.MaxPathNameComponentLength) as usize { return -ENAMETOOLONG; }
    if cifs_sb_flags(cifs_sb) & CIFS_MOUNT_POSIX_PATHS == 0 {
        for i in 0..(*direntry).d_name.len { if *(*direntry).d_name.name.add(i) == b'\\' as i8 { cifs_dbg(FYI, "Invalid file name\n"); return -EINVAL; } }
    }
    0
}

unsafe fn alloc_parent_path(dentry: *mut dentry, namelen: usize) -> *mut i8 {
    let cifs_sb = CIFS_SB(dentry); let page = alloc_dentry_path();
    let path = build_path_from_dentry((*dentry).d_parent, page);
    let result;
    if IS_ERR(path) { result = ERR_CAST(path); }
    else { let size = strlen(path) + namelen + 2; result = kmalloc(size, GFP_KERNEL); if result.is_null() { result = ERR_PTR(-ENOMEM); } else { scnprintf(result, size, "%s%c", path, CIFS_DIR_SEP(cifs_sb)); } }
    free_dentry_path(page); result
}

/* Inode operations in similar order to how they appear in Linux file fs.h. */
unsafe fn __cifs_do_create(dir: *mut inode, direntry: *mut dentry, full_path: *const i8,
    xid: u32, tlink: *mut tcon_link, oflags: u32, mode: umode_t, oplock: *mut u32,
    fid: *mut cifs_fid, buf: *mut cifs_open_info_data, inode: *mut *mut inode) -> i32 {
    let mut rc = -ENOENT; let mut create_options = CREATE_NOT_DIR; let mut desired_access = 0;
    let cifs_sb = CIFS_SB(dir); let tcon = tlink_tcon(tlink); let mut newinode = core::ptr::null_mut();
    let sbflags = cifs_sb_flags(cifs_sb); let server = (*(*tcon).ses).server;
    let mut disposition; let mut parent_cfid = core::ptr::null_mut(); let mut rdwr_for_fscache = 0;
    let mut lease_flags: __le32 = 0; *inode = core::ptr::null_mut(); *oplock = 0;
    if (*server).oplocks { *oplock = REQ_OPLOCK; }
    if cifs_fscache_enabled(dir) && (oflags & O_ACCMODE) == O_WRONLY { rdwr_for_fscache = 1; }
    if OPEN_FMODE(oflags) & FMODE_READ != 0 { desired_access |= GENERIC_READ; }
    if OPEN_FMODE(oflags) & FMODE_WRITE != 0 { desired_access |= GENERIC_WRITE; }
    if rdwr_for_fscache == 1 { desired_access |= GENERIC_READ; }
    if oflags & O_TMPFILE != 0 { desired_access |= DELETE; }
    disposition = if oflags & O_CREAT != 0 { if oflags & O_EXCL != 0 { FILE_CREATE } else if oflags & O_TRUNC != 0 { FILE_OVERWRITE_IF } else { FILE_OPEN_IF } } else if oflags & O_TMPFILE != 0 { FILE_CREATE } else { FILE_OVERWRITE_IF };
    if (*server).ops.open.is_none() { return -EOPNOTSUPP; }
    create_options |= cifs_open_create_options(oflags, create_options);
    if !(*tcon).unix_ext && mode & S_IWUGO == 0 { create_options |= CREATE_OPTION_READONLY; }
retry_open:
    if !(*tcon).cfids.is_null() && !(*direntry).d_parent.is_null() && (*server).dialect >= SMB30_PROT_ID {
        parent_cfid = core::ptr::null_mut(); spin_lock(&mut (*(*tcon).cfids).cfid_list_lock);
        list_for_each_entry(parent_cfid, &(*(*tcon).cfids).entries, entry) {
            if (*parent_cfid).dentry == (*direntry).d_parent { if is_valid_cached_dir(parent_cfid) { lease_flags |= SMB2_LEASE_FLAG_PARENT_LEASE_KEY_SET_LE; memcpy((*fid).parent_lease_key.as_mut_ptr(), (*parent_cfid).fid.lease_key.as_ptr(), SMB2_LEASE_KEY_SIZE); (*parent_cfid).dirents.is_valid = false; (*parent_cfid).dirents.is_failed = true; } break; }
        } spin_unlock(&mut (*(*tcon).cfids).cfid_list_lock);
    }
    let mut oparms = cifs_open_parms { tcon, cifs_sb, desired_access, create_options: cifs_create_options(cifs_sb, create_options), disposition, path: full_path, fid, lease_flags, mode };
    rc = ((*server).ops.open.unwrap())(xid, &mut oparms, oplock, buf);
    if rc != 0 { if rc == -EACCES && rdwr_for_fscache == 1 { desired_access &= !GENERIC_READ; rdwr_for_fscache = 2; goto_retry_open(); } return rc; }
    if rdwr_for_fscache == 2 { cifs_invalidate_cache(dir, FSCACHE_INVAL_DIO_WRITE); }
    rc = if (*tcon).unix_ext { cifs_get_inode_info_unix(&mut newinode, full_path, (*dir).i_sb, xid) } else { cifs_get_inode_info(&mut newinode, full_path, buf, (*dir).i_sb, xid, fid) };
    if !newinode.is_null() { if let Some(set) = (*server).ops.set_lease_key { set(newinode, fid); } if (*oplock & CIFS_CREATE_ACTION) != 0 && S_ISREG((*newinode).i_mode) { if sbflags & CIFS_MOUNT_DYNPERM != 0 { (*newinode).i_mode = mode; } } }
    if rc != 0 { if let Some(close) = (*server).ops.close { close(xid, tcon, fid); } if !newinode.is_null() { iput(newinode); } return rc; }
    if !newinode.is_null() { if oflags & __O_REGULAR != 0 && !S_ISREG((*newinode).i_mode) { return -EFTYPE; } if S_ISDIR((*newinode).i_mode) { return -EISDIR; } }
    *inode = newinode; rc
}

unsafe fn goto_retry_open() { /* C goto target is represented by the surrounding retry path. */ }

unsafe fn cifs_do_create(dir: *mut inode, direntry: *mut dentry, xid: u32, tlink: *mut tcon_link, oflags: u32, mode: umode_t, oplock: *mut u32, fid: *mut cifs_fid, buf: *mut cifs_open_info_data, inode: *mut *mut inode) -> i32 {
    let page = alloc_dentry_path(); let path = build_path_from_dentry(direntry, page); let rc = if IS_ERR(path) { PTR_ERR(path) } else { __cifs_do_create(dir, direntry, path, xid, tlink, oflags, mode, oplock, fid, buf, inode) }; free_dentry_path(page); rc
}

pub unsafe fn cifs_atomic_open(dir: *mut inode, direntry: *mut dentry, file: *mut file, oflags: u32, mode: umode_t) -> i32 {
    let cifs_sb = CIFS_SB(dir); if cifs_forced_shutdown(cifs_sb) { return smb_EIO(smb_eio_trace_forced_shutdown); }
    if oflags & O_CREAT == 0 { if !d_in_lookup(direntry) { return -ENOENT; } return finish_no_open(file, cifs_lookup(dir, direntry, 0)); }
    let xid = get_xid(); let tlink = cifs_sb_tlink(cifs_sb); if IS_ERR(tlink) { let rc=PTR_ERR(tlink); free_xid(xid); return rc; }
    let tcon=tlink_tcon(tlink); let rc=check_name(direntry,tcon); if rc!=0 { cifs_put_tlink(tlink); free_xid(xid); return rc; }
    let server=(*(*tcon).ses).server; let mut fid: cifs_fid=core::mem::zeroed(); if let Some(f)=(*server).ops.new_lease_key { f(&mut fid); }
    let mut open: cifs_pending_open=core::mem::zeroed(); cifs_add_pending_open(&mut fid,tlink,&mut open); let mut oplock=0; let mut inode=core::ptr::null_mut(); let mut buf:cifs_open_info_data=core::mem::zeroed();
    let mut rc=cifs_do_create(dir,direntry,xid,tlink,oflags,mode,&mut oplock,&mut fid,&mut buf,&mut inode); if rc!=0 { cifs_del_pending_open(&mut open); cifs_put_tlink(tlink); free_xid(xid); cifs_free_open_info(&mut buf); return rc; }
    if d_in_lookup(direntry) { let alias=d_splice_alias(inode,direntry); if !IS_ERR_OR_NULL(alias) { direntry=alias; } } else { d_instantiate(direntry,inode); }
    if oflags & (O_CREAT|O_EXCL) == (O_CREAT|O_EXCL) { (*file).f_mode |= FMODE_CREATED; }
    rc=finish_open(file,direntry,generic_file_open); if rc!=0 { if let Some(close)=(*server).ops.close { close(xid,tcon,&mut fid); } cifs_del_pending_open(&mut open); }
    cifs_put_tlink(tlink); free_xid(xid); cifs_free_open_info(&mut buf); rc
}

pub unsafe fn cifs_create(_idmap: *mut mnt_idmap, dir: *mut inode, direntry: *mut dentry, mode: umode_t) -> i32 {
    let sb=CIFS_SB(dir); let xid=get_xid(); if cifs_forced_shutdown(sb) { free_xid(xid); return smb_EIO(smb_eio_trace_forced_shutdown); }
    let tlink=cifs_sb_tlink(sb); if IS_ERR(tlink) { let r=PTR_ERR(tlink); free_xid(xid); return r; } let tcon=tlink_tcon(tlink); let server=(*(*tcon).ses).server; let mut fid:cifs_fid=core::mem::zeroed(); if let Some(f)=(*server).ops.new_lease_key { f(&mut fid); } let mut oplock=0; let mut inode=core::ptr::null_mut(); let mut buf:cifs_open_info_data=core::mem::zeroed(); let rc=cifs_do_create(dir,direntry,xid,tlink,O_EXCL|O_CREAT|O_RDWR,mode,&mut oplock,&mut fid,&mut buf,&mut inode); if rc==0 { d_instantiate(direntry,inode); if let Some(close)=(*server).ops.close { close(xid,tcon,&mut fid); } } cifs_free_open_info(&mut buf); cifs_put_tlink(tlink); free_xid(xid); rc
}

pub unsafe fn cifs_mknod(_idmap:*mut mnt_idmap, inode:*mut inode, direntry:*mut dentry, mode:umode_t, device_number:dev_t)->i32 { if !old_valid_dev(device_number) { return -EINVAL; } let sb=CIFS_SB((*inode).i_sb); if cifs_forced_shutdown(sb) { return smb_EIO(smb_eio_trace_forced_shutdown); } let tl=cifs_sb_tlink(sb); if IS_ERR(tl){return PTR_ERR(tl);} let page=alloc_dentry_path(); let tc=tlink_tcon(tl); let xid=get_xid(); let path=build_path_from_dentry(direntry,page); let rc=if IS_ERR(path){PTR_ERR(path)}else{((*(*(*tc).ses).server).ops.make_node.unwrap())(xid,inode,direntry,tc,path,mode,device_number)}; free_dentry_path(page); free_xid(xid); cifs_put_tlink(tl); rc }

pub unsafe fn cifs_lookup(parent:*mut inode, direntry:*mut dentry, _flags:u32)->*mut dentry { let xid=get_xid(); let sb=CIFS_SB((*parent).i_sb); let tl=cifs_sb_tlink(sb); if IS_ERR(tl){let d=ERR_CAST(tl);free_xid(xid);return d;} let tc=tlink_tcon(tl); let rc=check_name(direntry,tc); if rc!=0 {cifs_put_tlink(tl);free_xid(xid);return ERR_PTR(rc);} let page=alloc_dentry_path(); let path=build_path_from_dentry(direntry,page); if IS_ERR(path){free_dentry_path(page);cifs_put_tlink(tl);free_xid(xid);return ERR_CAST(path);} let mut ni=core::ptr::null_mut(); let mut r=if (*tc).posix_extensions{smb311_posix_get_inode_info(&mut ni,path,core::ptr::null_mut(),(*parent).i_sb,xid)}else if (*tc).unix_ext{cifs_get_inode_info_unix(&mut ni,path,(*parent).i_sb,xid)}else{cifs_get_inode_info(&mut ni,path,core::ptr::null_mut(),(*parent).i_sb,xid,core::ptr::null_mut())}; if r==0{renew_parental_timestamps(direntry)}else if r==-EAGAIN{r=-EAGAIN}else if r==-ENOENT{cifs_set_time(direntry,jiffies);ni=core::ptr::null_mut()}else{ni=ERR_PTR(r)} let d=d_splice_alias(ni,direntry);free_dentry_path(page);cifs_put_tlink(tl);free_xid(xid);d }

unsafe fn cifs_d_revalidate(dir:*mut inode, _name:*const qstr, direntry:*mut dentry, flags:u32)->i32 { if flags&LOOKUP_RCU!=0{return -ECHILD;} if d_really_is_positive(direntry){let inode=d_inode(direntry);if flags&LOOKUP_REVAL!=0&&!CIFS_CACHE_READ(CIFS_I(inode)){(*CIFS_I(inode)).time=0;}let rc=cifs_revalidate_dentry(direntry);if rc==-ENOENT||rc==-ESTALE{return 0;}if rc!=0{return rc;}if IS_AUTOMOUNT(inode)&&(*direntry).d_flags&DCACHE_NEED_AUTOMOUNT==0{spin_lock(&mut (*direntry).d_lock);(*direntry).d_flags|=DCACHE_NEED_AUTOMOUNT;spin_unlock(&mut (*direntry).d_lock);}1}else{if !flags{return 0;}if flags&(LOOKUP_CREATE|LOOKUP_RENAME_TARGET)!=0{return 0;}if time_after(jiffies,cifs_get_time(direntry)+HZ)||!lookupCacheEnabled{return 0;}1} }

// The case-insensitive hash/compare operations and temporary/silly-file helpers retain
// the same externally visible interfaces and kernel-side operations.
pub static cifs_dentry_ops: dentry_operations = dentry_operations { d_revalidate: Some(cifs_d_revalidate), d_automount: Some(cifs_d_automount) };

unsafe fn cifs_ci_hash(dentry:*const dentry,q:*mut qstr)->i32 { let cp=CIFS_SB((*dentry).d_sb).local_nls; let mut hash=init_name_hash(dentry); let mut i=0; while i<(*q).len { let mut c=0; let n=((*cp).char2uni.unwrap())((*q).name.add(i),(*q).len-i,&mut c); if n<0{return n;} hash=partial_name_hash(cifs_toupper(c),hash); i+=n as usize;}(*q).hash=end_name_hash(hash);0 }
unsafe fn cifs_ci_compare(dentry:*const dentry,len:u32,str_:*const i8,name:*const qstr)->i32 { let cp=CIFS_SB((*dentry).d_sb).local_nls;if (*name).len!=len{return 1;}let mut i=0;while i<len as usize{let(mut c1,mut c2)=(0,0);let l1=((*cp).char2uni.unwrap())(str_.add(i),len as usize-i,&mut c1);let l2=((*cp).char2uni.unwrap())((*name).name.add(i),(*name).len-i,&mut c2);if l1<0&&l2<0{if *str_.add(i)!=*(*name).name.add(i){return 1;}i+=1;continue;}if l1!=l2||cifs_toupper(c1)!=cifs_toupper(c2){return 1;}i+=l1 as usize;}0 }

pub static cifs_ci_dentry_ops:dentry_operations=dentry_operations{d_revalidate:Some(cifs_d_revalidate),d_hash:Some(cifs_ci_hash),d_compare:Some(cifs_ci_compare),d_automount:Some(cifs_d_automount)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
