/* SPDX-License-Identifier: GPL-2.0+ */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zl3073x_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn zl3073x_flash_mode_enter(
        zldev: *mut zl3073x_dev,
        util_ptr: *const c_void,
        util_size: usize,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn zl3073x_flash_mode_leave(
        zldev: *mut zl3073x_dev,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn zl3073x_flash_page(
        zldev: *mut zl3073x_dev,
        component: *const c_char,
        page: u32,
        addr: u32,
        data: *const c_void,
        size: usize,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn zl3073x_flash_page_copy(
        zldev: *mut zl3073x_dev,
        component: *const c_char,
        src_page: u32,
        dst_page: u32,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn zl3073x_flash_sectors(
        zldev: *mut zl3073x_dev,
        component: *const c_char,
        page: u32,
        addr: u32,
        data: *const c_void,
        size: usize,
        extack: *mut netlink_ext_ack,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
