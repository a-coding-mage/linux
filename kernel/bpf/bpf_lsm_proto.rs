// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2025 Google LLC.
 */

// Dependencies corresponding to <linux/fs.h> and <linux/bpf_lsm.h> are
// supplied externally.

/// Opaque declaration corresponding to `struct file` from the Linux headers.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

/*
 * Strong definition of the mmap_file() BPF LSM hook. The __nullable suffix on
 * the struct file pointer parameter name marks it as PTR_MAYBE_NULL. This
 * explicitly enforces that BPF LSM programs check for NULL before attempting
 * to dereference it.
 */
#[no_mangle]
pub unsafe extern "C" fn bpf_lsm_mmap_file(
    file__nullable: *mut file,
    reqprot: kernel::ffi::c_ulong,
    prot: kernel::ffi::c_ulong,
    flags: kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
