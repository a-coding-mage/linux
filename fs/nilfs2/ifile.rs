// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS inode file
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Amagai Yoshiji.
 * Revised by Ryusuke Konishi.
 *
 */

/**
 * struct nilfs_ifile_info - on-memory private data of ifile
 * @mi: on-memory private data of metadata file
 * @palloc_cache: persistent object allocator cache of ifile
 */
#[repr(C)]
pub struct nilfs_ifile_info {
    pub mi: nilfs_mdt_info,
    pub palloc_cache: nilfs_palloc_cache,
}

#[inline]
unsafe fn NILFS_IFILE_I(ifile: *mut inode) -> *mut nilfs_ifile_info {
    NILFS_MDT(ifile) as *mut nilfs_ifile_info
}

/**
 * nilfs_ifile_create_inode - create a new disk inode
 * @ifile: ifile inode
 * @out_ino: pointer to a variable to store inode number
 * @out_bh: buffer_head contains newly allocated disk inode
 *
 * nilfs_ifile_create_inode() allocates a new inode in the ifile metadata
 * file and stores the inode number in the variable pointed to by @out_ino,
 * as well as storing the ifile's buffer with the disk inode in the location
 * pointed to by @out_bh.
 *
 * Return: 0 on success, or one of the following negative error codes on
 * failure:
 * * %-EIO      - I/O error (including metadata corruption).
 * * %-ENOMEM   - Insufficient memory available.
 * * %-ENOSPC   - No inode left.
 */
pub unsafe fn nilfs_ifile_create_inode(
    ifile: *mut inode,
    out_ino: *mut u64,
    out_bh: *mut *mut buffer_head,
) -> i32 {
    let mut req: nilfs_palloc_req = core::mem::zeroed();
    let mut ret: i32;

    req.pr_entry_nr = NILFS_FIRST_INO((*ifile).i_sb);
    req.pr_entry_bh = core::ptr::null_mut();

    ret = nilfs_palloc_prepare_alloc_entry(ifile, &mut req, false);
    if ret == 0 {
        ret = nilfs_palloc_get_entry_block(ifile, req.pr_entry_nr, 1,
                                           &mut req.pr_entry_bh);
        if ret < 0 {
            nilfs_palloc_abort_alloc_entry(ifile, &mut req);
        }
    }
    if ret < 0 {
        brelse(req.pr_entry_bh);
        return ret;
    }
    nilfs_palloc_commit_alloc_entry(ifile, &mut req);
    mark_buffer_dirty(req.pr_entry_bh);
    nilfs_mdt_mark_dirty(ifile);
    *out_ino = req.pr_entry_nr;
    *out_bh = req.pr_entry_bh;
    0
}

/**
 * nilfs_ifile_delete_inode - delete a disk inode
 * @ifile: ifile inode
 * @ino: inode number
 *
 * Return: 0 on success, or one of the following negative error codes on
 * failure:
 * * %-EIO      - I/O error (including metadata corruption).
 * * %-ENOENT   - Inode number unallocated.
 * * %-ENOMEM   - Insufficient memory available.
 */
pub unsafe fn nilfs_ifile_delete_inode(ifile: *mut inode, ino: u64) -> i32 {
    let mut req = nilfs_palloc_req {
        pr_entry_nr: ino,
        pr_entry_bh: core::ptr::null_mut(),
    };
    let raw_inode: *mut nilfs_inode;
    let offset: usize;
    let mut ret: i32;

    ret = nilfs_palloc_prepare_free_entry(ifile, &mut req);
    if ret == 0 {
        ret = nilfs_palloc_get_entry_block(ifile, req.pr_entry_nr, 0,
                                           &mut req.pr_entry_bh);
        if ret < 0 {
            nilfs_palloc_abort_free_entry(ifile, &mut req);
        }
    }
    if ret < 0 {
        brelse(req.pr_entry_bh);
        return ret;
    }

    offset = nilfs_palloc_entry_offset(ifile, req.pr_entry_nr, req.pr_entry_bh);
    raw_inode = kmap_local_folio((*req.pr_entry_bh).b_folio, offset);
    (*raw_inode).i_flags = 0;
    kunmap_local(raw_inode as *mut core::ffi::c_void);

    mark_buffer_dirty(req.pr_entry_bh);
    brelse(req.pr_entry_bh);

    nilfs_palloc_commit_free_entry(ifile, &mut req);

    0
}

pub unsafe fn nilfs_ifile_get_inode_block(
    ifile: *mut inode,
    ino: u64,
    out_bh: *mut *mut buffer_head,
) -> i32 {
    let sb: *mut super_block = (*ifile).i_sb;
    let mut err: i32;

    if !NILFS_VALID_INODE(sb, ino) {
        nilfs_error(sb, "bad inode number: %llu", ino);
        return -EINVAL;
    }

    err = nilfs_palloc_get_entry_block(ifile, ino, 0, out_bh);
    if err != 0 {
        nilfs_warn(sb, "error %d reading inode: ino=%llu", err, ino);
    }
    err
}

/**
 * nilfs_ifile_count_free_inodes - calculate free inodes count
 * @ifile: ifile inode
 * @nmaxinodes: current maximum of available inodes count [out]
 * @nfreeinodes: free inodes count [out]
 *
 * Return: 0 on success, or a negative error code on failure.
 */
pub unsafe fn nilfs_ifile_count_free_inodes(
    ifile: *mut inode,
    nmaxinodes: *mut u64,
    nfreeinodes: *mut u64,
) -> i32 {
    let nused: u64;
    let mut err: i32;

    *nmaxinodes = 0;
    *nfreeinodes = 0;

    nused = atomic64_read(&(*NILFS_I(ifile)).i_root.inodes_count);
    err = nilfs_palloc_count_max_entries(ifile, nused, nmaxinodes);
    if err == 0 {
        *nfreeinodes = (*nmaxinodes).wrapping_sub(nused);
    }
    err
}

/**
 * nilfs_ifile_read - read or get ifile inode
 * @sb: super block instance
 * @root: root object
 * @cno: number of checkpoint entry to read
 * @inode_size: size of an inode
 *
 * Return: 0 on success, or one of the following negative error codes on
 * failure:
 * * %-EINVAL    - Invalid checkpoint.
 * * %-ENOMEM    - Insufficient memory available.
 * * %-EIO       - I/O error (including metadata corruption).
 */
pub unsafe fn nilfs_ifile_read(
    sb: *mut super_block,
    root: *mut nilfs_root,
    cno: u64,
    inode_size: usize,
) -> i32 {
    let nilfs: *mut the_nilfs;
    let ifile: *mut inode;
    let mut err: i32;

    ifile = nilfs_iget_locked(sb, root, NILFS_IFILE_INO);
    if ifile.is_null() {
        return -ENOMEM;
    }
    if (inode_state_read_once(ifile) & I_NEW) == 0 {
        return 0;
    }

    err = nilfs_mdt_init(ifile, NILFS_MDT_GFP,
                        core::mem::size_of::<nilfs_ifile_info>());
    if err != 0 {
        iget_failed(ifile);
        return err;
    }

    err = nilfs_palloc_init_blockgroup(ifile, inode_size);
    if err != 0 {
        iget_failed(ifile);
        return err;
    }

    nilfs_palloc_setup_cache(ifile, &mut (*NILFS_IFILE_I(ifile)).palloc_cache);

    nilfs = (*sb).s_fs_info;
    err = nilfs_cpfile_read_checkpoint((*nilfs).ns_cpfile, cno, root, ifile);
    if err != 0 {
        iget_failed(ifile);
        return err;
    }

    unlock_new_inode(ifile);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
