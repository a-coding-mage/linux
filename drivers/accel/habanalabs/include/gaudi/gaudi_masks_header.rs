/* SPDX-License-Identifier: GPL-2.0
 * Copyright 2016-2020 HabanaLabs, Ltd.
 * Rust translation of gaudi_masks.h.
 *
 * The register symbols below are supplied by the corresponding register
 * definitions.  BIT_MASK, GENMASK, and FIELD_PREP are represented locally.
 */

pub const fn bit_mask(n: u32) -> u32 { 1u32 << n }
pub const fn genmask(high: u32, low: u32) -> u32 {
    (((1u64 << (high - low + 1)) - 1) << low) as u32
}
pub const fn field_prep(mask: u32, value: u32) -> u32 {
    (value << mask.trailing_zeros()) & mask
}

pub const PCI_DMA_QMAN_ENABLE: u32 = field_prep(DMA0_QM_GLBL_CFG0_PQF_EN_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_CFG0_CQF_EN_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_CFG0_CP_EN_MASK, 0xf);
pub const QMAN_EXTERNAL_MAKE_TRUSTED: u32 = field_prep(DMA0_QM_GLBL_PROT_PQF_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_PROT_CQF_MASK, 0xf) | field_prep(DMA0_QM_GLBL_PROT_CP_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_PROT_ERR_MASK, 1);
pub const QMAN_INTERNAL_MAKE_TRUSTED: u32 = field_prep(DMA0_QM_GLBL_PROT_PQF_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_PROT_ERR_MASK, 1);
pub const HBM_DMA_QMAN_ENABLE: u32 = field_prep(DMA0_QM_GLBL_CFG0_PQF_EN_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_CFG0_CQF_EN_MASK, 0x1f) | field_prep(DMA0_QM_GLBL_CFG0_CP_EN_MASK, 0x1f);
pub const QMAN_MME_ENABLE: u32 = field_prep(MME0_QM_GLBL_CFG0_PQF_EN_MASK, 0xf)
    | field_prep(MME0_QM_GLBL_CFG0_CQF_EN_MASK, 0x1f) | field_prep(MME0_QM_GLBL_CFG0_CP_EN_MASK, 0x1f);
pub const QMAN_TPC_ENABLE: u32 = field_prep(TPC0_QM_GLBL_CFG0_PQF_EN_MASK, 0xf)
    | field_prep(TPC0_QM_GLBL_CFG0_CQF_EN_MASK, 0x1f) | field_prep(TPC0_QM_GLBL_CFG0_CP_EN_MASK, 0x1f);
pub const NIC_QMAN_ENABLE: u32 = field_prep(NIC0_QM0_GLBL_CFG0_PQF_EN_MASK, 0xf)
    | field_prep(NIC0_QM0_GLBL_CFG0_CQF_EN_MASK, 0xf) | field_prep(NIC0_QM0_GLBL_CFG0_CP_EN_MASK, 0xf);
pub const QMAN_UPPER_CP_CGM_PWR_GATE_EN: u32 = field_prep(DMA0_QM_CGM_CFG_IDLE_TH_MASK, 0x20)
    | field_prep(DMA0_QM_CGM_CFG_G2F_TH_MASK, 0xa) | field_prep(DMA0_QM_CGM_CFG_CP_IDLE_MASK_MASK, 0x10)
    | field_prep(DMA0_QM_CGM_CFG_EN_MASK, 1);
pub const QMAN_COMMON_CP_CGM_PWR_GATE_EN: u32 = field_prep(DMA0_QM_CGM_CFG_IDLE_TH_MASK, 0x20)
    | field_prep(DMA0_QM_CGM_CFG_G2F_TH_MASK, 0xa) | field_prep(DMA0_QM_CGM_CFG_CP_IDLE_MASK_MASK, 0xf)
    | field_prep(DMA0_QM_CGM_CFG_EN_MASK, 1);
pub const PCI_DMA_QMAN_GLBL_ERR_CFG_MSG_EN_MASK: u32 = field_prep(DMA0_QM_GLBL_ERR_CFG_PQF_ERR_MSG_EN_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_ERR_CFG_CQF_ERR_MSG_EN_MASK, 0xf) | field_prep(DMA0_QM_GLBL_ERR_CFG_CP_ERR_MSG_EN_MASK, 0xf);
pub const PCI_DMA_QMAN_GLBL_ERR_CFG_STOP_ON_ERR_EN_MASK: u32 = field_prep(DMA0_QM_GLBL_ERR_CFG_PQF_STOP_ON_ERR_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_ERR_CFG_CQF_STOP_ON_ERR_MASK, 0xf) | field_prep(DMA0_QM_GLBL_ERR_CFG_CP_STOP_ON_ERR_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_ERR_CFG_ARB_STOP_ON_ERR_MASK, 1);
pub const HBM_DMA_QMAN_GLBL_ERR_CFG_MSG_EN_MASK: u32 = field_prep(DMA0_QM_GLBL_ERR_CFG_PQF_ERR_MSG_EN_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_ERR_CFG_CQF_ERR_MSG_EN_MASK, 0x1f) | field_prep(DMA0_QM_GLBL_ERR_CFG_CP_ERR_MSG_EN_MASK, 0x1f);
pub const HBM_DMA_QMAN_GLBL_ERR_CFG_STOP_ON_ERR_EN_MASK: u32 = field_prep(DMA0_QM_GLBL_ERR_CFG_PQF_STOP_ON_ERR_MASK, 0xf)
    | field_prep(DMA0_QM_GLBL_ERR_CFG_CQF_STOP_ON_ERR_MASK, 0x1f) | field_prep(DMA0_QM_GLBL_ERR_CFG_CP_STOP_ON_ERR_MASK, 0x1f)
    | field_prep(DMA0_QM_GLBL_ERR_CFG_ARB_STOP_ON_ERR_MASK, 1);
pub const TPC_QMAN_GLBL_ERR_CFG_MSG_EN_MASK: u32 = field_prep(TPC0_QM_GLBL_ERR_CFG_PQF_ERR_MSG_EN_MASK, 0xf)
    | field_prep(TPC0_QM_GLBL_ERR_CFG_CQF_ERR_MSG_EN_MASK, 0x1f) | field_prep(TPC0_QM_GLBL_ERR_CFG_CP_ERR_MSG_EN_MASK, 0x1f);
pub const TPC_QMAN_GLBL_ERR_CFG_STOP_ON_ERR_EN_MASK: u32 = field_prep(TPC0_QM_GLBL_ERR_CFG_PQF_STOP_ON_ERR_MASK, 0xf)
    | field_prep(TPC0_QM_GLBL_ERR_CFG_CQF_STOP_ON_ERR_MASK, 0x1f) | field_prep(TPC0_QM_GLBL_ERR_CFG_CP_STOP_ON_ERR_MASK, 0x1f)
    | field_prep(TPC0_QM_GLBL_ERR_CFG_ARB_STOP_ON_ERR_MASK, 1);
pub const MME_QMAN_GLBL_ERR_CFG_MSG_EN_MASK: u32 = field_prep(MME0_QM_GLBL_ERR_CFG_PQF_ERR_MSG_EN_MASK, 0xf)
    | field_prep(MME0_QM_GLBL_ERR_CFG_CQF_ERR_MSG_EN_MASK, 0x1f) | field_prep(MME0_QM_GLBL_ERR_CFG_CP_ERR_MSG_EN_MASK, 0x1f);
pub const MME_QMAN_GLBL_ERR_CFG_STOP_ON_ERR_EN_MASK: u32 = field_prep(MME0_QM_GLBL_ERR_CFG_PQF_STOP_ON_ERR_MASK, 0xf)
    | field_prep(MME0_QM_GLBL_ERR_CFG_CQF_STOP_ON_ERR_MASK, 0x1f) | field_prep(MME0_QM_GLBL_ERR_CFG_CP_STOP_ON_ERR_MASK, 0x1f)
    | field_prep(MME0_QM_GLBL_ERR_CFG_ARB_STOP_ON_ERR_MASK, 1);
pub const NIC_QMAN_GLBL_ERR_CFG_MSG_EN_MASK: u32 = field_prep(NIC0_QM0_GLBL_ERR_CFG_PQF_ERR_MSG_EN_MASK, 0xf)
    | field_prep(NIC0_QM0_GLBL_ERR_CFG_CQF_ERR_MSG_EN_MASK, 0xf) | field_prep(NIC0_QM0_GLBL_ERR_CFG_CP_ERR_MSG_EN_MASK, 0xf);
pub const NIC_QMAN_GLBL_ERR_CFG_STOP_ON_ERR_EN_MASK: u32 = field_prep(NIC0_QM0_GLBL_ERR_CFG_PQF_STOP_ON_ERR_MASK, 0xf)
    | field_prep(NIC0_QM0_GLBL_ERR_CFG_CQF_STOP_ON_ERR_MASK, 0xf) | field_prep(NIC0_QM0_GLBL_ERR_CFG_CP_STOP_ON_ERR_MASK, 0xf)
    | field_prep(NIC0_QM0_GLBL_ERR_CFG_ARB_STOP_ON_ERR_MASK, 1);
pub const QMAN_CGM1_PWR_GATE_EN: u32 = field_prep(DMA0_QM_CGM_CFG1_MASK_TH_MASK, 0xa);

// Reset register masks and unit shifts.
pub const CFG_RST_L_PSOC_MASK: u32 = bit_mask(0); pub const CFG_RST_L_PCIE_MASK: u32 = bit_mask(1);
pub const CFG_RST_L_PCIE_IF_MASK: u32 = bit_mask(2); pub const CFG_RST_L_HBM_S_PLL_MASK: u32 = bit_mask(3);
pub const CFG_RST_L_TPC_S_PLL_MASK: u32 = bit_mask(4); pub const CFG_RST_L_MME_S_PLL_MASK: u32 = bit_mask(5);
pub const CFG_RST_L_CPU_PLL_MASK: u32 = bit_mask(6); pub const CFG_RST_L_PCIE_PLL_MASK: u32 = bit_mask(7);
pub const CFG_RST_L_NIC_S_PLL_MASK: u32 = bit_mask(8); pub const CFG_RST_L_HBM_N_PLL_MASK: u32 = bit_mask(9);
pub const CFG_RST_L_TPC_N_PLL_MASK: u32 = bit_mask(10); pub const CFG_RST_L_MME_N_PLL_MASK: u32 = bit_mask(11);
pub const CFG_RST_L_NIC_N_PLL_MASK: u32 = bit_mask(12); pub const CFG_RST_L_DMA_W_PLL_MASK: u32 = bit_mask(13);
pub const CFG_RST_L_SIF_W_PLL_MASK: u32 = bit_mask(14); pub const CFG_RST_L_MESH_W_PLL_MASK: u32 = bit_mask(15);
pub const CFG_RST_L_SRAM_W_PLL_MASK: u32 = bit_mask(16); pub const CFG_RST_L_DMA_E_PLL_MASK: u32 = bit_mask(17);
pub const CFG_RST_L_SIF_E_PLL_MASK: u32 = bit_mask(18); pub const CFG_RST_L_MESH_E_PLL_MASK: u32 = bit_mask(19);
pub const CFG_RST_L_SRAM_E_PLL_MASK: u32 = bit_mask(20);
pub const CFG_RST_L_IF_1_MASK: u32 = bit_mask(21); pub const CFG_RST_L_IF_0_MASK: u32 = bit_mask(22);
pub const CFG_RST_L_IF_2_MASK: u32 = bit_mask(23); pub const CFG_RST_L_IF_3_MASK: u32 = bit_mask(24);
pub const CFG_RST_L_IF_MASK: u32 = genmask(24, 21);
pub const CFG_RST_L_TPC_0_MASK: u32 = bit_mask(25); pub const CFG_RST_L_TPC_1_MASK: u32 = bit_mask(26);
pub const CFG_RST_L_TPC_2_MASK: u32 = bit_mask(27); pub const CFG_RST_L_TPC_3_MASK: u32 = bit_mask(28);
pub const CFG_RST_L_TPC_4_MASK: u32 = bit_mask(29); pub const CFG_RST_L_TPC_5_MASK: u32 = bit_mask(30);
pub const CFG_RST_L_TPC_6_MASK: u32 = bit_mask(31); pub const CFG_RST_L_TPC_MASK: u32 = genmask(31, 25);
pub const CFG_RST_H_TPC_7_MASK: u32 = bit_mask(0);
pub const CFG_RST_H_MME_0_MASK: u32 = bit_mask(1); pub const CFG_RST_H_MME_1_MASK: u32 = bit_mask(2);
pub const CFG_RST_H_MME_2_MASK: u32 = bit_mask(3); pub const CFG_RST_H_MME_3_MASK: u32 = bit_mask(4); pub const CFG_RST_H_MME_MASK: u32 = genmask(4,1);
pub const CFG_RST_H_HBM_0_MASK: u32 = bit_mask(5); pub const CFG_RST_H_HBM_1_MASK: u32 = bit_mask(6);
pub const CFG_RST_H_HBM_2_MASK: u32 = bit_mask(7); pub const CFG_RST_H_HBM_3_MASK: u32 = bit_mask(8); pub const CFG_RST_H_HBM_MASK: u32 = genmask(8,5);
pub const CFG_RST_H_NIC_0_MASK: u32 = bit_mask(9); pub const CFG_RST_H_NIC_1_MASK: u32 = bit_mask(10); pub const CFG_RST_H_NIC_2_MASK: u32 = bit_mask(11); pub const CFG_RST_H_NIC_3_MASK: u32 = bit_mask(12); pub const CFG_RST_H_NIC_4_MASK: u32 = bit_mask(13); pub const CFG_RST_H_NIC_MASK: u32 = genmask(13,9);
pub const CFG_RST_H_SM_0_MASK: u32 = bit_mask(14); pub const CFG_RST_H_SM_1_MASK: u32 = bit_mask(15); pub const CFG_RST_H_SM_2_MASK: u32 = bit_mask(16); pub const CFG_RST_H_SM_3_MASK: u32 = bit_mask(17); pub const CFG_RST_H_SM_MASK: u32 = genmask(17,14);
pub const CFG_RST_H_DMA_0_MASK: u32 = bit_mask(18); pub const CFG_RST_H_DMA_1_MASK: u32 = bit_mask(19); pub const CFG_RST_H_DMA_MASK: u32 = genmask(19,18);
pub const CFG_RST_H_CPU_MASK: u32 = bit_mask(20); pub const CFG_RST_H_MMU_MASK: u32 = bit_mask(21);

// The C header's shift constants are preserved as u32 constants.
pub const UNIT_RST_L_PSOC_SHIFT:u32=0; pub const UNIT_RST_L_PCIE_SHIFT:u32=1; pub const UNIT_RST_L_PCIE_IF_SHIFT:u32=2; pub const UNIT_RST_L_HBM_S_PLL_SHIFT:u32=3; pub const UNIT_RST_L_TPC_S_PLL_SHIFT:u32=4; pub const UNIT_RST_L_MME_S_PLL_SHIFT:u32=5; pub const UNIT_RST_L_CPU_PLL_SHIFT:u32=6; pub const UNIT_RST_L_PCIE_PLL_SHIFT:u32=7; pub const UNIT_RST_L_NIC_S_PLL_SHIFT:u32=8; pub const UNIT_RST_L_HBM_N_PLL_SHIFT:u32=9; pub const UNIT_RST_L_TPC_N_PLL_SHIFT:u32=10; pub const UNIT_RST_L_MME_N_PLL_SHIFT:u32=11; pub const UNIT_RST_L_NIC_N_PLL_SHIFT:u32=12; pub const UNIT_RST_L_DMA_W_PLL_SHIFT:u32=13; pub const UNIT_RST_L_SIF_W_PLL_SHIFT:u32=14; pub const UNIT_RST_L_MESH_W_PLL_SHIFT:u32=15; pub const UNIT_RST_L_SRAM_W_PLL_SHIFT:u32=16; pub const UNIT_RST_L_DMA_E_PLL_SHIFT:u32=17; pub const UNIT_RST_L_SIF_E_PLL_SHIFT:u32=18; pub const UNIT_RST_L_MESH_E_PLL_SHIFT:u32=19; pub const UNIT_RST_L_SRAM_E_PLL_SHIFT:u32=20;
pub const UNIT_RST_L_TPC_0_SHIFT:u32=21; pub const UNIT_RST_L_TPC_1_SHIFT:u32=22; pub const UNIT_RST_L_TPC_2_SHIFT:u32=23; pub const UNIT_RST_L_TPC_3_SHIFT:u32=24; pub const UNIT_RST_L_TPC_4_SHIFT:u32=25; pub const UNIT_RST_L_TPC_5_SHIFT:u32=26; pub const UNIT_RST_L_TPC_6_SHIFT:u32=27; pub const UNIT_RST_L_TPC_7_SHIFT:u32=28; pub const UNIT_RST_L_MME_0_SHIFT:u32=29; pub const UNIT_RST_L_MME_1_SHIFT:u32=30; pub const UNIT_RST_L_MME_2_SHIFT:u32=31;
pub const UNIT_RST_H_MME_3_SHIFT:u32=0; pub const UNIT_RST_H_HBM_0_SHIFT:u32=1; pub const UNIT_RST_H_HBM_1_SHIFT:u32=2; pub const UNIT_RST_H_HBM_2_SHIFT:u32=3; pub const UNIT_RST_H_HBM_3_SHIFT:u32=4; pub const UNIT_RST_H_NIC_0_SHIFT:u32=5; pub const UNIT_RST_H_NIC_1_SHIFT:u32=6; pub const UNIT_RST_H_NIC_2_SHIFT:u32=7; pub const UNIT_RST_H_NIC_3_SHIFT:u32=8; pub const UNIT_RST_H_NIC_4_SHIFT:u32=9; pub const UNIT_RST_H_SM_0_SHIFT:u32=10; pub const UNIT_RST_H_SM_1_SHIFT:u32=11; pub const UNIT_RST_H_SM_2_SHIFT:u32=12; pub const UNIT_RST_H_SM_3_SHIFT:u32=13; pub const UNIT_RST_H_IF_0_SHIFT:u32=14; pub const UNIT_RST_H_IF_1_SHIFT:u32=15; pub const UNIT_RST_H_IF_2_SHIFT:u32=16; pub const UNIT_RST_H_IF_3_SHIFT:u32=17; pub const UNIT_RST_H_DMA_0_SHIFT:u32=18; pub const UNIT_RST_H_DMA_1_SHIFT:u32=19; pub const UNIT_RST_H_CPU_SHIFT:u32=20; pub const UNIT_RST_H_MMU_SHIFT:u32=21;
pub const UNIT_RST_H_HBM_MASK:u32=(1<<UNIT_RST_H_HBM_0_SHIFT)|(1<<UNIT_RST_H_HBM_1_SHIFT)|(1<<UNIT_RST_H_HBM_2_SHIFT)|(1<<UNIT_RST_H_HBM_3_SHIFT);
pub const UNIT_RST_H_NIC_MASK:u32=(1<<UNIT_RST_H_NIC_0_SHIFT)|(1<<UNIT_RST_H_NIC_1_SHIFT)|(1<<UNIT_RST_H_NIC_2_SHIFT)|(1<<UNIT_RST_H_NIC_3_SHIFT)|(1<<UNIT_RST_H_NIC_4_SHIFT);
pub const UNIT_RST_H_SM_MASK:u32=(1<<UNIT_RST_H_SM_0_SHIFT)|(1<<UNIT_RST_H_SM_1_SHIFT)|(1<<UNIT_RST_H_SM_2_SHIFT)|(1<<UNIT_RST_H_SM_3_SHIFT);
pub const UNIT_RST_H_MME_MASK:u32=(1<<UNIT_RST_H_MME_0_SHIFT)|(1<<UNIT_RST_H_MME_1_SHIFT)|(1<<UNIT_RST_H_MME_2_SHIFT);
pub const UNIT_RST_L_MME_MASK:u32=1<<UNIT_RST_L_MME_3_SHIFT;
pub const UNIT_RST_L_IF_MASK:u32=(1<<UNIT_RST_L_IF_0_SHIFT)|(1<<UNIT_RST_L_IF_1_SHIFT)|(1<<UNIT_RST_L_IF_2_SHIFT)|(1<<UNIT_RST_L_IF_3_SHIFT);
pub const UNIT_RST_L_TPC_MASK:u32=(1<<UNIT_RST_L_TPC_0_SHIFT)|(1<<UNIT_RST_L_TPC_1_SHIFT)|(1<<UNIT_RST_L_TPC_2_SHIFT)|(1<<UNIT_RST_L_TPC_3_SHIFT)|(1<<UNIT_RST_L_TPC_4_SHIFT)|(1<<UNIT_RST_L_TPC_5_SHIFT)|(1<<UNIT_RST_L_TPC_6_SHIFT)|(1<<UNIT_RST_L_TPC_7_SHIFT);

pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCPUPORESET_SHIFT:u32=0; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCPUPORESET_MASK:u32=0x3; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCORERESET_SHIFT:u32=4; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NCORERESET_MASK:u32=0x30; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NL2RESET_SHIFT:u32=8; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NL2RESET_MASK:u32=0x100; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NPRESETDBG_SHIFT:u32=12; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NPRESETDBG_MASK:u32=0x1000; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_SHIFT:u32=16; pub const CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_MASK:u32=0x10000; pub const CPU_CA53_CFG_ARM_RST_CONTROL_WARMRSTREQ_SHIFT:u32=20; pub const CPU_CA53_CFG_ARM_RST_CONTROL_WARMRSTREQ_MASK:u32=0x300000;
pub const CPU_RESET_ASSERT:u32=1<<CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_SHIFT;
pub const CPU_RESET_CORE0_DEASSERT:u32=(1<<CPU_CA53_CFG_ARM_RST_CONTROL_NCPUPORESET_SHIFT)|(1<<CPU_CA53_CFG_ARM_RST_CONTROL_NCORERESET_SHIFT)|(1<<CPU_CA53_CFG_ARM_RST_CONTROL_NL2RESET_SHIFT)|(1<<CPU_CA53_CFG_ARM_RST_CONTROL_NMBISTRESET_SHIFT);
pub const QM_IDLE_MASK:u32=DMA0_QM_GLBL_STS0_PQF_IDLE_MASK|DMA0_QM_GLBL_STS0_CQF_IDLE_MASK|DMA0_QM_GLBL_STS0_CP_IDLE_MASK;
pub const CGM_IDLE_MASK:u32=DMA0_QM_CGM_STS_AGENT_IDLE_MASK;
pub const TPC_IDLE_MASK:u32=(1<<TPC0_CFG_STATUS_SCALAR_PIPE_EMPTY_SHIFT)|(1<<TPC0_CFG_STATUS_VECTOR_PIPE_EMPTY_SHIFT)|(1<<TPC0_CFG_STATUS_IQ_EMPTY_SHIFT)|(1<<TPC0_CFG_STATUS_SB_EMPTY_SHIFT)|(1<<TPC0_CFG_STATUS_QM_IDLE_SHIFT)|(1<<TPC0_CFG_STATUS_QM_RDY_SHIFT);
pub const MME0_CTRL_ARCH_STATUS_SB_A_EMPTY_MASK:u32=0x80; pub const MME0_CTRL_ARCH_STATUS_SB_B_EMPTY_MASK:u32=0x100; pub const MME0_CTRL_ARCH_STATUS_WBC_AXI_IDLE_MASK:u32=0x1000;
pub const MME_ARCH_IDLE_MASK:u32=MME0_CTRL_ARCH_STATUS_SB_A_EMPTY_MASK|MME0_CTRL_ARCH_STATUS_SB_B_EMPTY_MASK|MME0_CTRL_ARCH_STATUS_WBC_AXI_IDLE_MASK;
pub const fn is_qm_idle(qm_glbl_sts0:u32,qm_cgm_sts:u32)->bool { (qm_glbl_sts0&QM_IDLE_MASK)==QM_IDLE_MASK && (qm_cgm_sts&CGM_IDLE_MASK)==CGM_IDLE_MASK }
pub const fn is_dma_idle(dma_core_sts0:u32)->bool { (dma_core_sts0&DMA0_CORE_STS0_BUSY_MASK)==0 }
pub const fn is_tpc_idle(tpc_cfg_sts:u32)->bool { (tpc_cfg_sts&TPC_IDLE_MASK)==TPC_IDLE_MASK }
pub const fn is_mme_idle(mme_arch_sts:u32)->bool { (mme_arch_sts&MME_ARCH_IDLE_MASK)==MME_ARCH_IDLE_MASK }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum axi_id { AXI_ID_MME, AXI_ID_TPC, AXI_ID_DMA, AXI_ID_NIC, AXI_ID_PCI, AXI_ID_CPU, AXI_ID_PSOC, AXI_ID_MMU, AXI_ID_NIC_FT }

pub const RAZWI_INITIATOR_AXI_ID_SHIFT:u32=20; pub const RAZWI_INITIATOR_AXI_ID_MASK:u32=0xf; pub const RAZWI_INITIATOR_X_SHIFT:u32=24; pub const RAZWI_INITIATOR_X_MASK:u32=0xf; pub const RAZWI_INITIATOR_Y_SHIFT:u32=28; pub const RAZWI_INITIATOR_Y_MASK:u32=0x7;
pub const fn razwi_initiator_id_axi_id(axi_id:u32)->u32 {(axi_id&RAZWI_INITIATOR_AXI_ID_MASK)<<RAZWI_INITIATOR_AXI_ID_SHIFT}
pub const fn razwi_initiator_id_x_y(x:u32,y:u32)->u32 {((y&RAZWI_INITIATOR_Y_MASK)<<RAZWI_INITIATOR_Y_SHIFT)|((x&RAZWI_INITIATOR_X_MASK)<<RAZWI_INITIATOR_X_SHIFT)}
pub const RAZWI_INITIATOR_ID_X_Y_TPC0_NIC0:u32=razwi_initiator_id_x_y(1,1); pub const RAZWI_INITIATOR_ID_X_Y_TPC1:u32=razwi_initiator_id_x_y(2,1); pub const RAZWI_INITIATOR_ID_X_Y_MME0_0:u32=razwi_initiator_id_x_y(3,1); pub const RAZWI_INITIATOR_ID_X_Y_MME0_1:u32=razwi_initiator_id_x_y(4,1); pub const RAZWI_INITIATOR_ID_X_Y_MME1_0:u32=razwi_initiator_id_x_y(5,1); pub const RAZWI_INITIATOR_ID_X_Y_MME1_1:u32=razwi_initiator_id_x_y(6,1); pub const RAZWI_INITIATOR_ID_X_Y_TPC2:u32=razwi_initiator_id_x_y(7,1); pub const RAZWI_INITIATOR_ID_X_Y_TPC3_PCI_CPU_PSOC:u32=razwi_initiator_id_x_y(8,1); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_W_S_0:u32=razwi_initiator_id_x_y(0,1); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_E_S_0:u32=razwi_initiator_id_x_y(9,1); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_W_S_1:u32=razwi_initiator_id_x_y(0,2); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_E_S_1:u32=razwi_initiator_id_x_y(9,2); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_W_N_0:u32=razwi_initiator_id_x_y(0,3); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_E_N_0:u32=razwi_initiator_id_x_y(9,3); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_W_N_1:u32=razwi_initiator_id_x_y(0,4); pub const RAZWI_INITIATOR_ID_X_Y_DMA_IF_E_N_1:u32=razwi_initiator_id_x_y(9,4); pub const RAZWI_INITIATOR_ID_X_Y_TPC4_NIC1_NIC2:u32=razwi_initiator_id_x_y(1,6); pub const RAZWI_INITIATOR_ID_X_Y_TPC5:u32=razwi_initiator_id_x_y(2,6); pub const RAZWI_INITIATOR_ID_X_Y_MME2_0:u32=razwi_initiator_id_x_y(3,6); pub const RAZWI_INITIATOR_ID_X_Y_MME2_1:u32=razwi_initiator_id_x_y(4,6); pub const RAZWI_INITIATOR_ID_X_Y_MME3_0:u32=razwi_initiator_id_x_y(5,6); pub const RAZWI_INITIATOR_ID_X_Y_MME3_1:u32=razwi_initiator_id_x_y(6,6); pub const RAZWI_INITIATOR_ID_X_Y_TPC6:u32=razwi_initiator_id_x_y(7,6); pub const RAZWI_INITIATOR_ID_X_Y_TPC7_NIC4_NIC5:u32=razwi_initiator_id_x_y(8,6);

pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_SHIFT:u32=1; pub const PSOC_ETR_AXICTL_PROTCTRLBIT0_MASK:u32=1; pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_MASK:u32=2; pub const PSOC_ETR_AXICTL_WRBURSTLEN_MASK:u32=0xf00;
pub const STLB_CACHE_INV_PRODUCER_INDEX_SHIFT:u32=0; pub const STLB_CACHE_INV_PRODUCER_INDEX_MASK:u32=0xff; pub const STLB_CACHE_INV_INDEX_MASK_SHIFT:u32=8; pub const STLB_CACHE_INV_INDEX_MASK_MASK:u32=0xff00;
pub const MME_ACC_ACC_STALL_R_SHIFT:u32=0; pub const MME_SBAB_SB_STALL_R_SHIFT:u32=0; pub const PCIE_WRAP_LBW_PROT_OVR_RD_EN_MASK:u32=0x700; pub const PCIE_WRAP_LBW_PROT_OVR_WR_EN_MASK:u32=0x7000; pub const PCIE_WRAP_LBW_DRAIN_CFG_EN_SHIFT:u32=0; pub const PCIE_WRAP_HBW_DRAIN_CFG_EN_SHIFT:u32=0;
pub const DMA_IF_HBM_CRED_EN_READ_CREDIT_EN_SHIFT:u32=0; pub const DMA_IF_HBM_CRED_EN_READ_CREDIT_EN_MASK:u32=1; pub const DMA_IF_HBM_CRED_EN_WRITE_CREDIT_EN_SHIFT:u32=1; pub const DMA_IF_HBM_CRED_EN_WRITE_CREDIT_EN_MASK:u32=2;
pub const DMA_IF_DOWN_CHX_SCRAM_SRAM_EN_VAL_SHIFT:u32=0; pub const DMA_IF_DOWN_CHX_SCRAM_HBM_EN_VAL_SHIFT:u32=0; pub const DMA_IF_DOWN_CHX_E2E_HBM_EN_VAL_SHIFT:u32=0; pub const DMA_IF_DOWN_CHX_E2E_PCI_EN_VAL_SHIFT:u32=0; pub const IF_RTR_CTRL_SCRAM_SRAM_EN_VAL_SHIFT:u32=0; pub const IF_RTR_CTRL_SCRAM_HBM_EN_VAL_SHIFT:u32=0; pub const IF_RTR_CTRL_E2E_HBM_EN_VAL_SHIFT:u32=0; pub const IF_RTR_CTRL_E2E_PCI_EN_VAL_SHIFT:u32=0;
pub const MMU_UP_PAGE_ERROR_CAPTURE_VA_49_32_MASK:u32=0x3ffff; pub const MMU_UP_PAGE_ERROR_CAPTURE_ENTRY_VALID_MASK:u32=0x40000; pub const MMU_UP_ACCESS_ERROR_CAPTURE_VA_49_32_MASK:u32=0x3ffff; pub const MMU_UP_ACCESS_ERROR_CAPTURE_ENTRY_VALID_MASK:u32=0x40000;
pub const QM_ARB_ERR_MSG_EN_CHOISE_OVF_MASK:u32=1; pub const QM_ARB_ERR_MSG_EN_CHOISE_WDT_MASK:u32=2; pub const QM_ARB_ERR_MSG_EN_AXI_LBW_ERR_MASK:u32=4; pub const QM_ARB_ERR_MSG_EN_MASK:u32=7;
pub const PCIE_AUX_FLR_CTRL_HW_CTRL_MASK:u32=1; pub const PCIE_AUX_FLR_CTRL_INT_MASK_MASK:u32=2;
pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_VALID_SHIFT:u32=0; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_VALID_MASK:u32=1; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_PENDING_SHIFT:u32=1; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_STATUS_0_PENDING_MASK:u32=0x1fe; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SID_SHIFT:u32=0; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SID_MASK:u32=0xff; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_MASK_SHIFT:u32=8; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_MASK_MASK:u32=0xff00; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOP_SHIFT:u32=16; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOP_MASK:u32=0x10000; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOD_SHIFT:u32=17; pub const SYNC_MNGR_W_S_SYNC_MNGR_OBJS_MON_ARM_0_SOD_MASK:u32=0xfffe0000; pub const TPC0_QM_CP_STS_0_FENCE_ID_SHIFT:u32=20; pub const TPC0_QM_CP_STS_0_FENCE_ID_MASK:u32=0x300000; pub const TPC0_QM_CP_STS_0_FENCE_IN_PROGRESS_SHIFT:u32=22; pub const TPC0_QM_CP_STS_0_FENCE_IN_PROGRESS_MASK:u32=0x400000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
