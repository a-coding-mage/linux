/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Virtual PTP 1588 clock for use with KVM guests
 *
 * Copyright (C) 2017 Red Hat Inc.
 */

// Declarations supplied by the corresponding Linux headers.

#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}

#[repr(C)]
pub enum clocksource_ids {}

extern "C" {
    pub fn kvm_arch_ptp_init() -> ::core::ffi::c_int;
    pub fn kvm_arch_ptp_exit();
    pub fn kvm_arch_ptp_get_clock(ts: *mut timespec64) -> ::core::ffi::c_int;
    pub fn kvm_arch_ptp_get_crosststamp(
        cycle: *mut u64,
        tspec: *mut timespec64,
        cs_id: *mut clocksource_ids,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
