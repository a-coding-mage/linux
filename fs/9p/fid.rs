// SPDX-License-Identifier: GPL-2.0-only
/*
 * V9FS FID Management
 *
 *  Copyright (C) 2007 by Latchesar Ionkov <lucho@ionkov.net>
 *  Copyright (C) 2005, 2006 by Eric Van Hensbergen <ericvh@gmail.com>
 */

// Dependencies are supplied by the surrounding kernel/V9FS translation unit.

#[inline]
unsafe fn __add_fid(dentry: *mut dentry, fid: *mut p9_fid) {
    let v9fs_dentry = to_v9fs_dentry(dentry);
    hlist_add_head(&mut (*fid).dlist, &mut (*v9fs_dentry).head);
}

/// Add a fid to a dentry; consumes the caller's fid pointer.
pub unsafe fn v9fs_fid_add(dentry: *mut dentry, pfid: *mut *mut p9_fid) {
    let fid = *pfid;
    spin_lock(&mut (*dentry).d_lock);
    __add_fid(dentry, fid);
    spin_unlock(&mut (*dentry).d_lock);
    *pfid = core::ptr::null_mut();
}

unsafe fn v9fs_is_writeable(mode: i32) -> bool {
    if mode & (P9_OWRITE | P9_ORDWR) != 0 { true } else { false }
}

/// Search for an open fid off of the inode list.
pub unsafe fn v9fs_fid_find_inode(
    inode: *mut inode,
    want_writeable: bool,
    uid: kuid_t,
    any: bool,
) -> *mut p9_fid {
    let mut ret: *mut p9_fid = core::ptr::null_mut();
    p9_debug(P9_DEBUG_VFS, " inode: %p\n", inode);
    spin_lock(&mut (*inode).i_lock);
    let h = &mut *(&mut (*inode).i_private as *mut _ as *mut hlist_head);
    hlist_for_each_entry!(fid, h, ilist, {
        if any || uid_eq((*fid).uid, uid) {
            if want_writeable && !v9fs_is_writeable((*fid).mode) {
                p9_debug(P9_DEBUG_VFS, " mode: %x not writeable?\n", (*fid).mode);
                continue;
            }
            p9_fid_get(fid);
            ret = fid;
            break;
        }
    });
    spin_unlock(&mut (*inode).i_lock);
    ret
}

/// Add an open fid to an inode; consumes the caller's fid pointer.
pub unsafe fn v9fs_open_fid_add(inode: *mut inode, pfid: *mut *mut p9_fid) {
    let fid = *pfid;
    spin_lock(&mut (*inode).i_lock);
    hlist_add_head(&mut (*fid).ilist, &mut *(&mut (*inode).i_private as *mut _ as *mut hlist_head));
    spin_unlock(&mut (*inode).i_lock);
    *pfid = core::ptr::null_mut();
}

unsafe fn v9fs_fid_find(dentry: *mut dentry, uid: kuid_t, any: i32) -> *mut p9_fid {
    let v9fs_dentry = to_v9fs_dentry(dentry);
    let mut ret: *mut p9_fid = core::ptr::null_mut();
    p9_debug(P9_DEBUG_VFS, " dentry: %pd (%p) uid %d any %d\n", dentry, dentry,
             from_kuid(&init_user_ns, uid), any);
    if !hlist_empty(&(*v9fs_dentry).head) {
        spin_lock(&mut (*dentry).d_lock);
        hlist_for_each_entry!(fid, &mut (*v9fs_dentry).head, dlist, {
            if any != 0 || uid_eq((*fid).uid, uid) {
                ret = fid;
                p9_fid_get(ret);
                break;
            }
        });
        spin_unlock(&mut (*dentry).d_lock);
    }
    if ret.is_null() && !(*dentry).d_inode.is_null() {
        ret = v9fs_fid_find_inode((*dentry).d_inode, false, uid, any != 0);
    }
    ret
}

unsafe fn build_path_from_dentry(
    _v9ses: *mut v9fs_session_info,
    dentry: *mut dentry,
    names: *mut *const *const u8,
) -> i32 {
    let mut n = 0;
    let mut ds = dentry;
    while !IS_ROOT(ds) { n += 1; ds = (*ds).d_parent; }
    let wnames = kmalloc_array(n as usize, core::mem::size_of::<*const u8>(), GFP_KERNEL);
    if wnames.is_null() { return -ENOMEM; }
    ds = dentry;
    let mut i = n - 1;
    while i >= 0 {
        *wnames.add(i as usize) = (*ds).d_name.name;
        ds = (*ds).d_parent;
        i -= 1;
    }
    *names = wnames;
    n
}

unsafe fn v9fs_fid_lookup_with_uid(dentry: *mut dentry, uid: kuid_t, any: i32) -> *mut p9_fid {
    let v9ses = v9fs_dentry2v9ses(dentry);
    let access = (*v9ses).flags & V9FS_ACCESS_MASK;
    let mut fid = v9fs_fid_find(dentry, uid, any);
    if !fid.is_null() { return fid; }
    down_read(&mut (*v9ses).rename_sem);
    let ds = (*dentry).d_parent;
    fid = v9fs_fid_find(ds, uid, any);
    if !fid.is_null() {
        let old_fid = fid;
        fid = p9_client_walk(old_fid, 1, &(*dentry).d_name.name, 1);
        p9_fid_put(old_fid);
        return fid_out(dentry, v9ses, fid);
    }
    up_read(&mut (*v9ses).rename_sem);
    let mut root_fid = v9fs_fid_find((*dentry).d_sb.s_root, uid, any);
    if root_fid.is_null() {
        if access == V9FS_ACCESS_SINGLE { return ERR_PTR(-EPERM); }
        let uname = if v9fs_proto_dotu(v9ses) || v9fs_proto_dotl(v9ses) { core::ptr::null() } else { (*v9ses).uname };
        fid = p9_client_attach((*v9ses).clnt, core::ptr::null_mut(), uname, uid, (*v9ses).aname);
        if IS_ERR(fid) { return fid; }
        root_fid = p9_fid_get(fid);
        v9fs_fid_add((*dentry).d_sb.s_root, &mut fid);
    }
    if (*dentry).d_sb.s_root == dentry { return root_fid; }
    down_read(&mut (*v9ses).rename_sem);
    let mut names: *const *const u8 = core::ptr::null();
    let n = build_path_from_dentry(v9ses, dentry, &mut names);
    if n < 0 { return err_out(v9ses, ERR_PTR(n)); }
    let mut old_fid = root_fid;
    fid = root_fid;
    let mut i = 0;
    while i < n {
        let l = core::cmp::min(n - i, P9_MAXWELEM);
        fid = p9_client_walk(old_fid, l, names.add(i as usize), if old_fid == root_fid { 1 } else { 0 });
        if fid != old_fid { p9_fid_put(old_fid); old_fid = fid; }
        if IS_ERR(fid) { kfree(names as *mut _); return err_out(v9ses, fid); }
        i += l;
    }
    kfree(names as *mut _);
    fid_out(dentry, v9ses, fid)
}

unsafe fn fid_out(dentry: *mut dentry, v9ses: *mut v9fs_session_info, mut fid: *mut p9_fid) -> *mut p9_fid {
    if !IS_ERR(fid) {
        spin_lock(&mut (*dentry).d_lock);
        if d_unhashed(dentry) { spin_unlock(&mut (*dentry).d_lock); p9_fid_put(fid); fid = ERR_PTR(-ENOENT); }
        else { __add_fid(dentry, fid); p9_fid_get(fid); spin_unlock(&mut (*dentry).d_lock); }
    }
    up_read(&mut (*v9ses).rename_sem);
    fid
}

pub unsafe fn v9fs_fid_lookup(dentry: *mut dentry) -> *mut p9_fid {
    let v9ses = v9fs_dentry2v9ses(dentry);
    let access = (*v9ses).flags & V9FS_ACCESS_MASK;
    let (uid, any) = match access {
        V9FS_ACCESS_SINGLE | V9FS_ACCESS_USER | V9FS_ACCESS_CLIENT => (current_fsuid(), 0),
        V9FS_ACCESS_ANY => ((*v9ses).uid, 1),
        _ => (INVALID_UID, 0),
    };
    v9fs_fid_lookup_with_uid(dentry, uid, any)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
