/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2020 HabanaLabs Ltd.
 * All Rights Reserved.
 */

pub const CPU_ID_SCHED_ARC0: u32 = 0; /* FARM_ARC0 */
pub const CPU_ID_SCHED_ARC1: u32 = 1; /* FARM_ARC1 */
pub const CPU_ID_SCHED_ARC2: u32 = 2; /* FARM_ARC2 */
pub const CPU_ID_SCHED_ARC3: u32 = 3; /* FARM_ARC3 */
/* Dcore1 MME Engine ARC instance used as scheduler */
pub const CPU_ID_SCHED_ARC4: u32 = 4; /* DCORE1_MME0 */
/* Dcore3 MME Engine ARC instance used as scheduler */
pub const CPU_ID_SCHED_ARC5: u32 = 5; /* DCORE3_MME0 */

pub const CPU_ID_TPC_QMAN_ARC0: u32 = 6; /* DCORE0_TPC0 */
pub const CPU_ID_TPC_QMAN_ARC1: u32 = 7; /* DCORE0_TPC1 */
pub const CPU_ID_TPC_QMAN_ARC2: u32 = 8; /* DCORE0_TPC2 */
pub const CPU_ID_TPC_QMAN_ARC3: u32 = 9; /* DCORE0_TPC3 */
pub const CPU_ID_TPC_QMAN_ARC4: u32 = 10; /* DCORE0_TPC4 */
pub const CPU_ID_TPC_QMAN_ARC5: u32 = 11; /* DCORE0_TPC5 */
pub const CPU_ID_TPC_QMAN_ARC6: u32 = 12; /* DCORE1_TPC0 */
pub const CPU_ID_TPC_QMAN_ARC7: u32 = 13; /* DCORE1_TPC1 */
pub const CPU_ID_TPC_QMAN_ARC8: u32 = 14; /* DCORE1_TPC2 */
pub const CPU_ID_TPC_QMAN_ARC9: u32 = 15; /* DCORE1_TPC3 */
pub const CPU_ID_TPC_QMAN_ARC10: u32 = 16; /* DCORE1_TPC4 */
pub const CPU_ID_TPC_QMAN_ARC11: u32 = 17; /* DCORE1_TPC5 */
pub const CPU_ID_TPC_QMAN_ARC12: u32 = 18; /* DCORE2_TPC0 */
pub const CPU_ID_TPC_QMAN_ARC13: u32 = 19; /* DCORE2_TPC1 */
pub const CPU_ID_TPC_QMAN_ARC14: u32 = 20; /* DCORE2_TPC2 */
pub const CPU_ID_TPC_QMAN_ARC15: u32 = 21; /* DCORE2_TPC3 */
pub const CPU_ID_TPC_QMAN_ARC16: u32 = 22; /* DCORE2_TPC4 */
pub const CPU_ID_TPC_QMAN_ARC17: u32 = 23; /* DCORE2_TPC5 */
pub const CPU_ID_TPC_QMAN_ARC18: u32 = 24; /* DCORE3_TPC0 */
pub const CPU_ID_TPC_QMAN_ARC19: u32 = 25; /* DCORE3_TPC1 */
pub const CPU_ID_TPC_QMAN_ARC20: u32 = 26; /* DCORE3_TPC2 */
pub const CPU_ID_TPC_QMAN_ARC21: u32 = 27; /* DCORE3_TPC3 */
pub const CPU_ID_TPC_QMAN_ARC22: u32 = 28; /* DCORE3_TPC4 */
pub const CPU_ID_TPC_QMAN_ARC23: u32 = 29; /* DCORE3_TPC5 */
pub const CPU_ID_TPC_QMAN_ARC24: u32 = 30; /* DCORE0_TPC6 - Never present */

pub const CPU_ID_MME_QMAN_ARC0: u32 = 31; /* DCORE0_MME0 */
pub const CPU_ID_MME_QMAN_ARC1: u32 = 32; /* DCORE2_MME0 */

pub const CPU_ID_EDMA_QMAN_ARC0: u32 = 33; /* DCORE0_EDMA0 */
pub const CPU_ID_EDMA_QMAN_ARC1: u32 = 34; /* DCORE0_EDMA1 */
pub const CPU_ID_EDMA_QMAN_ARC2: u32 = 35; /* DCORE1_EDMA0 */
pub const CPU_ID_EDMA_QMAN_ARC3: u32 = 36; /* DCORE1_EDMA1 */
pub const CPU_ID_EDMA_QMAN_ARC4: u32 = 37; /* DCORE2_EDMA0 */
pub const CPU_ID_EDMA_QMAN_ARC5: u32 = 38; /* DCORE2_EDMA1 */
pub const CPU_ID_EDMA_QMAN_ARC6: u32 = 39; /* DCORE3_EDMA0 */
pub const CPU_ID_EDMA_QMAN_ARC7: u32 = 40; /* DCORE3_EDMA1 */

pub const CPU_ID_PDMA_QMAN_ARC0: u32 = 41; /* DCORE0_PDMA0 */
pub const CPU_ID_PDMA_QMAN_ARC1: u32 = 42; /* DCORE0_PDMA1 */

pub const CPU_ID_ROT_QMAN_ARC0: u32 = 43; /* ROT0 */
pub const CPU_ID_ROT_QMAN_ARC1: u32 = 44; /* ROT1 */

pub const CPU_ID_NIC_QMAN_ARC0: u32 = 45; /* NIC0_0 */
pub const CPU_ID_NIC_QMAN_ARC1: u32 = 46; /* NIC0_1 */
pub const CPU_ID_NIC_QMAN_ARC2: u32 = 47; /* NIC1_0 */
pub const CPU_ID_NIC_QMAN_ARC3: u32 = 48; /* NIC1_1 */
pub const CPU_ID_NIC_QMAN_ARC4: u32 = 49; /* NIC2_0 */
pub const CPU_ID_NIC_QMAN_ARC5: u32 = 50; /* NIC2_1 */
pub const CPU_ID_NIC_QMAN_ARC6: u32 = 51; /* NIC3_0 */
pub const CPU_ID_NIC_QMAN_ARC7: u32 = 52; /* NIC3_1 */
pub const CPU_ID_NIC_QMAN_ARC8: u32 = 53; /* NIC4_0 */
pub const CPU_ID_NIC_QMAN_ARC9: u32 = 54; /* NIC4_1 */
pub const CPU_ID_NIC_QMAN_ARC10: u32 = 55; /* NIC5_0 */
pub const CPU_ID_NIC_QMAN_ARC11: u32 = 56; /* NIC5_1 */
pub const CPU_ID_NIC_QMAN_ARC12: u32 = 57; /* NIC6_0 */
pub const CPU_ID_NIC_QMAN_ARC13: u32 = 58; /* NIC6_1 */
pub const CPU_ID_NIC_QMAN_ARC14: u32 = 59; /* NIC7_0 */
pub const CPU_ID_NIC_QMAN_ARC15: u32 = 60; /* NIC7_1 */
pub const CPU_ID_NIC_QMAN_ARC16: u32 = 61; /* NIC8_0 */
pub const CPU_ID_NIC_QMAN_ARC17: u32 = 62; /* NIC8_1 */
pub const CPU_ID_NIC_QMAN_ARC18: u32 = 63; /* NIC9_0 */
pub const CPU_ID_NIC_QMAN_ARC19: u32 = 64; /* NIC9_1 */
pub const CPU_ID_NIC_QMAN_ARC20: u32 = 65; /* NIC10_0 */
pub const CPU_ID_NIC_QMAN_ARC21: u32 = 66; /* NIC10_1 */
pub const CPU_ID_NIC_QMAN_ARC22: u32 = 67; /* NIC11_0 */
pub const CPU_ID_NIC_QMAN_ARC23: u32 = 68; /* NIC11_1 */

pub const CPU_ID_MAX: u32 = 69;
pub const CPU_ID_SCHED_MAX: u32 = 6;
pub const CPU_ID_ALL: u32 = 0xFE;
pub const CPU_ID_INVALID: u32 = 0xFF;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcRegionsT {
    ARC_REGION0_UNSED = 0,
    /*
     * Extension registers
     * None
     */
    ARC_REGION1_SRAM = 1,
    /*
     * Extension registers
     * AUX_SRAM_LSB_ADDR
     * AUX_SRAM_MSB_ADDR
     * ARC Address: 0x1000_0000
     */
    ARC_REGION2_CFG = 2,
    /*
     * Extension registers
     * AUX_CFG_LSB_ADDR
     * AUX_CFG_MSB_ADDR
     * ARC Address: 0x2000_0000
     */
    ARC_REGION3_GENERAL = 3,
    /*
     * Extension registers
     * AUX_GENERAL_PURPOSE_LSB_ADDR_0
     * AUX_GENERAL_PURPOSE_MSB_ADDR_0
     * ARC Address: 0x3000_0000
     */
    ARC_REGION4_HBM0_FW = 4,
    /*
     * Extension registers
     * AUX_HBM0_LSB_ADDR
     * AUX_HBM0_MSB_ADDR
     * AUX_HBM0_OFFSET
     * ARC Address: 0x4000_0000
     */
    ARC_REGION5_HBM1_GC_DATA = 5,
    /*
     * Extension registers
     * AUX_HBM1_LSB_ADDR
     * AUX_HBM1_MSB_ADDR
     * AUX_HBM1_OFFSET
     * ARC Address: 0x5000_0000
     */
    ARC_REGION6_HBM2_GC_DATA = 6,
    /*
     * Extension registers
     * AUX_HBM2_LSB_ADDR
     * AUX_HBM2_MSB_ADDR
     * AUX_HBM2_OFFSET
     * ARC Address: 0x6000_0000
     */
    ARC_REGION7_HBM3_GC_DATA = 7,
    /*
     * Extension registers
     * AUX_HBM3_LSB_ADDR
     * AUX_HBM3_MSB_ADDR
     * AUX_HBM3_OFFSET
     * ARC Address: 0x7000_0000
     */
    ARC_REGION8_DCCM = 8,
    /*
     * Extension registers
     * None
     * ARC Address: 0x8000_0000
     */
    ARC_REGION9_PCIE = 9,
    /*
     * Extension registers
     * AUX_PCIE_LSB_ADDR
     * AUX_PCIE_MSB_ADDR
     * ARC Address: 0x9000_0000
     */
    ARC_REGION10_GENERAL = 10,
    /*
     * Extension registers
     * AUX_GENERAL_PURPOSE_LSB_ADDR_1
     * AUX_GENERAL_PURPOSE_MSB_ADDR_1
     * ARC Address: 0xA000_0000
     */
    ARC_REGION11_GENERAL = 11,
    /*
     * Extension registers
     * AUX_GENERAL_PURPOSE_LSB_ADDR_2
     * AUX_GENERAL_PURPOSE_MSB_ADDR_2
     * ARC Address: 0xB000_0000
     */
    ARC_REGION12_GENERAL = 12,
    /*
     * Extension registers
     * AUX_GENERAL_PURPOSE_LSB_ADDR_3
     * AUX_GENERAL_PURPOSE_MSB_ADDR_3
     * ARC Address: 0xC000_0000
     */
    ARC_REGION13_GENERAL = 13,
    /*
     * Extension registers
     * AUX_GENERAL_PURPOSE_LSB_ADDR_4
     * AUX_GENERAL_PURPOSE_MSB_ADDR_4
     * ARC Address: 0xD000_0000
     */
    ARC_REGION14_GENERAL = 14,
    /*
     * Extension registers
     * AUX_GENERAL_PURPOSE_LSB_ADDR_5
     * AUX_GENERAL_PURPOSE_MSB_ADDR_5
     * ARC Address: 0xE000_0000
     */
    ARC_REGION15_LBU = 15,
    /*
     * Extension registers
     * None
     * ARC Address: 0xF000_0000
     */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
