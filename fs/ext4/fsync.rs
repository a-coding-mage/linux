// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ext4/fsync.c
 *
 *  Copyright (C) 1993  Stephen Tweedie (sct@redhat.com)
 *  from
 *  Copyright (C) 1992  Remy Card (card@masi.ibp.fr)
 *                      Laboratoire MASI - Institut Blaise Pascal
 *                      Universite Pierre et Marie Curie (Paris VI)
 *  from
 *  linux/fs/minix/truncate.c   Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  ext4fs fsync primitive
 *
 *  Big-endian to little-endian byte-swapping/bitmaps by
 *        David S. Miller (davem@caip.rutgers.edu), 1995
 *
 *  Removed unnecessary code duplication for little endian machines
 *  and excessive __inline__s.
 *        Andi Kleen, 1997
 *
 * Major simplications and cleanup - we only need to do the metadata, because
 * we can depend on generic_block_fdatasync() to sync the data blocks.
 */

// Dependencies supplied by the kernel and ext4 headers:
// linux/time.h, linux/fs.h, linux/sched.h, linux/writeback.h,
// linux/blkdev.h, linux/buffer_head.h, ext4.h, ext4_jbd2.h,
// and trace/events/ext4.h.

/*
 * If we're not journaling and this is a just-created file, we have to
 * sync our parent directory (if it was freshly created) since
 * otherwise it will only be written by writeback, leaving a huge
 * window during which a crash may lose the file.  This may apply for
 * the parent directory's parent as well, and so on recursively, if
 * they are also freshly created.
 */
unsafe fn ext4_sync_parent(mut inode: *mut inode) -> i32 {
    let mut dentry: *mut dentry;
    let mut next: *mut dentry;
    let mut ret: i32 = 0;

    if !ext4_test_inode_state(inode, EXT4_STATE_NEWENTRY) {
        return 0;
    }
    dentry = d_find_any_alias(inode);
    if dentry.is_null() {
        return 0;
    }
    while ext4_test_inode_state(inode, EXT4_STATE_NEWENTRY) {
        ext4_clear_inode_state(inode, EXT4_STATE_NEWENTRY);

        next = dget_parent(dentry);
        dput(dentry);
        dentry = next;
        inode = (*dentry).d_inode;

        /*
         * The directory inode may have gone through rmdir by now. But
         * the inode itself and its blocks are still allocated (we hold
         * a reference to the inode via its dentry), so it didn't go
         * through ext4_evict_inode()) and so we are safe to flush
         * metadata blocks and the inode.
         */
        ret = sync_inode_metadata(inode, 1);
        if ret != 0 {
            break;
        }
    }
    dput(dentry);
    ret
}

unsafe fn ext4_fsync_nojournal(
    file: *mut file,
    _start: loff_t,
    _end: loff_t,
    _datasync: i32,
    needs_barrier: *mut bool,
) -> i32 {
    let inode: *mut inode = (*file).f_inode;
    let mut ret: i32;

    ret = sync_inode_metadata(inode, 1);
    if ret != 0 {
        return ret;
    }
    ret = ext4_sync_parent(inode);

    if test_opt((*inode).i_sb, BARRIER) {
        *needs_barrier = true;
    }

    ret
}

unsafe fn ext4_fsync_journal(
    inode: *mut inode,
    datasync: bool,
    needs_barrier: *mut bool,
) -> i32 {
    let ei: *mut ext4_inode_info = EXT4_I(inode);
    let journal: *mut journal_t = EXT4_SB((*inode).i_sb).s_journal;
    let commit_tid: tid_t = if datasync {
        (*ei).i_datasync_tid
    } else {
        (*ei).i_sync_tid
    };

    /*
     * Fastcommit does not really support fsync on directories or other
     * special files. Force a full commit.
     */
    if !S_ISREG((*inode).i_mode) {
        return ext4_force_commit((*inode).i_sb);
    }

    if (*journal).j_flags & JBD2_BARRIER != 0
        && !jbd2_trans_will_send_data_barrier(journal, commit_tid)
    {
        *needs_barrier = true;
    }

    ext4_fc_commit(journal, commit_tid)
}

/*
 * akpm: A new design for ext4_sync_file().
 *
 * This is only called from sys_fsync(), sys_fdatasync() and sys_msync().
 * There cannot be a transaction open by this task.
 * Another task could have dirtied this inode.  Its data can be in any
 * state in the journalling system.
 *
 * What we do is just kick off a commit and wait on it.  This will snapshot the
 * inode to disk.
 */
pub unsafe fn ext4_sync_file(file: *mut file, start: loff_t, end: loff_t, datasync: i32) -> i32 {
    let mut ret: i32 = 0;
    let mut err: i32;
    let mut needs_barrier: bool = false;
    let inode: *mut inode = (*(*file).f_mapping).host;

    ret = ext4_emergency_state((*inode).i_sb);
    if unlikely(ret != 0) {
        return ret;
    }

    ASSERT(ext4_journal_current_handle().is_null());

    trace_ext4_sync_file_enter(file, datasync);

    if !sb_rdonly((*inode).i_sb) {
        ret = file_write_and_wait_range(file, start, end);
        if ret != 0 {
        } else if EXT4_SB((*inode).i_sb).s_journal.is_null() {
            ret = ext4_fsync_nojournal(file, start, end, datasync, &mut needs_barrier);
        } else {
            /*
             *  The caller's filemap_fdatawrite()/wait will sync the data.
             *  Metadata is in the journal, we wait for proper transaction to
             *  commit here.
             */
            ret = ext4_fsync_journal(inode, datasync != 0, &mut needs_barrier);
        }
    }

    if needs_barrier {
        err = blkdev_issue_flush((*(*inode).i_sb).s_bdev);
        if ret == 0 {
            ret = err;
        }
    }
    err = file_check_and_advance_wb_err(file);
    if ret == 0 {
        ret = err;
    }
    trace_ext4_sync_file_exit(inode, ret);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
