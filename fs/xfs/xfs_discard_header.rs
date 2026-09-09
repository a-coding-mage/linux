/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations corresponding to the C header's opaque structures.
#[repr(C)]
pub struct fstrim_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_busy_extents {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_discard_extents(mp: *mut xfs_mount, busy: *mut xfs_busy_extents);

    // The C declaration uses the kernel __user annotation on `fstrim`.
    pub fn xfs_ioc_trim(mp: *mut xfs_mount, fstrim: *mut fstrim_range) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
