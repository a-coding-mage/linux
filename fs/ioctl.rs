// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ioctl.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const FIEMAP_MAX_EXTENTS: usize = u32::MAX as usize / core::mem::size_of::<fiemap_extent>();

unsafe fn vfs_ioctl(filp: *mut file, cmd: u32, arg: c_ulong) -> c_int {
    let mut error = -ENOTTY;
    if (*(*filp).f_op).unlocked_ioctl.is_none() { return error; }
    error = ((*(*filp).f_op).unlocked_ioctl.unwrap())(filp, cmd, arg);
    if error == -ENOIOCTLCMD { error = -ENOTTY; }
    error
}

unsafe fn ioctl_fibmap(filp: *mut file, p: *mut c_int) -> c_int {
    let inode = file_inode(filp);
    let sb = (*inode).i_sb;
    if !capable(CAP_SYS_RAWIO) { return -EPERM; }
    let mut ur_block: c_int = 0;
    let mut error = get_user(&mut ur_block, p);
    if error != 0 { return error; }
    if ur_block < 0 { return -EINVAL; }
    let mut block = ur_block as sector_t;
    error = bmap(inode, &mut block);
    if block > INT_MAX as sector_t {
        error = -ERANGE;
        pr_warn_ratelimited!("[{}] FS: {} File: {:?} would truncate fibmap result\n", current_comm(), (*sb).s_id, filp);
    }
    if error != 0 { ur_block = 0; } else { ur_block = block as c_int; }
    if put_user(ur_block, p) != 0 { error = -EFAULT; }
    error
}

pub unsafe fn fiemap_fill_next_extent(fieinfo: *mut fiemap_extent_info, logical: u64, phys: u64, len: u64, mut flags: u32) -> c_int {
    let mut extent: fiemap_extent = core::mem::zeroed();
    let dest = (*fieinfo).fi_extents_start;
    if (*fieinfo).fi_extents_max == 0 {
        (*fieinfo).fi_extents_mapped += 1;
        return if flags & FIEMAP_EXTENT_LAST != 0 { 1 } else { 0 };
    }
    if (*fieinfo).fi_extents_mapped >= (*fieinfo).fi_extents_max { return 1; }
    const SET_UNKNOWN_FLAGS: u32 = FIEMAP_EXTENT_DELALLOC;
    const SET_NO_UNMOUNTED_IO_FLAGS: u32 = FIEMAP_EXTENT_DATA_ENCRYPTED;
    const SET_NOT_ALIGNED_FLAGS: u32 = FIEMAP_EXTENT_DATA_TAIL | FIEMAP_EXTENT_DATA_INLINE;
    if flags & SET_UNKNOWN_FLAGS != 0 { flags |= FIEMAP_EXTENT_UNKNOWN; }
    if flags & SET_NO_UNMOUNTED_IO_FLAGS != 0 { flags |= FIEMAP_EXTENT_ENCODED; }
    if flags & SET_NOT_ALIGNED_FLAGS != 0 { flags |= FIEMAP_EXTENT_NOT_ALIGNED; }
    extent.fe_logical = logical; extent.fe_physical = phys; extent.fe_length = len; extent.fe_flags = flags;
    let dest = dest.add((*fieinfo).fi_extents_mapped as usize);
    if copy_to_user(dest, &extent, core::mem::size_of::<fiemap_extent>()) != 0 { return -EFAULT; }
    (*fieinfo).fi_extents_mapped += 1;
    if (*fieinfo).fi_extents_mapped == (*fieinfo).fi_extents_max { return 1; }
    if flags & FIEMAP_EXTENT_LAST != 0 { 1 } else { 0 }
}

pub unsafe fn fiemap_prep(inode: *mut inode, fieinfo: *mut fiemap_extent_info, start: u64, len: *mut u64, mut supported_flags: u32) -> c_int {
    let maxbytes = (*(*inode).i_sb).s_maxbytes;
    if *len == 0 { return -EINVAL; }
    if start >= maxbytes { return -EFBIG; }
    if *len > maxbytes || maxbytes - *len < start { *len = maxbytes - start; }
    supported_flags |= FIEMAP_FLAG_SYNC;
    supported_flags &= FIEMAP_FLAGS_COMPAT;
    let incompat_flags = (*fieinfo).fi_flags & !supported_flags;
    if incompat_flags != 0 { (*fieinfo).fi_flags = incompat_flags; return -EBADR; }
    if (*fieinfo).fi_flags & FIEMAP_FLAG_SYNC != 0 { filemap_write_and_wait((*inode).i_mapping) } else { 0 }
}

unsafe fn ioctl_fiemap(filp: *mut file, ufiemap: *mut fiemap) -> c_int {
    let inode = file_inode(filp);
    if (*(*inode).i_op).fiemap.is_none() { return -EOPNOTSUPP; }
    let mut fiemap: fiemap = core::mem::zeroed();
    if copy_from_user(&mut fiemap, ufiemap, core::mem::size_of::<fiemap>()) != 0 { return -EFAULT; }
    if fiemap.fm_extent_count > FIEMAP_MAX_EXTENTS as u32 { return -EINVAL; }
    let mut fieinfo: fiemap_extent_info = core::mem::zeroed();
    fieinfo.fi_flags = fiemap.fm_flags; fieinfo.fi_extents_max = fiemap.fm_extent_count; fieinfo.fi_extents_start = (*ufiemap).fm_extents;
    let mut error = ((*(*inode).i_op).fiemap.unwrap())(inode, &mut fieinfo, fiemap.fm_start, fiemap.fm_length);
    fiemap.fm_flags = fieinfo.fi_flags; fiemap.fm_mapped_extents = fieinfo.fi_extents_mapped;
    if copy_to_user(ufiemap, &fiemap, core::mem::size_of::<fiemap>()) != 0 { error = -EFAULT; }
    error
}

unsafe fn ioctl_file_clone(dst_file: *mut file, srcfd: c_ulong, off: u64, olen: u64, destoff: u64) -> c_int {
    let src_file = fdget(srcfd as c_int);
    if src_file.is_null() { return -EBADF; }
    let cloned = vfs_clone_file_range(src_file, off as loff_t, dst_file, destoff as loff_t, olen as loff_t, 0);
    fdput(src_file);
    if cloned < 0 { cloned as c_int } else if olen != 0 && cloned as u64 != olen { -EINVAL } else { 0 }
}

unsafe fn ioctl_file_clone_range(file: *mut file, argp: *mut file_clone_range) -> c_int {
    let mut args: file_clone_range = core::mem::zeroed();
    if copy_from_user(&mut args, argp, core::mem::size_of::<file_clone_range>()) != 0 { return -EFAULT; }
    ioctl_file_clone(file, args.src_fd as c_ulong, args.src_offset as u64, args.src_length as u64, args.dest_offset as u64)
}

unsafe fn ioctl_preallocate(filp: *mut file, mode: c_int, argp: *mut c_void) -> c_int {
    let inode = file_inode(filp); let mut sr: space_resv = core::mem::zeroed();
    if copy_from_user(&mut sr, argp, core::mem::size_of::<space_resv>()) != 0 { return -EFAULT; }
    match sr.l_whence { SEEK_SET => {}, SEEK_CUR => sr.l_start += (*filp).f_pos, SEEK_END => sr.l_start += i_size_read(inode), _ => return -EINVAL }
    vfs_fallocate(filp, mode | FALLOC_FL_KEEP_SIZE, sr.l_start, sr.l_len)
}

unsafe fn file_ioctl(filp: *mut file, cmd: u32, p: *mut c_int) -> c_int {
    match cmd {
        FIBMAP => ioctl_fibmap(filp, p),
        FS_IOC_RESVSP | FS_IOC_RESVSP64 => ioctl_preallocate(filp, 0, p as *mut c_void),
        FS_IOC_UNRESVSP | FS_IOC_UNRESVSP64 => ioctl_preallocate(filp, FALLOC_FL_PUNCH_HOLE, p as *mut c_void),
        FS_IOC_ZERO_RANGE => ioctl_preallocate(filp, FALLOC_FL_ZERO_RANGE, p as *mut c_void),
        _ => -ENOIOCTLCMD,
    }
}

unsafe fn ioctl_fionbio(filp: *mut file, argp: *mut c_int) -> c_int {
    let mut on = 0; let error = get_user(&mut on, argp); if error != 0 { return error; }
    spin_lock(&mut (*filp).f_lock);
    if on != 0 { (*filp).f_flags |= O_NONBLOCK; } else { (*filp).f_flags &= !O_NONBLOCK; }
    spin_unlock(&mut (*filp).f_lock); error
}

unsafe fn ioctl_fioasync(fd: u32, filp: *mut file, argp: *mut c_int) -> c_int {
    let mut on = 0; let error = get_user(&mut on, argp); if error != 0 { return error; }
    let flag = if on != 0 { FASYNC } else { 0 };
    if (flag ^ (*filp).f_flags) & FASYNC != 0 {
        if let Some(fasync) = (*(*filp).f_op).fasync { return fasync(fd as c_int, filp, on); }
        return -ENOTTY;
    } 0
}

unsafe fn ioctl_file_dedupe_range(file: *mut file, argp: *mut c_void) -> c_int {
    let mut count: u16 = 0; if get_user(&mut count, (argp as *mut file_dedupe_range).cast::<u8>().add(offsetof_dest_count()) as *mut u16) != 0 { return -EFAULT; }
    let size = struct_size_file_dedupe_range(count); if size > PAGE_SIZE { return -ENOMEM; }
    let same = memdup_user(argp, size); if IS_ERR(same) { return PTR_ERR(same); }
    (*same).dest_count = count; let ret = vfs_dedupe_file_range(file, same);
    if ret != 0 { kfree(same); return ret; }
    let ret = if copy_to_user(argp, same, size) != 0 { -EFAULT } else { 0 }; kfree(same); ret
}

unsafe fn ioctl_getfsuuid(file: *mut file, argp: *mut c_void) -> c_int {
    let sb = (*file_inode(file)).i_sb; if (*sb).s_uuid_len == 0 { return -ENOTTY; }
    let mut u: fsuuid2 = core::mem::zeroed(); u.len = (*sb).s_uuid_len; memcpy(u.uuid.as_mut_ptr(), (*sb).s_uuid.as_ptr(), (*sb).s_uuid_len as usize);
    if copy_to_user(argp, &u, core::mem::size_of::<fsuuid2>()) != 0 { -EFAULT } else { 0 }
}

unsafe fn ioctl_get_fs_sysfs_path(file: *mut file, argp: *mut c_void) -> c_int {
    let sb = (*file_inode(file)).i_sb; if strlen((*sb).s_sysfs_name) == 0 { return -ENOTTY; }
    let mut u: fs_sysfs_path = core::mem::zeroed(); u.len = scnprintf(u.name.as_mut_ptr(), u.name.len(), (*sb).s_type.name, (*sb).s_sysfs_name);
    if copy_to_user(argp, &u, core::mem::size_of::<fs_sysfs_path>()) != 0 { -EFAULT } else { 0 }
}

// The remaining syscall dispatch and compatibility wrappers preserve the kernel's
// externally supplied ioctl constants, structures, helpers, and function pointers.
pub unsafe fn ioctl(fd: u32, cmd: u32, arg: c_ulong) -> c_int {
    let f = fdget(fd as c_int); if f.is_null() { return -EBADF; }
    let mut error = security_file_ioctl(f, cmd, arg);
    if error == 0 { error = do_vfs_ioctl(f, fd, cmd, arg); }
    if error == -ENOIOCTLCMD { error = vfs_ioctl(f, cmd, arg); }
    fdput(f); error
}

unsafe fn do_vfs_ioctl(filp: *mut file, fd: u32, cmd: u32, arg: c_ulong) -> c_int {
    let inode = file_inode(filp); let argp = arg as *mut c_void;
    match cmd {
        FIOCLEX => { set_close_on_exec(fd, 1); 0 }, FIONCLEX => { set_close_on_exec(fd, 0); 0 },
        FIONBIO => ioctl_fionbio(filp, argp as *mut c_int), FIOASYNC => ioctl_fioasync(fd, filp, argp as *mut c_int),
        FIFREEZE => ioctl_fsfreeze(filp), FITHAW => ioctl_fsthaw(filp),
        FIGETBSZ => { if (*(*inode).i_sb).s_blocksize == 0 { -EINVAL } else { put_user((*(*inode).i_sb).s_blocksize, argp as *mut c_int) } },
        FIONREAD => { if !S_ISREG((*inode).i_mode) || IS_ANON_FILE(inode) { vfs_ioctl(filp, cmd, arg) } else { put_user((i_size_read(inode) - (*filp).f_pos) as c_int, argp as *mut c_int) } },
        FS_IOC_GETFLAGS => ioctl_getflags(filp, argp), FS_IOC_SETFLAGS => ioctl_setflags(filp, argp),
        FS_IOC_FSGETXATTR => ioctl_fsgetxattr(filp, argp), FS_IOC_FSSETXATTR => ioctl_fssetxattr(filp, argp),
        FS_IOC_FIEMAP => ioctl_fiemap(filp, argp as *mut fiemap),
        FICLONE => ioctl_file_clone(filp, arg, 0, 0, 0), FICLONERANGE => ioctl_file_clone_range(filp, argp as *mut file_clone_range),
        FIDEDUPERANGE => ioctl_file_dedupe_range(filp, argp), FS_IOC_GETFSUUID => ioctl_getfsuuid(filp, argp),
        FS_IOC_GETFSSYSFSPATH => ioctl_get_fs_sysfs_path(filp, argp),
        _ if S_ISREG((*inode).i_mode) && !IS_ANON_FILE(inode) => file_ioctl(filp, cmd, argp as *mut c_int),
        _ => -ENOIOCTLCMD,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
