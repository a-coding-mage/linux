// SPDX-License-Identifier: GPL-2.0-or-later
/* FS-Cache statistics viewing interface
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// FSCACHE_DEBUG_LEVEL is set to CACHE in the C source.
// The Linux kernel headers and "internal.h" provide the declarations used here.

use core::ffi::c_void;

extern "C" {
    fn proc_symlink(
        name: *const core::ffi::c_char,
        parent: *mut c_void,
        dest: *const core::ffi::c_char,
    ) -> *mut c_void;
    fn proc_create_seq(
        name: *const core::ffi::c_char,
        mode: u32,
        parent: *mut c_void,
        ops: *const c_void,
    ) -> *mut c_void;
    fn remove_proc_entry(name: *const core::ffi::c_char, parent: *mut c_void);
    fn remove_proc_subtree(name: *const core::ffi::c_char, parent: *mut c_void);

    static fscache_caches_seq_ops: c_void;
    static fscache_volumes_seq_ops: c_void;
    static fscache_cookies_seq_ops: c_void;
}

const S_IFREG: u32 = 0o100000;
const ENOMEM: i32 = 12;

/*
 * Add files to /proc/fs/netfs/.
 */
#[no_mangle]
pub unsafe extern "C" fn fscache_proc_init() -> i32 {
    if proc_symlink(
        b"fs/fscache\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null_mut(),
        b"netfs\0".as_ptr() as *const core::ffi::c_char,
    )
    .is_null()
    {
        return -ENOMEM;
    }

    if proc_create_seq(
        b"fs/netfs/caches\0".as_ptr() as *const core::ffi::c_char,
        S_IFREG | 0o444,
        core::ptr::null_mut(),
        &fscache_caches_seq_ops as *const c_void,
    )
    .is_null()
    {
        remove_proc_entry(
            b"fs/fscache\0".as_ptr() as *const core::ffi::c_char,
            core::ptr::null_mut(),
        );
        return -ENOMEM;
    }

    if proc_create_seq(
        b"fs/netfs/volumes\0".as_ptr() as *const core::ffi::c_char,
        S_IFREG | 0o444,
        core::ptr::null_mut(),
        &fscache_volumes_seq_ops as *const c_void,
    )
    .is_null()
    {
        remove_proc_entry(
            b"fs/fscache\0".as_ptr() as *const core::ffi::c_char,
            core::ptr::null_mut(),
        );
        return -ENOMEM;
    }

    if proc_create_seq(
        b"fs/netfs/cookies\0".as_ptr() as *const core::ffi::c_char,
        S_IFREG | 0o444,
        core::ptr::null_mut(),
        &fscache_cookies_seq_ops as *const c_void,
    )
    .is_null()
    {
        remove_proc_entry(
            b"fs/fscache\0".as_ptr() as *const core::ffi::c_char,
            core::ptr::null_mut(),
        );
        return -ENOMEM;
    }

    0
}

/*
 * Clean up the /proc/fs/fscache symlink.
 */
#[no_mangle]
pub unsafe extern "C" fn fscache_proc_cleanup() {
    remove_proc_subtree(
        b"fs/fscache\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
