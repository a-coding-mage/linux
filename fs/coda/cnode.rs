// SPDX-License-Identifier: GPL-2.0
/* cnode related routines for the coda kernel code
   (C) 1996 Peter Braam
   */

// Linux and Coda declarations supplied by the surrounding kernel translation.

#[inline]
unsafe fn coda_fideq(fid1: *const CodaFid, fid2: *const CodaFid) -> i32 {
    (core::slice::from_raw_parts(
        fid1 as *const u8,
        core::mem::size_of::<CodaFid>(),
    ) == core::slice::from_raw_parts(
        fid2 as *const u8,
        core::mem::size_of::<CodaFid>(),
    )) as i32
}

static CODA_SYMLINK_INODE_OPERATIONS: inode_operations = inode_operations {
    get_link: Some(page_get_link),
    setattr: Some(coda_setattr),
};

/* cnode.c */
unsafe fn coda_fill_inode(inode: *mut inode, attr: *mut coda_vattr) {
    coda_vattr_to_iattr(inode, attr);

    if S_ISREG((*inode).i_mode) {
        (*inode).i_op = &coda_file_inode_operations;
        (*inode).i_fop = &coda_file_operations;
    } else if S_ISDIR((*inode).i_mode) {
        (*inode).i_op = &coda_dir_inode_operations;
        (*inode).i_fop = &coda_dir_operations;
    } else if S_ISLNK((*inode).i_mode) {
        (*inode).i_op = &CODA_SYMLINK_INODE_OPERATIONS;
        inode_nohighmem(inode);
        (*inode).i_data.a_ops = &coda_symlink_aops;
        (*inode).i_mapping = &mut (*inode).i_data;
    } else {
        init_special_inode(
            inode,
            (*inode).i_mode,
            huge_decode_dev((*attr).va_rdev),
        );
    }
}

unsafe fn coda_test_inode(inode: *mut inode, data: *mut core::ffi::c_void) -> i32 {
    let fid = data as *mut CodaFid;
    let cii = ITOC(inode);
    coda_fideq(&(*cii).c_fid, fid)
}

unsafe fn coda_set_inode(inode: *mut inode, data: *mut core::ffi::c_void) -> i32 {
    let fid = data as *mut CodaFid;
    let cii = ITOC(inode);
    (*cii).c_fid = *fid;
    0
}

unsafe fn coda_iget(
    sb: *mut super_block,
    fid: *mut CodaFid,
    attr: *mut coda_vattr,
) -> *mut inode {
    let mut inode: *mut inode;
    let mut cii: *mut coda_inode_info;
    let hash: c_ulong = coda_f2i(fid);
    let inode_type: umode_t = coda_inode_type(attr);

    'retry: loop {
        inode = iget5_locked(sb, hash, Some(coda_test_inode), Some(coda_set_inode), fid as *mut core::ffi::c_void);
        if inode.is_null() {
            return ERR_PTR(-ENOMEM);
        }

        if inode_state_read_once(inode) & I_NEW != 0 {
            cii = ITOC(inode);
            /* we still need to set i_ino for things like stat(2) */
            (*inode).i_ino = hash;
            /* inode is locked and unique, no need to grab cii->c_lock */
            (*cii).c_mapcount = 0;
            coda_fill_inode(inode, attr);
            unlock_new_inode(inode);
        } else if ((*inode).i_mode & S_IFMT) != inode_type {
            /* Inode has changed type, mark bad and grab a new one */
            remove_inode_hash(inode);
            coda_flag_inode(inode, C_PURGE);
            iput(inode);
            continue 'retry;
        }
        return inode;
    }
}

/* this is effectively coda_iget:
   - get attributes (might be cached)
   - get the inode for the fid using vfs iget
   - link the two up if this is needed
   - fill in the attributes
*/
unsafe fn coda_cnode_make(fid: *mut CodaFid, sb: *mut super_block) -> *mut inode {
    let mut attr: coda_vattr = core::mem::zeroed();
    let error: i32;

    /* We get inode numbers from Venus -- see venus source */
    error = venus_getattr(sb, fid, &mut attr);
    if error != 0 {
        return ERR_PTR(error);
    }

    let inode = coda_iget(sb, fid, &mut attr);
    if IS_ERR(inode) {
        pr_warn!("%s: coda_iget failed\n", "coda_cnode_make");
    }
    inode
}

/* Although we treat Coda file identifiers as immutable, there is one
 * special case for files created during a disconnection where they may
 * not be globally unique. When an identifier collision is detected we
 * first try to flush the cached inode from the kernel and finally
 * resort to renaming/rehashing in-place. Userspace remembers both old
 * and new values of the identifier to handle any in-flight upcalls.
 * The real solution is to use globally unique UUIDs as identifiers, but
 * retrofitting the existing userspace code for this is non-trivial. */
unsafe fn coda_replace_fid(
    inode: *mut inode,
    oldfid: *mut CodaFid,
    newfid: *mut CodaFid,
) {
    let cii = ITOC(inode);
    let hash: c_ulong = coda_f2i(newfid);

    BUG_ON(!coda_fideq(&(*cii).c_fid, oldfid));

    /* replace fid and rehash inode */
    /* XXX we probably need to hold some lock here! */
    remove_inode_hash(inode);
    (*cii).c_fid = *newfid;
    (*inode).i_ino = hash;
    __insert_inode_hash(inode, hash);
}

/* convert a fid to an inode. */
unsafe fn coda_fid_to_inode(fid: *mut CodaFid, sb: *mut super_block) -> *mut inode {
    let hash: c_ulong = coda_f2i(fid);
    let inode = ilookup5(sb, hash, Some(coda_test_inode), fid as *mut core::ffi::c_void);
    if inode.is_null() {
        return core::ptr::null_mut();
    }

    /* we should never see newly created inodes because we intentionally
     * fail in the initialization callback */
    BUG_ON(inode_state_read_once(inode) & I_NEW != 0);
    inode
}

unsafe fn coda_ftoc(file: *mut file) -> *mut coda_file_info {
    let cfi = (*file).private_data as *mut coda_file_info;
    BUG_ON(cfi.is_null() || (*cfi).cfi_magic != CODA_MAGIC);
    cfi
}

/* the CONTROL inode is made without asking attributes from Venus */
unsafe fn coda_cnode_makectl(sb: *mut super_block) -> *mut inode {
    let inode = new_inode(sb);
    if !inode.is_null() {
        (*inode).i_ino = CTL_INO;
        (*inode).i_op = &coda_ioctl_inode_operations;
        (*inode).i_fop = &coda_ioctl_operations;
        (*inode).i_mode = 0o444;
        return inode;
    }
    ERR_PTR(-ENOMEM)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
