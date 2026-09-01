/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the C header's dependency on <linux/compiler.h>. */
unsafe extern "C" {
    pub static mut __user_addr_min: *mut core::ffi::c_void;
    pub static mut __user_addr_max: *mut core::ffi::c_void;
}

/* C put_user(x, ptr): checks the user pointer, WRITE_ONCEs x through ptr, and returns 0. */
#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => {{
        let __pu_ptr = $ptr;
        __chk_user_ptr(__pu_ptr);
        unsafe {
            core::ptr::write_volatile(__pu_ptr, $x);
        }
        0
    }};
}

/* C get_user(x, ptr): checks the user pointer, READ_ONCEs through ptr into x, and returns 0. */
#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => {{
        let __pu_ptr = $ptr;
        __chk_user_ptr(__pu_ptr);
        unsafe {
            $x = core::ptr::read_volatile(__pu_ptr);
        }
        0
    }};
}

pub unsafe fn volatile_memcpy(
    mut to: *mut core::ffi::c_char,
    mut from: *const core::ffi::c_char,
    mut n: core::ffi::c_ulong,
) {
    while n != 0 {
        n -= 1;
        unsafe {
            core::ptr::write_volatile(to, core::ptr::read_volatile(from));
            to = to.add(1);
            from = from.add(1);
        }
    }
}

#[inline]
pub unsafe fn copy_from_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: core::ffi::c_ulong,
) -> core::ffi::c_int {
    unsafe {
        volatile_memcpy(to as *mut core::ffi::c_char, from as *const core::ffi::c_char, n);
    }
    0
}

#[inline]
pub unsafe fn copy_to_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: core::ffi::c_ulong,
) -> core::ffi::c_int {
    unsafe {
        volatile_memcpy(to as *mut core::ffi::c_char, from as *const core::ffi::c_char, n);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
