// SPDX-License-Identifier: GPL-2.0
/*
 * sys_ia32.c: Conversion between 32bit and 64bit native syscalls. Based on
 *             sys_sparc32
 *
 * These routines maintain argument size conversion between 32bit and 64bit
 * environment.
 */

// C headers omitted; their symbols are supplied by the surrounding kernel.

#[inline]
unsafe fn aa<T>(x: T) -> c_ulong where T: Into<c_ulong> { x.into() }

pub unsafe fn ia32_truncate64(filename: *const c_char, offset_low: c_ulong,
                               offset_high: c_ulong) -> c_long {
    ksys_truncate(filename, ((offset_high as loff_t) << 32) | offset_low as loff_t)
}

pub unsafe fn ia32_ftruncate64(fd: c_uint, offset_low: c_ulong,
                                offset_high: c_ulong) -> c_long {
    ksys_ftruncate(fd, ((offset_high as loff_t) << 32) | offset_low as loff_t, FTRUNCATE_LFS)
}

// The next two assume little endian.
pub unsafe fn ia32_pread64(fd: c_uint, ubuf: *mut c_char, count: u32,
                           poslo: u32, poshi: u32) -> c_long {
    ksys_pread64(fd, ubuf, count, ((aa(poshi) as loff_t) << 32) | aa(poslo) as loff_t)
}

pub unsafe fn ia32_pwrite64(fd: c_uint, ubuf: *const c_char, count: u32,
                            poslo: u32, poshi: u32) -> c_long {
    ksys_pwrite64(fd, ubuf, count, ((aa(poshi) as loff_t) << 32) | aa(poslo) as loff_t)
}

pub unsafe fn ia32_fadvise64_64(fd: c_int, offset_low: u32, offset_high: u32,
                                len_low: u32, len_high: u32, advice: c_int) -> c_long {
    ksys_fadvise64_64(fd, ((offset_high as u64) << 32) | offset_low as u64,
                      ((len_high as u64) << 32) | len_low as u64, advice)
}

pub unsafe fn ia32_readahead(fd: c_int, off_lo: c_uint, off_hi: c_uint,
                             count: usize) -> c_long {
    ksys_readahead(fd, ((off_hi as u64) << 32) | off_lo as u64, count)
}

pub unsafe fn ia32_sync_file_range(fd: c_int, off_low: c_uint, off_hi: c_uint,
                                   n_low: c_uint, n_hi: c_uint, flags: c_int) -> c_long {
    ksys_sync_file_range(fd, ((off_hi as u64) << 32) | off_low as u64,
                         ((n_hi as u64) << 32) | n_low as u64, flags)
}

pub unsafe fn ia32_fadvise64(fd: c_int, offset_lo: c_uint, offset_hi: c_uint,
                             len: usize, advice: c_int) -> c_long {
    ksys_fadvise64_64(fd, ((offset_hi as u64) << 32) | offset_lo as u64, len, advice)
}

pub unsafe fn ia32_fallocate(fd: c_int, mode: c_int, offset_lo: c_uint,
                             offset_hi: c_uint, len_lo: c_uint, len_hi: c_uint) -> c_long {
    ksys_fallocate(fd, mode, ((offset_hi as u64) << 32) | offset_lo as u64,
                   ((len_hi as u64) << 32) | len_lo as u64)
}

// CONFIG_IA32_EMULATION

unsafe fn cp_stat64(ubuf: *mut stat64, stat: *mut kstat) -> c_int {
    let mut uid = 0;
    let mut gid = 0;
    uid = from_kuid_munged(current_user_ns(), (*stat).uid);
    gid = from_kgid_munged(current_user_ns(), (*stat).gid);
    if !user_write_access_begin(ubuf as *mut c_void, core::mem::size_of::<stat64>()) {
        return -EFAULT;
    }
    unsafe_put_user(huge_encode_dev((*stat).dev), &mut (*ubuf).st_dev)?;
    unsafe_put_user((*stat).ino, &mut (*ubuf).__st_ino)?;
    unsafe_put_user((*stat).ino, &mut (*ubuf).st_ino)?;
    unsafe_put_user((*stat).mode, &mut (*ubuf).st_mode)?;
    unsafe_put_user((*stat).nlink, &mut (*ubuf).st_nlink)?;
    unsafe_put_user(uid, &mut (*ubuf).st_uid)?;
    unsafe_put_user(gid, &mut (*ubuf).st_gid)?;
    unsafe_put_user(huge_encode_dev((*stat).rdev), &mut (*ubuf).st_rdev)?;
    unsafe_put_user((*stat).size, &mut (*ubuf).st_size)?;
    unsafe_put_user((*stat).atime.tv_sec, &mut (*ubuf).st_atime)?;
    unsafe_put_user((*stat).atime.tv_nsec, &mut (*ubuf).st_atime_nsec)?;
    unsafe_put_user((*stat).mtime.tv_sec, &mut (*ubuf).st_mtime)?;
    unsafe_put_user((*stat).mtime.tv_nsec, &mut (*ubuf).st_mtime_nsec)?;
    unsafe_put_user((*stat).ctime.tv_sec, &mut (*ubuf).st_ctime)?;
    unsafe_put_user((*stat).ctime.tv_nsec, &mut (*ubuf).st_ctime_nsec)?;
    unsafe_put_user((*stat).blksize, &mut (*ubuf).st_blksize)?;
    unsafe_put_user((*stat).blocks, &mut (*ubuf).st_blocks)?;
    user_access_end();
    0
}

pub unsafe fn ia32_stat64(filename: *const c_char, statbuf: *mut stat64) -> c_int {
    let mut stat = core::mem::MaybeUninit::<kstat>::uninit();
    let ret = vfs_stat(filename, stat.as_mut_ptr());
    if ret == 0 { cp_stat64(statbuf, stat.as_mut_ptr()) } else { ret }
}

pub unsafe fn ia32_lstat64(filename: *const c_char, statbuf: *mut stat64) -> c_int {
    let mut stat = core::mem::MaybeUninit::<kstat>::uninit();
    let ret = vfs_lstat(filename, stat.as_mut_ptr());
    if ret == 0 { cp_stat64(statbuf, stat.as_mut_ptr()) } else { ret }
}

pub unsafe fn ia32_fstat64(fd: c_uint, statbuf: *mut stat64) -> c_int {
    let mut stat = core::mem::MaybeUninit::<kstat>::uninit();
    let ret = vfs_fstat(fd, stat.as_mut_ptr());
    if ret == 0 { cp_stat64(statbuf, stat.as_mut_ptr()) } else { ret }
}

pub unsafe fn ia32_fstatat64(dfd: c_uint, filename: *const c_char,
                             statbuf: *mut stat64, flag: c_int) -> c_int {
    let mut stat = core::mem::MaybeUninit::<kstat>::uninit();
    let error = vfs_fstatat(dfd, filename, stat.as_mut_ptr(), flag);
    if error != 0 { return error; }
    cp_stat64(statbuf, stat.as_mut_ptr())
}

#[repr(C)]
pub struct mmap_arg_struct32 {
    pub addr: c_uint, pub len: c_uint, pub prot: c_uint,
    pub flags: c_uint, pub fd: c_uint, pub offset: c_uint,
}

pub unsafe fn ia32_mmap(arg: *const mmap_arg_struct32) -> c_long {
    let mut a = core::mem::MaybeUninit::<mmap_arg_struct32>::uninit();
    if copy_from_user(a.as_mut_ptr() as *mut c_void, arg as *const c_void,
                      core::mem::size_of::<mmap_arg_struct32>()) != 0 { return -EFAULT; }
    let a = a.assume_init();
    if a.offset & !PAGE_MASK != 0 { return -EINVAL; }
    ksys_mmap_pgoff(a.addr, a.len, a.prot, a.flags, a.fd, a.offset >> PAGE_SHIFT)
}

pub unsafe fn ia32_clone(clone_flags: c_ulong, newsp: c_ulong,
                         parent_tidptr: *mut c_int, tls_val: c_ulong,
                         child_tidptr: *mut c_int) -> c_long {
    let args = kernel_clone_args {
        flags: clone_flags & !CSIGNAL, pidfd: parent_tidptr,
        child_tid: child_tidptr, parent_tid: parent_tidptr,
        exit_signal: clone_flags & CSIGNAL, stack: newsp, tls: tls_val,
    };
    kernel_clone(&args)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
