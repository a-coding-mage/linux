// SPDX-License-Identifier: GPL-2.0
/*
 * fs/isofs/export.c
 *
 *  (C) 2004  Paul Serice - The new inode scheme requires switching
 *                          from iget() to iget5_locked() which means
 *                          the NFS export operations have to be hand
 *                          coded because the default routines rely on
 *                          iget().
 *
 * The following files are helpful:
 *
 *     Documentation/filesystems/nfs/exporting.rst
 *     fs/exportfs/expfs.c.
 */

// Dependency declarations and macros are supplied by isofs.h and other
// translation units.

unsafe fn isofs_export_iget(
    sb: *mut super_block,
    block: c_ulong,
    offset: c_ulong,
    generation: u32,
) -> *mut dentry {
    let mut inode: *mut inode;

    if block == 0 || block >= (*ISOFS_SB(sb)).s_nzones {
        return ERR_PTR(-ESTALE);
    }
    inode = isofs_iget(sb, block, offset);
    if IS_ERR(inode) {
        return ERR_CAST(inode);
    }
    if generation != 0 && (*inode).i_generation != generation {
        iput(inode);
        return ERR_PTR(-ESTALE);
    }
    d_obtain_alias(inode)
}

/* This function is surprisingly simple.  The trick is understanding
 * that "child" is always a directory. So, to find its parent, you
 * simply need to find its ".." entry, normalize its block and offset,
 * and return the underlying inode.  See the comments for
 * isofs_normalize_block_and_offset(). */
unsafe fn isofs_export_get_parent(child: *mut dentry) -> *mut dentry {
    let mut parent_block: c_ulong = 0;
    let mut parent_offset: c_ulong = 0;
    let child_inode: *mut inode = d_inode(child);
    let e_child_inode: *mut iso_inode_info = ISOFS_I(child_inode);
    let mut de: *mut iso_directory_record = core::ptr::null_mut();
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut rv: *mut dentry = core::ptr::null_mut();

    /* "child" must always be a directory. */
    if !S_ISDIR((*child_inode).i_mode) {
        printk(KERN_ERR "isofs: isofs_export_get_parent(): child is not a directory!\n");
        rv = ERR_PTR(-EACCES);
        goto out;
    }

    /* It is an invariant that the directory offset is zero.  If
     * it is not zero, it means the directory failed to be
     * normalized for some reason. */
    if (*e_child_inode).i_iget5_offset != 0 {
        printk(KERN_ERR "isofs: isofs_export_get_parent(): child directory not normalized!\n");
        rv = ERR_PTR(-EACCES);
        goto out;
    }

    /* The child inode has been normalized such that its
     * i_iget5_block value points to the "." entry.  Fortunately,
     * the ".." entry is located in the same block. */
    parent_block = (*e_child_inode).i_iget5_block;

    /* Get the block in question. */
    bh = sb_bread((*child_inode).i_sb, parent_block);
    if bh.is_null() {
        rv = ERR_PTR(-EACCES);
        goto out;
    }

    /* This is the "." entry. */
    de = (*bh).b_data as *mut iso_directory_record;
    if !isofs_dir_record_valid(de, 0, (*(*child_inode).i_sb).s_blocksize)
        || isonum_711((*de).name_len) != 1
        || (*de).name[0] != 0
    {
        printk(KERN_ERR "isofs: Unable to find the \".\" directory for NFS.\n");
        rv = ERR_PTR(-EACCES);
        goto out;
    }

    /* The ".." entry is always the second entry. */
    parent_offset = isonum_711((*de).length) as c_ulong;
    de = ((*bh).b_data.add(parent_offset as usize)) as *mut iso_directory_record;

    /* Verify it is in fact the ".." entry. */
    if !isofs_dir_record_valid(de, parent_offset, (*(*child_inode).i_sb).s_blocksize)
        || isonum_711((*de).name_len) != 1
        || (*de).name[0] != 1
    {
        printk(KERN_ERR "isofs: Unable to find the \"..\" directory for NFS.\n");
        rv = ERR_PTR(-EACCES);
        goto out;
    }

    /* Normalize */
    isofs_normalize_block_and_offset(de, &mut parent_block, &mut parent_offset);

    rv = d_obtain_alias(isofs_iget((*child_inode).i_sb, parent_block, parent_offset));
out:
    if !bh.is_null() {
        brelse(bh);
    }
    rv
}

unsafe fn isofs_export_encode_fh(
    inode: *mut inode,
    fh32: *mut u32,
    max_len: *mut c_int,
    parent: *mut inode,
) -> c_int {
    let ei: *mut iso_inode_info = ISOFS_I(inode);
    let mut len = *max_len;
    let mut typ = 1;
    let fh16 = fh32 as *mut u16;

    /*
     * WARNING: max_len is 5 for NFSv2.  Because of this
     * limitation, we use the lower 16 bits of fh32[1] to hold the
     * offset of the inode and the upper 16 bits of fh32[1] to
     * hold the offset of the parent.
     */
    if !parent.is_null() && len < 5 {
        *max_len = 5;
        return FILEID_INVALID;
    } else if len < 3 {
        *max_len = 3;
        return FILEID_INVALID;
    }

    len = 3;
    *fh32.add(0) = (*ei).i_iget5_block as u32;
    *fh16.add(2) = (*ei).i_iget5_offset as u16; /* fh16 [sic] */
    *fh16.add(3) = 0; /* avoid leaking uninitialized data */
    *fh32.add(2) = (*inode).i_generation;
    if !parent.is_null() {
        let eparent: *mut iso_inode_info = ISOFS_I(parent);
        *fh32.add(3) = (*eparent).i_iget5_block as u32;
        *fh16.add(3) = (*eparent).i_iget5_offset as u16; /* fh16 [sic] */
        *fh32.add(4) = (*parent).i_generation;
        len = 5;
        typ = 2;
    }
    *max_len = len;
    typ
}

#[repr(C)]
struct isofs_fid {
    block: u32,
    offset: u16,
    parent_offset: u16,
    generation: u32,
    parent_block: u32,
    parent_generation: u32,
}

unsafe fn isofs_fh_to_dentry(
    sb: *mut super_block,
    fid: *mut fid,
    fh_len: c_int,
    fh_type: c_int,
) -> *mut dentry {
    let ifid = fid as *mut isofs_fid;

    if fh_len < 3 || fh_type > 2 {
        return core::ptr::null_mut();
    }

    isofs_export_iget(sb, (*ifid).block as c_ulong, (*ifid).offset as c_ulong,
                      (*ifid).generation)
}

unsafe fn isofs_fh_to_parent(
    sb: *mut super_block,
    fid: *mut fid,
    fh_len: c_int,
    fh_type: c_int,
) -> *mut dentry {
    let ifid = fid as *mut isofs_fid;

    if fh_len < 2 || fh_type != 2 {
        return core::ptr::null_mut();
    }

    isofs_export_iget(
        sb,
        if fh_len > 3 { (*ifid).parent_block as c_ulong } else { 0 },
        (*ifid).parent_offset as c_ulong,
        if fh_len > 4 { (*ifid).parent_generation } else { 0 },
    )
}

const isofs_export_ops: export_operations = export_operations {
    encode_fh: Some(isofs_export_encode_fh),
    fh_to_dentry: Some(isofs_fh_to_dentry),
    fh_to_parent: Some(isofs_fh_to_parent),
    get_parent: Some(isofs_export_get_parent),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
