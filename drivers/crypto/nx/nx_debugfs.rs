// SPDX-License-Identifier: GPL-2.0-only
/*
 * debugfs routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependency intent preserved from the C source:
// linux/device.h, linux/kobject.h, linux/string.h, linux/debugfs.h,
// linux/module.h, linux/init.h, linux/crypto.h, crypto/hash.h, asm/vio.h,
// nx_csbcpb.h, and nx.h

// This implementation is conditional on CONFIG_DEBUG_FS in the original
// source. The surrounding build is expected to provide the corresponding
// configuration and external kernel symbols.

/*
 * debugfs
 *
 * For documentation on these attributes, please see:
 *
 * Documentation/ABI/testing/debugfs-pfo-nx-crypto
 */

pub unsafe fn nx_debugfs_init(drv: *mut nx_crypto_driver) {
    let root: *mut dentry;

    root = debugfs_create_dir(NX_NAME, core::ptr::null_mut());
    (*drv).dfs_root = root;

    debugfs_create_u32(
        b"aes_ops\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.aes_ops.counter,
    );
    debugfs_create_u32(
        b"sha256_ops\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.sha256_ops.counter,
    );
    debugfs_create_u32(
        b"sha512_ops\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.sha512_ops.counter,
    );
    debugfs_create_u64(
        b"aes_bytes\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.aes_bytes.counter,
    );
    debugfs_create_u64(
        b"sha256_bytes\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.sha256_bytes.counter,
    );
    debugfs_create_u64(
        b"sha512_bytes\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.sha512_bytes.counter,
    );
    debugfs_create_u32(
        b"errors\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.errors.counter,
    );
    debugfs_create_u32(
        b"last_error\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.last_error.counter,
    );
    debugfs_create_u32(
        b"last_error_pid\0".as_ptr() as *const i8,
        S_IRUSR | S_IRGRP | S_IROTH,
        root,
        &mut (*drv).stats.last_error_pid.counter,
    );
}

pub unsafe fn nx_debugfs_fini(drv: *mut nx_crypto_driver) {
    debugfs_remove_recursive((*drv).dfs_root);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
