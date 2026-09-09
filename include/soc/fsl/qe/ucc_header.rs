/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2006 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Authors:  Shlomi Gridish <gridish@freescale.com>
 *           Li Yang <leoli@freescale.com>
 *
 * Description:
 * Internal header file for UCC unit routines.
 */

// Dependencies supplied by the surrounding QE implementation:
// soc/fsl/qe/immap_qe.h and soc/fsl/qe/qe.h

pub const STATISTICS: bool = true;

pub const UCC_MAX_NUM: u32 = 8;

/* Slow or fast type for UCCs. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ucc_speed_type {
    UCC_SPEED_TYPE_FAST = UCC_GUEMR_MODE_FAST_RX | UCC_GUEMR_MODE_FAST_TX,
    UCC_SPEED_TYPE_SLOW = UCC_GUEMR_MODE_SLOW_RX | UCC_GUEMR_MODE_SLOW_TX,
}

unsafe extern "C" {
    /*
     * ucc_set_type
     * Sets UCC to slow or fast mode.
     *
     * ucc_num - (In) number of UCC (0-7).
     * speed   - (In) slow or fast mode for UCC.
     */
    pub fn ucc_set_type(ucc_num: core::ffi::c_uint, speed: ucc_speed_type) -> core::ffi::c_int;

    pub fn ucc_set_qe_mux_mii_mng(ucc_num: core::ffi::c_uint) -> core::ffi::c_int;

    pub fn ucc_set_qe_mux_rxtx(
        ucc_num: core::ffi::c_uint,
        clock: qe_clock,
        mode: comm_dir,
    ) -> core::ffi::c_int;
    pub fn ucc_set_tdm_rxtx_clk(
        tdm_num: core::ffi::c_uint,
        clock: qe_clock,
        mode: comm_dir,
    ) -> core::ffi::c_int;
    pub fn ucc_set_tdm_rxtx_sync(
        tdm_num: core::ffi::c_uint,
        clock: qe_clock,
        mode: comm_dir,
    ) -> core::ffi::c_int;

    pub fn ucc_mux_set_grant_tsa_bkpt(
        ucc_num: core::ffi::c_uint,
        set: core::ffi::c_int,
        mask: u32,
    ) -> core::ffi::c_int;
}

/* QE MUX clock routing for UCC. */
#[inline]
pub unsafe fn ucc_set_qe_mux_grant(ucc_num: core::ffi::c_uint, set: core::ffi::c_int) -> core::ffi::c_int {
    unsafe { ucc_mux_set_grant_tsa_bkpt(ucc_num, set, QE_CMXUCR_GRANT) }
}

#[inline]
pub unsafe fn ucc_set_qe_mux_tsa(ucc_num: core::ffi::c_uint, set: core::ffi::c_int) -> core::ffi::c_int {
    unsafe { ucc_mux_set_grant_tsa_bkpt(ucc_num, set, QE_CMXUCR_TSA) }
}

#[inline]
pub unsafe fn ucc_set_qe_mux_bkpt(ucc_num: core::ffi::c_uint, set: core::ffi::c_int) -> core::ffi::c_int {
    unsafe { ucc_mux_set_grant_tsa_bkpt(ucc_num, set, QE_CMXUCR_BKPT) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
