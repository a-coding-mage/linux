/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 Synopsys, Inc. and/or its affiliates.
 * Synopsys DesignWare XPCS helpers
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* AN mode */
pub const DW_AN_C73: i32 = 1;
pub const DW_AN_C37_SGMII: i32 = 2;
pub const DW_2500BASEX: i32 = 3;
pub const DW_AN_C37_1000BASEX: i32 = 4;
pub const DW_10GBASER: i32 = 5;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dw_xpcs_pcs_id {
    DW_XPCS_ID_NATIVE = 0,
    NXP_SJA1105_XPCS_ID = 0x00000010,
    NXP_SJA1110_XPCS_ID = 0x00000020,
    DW_XPCS_ID = 0x7996ced0,
    DW_XPCS_ID_MASK = 0xffffffff,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dw_xpcs_pma_id {
    DW_XPCS_PMA_ID_NATIVE = 0,
    DW_XPCS_PMA_GEN1_3G_ID,
    DW_XPCS_PMA_GEN2_3G_ID,
    DW_XPCS_PMA_GEN2_6G_ID,
    DW_XPCS_PMA_GEN4_3G_ID,
    DW_XPCS_PMA_GEN4_6G_ID,
    DW_XPCS_PMA_GEN5_10G_ID,
    DW_XPCS_PMA_GEN5_12G_ID,
    WX_TXGBE_XPCS_PMA_10G_ID = 0xfc806000,
    /* Meta Platforms OUI 88:25:08, model 0, revision 0 */
    MP_FBNIC_XPCS_PMA_100G_ID = 0x46904000,
}

#[repr(C)]
pub struct dw_xpcs_info {
    pub pcs: u32,
    pub pma: u32,
}

#[repr(C)]
pub struct dw_xpcs;

extern "C" {
    pub fn xpcs_to_phylink_pcs(xpcs: *mut dw_xpcs) -> *mut phylink_pcs;
    pub fn xpcs_get_an_mode(xpcs: *mut dw_xpcs, interface: phy_interface_t) -> i32;
    pub fn xpcs_config_eee_mult_fact(xpcs: *mut dw_xpcs, mult_fact: u8);
    pub fn xpcs_create_mdiodev(bus: *mut mii_bus, addr: i32) -> *mut dw_xpcs;
    pub fn xpcs_create_fwnode(fwnode: *mut fwnode_handle) -> *mut dw_xpcs;
    pub fn xpcs_destroy(xpcs: *mut dw_xpcs);

    pub fn xpcs_create_pcs_mdiodev(bus: *mut mii_bus, addr: i32) -> *mut phylink_pcs;
    pub fn xpcs_destroy_pcs(pcs: *mut phylink_pcs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
