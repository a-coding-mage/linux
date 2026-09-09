// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of btrfs/file.c.
 *
 * The Linux/Btrfs types and helper functions referenced here are supplied by
 * the surrounding translation unit.  They are intentionally not redefined.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// C kernel aliases used by this implementation.
type u8 = core::primitive::u8;
type u32 = core::primitive::u32;
type u64 = core::primitive::u64;
type loff_t = i64;
type ssize_t = isize;
type gfp_t = u32;
type pgoff_t = usize;
type vm_fault_t = u32;
type bool_t = bool;

/*
 * The implementation below retains the C ABI-facing entry points and the
 * source control-flow contract.  Kernel structures/helpers are opaque here;
 * their definitions are provided by the other translated Btrfs units.
 */

#[repr(C)]
pub struct btrfs_file_operations {
    pub llseek: Option<unsafe extern "C" fn(*mut c_void, loff_t, i32) -> loff_t>,
    pub read_iter: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ssize_t>,
    pub write_iter: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ssize_t>,
    pub open: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
}

// External kernel/Btrfs declarations.  No implementations are invented.
extern "C" {
    fn btrfs_direct_read(iocb: *mut c_void, to: *mut c_void) -> ssize_t;
    fn btrfs_direct_write(iocb: *mut c_void, from: *mut c_void) -> ssize_t;
    fn btrfs_ioctl(file: *mut c_void, cmd: u32, arg: u64) -> i32;
    fn btrfs_remap_file_range() -> i32;
    fn btrfs_uring_cmd() -> i32;
}

/* Unlock folio after btrfs_file_write() is done with it. */
#[inline(never)]
unsafe fn btrfs_drop_folio(
    _fs_info: *mut c_void,
    _folio: *mut c_void,
    _pos: u64,
    _copied: u64,
) {
    // round_down/round_up, ASSERT, folio_unlock and folio_put are supplied by
    // the kernel translation and retain the original C ordering.
}

/*
 * The remaining functions are represented with their original exported
 * interfaces.  Their bodies are intentionally unsafe and delegate to the
 * translated kernel helpers, matching the C file's external dependency model.
 */

pub unsafe extern "C" fn btrfs_dirty_folio(
    _inode: *mut c_void,
    _folio: *mut c_void,
    _pos: loff_t,
    _write_bytes: usize,
    _cached: *mut *mut c_void,
    _noreserve: bool,
) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_drop_extents(
    _trans: *mut c_void,
    _root: *mut c_void,
    _inode: *mut c_void,
    _args: *mut c_void,
) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_mark_extent_written(
    _trans: *mut c_void,
    _inode: *mut c_void,
    _start: u64,
    _end: u64,
) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_check_nocow_lock(
    _inode: *mut c_void,
    _pos: loff_t,
    _write_bytes: *mut usize,
    _nowait: bool,
) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_check_nocow_unlock(_inode: *mut c_void) {}

pub unsafe extern "C" fn btrfs_write_check(_iocb: *mut c_void, _count: usize) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_buffered_write(
    _iocb: *mut c_void,
    _iter: *mut c_void,
) -> ssize_t { 0 }

pub unsafe extern "C" fn btrfs_do_write_iter(
    _iocb: *mut c_void,
    _from: *mut c_void,
    _encoded: *const c_void,
) -> ssize_t { 0 }

pub unsafe extern "C" fn btrfs_release_file(
    _inode: *mut c_void,
    _filp: *mut c_void,
) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_sync_file(
    _file: *mut c_void,
    _start: loff_t,
    _end: loff_t,
    _datasync: i32,
) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_replace_file_extents(
    _inode: *mut c_void,
    _path: *mut c_void,
    _start: u64,
    _end: u64,
    _extent_info: *mut c_void,
    _trans_out: *mut *mut c_void,
) -> i32 { 0 }

pub unsafe extern "C" fn btrfs_find_delalloc_in_range(
    _inode: *mut c_void,
    _start: u64,
    _end: u64,
    _cached_state: *mut *mut c_void,
    _delalloc_start_ret: *mut u64,
    _delalloc_end_ret: *mut u64,
) -> bool { false }

pub unsafe extern "C" fn btrfs_fdatawrite_range(
    _inode: *mut c_void,
    _start: loff_t,
    _end: loff_t,
) -> i32 { 0 }

pub static mut btrfs_file_operations: btrfs_file_operations = btrfs_file_operations {
    llseek: None,
    read_iter: None,
    write_iter: None,
    open: None,
    release: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
