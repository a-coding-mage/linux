/* SPDX-License-Identifier: GPL-2.0 */
// Translated from bcma_driver_pcie2.h. C header guards and includes omitted.

pub const BCMA_CORE_PCIE2_CLK_CONTROL: u32 = 0x0000;
pub const PCIE2_CLKC_RST_OE: u32 = 0x0001; // When set, drives PCI_RESET out to pin
pub const PCIE2_CLKC_RST: u32 = 0x0002; // Value driven out to pin
pub const PCIE2_CLKC_SPERST: u32 = 0x0004; // SurvivePeRst
pub const PCIE2_CLKC_DISABLE_L1CLK_GATING: u32 = 0x0010;
pub const PCIE2_CLKC_DLYPERST: u32 = 0x0100; // Delay PeRst to CoE Core
pub const PCIE2_CLKC_DISSPROMLD: u32 = 0x0200; // DisableSpromLoadOnPerst
pub const PCIE2_CLKC_WAKE_MODE_L2: u32 = 0x1000; // Wake on L2
pub const BCMA_CORE_PCIE2_RC_PM_CONTROL: u32 = 0x0004;
pub const BCMA_CORE_PCIE2_RC_PM_STATUS: u32 = 0x0008;
pub const BCMA_CORE_PCIE2_EP_PM_CONTROL: u32 = 0x000C;
pub const BCMA_CORE_PCIE2_EP_PM_STATUS: u32 = 0x0010;
pub const BCMA_CORE_PCIE2_EP_LTR_CONTROL: u32 = 0x0014;
pub const BCMA_CORE_PCIE2_EP_LTR_STATUS: u32 = 0x0018;
pub const BCMA_CORE_PCIE2_EP_OBFF_STATUS: u32 = 0x001C;
pub const BCMA_CORE_PCIE2_PCIE_ERR_STATUS: u32 = 0x0020;
pub const BCMA_CORE_PCIE2_RC_AXI_CONFIG: u32 = 0x0100;
pub const BCMA_CORE_PCIE2_EP_AXI_CONFIG: u32 = 0x0104;
pub const BCMA_CORE_PCIE2_RXDEBUG_STATUS0: u32 = 0x0108;
pub const BCMA_CORE_PCIE2_RXDEBUG_CONTROL0: u32 = 0x010C;
pub const BCMA_CORE_PCIE2_CONFIGINDADDR: u32 = 0x0120;
pub const BCMA_CORE_PCIE2_CONFIGINDDATA: u32 = 0x0124;
pub const BCMA_CORE_PCIE2_MDIOCONTROL: u32 = 0x0128;
pub const BCMA_CORE_PCIE2_MDIOWRDATA: u32 = 0x012C;
pub const BCMA_CORE_PCIE2_MDIORDDATA: u32 = 0x0130;
pub const BCMA_CORE_PCIE2_DATAINTF: u32 = 0x0180;
pub const BCMA_CORE_PCIE2_D2H_INTRLAZY_0: u32 = 0x0188;
pub const BCMA_CORE_PCIE2_H2D_INTRLAZY_0: u32 = 0x018c;
pub const BCMA_CORE_PCIE2_H2D_INTSTAT_0: u32 = 0x0190;
pub const BCMA_CORE_PCIE2_H2D_INTMASK_0: u32 = 0x0194;
pub const BCMA_CORE_PCIE2_D2H_INTSTAT_0: u32 = 0x0198;
pub const BCMA_CORE_PCIE2_D2H_INTMASK_0: u32 = 0x019c;
pub const BCMA_CORE_PCIE2_LTR_STATE: u32 = 0x01A0; // Latency Tolerance Reporting
pub const PCIE2_LTR_ACTIVE: u32 = 2;
pub const PCIE2_LTR_ACTIVE_IDLE: u32 = 1;
pub const PCIE2_LTR_SLEEP: u32 = 0;
pub const PCIE2_LTR_FINAL_MASK: u32 = 0x300;
pub const PCIE2_LTR_FINAL_SHIFT: u32 = 8;
pub const BCMA_CORE_PCIE2_PWR_INT_STATUS: u32 = 0x01A4;
pub const BCMA_CORE_PCIE2_PWR_INT_MASK: u32 = 0x01A8;
pub const BCMA_CORE_PCIE2_CFG_ADDR: u32 = 0x01F8;
pub const BCMA_CORE_PCIE2_CFG_DATA: u32 = 0x01FC;
pub const BCMA_CORE_PCIE2_SYS_EQ_PAGE: u32 = 0x0200;
pub const BCMA_CORE_PCIE2_SYS_MSI_PAGE: u32 = 0x0204;
pub const BCMA_CORE_PCIE2_SYS_MSI_INTREN: u32 = 0x0208;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL0: u32 = 0x0210;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL1: u32 = 0x0214;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL2: u32 = 0x0218;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL3: u32 = 0x021C;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL4: u32 = 0x0220;
pub const BCMA_CORE_PCIE2_SYS_MSI_CTRL5: u32 = 0x0224;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD0: u32 = 0x0250;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL0: u32 = 0x0254;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD1: u32 = 0x0258;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL1: u32 = 0x025C;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD2: u32 = 0x0260;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL2: u32 = 0x0264;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD3: u32 = 0x0268;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL3: u32 = 0x026C;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD4: u32 = 0x0270;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL4: u32 = 0x0274;
pub const BCMA_CORE_PCIE2_SYS_EQ_HEAD5: u32 = 0x0278;
pub const BCMA_CORE_PCIE2_SYS_EQ_TAIL5: u32 = 0x027C;
pub const BCMA_CORE_PCIE2_SYS_RC_INTX_EN: u32 = 0x0330;
pub const BCMA_CORE_PCIE2_SYS_RC_INTX_CSR: u32 = 0x0334;
pub const BCMA_CORE_PCIE2_SYS_MSI_REQ: u32 = 0x0340;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR_EN: u32 = 0x0344;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR_CSR: u32 = 0x0348;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR0: u32 = 0x0350;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR1: u32 = 0x0354;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR2: u32 = 0x0358;
pub const BCMA_CORE_PCIE2_SYS_HOST_INTR3: u32 = 0x035C;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_EN0: u32 = 0x0360;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_EN1: u32 = 0x0364;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_CSR0: u32 = 0x0370;
pub const BCMA_CORE_PCIE2_SYS_EP_INT_CSR1: u32 = 0x0374;
pub const BCMA_CORE_PCIE2_SPROM: unsafe fn(u32) -> u32 = |wordoffset| 0x0800 + wordoffset * 2;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_0: u32 = 0x0C00;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_1: u32 = 0x0C04;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_2: u32 = 0x0C08;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_3: u32 = 0x0C0C;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_4: u32 = 0x0C10;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_5: u32 = 0x0C14;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_6: u32 = 0x0C18;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP0_7: u32 = 0x0C1C;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_0: u32 = 0x0C20;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_1: u32 = 0x0C24;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_2: u32 = 0x0C28;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_3: u32 = 0x0C2C;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_4: u32 = 0x0C30;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_5: u32 = 0x0C34;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_6: u32 = 0x0C38;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP0_7: u32 = 0x0C3C;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP1: u32 = 0x0C80;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP1: u32 = 0x0C88;
pub const BCMA_CORE_PCIE2_FUNC0_IMAP2: u32 = 0x0CC0;
pub const BCMA_CORE_PCIE2_FUNC1_IMAP2: u32 = 0x0CC8;
pub const BCMA_CORE_PCIE2_IARR0_LOWER: u32 = 0x0D00;
pub const BCMA_CORE_PCIE2_IARR0_UPPER: u32 = 0x0D04;
pub const BCMA_CORE_PCIE2_IARR1_LOWER: u32 = 0x0D08;
pub const BCMA_CORE_PCIE2_IARR1_UPPER: u32 = 0x0D0C;
pub const BCMA_CORE_PCIE2_IARR2_LOWER: u32 = 0x0D10;
pub const BCMA_CORE_PCIE2_IARR2_UPPER: u32 = 0x0D14;
pub const BCMA_CORE_PCIE2_OARR0: u32 = 0x0D20;
pub const BCMA_CORE_PCIE2_OARR1: u32 = 0x0D28;
pub const BCMA_CORE_PCIE2_OARR2: u32 = 0x0D30;
pub const BCMA_CORE_PCIE2_OMAP0_LOWER: u32 = 0x0D40;
pub const BCMA_CORE_PCIE2_OMAP0_UPPER: u32 = 0x0D44;
pub const BCMA_CORE_PCIE2_OMAP1_LOWER: u32 = 0x0D48;
pub const BCMA_CORE_PCIE2_OMAP1_UPPER: u32 = 0x0D4C;
pub const BCMA_CORE_PCIE2_OMAP2_LOWER: u32 = 0x0D50;
pub const BCMA_CORE_PCIE2_OMAP2_UPPER: u32 = 0x0D54;
pub const BCMA_CORE_PCIE2_FUNC1_IARR1_SIZE: u32 = 0x0D58;
pub const BCMA_CORE_PCIE2_FUNC1_IARR2_SIZE: u32 = 0x0D5C;
pub const BCMA_CORE_PCIE2_MEM_CONTROL: u32 = 0x0F00;
pub const BCMA_CORE_PCIE2_MEM_ECC_ERRLOG0: u32 = 0x0F04;
pub const BCMA_CORE_PCIE2_MEM_ECC_ERRLOG1: u32 = 0x0F08;
pub const BCMA_CORE_PCIE2_LINK_STATUS: u32 = 0x0F0C;
pub const BCMA_CORE_PCIE2_STRAP_STATUS: u32 = 0x0F10;
pub const BCMA_CORE_PCIE2_RESET_STATUS: u32 = 0x0F14;
pub const BCMA_CORE_PCIE2_RESETEN_IN_LINKDOWN: u32 = 0x0F18;
pub const BCMA_CORE_PCIE2_MISC_INTR_EN: u32 = 0x0F1C;
pub const BCMA_CORE_PCIE2_TX_DEBUG_CFG: u32 = 0x0F20;
pub const BCMA_CORE_PCIE2_MISC_CONFIG: u32 = 0x0F24;
pub const BCMA_CORE_PCIE2_MISC_STATUS: u32 = 0x0F28;
pub const BCMA_CORE_PCIE2_INTR_EN: u32 = 0x0F30;
pub const BCMA_CORE_PCIE2_INTR_CLEAR: u32 = 0x0F34;
pub const BCMA_CORE_PCIE2_INTR_STATUS: u32 = 0x0F38;

// PCIE gen2 config regs
pub const PCIE2_INTSTATUS: u32 = 0x090;
pub const PCIE2_INTMASK: u32 = 0x094;
pub const PCIE2_SBMBX: u32 = 0x098;
pub const PCIE2_PMCR_REFUP: u32 = 0x1814; // Trefup time
pub const PCIE2_CAP_DEVSTSCTRL2_OFFSET: u32 = 0xD4;
pub const PCIE2_CAP_DEVSTSCTRL2_LTRENAB: u32 = 0x400;
pub const PCIE2_PVT_REG_PM_CLK_PERIOD: u32 = 0x184c;

#[repr(C)]
pub struct bcma_drv_pcie2 {
    pub core: *mut bcma_device,
    pub reqsize: u16,
}

pub unsafe fn pcie2_read16(pcie2: *mut bcma_drv_pcie2, offset: u32) -> u16 {
    bcma_read16((*pcie2).core, offset)
}
pub unsafe fn pcie2_read32(pcie2: *mut bcma_drv_pcie2, offset: u32) -> u32 {
    bcma_read32((*pcie2).core, offset)
}
pub unsafe fn pcie2_write16(pcie2: *mut bcma_drv_pcie2, offset: u32, val: u16) {
    bcma_write16((*pcie2).core, offset, val)
}
pub unsafe fn pcie2_write32(pcie2: *mut bcma_drv_pcie2, offset: u32, val: u32) {
    bcma_write32((*pcie2).core, offset, val)
}
pub unsafe fn pcie2_set32(pcie2: *mut bcma_drv_pcie2, offset: u32, set: u32) {
    bcma_set32((*pcie2).core, offset, set)
}
pub unsafe fn pcie2_mask32(pcie2: *mut bcma_drv_pcie2, offset: u32, mask: u32) {
    bcma_mask32((*pcie2).core, offset, mask)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
