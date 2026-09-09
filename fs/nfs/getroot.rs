// SPDX-License-Identifier: GPL-2.0-or-later
/* getroot.c: get the root dentry for an NFS mount
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and "internal.h" are supplied by the surrounding tree.
// Their declarations are intentionally not reproduced here.

// #define NFSDBG_FACILITY NFSDBG_CLIENT

/*
 * get a root dentry from the root filehandle
 */
pub unsafe fn nfs_get_root(s: *mut super::super::super::super::bindings::super_block,
                           fc: *mut super::super::super::super::bindings::fs_context) -> i32 {
    let ctx = nfs_fc2context(fc);
    let server = NFS_SB(s);
    let mut clone_server;
    let mut fsinfo;
    let mut root;
    let mut inode;
    let mut name: *mut core::ffi::c_char;
    let mut error: i32 = -ENOMEM;
    let mut kflags: core::ffi::c_ulong = 0;
    let mut kflags_out: core::ffi::c_ulong = 0;

    name = kstrdup((*fc).source, GFP_KERNEL);
    if name.is_null() {
        goto_out(error)
    }

    /* get the actual root for this mount */
    fsinfo.fattr = nfs_alloc_fattr_with_label(server);
    if fsinfo.fattr.is_null() {
        goto_out_name(error)
    }

    error = (*(*server).nfs_client).rpc_ops.getroot(server, (*ctx).mntfh, &mut fsinfo);
    if error < 0 {
        dprintk!("nfs_get_root: getattr error = %d\n", -error);
        nfs_errorf(fc, "NFS: Couldn't getattr on root");
        goto_out_fattr(error)
    }

    inode = nfs_fhget(s, (*ctx).mntfh, fsinfo.fattr);
    if IS_ERR(inode) {
        dprintk!("nfs_get_root: get root inode failed\n");
        error = PTR_ERR(inode);
        nfs_errorf(fc, "NFS: Couldn't get root inode");
        goto_out_fattr(error)
    }

    /* root dentries normally start off anonymous and get spliced in later
     * if the dentry tree reaches them; however if the dentry already
     * exists, we'll pick it up at this point and use it as the root
     */
    root = d_obtain_root(inode);
    if IS_ERR(root) {
        dprintk!("nfs_get_root: get root dentry failed\n");
        error = PTR_ERR(root);
        nfs_errorf(fc, "NFS: Couldn't get root dentry");
        goto_out_fattr(error)
    }

    security_d_instantiate(root, inode);
    spin_lock(&mut (*root).d_lock);
    if IS_ROOT(root) && (*root).d_fsdata.is_null()
        && ((*root).d_flags & DCACHE_NFSFS_RENAMED) == 0 {
        (*root).d_fsdata = name as *mut _;
        name = core::ptr::null_mut();
    }
    spin_unlock(&mut (*root).d_lock);
    if (*s).s_root.is_null() {
        (*s).s_root = dget(root);
    }
    (*fc).root = root;
    if (*server).caps & NFS_CAP_SECURITY_LABEL != 0 {
        kflags |= SECURITY_LSM_NATIVE_LABELS;
    }
    if !(*ctx).clone_data.sb.is_null() {
        if (*d_inode((*fc).root)).i_fop != &nfs_dir_operations {
            error = -ESTALE;
            goto_error_splat_root(error)
        }
        /* clone lsm security options from the parent to the new sb */
        error = security_sb_clone_mnt_opts((*ctx).clone_data.sb,
                                           s, kflags, &mut kflags_out);
        if error != 0 {
            goto_error_splat_root(error)
        }
        clone_server = NFS_SB((*ctx).clone_data.sb);
        (*server).has_sec_mnt_opts = (*clone_server).has_sec_mnt_opts;
    } else {
        error = security_sb_set_mnt_opts(s, (*fc).security,
                                         kflags, &mut kflags_out);
    }
    if error != 0 {
        goto_error_splat_root(error)
    }
    if (*server).caps & NFS_CAP_SECURITY_LABEL != 0
        && (kflags_out & SECURITY_LSM_NATIVE_LABELS) == 0 {
        (*server).caps &= !NFS_CAP_SECURITY_LABEL;
    }

    nfs_setsecurity(inode, fsinfo.fattr);
    error = 0;

    nfs_free_fattr(fsinfo.fattr);
    kfree(name);
    return error;

    // C goto labels retained as local control-flow markers for the direct translation.
    goto_error_splat_root(error);
    dput((*fc).root);
    (*fc).root = core::ptr::null_mut();
    nfs_free_fattr(fsinfo.fattr);
    kfree(name);
    return error;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
