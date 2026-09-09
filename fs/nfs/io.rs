// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2016 Trond Myklebust
 *
 * I/O and data path helper functionality.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external to this source file.

/**
 * nfs_start_io_read - declare the file is being used for buffered reads
 * @inode: file inode
 *
 * Declare that a buffered read operation is about to start, and ensure
 * that we block all direct I/O.
 */
pub unsafe fn nfs_start_io_read(inode: *mut inode) -> i32 {
    let nfsi: *mut nfs_inode = NFS_I(inode);
    let mut err: i32;

    /* Be an optimist! */
    err = down_read_killable(&mut (*inode).i_rwsem);
    if err != 0 {
        return err;
    }
    if test_bit(NFS_INO_ODIRECT, &(*nfsi).flags) == 0 {
        return 0;
    }
    up_read(&mut (*inode).i_rwsem);

    /* Slow path.... */
    err = down_write_killable(&mut (*inode).i_rwsem);
    if err != 0 {
        return err;
    }
    nfs_file_block_o_direct(nfsi);
    downgrade_write(&mut (*inode).i_rwsem);

    0
}

/**
 * nfs_end_io_read - declare that the buffered read operation is done
 * @inode: file inode
 */
pub unsafe fn nfs_end_io_read(inode: *mut inode) {
    up_read(&mut (*inode).i_rwsem);
}

/**
 * nfs_start_io_write - declare the file is being used for buffered writes
 * @inode: file inode
 */
pub unsafe fn nfs_start_io_write(inode: *mut inode) -> i32 {
    let err: i32 = down_write_killable(&mut (*inode).i_rwsem);

    if err == 0 {
        nfs_file_block_o_direct(NFS_I(inode));
    }
    err
}

// EXPORT_SYMBOL_GPL(nfs_start_io_write);

/**
 * nfs_end_io_write - declare that the buffered write operation is done
 * @inode: file inode
 */
pub unsafe fn nfs_end_io_write(inode: *mut inode) {
    up_write(&mut (*inode).i_rwsem);
}

// EXPORT_SYMBOL_GPL(nfs_end_io_write);

/* Call with exclusively locked inode->i_rwsem */
unsafe fn nfs_block_buffered(nfsi: *mut nfs_inode, inode: *mut inode) {
    if !test_bit(NFS_INO_ODIRECT, &(*nfsi).flags) {
        set_bit(NFS_INO_ODIRECT, &mut (*nfsi).flags);
        nfs_sync_mapping((*inode).i_mapping);
    }
}

unsafe fn nfs_block_buffered_nowait(nfsi: *mut nfs_inode, inode: *mut inode) -> i32 {
    if !test_bit(NFS_INO_ODIRECT, &(*nfsi).flags) {
        if (*(*inode).i_mapping).nrpages != 0 {
            return 1;
        }
        set_bit(NFS_INO_ODIRECT, &mut (*nfsi).flags);
    }
    0
}

/**
 * nfs_start_io_direct - declare the file is being used for direct i/o
 * @inode: file inode
 */
pub unsafe fn nfs_start_io_direct(inode: *mut inode) -> i32 {
    let nfsi: *mut nfs_inode = NFS_I(inode);
    let mut err: i32;

    /* Be an optimist! */
    err = down_read_killable(&mut (*inode).i_rwsem);
    if err != 0 {
        return err;
    }
    if test_bit(NFS_INO_ODIRECT, &(*nfsi).flags) != 0 {
        return 0;
    }
    up_read(&mut (*inode).i_rwsem);

    /* Slow path.... */
    err = down_write_killable(&mut (*inode).i_rwsem);
    if err != 0 {
        return err;
    }
    nfs_block_buffered(nfsi, inode);
    downgrade_write(&mut (*inode).i_rwsem);

    0
}

/**
 * nfs_start_io_direct_nowait - non-blocking variant of nfs_start_io_direct()
 * @inode: file inode
 */
pub unsafe fn nfs_start_io_direct_nowait(inode: *mut inode) -> i32 {
    let nfsi: *mut nfs_inode = NFS_I(inode);

    if !down_read_trylock(&mut (*inode).i_rwsem) {
        return -EAGAIN;
    }
    if test_bit(NFS_INO_ODIRECT, &(*nfsi).flags) {
        return 0;
    }
    up_read(&mut (*inode).i_rwsem);

    /* Slow path: try to flip NFS_INO_ODIRECT without blocking. */
    if !down_write_trylock(&mut (*inode).i_rwsem) {
        return -EAGAIN;
    }
    if nfs_block_buffered_nowait(nfsi, inode) != 0 {
        up_write(&mut (*inode).i_rwsem);
        return -EAGAIN;
    }
    downgrade_write(&mut (*inode).i_rwsem);
    0
}

/**
 * nfs_end_io_direct - declare that the direct i/o operation is done
 * @inode: file inode
 */
pub unsafe fn nfs_end_io_direct(inode: *mut inode) {
    up_read(&mut (*inode).i_rwsem);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
