/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Chip Selects
 */
pub const PXA_CS0_PHYS: usize = 0x00000000;
pub const PXA_CS1_PHYS: usize = 0x04000000;
pub const PXA_CS2_PHYS: usize = 0x08000000;
pub const PXA_CS3_PHYS: usize = 0x0C000000;
pub const PXA_CS4_PHYS: usize = 0x10000000;
pub const PXA_CS5_PHYS: usize = 0x14000000;

pub const PXA300_CS0_PHYS: usize = 0x00000000; /* PXA300/PXA310 _only_ */
pub const PXA300_CS1_PHYS: usize = 0x30000000; /* PXA300/PXA310 _only_ */
pub const PXA3xx_CS2_PHYS: usize = 0x10000000;
pub const PXA3xx_CS3_PHYS: usize = 0x14000000;

/*
 * Peripheral Bus
 */
pub const PERIPH_PHYS: usize = 0x40000000;
/* C macro: IOMEM(0xf2000000); represented as the virtual address value. */
pub const PERIPH_VIRT: usize = 0xf2000000;
pub const PERIPH_SIZE: usize = 0x02000000;

/*
 * Static Memory Controller (w/ SDRAM controls on PXA25x/PXA27x)
 */
pub const PXA2XX_SMEMC_PHYS: usize = 0x48000000;
pub const PXA3XX_SMEMC_PHYS: usize = 0x4a000000;
/* C macro: IOMEM(0xf6000000); represented as the virtual address value. */
pub const SMEMC_VIRT: usize = 0xf6000000;
pub const SMEMC_SIZE: usize = 0x00100000;

/*
 * Dynamic Memory Controller (only on PXA3xx)
 */
pub const DMEMC_PHYS: usize = 0x48100000;
/* C macro: IOMEM(0xf6100000); represented as the virtual address value. */
pub const DMEMC_VIRT: usize = 0xf6100000;
pub const DMEMC_SIZE: usize = 0x00100000;

/*
 * Reserved space for low level debug virtual addresses within
 * 0xf6200000..0xf6201000
 */

/*
 * DFI Bus for NAND, PXA3xx only
 */
pub const NAND_PHYS: usize = 0x43100000;
/* C macro: IOMEM(0xf6300000); represented as the virtual address value. */
pub const NAND_VIRT: usize = 0xf6300000;
pub const NAND_SIZE: usize = 0x00100000;

/*
 * Internal Memory Controller (PXA27x and later)
 */
pub const IMEMC_PHYS: usize = 0x58000000;
/* C macro: IOMEM(0xfe000000); represented as the virtual address value. */
pub const IMEMC_VIRT: usize = 0xfe000000;
pub const IMEMC_SIZE: usize = 0x00100000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
