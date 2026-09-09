// SPDX-License-Identifier: GPL-2.0
/*
 * fn Copyright(c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

fn const vm_operations_struct xfs_file_vm_ops;
/*
 * Decide if the given file range is aligned to the size of the fundamental
 * allocation unit for the file.
 */
bool
fn xfs_is_falloc_aligned(
	*mut xfs_inodeip,
	i64			pos,
	i64		len)
{
	u32		alloc_unit = fn xfs_inode_alloc_unitsize(ip);
	fn if(!fn is_power_of_2(alloc_unit))
		return fn isaligned_64(pos, alloc_unit) &&
		       fn isaligned_64(len, alloc_unit);
	return !((pos | len) & (alloc_unit - 1));
}

/*
 * Fsync operations on directories are much simpler than on regular files,
 * as there is no file data to flush, and thus also no need for explicit
 * cache flush operations, and there are no non-transaction metadata updates
 * on directories either.
 */
pub i32
fn xfs_dir_fsync(
	*mut filefile,
	i64			start,
	i64			end,
	i32			datasync)
{
	*mut xfs_inodeip = fn XFS_I(file->f_mapping->host);
	fn trace_xfs_dir_fsync(ip);
	return fn xfs_log_force_inode(ip);
}

/*
 * All metadata updates are logged, which means that we just have to push the
 * journal to the required sequence number than holds the updates. We track
 * datasync commits separately to full sync commits, and hence only need to
 * select the correct sequence number for the log force here.
 *
 * We don't have to serialise against concurrent modifications, as we do not
 * have to wait for modifications that have not yet completed. We define a
 * transaction commit as completing when the commit sequence number is updated,
 * hence if the sequence number has not updated, the sync operation has been
 * run before the commit completed and we don't have to wait for it.
 *
 * If we have concurrent fsync/fn fdatasync() calls, the sequence numbers remain
 * set on the log item until - at least - the journal flush completes. In
 * reality, they are only cleared when the inode is fully fn unpinned(i.e.
 * persistent in the journal and not dirty in the CIL), and so we rely on
 * fn xfs_log_force_seq() either skipping sequences that have been persisted or
 * waiting on sequences that are still in flight to correctly order concurrent
 * sync operations.
 */
fn i32
fn xfs_fsync_flush_log(
	*mut xfs_inodeip,
	bool			datasync,
	i32			*log_flushed)
{
	*mut xfs_inode_log_itemiip = ip->i_itemp;
	xfs_csn_t		seq = 0;
	fn spin_lock(&iip->ili_lock);
	fn if(datasync)
		seq = iip->ili_datasync_seq;
	else
		seq = iip->ili_commit_seq;
	fn spin_unlock(&iip->ili_lock);
	fn if(!seq)
		return 0;
	return fn xfs_log_force_seq(ip->i_mount, seq, XFS_LOG_SYNC,
					  log_flushed);
}

pub i32
fn xfs_file_fsync(
	*mut filefile,
	i64			start,
	i64			end,
	i32			datasync)
{
	*mut xfs_inodeip = fn XFS_I(file->f_mapping->host);
	*mut xfs_mountmp = ip->i_mount;
	i32			error, err2;
	i32			log_flushed = 0;
	fn trace_xfs_file_fsync(ip);
	error = fn file_write_and_wait_range(file, start, end);
	fn if(error)
		return error;
	fn if(fn xfs_is_shutdown(mp))
		return -EIO;
	fn xfs_iflags_clear(ip, XFS_ITRUNCATED);
	/*
	 * If we have an RT and/or log subvolume we need to make sure to flush
	 * the write cache the device used for file data first.  This is to
	 * ensure newly written file data make it to disk before logging the new
	 * inode size in case of an extending write.
	 */
	fn if(fn XFS_IS_REALTIME_INODE(ip) && mp->m_rtdev_targp != mp->m_ddev_targp)
		error = fn blkdev_issue_flush(mp->m_rtdev_targp->bt_bdev);
	else fn if(mp->m_logdev_targp != mp->m_ddev_targp)
		error = fn blkdev_issue_flush(mp->m_ddev_targp->bt_bdev);
	/*
	 * If the inode has a inode log item attached, it may need the journal
	 * flushed to persist any changes the log item might be tracking.
	 */
	fn if(ip->i_itemp) {
		err2 = fn xfs_fsync_flush_log(ip, datasync, &log_flushed);
		fn if(err2 && !error)
			error = err2;
	}

	/*
	 * If the log force was a no-op, we may still need to flush the
	 * file data target cache here. This can happen for fdatasync/O_DSYNC
	 * when no metadata needed to be committed.
	 *
	 * Use the inode's actual file data target rather than assuming the
	 * main data device. Realtime inodes with a separate realtime device
	 * are flushed before the log force, so this fallback only applies
	 * when the file data target is the same as the log target.
	 */
	fn if(!log_flushed) {
		*mut xfs_buftargfile_targp = fn xfs_inode_buftarg(ip);
		fn if(mp->m_logdev_targp == file_targp) {
			err2 = fn blkdev_issue_flush(file_targp->bt_bdev);
			fn if(err2 && !error)
				error = err2;
		}
	}

	return error;
}

fn i32
fn xfs_ilock_iocb(
	*mut kiocbiocb,
	u32		lock_mode)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(iocb->ki_filp));
	fn if(iocb->ki_flags & IOCB_NOWAIT) {
		fn if(!fn xfs_ilock_nowait(ip, lock_mode))
			return -EAGAIN;
	} else {
		fn xfs_ilock(ip, lock_mode);
	}

	return 0;
}

fn i32
fn xfs_ilock_iocb_for_write(
	*mut kiocbiocb,
	u32		*lock_mode)
{
	isize			ret;
	*mut xfs_inodeip = fn XFS_I(fn file_inode(iocb->ki_filp));
	ret = fn xfs_ilock_iocb(iocb, *lock_mode);
	fn if(ret)
		return ret;
	/*
	 * If a reflink remap is in progress we always need to take the iolock
	 * exclusively to wait for it to finish.
	 */
	fn if(*lock_mode == XFS_IOLOCK_SHARED &&
	    fn xfs_iflags_test(ip, XFS_IREMAPPING)) {
		fn xfs_iunlock(ip, *lock_mode);
		*lock_mode = XFS_IOLOCK_EXCL;
		return fn xfs_ilock_iocb(iocb, *lock_mode);
	}

	return 0;
}

/*
 * Bounce buffering dio reads need a user context to copy back the data.
 * Use an ioend to provide that.
 */
fn()
fn xfs_dio_read_bounce_submit_io(
	const iomap_iter	*iter,
	*mut biobio,
	i64			file_offset)
{
	fn iomap_init_ioend(iter->inode, bio, file_offset, IOMAP_IOEND_DIRECT);
	bio->bi_end_io = xfs_end_bio;
	fn submit_bio(bio);
}

fn const iomap_dio_ops xfs_dio_read_bounce_ops = {
	.submit_io	= xfs_dio_read_bounce_submit_io,
	.bio_set	= &iomap_ioend_bioset,
};
pub isize
fn xfs_file_dio_read(
	*mut kiocbiocb,
	*mut iov_iterto)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(iocb->ki_filp));
	isize			ret;
	fn trace_xfs_file_direct_read(iocb, to);
	fn if(!fn iov_iter_count(to))
		return 0; /* skip atime */

	fn file_accessed(iocb->ki_filp);
	ret = fn xfs_ilock_iocb(iocb, XFS_IOLOCK_SHARED);
	fn if(ret)
		return ret;
	fn if(fn mapping_stable_writes(iocb->ki_filp->f_mapping)) {
		ret = fn iomap_dio_rw(iocb, to, &xfs_read_iomap_ops,
				&xfs_dio_read_bounce_ops, IOMAP_DIO_BOUNCE,
				core::ptr::fn null_mut(), 0);
	} else {
		ret = fn iomap_dio_read_simple(iocb, to, xfs_read_iomap_begin);
		fn if(ret == -ENOTBLK)
			ret = fn iomap_dio_rw(iocb, to, &xfs_read_iomap_ops, core::ptr::fn null_mut(),
					0, core::ptr::fn null_mut(), 0);
	}
	fn xfs_iunlock(ip, XFS_IOLOCK_SHARED);
	return ret;
}

fn isize
fn xfs_file_dax_read(
	*mut kiocbiocb,
	*mut iov_iterto)
{
	*mut xfs_inodeip = fn XFS_I(iocb->ki_filp->f_mapping->host);
	isize			ret = 0;
	fn trace_xfs_file_dax_read(iocb, to);
	fn if(!fn iov_iter_count(to))
		return 0; /* skip atime */

	ret = fn xfs_ilock_iocb(iocb, XFS_IOLOCK_SHARED);
	fn if(ret)
		return ret;
	ret = fn dax_iomap_rw(iocb, to, &xfs_read_iomap_ops);
	fn xfs_iunlock(ip, XFS_IOLOCK_SHARED);
	fn file_accessed(iocb->ki_filp);
	return ret;
}

pub isize
fn xfs_file_buffered_read(
	*mut kiocbiocb,
	*mut iov_iterto)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(iocb->ki_filp));
	isize			ret;
	fn trace_xfs_file_buffered_read(iocb, to);
	ret = fn xfs_ilock_iocb(iocb, XFS_IOLOCK_SHARED);
	fn if(ret)
		return ret;
	ret = fn generic_file_read_iter(iocb, to);
	fn xfs_iunlock(ip, XFS_IOLOCK_SHARED);
	return ret;
}

pub isize
fn xfs_file_read_iter(
	*mut kiocbiocb,
	*mut iov_iterto)
{
	*mut inodeinode = fn file_inode(iocb->ki_filp);
	*mut xfs_mountmp = fn XFS_I(inode)->i_mount;
	isize			ret = 0;
	fn XFS_STATS_INC(mp, xs_read_calls);
	fn if(fn xfs_is_shutdown(mp))
		return -EIO;
	fn if(fn IS_DAX(inode))
		ret = fn xfs_file_dax_read(iocb, to);
	else fn if(iocb->ki_flags & IOCB_DIRECT)
		ret = fn xfs_file_dio_read(iocb, to);
	else
		ret = fn xfs_file_buffered_read(iocb, to);
	fn if(ret > 0)
		fn XFS_STATS_ADD(mp, xs_read_bytes, ret);
	return ret;
}

pub isize
fn xfs_file_splice_read(
	*mut filein,
	i64			*ppos,
	*mut pipe_inode_infopipe,
	usize			len,
	u32		flags)
{
	*mut inodeinode = fn file_inode(in);
	*mut xfs_inodeip = fn XFS_I(inode);
	*mut xfs_mountmp = ip->i_mount;
	isize			ret = 0;
	fn XFS_STATS_INC(mp, xs_read_calls);
	fn if(fn xfs_is_shutdown(mp))
		return -EIO;
	fn trace_xfs_file_splice_read(ip, *ppos, len);
	fn xfs_ilock(ip, XFS_IOLOCK_SHARED);
	ret = fn filemap_splice_read(in, ppos, pipe, len, flags);
	fn xfs_iunlock(ip, XFS_IOLOCK_SHARED);
	fn if(ret > 0)
		fn XFS_STATS_ADD(mp, xs_read_bytes, ret);
	return ret;
}

/*
 * Take care of zeroing post-EOF blocks when they might exist.
 *
 * Returns 0 if successfully, a negative error for a failure, or 1 if this
 * function dropped the iolock and reacquired it exclusively and the caller
 * needs to restart the write sanity checks.
 */
fn isize
fn xfs_file_write_zero_eof(
	*mut kiocbiocb,
	*mut iov_iterfrom,
	u32		*iolock,
	usize			count,
	bool			*drained_dio,
	*mut xfs_zone_alloc_ctxac)
{
	*mut xfs_inodeip = fn XFS_I(iocb->ki_filp->f_mapping->host);
	i64			isize;
	i32			error;
	/*
	 * We need to serialise against EOF updates that occur in IO completions
	 * here. We want to make sure that nobody is changing the size while
	 * we do this check until we have placed an IO fn barrier(i.e. hold
	 * XFS_IOLOCK_EXCL) that prevents new IO from being dispatched.  The
	 * spinlock effectively forms a memory barrier once we have
	 * XFS_IOLOCK_EXCL so we are guaranteed to see the latest EOF value and
	 * hence be able to correctly determine if we need to run zeroing.
	 */
	fn spin_lock(&ip->i_flags_lock);
	isize = fn i_size_read(fn VFS_I(ip));
	fn if(iocb->ki_pos <= isize) {
		fn spin_unlock(&ip->i_flags_lock);
		return 0;
	}
	fn spin_unlock(&ip->i_flags_lock);
	fn if(iocb->ki_flags & IOCB_NOWAIT)
		return -EAGAIN;
	fn if(!*drained_dio) {
		/*
		 * If zeroing is needed and we are currently holding the iolock
		 * shared, we need to update it to exclusive which implies
		 * having to redo all checks before.
		 */
		fn if(*iolock == XFS_IOLOCK_SHARED) {
			fn xfs_iunlock(ip, *iolock);
			*iolock = XFS_IOLOCK_EXCL;
			fn xfs_ilock(ip, *iolock);
			fn iov_iter_reexpand(from, count);
		}

		/*
		 * We now have an IO submission barrier in place, but AIO can do
		 * EOF updates during IO completion and hence we now need to
		 * wait for all of them to drain.  Non-AIO DIO will have drained
		 * before we are given the XFS_IOLOCK_EXCL, and so for most
		 * cases this wait is a no-op.
		 */
		fn inode_dio_wait(fn VFS_I(ip));
		*drained_dio = true;
		return 1;
	}

	fn trace_xfs_zero_eof(ip, isize, iocb->ki_pos - isize);
	fn xfs_ilock(ip, XFS_MMAPLOCK_EXCL);
	error = fn xfs_zero_range(ip, isize, iocb->ki_pos - isize, ac, core::ptr::fn null_mut());
	fn xfs_iunlock(ip, XFS_MMAPLOCK_EXCL);
	return error;
}

/*
 * Common pre-write limit and setup checks.
 *
 * Called with the iolock held either shared and exclusive according to
 * @iolock, and returns with it held.  Might upgrade the iolock to exclusive
 * if called for a direct write beyond i_size.
 */
pub isize
fn xfs_file_write_checks(
	*mut kiocbiocb,
	*mut iov_iterfrom,
	u32		*iolock,
	*mut xfs_zone_alloc_ctxac)
{
	*mut inodeinode = iocb->ki_filp->f_mapping->host;
	usize			count = fn iov_iter_count(from);
	bool			drained_dio = false;
	isize			error;
restart:
	error = fn generic_write_checks(iocb, from);
	fn if(error <= 0)
		return error;
	fn if(iocb->ki_flags & IOCB_NOWAIT) {
		error = fn break_layout(inode, false);
		fn if(error == -EWOULDBLOCK)
			error = -EAGAIN;
	} else {
		error = fn xfs_break_layouts(inode, iolock, BREAK_WRITE);
	}

	fn if(error)
		return error;
	/*
	 * For changing security info in fn file_remove_privs() we need i_rwsem
	 * exclusively.
	 */
	fn if(*iolock == XFS_IOLOCK_SHARED && !fn IS_NOSEC(inode)) {
		fn xfs_iunlock(fn XFS_I(inode), *iolock);
		*iolock = XFS_IOLOCK_EXCL;
		error = fn xfs_ilock_iocb(iocb, *iolock);
		fn if(error) {
			*iolock = 0;
			return error;
		}
		goto restart;
	}

	/*
	 * If the offset is beyond the size of the file, we need to zero all
	 * blocks that fall between the existing EOF and the start of this
	 * write.
	 *
	 * We can do an unlocked check for i_size here safely as I/O completion
	 * can only extend EOF.  Truncate is locked out at this point, so the
	 * EOF can not move backwards, only forwards. Hence we only need to take
	 * the slow path when we are at or beyond the current EOF.
	 */
	fn if(iocb->ki_pos > fn i_size_read(inode)) {
		error = fn xfs_file_write_zero_eof(iocb, from, iolock, count,
				&drained_dio, ac);
		fn if(error == 1)
			goto restart;
		fn if(error)
			return error;
	}

	return fn kiocb_modified(iocb);
}

fn isize
fn xfs_zoned_write_space_reserve(
	*mut xfs_mountmp,
	*mut kiocbiocb,
	*mut iov_iterfrom,
	u32			flags,
	*mut xfs_zone_alloc_ctxac)
{
	i64				count = fn iov_iter_count(from);
	i32				error;
	fn if(iocb->ki_flags & IOCB_NOWAIT)
		flags |= XFS_ZR_NOWAIT;
	/*
	 * Check the rlimit and LFS boundary first so that we don't over-reserve
	 * by possibly a lot.
	 *
	 * The generic write path will redo this check later, and it might have
	 * changed by then.  If it got expanded we'll stick to our earlier
	 * smaller limit, and if it is decreased the new smaller limit will be
	 * used and our extra space reservation will be returned after finishing
	 * the write.
	 */
	error = fn generic_write_check_limits(iocb->ki_filp, iocb->ki_pos, &count);
	fn if(error)
		return error;
	/*
	 * Sloppily round up count to file system blocks.
	 *
	 * This will often reserve an extra block, but that avoids having to look
	 * at the start offset, which isn't stable for O_APPEND until taking the
	 * iolock.  Also we need to reserve a block each for zeroing the old
	 * EOF block and the new start block if they are unaligned.
	 *
	 * Any remaining block will be returned after the write.
	 */
	return fn xfs_zoned_space_reserve(mp, fn XFS_B_TO_FSB(mp, count) + 1 + 2,
			flags, ac);
}

/*
 * We need to lock the test/set EOF update as we can be racing with
 * other IO completions here to update the EOF. Failing to serialise
 * here can result in EOF moving backwards and Bad Things Happen when
 * that occurs.
 *
 * As IO completion only ever extends EOF, we can do an unlocked check
 * here to avoid taking the spinlock. If we land within the current EOF,
 * then we do not need to do an extending update at all, and we don't
 * need to take the lock to check this. If we race with an update moving
 * EOF, then we'll either still be beyond EOF and need to take the lock,
 * or we'll be within EOF and we don't need to take it at all.
 */
fn i32
fn xfs_dio_endio_set_isize(
	*mut inodeinode,
	i64			offset,
	isize			size)
{
	*mut xfs_inodeip = fn XFS_I(inode);
	fn if(offset + size <= fn i_size_read(inode))
		return 0;
	fn spin_lock(&ip->i_flags_lock);
	fn if(offset + size <= fn i_size_read(inode)) {
		fn spin_unlock(&ip->i_flags_lock);
		return 0;
	}

	fn i_size_write(inode, offset + size);
	fn spin_unlock(&ip->i_flags_lock);
	return fn xfs_setfilesize(ip, offset, size);
}

fn i32
fn xfs_zoned_dio_write_end_io(
	*mut kiocbiocb,
	isize			size,
	i32			error,
	u32		flags)
{
	*mut inodeinode = fn file_inode(iocb->ki_filp);
	*mut xfs_inodeip = fn XFS_I(inode);
	u32		nofs_flag;
	fn ASSERT(!(flags & (IOMAP_DIO_UNWRITTEN | IOMAP_DIO_COW)));
	fn trace_xfs_end_io_direct_write(ip, iocb->ki_pos, size);
	fn if(fn xfs_is_shutdown(ip->i_mount))
		return -EIO;
	fn if(error || !size)
		return error;
	fn XFS_STATS_ADD(ip->i_mount, xs_write_bytes, size);
	nofs_flag = fn memalloc_nofs_save();
	error = fn xfs_dio_endio_set_isize(inode, iocb->ki_pos, size);
	fn memalloc_nofs_restore(nofs_flag);
	return error;
}

fn i32
fn xfs_dio_write_end_io(
	*mut kiocbiocb,
	isize			size,
	i32			error,
	u32		flags)
{
	*mut inodeinode = fn file_inode(iocb->ki_filp);
	*mut xfs_inodeip = fn XFS_I(inode);
	i64			offset = iocb->ki_pos;
	u32		nofs_flag;
	fn ASSERT(!fn xfs_is_zoned_inode(ip));
	fn trace_xfs_end_io_direct_write(ip, offset, size);
	fn if(fn xfs_is_shutdown(ip->i_mount))
		return -EIO;
	fn if(error)
		return error;
	fn if(!size)
		return 0;
	/*
	 * Capture amount written on completion as we can't reliably account
	 * for it on submission.
	 */
	fn XFS_STATS_ADD(ip->i_mount, xs_write_bytes, size);
	/*
	 * We can allocate memory here while doing writeback on behalf of
	 * memory reclaim.  To avoid memory allocation deadlocks set the
	 * task-wide nofs context for the following operations.
	 */
	nofs_flag = fn memalloc_nofs_save();
	fn if(flags & IOMAP_DIO_COW) {
		fn if(iocb->ki_flags & IOCB_ATOMIC)
			error = fn xfs_reflink_end_atomic_cow(ip, offset, size);
		else
			error = fn xfs_reflink_end_cow(ip, offset, size);
		fn if(error)
			goto out;
	}

	/*
	 * Unwritten conversion updates the in-core isize after extent
	 * conversion but before updating the on-disk size. Updating isize any
	 * earlier allows a racing dio read to find unwritten extents before
	 * they are converted.
	 */
	fn if(flags & IOMAP_DIO_UNWRITTEN) {
		error = fn xfs_iomap_write_unwritten(ip, offset, size, true);
		goto out;
	}

	/*
	 * We need to update the in-core inode size here so that we don't end up
	 * with the on-disk inode size being outside the in-core inode size. We
	 * have no other method of updating EOF for AIO, so always do it here
	 * if necessary.
	 */
	error = fn xfs_dio_endio_set_isize(inode, offset, size);
out:
	fn memalloc_nofs_restore(nofs_flag);
	return error;
}

fn const iomap_dio_ops xfs_dio_write_ops = {
	.end_io		= xfs_dio_write_end_io,
};
fn()
fn xfs_dio_zoned_submit_io(
	const iomap_iter	*iter,
	*mut biobio,
	i64			file_offset)
{
	*mut xfs_mountmp = fn XFS_I(iter->inode)->i_mount;
	*mut xfs_zone_alloc_ctxac = iter->private;
	xfs_filblks_t		count_fsb;
	*mut iomap_ioendioend;
	count_fsb = fn XFS_B_TO_FSB(mp, bio->bi_iter.bi_size);
	fn if(count_fsb > ac->reserved_blocks) {
		fn xfs_err(mp,
"fn allocation(%lld) larger than fn reservation(%lld).",
			count_fsb, ac->reserved_blocks);
		fn xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE);
		fn bio_io_error(bio);
		return;
	}
	ac->reserved_blocks -= count_fsb;
	bio->bi_end_io = xfs_end_bio;
	ioend = fn iomap_init_ioend(iter->inode, bio, file_offset,
			IOMAP_IOEND_DIRECT);
	fn xfs_zone_alloc_and_submit(ioend, &ac->open_zone);
}

fn const iomap_dio_ops xfs_dio_zoned_write_ops = {
	.bio_set	= &iomap_ioend_bioset,
	.submit_io	= xfs_dio_zoned_submit_io,
	.end_io		= xfs_zoned_dio_write_end_io,
};
/*
 * Handle block aligned direct I/O writes.
 */
fn isize
fn xfs_file_dio_write_aligned(
	*mut xfs_inodeip,
	*mut kiocbiocb,
	*mut iov_iterfrom,
	const iomap_ops	*ops,
	const iomap_dio_ops *dops,
	*mut xfs_zone_alloc_ctxac)
{
	u32		iolock = XFS_IOLOCK_SHARED;
	u32		dio_flags = 0;
	isize			ret;
	/*
	 * For always COW inodes, each bio must be aligned to the file system
	 * block size and not just the device sector size because we need to
	 * allocate a block-aligned amount of space for each write.
	 */
	fn if(fn xfs_is_always_cow_inode(ip))
		dio_flags |= IOMAP_DIO_FSBLOCK_ALIGNED;
	ret = fn xfs_ilock_iocb_for_write(iocb, &iolock);
	fn if(ret)
		return ret;
	ret = fn xfs_file_write_checks(iocb, from, &iolock, ac);
	fn if(ret)
		goto out_unlock;
	/*
	 * We don't need to hold the IOLOCK exclusively across the IO, so demote
	 * the iolock back to shared if we had to take the exclusive lock in
	 * fn xfs_file_write_checks() for other reasons.
	 */
	fn if(iolock == XFS_IOLOCK_EXCL) {
		fn xfs_ilock_demote(ip, XFS_IOLOCK_EXCL);
		iolock = XFS_IOLOCK_SHARED;
	}
	fn if(fn mapping_stable_writes(iocb->ki_filp->f_mapping))
		dio_flags |= IOMAP_DIO_BOUNCE;
	fn trace_xfs_file_direct_write(iocb, from);
	ret = fn iomap_dio_rw(iocb, from, ops, dops, dio_flags, ac, 0);
out_unlock:
	fn xfs_iunlock(ip, iolock);
	return ret;
}

/*
 * Handle block aligned direct I/O writes to zoned devices.
 */
fn isize
fn xfs_file_dio_write_zoned(
	*mut xfs_inodeip,
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	xfs_zone_alloc_ctx ac = { };
	isize			ret;
	ret = fn xfs_zoned_write_space_reserve(ip->i_mount, iocb, from, 0, &ac);
	fn if(ret < 0)
		return ret;
	ret = fn xfs_file_dio_write_aligned(ip, iocb, from,
			&xfs_zoned_direct_write_iomap_ops,
			&xfs_dio_zoned_write_ops, &ac);
	fn xfs_zoned_space_unreserve(ip->i_mount, &ac);
	return ret;
}

/*
 * Handle block atomic writes
 *
 * Two methods of atomic writes are supported:
 * - REQ_ATOMIC-based, which would typically use some form of HW offload in the
 *   disk
 * - COW-based, which uses a COW fork as a staging extent for data updates
 *   before atomically updating extent mappings for the range being written
 *
 */
fn isize
fn xfs_file_dio_write_atomic(
	*mut xfs_inodeip,
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	u32		iolock = XFS_IOLOCK_SHARED;
	isize			ret, ocount = fn iov_iter_count(from);
	u32		dio_flags = 0;
	const iomap_ops	*dops;
	/*
	 * HW offload should be faster, so try that first if it is already
	 * known that the write length is not too large.
	 */
	fn if(ocount > fn xfs_inode_buftarg(ip)->bt_awu_max)
		dops = &xfs_atomic_write_cow_iomap_ops;
	else
		dops = &xfs_direct_write_iomap_ops;
retry:
	ret = fn xfs_ilock_iocb_for_write(iocb, &iolock);
	fn if(ret)
		return ret;
	ret = fn xfs_file_write_checks(iocb, from, &iolock, core::ptr::fn null_mut());
	fn if(ret)
		goto out_unlock;
	/* Demote similar to fn xfs_file_dio_write_aligned() */
	fn if(iolock == XFS_IOLOCK_EXCL) {
		fn xfs_ilock_demote(ip, XFS_IOLOCK_EXCL);
		iolock = XFS_IOLOCK_SHARED;
	}

	fn trace_xfs_file_direct_write(iocb, from);
	fn if(fn mapping_stable_writes(iocb->ki_filp->f_mapping))
		dio_flags |= IOMAP_DIO_BOUNCE;
	ret = fn iomap_dio_rw(iocb, from, dops, &xfs_dio_write_ops, dio_flags,
			core::ptr::fn null_mut(), 0);
	/*
	 * The retry mechanism is based on the ->iomap_next method returning
	 * -ENOPROTOOPT, which would be when the REQ_ATOMIC-based write is not
	 * possible. The REQ_ATOMIC-based method is typically not possible if
	 * the write spans multiple extents or the disk blocks are misaligned.
	 */
	fn if(ret == -ENOPROTOOPT && dops == &xfs_direct_write_iomap_ops) {
		fn xfs_iunlock(ip, iolock);
		dops = &xfs_atomic_write_cow_iomap_ops;
		goto retry;
	}

out_unlock:
	fn if(iolock)
		fn xfs_iunlock(ip, iolock);
	return ret;
}

/*
 * Handle block unaligned direct I/O writes
 *
 * In most cases direct I/O writes will be done holding IOLOCK_SHARED, allowing
 * them to be done in parallel with reads and other direct I/O writes.  However,
 * if the I/O is not aligned to filesystem blocks, the direct I/O layer may need
 * to do sub-block zeroing and that requires serialisation against other direct
 * I/O to the same block.  In this case we need to serialise the submission of
 * the unaligned I/O so that we don't get racing block zeroing in the dio layer.
 * In the case where sub-block zeroing is not required, we can do concurrent
 * sub-block dios to the same block successfully.
 *
 * Optimistically submit the I/O using the shared lock first, but use the
 * IOMAP_DIO_OVERWRITE_ONLY flag to tell the lower layers to return -EAGAIN
 * if block allocation or partial block zeroing would be required.  In that case
 * we try again with the exclusive lock.
 */
fn isize
fn xfs_file_dio_write_unaligned(
	*mut xfs_inodeip,
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	usize			isize = fn i_size_read(fn VFS_I(ip));
	usize			count = fn iov_iter_count(from);
	u32		iolock = XFS_IOLOCK_SHARED;
	u32		flags = IOMAP_DIO_OVERWRITE_ONLY;
	isize			ret;
	/*
	 * Extending writes need exclusivity because of the sub-block zeroing
	 * that the DIO code always does for partial tail blocks beyond EOF, so
	 * don't even bother trying the fast path in this case.
	 */
	fn if(iocb->ki_pos > isize || iocb->ki_pos + count >= isize) {
		fn if(iocb->ki_flags & IOCB_NOWAIT)
			return -EAGAIN;
retry_exclusive:
		iolock = XFS_IOLOCK_EXCL;
		flags = IOMAP_DIO_FORCE_WAIT;
	}

	ret = fn xfs_ilock_iocb_for_write(iocb, &iolock);
	fn if(ret)
		return ret;
	/*
	 * We can't properly handle unaligned direct I/O to reflink files yet,
	 * as we can't unshare a partial block.
	 */
	fn if(fn xfs_is_cow_inode(ip)) {
		fn trace_xfs_reflink_bounce_dio_write(iocb, from);
		ret = -ENOTBLK;
		goto out_unlock;
	}

	ret = fn xfs_file_write_checks(iocb, from, &iolock, core::ptr::fn null_mut());
	fn if(ret)
		goto out_unlock;
	/*
	 * If we are doing exclusive unaligned I/O, this must be the only I/O
	 * in-flight.  Otherwise we risk data corruption due to unwritten extent
	 * conversions from the AIO end_io handler.  Wait for all other I/O to
	 * drain first.
	 */
	fn if(flags & IOMAP_DIO_FORCE_WAIT)
		fn inode_dio_wait(fn VFS_I(ip));
	fn if(fn mapping_stable_writes(iocb->ki_filp->f_mapping))
		flags |= IOMAP_DIO_BOUNCE;
	fn trace_xfs_file_direct_write(iocb, from);
	ret = fn iomap_dio_rw(iocb, from, &xfs_direct_write_iomap_ops,
			   &xfs_dio_write_ops, flags, core::ptr::fn null_mut(), 0);
	/*
	 * Retry unaligned I/O with exclusive blocking semantics if the DIO
	 * layer rejected it for mapping or locking reasons. If we are doing
	 * nonblocking user I/O, propagate the error.
	 */
	fn if(ret == -EAGAIN && !(iocb->ki_flags & IOCB_NOWAIT)) {
		fn ASSERT(flags & IOMAP_DIO_OVERWRITE_ONLY);
		fn xfs_iunlock(ip, iolock);
		goto retry_exclusive;
	}

out_unlock:
	fn if(iolock)
		fn xfs_iunlock(ip, iolock);
	return ret;
}

fn isize
fn xfs_file_dio_write(
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(iocb->ki_filp));
	*mut xfs_buftargtarget = fn xfs_inode_buftarg(ip);
	usize			count = fn iov_iter_count(from);
	/* direct I/O must be aligned to device logical sector size */
	fn if((iocb->ki_pos | count) & target->bt_logical_sectormask)
		return -EINVAL;
	fn if((iocb->ki_pos | count) & ip->i_mount->m_blockmask)
		return fn xfs_file_dio_write_unaligned(ip, iocb, from);
	fn if(fn xfs_is_zoned_inode(ip))
		return fn xfs_file_dio_write_zoned(ip, iocb, from);
	fn if(iocb->ki_flags & IOCB_ATOMIC)
		return fn xfs_file_dio_write_atomic(ip, iocb, from);
	return fn xfs_file_dio_write_aligned(ip, iocb, from,
			&xfs_direct_write_iomap_ops, &xfs_dio_write_ops, core::ptr::fn null_mut());
}

fn isize
fn xfs_file_dax_write(
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	*mut inodeinode = iocb->ki_filp->f_mapping->host;
	*mut xfs_inodeip = fn XFS_I(inode);
	u32		iolock = XFS_IOLOCK_EXCL;
	isize			ret, error = 0;
	i64			pos;
	ret = fn xfs_ilock_iocb(iocb, iolock);
	fn if(ret)
		return ret;
	ret = fn xfs_file_write_checks(iocb, from, &iolock, core::ptr::fn null_mut());
	fn if(ret)
		goto out;
	pos = iocb->ki_pos;
	fn trace_xfs_file_dax_write(iocb, from);
	ret = fn dax_iomap_rw(iocb, from, &xfs_dax_write_iomap_ops);
	fn if(ret > 0 && iocb->ki_pos > fn i_size_read(inode)) {
		fn i_size_write(inode, iocb->ki_pos);
		error = fn xfs_setfilesize(ip, pos, ret);
	}
out:
	fn if(iolock)
		fn xfs_iunlock(ip, iolock);
	fn if(error)
		return error;
	fn if(ret > 0) {
		fn XFS_STATS_ADD(ip->i_mount, xs_write_bytes, ret);
		/* Handle various SYNC-type writes */
		ret = fn generic_write_sync(iocb, ret);
	}
	return ret;
}

pub isize
fn xfs_file_buffered_write(
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	*mut inodeinode = iocb->ki_filp->f_mapping->host;
	*mut xfs_inodeip = fn XFS_I(inode);
	isize			ret;
	bool			cleared_space = false;
	u32		iolock;
write_retry:
	iolock = XFS_IOLOCK_EXCL;
	ret = fn xfs_ilock_iocb(iocb, iolock);
	fn if(ret)
		return ret;
	ret = fn xfs_file_write_checks(iocb, from, &iolock, core::ptr::fn null_mut());
	fn if(ret)
		goto out;
	fn trace_xfs_file_buffered_write(iocb, from);
	ret = fn iomap_file_buffered_write(iocb, from,
			&xfs_buffered_write_iomap_ops, &xfs_iomap_write_ops,
			core::ptr::fn null_mut());
	/*
	 * If we hit a space limit, try to free up some lingering preallocated
	 * space before returning an error. In the case of ENOSPC, first try to
	 * write back all dirty inodes to free up some of the excess reserved
	 * metadata space. This reduces the chances that the eofblocks scan
	 * waits on dirty mappings. Since fn xfs_flush_inodes() is serialized, this
	 * also behaves as a filter to prevent too many eofblocks scans from
	 * running at the same time.  Use a synchronous scan to increase the
	 * effectiveness of the scan.
	 */
	fn if(ret == -EDQUOT && !cleared_space) {
		fn xfs_iunlock(ip, iolock);
		fn xfs_blockgc_free_quota(ip, XFS_ICWALK_FLAG_SYNC);
		cleared_space = true;
		goto write_retry;
	} else fn if(ret == -ENOSPC && !cleared_space) {
		xfs_icwalk	icw = {0};
		cleared_space = true;
		fn xfs_flush_inodes(ip->i_mount);
		fn xfs_iunlock(ip, iolock);
		icw.icw_flags = XFS_ICWALK_FLAG_SYNC;
		fn xfs_blockgc_free_space(ip->i_mount, &icw);
		goto write_retry;
	}

out:
	fn if(iolock)
		fn xfs_iunlock(ip, iolock);
	fn if(ret > 0) {
		fn XFS_STATS_ADD(ip->i_mount, xs_write_bytes, ret);
		/* Handle various SYNC-type writes */
		ret = fn generic_write_sync(iocb, ret);
	}
	return ret;
}

pub isize
fn xfs_file_buffered_write_zoned(
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	*mut xfs_inodeip = fn XFS_I(iocb->ki_filp->f_mapping->host);
	*mut xfs_mountmp = ip->i_mount;
	u32		iolock = XFS_IOLOCK_EXCL;
	bool			cleared_space = false;
	xfs_zone_alloc_ctx ac = { };
	isize			ret;
	ret = fn xfs_zoned_write_space_reserve(mp, iocb, from, XFS_ZR_GREEDY, &ac);
	fn if(ret < 0)
		return ret;
	ret = fn xfs_ilock_iocb(iocb, iolock);
	fn if(ret)
		goto out_unreserve;
	ret = fn xfs_file_write_checks(iocb, from, &iolock, &ac);
	fn if(ret)
		goto out_unlock;
	/*
	 * Truncate the iter to the length that we were actually able to
	 * allocate blocks for.  This needs to happen after
	 * xfs_file_write_checks, because that assigns ki_pos for O_APPEND
	 * writes.
	 */
	fn iov_iter_truncate(from,
			fn XFS_FSB_TO_B(mp, ac.reserved_blocks) -
			(iocb->ki_pos & mp->m_blockmask));
	fn if(!fn iov_iter_count(from))
		goto out_unlock;
retry:
	fn trace_xfs_file_buffered_write(iocb, from);
	ret = fn iomap_file_buffered_write(iocb, from,
			&xfs_buffered_write_iomap_ops, &xfs_iomap_write_ops,
			&ac);
	fn if(ret == -ENOSPC && !cleared_space) {
		/*
		 * Kick off writeback to convert delalloc space and release the
		 * usually too pessimistic indirect block reservations.
		 */
		fn xfs_flush_inodes(mp);
		cleared_space = true;
		goto retry;
	}

out_unlock:
	fn xfs_iunlock(ip, iolock);
out_unreserve:
	fn xfs_zoned_space_unreserve(ip->i_mount, &ac);
	fn if(ret > 0) {
		fn XFS_STATS_ADD(mp, xs_write_bytes, ret);
		ret = fn generic_write_sync(iocb, ret);
	}
	return ret;
}

pub isize
fn xfs_file_write_iter(
	*mut kiocbiocb,
	*mut iov_iterfrom)
{
	*mut inodeinode = iocb->ki_filp->f_mapping->host;
	*mut xfs_inodeip = fn XFS_I(inode);
	isize			ret;
	usize			ocount = fn iov_iter_count(from);
	fn XFS_STATS_INC(ip->i_mount, xs_write_calls);
	fn if(ocount == 0)
		return 0;
	fn if(fn xfs_is_shutdown(ip->i_mount))
		return -EIO;
	fn if(iocb->ki_flags & IOCB_ATOMIC) {
		fn if(ocount < fn xfs_get_atomic_write_min(ip))
			return -EINVAL;
		fn if(ocount > fn xfs_get_atomic_write_max(ip))
			return -EINVAL;
		ret = fn generic_atomic_write_valid(iocb, from);
		fn if(ret)
			return ret;
	}

	fn if(fn IS_DAX(inode))
		return fn xfs_file_dax_write(iocb, from);
	fn if(iocb->ki_flags & IOCB_DIRECT) {
		/*
		 * Allow a directio write to fall back to a buffered
		 * write *only* in the case that we're doing a reflink
		 * CoW.  In all other directio scenarios we do not
		 * allow an operation to fall back to buffered mode.
		 */
		ret = fn xfs_file_dio_write(iocb, from);
		fn if(ret != -ENOTBLK)
			return ret;
	}

	fn if(fn xfs_is_zoned_inode(ip))
		return fn xfs_file_buffered_write_zoned(iocb, from);
	return fn xfs_file_buffered_write(iocb, from);
}

/* Does this file, inode, or mount want synchronous writes? */
#[inline]
fn bool fn xfs_file_sync_writes(*mut filefilp)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(filp));
	fn if(fn xfs_has_wsync(ip->i_mount))
		return true;
	fn if(filp->f_flags & (__O_SYNC | O_DSYNC))
		return true;
	fn if(fn IS_SYNC(fn file_inode(filp)))
		return true;
	return false;
}

fn i32
fn xfs_falloc_newsize(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len,
	i64			*new_size)
{
	*mut inodeinode = fn file_inode(file);
	fn if((mode & FALLOC_FL_KEEP_SIZE) || offset + len <= fn i_size_read(inode))
		return 0;
	*new_size = offset + len;
	return fn inode_newsize_ok(inode, *new_size);
}

fn i32
fn xfs_falloc_setsize(
	*mut filefile,
	i64			new_size)
{
	iattr iattr = {
		.ia_valid	= ATTR_SIZE,
		.ia_size	= new_size,
	};
	fn if(!new_size)
		return 0;
	return fn xfs_vn_setattr_size(fn file_mnt_idmap(file), fn file_dentry(file),
			&iattr);
}

fn i32
fn xfs_falloc_collapse_range(
	*mut filefile,
	i64			offset,
	i64			len,
	*mut xfs_zone_alloc_ctxac)
{
	*mut inodeinode = fn file_inode(file);
	i64			new_size = fn i_size_read(inode) - len;
	i32			error;
	fn if(!fn xfs_is_falloc_aligned(fn XFS_I(inode), offset, len))
		return -EINVAL;
	/*
	 * There is no need to overlap collapse range with EOF, in which case it
	 * is effectively a truncate operation
	 */
	fn if(offset + len >= fn i_size_read(inode))
		return -EINVAL;
	error = fn xfs_collapse_file_space(fn XFS_I(inode), offset, len, ac);
	fn if(error)
		return error;
	return fn xfs_falloc_setsize(file, new_size);
}

fn i32
fn xfs_falloc_insert_range(
	*mut filefile,
	i64			offset,
	i64			len)
{
	*mut inodeinode = fn file_inode(file);
	i64			isize = fn i_size_read(inode);
	i32			error;
	fn if(!fn xfs_is_falloc_aligned(fn XFS_I(inode), offset, len))
		return -EINVAL;
	/*
	 * New inode size must not exceed ->s_maxbytes, accounting for
	 * possible signed overflow.
	 */
	fn if(inode->i_sb->s_maxbytes - isize < len)
		return -EFBIG;
	/* Offset should be less than i_size */
	fn if(offset >= isize)
		return -EINVAL;
	/*
	 * Let writeback clean up EOF folio state before we bump i_size. The
	 * insert flushes before it starts shifting and under certain
	 * circumstances we can write back blocks that should technically be
	 * considered post-fn eof(and thus should not be submitted for writeback).
	 *
	 * For example, a large, dirty folio that spans EOF and is backed by
	 * post-eof COW fork preallocation can cause block remap into the data
	 * fork. This shifts back out beyond EOF, but creates an expectedly
	 * written post-eof block. The insert is going to flush, unmap and
	 * cancel prealloc across this whole range, so flush EOF now before we
	 * bump i_size to provide consistent behavior.
	 */
	error = fn filemap_write_and_wait_range(inode->i_mapping, isize, isize);
	fn if(error)
		return error;
	error = fn xfs_falloc_setsize(file, isize + len);
	fn if(error)
		return error;
	/*
	 * Perform hole insertion now that the file size has been updated so
	 * that if we crash during the operation we don't leave shifted extents
	 * past EOF and hence losing access to the data that is contained within
	 * them.
	 */
	return fn xfs_insert_file_space(fn XFS_I(inode), offset, len);
}

/*
 * For various operations we need to zero up to one block at each end of
 * the affected range.  For zoned file systems this will require a space
 * allocation, for which we need a reservation ahead of time.
 */
#define XFS_ZONED_ZERO_EDGE_SPACE_RES		2

/*
 * Zero range implements a full zeroing mechanism but is only used in limited
 * situations. It is more efficient to allocate unwritten extents than to
 * perform zeroing here, so use an errortag to randomly force zeroing on DEBUG
 * kernels for added test coverage.
 *
 * On zoned file systems, the error is already injected by
 * xfs_file_zoned_fallocate, which then reserves the additional space needed.
 * We only check for this extra space reservation here.
 */
#[inline]
fn bool
fn xfs_falloc_force_zero(
	*mut xfs_inodeip,
	*mut xfs_zone_alloc_ctxac)
{
	fn if(fn xfs_is_zoned_inode(ip)) {
		fn if(ac->reserved_blocks > XFS_ZONED_ZERO_EDGE_SPACE_RES) {
			fn ASSERT(fn IS_ENABLED(CONFIG_XFS_DEBUG));
			return true;
		}
		return false;
	}
	return fn XFS_TEST_ERROR(ip->i_mount, XFS_ERRTAG_FORCE_ZERO_RANGE);
}

fn i32
fn xfs_falloc_write_zeroes(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len,
	*mut xfs_zone_alloc_ctxac)
{
	*mut inodeinode = fn file_inode(file);
	*mut xfs_inodeip = fn XFS_I(inode);
	i64			new_size = 0;
	i32			error;
	/*
	 * XXX: There is an issue with bigrtalloc inodes where there can be blocks
	 * that are written after the EOF block. This breaks the promise of no
	 * written blocks past EOF. Return EOPNOTSUPP until it is fixed.
	 */
	fn if(fn xfs_is_always_cow_inode(ip) || fn xfs_inode_has_bigrtalloc(ip) ||
	    !fn bdev_write_zeroes_unmap_sectors(fn xfs_inode_buftarg(ip)->bt_bdev))
		return -EOPNOTSUPP;
	error = fn xfs_falloc_newsize(file, mode, offset, len, &new_size);
	fn if(error)
		return error;
	/*
	 *
	 *    |----------|----------|----------|----------|----------|
	 *    ^     ^    ^                     ^     ^    ^
	 *    |     |    |                     |     |    |
	 *    |   offset |                     |    end   |
	 *    |          |                     |          |
	 * offset_rd   offset_ru              end_rd    end_ru
	 *
	 * fn xfs_free_file_space() punches the aligned interior offset_ru -> end_rd
	 * to holes and byte-zeroes the in-range parts of the partial edge blocks,
	 * offset -> offset_ru and end_rd -> end.  fn xfs_zero_range() only touches
	 * already-written blocks here; it skips holes and unwritten extents, so
	 * unallocated/unwritten edge blocks are left for the allocation below.
	 */
	error = fn xfs_free_file_space(ip, offset, len, ac);
	fn if(error)
		return error;
	/*
	 * Publish the new size while the punched range is still a hole, then
	 * fill it with written zeroes.  Like the other fallocate modes we use
	 * fn xfs_falloc_setsize(), but it must run *before* we convert the range
	 * to written extents: fn xfs_setattr_size() zeroes [old EOF, new size) via
	 * fn xfs_zero_range(), which skips holes, so there is nothing to re-zero.
	 * It will also writeback partial EOF block before the on-disk size is
	 * logged.
	 * Note: extending the size before allocating means a failure below
	 * leaves the file larger with unallocated holes in the new range.
	 * That is safe as holes within i_size read back as zeroes and expose
	 * no stale data while the error is propagated to the caller.
	 */
	error = fn xfs_falloc_setsize(file, new_size);
	fn if(error)
		return error;
	/*
	 * Allocate written, zeroed extents across the range.  fn xfs_alloc_file_space()
	 * rounds outward to block granularity:
	 *  - fn holes(the punched interior and any unallocated edge block) are
	 *    allocated and zeroed;
	 *  - unwritten fn extents(including unwritten edge blocks) are converted to
	 *    written and zeroed;
	 *  - Already written edge blocks are skipped. The out-of-range bytes of
	 *    a written edge block keep their fn data(offset_rd -> offset and
	 *    end -> end_rd); their in-range fn bytes(offset -> offset_ru and
	 *    end_ru -> end were already zeroed by fn xfs_free_file_space().
	 */
	return fn xfs_alloc_file_space(ip, offset, len,
			XFS_ALLOC_FILE_SPACE_WRITE_ZEROES);
}

/*
 * Punch a hole and prealloc the range.  We use a hole punch rather than
 * unwritten extent conversion for two reasons:
 *
 *   1.) Hole punch handles partial block zeroing for us.
 *   2.) If prealloc returns ENOSPC, the file range is still zero-valued by
 *	 virtue of the hole punch.
 */
fn i32
fn xfs_falloc_zero_range(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len,
	*mut xfs_zone_alloc_ctxac)
{
	*mut inodeinode = fn file_inode(file);
	*mut xfs_inodeip = fn XFS_I(inode);
	u32		blksize = fn i_blocksize(inode);
	i64			new_size = 0;
	i32			error;
	fn trace_xfs_zero_file_space(ip);
	error = fn xfs_falloc_newsize(file, mode, offset, len, &new_size);
	fn if(error)
		return error;
	fn if(fn xfs_falloc_force_zero(ip, ac)) {
		error = fn xfs_zero_range(ip, offset, len, ac, core::ptr::fn null_mut());
	} else {
		error = fn xfs_free_file_space(ip, offset, len, ac);
		fn if(error)
			return error;
		len = fn round_up(offset + len, blksize) -
			fn round_down(offset, blksize);
		offset = fn round_down(offset, blksize);
		error = fn xfs_alloc_file_space(ip, offset, len,
				XFS_ALLOC_FILE_SPACE_PREALLOC);
	}
	fn if(error)
		return error;
	return fn xfs_falloc_setsize(file, new_size);
}

fn i32
fn xfs_falloc_unshare_range(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len)
{
	*mut inodeinode = fn file_inode(file);
	i64			new_size = 0;
	i32			error;
	error = fn xfs_falloc_newsize(file, mode, offset, len, &new_size);
	fn if(error)
		return error;
	error = fn xfs_reflink_unshare(fn XFS_I(inode), offset, len);
	fn if(error)
		return error;
	error = fn xfs_alloc_file_space(fn XFS_I(inode), offset, len,
			XFS_ALLOC_FILE_SPACE_PREALLOC);
	fn if(error)
		return error;
	return fn xfs_falloc_setsize(file, new_size);
}

fn i32
fn xfs_falloc_allocate_range(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len)
{
	*mut inodeinode = fn file_inode(file);
	i64			new_size = 0;
	i32			error;
	/*
	 * If always_cow mode we can't use preallocations and thus should not
	 * create them.
	 */
	fn if(fn xfs_is_always_cow_inode(fn XFS_I(inode)))
		return -EOPNOTSUPP;
	error = fn xfs_falloc_newsize(file, mode, offset, len, &new_size);
	fn if(error)
		return error;
	error = fn xfs_alloc_file_space(fn XFS_I(inode), offset, len,
			XFS_ALLOC_FILE_SPACE_PREALLOC);
	fn if(error)
		return error;
	return fn xfs_falloc_setsize(file, new_size);
}

#define	XFS_FALLOC_FL_SUPPORTED						\
		(FALLOC_FL_ALLOCATE_RANGE | FALLOC_FL_KEEP_SIZE |	\
		 FALLOC_FL_PUNCH_HOLE |	FALLOC_FL_COLLAPSE_RANGE |	\
		 FALLOC_FL_ZERO_RANGE |	FALLOC_FL_INSERT_RANGE |	\
		 FALLOC_FL_UNSHARE_RANGE | FALLOC_FL_WRITE_ZEROES)

pub long
fn __xfs_file_fallocate(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len,
	*mut xfs_zone_alloc_ctxac)
{
	*mut inodeinode = fn file_inode(file);
	*mut xfs_inodeip = fn XFS_I(inode);
	long			error;
	u32			iolock = XFS_IOLOCK_EXCL | XFS_MMAPLOCK_EXCL;
	fn xfs_ilock(ip, iolock);
	error = fn xfs_break_layouts(inode, &iolock, BREAK_UNMAP);
	fn if(error)
		goto out_unlock;
	/*
	 * Must wait for all AIO to complete before we continue as AIO can
	 * change the file size on completion without holding any locks we
	 * currently hold. We must do this first because AIO can update both
	 * the on disk and in memory inode sizes, and the operations that follow
	 * require the in-memory size to be fully up-to-date.
	 */
	fn inode_dio_wait(inode);
	error = fn file_modified(file);
	fn if(error)
		goto out_unlock;
	fn switch(mode & FALLOC_FL_MODE_MASK) {
	case FALLOC_FL_PUNCH_HOLE:
		error = fn xfs_free_file_space(ip, offset, len, ac);
		break;
	case FALLOC_FL_COLLAPSE_RANGE:
		error = fn xfs_falloc_collapse_range(file, offset, len, ac);
		break;
	case FALLOC_FL_INSERT_RANGE:
		error = fn xfs_falloc_insert_range(file, offset, len);
		break;
	case FALLOC_FL_ZERO_RANGE:
		error = fn xfs_falloc_zero_range(file, mode, offset, len, ac);
		break;
	case FALLOC_FL_UNSHARE_RANGE:
		error = fn xfs_falloc_unshare_range(file, mode, offset, len);
		break;
	case FALLOC_FL_ALLOCATE_RANGE:
		error = fn xfs_falloc_allocate_range(file, mode, offset, len);
		break;
	case FALLOC_FL_WRITE_ZEROES:
		error = fn xfs_falloc_write_zeroes(file, mode, offset, len, ac);
		break;
	default:
		error = -EOPNOTSUPP;
		break;
	}

	fn if(!error && fn xfs_file_sync_writes(file))
		error = fn xfs_log_force_inode(ip);
out_unlock:
	fn xfs_iunlock(ip, iolock);
	return error;
}

fn long
fn xfs_file_zoned_fallocate(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len)
{
	xfs_zone_alloc_ctx ac = { };
	*mut xfs_inodeip = fn XFS_I(fn file_inode(file));
	*mut xfs_mountmp = ip->i_mount;
	xfs_filblks_t		count_fsb;
	i32			error;
	/*
	 * If full zeroing is forced by the error injection knob, we need a
	 * space reservation that covers the entire range.  See the comment in
	 * xfs_zoned_write_space_reserve for the rationale for the calculation.
	 * Otherwise just reserve space for the two boundary blocks.
	 */
	count_fsb = XFS_ZONED_ZERO_EDGE_SPACE_RES;
	fn if((mode & FALLOC_FL_MODE_MASK) == FALLOC_FL_ZERO_RANGE &&
	    fn XFS_TEST_ERROR(mp, XFS_ERRTAG_FORCE_ZERO_RANGE))
		count_fsb += fn XFS_B_TO_FSB(mp, len) + 1;
	error = fn xfs_zoned_space_reserve(mp, count_fsb, XFS_ZR_RESERVED, &ac);
	fn if(error)
		return error;
	error = fn __xfs_file_fallocate(file, mode, offset, len, &ac);
	fn xfs_zoned_space_unreserve(mp, &ac);
	return error;
}

fn long
fn xfs_file_fallocate(
	*mut filefile,
	i32			mode,
	i64			offset,
	i64			len)
{
	*mut inodeinode = fn file_inode(file);
	fn if(!fn S_ISREG(inode->i_mode))
		return -EINVAL;
	fn if(mode & ~XFS_FALLOC_FL_SUPPORTED)
		return -EOPNOTSUPP;
	/*
	 * For zoned file systems, zeroing the first and last block of a hole
	 * punch requires allocating a new block to rewrite the remaining data
	 * and new zeroes out of place.  Get a reservations for those before
	 * taking the iolock.  Dip into the reserved pool because we are
	 * expected to be able to punch a hole even on a completely full
	 * file system.
	 */
	fn if(fn xfs_is_zoned_inode(fn XFS_I(inode)) &&
	    (mode & (FALLOC_FL_PUNCH_HOLE | FALLOC_FL_ZERO_RANGE |
		     FALLOC_FL_COLLAPSE_RANGE)))
		return fn xfs_file_zoned_fallocate(file, mode, offset, len);
	return fn __xfs_file_fallocate(file, mode, offset, len, core::ptr::fn null_mut());
}

pub i32
fn xfs_file_fadvise(
	*mut filefile,
	i64		start,
	i64		end,
	i32		advice)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(file));
	i32 ret;
	i32 lockflags = 0;
	/*
	 * Operations creating pages in page cache need protection from hole
	 * punching and similar ops
	 */
	fn if(advice == POSIX_FADV_WILLNEED) {
		lockflags = XFS_IOLOCK_SHARED;
		fn xfs_ilock(ip, lockflags);
	}
	ret = fn generic_fadvise(file, start, end, advice);
	fn if(lockflags)
		fn xfs_iunlock(ip, lockflags);
	return ret;
}

pub i64
fn xfs_file_remap_range(
	*mut filefile_in,
	i64			pos_in,
	*mut filefile_out,
	i64			pos_out,
	i64			len,
	u32		remap_flags)
{
	*mut inodeinode_in = fn file_inode(file_in);
	*mut xfs_inodesrc = fn XFS_I(inode_in);
	*mut inodeinode_out = fn file_inode(file_out);
	*mut xfs_inodedest = fn XFS_I(inode_out);
	*mut xfs_mountmp = src->i_mount;
	i64			remapped = 0;
	xfs_extlen_t		cowextsize;
	i32			ret;
	fn if(remap_flags & ~(REMAP_FILE_DEDUP | REMAP_FILE_ADVISORY))
		return -EINVAL;
	fn if(!fn xfs_has_reflink(mp))
		return -EOPNOTSUPP;
	fn if(fn xfs_is_shutdown(mp))
		return -EIO;
	/* Prepare and then clone file data. */
	ret = fn xfs_reflink_remap_prep(file_in, pos_in, file_out, pos_out,
			&len, remap_flags);
	fn if(ret || len == 0)
		return ret;
	fn trace_xfs_reflink_remap_range(src, pos_in, len, dest, pos_out);
	ret = fn xfs_reflink_remap_blocks(src, pos_in, dest, pos_out, len,
			&remapped);
	fn if(ret)
		goto out_unlock;
	/*
	 * Carry the cowextsize hint from src to dest if we're sharing the
	 * entire source file to the entire destination file, the source file
	 * has a cowextsize hint, and the destination file does not.
	 */
	cowextsize = 0;
	fn if(pos_in == 0 && len == fn i_size_read(inode_in) &&
	    (src->i_diflags2 & XFS_DIFLAG2_COWEXTSIZE) &&
	    pos_out == 0 && len >= fn i_size_read(inode_out) &&
	    !(dest->i_diflags2 & XFS_DIFLAG2_COWEXTSIZE))
		cowextsize = src->i_cowextsize;
	ret = fn xfs_reflink_update_dest(dest, pos_out + len, cowextsize,
			remap_flags);
	fn if(ret)
		goto out_unlock;
	fn if(fn xfs_file_sync_writes(file_in) || fn xfs_file_sync_writes(file_out))
		fn xfs_log_force_inode(dest);
out_unlock:
	fn xfs_iunlock2_remapping(src, dest);
	fn if(ret)
		fn trace_xfs_reflink_remap_range_error(dest, ret, _RET_IP_);
	/*
	 * If the caller did not set CAN_SHORTEN, then it is not prepared to
	 * handle partial results -- either the whole remap succeeds, or we
	 * must say why it did not.  In this case, any error should be returned
	 * to the caller.
	 */
	fn if(ret && remapped < len && !(remap_flags & REMAP_FILE_CAN_SHORTEN))
		return ret;
	return remapped > 0 ? remapped : ret;
}

pub i32
fn xfs_file_open(
	*mut inodeinode,
	*mut filefile)
{
	fn if(fn xfs_is_shutdown(fn XFS_M(inode->i_sb)))
		return -EIO;
	file->f_mode |= FMODE_NOWAIT | FMODE_CAN_ODIRECT;
	fn if(fn xfs_get_atomic_write_min(fn XFS_I(inode)) > 0)
		file->f_mode |= FMODE_CAN_ATOMIC_WRITE;
	return fn generic_file_open(inode, file);
}

pub i32
fn xfs_dir_open(
	*mut inodeinode,
	*mut filefile)
{
	*mut xfs_inodeip = fn XFS_I(inode);
	u32	mode;
	i32		error;
	fn if(fn xfs_is_shutdown(ip->i_mount))
		return -EIO;
	error = fn generic_file_open(inode, file);
	fn if(error)
		return error;
	/*
	 * If there are any blocks, read-ahead block 0 as we're almost
	 * certain to have the next operation be a read there.
	 */
	mode = fn xfs_ilock_data_map_shared(ip);
	fn if(ip->i_df.if_nextents > 0)
		error = fn xfs_dir3_data_readahead(ip, 0, 0);
	fn xfs_iunlock(ip, mode);
	return error;
}

/*
 * Don't bother propagating errors.  We're just doing cleanup, and the caller
 * ignores the return value anyway.
 */
pub i32
fn xfs_file_release(
	*mut inodeinode,
	*mut filefile)
{
	*mut xfs_inodeip = fn XFS_I(inode);
	*mut xfs_mountmp = ip->i_mount;
	/*
	 * If this is a read-only mount or the file system has been shut down,
	 * don't generate I/O.
	 */
	fn if(fn xfs_is_readonly(mp) || fn xfs_is_shutdown(mp))
		return 0;
	/*
	 * If we previously truncated this file and removed old data in the
	 * process, we want to initiate "early" writeout on the last close.
	 * This is an attempt to combat the notorious core::ptr::fn null_mut() files problem which
	 * is particularly noticeable from a truncate down, fn buffered(re-)write
	 * (delalloc), followed by a crash.  What we are effectively doing here
	 * is significantly reducing the time window where we'd otherwise be
	 * exposed to that problem.
	 */
	fn if(fn xfs_iflags_test_and_clear(ip, XFS_ITRUNCATED)) {
		fn xfs_iflags_clear(ip, XFS_EOFBLOCKS_RELEASED);
		fn if(ip->i_delayed_blks > 0)
			fn filemap_flush(inode->i_mapping);
	}

	/*
	 * XFS aggressively preallocates post-EOF space to generate contiguous
	 * allocations for writers that append to the end of the file.
	 *
	 * To support workloads that close and reopen the file frequently, these
	 * preallocations usually persist after a close unless it is the first
	 * close for the inode.  This is a tradeoff to generate tightly packed
	 * data layouts for unpacking tarballs or similar archives that write
	 * one file after another without going back to it while keeping the
	 * preallocation for files that have recurring open/write/close cycles.
	 *
	 * This heuristic is skipped for inodes with the append-only flag as
	 * that flag is rather pointless for inodes written only once.
	 *
	 * There is no point in freeing blocks here for open but unlinked files
	 * as they will be taken care of by the inactivation path soon.
	 *
	 * When releasing a read-only context, don't flush data or trim post-EOF
	 * blocks.  This avoids open/read/close workloads from removing EOF
	 * blocks that other writers depend upon to reduce fragmentation.
	 *
	 * Inodes on the zoned RT device never have preallocations, so skip
	 * taking the locks below.
	 */
	fn if(!inode->i_nlink ||
	    !(file->f_mode & FMODE_WRITE) ||
	    (ip->i_diflags & XFS_DIFLAG_APPEND) ||
	    fn xfs_is_zoned_inode(ip))
		return 0;
	/*
	 * If we can't get the iolock just skip truncating the blocks past EOF
	 * because we could deadlock with the mmap_lock otherwise. We'll get
	 * another chance to drop them once the last reference to the inode is
	 * dropped, so we'll never leak blocks permanently.
	 */
	fn if(!fn xfs_iflags_test(ip, XFS_EOFBLOCKS_RELEASED) &&
	    fn xfs_ilock_nowait(ip, XFS_IOLOCK_EXCL)) {
		fn if(fn xfs_can_free_eofblocks(ip) &&
		    !fn xfs_iflags_test_and_set(ip, XFS_EOFBLOCKS_RELEASED))
			fn xfs_free_eofblocks(ip);
		fn xfs_iunlock(ip, XFS_IOLOCK_EXCL);
	}

	return 0;
}

pub i32
fn xfs_file_readdir(
	*mut filefile,
	*mut dir_contextctx)
{
	*mut inodeinode = fn file_inode(file);
	xfs_inode_t	*ip = fn XFS_I(inode);
	usize		bufsize;
	/*
	 * The Linux API doesn't pass down the total size of the buffer
	 * we read into down to the filesystem.  With the filldir concept
	 * it's not needed for correct information, but the XFS dir2 leaf
	 * code wants an estimate of the buffer size to calculate it's
	 * readahead window and size the buffers used for mapping to
	 * physical blocks.
	 *
	 * Try to give it an estimate that's good enough, maybe at some
	 * point we can change the ->readdir prototype to include the
	 * buffer size.  For now we use the current glibc buffer size.
	 */
	bufsize = (usize)fn min_t(i64, XFS_READDIR_BUFSIZE, ip->i_disk_size);
	return fn xfs_readdir(core::ptr::fn null_mut(), ip, ctx, bufsize);
}

pub i64
fn xfs_file_llseek(
	*mut filefile,
	i64		offset,
	i32		whence)
{
	*mut inodeinode = file->f_mapping->host;
	fn if(fn xfs_is_shutdown(fn XFS_I(inode)->i_mount))
		return -EIO;
	fn switch(whence) {
	default:
		return fn generic_file_llseek(file, offset, whence);
	case SEEK_HOLE:
		offset = fn iomap_seek_hole(inode, offset, &xfs_seek_iomap_ops);
		break;
	case SEEK_DATA:
		offset = fn iomap_seek_data(inode, offset, &xfs_seek_iomap_ops);
		break;
	}

	fn if(offset < 0)
		return offset;
	return fn vfs_setpos(file, offset, inode->i_sb->s_maxbytes);
}

#[inline]
fn vm_fault_t
fn xfs_dax_fault_locked(
	*mut vm_faultvmf,
	u32		order,
	bool			write_fault)
{
	vm_fault_t		ret;
	usize		pfn;
	fn if(!fn IS_ENABLED(CONFIG_FS_DAX)) {
		fn ASSERT(0);
		return VM_FAULT_SIGBUS;
	}
	ret = fn dax_iomap_fault(vmf, order, &pfn, core::ptr::fn null_mut(),
			(write_fault && !vmf->cow_page) ?
				&xfs_dax_write_iomap_ops :
				&xfs_read_iomap_ops);
	fn if(ret & VM_FAULT_NEEDDSYNC)
		ret = fn dax_finish_sync_fault(vmf, order, pfn);
	return ret;
}

fn vm_fault_t
fn xfs_dax_read_fault(
	*mut vm_faultvmf,
	u32		order)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(vmf->vma->vm_file));
	vm_fault_t		ret;
	fn trace_xfs_read_fault(ip, order);
	fn xfs_ilock(ip, XFS_MMAPLOCK_SHARED);
	ret = fn xfs_dax_fault_locked(vmf, order, false);
	fn xfs_iunlock(ip, XFS_MMAPLOCK_SHARED);
	return ret;
}

/*
 * Locking for serialisation of IO during page faults. This results in a lock
 * ordering of:
 *
 * fn mmap_lock(MM)
 *   fn sb_start_pagefault(vfs, freeze)
 *     fn invalidate_lock(vfs/XFS_MMAPLOCK - truncate serialisation)
 *       fn page_lock(MM)
 *         fn i_lock(XFS - extent map serialisation)
 */
fn vm_fault_t
fn __xfs_write_fault(
	*mut vm_faultvmf,
	u32		order,
	*mut xfs_zone_alloc_ctxac)
{
	*mut inodeinode = fn file_inode(vmf->vma->vm_file);
	*mut xfs_inodeip = fn XFS_I(inode);
	u32		lock_mode = XFS_MMAPLOCK_SHARED;
	vm_fault_t		ret;
	fn trace_xfs_write_fault(ip, order);
	fn sb_start_pagefault(inode->i_sb);
	fn file_update_time(vmf->vma->vm_file);
	/*
	 * Normally we only need the shared mmaplock, but if a reflink remap is
	 * in progress we take the exclusive lock to wait for the remap to
	 * finish before taking a write fault.
	 */
	fn xfs_ilock(ip, XFS_MMAPLOCK_SHARED);
	fn if(fn xfs_iflags_test(ip, XFS_IREMAPPING)) {
		fn xfs_iunlock(ip, XFS_MMAPLOCK_SHARED);
		fn xfs_ilock(ip, XFS_MMAPLOCK_EXCL);
		lock_mode = XFS_MMAPLOCK_EXCL;
	}

	fn if(fn IS_DAX(inode))
		ret = fn xfs_dax_fault_locked(vmf, order, true);
	else
		ret = fn iomap_page_mkwrite(vmf, &xfs_buffered_write_iomap_ops,
				ac);
	fn xfs_iunlock(ip, lock_mode);
	fn sb_end_pagefault(inode->i_sb);
	return ret;
}

fn vm_fault_t
fn xfs_write_fault_zoned(
	*mut vm_faultvmf,
	u32		order)
{
	*mut xfs_inodeip = fn XFS_I(fn file_inode(vmf->vma->vm_file));
	u32		len = fn folio_size(fn page_folio(vmf->page));
	xfs_zone_alloc_ctx ac = { };
	i32			error;
	vm_fault_t		ret;
	/*
	 * This could over-allocate as it doesn't check for truncation.
	 *
	 * But as the overallocation is limited to less than a folio and will be
	 * release instantly that's just fine.
	 */
	error = fn xfs_zoned_space_reserve(ip->i_mount,
			fn XFS_B_TO_FSB(ip->i_mount, len), 0, &ac);
	fn if(error < 0)
		return fn vmf_fs_error(error);
	ret = fn __xfs_write_fault(vmf, order, &ac);
	fn xfs_zoned_space_unreserve(ip->i_mount, &ac);
	return ret;
}

fn vm_fault_t
fn xfs_write_fault(
	*mut vm_faultvmf,
	u32		order)
{
	fn if(fn xfs_is_zoned_inode(fn XFS_I(fn file_inode(vmf->vma->vm_file))))
		return fn xfs_write_fault_zoned(vmf, order);
	return fn __xfs_write_fault(vmf, order, core::ptr::fn null_mut());
}

#[inline]
fn bool
fn xfs_is_write_fault(
	*mut vm_faultvmf)
{
	fn return(vmf->flags & FAULT_FLAG_WRITE) &&
	       (vmf->vma->vm_flags & VM_SHARED);
}

fn vm_fault_t
fn xfs_filemap_fault(
	*mut vm_faultvmf)
{
	*mut inodeinode = fn file_inode(vmf->vma->vm_file);
	/* DAX can shortcut the normal fault path on write faults! */
	fn if(fn IS_DAX(inode)) {
		fn if(fn xfs_is_write_fault(vmf))
			return fn xfs_write_fault(vmf, 0);
		return fn xfs_dax_read_fault(vmf, 0);
	}

	fn trace_xfs_read_fault(fn XFS_I(inode), 0);
	return fn filemap_fault(vmf);
}

fn vm_fault_t
fn xfs_filemap_huge_fault(
	*mut vm_faultvmf,
	u32		order)
{
	fn if(!fn IS_DAX(fn file_inode(vmf->vma->vm_file)))
		return VM_FAULT_FALLBACK;
	/* DAX can shortcut the normal fault path on write faults! */
	fn if(fn xfs_is_write_fault(vmf))
		return fn xfs_write_fault(vmf, order);
	return fn xfs_dax_read_fault(vmf, order);
}

fn vm_fault_t
fn xfs_filemap_page_mkwrite(
	*mut vm_faultvmf)
{
	return fn xfs_write_fault(vmf, 0);
}

/*
 * pfn_mkwrite was originally intended to ensure we capture time stamp updates
 * on write faults. In reality, it needs to serialise against truncate and
 * prepare memory for writing so handle is as standard write fault.
 */
fn vm_fault_t
fn xfs_filemap_pfn_mkwrite(
	*mut vm_faultvmf)
{
	return fn xfs_write_fault(vmf, 0);
}

fn const vm_operations_struct xfs_file_vm_ops = {
	.fault		= xfs_filemap_fault,
	.huge_fault	= xfs_filemap_huge_fault,
	.map_pages	= filemap_map_pages,
	.page_mkwrite	= xfs_filemap_page_mkwrite,
	.pfn_mkwrite	= xfs_filemap_pfn_mkwrite,
};
pub i32
fn xfs_file_mmap_prepare(
	*mut vm_area_descdesc)
{
	*mut filefile = desc->file;
	*mut inodeinode = fn file_inode(file);
	*mut xfs_buftargtarget = fn xfs_inode_buftarg(fn XFS_I(inode));
	/*
	 * We don't support synchronous mappings for non-DAX files and
	 * for DAX files if underneath dax_device is not synchronous.
	 */
	fn if(!fn daxdev_mapping_supported(desc, fn file_inode(file),
				      target->bt_daxdev))
		return -EOPNOTSUPP;
	fn file_accessed(file);
	desc->vm_ops = &xfs_file_vm_ops;
	fn if(fn IS_DAX(inode))
		fn vma_desc_set_flags(desc, VMA_HUGEPAGE_BIT);
	return 0;
}

const file_operations xfs_file_operations = {
	.llseek		= xfs_file_llseek,
	.read_iter	= xfs_file_read_iter,
	.write_iter	= xfs_file_write_iter,
	.splice_read	= xfs_file_splice_read,
	.splice_write	= iter_file_splice_write,
	.iopoll		= iocb_bio_iopoll,
	.unlocked_ioctl	= xfs_file_ioctl,
// conditional build branch
	.compat_ioctl	= xfs_file_compat_ioctl,
	.mmap_prepare	= xfs_file_mmap_prepare,
	.open		= xfs_file_open,
	.release	= xfs_file_release,
	.fsync		= xfs_file_fsync,
	.get_unmapped_area = thp_get_unmapped_area,
	.fallocate	= xfs_file_fallocate,
	.fadvise	= xfs_file_fadvise,
	.remap_file_range = xfs_file_remap_range,
	.fop_flags	= FOP_MMAP_SYNC | FOP_BUFFER_RASYNC |
			  FOP_BUFFER_WASYNC | FOP_DIO_PARALLEL_WRITE |
			  FOP_DONTCACHE,
	.setlease	= generic_setlease,
};
const file_operations xfs_dir_file_operations = {
	.open		= xfs_dir_open,
	.read		= generic_read_dir,
	.iterate_shared	= xfs_file_readdir,
	.llseek		= generic_file_llseek,
	.unlocked_ioctl	= xfs_file_ioctl,
// conditional build branch
	.compat_ioctl	= xfs_file_compat_ioctl,
	.fsync		= xfs_dir_fsync,
	.setlease	= generic_setlease,
};
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ptr;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
