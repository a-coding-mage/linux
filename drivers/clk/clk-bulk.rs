// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2017 NXP
 *
 * Dong Aisheng <aisheng.dong@nxp.com>
 */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const c_char,
    pub clk: *mut clk,
}

extern "C" {
    fn of_property_read_string_index(
        np: *mut device_node,
        propname: *const c_char,
        index: i32,
        output: *mut *const c_char,
    ) -> i32;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn of_clk_get_parent_count(np: *mut device_node) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const c_char, ...);
    fn clk_put(clk: *mut clk);
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_unprepare(clk: *mut clk);
    fn clk_prepare(clk: *mut clk) -> i32;
    fn clk_disable(clk: *mut clk);
    fn clk_enable(clk: *mut clk) -> i32;
    fn pr_err(fmt: *const c_char, ...);
    fn dev_of_node(dev: *mut device) -> *mut device_node;
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

const ENOMEM: i32 = 12;
const ENOENT: i32 = 2;

#[inline]
unsafe fn is_err(ptr: *mut clk) -> bool {
    (ptr as isize) >= -4095 && (ptr as isize) < 0
}

#[inline]
unsafe fn ptr_err(ptr: *mut clk) -> i32 {
    ptr as isize as i32
}

unsafe fn of_clk_bulk_get(
    dev: *mut device,
    np: *mut device_node,
    num_clks: i32,
    clks: *mut clk_bulk_data,
) -> i32 {
    let mut ret: i32;
    let mut i: i32;

    i = 0;
    while i < num_clks {
        (*clks.add(i as usize)).id = core::ptr::null();
        (*clks.add(i as usize)).clk = core::ptr::null_mut();
        i += 1;
    }

    i = 0;
    while i < num_clks {
        of_property_read_string_index(
            np,
            b"clock-names\0".as_ptr() as *const c_char,
            i,
            &mut (*clks.add(i as usize)).id,
        );
        (*clks.add(i as usize)).clk = of_clk_get(np, i);
        if is_err((*clks.add(i as usize)).clk) {
            ret = ptr_err((*clks.add(i as usize)).clk);
            dev_err_probe(
                dev,
                ret,
                b"%pOF: Failed to get clk index: %d (%s)\n\0".as_ptr() as *const c_char,
                np,
                i,
                (*clks.add(i as usize)).id,
            );
            (*clks.add(i as usize)).clk = core::ptr::null_mut();
            break;
        }
        i += 1;
    }

    if i == num_clks {
        return 0;
    }

    clk_bulk_put(i, clks);
    ret
}

unsafe fn of_clk_bulk_get_all(
    dev: *mut device,
    np: *mut device_node,
    clks: *mut *mut clk_bulk_data,
) -> i32 {
    let num_clks = of_clk_get_parent_count(np);
    if num_clks == 0 {
        return 0;
    }

    let clk_bulk = kmalloc(
        (num_clks as usize) * core::mem::size_of::<clk_bulk_data>(),
        0,
    ) as *mut clk_bulk_data;
    if clk_bulk.is_null() {
        return -ENOMEM;
    }

    let ret = of_clk_bulk_get(dev, np, num_clks, clk_bulk);
    if ret != 0 {
        kfree(clk_bulk as *mut c_void);
        return ret;
    }

    *clks = clk_bulk;
    num_clks
}

#[no_mangle]
pub unsafe extern "C" fn clk_bulk_put(mut num_clks: i32, clks: *mut clk_bulk_data) {
    while {
        num_clks -= 1;
        num_clks >= 0
    } {
        clk_put((*clks.add(num_clks as usize)).clk);
        (*clks.add(num_clks as usize)).clk = core::ptr::null_mut();
    }
}

unsafe fn __clk_bulk_get(
    dev: *mut device,
    num_clks: i32,
    clks: *mut clk_bulk_data,
    optional: bool,
) -> i32 {
    let mut i = 0;
    while i < num_clks {
        (*clks.add(i as usize)).clk = core::ptr::null_mut();
        i += 1;
    }

    i = 0;
    while i < num_clks {
        (*clks.add(i as usize)).clk = clk_get(dev, (*clks.add(i as usize)).id);
        if is_err((*clks.add(i as usize)).clk) {
            let ret = ptr_err((*clks.add(i as usize)).clk);
            (*clks.add(i as usize)).clk = core::ptr::null_mut();
            if ret == -ENOENT && optional {
                i += 1;
                continue;
            }
            dev_err_probe(
                dev,
                ret,
                b"Failed to get clk '%s'\n\0".as_ptr() as *const c_char,
                (*clks.add(i as usize)).id,
            );
            clk_bulk_put(i, clks);
            return ret;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn clk_bulk_get(
    dev: *mut device,
    num_clks: i32,
    clks: *mut clk_bulk_data,
) -> i32 {
    __clk_bulk_get(dev, num_clks, clks, false)
}

#[no_mangle]
pub unsafe extern "C" fn clk_bulk_get_optional(
    dev: *mut device,
    num_clks: i32,
    clks: *mut clk_bulk_data,
) -> i32 {
    __clk_bulk_get(dev, num_clks, clks, true)
}

#[no_mangle]
pub unsafe extern "C" fn clk_bulk_put_all(num_clks: i32, clks: *mut clk_bulk_data) {
    if clks.is_null() || is_err(clks as *mut clk) {
        return;
    }
    clk_bulk_put(num_clks, clks);
    kfree(clks as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn clk_bulk_get_all(
    dev: *mut device,
    clks: *mut *mut clk_bulk_data,
) -> i32 {
    let np = dev_of_node(dev);
    if np.is_null() {
        return 0;
    }
    of_clk_bulk_get_all(dev, np, clks)
}

// CONFIG_HAVE_CLK_PREPARE controls availability of the following functions.

/// clk_bulk_unprepare - undo preparation of a set of clock sources
/// @num_clks: the number of clk_bulk_data
/// @clks: the clk_bulk_data table being unprepared
///
/// clk_bulk_unprepare may sleep, which differentiates it from clk_bulk_disable.
/// Returns 0 on success, -EERROR otherwise.
#[no_mangle]
pub unsafe extern "C" fn clk_bulk_unprepare(
    mut num_clks: i32,
    clks: *const clk_bulk_data,
) {
    while {
        num_clks -= 1;
        num_clks >= 0
    } {
        clk_unprepare((*clks.add(num_clks as usize)).clk);
    }
}

/// clk_bulk_prepare - prepare a set of clocks
/// @num_clks: the number of clk_bulk_data
/// @clks: the clk_bulk_data table being prepared
///
/// clk_bulk_prepare may sleep, which differentiates it from clk_bulk_enable.
/// Returns 0 on success, -EERROR otherwise.
#[no_mangle]
pub unsafe extern "C" fn clk_bulk_prepare(
    num_clks: i32,
    clks: *const clk_bulk_data,
) -> i32 {
    let mut i = 0;
    while i < num_clks {
        let ret = clk_prepare((*clks.add(i as usize)).clk);
        if ret != 0 {
            pr_err(
                b"Failed to prepare clk '%s': %d\n\0".as_ptr() as *const c_char,
                (*clks.add(i as usize)).id,
                ret,
            );
            clk_bulk_unprepare(i, clks);
            return ret;
        }
        i += 1;
    }
    0
}

/// clk_bulk_disable - gate a set of clocks
/// @num_clks: the number of clk_bulk_data
/// @clks: the clk_bulk_data table being gated
///
/// clk_bulk_disable must not sleep, which differentiates it from
/// clk_bulk_unprepare. clk_bulk_disable must be called before
/// clk_bulk_unprepare.
#[no_mangle]
pub unsafe extern "C" fn clk_bulk_disable(mut num_clks: i32, clks: *const clk_bulk_data) {
    while {
        num_clks -= 1;
        num_clks >= 0
    } {
        clk_disable((*clks.add(num_clks as usize)).clk);
    }
}

/// clk_bulk_enable - ungate a set of clocks
/// @num_clks: the number of clk_bulk_data
/// @clks: the clk_bulk_data table being ungated
///
/// clk_bulk_enable must not sleep
/// Returns 0 on success, -EERROR otherwise.
#[no_mangle]
pub unsafe extern "C" fn clk_bulk_enable(
    num_clks: i32,
    clks: *const clk_bulk_data,
) -> i32 {
    let mut i = 0;
    while i < num_clks {
        let ret = clk_enable((*clks.add(i as usize)).clk);
        if ret != 0 {
            pr_err(
                b"Failed to enable clk '%s': %d\n\0".as_ptr() as *const c_char,
                (*clks.add(i as usize)).id,
                ret,
            );
            clk_bulk_disable(i, clks);
            return ret;
        }
        i += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
