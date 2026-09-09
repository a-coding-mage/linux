/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2021 Xillybus Ltd, http://www.xillybus.com
 *
 * Header file for the Xillybus class
 */

use core::ffi::{c_char, c_int, c_void};

// Types supplied by the Linux kernel headers included by the original C header.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

extern "C" {
    pub fn xillybus_init_chrdev(
        dev: *mut device,
        fops: *const file_operations,
        owner: *mut module,
        private_data: *mut c_void,
        idt: *mut u8,
        len: u32,
        num_nodes: c_int,
        prefix: *const c_char,
        enumerate: bool,
    ) -> c_int;

    pub fn xillybus_cleanup_chrdev(
        private_data: *mut c_void,
        dev: *mut device,
    );

    pub fn xillybus_find_inode(
        inode: *mut inode,
        private_data: *mut *mut c_void,
        index: *mut c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
