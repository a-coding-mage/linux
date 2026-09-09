// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Neil Brown 2002
 * Copyright (C) Christoph Hellwig 2007
 *
 * This file contains the code mapping from inodes to NFS file handles,
 * and for mapping back from file handles to dentries.
 *
 * For details on why we do all the strange and hairy things in here
 * take a look at Documentation/filesystems/nfs/exporting.rst.
 */

// Linux dependencies supplied by the surrounding kernel translation.

const FILEID_INO64_GEN_LEN: i32 = 3;

unsafe fn get_name(path: *const path, name: *mut i8, child: *mut dentry) -> i32;

unsafe fn exportfs_get_name(mnt: *mut vfsmount, dir: *mut dentry,
                            name: *mut i8, child: *mut dentry) -> i32 {
    let nop = (*(*dir).d_sb).s_export_op;
    let path = path { mnt, dentry: dir };

    if !(*nop).get_name.is_null() {
        ((*nop).get_name)(dir, name, child)
    } else {
        get_name(&path, name, child)
    }
}

/*
 * Check if the dentry or any of it's aliases is acceptable.
 */
unsafe fn find_acceptable_alias(
    mut result: *mut dentry,
    acceptable: Option<unsafe extern "C" fn(*mut c_void, *mut dentry) -> i32>,
    context: *mut c_void,
) -> *mut dentry {
    if acceptable.unwrap()(context, result) != 0 { return result; }
    let inode = (*result).d_inode;
    let mut toput: *mut dentry = core::ptr::null_mut();
    let mut dentry: *mut dentry;
    spin_lock(&mut (*inode).i_lock);
    for_each_alias!(dentry, inode) {
        if dget_alias_ilocked(dentry).is_null() { continue; }
        spin_unlock(&mut (*inode).i_lock);
        dput(toput);
        if dentry != result && acceptable.unwrap()(context, dentry) != 0 {
            dput(result);
            return dentry;
        }
        spin_lock(&mut (*inode).i_lock);
        toput = dentry;
    }
    spin_unlock(&mut (*inode).i_lock);
    dput(toput);
    core::ptr::null_mut()
}

unsafe fn dentry_connected(mut dentry: *mut dentry) -> bool {
    dget(dentry);
    while (*dentry).d_flags & DCACHE_DISCONNECTED != 0 {
        let parent = dget_parent(dentry);
        dput(dentry);
        if dentry == parent { dput(parent); return false; }
        dentry = parent;
    }
    dput(dentry);
    true
}

unsafe fn clear_disconnected(mut dentry: *mut dentry) {
    dget(dentry);
    while (*dentry).d_flags & DCACHE_DISCONNECTED != 0 {
        let parent = dget_parent(dentry);
        WARN_ON_ONCE!(IS_ROOT(dentry));
        spin_lock(&mut (*dentry).d_lock);
        (*dentry).d_flags &= !DCACHE_DISCONNECTED;
        spin_unlock(&mut (*dentry).d_lock);
        dput(dentry);
        dentry = parent;
    }
    dput(dentry);
}

unsafe fn reconnect_one(mnt: *mut vfsmount, dentry: *mut dentry, nbuf: *mut i8) -> *mut dentry {
    let mut parent = ERR_PTR(-EACCES);
    if !(*(*mnt).mnt_sb).s_export_op.get_parent.is_null() {
        parent = ((*(*mnt).mnt_sb).s_export_op.get_parent)(dentry);
    }
    if IS_ERR(parent) { return parent; }
    let err = exportfs_get_name(mnt, parent, nbuf, dentry);
    if err == -ENOENT { dput(parent); return if dentry_connected(dentry) { core::ptr::null_mut() } else { ERR_PTR(-ESTALE) }; }
    if err != 0 { dput(parent); return ERR_PTR(err); }
    let tmp = lookup_one_unlocked(mnt_idmap(mnt), &QSTR(nbuf), parent);
    if IS_ERR(tmp) { let e = PTR_ERR(tmp); dput(parent); return ERR_PTR(e); }
    if tmp != dentry { dput(tmp); dput(parent); return core::ptr::null_mut(); }
    dput(tmp);
    if IS_ROOT(dentry) { dput(parent); return ERR_PTR(-ESTALE); }
    parent
}

unsafe fn reconnect_path(mnt: *mut vfsmount, target_dir: *mut dentry, nbuf: *mut i8) -> i32 {
    let mut dentry = dget(target_dir);
    while (*dentry).d_flags & DCACHE_DISCONNECTED != 0 {
        BUG_ON!(dentry == (*(*mnt).mnt_sb).s_root);
        let parent = if IS_ROOT(dentry) { reconnect_one(mnt, dentry, nbuf) } else { dget_parent(dentry) };
        if parent.is_null() { break; }
        dput(dentry);
        if IS_ERR(parent) { return PTR_ERR(parent); }
        dentry = parent;
    }
    dput(dentry);
    clear_disconnected(target_dir);
    0
}

#[repr(C)]
struct getdents_callback {
    ctx: dir_context,
    name: *mut i8,
    ino: u64,
    found: i32,
    sequence: i32,
}

unsafe extern "C" fn filldir_one(ctx: *mut dir_context, name: *const i8, len: i32,
                                  _pos: loff_t, ino: u64, _d_type: u32) -> bool {
    let buf = container_of!(ctx, getdents_callback, ctx);
    (*buf).sequence += 1;
    if (*buf).ino == ino && len <= NAME_MAX && !name_is_dot_dotdot(name, len) {
        memcpy((*buf).name as *mut c_void, name as *const c_void, len as usize);
        *(*buf).name.add(len as usize) = 0;
        (*buf).found = 1;
        return false;
    }
    true
}

unsafe fn get_name(path: *const path, name: *mut i8, child: *mut dentry) -> i32 {
    let cred = current_cred();
    let dir = (*path).dentry.as_ref().unwrap().d_inode;
    let child_path = path { mnt: (*path).mnt, dentry: child };
    let mut stat: kstat = core::mem::zeroed();
    let mut buffer = getdents_callback { ctx: dir_context { actor: Some(filldir_one), count: INT_MAX }, name, ino: 0, found: 0, sequence: 0 };
    if dir.is_null() || !S_ISDIR((*dir).i_mode) { return -ENOTDIR; }
    if (*dir).i_fop.is_null() { return -EINVAL; }
    let error = vfs_getattr_nosec(&child_path, &mut stat, STATX_INO, AT_STATX_SYNC_AS_STAT);
    if error != 0 { return error; }
    buffer.ino = stat.ino;
    let file = dentry_open(path, O_RDONLY, cred);
    if IS_ERR(file) { return PTR_ERR(file); }
    if (*file).f_op.iterate_shared.is_null() { fput(file); return -EINVAL; }
    loop {
        let old_seq = buffer.sequence;
        let mut error = iterate_dir(file, &mut buffer.ctx);
        if buffer.found != 0 { error = 0; fput(file); return error; }
        if error < 0 { fput(file); return error; }
        error = -ENOENT;
        if old_seq == buffer.sequence { fput(file); return error; }
    }
}

unsafe fn exportfs_encode_ino64_fid(inode: *mut inode, fid: *mut fid, max_len: *mut i32) -> i32 {
    if *max_len < FILEID_INO64_GEN_LEN { *max_len = FILEID_INO64_GEN_LEN; return FILEID_INVALID; }
    (*fid).i64.ino = (*inode).i_ino;
    (*fid).i64.gen = (*inode).i_generation;
    *max_len = FILEID_INO64_GEN_LEN;
    FILEID_INO64_GEN
}

#[no_mangle]
pub unsafe extern "C" fn exportfs_encode_inode_fh(inode: *mut inode, fid: *mut fid, max_len: *mut i32, parent: *mut inode, flags: i32) -> i32 {
    let nop = (*(*inode).i_sb).s_export_op;
    if !exportfs_can_encode_fh(nop, flags) { return -EOPNOTSUPP; }
    let type_ = if nop.is_null() && flags & EXPORT_FH_FID != 0 { exportfs_encode_ino64_fid(inode, fid, max_len) } else { ((*nop).encode_fh)(inode, (*fid).raw.as_mut_ptr(), max_len, parent) };
    if type_ > 0 && FILEID_USER_FLAGS(type_) != 0 { return -EINVAL; }
    type_
}

#[no_mangle]
pub unsafe extern "C" fn exportfs_encode_fh(dentry: *mut dentry, fid: *mut fid, max_len: *mut i32, flags: i32) -> i32 {
    let mut p = core::ptr::null_mut(); let mut parent = core::ptr::null_mut();
    let inode = (*dentry).d_inode;
    if flags & EXPORT_FH_CONNECTABLE != 0 && !S_ISDIR((*inode).i_mode) { p = dget_parent(dentry); parent = (*p).d_inode; }
    let error = exportfs_encode_inode_fh(inode, fid, max_len, parent, flags); dput(p); error
}

#[no_mangle]
pub unsafe extern "C" fn exportfs_decode_fh_raw(mnt: *mut vfsmount, fid: *mut fid, fh_len: i32, fileid_type: i32, flags: u32, acceptable: Option<unsafe extern "C" fn(*mut c_void, *mut dentry) -> i32>, context: *mut c_void) -> *mut dentry {
    let nop = (*(*mnt).mnt_sb).s_export_op;
    if fileid_type < 0 || FILEID_USER_FLAGS(fileid_type) != 0 || !exportfs_can_decode_fh(nop) { return ERR_PTR(-ESTALE); }
    let result = ((*nop).fh_to_dentry)((*mnt).mnt_sb, fid, fh_len, fileid_type);
    if IS_ERR_OR_NULL(result) { return result; }
    if flags & EXPORT_FH_DIR_ONLY != 0 && !d_is_dir(result) { dput(result); return ERR_PTR(-ENOTDIR); }
    if acceptable.is_none() { return result; }
    let mut nbuf = [0i8; (NAME_MAX + 1) as usize];
    if d_is_dir(result) {
        if (*result).d_flags & DCACHE_DISCONNECTED != 0 && reconnect_path(mnt, result, nbuf.as_mut_ptr()) != 0 { dput(result); return ERR_PTR(-ESTALE); }
        if acceptable.unwrap()(context, result) == 0 { dput(result); return ERR_PTR(-EACCES); }
        return result;
    }
    let alias = find_acceptable_alias(result, acceptable, context); if !alias.is_null() { return alias; }
    if (*nop).fh_to_parent.is_null() { dput(result); return ERR_PTR(-ESTALE); }
    let target_dir = ((*nop).fh_to_parent)((*mnt).mnt_sb, fid, fh_len, fileid_type);
    if target_dir.is_null() || IS_ERR(target_dir) { dput(result); return if target_dir.is_null() { ERR_PTR(-ESTALE) } else { target_dir }; }
    if reconnect_path(mnt, target_dir, nbuf.as_mut_ptr()) != 0 { dput(target_dir); dput(result); return ERR_PTR(-ESTALE); }
    let err = exportfs_get_name(mnt, target_dir, nbuf.as_mut_ptr(), result); if err != 0 { dput(target_dir); dput(result); return ERR_PTR(err); }
    let nresult = lookup_one_unlocked(mnt_idmap(mnt), &QSTR(nbuf.as_mut_ptr()), target_dir); dput(target_dir);
    if IS_ERR(nresult) { dput(result); return nresult; }
    if (*nresult).d_inode != (*result).d_inode { dput(nresult); dput(result); return ERR_PTR(-ESTALE); }
    dput(result);
    let alias = find_acceptable_alias(nresult, acceptable, context); if alias.is_null() { dput(nresult); ERR_PTR(-EACCES) } else { alias }
}

#[no_mangle]
pub unsafe extern "C" fn exportfs_decode_fh(mnt: *mut vfsmount, fid: *mut fid, fh_len: i32, fileid_type: i32, acceptable: Option<unsafe extern "C" fn(*mut c_void, *mut dentry) -> i32>, context: *mut c_void) -> *mut dentry {
    let ret = exportfs_decode_fh_raw(mnt, fid, fh_len, fileid_type, 0, acceptable, context);
    if IS_ERR_OR_NULL(ret) && ret != ERR_PTR(-ENOMEM) { return ERR_PTR(-ESTALE); }
    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
