/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP4 Power/Reset Management (PRM) function prototypes
 *
 * Copyright (C) 2010 Nokia Corporation
 * Copyright (C) 2011 Texas Instruments, Inc.
 * Paul Walmsley
 */

pub const PRM_INSTANCE_UNKNOWN: s32 = -1;

extern "C" {
    pub fn omap4_prmst_get_prm_dev_inst() -> s32;
    pub fn omap4_prminst_set_prm_dev_inst(dev_inst: s32);

    /*
     * In an ideal world, we would not export these low-level functions,
     * but this will probably take some time to fix properly
     */
    pub fn omap4_prminst_read_inst_reg(part: u8, inst: s16, idx: u16) -> u32;
    pub fn omap4_prminst_write_inst_reg(val: u32, part: u8, inst: s16, idx: u16);
    pub fn omap4_prminst_rmw_inst_reg_bits(
        mask: u32,
        bits: u32,
        part: u8,
        inst: s16,
        idx: u16,
    ) -> u32;

    pub fn omap4_prminst_global_warm_sw_reset();

    pub fn omap4_prminst_is_hardreset_asserted(
        shift: u8,
        part: u8,
        inst: s16,
        rstctrl_offs: u16,
    ) -> ::core::ffi::c_int;
    pub fn omap4_prminst_assert_hardreset(
        shift: u8,
        part: u8,
        inst: s16,
        rstctrl_offs: u16,
    ) -> ::core::ffi::c_int;
    pub fn omap4_prminst_deassert_hardreset(
        shift: u8,
        st_shift: u8,
        part: u8,
        inst: s16,
        rstctrl_offs: u16,
        rstst_offs: u16,
    ) -> ::core::ffi::c_int;

    pub fn omap_prm_base_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
