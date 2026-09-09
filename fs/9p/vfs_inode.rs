// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contains vfs inode ops for the 9P2000 protocol.
 *
 *  Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

// C dependencies supplied by the surrounding kernel translation unit.

static mut V9FS_DIR_INODE_OPERATIONS: inode_operations = inode_operations::default();
static mut V9FS_DIR_INODE_OPERATIONS_DOTU: inode_operations = inode_operations::default();
static mut V9FS_FILE_INODE_OPERATIONS: inode_operations = inode_operations::default();
static mut V9FS_SYMLINK_INODE_OPERATIONS: inode_operations = inode_operations::default();

unsafe fn unixmode2p9mode(v9ses: *mut v9fs_session_info, mode: umode_t) -> u32 {
    let mut res = mode & 0o777;
    if S_ISDIR(mode) { res |= P9_DMDIR; }
    if v9fs_proto_dotu(v9ses) {
        if (*v9ses).nodev == 0 {
            if S_ISSOCK(mode) { res |= P9_DMSOCKET; }
            if S_ISFIFO(mode) { res |= P9_DMNAMEDPIPE; }
            if S_ISBLK(mode) { res |= P9_DMDEVICE; }
            if S_ISCHR(mode) { res |= P9_DMDEVICE; }
        }
        if (mode & S_ISUID) == S_ISUID { res |= P9_DMSETUID; }
        if (mode & S_ISGID) == S_ISGID { res |= P9_DMSETGID; }
        if (mode & S_ISVTX) == S_ISVTX { res |= P9_DMSETVTX; }
    }
    res
}

unsafe fn p9mode2perm(v9ses: *mut v9fs_session_info, stat: *mut p9_wstat) -> i32 {
    let mode = (*stat).mode;
    let mut res = (mode & 0o777) as i32;
    if v9fs_proto_dotu(v9ses) {
        if (mode & P9_DMSETUID) == P9_DMSETUID { res |= S_ISUID as i32; }
        if (mode & P9_DMSETGID) == P9_DMSETGID { res |= S_ISGID as i32; }
        if (mode & P9_DMSETVTX) == P9_DMSETVTX { res |= S_ISVTX as i32; }
    }
    res
}

unsafe fn p9mode2unixmode(v9ses: *mut v9fs_session_info, stat: *mut p9_wstat, rdev: *mut dev_t) -> umode_t {
    let mode = (*stat).mode;
    *rdev = 0;
    let mut res = p9mode2perm(v9ses, stat) as umode_t;
    if (mode & P9_DMDIR) == P9_DMDIR { res |= S_IFDIR; }
    else if (mode & P9_DMSYMLINK) != 0 && v9fs_proto_dotu(v9ses) { res |= S_IFLNK; }
    else if (mode & P9_DMSOCKET) != 0 && v9fs_proto_dotu(v9ses) && (*v9ses).nodev == 0 { res |= S_IFSOCK; }
    else if (mode & P9_DMNAMEDPIPE) != 0 && v9fs_proto_dotu(v9ses) && (*v9ses).nodev == 0 { res |= S_IFIFO; }
    else if (mode & P9_DMDEVICE) != 0 && v9fs_proto_dotu(v9ses) && (*v9ses).nodev == 0 {
        let mut ty = 0i8; let mut major = -1i32; let mut minor = -1i32;
        let r = sscanf((*stat).extension, b"%c %i %i\0".as_ptr(), &mut ty, &mut major, &mut minor);
        if r != 3 { p9_debug(P9_DEBUG_ERROR, b"invalid device string, umode will be bogus: %s\n\0".as_ptr(), (*stat).extension); return res; }
        match ty as u8 { b'c' => res |= S_IFCHR, b'b' => res |= S_IFBLK, _ => p9_debug(P9_DEBUG_ERROR, b"Unknown special type %c %s\n\0".as_ptr(), ty, (*stat).extension) }
        *rdev = MKDEV(major as u32, minor as u32);
    } else { res |= S_IFREG; }
    res
}

pub unsafe fn v9fs_uflags2omode(uflags: i32, extended: i32) -> i32 {
    let mut ret = match uflags & 3 { O_WRONLY => P9_OWRITE, O_RDWR => P9_ORDWR, _ => P9_OREAD };
    if uflags & O_TRUNC != 0 { ret |= P9_OTRUNC; }
    if extended != 0 { if uflags & O_EXCL != 0 { ret |= P9_OEXCL; } if uflags & O_APPEND != 0 { ret |= P9_OAPPEND; } }
    ret
}

pub unsafe fn v9fs_blank_wstat(wstat: *mut p9_wstat) {
    (*wstat).type_ = !0; (*wstat).dev = !0; (*wstat).qid.type_ = !0; (*wstat).qid.version = !0;
    *((&mut (*wstat).qid.path) as *mut _ as *mut i64) = !0;
    (*wstat).mode = !0; (*wstat).atime = !0; (*wstat).mtime = !0; (*wstat).length = !0;
    (*wstat).name = core::ptr::null_mut(); (*wstat).uid = core::ptr::null_mut(); (*wstat).gid = core::ptr::null_mut(); (*wstat).muid = core::ptr::null_mut();
    (*wstat).n_uid = INVALID_UID; (*wstat).n_gid = INVALID_GID; (*wstat).n_muid = INVALID_UID; (*wstat).extension = core::ptr::null_mut();
}

pub unsafe fn v9fs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let v9inode = alloc_inode_sb(sb, v9fs_inode_cache, GFP_KERNEL);
    if v9inode.is_null() { return core::ptr::null_mut(); }
    (*v9inode).cache_validity = 0; mutex_init(&mut (*v9inode).v_mutex); &mut (*v9inode).netfs.inode
}

pub unsafe fn v9fs_free_inode(inode: *mut inode) { kmem_cache_free(v9fs_inode_cache, V9FS_I(inode)); }

pub unsafe fn v9fs_set_netfs_context(inode: *mut inode) { let v9inode = V9FS_I(inode); netfs_inode_init(&mut (*v9inode).netfs, &v9fs_req_ops, true); }

pub unsafe fn v9fs_init_inode(v9ses: *mut v9fs_session_info, inode: *mut inode, mode: umode_t, rdev: dev_t) -> i32 {
    inode_init_owner(&nop_mnt_idmap, inode, core::ptr::null_mut(), mode); (*inode).i_blocks = 0; (*inode).i_rdev = rdev; simple_inode_init_ts(inode); (*inode).i_mapping.a_ops = &v9fs_addr_operations; (*inode).i_private = core::ptr::null_mut();
    match mode & S_IFMT {
        S_IFIFO | S_IFBLK | S_IFCHR | S_IFSOCK => { if v9fs_proto_dotl(v9ses) { (*inode).i_op = &v9fs_file_inode_operations_dotl; } else if v9fs_proto_dotu(v9ses) { (*inode).i_op = &V9FS_FILE_INODE_OPERATIONS; } else { p9_debug(P9_DEBUG_ERROR, b"special files without extended mode\n\0".as_ptr()); return -EINVAL; } init_special_inode(inode, (*inode).i_mode, (*inode).i_rdev); }
        S_IFREG => { if v9fs_proto_dotl(v9ses) { (*inode).i_op = &v9fs_file_inode_operations_dotl; (*inode).i_fop = &v9fs_file_operations_dotl; } else { (*inode).i_op = &V9FS_FILE_INODE_OPERATIONS; (*inode).i_fop = &v9fs_file_operations; } }
        S_IFLNK => { if !v9fs_proto_dotu(v9ses) && !v9fs_proto_dotl(v9ses) { p9_debug(P9_DEBUG_ERROR, b"extended modes used with legacy protocol\n\0".as_ptr()); return -EINVAL; } if v9fs_proto_dotl(v9ses) { (*inode).i_op = &v9fs_symlink_inode_operations_dotl; inode_nohighmem(inode); } else { (*inode).i_op = &V9FS_SYMLINK_INODE_OPERATIONS; } }
        S_IFDIR => { inc_nlink(inode); if v9fs_proto_dotl(v9ses) { (*inode).i_op = &v9fs_dir_inode_operations_dotl; } else if v9fs_proto_dotu(v9ses) { (*inode).i_op = &V9FS_DIR_INODE_OPERATIONS_DOTU; } else { (*inode).i_op = &V9FS_DIR_INODE_OPERATIONS; } (*inode).i_fop = if v9fs_proto_dotl(v9ses) { &v9fs_dir_operations_dotl } else { &v9fs_dir_operations }; }
        _ => { p9_debug(P9_DEBUG_ERROR, b"BAD mode 0x%hx S_IFMT 0x%x\n\0".as_ptr(), mode, mode & S_IFMT); return -EINVAL; }
    } 0
}

pub unsafe fn v9fs_evict_inode(inode: *mut inode) {
    let v9inode = V9FS_I(inode); if !is_bad_inode(inode) { netfs_wait_for_outstanding_io(inode); truncate_inode_pages_final(&mut (*inode).i_data); let version = cpu_to_le32((*v9inode).qid.version); netfs_clear_inode_writeback(inode, &version); clear_inode(inode); filemap_fdatawrite(&mut (*inode).i_data); } else { clear_inode(inode); }
}

unsafe fn v9fs_test_inode(inode: *mut inode, data: *mut core::ffi::c_void) -> i32 { let v9inode = V9FS_I(inode); let st = data as *mut p9_wstat; let ses = v9fs_inode2v9ses(inode); let mut rdev = 0; let umode = p9mode2unixmode(ses, st, &mut rdev); if inode_wrong_type(inode, umode) != 0 { return 0; } if memcmp(&(*v9inode).qid.version as *const _ as *const _, &(*st).qid.version as *const _ as *const _, core::mem::size_of_val(&(*v9inode).qid.version)) != 0 || (*v9inode).qid.type_ != (*st).qid.type_ || (*v9inode).qid.path != (*st).qid.path { return 0; } 1 }
unsafe fn v9fs_test_new_inode(_: *mut inode, _: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn v9fs_set_inode(inode: *mut inode, data: *mut core::ffi::c_void) -> i32 { (*V9FS_I(inode)).qid = (*(data as *mut p9_wstat)).qid; 0 }

// Remaining declarations retain the C implementation's externally supplied kernel helpers and operation tables.
extern "C" {
    static v9fs_file_inode_operations_dotl: inode_operations; static v9fs_symlink_inode_operations_dotl: inode_operations; static v9fs_dir_inode_operations_dotl: inode_operations;
    static v9fs_file_operations_dotl: file_operations; static v9fs_file_operations: file_operations; static v9fs_dir_operations_dotl: file_operations; static v9fs_dir_operations: file_operations;
}

unsafe fn v9fs_qid_iget(sb: *mut super_block, qid: *mut p9_qid, st: *mut p9_wstat, new_: i32) -> *mut inode {
    let ses = (*sb).s_fs_info as *mut v9fs_session_info; let mut rdev = 0; let test = if new_ != 0 { v9fs_test_new_inode } else { v9fs_test_inode };
    let inode = iget5_locked(sb, QID2INO(qid), test, v9fs_set_inode, st); if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if inode_state_read_once(inode) & I_NEW == 0 { return inode; }
    (*inode).i_ino = QID2INO(qid); let mode = p9mode2unixmode(ses, st, &mut rdev); let ret = v9fs_init_inode(ses, inode, mode, rdev); if ret != 0 { iget_failed(inode); return ERR_PTR(ret); }
    v9fs_stat2inode(st, inode, sb, 0); v9fs_set_netfs_context(inode); v9fs_cache_inode_get_cookie(inode); unlock_new_inode(inode); inode
}

pub unsafe fn v9fs_inode_from_fid(v9ses: *mut v9fs_session_info, fid: *mut p9_fid, sb: *mut super_block, new_: i32) -> *mut inode { let st = p9_client_stat(fid); if IS_ERR(st) { return ERR_CAST(st); } let inode = v9fs_qid_iget(sb, &mut (*st).qid, st, new_); p9stat_free(st); kfree(st); inode }
unsafe fn v9fs_at_to_dotl_flags(flags: i32) -> i32 { if flags & AT_REMOVEDIR != 0 { P9_DOTL_AT_REMOVEDIR } else { 0 } }
unsafe fn v9fs_dec_count(inode: *mut inode) { let ses = v9fs_inode2v9ses(inode); if (*ses).cache & (CACHE_META | CACHE_LOOSE) == 0 { return; } if !S_ISDIR((*inode).i_mode) || (*inode).i_nlink > 2 { drop_nlink(inode); } }

unsafe fn v9fs_remove(dir: *mut inode, dentry: *mut dentry, flags: i32) -> i32 {
    let inode = d_inode(dentry); let ses = v9fs_inode2v9ses(dir); let dfid = v9fs_parent_fid(dentry); if IS_ERR(dfid) { return PTR_ERR(dfid); }
    let mut ret = if v9fs_proto_dotl(ses) { p9_client_unlinkat(dfid, (*dentry).d_name.name, v9fs_at_to_dotl_flags(flags)) } else { -EOPNOTSUPP }; p9_fid_put(dfid);
    if ret == -EOPNOTSUPP { let fid = v9fs_fid_clone(dentry); if IS_ERR(fid) { return PTR_ERR(fid); } ret = p9_client_remove(fid); }
    if ret == 0 { if flags & AT_REMOVEDIR != 0 { clear_nlink(inode); v9fs_dec_count(dir); } else { v9fs_dec_count(inode); } v9fs_invalidate_inode_attr(inode); v9fs_invalidate_inode_attr(dir); v9fs_dentry_fid_remove(dentry); } ret
}

pub unsafe fn v9fs_vfs_unlink(i: *mut inode, d: *mut dentry) -> i32 { v9fs_remove(i, d, 0) }
pub unsafe fn v9fs_vfs_rmdir(i: *mut inode, d: *mut dentry) -> i32 { v9fs_remove(i, d, AT_REMOVEDIR) }

pub unsafe fn v9fs_stat2inode(stat: *mut p9_wstat, inode: *mut inode, sb: *mut super_block, flags: u32) {
    let ses = (*sb).s_fs_info as *mut v9fs_session_info; let vi = V9FS_I(inode); inode_set_atime(inode, (*stat).atime, 0); inode_set_mtime(inode, (*stat).mtime, 0); inode_set_ctime(inode, (*stat).mtime, 0); (*inode).i_uid = (*ses).dfltuid; (*inode).i_gid = (*ses).dfltgid;
    if v9fs_proto_dotu(ses) { (*inode).i_uid = (*stat).n_uid; (*inode).i_gid = (*stat).n_gid; }
    if (S_ISREG((*inode).i_mode) || S_ISDIR((*inode).i_mode)) && v9fs_proto_dotu(ses) { let mut n = 0; if sscanf((*stat).extension, b" HARDLINKCOUNT %u\0".as_ptr(), &mut n) == 1 { set_nlink(inode, n); } }
    let mode = p9mode2perm(ses, stat) as umode_t; (*inode).i_mode = mode | ((*inode).i_mode & !S_IALLUGO); spin_lock(&mut (*inode).i_lock); netfs_write_remote_i_size(inode, (*stat).length); if flags & V9FS_STAT2INODE_KEEP_ISIZE == 0 { i_size_write(inode, (*stat).length); } (*inode).i_blocks = ((*stat).length + 511) >> 9; spin_unlock(&mut (*inode).i_lock); (*vi).cache_validity &= !V9FS_INO_INVALID_ATTR;
}

unsafe fn v9fs_vfs_get_link(dentry: *mut dentry, _: *mut inode, done: *mut delayed_call) -> *const i8 {
    if dentry.is_null() { return ERR_PTR(-ECHILD); } let ses = v9fs_dentry2v9ses(dentry); if !v9fs_proto_dotu(ses) { return ERR_PTR(-EBADF); } let fid = v9fs_fid_lookup(dentry); if IS_ERR(fid) { return ERR_CAST(fid); } let st = p9_client_stat(fid); p9_fid_put(fid); if IS_ERR(st) { return ERR_CAST(st); } if (*st).mode & P9_DMSYMLINK == 0 { p9stat_free(st); kfree(st); return ERR_PTR(-EINVAL); } let res = (*st).extension; (*st).extension = core::ptr::null_mut(); if strlen(res) >= PATH_MAX { *res.add(PATH_MAX - 1) = 0; } p9stat_free(st); kfree(st); set_delayed_call(done, kfree_link, res); res
}

pub unsafe fn v9fs_refresh_inode(fid: *mut p9_fid, inode: *mut inode) -> i32 { let ses = v9fs_inode2v9ses(inode); let st = p9_client_stat(fid); if IS_ERR(st) { return PTR_ERR(st); } let mut rdev = 0; let mode = p9mode2unixmode(ses, st, &mut rdev); if inode_wrong_type(inode, mode) == 0 { let flags = if (*ses).cache & CACHE_LOOSE != 0 { V9FS_STAT2INODE_KEEP_ISIZE } else { 0 }; v9fs_stat2inode(st, inode, (*inode).i_sb, flags); } p9stat_free(st); kfree(st); 0 }

// The following hooks mirror the remaining C VFS entry points; their kernel
// object and client-operation definitions are provided by the surrounding
// translation unit.
extern "C" {
    fn v9fs_vfs_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> i32;
    fn v9fs_vfs_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry;
    fn v9fs_vfs_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry;
    fn v9fs_vfs_atomic_open(dir: *mut inode, dentry: *mut dentry, file: *mut file, flags: u32, mode: umode_t) -> i32;
    fn v9fs_vfs_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const i8) -> i32;
    fn v9fs_vfs_link(old: *mut dentry, dir: *mut inode, dentry: *mut dentry) -> i32;
    fn v9fs_vfs_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> i32;
    fn v9fs_vfs_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: u32) -> i32;
}

static V9FS_DIR_DOTU_OPS: inode_operations = inode_operations { create: Some(v9fs_vfs_create), lookup: Some(v9fs_vfs_lookup), atomic_open: Some(v9fs_vfs_atomic_open), symlink: Some(v9fs_vfs_symlink), link: Some(v9fs_vfs_link), unlink: Some(v9fs_vfs_unlink), mkdir: Some(v9fs_vfs_mkdir), rmdir: Some(v9fs_vfs_rmdir), mknod: Some(v9fs_vfs_mknod), rename: Some(v9fs_vfs_rename), ..inode_operations::default() };
static V9FS_DIR_OPS: inode_operations = inode_operations { create: Some(v9fs_vfs_create), lookup: Some(v9fs_vfs_lookup), atomic_open: Some(v9fs_vfs_atomic_open), unlink: Some(v9fs_vfs_unlink), mkdir: Some(v9fs_vfs_mkdir), rmdir: Some(v9fs_vfs_rmdir), mknod: Some(v9fs_vfs_mknod), rename: Some(v9fs_vfs_rename), ..inode_operations::default() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
