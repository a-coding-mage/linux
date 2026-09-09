// SPDX-License-Identifier: GPL-2.0
/* sys_sparc32.c: Conversion between 32bit and 64bit native syscalls.
 *
 * Copyright (C) 1997,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 * Copyright (C) 1997, 2007 David S. Miller (davem@davemloft.net)
 *
 * These routines maintain argument size conversion between 32bit and 64bit
 * environment.
 */

// Linux kernel and architecture dependencies are supplied by the surrounding translation unit.

pub unsafe fn truncate64(path: *const core::ffi::c_char, high: u32, low: u32) -> i32 {
    ksys_truncate(path, ((high as u64) << 32) | low as u64)
}

pub unsafe fn ftruncate64(fd: u32, high: u32, low: u32) -> i32 {
    ksys_ftruncate(fd, ((high as u64) << 32) | low as u64, FTRUNCATE_LFS)
}

unsafe fn cp_compat_stat64(stat: *mut kstat, statbuf: *mut compat_stat64) -> i32 {
    let mut err: i32;
    err = put_user(huge_encode_dev((*stat).dev), &mut (*statbuf).st_dev);
    err |= put_user((*stat).ino, &mut (*statbuf).st_ino);
    err |= put_user((*stat).mode, &mut (*statbuf).st_mode);
    err |= put_user((*stat).nlink, &mut (*statbuf).st_nlink);
    err |= put_user(from_kuid_munged(current_user_ns(), (*stat).uid), &mut (*statbuf).st_uid);
    err |= put_user(from_kgid_munged(current_user_ns(), (*stat).gid), &mut (*statbuf).st_gid);
    err |= put_user(huge_encode_dev((*stat).rdev), &mut (*statbuf).st_rdev);
    err |= put_user(0, (*statbuf).__pad3.as_mut_ptr() as *mut usize);
    err |= put_user((*stat).size, &mut (*statbuf).st_size);
    err |= put_user((*stat).blksize, &mut (*statbuf).st_blksize);
    err |= put_user(0, (*statbuf).__pad4.as_mut_ptr() as *mut u32);
    err |= put_user(0, (*statbuf).__pad4.as_mut_ptr().add(4) as *mut u32);
    err |= put_user((*stat).blocks, &mut (*statbuf).st_blocks);
    err |= put_user((*stat).atime.tv_sec, &mut (*statbuf).st_atime);
    err |= put_user((*stat).atime.tv_nsec, &mut (*statbuf).st_atime_nsec);
    err |= put_user((*stat).mtime.tv_sec, &mut (*statbuf).st_mtime);
    err |= put_user((*stat).mtime.tv_nsec, &mut (*statbuf).st_mtime_nsec);
    err |= put_user((*stat).ctime.tv_sec, &mut (*statbuf).st_ctime);
    err |= put_user((*stat).ctime.tv_nsec, &mut (*statbuf).st_ctime_nsec);
    err |= put_user(0, &mut (*statbuf).__unused4);
    err |= put_user(0, &mut (*statbuf).__unused5);
    err
}

pub unsafe fn stat64(filename: *const core::ffi::c_char, statbuf: *mut compat_stat64) -> i32 {
    let mut stat: kstat = core::mem::zeroed();
    let mut error = vfs_stat(filename, &mut stat);
    if error == 0 { error = cp_compat_stat64(&mut stat, statbuf); }
    error
}

pub unsafe fn lstat64(filename: *const core::ffi::c_char, statbuf: *mut compat_stat64) -> i32 {
    let mut stat: kstat = core::mem::zeroed();
    let mut error = vfs_lstat(filename, &mut stat);
    if error == 0 { error = cp_compat_stat64(&mut stat, statbuf); }
    error
}

pub unsafe fn fstat64(fd: u32, statbuf: *mut compat_stat64) -> i32 {
    let mut stat: kstat = core::mem::zeroed();
    let mut error = vfs_fstat(fd, &mut stat);
    if error == 0 { error = cp_compat_stat64(&mut stat, statbuf); }
    error
}

pub unsafe fn fstatat64(dfd: u32, filename: *const core::ffi::c_char,
                        statbuf: *mut compat_stat64, flag: i32) -> i32 {
    let mut stat: kstat = core::mem::zeroed();
    let error = vfs_fstatat(dfd, filename, &mut stat, flag);
    if error != 0 { return error; }
    cp_compat_stat64(&mut stat, statbuf)
}

pub unsafe fn sparc_sigaction(sig: i32, act: *mut compat_old_sigaction,
                              oact: *mut compat_old_sigaction) -> i32 {
    WARN_ON_ONCE(sig >= 0);
    compat_sys_sigaction(-sig, act, oact)
}

pub unsafe fn rt_sigaction(sig: i32, act: *mut compat_sigaction,
                           oact: *mut compat_sigaction, restorer: *mut core::ffi::c_void,
                           sigsetsize: usize) -> i32 {
    let mut new_ka: k_sigaction = core::mem::zeroed();
    let mut old_ka: k_sigaction = core::mem::zeroed();
    if sigsetsize != core::mem::size_of::<compat_sigset_t>() { return -EINVAL; }
    if !act.is_null() {
        let mut u_handler: u32 = 0;
        let mut u_restorer: u32 = 0;
        new_ka.ka_restorer = restorer;
        let mut ret = get_user(&mut u_handler, &(*act).sa_handler);
        new_ka.sa.sa_handler = compat_ptr(u_handler);
        ret |= get_compat_sigset(&mut new_ka.sa.sa_mask, &(*act).sa_mask);
        ret |= get_user(&mut new_ka.sa.sa_flags, &(*act).sa_flags);
        ret |= get_user(&mut u_restorer, &(*act).sa_restorer);
        new_ka.sa.sa_restorer = compat_ptr(u_restorer);
        if ret != 0 { return -EFAULT; }
    }
    let mut ret = do_sigaction(sig, if !act.is_null() { &mut new_ka } else { core::ptr::null_mut() },
                               if !oact.is_null() { &mut old_ka } else { core::ptr::null_mut() });
    if ret == 0 && !oact.is_null() {
        ret = put_user(ptr_to_compat(old_ka.sa.sa_handler), &mut (*oact).sa_handler);
        ret |= put_compat_sigset(&(*oact).sa_mask, &old_ka.sa.sa_mask, core::mem::size_of_val(&(*oact).sa_mask));
        ret |= put_user(old_ka.sa.sa_flags, &mut (*oact).sa_flags);
        ret |= put_user(ptr_to_compat(old_ka.sa.sa_restorer), &mut (*oact).sa_restorer);
        if ret != 0 { ret = -EFAULT; }
    }
    ret
}

pub unsafe fn pread64(fd: u32, ubuf: *mut core::ffi::c_char, count: usize, poshi: u32, poslo: u32) -> i64 { ksys_pread64(fd, ubuf, count, ((poshi as u64) << 32) | poslo as u64) }
pub unsafe fn pwrite64(fd: u32, ubuf: *const core::ffi::c_char, count: usize, poshi: u32, poslo: u32) -> i64 { ksys_pwrite64(fd, ubuf, count, ((poshi as u64) << 32) | poslo as u64) }
pub unsafe fn readahead(fd: i32, offhi: u32, offlo: u32, count: usize) -> i64 { ksys_readahead(fd, ((offhi as u64) << 32) | offlo as u64, count) }
pub unsafe fn fadvise64(fd: i32, offhi: u32, offlo: u32, len: usize, advice: i32) -> i64 { ksys_fadvise64_64(fd, ((offhi as u64) << 32) | offlo as u64, len, advice) }
pub unsafe fn fadvise64_64(fd: i32, offhi: u32, offlo: u32, lenhi: u32, lenlo: u32, advice: i32) -> i64 { ksys_fadvise64_64(fd, ((offhi as u64) << 32) | offlo as u64, ((lenhi as u64) << 32) | lenlo as u64, advice) }
pub unsafe fn sync_file_range(fd: u32, off_high: u32, off_low: u32, nb_high: u32, nb_low: u32, flags: u32) -> i64 { ksys_sync_file_range(fd, ((off_high as u64) << 32) | off_low as u64, ((nb_high as u64) << 32) | nb_low as u64, flags) }
pub unsafe fn fallocate(fd: i32, mode: i32, offhi: u32, offlo: u32, lenhi: u32, lenlo: u32) -> i64 { ksys_fallocate(fd, mode, ((offhi as i64) << 32) | offlo as i64, ((lenhi as i64) << 32) | lenlo as i64) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
