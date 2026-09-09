/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Ralink RT3662/RT3883 SoC register definitions
 *
 * Copyright (C) 2011-2012 Gabor Juhos <juhosg@openwrt.org>
 */

// Linux IOMEM/KSEG1ADDR mapping is represented here by the physical address
// value; platform-specific users may apply their mapping as needed.

pub const RT3883_SDRAM_BASE: u32 = 0x00000000;
pub const RT3883_SYSC_BASE: usize = 0x10000000;
pub const RT3883_TIMER_BASE: u32 = 0x10000100;
pub const RT3883_INTC_BASE: u32 = 0x10000200;
pub const RT3883_MEMC_BASE: u32 = 0x10000300;
pub const RT3883_UART0_BASE: u32 = 0x10000500;
pub const RT3883_PIO_BASE: u32 = 0x10000600;
pub const RT3883_FSCC_BASE: u32 = 0x10000700;
pub const RT3883_NANDC_BASE: u32 = 0x10000810;
pub const RT3883_I2C_BASE: u32 = 0x10000900;
pub const RT3883_I2S_BASE: u32 = 0x10000a00;
pub const RT3883_SPI_BASE: u32 = 0x10000b00;
pub const RT3883_UART1_BASE: u32 = 0x10000c00;
pub const RT3883_PCM_BASE: u32 = 0x10002000;
pub const RT3883_GDMA_BASE: u32 = 0x10002800;
pub const RT3883_CODEC1_BASE: u32 = 0x10003000;
pub const RT3883_CODEC2_BASE: u32 = 0x10003800;
pub const RT3883_FE_BASE: u32 = 0x10100000;
pub const RT3883_ROM_BASE: u32 = 0x10118000;
pub const RT3883_USBDEV_BASE: u32 = 0x10112000;
pub const RT3883_PCI_BASE: u32 = 0x10140000;
pub const RT3883_WLAN_BASE: u32 = 0x10180000;
pub const RT3883_USBHOST_BASE: u32 = 0x101c0000;
pub const RT3883_BOOT_BASE: u32 = 0x1c000000;
pub const RT3883_SRAM_BASE: u32 = 0x1e000000;
pub const RT3883_PCIMEM_BASE: u32 = 0x20000000;

pub const RT3883_EHCI_BASE: u32 = RT3883_USBHOST_BASE;
pub const RT3883_OHCI_BASE: u32 = RT3883_USBHOST_BASE + 0x1000;

pub const RT3883_SYSC_SIZE: u32 = 0x100;
pub const RT3883_TIMER_SIZE: u32 = 0x100;
pub const RT3883_INTC_SIZE: u32 = 0x100;
pub const RT3883_MEMC_SIZE: u32 = 0x100;
pub const RT3883_UART0_SIZE: u32 = 0x100;
pub const RT3883_UART1_SIZE: u32 = 0x100;
pub const RT3883_PIO_SIZE: u32 = 0x100;
pub const RT3883_FSCC_SIZE: u32 = 0x100;
pub const RT3883_NANDC_SIZE: u32 = 0x0f0;
pub const RT3883_I2C_SIZE: u32 = 0x100;
pub const RT3883_I2S_SIZE: u32 = 0x100;
pub const RT3883_SPI_SIZE: u32 = 0x100;
pub const RT3883_PCM_SIZE: u32 = 0x800;
pub const RT3883_GDMA_SIZE: u32 = 0x800;
pub const RT3883_CODEC1_SIZE: u32 = 0x800;
pub const RT3883_CODEC2_SIZE: u32 = 0x800;
pub const RT3883_FE_SIZE: u32 = 0x10000;
pub const RT3883_ROM_SIZE: u32 = 0x4000;
pub const RT3883_USBDEV_SIZE: u32 = 0x4000;
pub const RT3883_PCI_SIZE: u32 = 0x40000;
pub const RT3883_WLAN_SIZE: u32 = 0x40000;
pub const RT3883_USBHOST_SIZE: u32 = 0x40000;
pub const RT3883_BOOT_SIZE: u32 = 32 * 1024 * 1024;
pub const RT3883_SRAM_SIZE: u32 = 32 * 1024 * 1024;

pub const RT3883_SYSC_REG_CHIPID0_3: u32 = 0x00;
pub const RT3883_SYSC_REG_CHIPID4_7: u32 = 0x04;
pub const RT3883_SYSC_REG_REVID: u32 = 0x0c;
pub const RT3883_SYSC_REG_SYSCFG0: u32 = 0x10;
pub const RT3883_SYSC_REG_SYSCFG1: u32 = 0x14;
pub const RT3883_SYSC_REG_CLKCFG0: u32 = 0x2c;
pub const RT3883_SYSC_REG_CLKCFG1: u32 = 0x30;
pub const RT3883_SYSC_REG_RSTCTRL: u32 = 0x34;
pub const RT3883_SYSC_REG_RSTSTAT: u32 = 0x38;
pub const RT3883_SYSC_REG_USB_PS: u32 = 0x5c;
pub const RT3883_SYSC_REG_GPIO_MODE: u32 = 0x60;
pub const RT3883_SYSC_REG_PCIE_CLK_GEN0: u32 = 0x7c;
pub const RT3883_SYSC_REG_PCIE_CLK_GEN1: u32 = 0x80;
pub const RT3883_SYSC_REG_PCIE_CLK_GEN2: u32 = 0x84;
pub const RT3883_SYSC_REG_PMU: u32 = 0x88;
pub const RT3883_SYSC_REG_PMU1: u32 = 0x8c;

pub const RT3883_CHIP_NAME0: u32 = 0x38335452;
pub const RT3883_CHIP_NAME1: u32 = 0x20203338;
pub const RT3883_REVID_VER_ID_MASK: u32 = 0x0f;
pub const RT3883_REVID_VER_ID_SHIFT: u32 = 8;
pub const RT3883_REVID_ECO_ID_MASK: u32 = 0x0f;

pub const RT3883_SYSCFG1_USB0_HOST_MODE: u32 = 1 << 10;
pub const RT3883_SYSCFG1_PCIE_RC_MODE: u32 = 1 << 8;
pub const RT3883_SYSCFG1_PCI_HOST_MODE: u32 = 1 << 7;
pub const RT3883_SYSCFG1_PCI_66M_MODE: u32 = 1 << 6;
pub const RT3883_SYSCFG1_GPIO2_AS_WDT_OUT: u32 = 1 << 2;
pub const RT3883_CLKCFG1_PCIE_CLK_EN: u32 = 1 << 21;
pub const RT3883_CLKCFG1_UPHY1_CLK_EN: u32 = 1 << 20;
pub const RT3883_CLKCFG1_PCI_CLK_EN: u32 = 1 << 19;
pub const RT3883_CLKCFG1_UPHY0_CLK_EN: u32 = 1 << 18;

pub const RT3883_GPIO_I2C_SD: u32 = 1;
pub const RT3883_GPIO_I2C_SCLK: u32 = 2;
pub const RT3883_GPIO_SPI_CS0: u32 = 3;
pub const RT3883_GPIO_SPI_CLK: u32 = 4;
pub const RT3883_GPIO_SPI_MOSI: u32 = 5;
pub const RT3883_GPIO_SPI_MISO: u32 = 6;
pub const RT3883_GPIO_7: u32 = 7;
pub const RT3883_GPIO_10: u32 = 10;
pub const RT3883_GPIO_11: u32 = 11;
pub const RT3883_GPIO_14: u32 = 14;
pub const RT3883_GPIO_UART1_TXD: u32 = 15;
pub const RT3883_GPIO_UART1_RXD: u32 = 16;
pub const RT3883_GPIO_JTAG_TDO: u32 = 17;
pub const RT3883_GPIO_JTAG_TDI: u32 = 18;
pub const RT3883_GPIO_JTAG_TMS: u32 = 19;
pub const RT3883_GPIO_JTAG_TCLK: u32 = 20;
pub const RT3883_GPIO_JTAG_TRST_N: u32 = 21;
pub const RT3883_GPIO_MDIO_MDC: u32 = 22;
pub const RT3883_GPIO_MDIO_MDIO: u32 = 23;
pub const RT3883_GPIO_LNA_PE_A0: u32 = 32;
pub const RT3883_GPIO_LNA_PE_A1: u32 = 33;
pub const RT3883_GPIO_LNA_PE_A2: u32 = 34;
pub const RT3883_GPIO_LNA_PE_G0: u32 = 35;
pub const RT3883_GPIO_LNA_PE_G1: u32 = 36;
pub const RT3883_GPIO_LNA_PE_G2: u32 = 37;
pub const RT3883_GPIO_PCI_AD0: u32 = 40;
pub const RT3883_GPIO_PCI_AD31: u32 = 71;
pub const RT3883_GPIO_GE2_TXD0: u32 = 72;
pub const RT3883_GPIO_GE2_TXD1: u32 = 73;
pub const RT3883_GPIO_GE2_TXD2: u32 = 74;
pub const RT3883_GPIO_GE2_TXD3: u32 = 75;
pub const RT3883_GPIO_GE2_TXEN: u32 = 76;
pub const RT3883_GPIO_GE2_TXCLK: u32 = 77;
pub const RT3883_GPIO_GE2_RXD0: u32 = 78;
pub const RT3883_GPIO_GE2_RXD1: u32 = 79;
pub const RT3883_GPIO_GE2_RXD2: u32 = 80;
pub const RT3883_GPIO_GE2_RXD3: u32 = 81;
pub const RT3883_GPIO_GE2_RXDV: u32 = 82;
pub const RT3883_GPIO_GE2_RXCLK: u32 = 83;
pub const RT3883_GPIO_GE1_TXD0: u32 = 84;
pub const RT3883_GPIO_GE1_TXD1: u32 = 85;
pub const RT3883_GPIO_GE1_TXD2: u32 = 86;
pub const RT3883_GPIO_GE1_TXD3: u32 = 87;
pub const RT3883_GPIO_GE1_TXEN: u32 = 88;
pub const RT3883_GPIO_GE1_TXCLK: u32 = 89;
pub const RT3883_GPIO_GE1_RXD0: u32 = 90;
pub const RT3883_GPIO_GE1_RXD1: u32 = 91;
pub const RT3883_GPIO_GE1_RXD2: u32 = 92;
pub const RT3883_GPIO_GE1_RXD3: u32 = 93;
pub const RT3883_GPIO_GE1_RXDV: u32 = 94;
pub const RT3883_GPIO_GE1_RXCLK: u32 = 95;

pub const RT3883_RSTCTRL_PCIE_PCI_PDM: u32 = 1 << 27;
pub const RT3883_RSTCTRL_FLASH: u32 = 1 << 26;
pub const RT3883_RSTCTRL_UDEV: u32 = 1 << 25;
pub const RT3883_RSTCTRL_PCI: u32 = 1 << 24;
pub const RT3883_RSTCTRL_PCIE: u32 = 1 << 23;
pub const RT3883_RSTCTRL_UHST: u32 = 1 << 22;
pub const RT3883_RSTCTRL_FE: u32 = 1 << 21;
pub const RT3883_RSTCTRL_WLAN: u32 = 1 << 20;
pub const RT3883_RSTCTRL_UART1: u32 = 1 << 29;
pub const RT3883_RSTCTRL_SPI: u32 = 1 << 18;
pub const RT3883_RSTCTRL_I2S: u32 = 1 << 17;
pub const RT3883_RSTCTRL_I2C: u32 = 1 << 16;
pub const RT3883_RSTCTRL_NAND: u32 = 1 << 15;
pub const RT3883_RSTCTRL_DMA: u32 = 1 << 14;
pub const RT3883_RSTCTRL_PIO: u32 = 1 << 13;
pub const RT3883_RSTCTRL_UART: u32 = 1 << 12;
pub const RT3883_RSTCTRL_PCM: u32 = 1 << 11;
pub const RT3883_RSTCTRL_MC: u32 = 1 << 10;
pub const RT3883_RSTCTRL_INTC: u32 = 1 << 9;
pub const RT3883_RSTCTRL_TIMER: u32 = 1 << 8;
pub const RT3883_RSTCTRL_SYS: u32 = 1 << 0;

pub const RT3883_INTC_INT_SYSCTL: u32 = 1 << 0;
pub const RT3883_INTC_INT_TIMER0: u32 = 1 << 1;
pub const RT3883_INTC_INT_TIMER1: u32 = 1 << 2;
pub const RT3883_INTC_INT_IA: u32 = 1 << 3;
pub const RT3883_INTC_INT_PCM: u32 = 1 << 4;
pub const RT3883_INTC_INT_UART0: u32 = 1 << 5;
pub const RT3883_INTC_INT_PIO: u32 = 1 << 6;
pub const RT3883_INTC_INT_DMA: u32 = 1 << 7;
pub const RT3883_INTC_INT_NAND: u32 = 1 << 8;
pub const RT3883_INTC_INT_PERFC: u32 = 1 << 9;
pub const RT3883_INTC_INT_I2S: u32 = 1 << 10;
pub const RT3883_INTC_INT_UART1: u32 = 1 << 12;
pub const RT3883_INTC_INT_UHST: u32 = 1 << 18;
pub const RT3883_INTC_INT_UDEV: u32 = 1 << 19;

pub const RT3883_FSCC_REG_FLASH_CFG0: u32 = 0x00;
pub const RT3883_FSCC_REG_FLASH_CFG1: u32 = 0x04;
pub const RT3883_FSCC_REG_CODEC_CFG0: u32 = 0x40;
pub const RT3883_FSCC_REG_CODEC_CFG1: u32 = 0x44;
pub const RT3883_FLASH_CFG_WIDTH_SHIFT: u32 = 26;
pub const RT3883_FLASH_CFG_WIDTH_MASK: u32 = 0x3;
pub const RT3883_FLASH_CFG_WIDTH_8BIT: u32 = 0x0;
pub const RT3883_FLASH_CFG_WIDTH_16BIT: u32 = 0x1;
pub const RT3883_FLASH_CFG_WIDTH_32BIT: u32 = 0x2;

pub const RT3883_MEM_SIZE_MIN: u32 = 2;
pub const RT3883_MEM_SIZE_MAX: u32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
