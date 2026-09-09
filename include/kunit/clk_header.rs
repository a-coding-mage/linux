/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn clk_get_kunit(
        test: *mut kunit,
        dev: *mut device,
        con_id: *const c_char,
    ) -> *mut clk;

    pub fn of_clk_get_kunit(
        test: *mut kunit,
        np: *mut device_node,
        index: i32,
    ) -> *mut clk;

    pub fn clk_hw_get_clk_kunit(
        test: *mut kunit,
        hw: *mut clk_hw,
        con_id: *const c_char,
    ) -> *mut clk;

    pub fn clk_hw_get_clk_prepared_enabled_kunit(
        test: *mut kunit,
        hw: *mut clk_hw,
        con_id: *const c_char,
    ) -> *mut clk;

    pub fn clk_prepare_enable_kunit(test: *mut kunit, clk: *mut clk) -> i32;

    pub fn clk_hw_register_kunit(
        test: *mut kunit,
        dev: *mut device,
        hw: *mut clk_hw,
    ) -> i32;

    pub fn of_clk_hw_register_kunit(
        test: *mut kunit,
        node: *mut device_node,
        hw: *mut clk_hw,
    ) -> i32;

    pub fn of_clk_add_hw_provider_kunit(
        test: *mut kunit,
        np: *mut device_node,
        get: Option<unsafe extern "C" fn(
            clkspec: *mut of_phandle_args,
            data: *mut c_void,
        ) -> *mut clk_hw>,
        data: *mut c_void,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
