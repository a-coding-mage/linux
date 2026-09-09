/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2001 - 2003 Sistina Software
 * Copyright (C) 2004 - 2008 Red Hat, Inc. All rights reserved.
 *
 * kcopyd provides a simple interface for copying an area of one
 * block-device to one or more other block-devices, either synchronous
 * or with an asynchronous completion notification.
 *
 * This file is released under the GPL.
 */

/* C header guard: _LINUX_DM_KCOPYD_H */
/* The declarations below are conditional on the C __KERNEL__ build. */

/* Dependency supplied by linux/dm-io.h. */
#[repr(C)]
pub struct dm_io_region {
    _private: [u8; 0],
}

/* FIXME: make this configurable */
pub const DM_KCOPYD_MAX_REGIONS: usize = 8;

pub const DM_KCOPYD_IGNORE_ERROR: u32 = 1;
pub const DM_KCOPYD_WRITE_SEQ: u32 = 2;

#[repr(C)]
pub struct dm_kcopyd_throttle {
    pub throttle: ::core::ffi::c_uint,
    pub num_io_jobs: ::core::ffi::c_uint,
    pub io_period: ::core::ffi::c_uint,
    pub total_period: ::core::ffi::c_uint,
    pub last_jiffies: ::core::ffi::c_uint,
}

/*
 * C macro DECLARE_DM_KCOPYD_THROTTLE_WITH_MODULE_PARM(name, description):
 * declares a static throttle initialized to { 100, 0, 0, 0, 0 }, registers
 * its throttle field as a uint module parameter with mode 0644, and supplies
 * the module parameter description.  Module parameters are build-system and
 * kernel-macro facilities and therefore have no file-local Rust equivalent.
 */

/*
 * To use kcopyd you must first create a dm_kcopyd_client object.
 * throttle can be NULL if you don't want any throttling.
 */
#[repr(C)]
pub struct dm_kcopyd_client {
    _private: [u8; 0],
}

pub type dm_kcopyd_notify_fn = unsafe extern "C" fn(
    read_err: ::core::ffi::c_int,
    write_err: ::core::ffi::c_ulong,
    context: *mut ::core::ffi::c_void,
);

extern "C" {
    pub fn dm_kcopyd_client_create(
        throttle: *mut dm_kcopyd_throttle,
    ) -> *mut dm_kcopyd_client;
    pub fn dm_kcopyd_client_destroy(kc: *mut dm_kcopyd_client);
    pub fn dm_kcopyd_client_flush(kc: *mut dm_kcopyd_client);

    /*
     * Submit a copy job to kcopyd.  This is built on top of the
     * previous three fns.
     *
     * read_err is a boolean,
     * write_err is a bitset, with 1 bit for each destination region
     */
    pub fn dm_kcopyd_copy(
        kc: *mut dm_kcopyd_client,
        from: *mut dm_io_region,
        num_dests: ::core::ffi::c_uint,
        dests: *mut dm_io_region,
        flags: ::core::ffi::c_uint,
        f: dm_kcopyd_notify_fn,
        context: *mut ::core::ffi::c_void,
    );

    /*
     * Prepare a callback and submit it via the kcopyd thread.
     *
     * dm_kcopyd_prepare_callback allocates a callback structure and returns it.
     * It must not be called from interrupt context.
     * The returned value should be passed into dm_kcopyd_do_callback.
     *
     * dm_kcopyd_do_callback submits the callback.
     * It may be called from interrupt context.
     * The callback is issued from the kcopyd thread.
     */
    pub fn dm_kcopyd_prepare_callback(
        kc: *mut dm_kcopyd_client,
        f: dm_kcopyd_notify_fn,
        context: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    pub fn dm_kcopyd_do_callback(
        job: *mut ::core::ffi::c_void,
        read_err: ::core::ffi::c_int,
        write_err: ::core::ffi::c_ulong,
    );

    pub fn dm_kcopyd_zero(
        kc: *mut dm_kcopyd_client,
        num_dests: ::core::ffi::c_uint,
        dests: *mut dm_io_region,
        flags: ::core::ffi::c_uint,
        f: dm_kcopyd_notify_fn,
        context: *mut ::core::ffi::c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
