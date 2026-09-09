/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PowerPC 4xx related functions
 *
 * Copyright 2007 IBM Corporation.
 * Josh Boyer <jwboyer@linux.vnet.ibm.com>
 */

// Translated from the C header. The declarations below are provided by
// external implementation units.
unsafe extern "C" {
    pub fn ibm4xx_sdram_fixup_memsize();
    pub fn ibm440spe_fixup_memsize();
    pub fn ibm4xx_denali_fixup_memsize();
    pub fn ibm44x_dbcr_reset();
    pub fn ibm4xx_quiesce_eth(emac0: *mut u32, emac1: *mut u32);
    pub fn ibm4xx_fixup_ebc_ranges(ebc: *const core::ffi::c_char);

    pub fn ibm440gp_fixup_clocks(sys_clk: core::ffi::c_uint, ser_clk: core::ffi::c_uint);
    pub fn ibm440ep_fixup_clocks(
        sys_clk: core::ffi::c_uint,
        ser_clk: core::ffi::c_uint,
        tmr_clk: core::ffi::c_uint,
    );
    pub fn ibm440gx_fixup_clocks(
        sys_clk: core::ffi::c_uint,
        ser_clk: core::ffi::c_uint,
        tmr_clk: core::ffi::c_uint,
    );
    pub fn ibm440spe_fixup_clocks(
        sys_clk: core::ffi::c_uint,
        ser_clk: core::ffi::c_uint,
        tmr_clk: core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
