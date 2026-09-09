/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  include/linux/clkdev.h
 *
 *  Copyright (C) 2008 Russell King.
 *
 * Helper for the clk API to assist looking up a struct clk.
 */

// Dependency supplied by the corresponding Linux headers.
use crate::list_head;

#[repr(C)]
pub struct clk;

#[repr(C)]
pub struct clk_hw;

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct clk_lookup {
    pub node: list_head,
    pub dev_id: *const core::ffi::c_char,
    pub con_id: *const core::ffi::c_char,
    pub clk: *mut clk,
    pub clk_hw: *mut clk_hw,
}

#[macro_export]
macro_rules! CLKDEV_INIT {
    ($d:expr, $n:expr, $c:expr) => {
        $crate::clk_lookup {
            dev_id: $d,
            con_id: $n,
            clk: $c,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

unsafe extern "C" {
    pub fn clkdev_add(cl: *mut clk_lookup);
    pub fn clkdev_drop(cl: *mut clk_lookup);

    pub fn clkdev_create(
        clk: *mut clk,
        con_id: *const core::ffi::c_char,
        dev_fmt: *const core::ffi::c_char,
        ...,
    ) -> *mut clk_lookup;
    pub fn clkdev_hw_create(
        hw: *mut clk_hw,
        con_id: *const core::ffi::c_char,
        dev_fmt: *const core::ffi::c_char,
        ...,
    ) -> *mut clk_lookup;

    pub fn clkdev_add_table(cl: *mut clk_lookup, size: usize);
    pub fn clk_add_alias(
        alias: *const core::ffi::c_char,
        alias_dev_name: *const core::ffi::c_char,
        id: *const core::ffi::c_char,
        dev: *mut device,
    ) -> core::ffi::c_int;

    pub fn clk_register_clkdev(
        clk: *mut clk,
        con_id: *const core::ffi::c_char,
        dev_id: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn clk_hw_register_clkdev(
        hw: *mut clk_hw,
        con_id: *const core::ffi::c_char,
        dev_id: *const core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn devm_clk_hw_register_clkdev(
        dev: *mut device,
        hw: *mut clk_hw,
        con_id: *const core::ffi::c_char,
        dev_id: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
