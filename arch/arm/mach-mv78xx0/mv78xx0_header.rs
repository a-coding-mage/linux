/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic definitions for Marvell MV78xx0 SoC flavors:
 *  MV781x0 and MV782x0.
 */

// C header dependency: "irqs.h".

/*
 * Marvell MV78xx0 address maps.
 *
 * phys
 * c0000000 PCIe Memory space
 * f0800000 PCIe #0 I/O space
 * f0900000 PCIe #1 I/O space
 * f0a00000 PCIe #2 I/O space
 * f0b00000 PCIe #3 I/O space
 * f0c00000 PCIe #4 I/O space
 * f0d00000 PCIe #5 I/O space
 * f0e00000 PCIe #6 I/O space
 * f0f00000 PCIe #7 I/O space
 * f1000000 on-chip peripheral registers
 *
 * virt        phys        size    description
 * fe400000    f102x000    16K     core-specific peripheral registers
 * fee00000    f0800000    64K     PCIe #0 I/O space
 * fee10000    f0900000    64K     PCIe #1 I/O space
 * fee20000    f0a00000    64K     PCIe #2 I/O space
 * fee30000    f0b00000    64K     PCIe #3 I/O space
 * fee40000    f0c00000    64K     PCIe #4 I/O space
 * fee50000    f0d00000    64K     PCIe #5 I/O space
 * fee60000    f0e00000    64K     PCIe #6 I/O space
 * fee70000    f0f00000    64K     PCIe #7 I/O space
 * fec00000    f1000000    1M      on-chip peripheral registers
 */
pub const MV78XX0_CORE0_REGS_PHYS_BASE: usize = 0xf1020000;
pub const MV78XX0_CORE1_REGS_PHYS_BASE: usize = 0xf1024000;
pub const MV78XX0_CORE_REGS_VIRT_BASE: usize = 0xfe400000;
pub const MV78XX0_CORE_REGS_PHYS_BASE: usize = 0xfe400000;
pub const MV78XX0_CORE_REGS_SIZE: usize = 16 * 1024;

pub const fn mv78xx0_pcie_io_phys_base(i: usize) -> usize {
    0xf0800000usize + (i << 20)
}
pub const MV78XX0_PCIE_IO_SIZE: usize = 1024 * 1024;

pub const MV78XX0_REGS_PHYS_BASE: usize = 0xf1000000;
pub const MV78XX0_REGS_VIRT_BASE: usize = 0xfec00000;
pub const MV78XX0_REGS_SIZE: usize = 1024 * 1024;

pub const MV78XX0_SRAM_PHYS_BASE: usize = 0xf2200000;
pub const MV78XX0_SRAM_SIZE: usize = 8 * 1024;

pub const MV78XX0_PCIE_MEM_PHYS_BASE: usize = 0xc0000000;
pub const MV78XX0_PCIE_MEM_SIZE: usize = 0x30000000;

pub const MV78XX0_MBUS_SRAM_TARGET: usize = 0x09;
pub const MV78XX0_MBUS_SRAM_ATTR: usize = 0x00;

/* Core-specific peripheral registers. */
pub const BRIDGE_VIRT_BASE: usize = MV78XX0_CORE_REGS_VIRT_BASE;
pub const BRIDGE_PHYS_BASE: usize = MV78XX0_CORE_REGS_PHYS_BASE;
pub const BRIDGE_WINS_CPU0_BASE: usize = MV78XX0_CORE0_REGS_PHYS_BASE;
pub const BRIDGE_WINS_CPU1_BASE: usize = MV78XX0_CORE1_REGS_PHYS_BASE;
pub const BRIDGE_WINS_SZ: usize = 0xA000;

/* Register Map */
pub const DDR_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x00000;
pub const DDR_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x00000;
pub const DDR_WINDOW_CPU0_BASE: usize = DDR_PHYS_BASE + 0x1500;
pub const DDR_WINDOW_CPU1_BASE: usize = DDR_PHYS_BASE + 0x1570;
pub const DDR_WINDOW_CPU_SZ: usize = 0x20;

pub const DEV_BUS_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x10000;
pub const DEV_BUS_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x10000;
pub const SAMPLE_AT_RESET_LOW: usize = DEV_BUS_VIRT_BASE + 0x0030;
pub const SAMPLE_AT_RESET_HIGH: usize = DEV_BUS_VIRT_BASE + 0x0034;
pub const GPIO_VIRT_BASE: usize = DEV_BUS_VIRT_BASE + 0x0100;
pub const I2C_0_PHYS_BASE: usize = DEV_BUS_PHYS_BASE + 0x1000;
pub const I2C_1_PHYS_BASE: usize = DEV_BUS_PHYS_BASE + 0x1100;
pub const UART0_PHYS_BASE: usize = DEV_BUS_PHYS_BASE + 0x2000;
pub const UART0_VIRT_BASE: usize = DEV_BUS_VIRT_BASE + 0x2000;
pub const UART1_PHYS_BASE: usize = DEV_BUS_PHYS_BASE + 0x2100;
pub const UART1_VIRT_BASE: usize = DEV_BUS_VIRT_BASE + 0x2100;
pub const UART2_PHYS_BASE: usize = DEV_BUS_PHYS_BASE + 0x2200;
pub const UART2_VIRT_BASE: usize = DEV_BUS_VIRT_BASE + 0x2200;
pub const UART3_PHYS_BASE: usize = DEV_BUS_PHYS_BASE + 0x2300;
pub const UART3_VIRT_BASE: usize = DEV_BUS_VIRT_BASE + 0x2300;

pub const GE10_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x30000;
pub const GE11_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x34000;
pub const PCIE00_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x40000;
pub const PCIE01_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x44000;
pub const PCIE02_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x48000;
pub const PCIE03_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x4c000;
pub const USB0_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x50000;
pub const USB1_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x51000;
pub const USB2_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x52000;
pub const XOR_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x60900;
pub const GE00_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x70000;
pub const GE01_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x74000;
pub const PCIE10_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x80000;
pub const PCIE11_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x84000;
pub const PCIE12_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x88000;
pub const PCIE13_VIRT_BASE: usize = MV78XX0_REGS_VIRT_BASE + 0x8c000;
pub const CRYPTO_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0x90000;
pub const SATA_PHYS_BASE: usize = MV78XX0_REGS_PHYS_BASE + 0xa0000;

/* Supported devices and revisions. */
pub const MV78X00_Z0_DEV_ID: usize = 0x6381;
pub const MV78X00_REV_Z0: usize = 1;
pub const MV78100_DEV_ID: usize = 0x7810;
pub const MV78100_REV_A0: usize = 1;
pub const MV78100_REV_A1: usize = 2;
pub const MV78200_DEV_ID: usize = 0x7820;
pub const MV78200_REV_A0: usize = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
