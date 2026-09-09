/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * posix-clock.h - support for dynamic clock devices
 *
 * Copyright (C) 2010 OMICRON electronics GmbH
 */

// Dependencies supplied by other translation units.

pub struct posix_clock;
pub struct posix_clock_context;

#[repr(C)]
pub struct posix_clock_operations {
    pub owner: *mut module,

    pub clock_adjtime: Option<unsafe extern "C" fn(
        pc: *mut posix_clock,
        tx: *mut __kernel_timex,
    ) -> ::core::ffi::c_int>,

    pub clock_gettime: Option<unsafe extern "C" fn(
        pc: *mut posix_clock,
        ts: *mut timespec64,
    ) -> ::core::ffi::c_int>,

    pub clock_getres: Option<unsafe extern "C" fn(
        pc: *mut posix_clock,
        ts: *mut timespec64,
    ) -> ::core::ffi::c_int>,

    pub clock_settime: Option<unsafe extern "C" fn(
        pc: *mut posix_clock,
        ts: *const timespec64,
    ) -> ::core::ffi::c_int>,

    /* Optional character device methods. */
    pub ioctl: Option<unsafe extern "C" fn(
        pccontext: *mut posix_clock_context,
        cmd: u32,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long>,

    pub open: Option<unsafe extern "C" fn(
        pccontext: *mut posix_clock_context,
        f_mode: fmode_t,
    ) -> ::core::ffi::c_int>,

    pub poll: Option<unsafe extern "C" fn(
        pccontext: *mut posix_clock_context,
        file: *mut file,
        wait: *mut poll_table,
    ) -> __poll_t>,

    pub release: Option<unsafe extern "C" fn(
        pccontext: *mut posix_clock_context,
    ) -> ::core::ffi::c_int>,

    pub read: Option<unsafe extern "C" fn(
        pccontext: *mut posix_clock_context,
        flags: u32,
        buf: *mut ::core::ffi::c_char,
        cnt: usize,
    ) -> ssize_t>,
}

#[repr(C)]
pub struct posix_clock {
    pub ops: posix_clock_operations,
    pub cdev: cdev,
    pub dev: *mut device,
    pub rwsem: rw_semaphore,
    pub zombie: bool,
}

#[repr(C)]
pub struct posix_clock_context {
    pub clk: *mut posix_clock,
    pub fp: *mut file,
    pub private_clkdata: *mut ::core::ffi::c_void,
}

extern "C" {
    pub fn posix_clock_register(clk: *mut posix_clock, dev: *mut device) -> ::core::ffi::c_int;
    pub fn posix_clock_unregister(clk: *mut posix_clock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
