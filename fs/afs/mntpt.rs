// SPDX-License-Identifier: GPL-2.0-or-later
/* mountpoint management
 *
 * Copyright (C) 2002 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies: linux kernel/project headers and internal declarations are
// supplied by the surrounding translation unit/build.

unsafe extern "C" {
    fn noop_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn afs_readlink(dentry: *mut dentry, buffer: *mut c_char, buflen: c_int) -> c_int;
    fn afs_getattr(
        path: *const path,
        stat: *mut kstat,
        request_mask: u32,
        query_flags: u32,
    ) -> c_int;
}

#[repr(C)]
pub struct FileOperations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
}

#[repr(C)]
pub struct InodeOperations {
    pub lookup: Option<unsafe extern "C" fn(*mut inode, *mut dentry, c_uint) -> *mut dentry>,
    pub readlink: Option<unsafe extern "C" fn(*mut dentry, *mut c_char, c_int) -> c_int>,
    pub getattr: Option<unsafe extern "C" fn(*const path, *mut kstat, u32, u32) -> c_int>,
}

#[no_mangle]
pub static afs_mntpt_file_operations: FileOperations = FileOperations {
    open: Some(afs_mntpt_open),
    llseek: Some(noop_llseek),
};

#[no_mangle]
pub static afs_mntpt_inode_operations: InodeOperations = InodeOperations {
    lookup: Some(afs_mntpt_lookup),
    readlink: Some(afs_readlink),
    getattr: Some(afs_getattr),
};

#[no_mangle]
pub static afs_autocell_inode_operations: InodeOperations = InodeOperations {
    lookup: None,
    readlink: None,
    getattr: Some(afs_getattr),
};

static mut AFS_VFSMOUNTS: ListHead = ListHead::new();
static mut AFS_MNTPT_EXPIRY_TIMER: DelayedWork =
    DelayedWork::new(afs_mntpt_expiry_timed_out);

static mut AFS_MNTPT_EXPIRY_TIMEOUT: c_ulong = 10 * 60;

static AFS_ROOT_VOLUME: [c_char; 10] = *b"root.cell\0";

/*
 * no valid lookup procedure on this sort of dir
 */
unsafe fn afs_mntpt_lookup(
    dir: *mut inode,
    dentry: *mut dentry,
    flags: c_uint,
) -> *mut dentry {
    _enter!("%p,%p{%pd2}", dir, dentry, dentry);
    ERR_PTR!(-EREMOTE)
}

/*
 * no valid open procedure on this sort of dir
 */
unsafe fn afs_mntpt_open(inode: *mut inode, file: *mut file) -> c_int {
    _enter!("%p,%p{%pD2}", inode, file, file);
    -EREMOTE
}

/*
 * Set the parameters for the proposed superblock.
 */
unsafe fn afs_mntpt_set_params(fc: *mut fs_context, mntpt: *mut dentry) -> c_int {
    let ctx: *mut afs_fs_context = (*fc).fs_private as *mut afs_fs_context;
    let src_as: *mut afs_super_info = AFS_FS_S!((*mntpt).d_sb);
    let vnode: *mut afs_vnode = AFS_FS_I!(d_inode!((*mntpt).d_sb));
    let mut cell: *mut afs_cell;
    let mut p: *const c_char;
    let mut ret: c_int;

    if (*fc).net_ns != (*src_as).net_ns {
        put_net((*fc).net_ns);
        (*fc).net_ns = get_net((*src_as).net_ns);
    }

    if !(*src_as).volume.is_null() && (*(*src_as).volume).type_ == AFSVL_RWVOL {
        (*ctx).type_ = AFSVL_RWVOL;
        (*ctx).force = true;
    }
    if !(*ctx).cell.is_null() {
        afs_unuse_cell((*ctx).cell, afs_cell_trace_unuse_mntpt);
        (*ctx).cell = core::ptr::null_mut();
    }
    if test_bit!(AFS_VNODE_PSEUDODIR, &(*vnode).flags) {
        /* if the directory is a pseudo directory, use the d_name */
        let mut size: c_uint = (*mntpt).d_name.len;

        if size < 2 {
            return -ENOENT;
        }

        p = (*mntpt).d_name.name;
        if *p == b'.' as c_char {
            size -= 1;
            p = p.add(1);
            (*ctx).type_ = AFSVL_RWVOL;
            (*ctx).force = true;
        }
        if size > AFS_MAXCELLNAME {
            return -ENAMETOOLONG;
        }

        cell = afs_lookup_cell(
            (*ctx).net,
            p,
            size,
            core::ptr::null_mut(),
            AFS_LOOKUP_CELL_MOUNTPOINT,
            afs_cell_trace_use_lookup_mntpt,
        );
        if IS_ERR!(cell) {
            pr_err!("kAFS: unable to lookup cell '%pd'\n", mntpt);
            return PTR_ERR!(cell);
        }
        (*ctx).cell = cell;

        (*ctx).volname = AFS_ROOT_VOLUME.as_ptr();
        (*ctx).volnamesz = AFS_ROOT_VOLUME.len() - 1;
    } else {
        /* read the contents of the AFS special symlink */
        let mut cleanup = DelayedCall::new();
        let content: *const c_char;
        let size: loff_t = i_size_read!(d_inode!(mntpt));

        if !(*src_as).cell.is_null() {
            (*ctx).cell = afs_use_cell((*src_as).cell, afs_cell_trace_use_mntpt);
        }

        if size < 2 || size > PAGE_SIZE - 1 {
            return -EINVAL;
        }

        content = afs_get_link(mntpt, d_inode!(mntpt), &mut cleanup);
        if IS_ERR!(content) {
            do_delayed_call!(&mut cleanup);
            return PTR_ERR!(content);
        }

        ret = -EINVAL;
        if *content.add(size as usize - 1) == b'.' as c_char {
            ret = vfs_parse_fs_qstr!(fc, "source", QSTR_LEN!(content, size - 1));
        }
        do_delayed_call!(&mut cleanup);
        if ret < 0 {
            return ret;
        }

        /* Don't cross a backup volume mountpoint from a backup volume */
        if !(*src_as).volume.is_null()
            && (*(*src_as).volume).type_ == AFSVL_BACKVOL
            && (*ctx).type_ == AFSVL_BACKVOL
        {
            return -ENODEV;
        }
    }

    0
}

/*
 * create a vfsmount to be automounted
 */
unsafe fn afs_mntpt_do_automount(mntpt: *mut dentry) -> *mut vfsmount {
    let fc: *mut fs_context;
    let mnt: *mut vfsmount;
    let mut ret: c_int;

    BUG_ON!(!d_inode!(mntpt).is_null());

    fc = fs_context_for_submount(&afs_fs_type, mntpt);
    if IS_ERR!(fc) {
        return ERR_CAST!(fc);
    }

    ret = afs_mntpt_set_params(fc, mntpt);
    if ret == 0 {
        mnt = fc_mount(fc);
    } else {
        mnt = ERR_PTR!(ret);
    }

    put_fs_context(fc);
    mnt
}

/*
 * handle an automount point
 */
unsafe fn afs_d_automount(path: *mut path) -> *mut vfsmount {
    let newmnt: *mut vfsmount;

    _enter!("{%pd}", (*path).dentry);

    newmnt = afs_mntpt_do_automount((*path).dentry);
    if IS_ERR!(newmnt) {
        return newmnt;
    }

    mnt_set_expiry(newmnt, &mut AFS_VFSMOUNTS);
    queue_delayed_work!(
        afs_wq,
        &mut AFS_MNTPT_EXPIRY_TIMER,
        AFS_MNTPT_EXPIRY_TIMEOUT * HZ,
    );
    _leave!(" = %p", newmnt);
    newmnt
}

/*
 * handle mountpoint expiry timer going off
 */
unsafe fn afs_mntpt_expiry_timed_out(_work: *mut work_struct) {
    _enter!("");

    if !list_empty!(&AFS_VFSMOUNTS) {
        mark_mounts_for_expiry(&mut AFS_VFSMOUNTS);
        queue_delayed_work!(
            afs_wq,
            &mut AFS_MNTPT_EXPIRY_TIMER,
            AFS_MNTPT_EXPIRY_TIMEOUT * HZ,
        );
    }

    _leave!("");
}

/*
 * kill the AFS mountpoint timer if it's still running
 */
unsafe fn afs_mntpt_kill_timer() {
    _enter!("");

    ASSERT!(list_empty!(&AFS_VFSMOUNTS));
    cancel_delayed_work_sync(&mut AFS_MNTPT_EXPIRY_TIMER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
