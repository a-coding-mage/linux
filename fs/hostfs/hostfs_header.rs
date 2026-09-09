/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by <os.h> and <generated/asm-offsets.h> are expected
// to provide uid_t, gid_t, and loff_t in the surrounding translation unit.

#[repr(C)]
pub struct hostfs_timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct hostfs_iattr {
    pub ia_valid: u32,
    pub ia_mode: u16,
    pub ia_uid: uid_t,
    pub ia_gid: gid_t,
    pub ia_size: loff_t,
    pub ia_atime: hostfs_timespec,
    pub ia_mtime: hostfs_timespec,
    pub ia_ctime: hostfs_timespec,
}

#[repr(C)]
pub struct hostfs_stat {
    pub ino: u64,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub atime: hostfs_timespec,
    pub mtime: hostfs_timespec,
    pub ctime: hostfs_timespec,
    pub btime: hostfs_timespec,
    pub blksize: u32,
    pub blocks: u64,
    pub rdev: hostfs_dev,
    pub dev: hostfs_dev,
}

#[repr(C)]
pub struct hostfs_dev {
    pub maj: u32,
    pub min: u32,
}

extern "C" {
    pub fn stat_file(path: *const core::ffi::c_char, p: *mut hostfs_stat, fd: i32) -> i32;
    pub fn access_file(path: *mut core::ffi::c_char, r: i32, w: i32, x: i32) -> i32;
    pub fn open_file(path: *mut core::ffi::c_char, r: i32, w: i32, append: i32) -> i32;
    pub fn open_dir(path: *mut core::ffi::c_char, err_out: *mut i32) -> *mut core::ffi::c_void;
    pub fn seek_dir(stream: *mut core::ffi::c_void, pos: u64);
    pub fn read_dir(
        stream: *mut core::ffi::c_void,
        pos_out: *mut u64,
        ino_out: *mut u64,
        len_out: *mut i32,
        type_out: *mut u32,
    ) -> *mut core::ffi::c_char;
    pub fn close_file(stream: *mut core::ffi::c_void);
    pub fn replace_file(oldfd: i32, fd: i32) -> i32;
    pub fn close_dir(stream: *mut core::ffi::c_void);
    pub fn read_file(fd: i32, offset: *mut u64, buf: *mut core::ffi::c_char, len: i32) -> i32;
    pub fn write_file(
        fd: i32,
        offset: *mut u64,
        buf: *const core::ffi::c_char,
        len: i32,
    ) -> i32;
    pub fn lseek_file(fd: i32, offset: i64, whence: i32) -> i32;
    pub fn fsync_file(fd: i32, datasync: i32) -> i32;
    pub fn file_create(name: *mut core::ffi::c_char, mode: i32) -> i32;
    pub fn set_attr(file: *const core::ffi::c_char, attrs: *mut hostfs_iattr, fd: i32) -> i32;
    pub fn make_symlink(from: *const core::ffi::c_char, to: *const core::ffi::c_char) -> i32;
    pub fn unlink_file(file: *const core::ffi::c_char) -> i32;
    pub fn do_mkdir(file: *const core::ffi::c_char, mode: i32) -> i32;
    pub fn hostfs_do_rmdir(file: *const core::ffi::c_char) -> i32;
    pub fn do_mknod(file: *const core::ffi::c_char, mode: i32, major: u32, minor: u32) -> i32;
    pub fn link_file(to: *const core::ffi::c_char, from: *const core::ffi::c_char) -> i32;
    pub fn hostfs_do_readlink(
        file: *mut core::ffi::c_char,
        buf: *mut core::ffi::c_char,
        size: i32,
    ) -> i32;
    pub fn rename_file(from: *mut core::ffi::c_char, to: *mut core::ffi::c_char) -> i32;
    pub fn rename2_file(from: *mut core::ffi::c_char, to: *mut core::ffi::c_char, flags: u32) -> i32;
    pub fn do_statfs(
        root: *mut core::ffi::c_char,
        bsize_out: *mut core::ffi::c_long,
        blocks_out: *mut i64,
        bfree_out: *mut i64,
        bavail_out: *mut i64,
        files_out: *mut i64,
        ffree_out: *mut i64,
        fsid_out: *mut core::ffi::c_void,
        fsid_size: i32,
        namelen_out: *mut core::ffi::c_long,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
