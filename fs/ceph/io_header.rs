/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _FS_CEPH_IO_H
// Dependency: <linux/compiler_attributes.h>

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

extern "C" {
    pub fn ceph_start_io_read(inode: *mut inode) -> ::core::ffi::c_int;
    pub fn ceph_end_io_read(inode: *mut inode);
    pub fn ceph_start_io_write(inode: *mut inode) -> ::core::ffi::c_int;
    pub fn ceph_end_io_write(inode: *mut inode);
    pub fn ceph_start_io_direct(inode: *mut inode) -> ::core::ffi::c_int;
    pub fn ceph_end_io_direct(inode: *mut inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
