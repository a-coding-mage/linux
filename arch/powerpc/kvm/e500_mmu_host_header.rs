/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008-2013 Freescale Semiconductor, Inc. All rights reserved.
 */

use core::ffi::c_int;

#[repr(C)]
pub struct kvmppc_vcpu_e500 {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn inval_gtlbe_on_host(
        vcpu_e500: *mut kvmppc_vcpu_e500,
        tlbsel: c_int,
        esel: c_int,
    );

    pub fn e500_mmu_host_init(vcpu_e500: *mut kvmppc_vcpu_e500) -> c_int;
    pub fn e500_mmu_host_uninit(vcpu_e500: *mut kvmppc_vcpu_e500);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
