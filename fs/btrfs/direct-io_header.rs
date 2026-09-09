/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: BTRFS_DIRECT_IO_H

// Dependency: <linux/types.h>

#[repr(C)]
pub struct kiocb;

#[repr(C)]
pub struct iov_iter;

// C __init attribute preserved as a comment.
// C __cold attribute preserved as a comment.
unsafe extern "C" {
    pub fn btrfs_init_dio() -> core::ffi::c_int;
    pub fn btrfs_destroy_dio();

    pub fn btrfs_direct_write(
        iocb: *mut kiocb,
        from: *mut iov_iter,
    ) -> isize;
    pub fn btrfs_direct_read(
        iocb: *mut kiocb,
        to: *mut iov_iter,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
