/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations corresponding to the C header's externally supplied types.
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct extent_state;
#[repr(C)]
pub struct kiocb;
#[repr(C)]
pub struct iov_iter;
#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct folio;
#[repr(C)]
pub struct page;
#[repr(C)]
pub struct btrfs_ioctl_encoded_io_args;
#[repr(C)]
pub struct btrfs_drop_extents_args;
#[repr(C)]
pub struct btrfs_inode;
#[repr(C)]
pub struct btrfs_root;
#[repr(C)]
pub struct btrfs_path;
#[repr(C)]
pub struct btrfs_replace_extent_info;
#[repr(C)]
pub struct btrfs_trans_handle;
#[repr(C)]
pub struct file_operations;

unsafe extern "C" {
    pub static btrfs_file_operations: file_operations;

    pub fn btrfs_sync_file(file: *mut file, start: i64, end: i64, datasync: i32) -> i32;
    pub fn btrfs_drop_extents(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        inode: *mut btrfs_inode,
        args: *mut btrfs_drop_extents_args,
    ) -> i32;
    pub fn btrfs_replace_file_extents(
        inode: *mut btrfs_inode,
        path: *mut btrfs_path,
        start: u64,
        end: u64,
        extent_info: *mut btrfs_replace_extent_info,
        trans_out: *mut *mut btrfs_trans_handle,
    ) -> i32;
    pub fn btrfs_mark_extent_written(
        trans: *mut btrfs_trans_handle,
        inode: *mut btrfs_inode,
        start: u64,
        end: u64,
    ) -> i32;
    pub fn btrfs_do_write_iter(
        iocb: *mut kiocb,
        from: *mut iov_iter,
        encoded: *const btrfs_ioctl_encoded_io_args,
    ) -> isize;
    pub fn btrfs_release_file(inode: *mut inode, file: *mut file) -> i32;
    pub fn btrfs_dirty_folio(
        inode: *mut btrfs_inode,
        folio: *mut folio,
        pos: i64,
        write_bytes: usize,
        cached: *mut *mut extent_state,
        noreserve: bool,
    ) -> i32;
    pub fn btrfs_fdatawrite_range(inode: *mut btrfs_inode, start: i64, end: i64) -> i32;
    pub fn btrfs_check_nocow_lock(
        inode: *mut btrfs_inode,
        pos: i64,
        write_bytes: *mut usize,
        nowait: bool,
    ) -> i32;
    pub fn btrfs_check_nocow_unlock(inode: *mut btrfs_inode);
    pub fn btrfs_find_delalloc_in_range(
        inode: *mut btrfs_inode,
        start: u64,
        end: u64,
        cached_state: *mut *mut extent_state,
        delalloc_start_ret: *mut u64,
        delalloc_end_ret: *mut u64,
    ) -> bool;
    pub fn btrfs_write_check(iocb: *mut kiocb, count: usize) -> i32;
    pub fn btrfs_buffered_write(iocb: *mut kiocb, i: *mut iov_iter) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
