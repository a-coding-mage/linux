// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Contains mounting routines used for handling traversal via SMB junctions.
 *
 *   Copyright (c) 2007 Igor Mammedov
 *   Copyright (C) International Business Machines  Corp., 2008
 *   Author(s): Igor Mammedov (niallain@gmail.com)
 *              Steve French (sfrench@us.ibm.com)
 *   Copyright (c) 2023 Paulo Alcantara <palcantara@suse.de>
 */

// Linux headers and local CIFS headers are supplied by the surrounding translation unit.

static mut CIFS_AUTOMOUNT_LIST: list_head = LIST_HEAD_INIT;

static mut CIFS_AUTOMOUNT_TASK: delayed_work = DECLARE_DELAYED_WORK!(cifs_expire_automounts);
static mut cifs_mountpoint_expiry_timeout: c_int = 500 * HZ;

unsafe fn cifs_expire_automounts(work: *mut work_struct) {
    let list: *mut list_head = &raw mut CIFS_AUTOMOUNT_LIST;

    mark_mounts_for_expiry(list);
    if !list_empty(list) {
        schedule_delayed_work(
            &raw mut CIFS_AUTOMOUNT_TASK,
            cifs_mountpoint_expiry_timeout,
        );
    }
}

pub unsafe fn cifs_release_automount_timer() {
    if WARN_ON(!list_empty(&raw mut CIFS_AUTOMOUNT_LIST)) {
        return;
    }
    cancel_delayed_work_sync(&raw mut CIFS_AUTOMOUNT_TASK);
}

/**
 * cifs_build_devname - build a devicename from a UNC and optional prepath
 * @nodename: pointer to UNC string
 * @prepath: pointer to prefixpath (or NULL if there isn't one)
 *
 * Build a new cifs devicename after chasing a DFS referral. Allocate a buffer
 * big enough to hold the final thing. Copy the UNC from the nodename, and
 * concatenate the prepath onto the end of it if there is one.
 *
 * Returns pointer to the built string, or a ERR_PTR. Caller is responsible
 * for freeing the returned string.
 */
pub unsafe fn cifs_build_devname(mut nodename: *mut c_char, prepath: *const c_char) -> *mut c_char {
    let mut pplen: usize;
    let mut unclen: usize;
    let dev: *mut c_char;
    let mut pos: *mut c_char;

    nodename = nodename.add(strspn(nodename, b"\\\0".as_ptr() as *const c_char));
    if *nodename == 0 {
        return ERR_PTR(-EINVAL);
    }

    unclen = strlen(nodename);
    pos = nodename.add(unclen).sub(1);
    while *pos == b'\\' as c_char {
        pos = pos.sub(1);
        unclen -= 1;
    }

    pplen = if !prepath.is_null() { strlen(prepath) } else { 0 };
    dev = kmalloc(2 + unclen + 1 + pplen + 1, GFP_KERNEL);
    if dev.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    pos = dev;
    *pos = b'/' as c_char;
    pos = pos.add(1);
    *pos = b'/' as c_char;
    pos = pos.add(1);

    memcpy(pos as *mut c_void, nodename as *const c_void, unclen);
    pos = pos.add(unclen);

    if pplen != 0 {
        *pos = b'/' as c_char;
        pos = pos.add(1);
        memcpy(pos as *mut c_void, prepath as *const c_void, pplen);
        pos = pos.add(pplen);
    }

    *pos = 0;
    convert_delimiter(dev, b'/' as c_char);
    dev
}

unsafe fn is_dfs_mount(dentry: *mut dentry) -> bool {
    let cifs_sb: *mut cifs_sb_info = CIFS_SB((*(*dentry).d_sb));
    let tcon: *mut cifs_tcon = cifs_sb_master_tcon(cifs_sb);
    let ret: bool;

    spin_lock(&raw mut (*tcon).tc_lock);
    ret = !(*tcon).origin_fullpath.is_null();
    spin_unlock(&raw mut (*tcon).tc_lock);
    ret
}

/* Return full path out of a dentry set for automount */
unsafe fn automount_fullpath(dentry: *mut dentry, page: *mut c_void) -> *mut c_char {
    let cifs_sb: *mut cifs_sb_info = CIFS_SB((*(*dentry).d_sb));
    let tcon: *mut cifs_tcon = cifs_sb_master_tcon(cifs_sb);
    let mut len: usize;
    let mut s: *mut c_char;

    spin_lock(&raw mut (*tcon).tc_lock);
    if (*tcon).origin_fullpath.is_null() {
        spin_unlock(&raw mut (*tcon).tc_lock);
        return build_path_from_dentry_optional_prefix(dentry, page, true);
    }
    spin_unlock(&raw mut (*tcon).tc_lock);

    if page.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    s = dentry_path_raw(dentry, page, PATH_MAX);
    if IS_ERR(s) {
        return s;
    }
    if *s.add(1) == 0 {
        s = s.add(1);
    }

    spin_lock(&raw mut (*tcon).tc_lock);
    len = strlen((*tcon).origin_fullpath);
    if (s < (page as *mut c_char).add(len)) {
        spin_unlock(&raw mut (*tcon).tc_lock);
        return ERR_PTR(-ENAMETOOLONG);
    }
    s = s.sub(len);
    memcpy(s as *mut c_void, (*tcon).origin_fullpath as *const c_void, len);
    spin_unlock(&raw mut (*tcon).tc_lock);
    convert_delimiter(s, b'/' as c_char);
    s
}

unsafe fn fs_context_set_ids(ctx: *mut smb3_fs_context) {
    let uid = current_fsuid();
    let gid = current_fsgid();

    if (*ctx).multiuser {
        if !(*ctx).uid_specified { (*ctx).linux_uid = uid; }
        if !(*ctx).gid_specified { (*ctx).linux_gid = gid; }
    }
    if !(*ctx).cruid_specified { (*ctx).cred_uid = uid; }
}

/* Create a vfsmount that we can automount */
unsafe fn cifs_do_automount(path: *mut path) -> *mut vfsmount {
    let mntpt = (*path).dentry;
    if IS_ROOT(mntpt) { return ERR_PTR(-ESTALE); }
    let mntpt_sb = CIFS_SB((*mntpt).d_sb);
    let ses = (*cifs_sb_master_tcon(mntpt_sb)).ses;
    let cur_ctx = (*mntpt_sb).ctx;
    let mut rc: c_int;
    let mut page: *mut c_void = core::ptr::null_mut();

    mutex_lock(&raw mut (*ses).session_mutex);
    rc = smb3_sync_session_ctx_passwords(mntpt_sb, ses);
    mutex_unlock(&raw mut (*ses).session_mutex);
    if rc != 0 { return ERR_PTR(rc); }

    let fc = fs_context_for_submount((*(*path).mnt).mnt_sb, mntpt);
    if IS_ERR(fc) { return ERR_CAST(fc); }
    let ctx = smb3_fc2context(fc);
    page = alloc_dentry_path();
    let full_path = automount_fullpath(mntpt, page);
    if IS_ERR(full_path) { let mnt = ERR_CAST(full_path); put_fs_context(fc); free_dentry_path(page); return mnt; }
    let mut tmp = *cur_ctx;
    tmp.source = core::ptr::null_mut(); tmp.leaf_fullpath = core::ptr::null_mut();
    tmp.UNC = core::ptr::null_mut(); tmp.prepath = core::ptr::null_mut(); tmp.dfs_root_ses = core::ptr::null_mut();
    fs_context_set_ids(&mut tmp);
    rc = smb3_fs_context_dup(ctx, &mut tmp);
    if rc != 0 { let mnt = ERR_PTR(rc); put_fs_context(fc); free_dentry_path(page); return mnt; }
    rc = smb3_parse_devname(full_path, ctx);
    if rc != 0 { let mnt = ERR_PTR(rc); put_fs_context(fc); free_dentry_path(page); return mnt; }
    (*ctx).source = smb3_fs_context_fullpath(ctx, b'/' as c_char);
    if IS_ERR((*ctx).source) { let mnt = ERR_CAST((*ctx).source); (*ctx).source = core::ptr::null_mut(); put_fs_context(fc); free_dentry_path(page); return mnt; }
    (*ctx).dfs_automount = is_dfs_mount(mntpt); (*ctx).dfs_conn = (*ctx).dfs_automount;
    let mnt = fc_mount(fc);
    put_fs_context(fc); free_dentry_path(page); mnt
}

/* Attempt to automount the referral */
pub unsafe fn cifs_d_automount(path: *mut path) -> *mut vfsmount {
    let newmnt = cifs_do_automount(path);
    if IS_ERR(newmnt) { return newmnt; }
    mnt_set_expiry(newmnt, &raw mut CIFS_AUTOMOUNT_LIST);
    schedule_delayed_work(&raw mut CIFS_AUTOMOUNT_TASK, cifs_mountpoint_expiry_timeout);
    newmnt
}

pub static cifs_namespace_inode_operations: inode_operations = inode_operations {
    fileattr_get: Some(cifs_fileattr_get),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
