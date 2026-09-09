/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2025 Mobileye Vision Technologies Ltd.
 */

/* ACC0 and ACC1 OLBs PLL and dividers */
pub const EQ7HC_ACC_PLL_VMP: u32 = 0;
pub const EQ7HC_ACC_PLL_MPC: u32 = 1;
pub const EQ7HC_ACC_PLL_PMA: u32 = 2;
pub const EQ7HC_ACC_PLL_NOC: u32 = 3;
pub const EQ7HC_ACC_DIV_PMA: u32 = 4;
pub const EQ7HC_ACC_DIV_NCORE: u32 = 5;
pub const EQ7HC_ACC_DIV_CFG: u32 = 6;

/* DDR0 and DDR1 OLBs PLL and dividers */
pub const EQ7HC_DDR_PLL: u32 = 0;
pub const EQ7HC_DDR_DIV_APB: u32 = 1;
pub const EQ7HC_DDR_DIV_PLLREF: u32 = 2;
pub const EQ7HC_DDR_DIV_DFI: u32 = 3;

/* east OLB PLL and dividers */
pub const EQ7HC_EAST_PLL_106P6: u32 = 0;
pub const EQ7HC_EAST_DIV_REF_106P6: u32 = 1;
pub const EQ7HC_EAST_PLL_NOC: u32 = 2;
pub const EQ7HC_EAST_PLL_ISP: u32 = 3;
pub const EQ7HC_EAST_PLL_VEU: u32 = 4;
pub const EQ7HC_EAST_DIV_REF_DDR_PHY: u32 = 5;
pub const EQ7HC_EAST_DIV_CORE: u32 = 6;
pub const EQ7HC_EAST_DIV_CORE_MBIST: u32 = 7;
pub const EQ7HC_EAST_DIV_ISRAM_MBIST: u32 = 8;
pub const EQ7HC_EAST_DIV_CFG: u32 = 9;
pub const EQ7HC_EAST_DIV_VEU_CORE: u32 = 10;
pub const EQ7HC_EAST_DIV_VEU_MBIST: u32 = 11;
pub const EQ7HC_EAST_DIV_VEU_OCP: u32 = 12;
pub const EQ7HC_EAST_DIV_LBITS: u32 = 13;
pub const EQ7HC_EAST_DIV_ISP0_CORE: u32 = 14;

/* MIPS0, MIPS1 and MIPS2 OLBs PLL and dividers */
pub const EQ7HC_MIPS_PLL_CPU: u32 = 0;
pub const EQ7HC_MIPS_DIV_CM: u32 = 1;

/* periph east OLB PLL and dividers */
pub const EQ7HC_PERIPH_EAST_PLL_PER: u32 = 0;
pub const EQ7HC_PERIPH_EAST_DIV_PER: u32 = 1;

/* periph west OLB PLL and dividers */
pub const EQ7HC_PERIPH_WEST_PLL_PER: u32 = 0;
pub const EQ7HC_PERIPH_WEST_PLL_I2S: u32 = 1;
pub const EQ7HC_PERIPH_WEST_DIV_PER: u32 = 2;
pub const EQ7HC_PERIPH_WEST_DIV_I2S: u32 = 3;

/* south OLB PLL and dividers */
pub const EQ7HC_SOUTH_PLL_100P0: u32 = 0;
pub const EQ7HC_SOUTH_DIV_REF_100P0: u32 = 1;
pub const EQ7HC_SOUTH_PLL_XSPI: u32 = 2;
pub const EQ7HC_SOUTH_PLL_VDIO: u32 = 3;
pub const EQ7HC_SOUTH_PLL_PER: u32 = 4;
pub const EQ7HC_SOUTH_DIV_VDO_DSI_SYS: u32 = 5;
pub const EQ7HC_SOUTH_DIV_PMA_CMN_REF: u32 = 6;
pub const EQ7HC_SOUTH_DIV_REF_UFS: u32 = 7;
pub const EQ7HC_SOUTH_DIV_XSPI_SYS: u32 = 8;
pub const EQ7HC_SOUTH_DIV_XSPI_MBIST: u32 = 9;
pub const EQ7HC_SOUTH_DIV_NOC_S: u32 = 10;
pub const EQ7HC_SOUTH_DIV_PCIE_SYS: u32 = 11;
pub const EQ7HC_SOUTH_DIV_PCIE_SYS_MBIST: u32 = 12;
pub const EQ7HC_SOUTH_DIV_PCIE_GBE_PHY: u32 = 13;
pub const EQ7HC_SOUTH_DIV_UFS_CORE: u32 = 14;
pub const EQ7HC_SOUTH_DIV_UFS_SMS: u32 = 15;
pub const EQ7HC_SOUTH_DIV_UFS_ROM_SMS: u32 = 16;
pub const EQ7HC_SOUTH_DIV_ETH_SYS: u32 = 17;
pub const EQ7HC_SOUTH_DIV_ETH_MBIST: u32 = 18;
pub const EQ7HC_SOUTH_DIV_CFG_S: u32 = 19;
pub const EQ7HC_SOUTH_DIV_TSU: u32 = 20;
pub const EQ7HC_SOUTH_DIV_VDIO: u32 = 21;
pub const EQ7HC_SOUTH_DIV_VDIO_CORE: u32 = 22;
pub const EQ7HC_SOUTH_DIV_VDIO_CORE_MBIST: u32 = 23;
pub const EQ7HC_SOUTH_DIV_VDO_CORE_MBIST: u32 = 24;
pub const EQ7HC_SOUTH_DIV_VDO_P: u32 = 25;
pub const EQ7HC_SOUTH_DIV_VDIO_CFG: u32 = 26;
pub const EQ7HC_SOUTH_DIV_VDIO_TXCLKESC: u32 = 27;

/* west OLB PLL and dividers */
pub const EQ7HC_WEST_PLL_106P6: u32 = 0;
pub const EQ7HC_WEST_DIV_REF_106P6: u32 = 1;
pub const EQ7HC_WEST_PLL_NOC: u32 = 2;
pub const EQ7HC_WEST_PLL_GPU: u32 = 3;
pub const EQ7HC_WEST_PLL_SSI: u32 = 4;
pub const EQ7HC_WEST_DIV_GPU: u32 = 5;
pub const EQ7HC_WEST_DIV_GPU_MBIST: u32 = 6;
pub const EQ7HC_WEST_DIV_LBITS: u32 = 7;
pub const EQ7HC_WEST_DIV_MIPS_TIMER: u32 = 8;
pub const EQ7HC_WEST_DIV_SSI_CORE: u32 = 9;
pub const EQ7HC_WEST_DIV_SSI_CORE_MBIST: u32 = 10;
pub const EQ7HC_WEST_DIV_SSI_ROM: u32 = 11;
pub const EQ7HC_WEST_DIV_SSI_ROM_MBIST: u32 = 12;
pub const EQ7HC_WEST_DIV_REF_DDR_PHY: u32 = 13;
pub const EQ7HC_WEST_DIV_CORE: u32 = 14;
pub const EQ7HC_WEST_DIV_CORE_MBIST: u32 = 15;
pub const EQ7HC_WEST_DIV_CFG: u32 = 16;
pub const EQ7HC_WEST_DIV_CAU: u32 = 17;
pub const EQ7HC_WEST_DIV_CAU_MBIST: u32 = 18;

/* XNN0 and XNN1 OLBs PLL and dividers */
pub const EQ7HC_XNN_PLL_XNN0: u32 = 0;
pub const EQ7HC_XNN_PLL_XNN1: u32 = 1;
pub const EQ7HC_XNN_PLL_XNN2: u32 = 2;
pub const EQ7HC_XNN_PLL_CLSTR: u32 = 3;
pub const EQ7HC_XNN_DIV_XNN0: u32 = 4;
pub const EQ7HC_XNN_DIV_XNN1: u32 = 5;
pub const EQ7HC_XNN_DIV_XNN2: u32 = 6;
pub const EQ7HC_XNN_DIV_CLSTR: u32 = 7;
pub const EQ7HC_XNN_DIV_I2: u32 = 8;
pub const EQ7HC_XNN_DIV_I2_SMS: u32 = 9;
pub const EQ7HC_XNN_DIV_CFG: u32 = 10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
