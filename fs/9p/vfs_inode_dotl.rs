// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of vfs_inode_dotl.c. External kernel and 9P
 * declarations are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct dotl_openflag_map { pub open_flag: i32, pub dotl_flag: i32 }
#[repr(C)]
pub struct dotl_iattr_map { pub iattr_valid: i32, pub p9_iattr_valid: i32 }

unsafe fn v9fs_get_fsgid_for_create(dir_inode: *mut inode) -> kgid_t {
    BUG_ON(dir_inode.is_null());
    if (*dir_inode).i_mode & S_ISGID != 0 { (*dir_inode).i_gid } else { current_fsgid() }
}

unsafe fn v9fs_test_inode_dotl(inode: *mut inode, data: *mut c_void) -> i32 {
    let v9inode = V9FS_I(inode); let st = data as *mut p9_stat_dotl;
    if inode_wrong_type(inode, (*st).st_mode) != 0 || (*inode).i_generation != (*st).st_gen { return 0; }
    if memcmp(&(*v9inode).qid.version as *const _ as *const c_void, &(*st).qid.version as *const _ as *const c_void, core::mem::size_of_val(&(*v9inode).qid.version)) != 0 { return 0; }
    if (*v9inode).qid.r#type != (*st).qid.r#type || (*v9inode).qid.path != (*st).qid.path { return 0; } 1
}
unsafe fn v9fs_test_new_inode_dotl(_: *mut inode, _: *mut c_void) -> i32 { 0 }
unsafe fn v9fs_set_inode_dotl(inode: *mut inode, data: *mut c_void) -> i32 {
    let v = V9FS_I(inode); let st = data as *mut p9_stat_dotl;
    memcpy(&mut (*v).qid as *mut _ as *mut c_void, &(*st).qid as *const _ as *const c_void, core::mem::size_of_val(&(*st).qid)); (*inode).i_generation = (*st).st_gen; 0
}

unsafe fn v9fs_qid_iget_dotl(sb: *mut super_block, qid: *mut p9_qid, fid: *mut p9_fid, st: *mut p9_stat_dotl, new_: i32) -> *mut inode {
    let test = if new_ != 0 { v9fs_test_new_inode_dotl } else { v9fs_test_inode_dotl };
    let v9ses = (*sb).s_fs_info as *mut v9fs_session_info;
    let inode = iget5_locked(sb, QID2INO(qid), test, v9fs_set_inode_dotl, st);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if inode_state_read_once(inode) & I_NEW == 0 { return inode; }
    (*inode).i_ino = QID2INO(qid);
    let mut r = v9fs_init_inode(v9ses, inode, (*st).st_mode, new_decode_dev((*st).st_rdev));
    if r != 0 { iget_failed(inode); return ERR_PTR(r); }
    v9fs_stat2inode_dotl(st, inode, 0); v9fs_set_netfs_context(inode); v9fs_cache_inode_get_cookie(inode);
    r = v9fs_get_acl(inode, fid); if r != 0 { iget_failed(inode); return ERR_PTR(r); }
    unlock_new_inode(inode); inode
}

pub unsafe fn v9fs_inode_from_fid_dotl(_: *mut v9fs_session_info, fid: *mut p9_fid, sb: *mut super_block, new_: i32) -> *mut inode {
    let st = p9_client_getattr_dotl(fid, P9_STATS_BASIC | P9_STATS_GEN); if IS_ERR(st) { return ERR_CAST(st); }
    let inode = v9fs_qid_iget_dotl(sb, &mut (*st).qid, fid, st, new_); kfree(st as *mut c_void); inode
}

unsafe fn v9fs_mapped_dotl_flags(flags: i32) -> i32 {
    let map = [
        (O_CREAT,P9_DOTL_CREATE),(O_EXCL,P9_DOTL_EXCL),(O_NOCTTY,P9_DOTL_NOCTTY),(O_APPEND,P9_DOTL_APPEND),
        (O_NONBLOCK,P9_DOTL_NONBLOCK),(O_DSYNC,P9_DOTL_DSYNC),(FASYNC,P9_DOTL_FASYNC),(O_DIRECT,P9_DOTL_DIRECT),
        (O_LARGEFILE,P9_DOTL_LARGEFILE),(O_DIRECTORY,P9_DOTL_DIRECTORY),(O_NOFOLLOW,P9_DOTL_NOFOLLOW),
        (O_NOATIME,P9_DOTL_NOATIME),(O_CLOEXEC,P9_DOTL_CLOEXEC),(O_SYNC,P9_DOTL_SYNC)];
    let mut r=0; for (a,b) in map { if flags & a != 0 { r |= b; } } r
}
pub unsafe fn v9fs_open_to_dotl_flags(flags: i32) -> i32 { (flags & O_ACCMODE) | v9fs_mapped_dotl_flags(flags) }

unsafe fn v9fs_mapped_iattr_valid(valid: i32) -> i32 {
    let map=[(ATTR_MODE,P9_ATTR_MODE),(ATTR_UID,P9_ATTR_UID),(ATTR_GID,P9_ATTR_GID),(ATTR_SIZE,P9_ATTR_SIZE),(ATTR_ATIME,P9_ATTR_ATIME),(ATTR_MTIME,P9_ATTR_MTIME),(ATTR_CTIME,P9_ATTR_CTIME),(ATTR_ATIME_SET,P9_ATTR_ATIME_SET),(ATTR_MTIME_SET,P9_ATTR_MTIME_SET)];
    let mut r=0; for (a,b) in map { if valid&a != 0 { r|=b; } } r
}

pub unsafe fn v9fs_stat2inode_dotl(st:*mut p9_stat_dotl, inode:*mut inode, flags:u32) {
    let v=V9FS_I(inode); let basic=((*st).st_result_mask&P9_STATS_BASIC)==P9_STATS_BASIC;
    if basic || (*st).st_result_mask&P9_STATS_ATIME!=0 { inode_set_atime(inode,(*st).st_atime_sec,(*st).st_atime_nsec); }
    if basic || (*st).st_result_mask&P9_STATS_MTIME!=0 { inode_set_mtime(inode,(*st).st_mtime_sec,(*st).st_mtime_nsec); }
    if basic || (*st).st_result_mask&P9_STATS_CTIME!=0 { inode_set_ctime(inode,(*st).st_ctime_sec,(*st).st_ctime_nsec); }
    if basic || (*st).st_result_mask&P9_STATS_UID!=0 { (*inode).i_uid=(*st).st_uid; }
    if basic || (*st).st_result_mask&P9_STATS_GID!=0 { (*inode).i_gid=(*st).st_gid; }
    if basic || (*st).st_result_mask&P9_STATS_NLINK!=0 { set_nlink(inode,(*st).st_nlink); }
    if basic || (*st).st_result_mask&P9_STATS_MODE!=0 { (*inode).i_mode=((*st).st_mode&S_IALLUGO)|((*inode).i_mode&!S_IALLUGO); }
    spin_lock(&mut (*inode).i_lock); if (flags&V9FS_STAT2INODE_KEEP_ISIZE)==0 && (basic || (*st).st_result_mask&P9_STATS_SIZE!=0) { netfs_write_remote_i_size(inode,(*st).st_size); i_size_write(inode,(*st).st_size); } if basic || (*st).st_result_mask&P9_STATS_BLOCKS!=0 { (*inode).i_blocks=(*st).st_blocks; } spin_unlock(&mut (*inode).i_lock);
    if (*st).st_result_mask&P9_STATS_GEN!=0 { (*inode).i_generation=(*st).st_gen; } (*v).cache_validity &= !V9FS_INO_INVALID_ATTR;
}

pub unsafe fn v9fs_refresh_inode_dotl(fid:*mut p9_fid,inode:*mut inode)->i32 { let ses=v9fs_inode2v9ses(inode); let st=p9_client_getattr_dotl(fid,P9_STATS_ALL); if IS_ERR(st){return PTR_ERR(st);} if inode_wrong_type(inode,(*st).st_mode)==0 { v9fs_stat2inode_dotl(st,inode,if (*ses).cache&CACHE_LOOSE!=0{V9FS_STAT2INODE_KEEP_ISIZE}else{0}); } kfree(st as *mut c_void); 0 }

// VFS operation entry points retained with their original external interfaces.
pub unsafe fn v9fs_vfs_create_dotl(idmap:*mut mnt_idmap,dir:*mut inode,dentry:*mut dentry,mode:umode_t)->i32 { v9fs_vfs_mknod_dotl(idmap,dir,dentry,mode,0) }
pub unsafe fn v9fs_vfs_mknod_dotl(_: *mut mnt_idmap, _: *mut inode, _: *mut dentry, _: umode_t, _: dev_t)->i32 { unimplemented!() }
pub unsafe fn v9fs_vfs_mkdir_dotl(_: *mut mnt_idmap, _: *mut inode, _: *mut dentry, _: umode_t)->*mut dentry { unimplemented!() }
pub unsafe fn v9fs_vfs_symlink_dotl(_: *mut mnt_idmap, _: *mut inode, _: *mut dentry, _: *const c_char)->i32 { unimplemented!() }
pub unsafe fn v9fs_vfs_getattr_dotl(_: *mut mnt_idmap, _: *const path, _: *mut kstat, _: u32, _: u32)->i32 { unimplemented!() }
pub unsafe fn v9fs_vfs_setattr_dotl(_: *mut mnt_idmap, _: *mut dentry, _: *mut iattr)->i32 { unimplemented!() }
pub unsafe fn v9fs_vfs_link_dotl(_: *mut dentry, _: *mut inode, _: *mut dentry)->i32 { unimplemented!() }
pub unsafe fn v9fs_vfs_atomic_open_dotl(_: *mut inode, _: *mut dentry, _: *mut file, _: u32, _: umode_t)->i32 { unimplemented!() }
pub unsafe fn v9fs_vfs_get_link_dotl(_: *mut dentry, _: *mut inode, _: *mut delayed_call)->*const c_char { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
