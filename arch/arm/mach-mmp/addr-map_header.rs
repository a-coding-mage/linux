/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *   Common address map definitions
 */

/* APB - Application Subsystem Peripheral Bus
 *
 * NOTE: the DMA controller registers are actually on the AXI fabric #1
 * slave port to AHB/APB bridge, due to its close relationship to those
 * peripherals on APB, let's count it into the ABP mapping area.
 */
pub const APB_PHYS_BASE: usize = 0xd4000000;
/* IOMEM(0xfe000000) */
pub const APB_VIRT_BASE: usize = 0xfe000000;
pub const APB_PHYS_SIZE: usize = 0x00200000;

pub const AXI_PHYS_BASE: usize = 0xd4200000;
/* IOMEM(0xfe200000) */
pub const AXI_VIRT_BASE: usize = 0xfe200000;
pub const AXI_PHYS_SIZE: usize = 0x00200000;

pub const PGU_PHYS_BASE: usize = 0xe0000000;
/* IOMEM(0xfe400000) */
pub const PGU_VIRT_BASE: usize = 0xfe400000;
pub const PGU_PHYS_SIZE: usize = 0x00100000;

/* Static Memory Controller - Chip Select 0 and 1 */
pub const SMC_CS0_PHYS_BASE: usize = 0x80000000;
pub const SMC_CS0_PHYS_SIZE: usize = 0x10000000;
pub const SMC_CS1_PHYS_BASE: usize = 0x90000000;
pub const SMC_CS1_PHYS_SIZE: usize = 0x10000000;

pub const APMU_VIRT_BASE: usize = AXI_VIRT_BASE + 0x82800;
macro_rules! APMU_REG {
    ($x:expr) => {
        APMU_VIRT_BASE + ($x)
    };
}

pub const APBC_VIRT_BASE: usize = APB_VIRT_BASE + 0x015000;
macro_rules! APBC_REG {
    ($x:expr) => {
        APBC_VIRT_BASE + ($x)
    };
}

pub const MPMU_VIRT_BASE: usize = APB_VIRT_BASE + 0x50000;
macro_rules! MPMU_REG {
    ($x:expr) => {
        MPMU_VIRT_BASE + ($x)
    };
}

pub const CIU_VIRT_BASE: usize = AXI_VIRT_BASE + 0x82c00;
macro_rules! CIU_REG {
    ($x:expr) => {
        CIU_VIRT_BASE + ($x)
    };
}

pub const SCU_VIRT_BASE: usize = PGU_VIRT_BASE;
macro_rules! SCU_REG {
    ($x:expr) => {
        SCU_VIRT_BASE + ($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
