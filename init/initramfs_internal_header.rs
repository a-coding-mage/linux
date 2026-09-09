// SPDX-License-Identifier: GPL-2.0
// Original header guard: __INITRAMFS_INTERNAL_H__

extern "C" {
    pub fn unpack_to_rootfs(buf: *mut core::ffi::c_char, len: core::ffi::c_ulong)
        -> *mut core::ffi::c_char;
}

pub const CPIO_HDRLEN: core::ffi::c_int = 110;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
