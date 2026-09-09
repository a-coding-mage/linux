/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ssip_slave.h
 *
 * SSIP slave support header file
 *
 * Copyright (C) 2010 Nokia Corporation. All rights reserved.
 *
 * Contact: Carlos Chinea <carlos.chinea@nokia.com>
 */

// Dependency supplied by <linux/hsi/hsi.h>.

#[inline]
pub unsafe fn ssip_slave_put_master(master: *mut hsi_client) {
}

extern "C" {
    pub fn ssip_slave_get_master(slave: *mut hsi_client) -> *mut hsi_client;
    pub fn ssip_slave_start_tx(master: *mut hsi_client) -> ::core::ffi::c_int;
    pub fn ssip_slave_stop_tx(master: *mut hsi_client) -> ::core::ffi::c_int;
    pub fn ssip_reset_event(master: *mut hsi_client);

    pub fn ssip_slave_running(master: *mut hsi_client) -> ::core::ffi::c_int;
    pub fn ssi_waketest(cl: *mut hsi_client, enable: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
