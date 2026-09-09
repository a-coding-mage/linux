// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding repository are intentionally not reimplemented here.

unsafe fn blkpg_do_ioctl(bdev: *mut block_device, upart: *mut blkpg_partition, op: c_int) -> c_int {
    let disk = (*bdev).bd_disk;
    let mut p: blkpg_partition = core::mem::zeroed();
    let mut start: sector_t;
    let mut length: sector_t;
    let capacity: sector_t;
    let mut end: sector_t;
    if !capable(CAP_SYS_ADMIN) { return -EACCES; }
    if copy_from_user(&mut p as *mut _, upart as *const _, core::mem::size_of::<blkpg_partition>()) != 0 { return -EFAULT; }
    if bdev_is_partition(bdev) { return -EINVAL; }
    if p.pno <= 0 { return -EINVAL; }
    if op == BLKPG_DEL_PARTITION { return bdev_del_partition(disk, p.pno); }
    if p.start < 0 || p.length <= 0 || LLONG_MAX - p.length < p.start { return -EINVAL; }
    /* Check that the partition is aligned to the block size */
    if !is_aligned((p.start | p.length) as u64, bdev_logical_block_size(bdev) as u64) { return -EINVAL; }
    start = p.start >> SECTOR_SHIFT;
    length = p.length >> SECTOR_SHIFT;
    capacity = get_capacity(disk);
    if check_add_overflow(start, length, &mut end) { return -EINVAL; }
    if start >= capacity || end > capacity { return -EINVAL; }
    match op {
        BLKPG_ADD_PARTITION => bdev_add_partition(disk, p.pno, start, length),
        BLKPG_RESIZE_PARTITION => bdev_resize_partition(disk, p.pno, start, length),
        _ => -EINVAL,
    }
}

unsafe fn blkpg_ioctl(bdev: *mut block_device, arg: *mut blkpg_ioctl_arg) -> c_int {
    let mut udata: *mut blkpg_partition = core::ptr::null_mut();
    let mut op = 0;
    if get_user(&mut op, &(*arg).op) != 0 || get_user(&mut udata, &(*arg).data) != 0 { return -EFAULT; }
    blkpg_do_ioctl(bdev, udata, op)
}

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] struct compat_blkpg_ioctl_arg { op: compat_int_t, flags: compat_int_t, datalen: compat_int_t, data: compat_caddr_t }
#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn compat_blkpg_ioctl(bdev: *mut block_device, arg: *mut compat_blkpg_ioctl_arg) -> c_int {
    let mut udata = 0; let mut op = 0;
    if get_user(&mut op, &(*arg).op) != 0 || get_user(&mut udata, &(*arg).data) != 0 { return -EFAULT; }
    blkpg_do_ioctl(bdev, compat_ptr(udata), op)
}

/* Check that [start, start + len) is valid and translates to logical blocks. */
unsafe fn blk_validate_byte_range(bdev: *mut block_device, start: u64, len: u64) -> c_int {
    let bs_mask = bdev_logical_block_size(bdev) as u64 - 1; let mut end = 0;
    if ((start | len) & bs_mask) != 0 || len == 0 { return -EINVAL; }
    if check_add_overflow(start, len, &mut end) || end > bdev_nr_bytes(bdev) { return -EINVAL; }
    0
}

unsafe fn blk_ioctl_discard(bdev: *mut block_device, mode: blk_mode_t, arg: c_ulong) -> c_int {
    let mut range = [0u64; 2];
    if copy_from_user(range.as_mut_ptr(), arg as *const _, core::mem::size_of_val(&range)) != 0 { return -EFAULT; }
    if bdev_max_discard_sectors(bdev) == 0 { return -EOPNOTSUPP; }
    if mode & BLK_OPEN_WRITE == 0 { return -EBADF; }
    if bdev_read_only(bdev) { return -EPERM; }
    let err = blk_validate_byte_range(bdev, range[0], range[1]); if err != 0 { return err; }
    inode_lock((*(*bdev).bd_mapping).host); filemap_invalidate_lock((*bdev).bd_mapping);
    let mut err = truncate_bdev_range(bdev, mode, range[0], range[0] + range[1] - 1);
    if err == 0 {
        let mut sector = range[0] >> SECTOR_SHIFT; let mut nr_sects = range[1] >> SECTOR_SHIFT;
        let mut prev: *mut bio = core::ptr::null_mut(); let mut plug: blk_plug = core::mem::zeroed(); blk_start_plug(&mut plug);
        while !fatal_signal_pending(current) { let bio = blk_alloc_discard_bio(bdev, &mut sector, &mut nr_sects, GFP_KERNEL); if bio.is_null() { break; } prev = bio_chain_and_submit(prev, bio); }
        if !prev.is_null() { err = bio_submit_or_kill(prev, BLKDEV_ZERO_KILLABLE); if err == -EOPNOTSUPP { err = 0; } bio_put(prev); }
        blk_finish_plug(&mut plug);
    }
    filemap_invalidate_unlock((*bdev).bd_mapping); inode_unlock((*(*bdev).bd_mapping).host); err
}

unsafe fn blk_ioctl_secure_erase(bdev: *mut block_device, mode: blk_mode_t, argp: *mut core::ffi::c_void) -> c_int {
    let mut range = [0u64; 2]; if mode & BLK_OPEN_WRITE == 0 { return -EBADF; }
    if bdev_max_secure_erase_sectors(bdev) == 0 { return -EOPNOTSUPP; }
    if copy_from_user(range.as_mut_ptr(), argp, core::mem::size_of_val(&range)) != 0 { return -EFAULT; }
    let (start, len) = (range[0], range[1]); let mut end = 0;
    if start & 511 != 0 || len & 511 != 0 || check_add_overflow(start, len, &mut end) || end > bdev_nr_bytes(bdev) { return -EINVAL; }
    inode_lock((*(*bdev).bd_mapping).host); filemap_invalidate_lock((*bdev).bd_mapping);
    let mut err = truncate_bdev_range(bdev, mode, start, end - 1); if err == 0 { err = blkdev_issue_secure_erase(bdev, start >> 9, len >> 9, GFP_KERNEL); }
    filemap_invalidate_unlock((*bdev).bd_mapping); inode_unlock((*(*bdev).bd_mapping).host); err
}

unsafe fn blk_ioctl_zeroout(bdev: *mut block_device, mode: blk_mode_t, arg: c_ulong) -> c_int {
    let mut range = [0u64; 2]; if mode & BLK_OPEN_WRITE == 0 { return -EBADF; }
    if copy_from_user(range.as_mut_ptr(), arg as *const _, core::mem::size_of_val(&range)) != 0 { return -EFAULT; }
    let (start, len) = (range[0], range[1]); let end = start.wrapping_add(len).wrapping_sub(1);
    if start & 511 != 0 || len & 511 != 0 || end >= bdev_nr_bytes(bdev) || end < start { return -EINVAL; }
    inode_lock((*(*bdev).bd_mapping).host); filemap_invalidate_lock((*bdev).bd_mapping);
    let mut err = truncate_bdev_range(bdev, mode, start, end);
    if err == 0 { err = blkdev_issue_zeroout(bdev, start >> 9, len >> 9, GFP_KERNEL, BLKDEV_ZERO_NOUNMAP | BLKDEV_ZERO_KILLABLE); }
    filemap_invalidate_unlock((*bdev).bd_mapping); inode_unlock((*(*bdev).bd_mapping).host); err
}

unsafe fn put_ushort(p: *mut u16, v: u16) -> c_int { put_user(v, p) }
unsafe fn put_int(p: *mut c_int, v: c_int) -> c_int { put_user(v, p) }
unsafe fn put_uint(p: *mut c_uint, v: c_uint) -> c_int { put_user(v, p) }
unsafe fn put_long(p: *mut c_long, v: c_long) -> c_int { put_user(v, p) }
unsafe fn put_ulong(p: *mut c_ulong, v: c_ulong) -> c_int { put_user(v, p) }
unsafe fn put_u64(p: *mut u64, v: u64) -> c_int { put_user(v, p) }

#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn compat_put_long(p: *mut compat_long_t, v: c_long) -> c_int { put_user(v, p) }
#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn compat_put_ulong(p: *mut compat_ulong_t, v: compat_ulong_t) -> c_int { put_user(v, p) }

#[repr(C)] enum pr_direction { PR_IN, PR_OUT }
unsafe fn blkdev_pr_allowed(bdev: *mut block_device, mode: blk_mode_t, dir: pr_direction) -> bool {
    if bdev_is_partition(bdev) { return false; } if capable(CAP_SYS_ADMIN) { return true; }
    match dir { pr_direction::PR_IN => mode & BLK_OPEN_READ != 0, pr_direction::PR_OUT => mode & BLK_OPEN_WRITE != 0 }
}

unsafe fn blkdev_pr_register(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_registration) -> c_int {
    let ops = (*(*bdev).bd_disk).fops.pr_ops; let mut reg: pr_registration = core::mem::zeroed();
    if !blkdev_pr_allowed(bdev, mode, pr_direction::PR_OUT) { return -EPERM; } if ops.is_null() || (*ops).pr_register.is_none() { return -EOPNOTSUPP; }
    if copy_from_user(&mut reg, arg as *const _, core::mem::size_of_val(&reg)) != 0 { return -EFAULT; }
    if reg.flags & !PR_FL_IGNORE_KEY != 0 { return -EOPNOTSUPP; } ((*ops).pr_register.unwrap())(bdev, reg.old_key, reg.new_key, reg.flags)
}
unsafe fn blkdev_pr_reserve(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_reservation) -> c_int {
    let ops = (*(*bdev).bd_disk).fops.pr_ops; let mut rsv: pr_reservation = core::mem::zeroed();
    if !blkdev_pr_allowed(bdev, mode, pr_direction::PR_OUT) { return -EPERM; } if ops.is_null() || (*ops).pr_reserve.is_none() { return -EOPNOTSUPP; }
    if copy_from_user(&mut rsv, arg as *const _, core::mem::size_of_val(&rsv)) != 0 { return -EFAULT; }
    if rsv.flags & !PR_FL_IGNORE_KEY != 0 { return -EOPNOTSUPP; } ((*ops).pr_reserve.unwrap())(bdev, rsv.key, rsv.type_, rsv.flags)
}
unsafe fn blkdev_pr_release(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_reservation) -> c_int {
    let ops = (*(*bdev).bd_disk).fops.pr_ops; let mut rsv: pr_reservation = core::mem::zeroed();
    if !blkdev_pr_allowed(bdev, mode, pr_direction::PR_OUT) { return -EPERM; } if ops.is_null() || (*ops).pr_release.is_none() { return -EOPNOTSUPP; }
    if copy_from_user(&mut rsv, arg as *const _, core::mem::size_of_val(&rsv)) != 0 { return -EFAULT; }
    if rsv.flags != 0 { return -EOPNOTSUPP; } ((*ops).pr_release.unwrap())(bdev, rsv.key, rsv.type_)
}
unsafe fn blkdev_pr_preempt(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_preempt, abort: bool) -> c_int {
    let ops = (*(*bdev).bd_disk).fops.pr_ops; let mut p: pr_preempt = core::mem::zeroed();
    if !blkdev_pr_allowed(bdev, mode, pr_direction::PR_OUT) { return -EPERM; } if ops.is_null() || (*ops).pr_preempt.is_none() { return -EOPNOTSUPP; }
    if copy_from_user(&mut p, arg as *const _, core::mem::size_of_val(&p)) != 0 { return -EFAULT; }
    if p.flags != 0 { return -EOPNOTSUPP; } ((*ops).pr_preempt.unwrap())(bdev, p.old_key, p.new_key, p.type_, abort)
}
unsafe fn blkdev_pr_clear(bdev: *mut block_device, mode: blk_mode_t, arg: *mut pr_clear) -> c_int {
    let ops = (*(*bdev).bd_disk).fops.pr_ops; let mut c: pr_clear = core::mem::zeroed();
    if !blkdev_pr_allowed(bdev, mode, pr_direction::PR_OUT) { return -EPERM; } if ops.is_null() || (*ops).pr_clear.is_none() { return -EOPNOTSUPP; }
    if copy_from_user(&mut c, arg as *const _, core::mem::size_of_val(&c)) != 0 { return -EFAULT; }
    if c.flags != 0 { return -EOPNOTSUPP; } ((*ops).pr_clear.unwrap())(bdev, c.key)
}

unsafe fn blkdev_flushbuf(bdev: *mut block_device, _cmd: c_uint, _arg: c_ulong) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EACCES; } mutex_lock(&mut (*bdev).bd_holder_lock);
    if !(*bdev).bd_holder_ops.is_null() && (*(*bdev).bd_holder_ops).sync.is_some() { ((*(*bdev).bd_holder_ops).sync.unwrap())(bdev); }
    else { mutex_unlock(&mut (*bdev).bd_holder_lock); sync_blockdev(bdev); } invalidate_bdev(bdev); 0
}
unsafe fn blkdev_roset(bdev: *mut block_device, _cmd: c_uint, arg: c_ulong) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EACCES; } let mut n = 0; if get_user(&mut n, arg as *const _) != 0 { return -EFAULT; }
    if (*(*bdev).bd_disk).fops.set_read_only.is_some() { let r = ((*(*bdev).bd_disk).fops.set_read_only.unwrap())(bdev, n); if r != 0 { return r; } }
    if n != 0 { bdev_set_flag(bdev, BD_READ_ONLY); } else { bdev_clear_flag(bdev, BD_READ_ONLY); } 0
}

// The remaining ioctl dispatch and io_uring routines retain the source dispatch structure.
// External kernel declarations and constants are supplied by the translated dependency set.
unsafe fn blkdev_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let bdev = I_BDEV((*(*file).f_mapping).host); let argp = arg as *mut core::ffi::c_void; let mode = file_to_blk_mode(file);
    match cmd {
        HDIO_GETGEO => blkdev_getgeo(bdev, argp) as c_long,
        BLKPG => blkpg_ioctl(bdev, argp as *mut _) as c_long,
        BLKRAGET | BLKFRAGET => { if argp.is_null() { return -EINVAL as c_long; } put_long(argp as *mut _, (((*(*bdev).bd_disk).bdi.ra_pages * PAGE_SIZE) / 512) as c_long) as c_long },
        BLKGETSIZE => { if bdev_nr_sectors(bdev) > !0usize as u64 { return -EFBIG as c_long; } put_ulong(argp as *mut _, bdev_nr_sectors(bdev) as c_ulong) as c_long },
        BLKBSZGET => put_int(argp as *mut _, block_size(bdev)) as c_long,
        BLKBSZSET => blkdev_bszset(file, mode, argp as *mut _) as c_long,
        BLKGETSIZE64 => put_u64(argp as *mut _, bdev_nr_bytes(bdev)) as c_long,
        BLKTRACESETUP | BLKTRACESETUP2 => blk_trace_ioctl(bdev, cmd, argp) as c_long,
        _ => { let r = blkdev_common_ioctl(bdev, mode, cmd, arg, argp); if r != -ENOIOCTLCMD { return r as c_long; } if (*(*bdev).bd_disk).fops.ioctl.is_none() { -ENOTTY as c_long } else { ((*(*bdev).bd_disk).fops.ioctl.unwrap())(bdev, mode, cmd, arg) as c_long } }
    }
}

unsafe fn blkdev_getgeo(bdev: *mut block_device, argp: *mut core::ffi::c_void) -> c_int {
    if argp.is_null() { return -EINVAL; }
    let disk = (*bdev).bd_disk; if (*disk).fops.getgeo.is_none() { return -ENOTTY; }
    let mut geo: hd_geometry = core::mem::zeroed(); geo.start = get_start_sect(bdev);
    let ret = ((*disk).fops.getgeo.unwrap())(disk, &mut geo); if ret != 0 { return ret; }
    if copy_to_user(argp, &geo as *const _, core::mem::size_of_val(&geo)) != 0 { -EFAULT } else { 0 }
}
unsafe fn blkdev_bszset(file: *mut file, mode: blk_mode_t, argp: *mut c_int) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EACCES; } if argp.is_null() { return -EINVAL; }
    let mut n = 0; if get_user(&mut n, argp) != 0 { return -EFAULT; }
    if mode & BLK_OPEN_EXCL != 0 { return set_blocksize(file, n); }
    let dev = I_BDEV((*(*file).f_mapping).host); let excl = bdev_file_open_by_dev((*dev).bd_dev, mode, &mut (*dev).bd_dev, core::ptr::null_mut());
    if is_err(excl) { return -EBUSY; } let ret = set_blocksize(excl, n); fput(excl); ret
}
unsafe fn blkdev_common_ioctl(bdev: *mut block_device, mode: blk_mode_t, cmd: c_uint, arg: c_ulong, argp: *mut core::ffi::c_void) -> c_int {
    match cmd {
        BLKFLSBUF => blkdev_flushbuf(bdev, cmd, arg), BLKROSET => blkdev_roset(bdev, cmd, arg),
        BLKDISCARD => blk_ioctl_discard(bdev, mode, arg), BLKSECDISCARD => blk_ioctl_secure_erase(bdev, mode, argp),
        BLKZEROOUT => blk_ioctl_zeroout(bdev, mode, arg), BLKGETDISKSEQ => put_u64(argp as *mut _, (*(*bdev).bd_disk).diskseq),
        BLKGETZONESZ => put_uint(argp as *mut _, bdev_zone_sectors(bdev)), BLKGETNRZONES => put_uint(argp as *mut _, bdev_nr_zones(bdev)),
        BLKROGET => put_int(argp as *mut _, bdev_read_only(bdev) as c_int), BLKSSZGET => put_int(argp as *mut _, bdev_logical_block_size(bdev) as c_int),
        BLKPBSZGET => put_uint(argp as *mut _, bdev_physical_block_size(bdev)), BLKIOMIN => put_uint(argp as *mut _, bdev_io_min(bdev)),
        BLKIOOPT => put_uint(argp as *mut _, bdev_io_opt(bdev)), BLKALIGNOFF => put_int(argp as *mut _, bdev_alignment_offset(bdev)),
        BLKDISCARDZEROES => put_uint(argp as *mut _, 0), BLKROTATIONAL => put_ushort(argp as *mut _, bdev_rot(bdev)),
        BLKRRPART => { if !capable(CAP_SYS_ADMIN) { -EACCES } else if bdev_is_partition(bdev) { -EINVAL } else { disk_scan_partitions((*bdev).bd_disk, mode | BLK_OPEN_STRICT_SCAN) } },
        BLKTRACESTART | BLKTRACESTOP | BLKTRACETEARDOWN => blk_trace_ioctl(bdev, cmd, argp),
        BLKCRYPTOIMPORTKEY | BLKCRYPTOGENERATEKEY | BLKCRYPTOPREPAREKEY => blk_crypto_ioctl(bdev, cmd, argp),
        IOC_PR_REGISTER => blkdev_pr_register(bdev, mode, argp as *mut _), IOC_PR_RESERVE => blkdev_pr_reserve(bdev, mode, argp as *mut _),
        IOC_PR_RELEASE => blkdev_pr_release(bdev, mode, argp as *mut _), IOC_PR_PREEMPT => blkdev_pr_preempt(bdev, mode, argp as *mut _, false),
        IOC_PR_PREEMPT_ABORT => blkdev_pr_preempt(bdev, mode, argp as *mut _, true), IOC_PR_CLEAR => blkdev_pr_clear(bdev, mode, argp as *mut _),
        _ => blk_get_meta_cap(bdev, cmd, argp),
    }
}

#[repr(C)] struct blk_iou_cmd { start: u64, len: u64, res: c_int, nowait: bool }
unsafe fn blk_cmd_complete(tw_req: io_tw_req, tw: io_tw_token_t) { let cmd = io_uring_cmd_from_tw(tw_req); let bic = io_uring_cmd_to_pdu::<blk_iou_cmd>(cmd); if (*bic).res == -EAGAIN && (*bic).nowait { io_uring_cmd_issue_blocking(cmd); } else { io_uring_cmd_done(cmd, (*bic).res, IO_URING_CMD_TASK_WORK_ISSUE_FLAGS); } }
unsafe fn bio_cmd_bio_end_io(bio: *mut bio) { let cmd = (*bio).bi_private as *mut io_uring_cmd; let bic = io_uring_cmd_to_pdu::<blk_iou_cmd>(cmd); if (*bio).bi_status != 0 && (*bic).res == 0 { (*bic).res = blk_status_to_errno((*bio).bi_status); } io_uring_cmd_do_in_task_lazy(cmd, blk_cmd_complete); bio_put(bio); }
unsafe fn blkdev_uring_cmd(cmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int { let bic = io_uring_cmd_to_pdu::<blk_iou_cmd>(cmd); (*bic).res = 0; (*bic).nowait = issue_flags & IO_URING_F_NONBLOCK != 0; match (*cmd).cmd_op { BLOCK_URING_CMD_DISCARD => -EOPNOTSUPP, BLOCK_URING_CMD_ZONE_RESET_ALL => -EOPNOTSUPP, _ => -EINVAL } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
