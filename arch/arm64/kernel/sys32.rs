// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm64/kernel/sys32.c
 *
 * Copyright (C) 2015 ARM Ltd.
 */

// Linux and architecture declarations supplied by the surrounding kernel.

extern "C" {
    fn compat_sys_sigreturn() -> c_long;
    fn compat_sys_rt_sigreturn() -> c_long;

    fn kcompat_sys_statfs64(pathname: *const c_char, sz: compat_size_t,
                            buf: *mut compat_statfs64) -> c_long;
    fn kcompat_sys_fstatfs64(fd: c_uint, sz: compat_size_t,
                             buf: *mut compat_statfs64) -> c_long;
    fn ksys_mmap_pgoff(addr: c_ulong, len: c_ulong, prot: c_ulong,
                       flags: c_ulong, fd: c_ulong, pgoff: c_ulong) -> c_long;
    fn ksys_pread64(fd: c_uint, buf: *mut c_char, count: usize, pos: u64) -> c_long;
    fn ksys_pwrite64(fd: c_uint, buf: *const c_char, count: usize, pos: u64) -> c_long;
    fn ksys_truncate(pathname: *const c_char, length: u64) -> c_long;
    fn ksys_ftruncate(fd: c_uint, length: u64, flag: c_uint) -> c_long;
    fn ksys_readahead(fd: c_int, offset: u64, count: usize) -> c_long;
    fn ksys_fadvise64_64(fd: c_int, offset: u64, len: u64, advice: c_int) -> c_long;
    fn ksys_sync_file_range(fd: c_int, offset: u64, nbytes: u64, flags: c_uint) -> c_long;
    fn ksys_fallocate(fd: c_int, mode: c_int, offset: u64, len: u64) -> c_long;
}

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_long = isize;
type compat_size_t = u32;

#[repr(C)]
pub struct compat_statfs64 { _private: [u8; 0] }

#[inline]
pub unsafe extern "C" fn compat_sys_aarch32_statfs64(
    pathname: *const c_char, mut sz: compat_size_t, buf: *mut compat_statfs64,
) -> c_long {
    /*
     * 32-bit ARM applies an OABI compatibility fixup to statfs64 and
     * fstatfs64 regardless of whether OABI is in use, and therefore
     * arbitrary binaries may rely upon it, so we must do the same.
     * For more details, see commit:
     *
     * 713c481519f19df9 ("[ARM] 3108/2: old ABI compat: statfs64 and
     * fstatfs64")
     */
    if sz == 88 { sz = 84; }
    kcompat_sys_statfs64(pathname, sz, buf)
}

#[inline]
pub unsafe extern "C" fn compat_sys_aarch32_fstatfs64(
    fd: c_uint, mut sz: compat_size_t, buf: *mut compat_statfs64,
) -> c_long {
    /* see aarch32_statfs64 */
    if sz == 88 { sz = 84; }
    kcompat_sys_fstatfs64(fd, sz, buf)
}

/* Note: off_4k is always in units of 4K. If we can't do the requested offset
 * because it is not page-aligned, we return -EINVAL. */
pub unsafe extern "C" fn compat_sys_aarch32_mmap2(
    addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong,
    fd: c_ulong, mut off_4k: c_ulong,
) -> c_long {
    if off_4k & (!PAGE_MASK >> 12) != 0 { return -EINVAL; }
    off_4k >>= PAGE_SHIFT - 12;
    ksys_mmap_pgoff(addr, len, prot, flags, fd, off_4k)
}

#[inline]
unsafe fn arg_u64(hi: u32, lo: u32) -> u64 { ((hi as u64) << 32) | lo as u64 }

pub unsafe extern "C" fn compat_sys_aarch32_pread64(
    fd: c_uint, buf: *mut c_char, count: usize, _pad: u32,
    pos_hi: u32, pos_lo: u32,
) -> c_long { ksys_pread64(fd, buf, count, arg_u64(pos_hi, pos_lo)) }

pub unsafe extern "C" fn compat_sys_aarch32_pwrite64(
    fd: c_uint, buf: *const c_char, count: usize, _pad: u32,
    pos_hi: u32, pos_lo: u32,
) -> c_long { ksys_pwrite64(fd, buf, count, arg_u64(pos_hi, pos_lo)) }

pub unsafe extern "C" fn compat_sys_aarch32_truncate64(
    pathname: *const c_char, _pad: u32, length_hi: u32, length_lo: u32,
) -> c_long { ksys_truncate(pathname, arg_u64(length_hi, length_lo)) }

pub unsafe extern "C" fn compat_sys_aarch32_ftruncate64(
    fd: c_uint, _pad: u32, length_hi: u32, length_lo: u32,
) -> c_long { ksys_ftruncate(fd, arg_u64(length_hi, length_lo), FTRUNCATE_LFS) }

pub unsafe extern "C" fn compat_sys_aarch32_readahead(
    fd: c_int, _pad: u32, offset_hi: u32, offset_lo: u32, count: usize,
) -> c_long { ksys_readahead(fd, arg_u64(offset_hi, offset_lo), count) }

pub unsafe extern "C" fn compat_sys_aarch32_fadvise64_64(
    fd: c_int, advice: c_int, offset_hi: u32, offset_lo: u32,
    len_hi: u32, len_lo: u32,
) -> c_long { ksys_fadvise64_64(fd, arg_u64(offset_hi, offset_lo), arg_u64(len_hi, len_lo), advice) }

pub unsafe extern "C" fn compat_sys_aarch32_sync_file_range2(
    fd: c_int, flags: c_uint, offset_hi: u32, offset_lo: u32,
    nbytes_hi: u32, nbytes_lo: u32,
) -> c_long { ksys_sync_file_range(fd, arg_u64(offset_hi, offset_lo), arg_u64(nbytes_hi, nbytes_lo), flags) }

pub unsafe extern "C" fn compat_sys_aarch32_fallocate(
    fd: c_int, mode: c_int, offset_hi: u32, offset_lo: u32,
    len_hi: u32, len_lo: u32,
) -> c_long { ksys_fallocate(fd, mode, arg_u64(offset_hi, offset_lo), arg_u64(len_hi, len_lo)) }

// The syscall-table include expands these declarations and designated entries.
// Its contents are supplied by the architecture build.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
