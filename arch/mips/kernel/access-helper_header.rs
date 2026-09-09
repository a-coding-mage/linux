/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <linux/uaccess.h>

extern "C" {
    fn get_user(value: *mut usize, ptr: *const usize) -> i32;
    fn get_kernel_nofault(value: *mut usize, ptr: *const usize) -> i32;
    fn get_user_u16(value: *mut u16, ptr: *const u16) -> i32;
    fn get_kernel_nofault_u16(value: *mut u16, ptr: *const u16) -> i32;
    fn get_user_u32(value: *mut u32, ptr: *const u32) -> i32;
    fn get_kernel_nofault_u32(value: *mut u32, ptr: *const u32) -> i32;
}

#[inline]
unsafe fn __get_addr(a: *mut usize, p: *mut usize, user: bool) -> i32 {
    if user {
        get_user(a, p as *const usize)
    } else {
        get_kernel_nofault(a, p as *const usize)
    }
}

#[inline]
unsafe fn __get_inst16(i: *mut u16, p: *mut u16, user: bool) -> i32 {
    if user {
        get_user_u16(i, p as *const u16)
    } else {
        get_kernel_nofault_u16(i, p as *const u16)
    }
}

#[inline]
unsafe fn __get_inst32(i: *mut u32, p: *mut u32, user: bool) -> i32 {
    if user {
        get_user_u32(i, p as *const u32)
    } else {
        get_kernel_nofault_u32(i, p as *const u32)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
