// SPDX-License-Identifier: GPL-2.0
/*
 * FUSE inode io modes.
 *
 * Copyright (c) 2024 CTERA Networks.
 */

/* Dependencies are supplied by the surrounding FUSE implementation. */

/*
 * Return true if need to wait for new opens in caching mode.
 */
#[inline]
unsafe fn fuse_is_io_cache_wait(fi: *mut fuse_inode) -> bool {
	READ_ONCE((*fi).iocachectr) < 0 && !fuse_inode_backing(fi)
}

/*
 * Called on cached file open() and on first mmap() of direct_io file.
 * Takes cached_io inode mode reference to be dropped on file release.
 *
 * Blocks new parallel dio writes and waits for the in-progress parallel dio
 * writes to complete.
 */
pub unsafe fn fuse_file_cached_io_open(
	inode: *mut inode,
	ff: *mut fuse_file,
) -> i32 {
	let fi = get_fuse_inode(inode);

	/* There are no io modes if server does not implement open */
	if (*ff).args.is_null() {
		return 0;
	}

	spin_lock(&mut (*fi).lock);
	/*
	 * Setting the bit advises new direct-io writes to use an exclusive
	 * lock - without it the wait below might be forever.
	 */
	while fuse_is_io_cache_wait(fi) {
		set_bit(FUSE_I_CACHE_IO_MODE, &mut (*fi).state);
		spin_unlock(&mut (*fi).lock);
		wait_event((*fi).direct_io_waitq, !fuse_is_io_cache_wait(fi));
		spin_lock(&mut (*fi).lock);
	}

	/*
	 * Check if inode entered passthrough io mode while waiting for parallel
	 * dio write completion.
	 */
	if fuse_inode_backing(fi) {
		clear_bit(FUSE_I_CACHE_IO_MODE, &mut (*fi).state);
		spin_unlock(&mut (*fi).lock);
		return -ETXTBSY;
	}

	WARN_ON((*ff).iomode == IOM_UNCACHED);
	if (*ff).iomode == IOM_NONE {
		(*ff).iomode = IOM_CACHED;
		if (*fi).iocachectr == 0 {
			set_bit(FUSE_I_CACHE_IO_MODE, &mut (*fi).state);
		}
		(*fi).iocachectr += 1;
	}
	spin_unlock(&mut (*fi).lock);
	0
}

unsafe fn fuse_file_cached_io_release(ff: *mut fuse_file, fi: *mut fuse_inode) {
	spin_lock(&mut (*fi).lock);
	WARN_ON((*fi).iocachectr <= 0);
	WARN_ON((*ff).iomode != IOM_CACHED);
	(*ff).iomode = IOM_NONE;
	(*fi).iocachectr -= 1;
	if (*fi).iocachectr == 0 {
		clear_bit(FUSE_I_CACHE_IO_MODE, &mut (*fi).state);
	}
	spin_unlock(&mut (*fi).lock);
}

/* Start strictly uncached io mode where cache access is not allowed */
pub unsafe fn fuse_inode_uncached_io_start(
	fi: *mut fuse_inode,
	fb: *mut fuse_backing,
) -> i32 {
	let mut oldfb: *mut fuse_backing;
	let mut err = 0;

	spin_lock(&mut (*fi).lock);
	/* deny conflicting backing files on same fuse inode */
	oldfb = fuse_inode_backing(fi);
	if !fb.is_null() && !oldfb.is_null() && oldfb != fb {
		err = -EBUSY;
		spin_unlock(&mut (*fi).lock);
		return err;
	}
	if (*fi).iocachectr > 0 {
		err = -ETXTBSY;
		spin_unlock(&mut (*fi).lock);
		return err;
	}
	(*fi).iocachectr -= 1;

	/* fuse inode holds a single refcount of backing file */
	if !fb.is_null() && oldfb.is_null() {
		oldfb = fuse_inode_backing_set(fi, fb);
		WARN_ON_ONCE(!oldfb.is_null());
	} else {
		fuse_backing_put(fb);
	}
	spin_unlock(&mut (*fi).lock);
	err
}

/* Takes uncached_io inode mode reference to be dropped on file release */
unsafe fn fuse_file_uncached_io_open(
	inode: *mut inode,
	ff: *mut fuse_file,
	fb: *mut fuse_backing,
) -> i32 {
	let fi = get_fuse_inode(inode);
	let err = fuse_inode_uncached_io_start(fi, fb);
	if err != 0 {
		return err;
	}
	WARN_ON((*ff).iomode != IOM_NONE);
	(*ff).iomode = IOM_UNCACHED;
	0
}

pub unsafe fn fuse_inode_uncached_io_end(fi: *mut fuse_inode) {
	let mut oldfb: *mut fuse_backing = core::ptr::null_mut();

	spin_lock(&mut (*fi).lock);
	WARN_ON((*fi).iocachectr >= 0);
	(*fi).iocachectr += 1;
	if (*fi).iocachectr == 0 {
		wake_up(&mut (*fi).direct_io_waitq);
		oldfb = fuse_inode_backing_set(fi, core::ptr::null_mut());
	}
	spin_unlock(&mut (*fi).lock);
	if !oldfb.is_null() {
		fuse_backing_put(oldfb);
	}
}

/* Drop uncached_io reference from passthrough open */
unsafe fn fuse_file_uncached_io_release(ff: *mut fuse_file, fi: *mut fuse_inode) {
	WARN_ON((*ff).iomode != IOM_UNCACHED);
	(*ff).iomode = IOM_NONE;
	fuse_inode_uncached_io_end(fi);
}

/* Open flags allowed in combination with FOPEN_PASSTHROUGH. */
const FOPEN_PASSTHROUGH_MASK: u32 =
	FOPEN_PASSTHROUGH | FOPEN_DIRECT_IO | FOPEN_PARALLEL_DIRECT_WRITES | FOPEN_NOFLUSH;

unsafe fn fuse_file_passthrough_open(inode: *mut inode, file: *mut file) -> i32 {
	let ff = (*file).private_data;
	let fc = get_fuse_conn(inode);
	let fb;

	if !IS_ENABLED(CONFIG_FUSE_PASSTHROUGH) || !(*fc).passthrough
		|| ((*ff).open_flags & !FOPEN_PASSTHROUGH_MASK) != 0
	{
		return -EINVAL;
	}

	fb = fuse_passthrough_open(file, (*(*ff).args).open_outarg.backing_id);
	if IS_ERR(fb) {
		return PTR_ERR(fb);
	}
	let err = fuse_file_uncached_io_open(inode, ff, fb);
	if err == 0 {
		return 0;
	}
	fuse_passthrough_release(ff, fb);
	fuse_backing_put(fb);
	err
}

/* Request access to submit new io to inode via open file */
pub unsafe fn fuse_file_io_open(file: *mut file, inode: *mut inode) -> i32 {
	let ff = (*file).private_data;
	let fi = get_fuse_inode(inode);
	let mut err;

	if FUSE_IS_DAX(inode) || (*ff).args.is_null() { return 0; }
	err = -EINVAL;
	if fuse_inode_backing(fi) && ((*ff).open_flags & FOPEN_PASSTHROUGH) == 0 {
		return -EIO;
	}
	if ((*ff).open_flags & FOPEN_DIRECT_IO) == 0 { (*ff).open_flags &= !FOPEN_PARALLEL_DIRECT_WRITES; }
	if ((*ff).open_flags & FOPEN_DIRECT_IO) != 0 && ((*ff).open_flags & FOPEN_PASSTHROUGH) == 0 { return 0; }
	err = if ((*ff).open_flags & FOPEN_PASSTHROUGH) != 0 { fuse_file_passthrough_open(inode, file) } else { fuse_file_cached_io_open(inode, ff) };
	if err != 0 {
		pr_debug!("failed to open file in requested io mode (open_flags=0x{:x}, err={}).\n", (*ff).open_flags, err);
		/*
		 * The file open mode determines the inode io mode.
		 * Using incorrect open mode is a server mistake, which results in
		 * user visible failure of open() with EIO error.
		 */
		return -EIO;
	}
	0
}

/* No more pending io and no new io possible to inode via open/mmapped file */
pub unsafe fn fuse_file_io_release(ff: *mut fuse_file, inode: *mut inode) {
	let fi = get_fuse_inode(inode);
	match (*ff).iomode {
		IOM_NONE => (),
		IOM_UNCACHED => fuse_file_uncached_io_release(ff, fi),
		IOM_CACHED => fuse_file_cached_io_release(ff, fi),
		_ => (),
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
