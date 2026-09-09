/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 BAIKAL ELECTRONICS, JSC
 *
 * Baikal-T1 CCU reset indices
 */

// __DT_BINDINGS_RESET_BT1_CCU_H

pub const CCU_AXI_MAIN_RST: u32 = 0;
pub const CCU_AXI_DDR_RST: u32 = 1;
pub const CCU_AXI_SATA_RST: u32 = 2;
pub const CCU_AXI_GMAC0_RST: u32 = 3;
pub const CCU_AXI_GMAC1_RST: u32 = 4;
pub const CCU_AXI_XGMAC_RST: u32 = 5;
pub const CCU_AXI_PCIE_M_RST: u32 = 6;
pub const CCU_AXI_PCIE_S_RST: u32 = 7;
pub const CCU_AXI_USB_RST: u32 = 8;
pub const CCU_AXI_HWA_RST: u32 = 9;
pub const CCU_AXI_SRAM_RST: u32 = 10;

pub const CCU_SYS_SATA_REF_RST: u32 = 0;
pub const CCU_SYS_APB_RST: u32 = 1;
pub const CCU_SYS_DDR_FULL_RST: u32 = 2;
pub const CCU_SYS_DDR_INIT_RST: u32 = 3;
pub const CCU_SYS_PCIE_PCS_PHY_RST: u32 = 4;
pub const CCU_SYS_PCIE_PIPE0_RST: u32 = 5;
pub const CCU_SYS_PCIE_CORE_RST: u32 = 6;
pub const CCU_SYS_PCIE_PWR_RST: u32 = 7;
pub const CCU_SYS_PCIE_STICKY_RST: u32 = 8;
pub const CCU_SYS_PCIE_NSTICKY_RST: u32 = 9;
pub const CCU_SYS_PCIE_HOT_RST: u32 = 10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
