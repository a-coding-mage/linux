/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations from the surrounding kernel code.
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_inode {
    _private: [u8; 0],
}

// CONFIG_FS_VERITY is a build-time C preprocessor condition.
// The corresponding Rust configuration is represented by the cfg attribute.
#[cfg(feature = "CONFIG_FS_VERITY")]
extern "C" {
    // Supplied by the fsverity dependency.
    pub static btrfs_verityops: fsverity_operations;

    pub fn btrfs_drop_verity_items(inode: *mut btrfs_inode) -> ::core::ffi::c_int;
    pub fn btrfs_get_verity_descriptor(
        inode: *mut inode,
        buf: *mut ::core::ffi::c_void,
        buf_size: usize,
    ) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_FS_VERITY")]
#[repr(C)]
pub struct fsverity_operations {
    _private: [u8; 0],
}

#[cfg(not(feature = "CONFIG_FS_VERITY"))]
#[inline]
pub unsafe fn btrfs_drop_verity_items(_inode: *mut btrfs_inode) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_FS_VERITY"))]
#[inline]
pub unsafe fn btrfs_get_verity_descriptor(
    _inode: *mut inode,
    _buf: *mut ::core::ffi::c_void,
    _buf_size: usize,
) -> ::core::ffi::c_int {
    -EPERM
}

// Supplied by the kernel errno dependency; equivalent to C's EPERM macro.
#[cfg(not(feature = "CONFIG_FS_VERITY"))]
const EPERM: ::core::ffi::c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
