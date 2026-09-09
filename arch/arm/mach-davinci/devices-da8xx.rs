// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DA8XX/OMAP L1XX platform device data
 *
 * Copyright (c) 2007-2009, MontaVista Software, Inc. <source@mvista.com>
 * Derived from code that was:
 *	Copyright (C) 2006 Komal Shah <komal_shah802003@yahoo.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const DA8XX_TPCC_BASE: usize = 0x01c00000;
pub const DA8XX_TPTC0_BASE: usize = 0x01c08000;
pub const DA8XX_TPTC1_BASE: usize = 0x01c08400;
pub const DA8XX_WDOG_BASE: usize = 0x01c21000; // DA8XX_TIMER64P1_BASE
pub const DA8XX_I2C0_BASE: usize = 0x01c22000;
pub const DA8XX_RTC_BASE: usize = 0x01c23000;
pub const DA8XX_PRUSS_MEM_BASE: usize = 0x01c30000;
pub const DA8XX_MMCSD0_BASE: usize = 0x01c40000;
pub const DA8XX_SPI0_BASE: usize = 0x01c41000;
pub const DA8XX_LCD_CNTRL_BASE: usize = 0x01e13000;
pub const DA850_SATA_BASE: usize = 0x01e18000;
pub const DA850_MMCSD1_BASE: usize = 0x01e1b000;
pub const DA8XX_EMAC_CPPI_PORT_BASE: usize = 0x01e20000;
pub const DA8XX_EMAC_CPGMACSS_BASE: usize = 0x01e22000;
pub const DA8XX_EMAC_CPGMAC_BASE: usize = 0x01e23000;
pub const DA8XX_EMAC_MDIO_BASE: usize = 0x01e24000;
pub const DA8XX_I2C1_BASE: usize = 0x01e28000;
pub const DA850_TPCC1_BASE: usize = 0x01e30000;
pub const DA850_TPTC2_BASE: usize = 0x01e38000;
pub const DA850_SPI1_BASE: usize = 0x01f0e000;
pub const DA8XX_DDR2_CTL_BASE: usize = 0xb0000000;

pub const DA8XX_EMAC_CTRL_REG_OFFSET: usize = 0x3000;
pub const DA8XX_EMAC_MOD_REG_OFFSET: usize = 0x2000;
pub const DA8XX_EMAC_RAM_OFFSET: usize = 0x0000;
// SZ_8K is provided by the kernel environment.
pub const DA8XX_EMAC_CTRL_RAM_SIZE: usize = SZ_8K;

pub static mut da8xx_syscfg0_base: *mut core::ffi::c_void = core::ptr::null_mut();
pub static mut da8xx_syscfg1_base: *mut core::ffi::c_void = core::ptr::null_mut();

static mut da8xx_ddr2_ctlr_base: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    fn ioremap(phys_addr: usize, size: usize) -> *mut core::ffi::c_void;
}

pub unsafe fn da8xx_get_mem_ctlr() -> *mut core::ffi::c_void {
    if !da8xx_ddr2_ctlr_base.is_null() {
        return da8xx_ddr2_ctlr_base;
    }

    da8xx_ddr2_ctlr_base = ioremap(DA8XX_DDR2_CTL_BASE, SZ_32K);
    if da8xx_ddr2_ctlr_base.is_null() {
        // Equivalent to: pr_warn("%s: Unable to map DDR2 controller", __func__)
        pr_warn!("{}: Unable to map DDR2 controller", "da8xx_get_mem_ctlr");
    }

    da8xx_ddr2_ctlr_base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
