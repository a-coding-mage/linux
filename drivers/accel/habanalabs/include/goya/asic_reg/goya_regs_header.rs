/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// Original header guard: ASIC_REG_GOYA_REGS_H_

// C dependencies retained as source-level intent; their declarations are
// supplied by the corresponding translated modules/dependencies.
// goya_blocks.h
// stlb_regs.h
// mmu_regs.h
// pcie_aux_regs.h
// pcie_wrap_regs.h
// psoc_global_conf_regs.h
// psoc_spi_regs.h
// psoc_mme_pll_regs.h
// psoc_pci_pll_regs.h
// psoc_emmc_pll_regs.h
// psoc_timestamp_regs.h
// cpu_if_regs.h
// cpu_ca53_cfg_regs.h
// cpu_pll_regs.h
// ic_pll_regs.h
// mc_pll_regs.h
// tpc_pll_regs.h
// dma_qm_0_regs.h
// dma_qm_1_regs.h
// dma_qm_2_regs.h
// dma_qm_3_regs.h
// dma_qm_4_regs.h
// dma_ch_0_regs.h
// dma_ch_1_regs.h
// dma_ch_2_regs.h
// dma_ch_3_regs.h
// dma_ch_4_regs.h
// dma_macro_regs.h
// dma_nrtr_regs.h
// pci_nrtr_regs.h
// sram_y0_x0_rtr_regs.h
// sram_y0_x1_rtr_regs.h
// sram_y0_x2_rtr_regs.h
// sram_y0_x3_rtr_regs.h
// sram_y0_x4_rtr_regs.h
// mme_regs.h
// mme_qm_regs.h
// mme_cmdq_regs.h
// mme1_rtr_regs.h
// mme2_rtr_regs.h
// mme3_rtr_regs.h
// mme4_rtr_regs.h
// mme5_rtr_regs.h
// mme6_rtr_regs.h
// tpc0_cfg_regs.h through tpc7_cfg_regs.h
// tpc0_qm_regs.h through tpc7_qm_regs.h
// tpc0_cmdq_regs.h through tpc7_cmdq_regs.h
// tpc0_nrtr_regs.h, tpc1_rtr_regs.h through tpc6_rtr_regs.h,
// tpc7_nrtr_regs.h
// tpc0_eml_cfg_regs.h
// psoc_etr_regs.h

// Mask dependencies retained as source-level intent.
// psoc_global_conf_masks.h, dma_macro_masks.h, dma_qm_0_masks.h,
// dma_ch_0_masks.h, tpc0_qm_masks.h, tpc0_cmdq_masks.h,
// mme_qm_masks.h, mme_cmdq_masks.h, tpc0_cfg_masks.h,
// tpc0_eml_cfg_masks.h, mme1_rtr_masks.h, tpc0_nrtr_masks.h,
// dma_nrtr_masks.h, pci_nrtr_masks.h, stlb_masks.h,
// cpu_ca53_cfg_masks.h, mmu_masks.h, mme_masks.h

pub const MMPCIE_DBI_DEVICE_ID_VENDOR_ID_REG: u32 = 0xC02000;
pub const MMPCIE_DBI_MSIX_DOORBELL_OFF: u32 = 0xC02948;

pub const MMSYNC_MNGR_MON_PAY_ADDRL_0: u32 = 0x113000;
pub const MMSYNC_MNGR_SOB_OBJ_0: u32 = 0x112000;
pub const MMSYNC_MNGR_SOB_OBJ_1000: u32 = 0x112FA0;
pub const MMSYNC_MNGR_SOB_OBJ_1007: u32 = 0x112FBC;
pub const MMSYNC_MNGR_SOB_OBJ_1023: u32 = 0x112FFC;
pub const MMSYNC_MNGR_MON_STATUS_0: u32 = 0x114000;
pub const MMSYNC_MNGR_MON_STATUS_255: u32 = 0x1143FC;

pub const MMGIC_DISTRIBUTOR__5_GICD_SETSPI_NSR: u32 = 0x800040;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
