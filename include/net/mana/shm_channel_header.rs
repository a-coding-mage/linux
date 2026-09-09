/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright (c) 2021, Microsoft Corporation. */

pub const SMC_APERTURE_BITS: usize = 256;
pub const SMC_BASIC_UNIT: usize = core::mem::size_of::<u32>();
pub const SMC_APERTURE_DWORDS: usize =
    SMC_APERTURE_BITS / (SMC_BASIC_UNIT * 8);
pub const SMC_LAST_DWORD: usize = SMC_APERTURE_DWORDS - 1;
pub const SMC_APERTURE_SIZE: usize = SMC_APERTURE_BITS / 8;

// External dependency supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct shm_channel {
    pub dev: *mut device,
    pub base: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn mana_smc_init(
        sc: *mut shm_channel,
        dev: *mut device,
        base: *mut core::ffi::c_void,
    );

    pub fn mana_smc_setup_hwc(
        sc: *mut shm_channel,
        reset_vf: bool,
        eq_addr: u64,
        cq_addr: u64,
        rq_addr: u64,
        sq_addr: u64,
        eq_msix_index: u32,
    ) -> i32;

    pub fn mana_smc_teardown_hwc(sc: *mut shm_channel, reset_vf: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
