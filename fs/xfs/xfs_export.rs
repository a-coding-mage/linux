// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2004-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS and kernel translation units.

unsafe fn xfs_fileid_length(fileid_type: i32) -> i32 {
    match fileid_type {
        FILEID_INO32_GEN => 2,
        FILEID_INO32_GEN_PARENT => 4,
        x if x == (FILEID_INO32_GEN | XFS_FILEID_TYPE_64FLAG) => 3,
        x if x == (FILEID_INO32_GEN_PARENT | XFS_FILEID_TYPE_64FLAG) => 6,
        _ => FILEID_INVALID,
    }
}

unsafe fn xfs_fs_encode_fh(
    inode: *mut inode,
    fh: *mut u32,
    max_len: *mut i32,
    parent: *mut inode,
) -> i32 {
    let mp: *mut xfs_mount = XFS_M((*inode).i_sb);
    let fid: *mut fid = fh as *mut fid;
    let fid64: *mut xfs_fid64 = fh as *mut xfs_fid64;
    let mut fileid_type: i32;
    let len: i32;

    /* Directories don't need their parent encoded, they have ".." */
    if parent.is_null() {
        fileid_type = FILEID_INO32_GEN;
    } else {
        fileid_type = FILEID_INO32_GEN_PARENT;
    }

    /*
     * If the filesystem may contain 64bit inode numbers, we need
     * to use larger file handles that can represent them.
     *
     * While we only allocate inodes that do not fit into 32 bits any
     * large enough filesystem may contain them, thus the slightly
     * confusing looking conditional below.
     */
    if !xfs_has_small_inums(mp) || xfs_is_inode32(mp) {
        fileid_type |= XFS_FILEID_TYPE_64FLAG;
    }

    /*
     * Only encode if there is enough space given.  In practice
     * this means we can't export a filesystem with 64bit inodes
     * over NFSv2 with the subtree_check export option; the other
     * seven combinations work.  The real answer is "don't use v2".
     */
    len = xfs_fileid_length(fileid_type);
    if *max_len < len {
        *max_len = len;
        return FILEID_INVALID;
    }
    *max_len = len;

    match fileid_type {
        FILEID_INO32_GEN_PARENT => {
            (*fid).i32.parent_ino = (*parent).i_ino;
            (*fid).i32.parent_gen = (*parent).i_generation;
            (*fid).i32.ino = (*inode).i_ino;
            (*fid).i32.gen = (*inode).i_generation;
        }
        FILEID_INO32_GEN => {
            (*fid).i32.ino = (*inode).i_ino;
            (*fid).i32.gen = (*inode).i_generation;
        }
        x if x == (FILEID_INO32_GEN_PARENT | XFS_FILEID_TYPE_64FLAG) => {
            (*fid64).parent_ino = (*parent).i_ino;
            (*fid64).parent_gen = (*parent).i_generation;
            (*fid64).ino = (*inode).i_ino;
            (*fid64).gen = (*inode).i_generation;
        }
        x if x == (FILEID_INO32_GEN | XFS_FILEID_TYPE_64FLAG) => {
            (*fid64).ino = (*inode).i_ino;
            (*fid64).gen = (*inode).i_generation;
        }
        _ => {}
    }

    fileid_type
}

unsafe fn xfs_nfs_get_inode(
    sb: *mut super_block,
    ino: u64,
    generation: u32,
) -> *mut inode {
    let mp: *mut xfs_mount = XFS_M(sb);
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let mut error: i32;

    /*
     * NFS can sometimes send requests for ino 0.  Fail them gracefully.
     */
    if ino == 0 {
        return ERR_PTR(-ESTALE);
    }

    /*
     * The XFS_IGET_UNTRUSTED means that an invalid inode number is just
     * fine and not an indication of a corrupted filesystem as clients can
     * send invalid file handles and we have to handle it gracefully..
     */
    error = xfs_iget(mp, core::ptr::null_mut(), ino, XFS_IGET_UNTRUSTED, 0, &mut ip);
    if error != 0 {
        /*
         * EINVAL means the inode cluster doesn't exist anymore.
         * EFSCORRUPTED means the metadata pointing to the inode cluster
         * or the inode cluster itself is corrupt.  This implies the
         * filehandle is stale, so we should translate it here.
         * We don't use ESTALE directly down the chain to not
         * confuse applications using bulkstat that expect EINVAL.
         */
        match error {
            -EINVAL | -ENOENT | -EFSCORRUPTED => error = -ESTALE,
            _ => {}
        }
        return ERR_PTR(error);
    }

    /*
     * Reload the incore unlinked list to avoid failure in inodegc.
     * Use an unlocked check here because unrecovered unlinked inodes
     * should be somewhat rare.
     */
    if xfs_inode_unlinked_incomplete(ip) {
        error = xfs_inode_reload_unlinked(ip);
        if error != 0 {
            xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE);
            xfs_irele(ip);
            return ERR_PTR(error);
        }
    }

    if (*VFS_I(ip)).i_generation != generation || IS_PRIVATE(VFS_I(ip)) {
        xfs_irele(ip);
        return ERR_PTR(-ESTALE);
    }

    VFS_I(ip)
}

unsafe fn xfs_fs_fh_to_dentry(
    sb: *mut super_block,
    fid: *mut fid,
    fh_len: i32,
    fileid_type: i32,
) -> *mut dentry {
    let fid64 = fid as *mut xfs_fid64;
    let mut inode: *mut inode = core::ptr::null_mut();

    if fh_len < xfs_fileid_length(fileid_type) {
        return core::ptr::null_mut();
    }

    match fileid_type {
        FILEID_INO32_GEN_PARENT | FILEID_INO32_GEN => {
            inode = xfs_nfs_get_inode(sb, (*fid).i32.ino, (*fid).i32.gen);
        }
        x if x == (FILEID_INO32_GEN_PARENT | XFS_FILEID_TYPE_64FLAG)
            || x == (FILEID_INO32_GEN | XFS_FILEID_TYPE_64FLAG) => {
            inode = xfs_nfs_get_inode(sb, (*fid64).ino, (*fid64).gen);
        }
        _ => {}
    }

    d_obtain_alias(inode)
}

unsafe fn xfs_fs_fh_to_parent(
    sb: *mut super_block,
    fid: *mut fid,
    fh_len: i32,
    fileid_type: i32,
) -> *mut dentry {
    let fid64 = fid as *mut xfs_fid64;
    let mut inode: *mut inode = core::ptr::null_mut();

    if fh_len < xfs_fileid_length(fileid_type) {
        return core::ptr::null_mut();
    }

    match fileid_type {
        FILEID_INO32_GEN_PARENT => {
            inode = xfs_nfs_get_inode(sb, (*fid).i32.parent_ino, (*fid).i32.parent_gen);
        }
        x if x == (FILEID_INO32_GEN_PARENT | XFS_FILEID_TYPE_64FLAG) => {
            inode = xfs_nfs_get_inode(sb, (*fid64).parent_ino, (*fid64).parent_gen);
        }
        _ => {}
    }

    d_obtain_alias(inode)
}

unsafe fn xfs_fs_get_parent(child: *mut dentry) -> *mut dentry {
    let mut cip: *mut xfs_inode = core::ptr::null_mut();
    let error = xfs_lookup(XFS_I(d_inode(child)), &xfs_name_dotdot, &mut cip, core::ptr::null_mut());
    if unlikely(error != 0) {
        return ERR_PTR(error);
    }

    d_obtain_alias(VFS_I(cip))
}

unsafe fn xfs_fs_nfs_commit_metadata(inode: *mut inode) -> i32 {
    xfs_log_force_inode(XFS_I(inode))
}

const xfs_export_operations: export_operations = export_operations {
    encode_fh: Some(xfs_fs_encode_fh),
    fh_to_dentry: Some(xfs_fs_fh_to_dentry),
    fh_to_parent: Some(xfs_fs_fh_to_parent),
    get_parent: Some(xfs_fs_get_parent),
    commit_metadata: Some(xfs_fs_nfs_commit_metadata),
    // #ifdef CONFIG_EXPORTFS_BLOCK_OPS
    // block_ops: &xfs_export_block_ops,
    // #endif
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
