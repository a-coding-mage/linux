/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic definitions of Orion SoC flavors:
 *  Orion-1, Orion-VoIP, Orion-NAS, Orion-2, and Orion-1-90.
 *
 * Maintainer: Tzachi Perelstein <tzachi@marvell.com>
 */

// The C header includes "irqs.h"; its declarations are supplied by the
// surrounding translation unit.

/*
 * Orion Address Maps
 *
 * phys
 * e0000000 PCIe MEM space
 * e8000000 PCI MEM space
 * f0000000 PCIe WA space (Orion-1/Orion-NAS only)
 * f1000000 on-chip peripheral registers
 * f2000000 PCIe I/O space
 * f2100000 PCI I/O space
 * f2200000 SRAM dedicated for the crypto unit
 * f4000000 device bus mappings (boot)
 * fa000000 device bus mappings (cs0)
 * fa800000 device bus mappings (cs2)
 * fc000000 device bus mappings (cs0/cs1)
 *
 * virt        phys        size
 * fec00000    f1000000    1M   on-chip peripheral registers
 * fee00000    f2000000    64K  PCIe I/O space
 * fee10000    f2100000    64K  PCI I/O space
 * fd000000    f0000000    16M  PCIe WA space (Orion-1/Orion-NAS only)
 */
pub const ORION5X_REGS_PHYS_BASE: usize = 0xf1000000;
pub const ORION5X_REGS_VIRT_BASE: usize = 0xfec00000; // IOMEM(0xfec00000)
pub const ORION5X_REGS_SIZE: usize = 1024 * 1024; // SZ_1M

pub const ORION5X_PCIE_IO_PHYS_BASE: usize = 0xf2000000;
pub const ORION5X_PCIE_IO_BUS_BASE: usize = 0x00000000;
pub const ORION5X_PCIE_IO_SIZE: usize = 64 * 1024; // SZ_64K

pub const ORION5X_PCI_IO_PHYS_BASE: usize = 0xf2100000;
pub const ORION5X_PCI_IO_BUS_BASE: usize = 0x00010000;
pub const ORION5X_PCI_IO_SIZE: usize = 64 * 1024; // SZ_64K

pub const ORION5X_SRAM_PHYS_BASE: usize = 0xf2200000;
pub const ORION5X_SRAM_SIZE: usize = 8 * 1024; // SZ_8K

/* Relevant only for Orion-1/Orion-NAS */
pub const ORION5X_PCIE_WA_PHYS_BASE: usize = 0xf0000000;
pub const ORION5X_PCIE_WA_VIRT_BASE: usize = 0xfd000000; // IOMEM(0xfd000000)
pub const ORION5X_PCIE_WA_SIZE: usize = 16 * 1024 * 1024; // SZ_16M

pub const ORION5X_PCIE_MEM_PHYS_BASE: usize = 0xe0000000;
pub const ORION5X_PCIE_MEM_SIZE: usize = 128 * 1024 * 1024; // SZ_128M

pub const ORION5X_PCI_MEM_PHYS_BASE: usize = 0xe8000000;
pub const ORION5X_PCI_MEM_SIZE: usize = 128 * 1024 * 1024; // SZ_128M

/* Orion Registers Map */
pub const ORION5X_DDR_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x00000;
pub const ORION5X_DDR_WINS_BASE: usize = ORION5X_DDR_PHYS_BASE + 0x1500;
pub const ORION5X_DDR_WINS_SZ: usize = 0x10;
pub const ORION5X_DDR_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x00000;
pub const ORION5X_DEV_BUS_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x10000;
pub const ORION5X_DEV_BUS_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x10000;
pub const fn ORION5X_DEV_BUS_REG(x: usize) -> usize { ORION5X_DEV_BUS_VIRT_BASE + x }
pub const GPIO_VIRT_BASE: usize = ORION5X_DEV_BUS_REG(0x0100);
pub const SPI_PHYS_BASE: usize = ORION5X_DEV_BUS_PHYS_BASE + 0x0600;
pub const I2C_PHYS_BASE: usize = ORION5X_DEV_BUS_PHYS_BASE + 0x1000;
pub const UART0_PHYS_BASE: usize = ORION5X_DEV_BUS_PHYS_BASE + 0x2000;
pub const UART0_VIRT_BASE: usize = ORION5X_DEV_BUS_VIRT_BASE + 0x2000;
pub const UART1_PHYS_BASE: usize = ORION5X_DEV_BUS_PHYS_BASE + 0x2100;
pub const UART1_VIRT_BASE: usize = ORION5X_DEV_BUS_VIRT_BASE + 0x2100;

pub const ORION5X_BRIDGE_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x20000;
pub const ORION5X_BRIDGE_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x20000;
pub const ORION5X_BRIDGE_WINS_BASE: usize = ORION5X_BRIDGE_PHYS_BASE;
pub const ORION5X_BRIDGE_WINS_SZ: usize = 0x80;
pub const ORION5X_PCI_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x30000;
pub const ORION5X_PCIE_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x40000;
pub const ORION5X_USB0_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x50000;
pub const ORION5X_USB0_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x50000;
pub const ORION5X_XOR_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x60900;
pub const ORION5X_XOR_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x60900;
pub const ORION5X_ETH_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x70000;
pub const ORION5X_ETH_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x70000;
pub const ORION5X_SATA_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x80000;
pub const ORION5X_SATA_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0x80000;
pub const ORION5X_CRYPTO_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0x90000;
pub const ORION5X_USB1_PHYS_BASE: usize = ORION5X_REGS_PHYS_BASE + 0xa0000;
pub const ORION5X_USB1_VIRT_BASE: usize = ORION5X_REGS_VIRT_BASE + 0xa0000;

/* Device Bus Registers */
pub const MPP_0_7_CTRL: usize = ORION5X_DEV_BUS_REG(0x000);
pub const MPP_8_15_CTRL: usize = ORION5X_DEV_BUS_REG(0x004);
pub const MPP_16_19_CTRL: usize = ORION5X_DEV_BUS_REG(0x050);
pub const MPP_DEV_CTRL: usize = ORION5X_DEV_BUS_REG(0x008);
pub const MPP_RESET_SAMPLE: usize = ORION5X_DEV_BUS_REG(0x010);
pub const DEV_BANK_0_PARAM: usize = ORION5X_DEV_BUS_REG(0x45c);
pub const DEV_BANK_1_PARAM: usize = ORION5X_DEV_BUS_REG(0x460);
pub const DEV_BANK_2_PARAM: usize = ORION5X_DEV_BUS_REG(0x464);
pub const DEV_BANK_BOOT_PARAM: usize = ORION5X_DEV_BUS_REG(0x46c);
pub const DEV_BUS_CTRL: usize = ORION5X_DEV_BUS_REG(0x4c0);
pub const DEV_BUS_INT_CAUSE: usize = ORION5X_DEV_BUS_REG(0x4d0);
pub const DEV_BUS_INT_MASK: usize = ORION5X_DEV_BUS_REG(0x4d4);

/* Supported Devices & Revisions */
/* Orion-1 (88F5181) and Orion-VoIP (88F5181L) */
pub const MV88F5181_DEV_ID: u32 = 0x5181;
pub const MV88F5181_REV_B1: u32 = 3;
pub const MV88F5181L_REV_A0: u32 = 8;
pub const MV88F5181L_REV_A1: u32 = 9;
/* Orion-NAS (88F5182) */
pub const MV88F5182_DEV_ID: u32 = 0x5182;
pub const MV88F5182_REV_A2: u32 = 2;
/* Orion-2 (88F5281) */
pub const MV88F5281_DEV_ID: u32 = 0x5281;
pub const MV88F5281_REV_D0: u32 = 4;
pub const MV88F5281_REV_D1: u32 = 5;
pub const MV88F5281_REV_D2: u32 = 6;
/* Orion-1-90 (88F6183) */
pub const MV88F6183_DEV_ID: u32 = 0x6183;
pub const MV88F6183_REV_B0: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
