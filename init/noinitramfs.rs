// SPDX-License-Identifier: GPL-2.0-only
/*
 * init/noinitramfs.c
 *
 * Copyright (C) 2006, NXP Semiconductors, All Rights Reserved
 * Author: Jean-Paul Saman <jean-paul.saman@nxp.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_uint};

const KERN_WARNING: &[u8] = b"<4>";
const S_IFCHR: c_uint = 0o020000;
const S_IRUSR: c_uint = 0o400;
const S_IWUSR: c_uint = 0o200;

extern "C" {
    fn usermodehelper_enable();
    fn init_mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn init_mknod(
        filename: *const c_char,
        mode: c_uint,
        dev: c_uint,
    ) -> c_int;
    fn new_encode_dev(dev: c_uint) -> c_uint;
    fn printk(fmt: *const c_char, ...) -> c_int;
}

/*
 * Create a simple rootfs that is similar to the default initramfs
 */
#[allow(non_snake_case)]
unsafe fn default_rootfs() -> c_int {
    let mut err: c_int;

    usermodehelper_enable();
    err = init_mkdir(b"/dev\0".as_ptr() as *const c_char, 0o755);
    if err < 0 {
        return rootfs_error(err);
    }

    err = init_mknod(
        b"/dev/console\0".as_ptr() as *const c_char,
        S_IFCHR | S_IRUSR | S_IWUSR,
        new_encode_dev(((5u32) << 20) | 1u32),
    );
    if err < 0 {
        return rootfs_error(err);
    }

    err = init_mkdir(b"/root\0".as_ptr() as *const c_char, 0o700);
    if err < 0 {
        return rootfs_error(err);
    }

    0
}

unsafe fn rootfs_error(err: c_int) -> c_int {
    let message = b"Failed to create a rootfs\n\0";
    let mut format = [0u8; 3 + 27];
    format[..3].copy_from_slice(KERN_WARNING);
    format[3..].copy_from_slice(message);
    printk(format.as_ptr() as *const c_char);
    err
}

// Equivalent of rootfs_initcall(default_rootfs); registration is provided by
// the surrounding kernel build and initialization infrastructure.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
