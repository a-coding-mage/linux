/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/bcma/bcma_driver_gmac_cmn.h. */

pub const BCMA_GMAC_CMN_STAG0: u32 = 0x000;
pub const BCMA_GMAC_CMN_STAG1: u32 = 0x004;
pub const BCMA_GMAC_CMN_STAG2: u32 = 0x008;
pub const BCMA_GMAC_CMN_STAG3: u32 = 0x00C;
pub const BCMA_GMAC_CMN_PARSER_CTL: u32 = 0x020;
pub const BCMA_GMAC_CMN_MIB_MAX_LEN: u32 = 0x024;
pub const BCMA_GMAC_CMN_PHY_ACCESS: u32 = 0x100;
pub const BCMA_GMAC_CMN_PA_DATA_MASK: u32 = 0x0000ffff;
pub const BCMA_GMAC_CMN_PA_ADDR_MASK: u32 = 0x001f0000;
pub const BCMA_GMAC_CMN_PA_ADDR_SHIFT: u32 = 16;
pub const BCMA_GMAC_CMN_PA_REG_MASK: u32 = 0x1f000000;
pub const BCMA_GMAC_CMN_PA_REG_SHIFT: u32 = 24;
pub const BCMA_GMAC_CMN_PA_WRITE: u32 = 0x20000000;
pub const BCMA_GMAC_CMN_PA_START: u32 = 0x40000000;
pub const BCMA_GMAC_CMN_PHY_CTL: u32 = 0x104;
pub const BCMA_GMAC_CMN_PC_EPA_MASK: u32 = 0x0000001f;
pub const BCMA_GMAC_CMN_PC_MCT_MASK: u32 = 0x007f0000;
pub const BCMA_GMAC_CMN_PC_MCT_SHIFT: u32 = 16;
pub const BCMA_GMAC_CMN_PC_MTE: u32 = 0x00800000;
pub const BCMA_GMAC_CMN_GMAC0_RGMII_CTL: u32 = 0x110;
pub const BCMA_GMAC_CMN_CFP_ACCESS: u32 = 0x200;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA0: u32 = 0x210;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA1: u32 = 0x214;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA2: u32 = 0x218;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA3: u32 = 0x21C;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA4: u32 = 0x220;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA5: u32 = 0x224;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA6: u32 = 0x228;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA7: u32 = 0x22C;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK0: u32 = 0x230;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK1: u32 = 0x234;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK2: u32 = 0x238;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK3: u32 = 0x23C;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK4: u32 = 0x240;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK5: u32 = 0x244;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK6: u32 = 0x248;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK7: u32 = 0x24C;
pub const BCMA_GMAC_CMN_CFP_ACTION_DATA: u32 = 0x250;
pub const BCMA_GMAC_CMN_TCAM_BIST_CTL: u32 = 0x2A0;
pub const BCMA_GMAC_CMN_TCAM_BIST_STATUS: u32 = 0x2A4;
pub const BCMA_GMAC_CMN_TCAM_CMP_STATUS: u32 = 0x2A8;
pub const BCMA_GMAC_CMN_TCAM_DISABLE: u32 = 0x2AC;
pub const BCMA_GMAC_CMN_TCAM_TEST_CTL: u32 = 0x2F0;

pub const BCMA_GMAC_CMN_UDF_0_A3_A0: u32 = 0x300;
pub const BCMA_GMAC_CMN_UDF_0_A7_A4: u32 = 0x304;
pub const BCMA_GMAC_CMN_UDF_0_A8: u32 = 0x308;
pub const BCMA_GMAC_CMN_UDF_1_A3_A0: u32 = 0x310;
pub const BCMA_GMAC_CMN_UDF_1_A7_A4: u32 = 0x314;
pub const BCMA_GMAC_CMN_UDF_1_A8: u32 = 0x318;
pub const BCMA_GMAC_CMN_UDF_2_A3_A0: u32 = 0x320;
pub const BCMA_GMAC_CMN_UDF_2_A7_A4: u32 = 0x324;
pub const BCMA_GMAC_CMN_UDF_2_A8: u32 = 0x328;
pub const BCMA_GMAC_CMN_UDF_0_B3_B0: u32 = 0x330;
pub const BCMA_GMAC_CMN_UDF_0_B7_B4: u32 = 0x334;
pub const BCMA_GMAC_CMN_UDF_0_B8: u32 = 0x338;
pub const BCMA_GMAC_CMN_UDF_1_B3_B0: u32 = 0x340;
pub const BCMA_GMAC_CMN_UDF_1_B7_B4: u32 = 0x344;
pub const BCMA_GMAC_CMN_UDF_1_B8: u32 = 0x348;
pub const BCMA_GMAC_CMN_UDF_2_B3_B0: u32 = 0x350;
pub const BCMA_GMAC_CMN_UDF_2_B7_B4: u32 = 0x354;
pub const BCMA_GMAC_CMN_UDF_2_B8: u32 = 0x358;
pub const BCMA_GMAC_CMN_UDF_0_C3_C0: u32 = 0x360;
pub const BCMA_GMAC_CMN_UDF_0_C7_C4: u32 = 0x364;
pub const BCMA_GMAC_CMN_UDF_0_C8: u32 = 0x368;
pub const BCMA_GMAC_CMN_UDF_1_C3_C0: u32 = 0x370;
pub const BCMA_GMAC_CMN_UDF_1_C7_C4: u32 = 0x374;
pub const BCMA_GMAC_CMN_UDF_1_C8: u32 = 0x378;
pub const BCMA_GMAC_CMN_UDF_2_C3_C0: u32 = 0x380;
pub const BCMA_GMAC_CMN_UDF_2_C7_C4: u32 = 0x384;
pub const BCMA_GMAC_CMN_UDF_2_C8: u32 = 0x388;
pub const BCMA_GMAC_CMN_UDF_0_D3_D0: u32 = 0x390;
pub const BCMA_GMAC_CMN_UDF_0_D7_D4: u32 = 0x394;
pub const BCMA_GMAC_CMN_UDF_0_D11_D8: u32 = 0x394;

pub struct bcma_drv_gmac_cmn {
    pub core: *mut bcma_device,

    /* Drivers accessing BCMA_GMAC_CMN_PHY_ACCESS and
     * BCMA_GMAC_CMN_PHY_CTL need to take that mutex first. */
    pub phy_mutex: mutex,
}

/* Register access */
pub unsafe fn gmac_cmn_read16(gc: *mut bcma_drv_gmac_cmn, offset: u32) -> u16 {
    bcma_read16((*gc).core, offset)
}

pub unsafe fn gmac_cmn_read32(gc: *mut bcma_drv_gmac_cmn, offset: u32) -> u32 {
    bcma_read32((*gc).core, offset)
}

pub unsafe fn gmac_cmn_write16(gc: *mut bcma_drv_gmac_cmn, offset: u32, val: u16) {
    bcma_write16((*gc).core, offset, val)
}

pub unsafe fn gmac_cmn_write32(gc: *mut bcma_drv_gmac_cmn, offset: u32, val: u32) {
    bcma_write32((*gc).core, offset, val)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
