/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel Atom SoC Power Management Controller Header File
 * Copyright (c) 2014-2015,2022 Intel Corporation.
 */

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(high: u32, low: u32) -> u32 {
    ((!0u32) >> (31 - high)) & (!0u32 << low)
}

/* ValleyView Power Control Unit PCI Device ID */
pub const PCI_DEVICE_ID_VLV_PMC: u32 = 0x0F1C;
/* CherryTrail Power Control Unit PCI Device ID */
pub const PCI_DEVICE_ID_CHT_PMC: u32 = 0x229C;

/* PMC Memory mapped IO registers */
pub const PMC_BASE_ADDR_OFFSET: u32 = 0x44;
pub const PMC_BASE_ADDR_MASK: u32 = 0xFFFFFE00;
pub const PMC_MMIO_REG_LEN: u32 = 0x100;
pub const PMC_REG_BIT_WIDTH: u32 = 32;

/* BIOS uses FUNC_DIS to disable specific function */
pub const PMC_FUNC_DIS: u32 = 0x34;
pub const PMC_FUNC_DIS_2: u32 = 0x38;

/* CHT specific bits in FUNC_DIS2 register */
pub const BIT_FD_GMM: u32 = bit(3);
pub const BIT_FD_ISH: u32 = bit(4);

/* S0ix wake event control */
pub const PMC_S0IX_WAKE_EN: u32 = 0x3C;

pub const BIT_LPC_CLOCK_RUN: u32 = bit(4);
pub const BIT_SHARED_IRQ_GPSC: u32 = bit(5);
pub const BIT_ORED_DEDICATED_IRQ_GPSS: u32 = bit(18);
pub const BIT_ORED_DEDICATED_IRQ_GPSC: u32 = bit(19);
pub const BIT_SHARED_IRQ_GPSS: u32 = bit(20);

pub const PMC_WAKE_EN_SETTING: u32 = !(BIT_LPC_CLOCK_RUN
    | BIT_SHARED_IRQ_GPSC
    | BIT_ORED_DEDICATED_IRQ_GPSS
    | BIT_ORED_DEDICATED_IRQ_GPSC
    | BIT_SHARED_IRQ_GPSS);

/* External clk generator settings */
pub const PMC_CLK_CTL_OFFSET: u32 = 0x60;
pub const PMC_CLK_CTL_SIZE: u32 = 4;
pub const PMC_CLK_NUM: u32 = 6;
pub const PMC_CLK_CTL_GATED_ON_D3: u32 = 0x0;
pub const PMC_CLK_CTL_FORCE_ON: u32 = 0x1;
pub const PMC_CLK_CTL_FORCE_OFF: u32 = 0x2;
pub const PMC_CLK_CTL_RESERVED: u32 = 0x3;
pub const PMC_MASK_CLK_CTL: u32 = genmask(1, 0);
pub const PMC_MASK_CLK_FREQ: u32 = bit(2);
pub const PMC_CLK_FREQ_XTAL: u32 = 0 << 2; /* 25 MHz */
pub const PMC_CLK_FREQ_PLL: u32 = 1 << 2; /* 19.2 MHz */

/* The timers accumulate time spent in sleep state */
pub const PMC_S0IR_TMR: u32 = 0x80;
pub const PMC_S0I1_TMR: u32 = 0x84;
pub const PMC_S0I2_TMR: u32 = 0x88;
pub const PMC_S0I3_TMR: u32 = 0x8C;
pub const PMC_S0_TMR: u32 = 0x90;
/* Sleep state counter is in units of 32us */
pub const PMC_TMR_SHIFT: u32 = 5;

/* Power status of power islands */
pub const PMC_PSS: u32 = 0x98;

pub const PMC_PSS_BIT_GBE: u32 = bit(0);
pub const PMC_PSS_BIT_SATA: u32 = bit(1);
pub const PMC_PSS_BIT_HDA: u32 = bit(2);
pub const PMC_PSS_BIT_SEC: u32 = bit(3);
pub const PMC_PSS_BIT_PCIE: u32 = bit(4);
pub const PMC_PSS_BIT_LPSS: u32 = bit(5);
pub const PMC_PSS_BIT_LPE: u32 = bit(6);
pub const PMC_PSS_BIT_DFX: u32 = bit(7);
pub const PMC_PSS_BIT_USH_CTRL: u32 = bit(8);
pub const PMC_PSS_BIT_USH_SUS: u32 = bit(9);
pub const PMC_PSS_BIT_USH_VCCS: u32 = bit(10);
pub const PMC_PSS_BIT_USH_VCCA: u32 = bit(11);
pub const PMC_PSS_BIT_OTG_CTRL: u32 = bit(12);
pub const PMC_PSS_BIT_OTG_VCCS: u32 = bit(13);
pub const PMC_PSS_BIT_OTG_VCCA_CLK: u32 = bit(14);
pub const PMC_PSS_BIT_OTG_VCCA: u32 = bit(15);
pub const PMC_PSS_BIT_USB: u32 = bit(16);
pub const PMC_PSS_BIT_USB_SUS: u32 = bit(17);

/* CHT specific bits in PSS register */
pub const PMC_PSS_BIT_CHT_UFS: u32 = bit(7);
pub const PMC_PSS_BIT_CHT_UXD: u32 = bit(11);
pub const PMC_PSS_BIT_CHT_UXD_FD: u32 = bit(12);
pub const PMC_PSS_BIT_CHT_UX_ENG: u32 = bit(15);
pub const PMC_PSS_BIT_CHT_USB_SUS: u32 = bit(16);
pub const PMC_PSS_BIT_CHT_GMM: u32 = bit(17);
pub const PMC_PSS_BIT_CHT_ISH: u32 = bit(18);
pub const PMC_PSS_BIT_CHT_DFX_MASTER: u32 = bit(26);
pub const PMC_PSS_BIT_CHT_DFX_CLUSTER1: u32 = bit(27);
pub const PMC_PSS_BIT_CHT_DFX_CLUSTER2: u32 = bit(28);
pub const PMC_PSS_BIT_CHT_DFX_CLUSTER3: u32 = bit(29);
pub const PMC_PSS_BIT_CHT_DFX_CLUSTER4: u32 = bit(30);
pub const PMC_PSS_BIT_CHT_DFX_CLUSTER5: u32 = bit(31);

/* These registers reflect D3 status of functions */
pub const PMC_D3_STS_0: u32 = 0xA0;

pub const BIT_LPSS1_F0_DMA: u32 = bit(0);
pub const BIT_LPSS1_F1_PWM1: u32 = bit(1);
pub const BIT_LPSS1_F2_PWM2: u32 = bit(2);
pub const BIT_LPSS1_F3_HSUART1: u32 = bit(3);
pub const BIT_LPSS1_F4_HSUART2: u32 = bit(4);
pub const BIT_LPSS1_F5_SPI: u32 = bit(5);
pub const BIT_LPSS1_F6_XXX: u32 = bit(6);
pub const BIT_LPSS1_F7_XXX: u32 = bit(7);
pub const BIT_SCC_EMMC: u32 = bit(8);
pub const BIT_SCC_SDIO: u32 = bit(9);
pub const BIT_SCC_SDCARD: u32 = bit(10);
pub const BIT_SCC_MIPI: u32 = bit(11);
pub const BIT_HDA: u32 = bit(12); /* CHT datasheet: reserved */
pub const BIT_LPE: u32 = bit(13);
pub const BIT_OTG: u32 = bit(14);
pub const BIT_USH: u32 = bit(15); /* CHT datasheet: reserved */
pub const BIT_GBE: u32 = bit(16); /* CHT datasheet: reserved */
pub const BIT_SATA: u32 = bit(17); /* CHT datasheet: reserved */
pub const BIT_USB_EHCI: u32 = bit(18); /* CHT datasheet: XHCI!    */
pub const BIT_SEC: u32 = bit(19); /* BYT datasheet: reserved */
pub const BIT_PCIE_PORT0: u32 = bit(20);
pub const BIT_PCIE_PORT1: u32 = bit(21);
pub const BIT_PCIE_PORT2: u32 = bit(22);
pub const BIT_PCIE_PORT3: u32 = bit(23);
pub const BIT_LPSS2_F0_DMA: u32 = bit(24);
pub const BIT_LPSS2_F1_I2C1: u32 = bit(25);
pub const BIT_LPSS2_F2_I2C2: u32 = bit(26);
pub const BIT_LPSS2_F3_I2C3: u32 = bit(27);
pub const BIT_LPSS2_F4_I2C4: u32 = bit(28);
pub const BIT_LPSS2_F5_I2C5: u32 = bit(29);
pub const BIT_LPSS2_F6_I2C6: u32 = bit(30);
pub const BIT_LPSS2_F7_I2C7: u32 = bit(31);

pub const PMC_D3_STS_1: u32 = 0xA4;
pub const BIT_SMB: u32 = bit(0);
pub const BIT_OTG_SS_PHY: u32 = bit(1);
pub const BIT_USH_SS_PHY: u32 = bit(2);
pub const BIT_DFX: u32 = bit(3);

/* CHT specific bits in PMC_D3_STS_1 register */
pub const BIT_STS_GMM: u32 = bit(1);
pub const BIT_STS_ISH: u32 = bit(2);

/* PMC I/O Registers */
pub const ACPI_BASE_ADDR_OFFSET: u32 = 0x40;
pub const ACPI_BASE_ADDR_MASK: u32 = 0xFFFFFE00;
pub const ACPI_MMIO_REG_LEN: u32 = 0x100;

pub const PM1_CNT: u32 = 0x4;
pub const SLEEP_TYPE_MASK: u32 = genmask(12, 10);
pub const SLEEP_TYPE_S5: u32 = 0x1C00;
pub const SLEEP_ENABLE: u32 = bit(13);

unsafe extern "C" {
    pub fn pmc_atom_read(offset: i32, value: *mut u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
