/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * These are internal functions, please use sb_start_{write,pagefault,intwrite}
 * instead.
 */
#[inline]
pub unsafe fn __sb_end_write(sb: *mut super_block, level: ::core::ffi::c_int) {
    percpu_up_read((*sb).s_writers.rw_sem.add((level - 1) as usize));
}

#[inline]
pub unsafe fn __sb_start_write(sb: *mut super_block, level: ::core::ffi::c_int) {
    percpu_down_read_freezable((*sb).s_writers.rw_sem.add((level - 1) as usize), true);
}

#[inline]
pub unsafe fn __sb_start_write_trylock(
    sb: *mut super_block,
    level: ::core::ffi::c_int,
) -> bool {
    percpu_down_read_trylock((*sb).s_writers.rw_sem.add((level - 1) as usize))
}

#[inline]
pub unsafe fn __sb_writers_acquired(sb: *mut super_block, lev: ::core::ffi::c_int) {
    percpu_rwsem_acquire(&mut (*sb).s_writers.rw_sem[(lev - 1) as usize], 1, _THIS_IP_);
}

#[inline]
pub unsafe fn __sb_writers_release(sb: *mut super_block, lev: ::core::ffi::c_int) {
    percpu_rwsem_release(&mut (*sb).s_writers.rw_sem[(lev - 1) as usize], _THIS_IP_);
}

/**
 * __sb_write_started - check if sb freeze level is held
 * @sb: the super we write to
 * @level: the freeze level
 *
 * * > 0 - sb freeze level is held
 * *   0 - sb freeze level is not held
 * * < 0 - !CONFIG_LOCKDEP/LOCK_STATE_UNKNOWN
 */
#[inline]
pub unsafe fn __sb_write_started(sb: *const super_block, level: ::core::ffi::c_int) -> ::core::ffi::c_int {
    lockdep_is_held_type((*sb).s_writers.rw_sem.add((level - 1) as usize), 1)
}

#[inline]
pub unsafe fn sb_write_started(sb: *const super_block) -> bool {
    __sb_write_started(sb, SB_FREEZE_WRITE) != 0
}

#[inline]
pub unsafe fn sb_write_not_started(sb: *const super_block) -> bool {
    __sb_write_started(sb, SB_FREEZE_WRITE) <= 0
}

#[inline]
pub unsafe fn sb_end_write(sb: *mut super_block) {
    __sb_end_write(sb, SB_FREEZE_WRITE);
}

#[inline]
pub unsafe fn sb_end_pagefault(sb: *mut super_block) {
    __sb_end_write(sb, SB_FREEZE_PAGEFAULT);
}

#[inline]
pub unsafe fn sb_end_intwrite(sb: *mut super_block) {
    __sb_end_write(sb, SB_FREEZE_FS);
}

#[inline]
pub unsafe fn sb_start_write(sb: *mut super_block) {
    __sb_start_write(sb, SB_FREEZE_WRITE);
}

// DEFINE_GUARD(super_write, struct super_block *, sb_start_write(_T), sb_end_write(_T))

#[inline]
pub unsafe fn sb_start_write_trylock(sb: *mut super_block) -> bool {
    __sb_start_write_trylock(sb, SB_FREEZE_WRITE)
}

#[inline]
pub unsafe fn sb_start_pagefault(sb: *mut super_block) {
    __sb_start_write(sb, SB_FREEZE_PAGEFAULT);
}

#[inline]
pub unsafe fn sb_start_intwrite(sb: *mut super_block) {
    __sb_start_write(sb, SB_FREEZE_FS);
}

#[inline]
pub unsafe fn sb_start_intwrite_trylock(sb: *mut super_block) -> bool {
    __sb_start_write_trylock(sb, SB_FREEZE_FS)
}

#[inline]
pub unsafe fn sb_rdonly(sb: *const super_block) -> bool {
    (*sb).s_flags & SB_RDONLY != 0
}

#[inline]
pub unsafe fn sb_is_blkdev_sb(sb: *mut super_block) -> bool {
    IS_ENABLED(CONFIG_BLOCK) && sb == blockdev_superblock
}

#[cfg(CONFIG_UNICODE)]
#[inline]
pub unsafe fn sb_encoding(sb: *const super_block) -> *mut unicode_map {
    (*sb).s_encoding
}

#[cfg(CONFIG_UNICODE)]
#[inline]
pub unsafe fn sb_same_encoding(sb1: *const super_block, sb2: *const super_block) -> bool {
    if (*sb1).s_encoding == (*sb2).s_encoding {
        return true;
    }
    !(*sb1).s_encoding.is_null()
        && !(*sb2).s_encoding.is_null()
        && (*(*sb1).s_encoding).version == (*(*sb2).s_encoding).version
        && (*sb1).s_encoding_flags == (*sb2).s_encoding_flags
}

#[cfg(not(CONFIG_UNICODE))]
#[inline]
pub unsafe fn sb_encoding(_sb: *const super_block) -> *mut unicode_map {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_UNICODE))]
#[inline]
pub unsafe fn sb_same_encoding(_sb1: *const super_block, _sb2: *const super_block) -> bool {
    true
}

#[inline]
pub unsafe fn sb_has_encoding(sb: *const super_block) -> bool {
    !sb_encoding(sb).is_null()
}

pub extern "C" fn sb_set_blocksize(sb: *mut super_block, size: ::core::ffi::c_int) -> ::core::ffi::c_int;
pub extern "C" fn sb_min_blocksize(sb: *mut super_block, size: ::core::ffi::c_int) -> ::core::ffi::c_int;

pub extern "C" fn freeze_super(
    super_: *mut super_block,
    who: freeze_holder,
    freeze_owner: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub extern "C" fn thaw_super(
    super_: *mut super_block,
    who: freeze_holder,
    freeze_owner: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;

pub extern "C" fn sb_init_dio_done_wq(sb: *mut super_block) -> ::core::ffi::c_int;

pub enum file {}

pub extern "C" fn fs_bdev_file_open_by_dev(
    dev: dev_t,
    mode: blk_mode_t,
    holder: *mut ::core::ffi::c_void,
    sb: *mut super_block,
) -> *mut file;
pub extern "C" fn fs_bdev_file_open_by_path(
    path: *const ::core::ffi::c_char,
    mode: blk_mode_t,
    holder: *mut ::core::ffi::c_void,
    sb: *mut super_block,
) -> *mut file;
pub extern "C" fn fs_bdev_unregister(bdev_file: *mut file, sb: *mut super_block);
pub extern "C" fn fs_bdev_file_release(bdev_file: *mut file, sb: *mut super_block);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
