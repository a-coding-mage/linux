/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */


// dependency: #include "gaudi_blocks.h"
// dependency: #include "psoc_global_conf_regs.h"
// dependency: #include "psoc_timestamp_regs.h"
// dependency: #include "cpu_if_regs.h"
// dependency: #include "mmu_up_regs.h"
// dependency: #include "stlb_regs.h"
// dependency: #include "dma0_qm_regs.h"
// dependency: #include "dma1_qm_regs.h"
// dependency: #include "dma2_qm_regs.h"
// dependency: #include "dma3_qm_regs.h"
// dependency: #include "dma4_qm_regs.h"
// dependency: #include "dma5_qm_regs.h"
// dependency: #include "dma6_qm_regs.h"
// dependency: #include "dma7_qm_regs.h"
// dependency: #include "dma0_core_regs.h"
// dependency: #include "dma1_core_regs.h"
// dependency: #include "dma2_core_regs.h"
// dependency: #include "dma3_core_regs.h"
// dependency: #include "dma4_core_regs.h"
// dependency: #include "dma5_core_regs.h"
// dependency: #include "dma6_core_regs.h"
// dependency: #include "dma7_core_regs.h"
// dependency: #include "mme0_ctrl_regs.h"
// dependency: #include "mme1_ctrl_regs.h"
// dependency: #include "mme2_ctrl_regs.h"
// dependency: #include "mme3_ctrl_regs.h"
// dependency: #include "mme0_qm_regs.h"
// dependency: #include "mme2_qm_regs.h"
// dependency: #include "tpc0_cfg_regs.h"
// dependency: #include "tpc1_cfg_regs.h"
// dependency: #include "tpc2_cfg_regs.h"
// dependency: #include "tpc3_cfg_regs.h"
// dependency: #include "tpc4_cfg_regs.h"
// dependency: #include "tpc5_cfg_regs.h"
// dependency: #include "tpc6_cfg_regs.h"
// dependency: #include "tpc7_cfg_regs.h"
// dependency: #include "tpc0_qm_regs.h"
// dependency: #include "tpc1_qm_regs.h"
// dependency: #include "tpc2_qm_regs.h"
// dependency: #include "tpc3_qm_regs.h"
// dependency: #include "tpc4_qm_regs.h"
// dependency: #include "tpc5_qm_regs.h"
// dependency: #include "tpc6_qm_regs.h"
// dependency: #include "tpc7_qm_regs.h"
// dependency: #include "dma_if_e_n_down_ch0_regs.h"
// dependency: #include "dma_if_e_n_down_ch1_regs.h"
// dependency: #include "dma_if_e_s_down_ch0_regs.h"
// dependency: #include "dma_if_e_s_down_ch1_regs.h"
// dependency: #include "dma_if_w_n_down_ch0_regs.h"
// dependency: #include "dma_if_w_n_down_ch1_regs.h"
// dependency: #include "dma_if_w_s_down_ch0_regs.h"
// dependency: #include "dma_if_w_s_down_ch1_regs.h"
// dependency: #include "dma_if_e_n_regs.h"
// dependency: #include "dma_if_e_s_regs.h"
// dependency: #include "dma_if_w_n_regs.h"
// dependency: #include "dma_if_w_s_regs.h"
// dependency: #include "nif_rtr_ctrl_0_regs.h"
// dependency: #include "nif_rtr_ctrl_1_regs.h"
// dependency: #include "nif_rtr_ctrl_2_regs.h"
// dependency: #include "nif_rtr_ctrl_3_regs.h"
// dependency: #include "nif_rtr_ctrl_4_regs.h"
// dependency: #include "nif_rtr_ctrl_5_regs.h"
// dependency: #include "nif_rtr_ctrl_6_regs.h"
// dependency: #include "nif_rtr_ctrl_7_regs.h"
// dependency: #include "sif_rtr_ctrl_0_regs.h"
// dependency: #include "sif_rtr_ctrl_1_regs.h"
// dependency: #include "sif_rtr_ctrl_2_regs.h"
// dependency: #include "sif_rtr_ctrl_3_regs.h"
// dependency: #include "sif_rtr_ctrl_4_regs.h"
// dependency: #include "sif_rtr_ctrl_5_regs.h"
// dependency: #include "sif_rtr_ctrl_6_regs.h"
// dependency: #include "sif_rtr_ctrl_7_regs.h"
// dependency: #include "psoc_etr_regs.h"
// dependency: #include "psoc_cpu_pll_regs.h"

// dependency: #include "dma0_qm_masks.h"
// dependency: #include "mme0_qm_masks.h"
// dependency: #include "tpc0_qm_masks.h"
// dependency: #include "dma0_core_masks.h"
// dependency: #include "tpc0_cfg_masks.h"
// dependency: #include "psoc_global_conf_masks.h"

// dependency: #include "nic0_qm0_regs.h"
// dependency: #include "nic1_qm0_regs.h"
// dependency: #include "nic2_qm0_regs.h"
// dependency: #include "nic3_qm0_regs.h"
// dependency: #include "nic4_qm0_regs.h"
// dependency: #include "nic0_qm1_regs.h"
// dependency: #include "nic1_qm1_regs.h"
// dependency: #include "nic2_qm1_regs.h"
// dependency: #include "nic3_qm1_regs.h"
// dependency: #include "nic4_qm1_regs.h"

// dependency: #include "nic0_qm0_masks.h"

pub const GAUDI_ECC_MEM_SEL_OFFSET: u32 = 0xF18;
pub const GAUDI_ECC_ADDRESS_OFFSET: u32 = 0xF1C;
pub const GAUDI_ECC_SYNDROME_OFFSET: u32 = 0xF20;
pub const GAUDI_ECC_MEM_INFO_CLR_OFFSET: u32 = 0xF28;
pub const GAUDI_ECC_MEM_INFO_CLR_SERR_MASK: u32 = ((1u32) << 8);
pub const GAUDI_ECC_MEM_INFO_CLR_DERR_MASK: u32 = ((1u32) << 9);
pub const GAUDI_ECC_SERR0_OFFSET: u32 = 0xF30;
pub const GAUDI_ECC_DERR0_OFFSET: u32 = 0xF40;

pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_SOB_OBJ_0: u32 = 0x492000;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_PAY_ADDRL_0: u32 = 0x494000;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_PAY_ADDRH_0: u32 = 0x494800;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_PAY_DATA_0: u32 = 0x495000;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0: u32 = 0x495800;
pub const mmSYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0: u32 = 0x496000;
pub const mmSYNC_MNGR_E_S_SYNC_MNGR_OBJS_SOB_OBJ_0: u32 = 0x4B2000;
pub const mmSYNC_MNGR_E_S_SYNC_MNGR_OBJS_MON_STATUS_0: u32 = 0x4B6000;
pub const mmSYNC_MNGR_W_N_SYNC_MNGR_OBJS_SOB_OBJ_0: u32 = 0x4D2000;
pub const mmSYNC_MNGR_W_N_SYNC_MNGR_OBJS_MON_STATUS_0: u32 = 0x4D6000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_0: u32 = 0x4F2000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_1: u32 = 0x4F2004;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_2047: u32 = 0x4F3FFC;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_PAY_ADDRL_0: u32 = 0x4F4000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_PAY_ADDRH_0: u32 = 0x4F4800;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_PAY_DATA_0: u32 = 0x4F5000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_ARM_0: u32 = 0x4F5800;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_STATUS_0: u32 = 0x4F6000;
pub const mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_STATUS_511: u32 = 0x4F67FC;

pub const mmSIF_RTR_0_LBW_RANGE_PROT_HIT_AW: u32 = 0x300400;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_HIT_AW: u32 = 0x310400;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_HIT_AW: u32 = 0x320400;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_HIT_AW: u32 = 0x330400;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_HIT_AW: u32 = 0x340400;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_HIT_AW: u32 = 0x350400;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_HIT_AW: u32 = 0x360400;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_HIT_AW: u32 = 0x370400;

pub const mmSIF_RTR_0_LBW_RANGE_PROT_HIT_AR: u32 = 0x300490;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_HIT_AR: u32 = 0x310490;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_HIT_AR: u32 = 0x320490;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_HIT_AR: u32 = 0x330490;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_HIT_AR: u32 = 0x340490;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_HIT_AR: u32 = 0x350490;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_HIT_AR: u32 = 0x360490;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_HIT_AR: u32 = 0x370490;

pub const mmSIF_RTR_0_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x300410;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x310410;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x320410;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x330410;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x340410;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x350410;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x360410;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x370410;

pub const mmSIF_RTR_0_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x300450;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x310450;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x320450;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x330450;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x340450;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x350450;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x360450;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x370450;

pub const mmSIF_RTR_0_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3004A0;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3104A0;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3204A0;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3304A0;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3404A0;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3504A0;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3604A0;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3704A0;

pub const mmSIF_RTR_0_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3004E0;
pub const mmSIF_RTR_1_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3104E0;
pub const mmSIF_RTR_2_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3204E0;
pub const mmSIF_RTR_3_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3304E0;
pub const mmSIF_RTR_4_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3404E0;
pub const mmSIF_RTR_5_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3504E0;
pub const mmSIF_RTR_6_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3604E0;
pub const mmSIF_RTR_7_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3704E0;

pub const mmNIF_RTR_0_LBW_RANGE_PROT_HIT_AW: u32 = 0x380400;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_HIT_AW: u32 = 0x390400;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_HIT_AW: u32 = 0x3A0400;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_HIT_AW: u32 = 0x3B0400;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_HIT_AW: u32 = 0x3C0400;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_HIT_AW: u32 = 0x3D0400;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_HIT_AW: u32 = 0x3E0400;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_HIT_AW: u32 = 0x3F0400;

pub const mmNIF_RTR_0_LBW_RANGE_PROT_HIT_AR: u32 = 0x380490;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_HIT_AR: u32 = 0x390490;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_HIT_AR: u32 = 0x3A0490;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_HIT_AR: u32 = 0x3B0490;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_HIT_AR: u32 = 0x3C0490;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_HIT_AR: u32 = 0x3D0490;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_HIT_AR: u32 = 0x3E0490;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_HIT_AR: u32 = 0x3F0490;

pub const mmNIF_RTR_0_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x380410;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x390410;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x3A0410;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x3B0410;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x3C0410;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x3D0410;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x3E0410;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MIN_AW_0: u32 = 0x3F0410;

pub const mmNIF_RTR_0_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x380450;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x390450;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x3A0450;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x3B0450;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x3C0450;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x3D0450;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x3E0450;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MAX_AW_0: u32 = 0x3F0450;

pub const mmNIF_RTR_0_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3804A0;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3904A0;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3A04A0;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3B04A0;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3C04A0;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3D04A0;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3E04A0;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MIN_AR_0: u32 = 0x3F04A0;

pub const mmNIF_RTR_0_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3804E0;
pub const mmNIF_RTR_1_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3904E0;
pub const mmNIF_RTR_2_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3A04E0;
pub const mmNIF_RTR_3_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3B04E0;
pub const mmNIF_RTR_4_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3C04E0;
pub const mmNIF_RTR_5_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3D04E0;
pub const mmNIF_RTR_6_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3E04E0;
pub const mmNIF_RTR_7_LBW_RANGE_PROT_MAX_AR_0: u32 = 0x3F04E0;

pub const mmDMA_IF_W_S_DOWN_RSP_MID_WGHT_0: u32 = 0x489030;
pub const mmDMA_IF_W_S_DOWN_RSP_MID_WGHT_1: u32 = 0x489034;

pub const mmDMA_IF_E_S_DOWN_RSP_MID_WGHT_0: u32 = 0x4A9030;
pub const mmDMA_IF_E_S_DOWN_RSP_MID_WGHT_1: u32 = 0x4A9034;

pub const mmDMA_IF_W_N_DOWN_RSP_MID_WGHT_0: u32 = 0x4C9030;
pub const mmDMA_IF_W_N_DOWN_RSP_MID_WGHT_1: u32 = 0x4C9034;

pub const mmDMA_IF_E_N_DOWN_RSP_MID_WGHT_0: u32 = 0x4E9030;
pub const mmDMA_IF_E_N_DOWN_RSP_MID_WGHT_1: u32 = 0x4E9034;

pub const mmMME1_QM_GLBL_CFG0: u32 = 0xE8000;
pub const mmMME1_QM_GLBL_STS0: u32 = 0xE8038;

pub const mmMME0_SBAB_SB_STALL: u32 = 0x4002C;
pub const mmMME0_SBAB_ARUSER0: u32 = 0x40034;
pub const mmMME0_SBAB_ARUSER1: u32 = 0x40038;
pub const mmMME0_SBAB_PROT: u32 = 0x40050;

pub const mmMME1_SBAB_SB_STALL: u32 = 0xC002C;
pub const mmMME1_SBAB_ARUSER0: u32 = 0xC0034;
pub const mmMME1_SBAB_ARUSER1: u32 = 0xC0038;
pub const mmMME1_SBAB_PROT: u32 = 0xC0050;

pub const mmMME2_SBAB_SB_STALL: u32 = 0x14002C;
pub const mmMME2_SBAB_ARUSER0: u32 = 0x140034;
pub const mmMME2_SBAB_ARUSER1: u32 = 0x140038;
pub const mmMME2_SBAB_PROT: u32 = 0x140050;

pub const mmMME3_SBAB_SB_STALL: u32 = 0x1C002C;
pub const mmMME3_SBAB_ARUSER0: u32 = 0x1C0034;
pub const mmMME3_SBAB_ARUSER1: u32 = 0x1C0038;
pub const mmMME3_SBAB_PROT: u32 = 0x1C0050;

pub const mmMME0_ACC_ACC_STALL: u32 = 0x20028;
pub const mmMME0_ACC_WBC: u32 = 0x20038;
pub const mmMME0_ACC_PROT: u32 = 0x20050;

pub const mmMME1_ACC_ACC_STALL: u32 = 0xA0028;
pub const mmMME1_ACC_WBC: u32 = 0xA0038;
pub const mmMME1_ACC_PROT: u32 = 0xA0050;

pub const mmMME2_ACC_ACC_STALL: u32 = 0x120028;
pub const mmMME2_ACC_WBC: u32 = 0x120038;
pub const mmMME2_ACC_PROT: u32 = 0x120050;

pub const mmMME3_ACC_ACC_STALL: u32 = 0x1A0028;
pub const mmMME3_ACC_WBC: u32 = 0x1A0038;
pub const mmMME3_ACC_PROT: u32 = 0x1A0050;

pub const mmGIC_DISTRIBUTOR__5_GICD_SETSPI_NSR: u32 = 0x800040;

pub const mmPSOC_EFUSE_READ: u32 = 0xC4A000;
pub const mmPSOC_EFUSE_DATA_0: u32 = 0xC4A080;

pub const mmPCIE_WRAP_MAX_OUTSTAND: u32 = 0xC01B20;
pub const mmPCIE_WRAP_LBW_PROT_OVR: u32 = 0xC01B48;
pub const mmPCIE_WRAP_HBW_DRAIN_CFG: u32 = 0xC01D54;
pub const mmPCIE_WRAP_LBW_DRAIN_CFG: u32 = 0xC01D5C;

pub const mmPCIE_MSI_INTR_0: u32 = 0xC13000;

pub const mmPCIE_DBI_DEVICE_ID_VENDOR_ID_REG: u32 = 0xC02000;

pub const mmPCIE_AUX_FLR_CTRL: u32 = 0xC07394;
pub const mmPCIE_AUX_DBI: u32 = 0xC07490;

pub const mmPCIE_CORE_MSI_REQ: u32 = 0xC04100;

pub const mmPSOC_PCI_PLL_NR: u32 = 0xC72100;
pub const mmSRAM_W_PLL_NR: u32 = 0x4C8100;
pub const mmPSOC_HBM_PLL_NR: u32 = 0xC74100;
pub const mmNIC0_PLL_NR: u32 = 0xCF9100;
pub const mmDMA_W_PLL_NR: u32 = 0x487100;
pub const mmMESH_W_PLL_NR: u32 = 0x4C7100;
pub const mmPSOC_MME_PLL_NR: u32 = 0xC71100;
pub const mmPSOC_TPC_PLL_NR: u32 = 0xC73100;
pub const mmIF_W_PLL_NR: u32 = 0x488100;

pub const mmPCIE_WRAP_RR_ELBI_RD_SEC_REG_CTRL: u32 = 0xC01208;

#endif /* ASIC_REG_GAUDI_REGS_H_ */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
