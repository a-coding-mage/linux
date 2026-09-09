/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */



// addressBlock: smuio_smuio_reset_SmuSmuioDec
// base address: 0x5a300
pub const regSMUIO_MP_RESET_INTR: u32 = 0x00c1;
pub const regSMUIO_MP_RESET_INTR_BASE_IDX: u32 = 0;
pub const regSMUIO_SOC_HALT: u32 = 0x00c2;
pub const regSMUIO_SOC_HALT_BASE_IDX: u32 = 0;


// addressBlock: smuio_smuio_tsc_SmuSmuioDec
// base address: 0x5a8a0
pub const regPWROK_REFCLK_GAP_CYCLES: u32 = 0x0028;
pub const regPWROK_REFCLK_GAP_CYCLES_BASE_IDX: u32 = 1;
pub const regGOLDEN_TSC_INCREMENT_UPPER: u32 = 0x002b;
pub const regGOLDEN_TSC_INCREMENT_UPPER_BASE_IDX: u32 = 1;
pub const regGOLDEN_TSC_INCREMENT_LOWER: u32 = 0x002c;
pub const regGOLDEN_TSC_INCREMENT_LOWER_BASE_IDX: u32 = 1;
pub const regGOLDEN_TSC_COUNT_UPPER: u32 = 0x002d;
pub const regGOLDEN_TSC_COUNT_UPPER_BASE_IDX: u32 = 1;
pub const regGOLDEN_TSC_COUNT_LOWER: u32 = 0x002e;
pub const regGOLDEN_TSC_COUNT_LOWER_BASE_IDX: u32 = 1;
pub const regSOC_GOLDEN_TSC_SHADOW_UPPER: u32 = 0x002f;
pub const regSOC_GOLDEN_TSC_SHADOW_UPPER_BASE_IDX: u32 = 1;
pub const regSOC_GOLDEN_TSC_SHADOW_LOWER: u32 = 0x0030;
pub const regSOC_GOLDEN_TSC_SHADOW_LOWER_BASE_IDX: u32 = 1;
pub const regSOC_GAP_PWROK: u32 = 0x0031;
pub const regSOC_GAP_PWROK_BASE_IDX: u32 = 1;


// addressBlock: smuio_smuio_swtimer_SmuSmuioDec
// base address: 0x5ac70
pub const regPWR_DISP_TIMER_CONTROL: u32 = 0x011d;
pub const regPWR_DISP_TIMER_CONTROL_BASE_IDX: u32 = 1;
pub const regPWR_DISP_TIMER_DEBUG: u32 = 0x011e;
pub const regPWR_DISP_TIMER_DEBUG_BASE_IDX: u32 = 1;
pub const regPWR_DISP_TIMER2_CONTROL: u32 = 0x011f;
pub const regPWR_DISP_TIMER2_CONTROL_BASE_IDX: u32 = 1;
pub const regPWR_DISP_TIMER2_DEBUG: u32 = 0x0120;
pub const regPWR_DISP_TIMER2_DEBUG_BASE_IDX: u32 = 1;
pub const regPWR_DISP_TIMER_GLOBAL_CONTROL: u32 = 0x0121;
pub const regPWR_DISP_TIMER_GLOBAL_CONTROL_BASE_IDX: u32 = 1;
pub const regPWR_IH_CONTROL: u32 = 0x0122;
pub const regPWR_IH_CONTROL_BASE_IDX: u32 = 1;


// addressBlock: smuio_smuio_misc_SmuSmuioDec
// base address: 0x5a000
pub const regSMUIO_MCM_CONFIG: u32 = 0x0023;
pub const regSMUIO_MCM_CONFIG_BASE_IDX: u32 = 0;
pub const regIP_DISCOVERY_VERSION: u32 = 0x0000;
pub const regIP_DISCOVERY_VERSION_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER0: u32 = 0x01bd;
pub const regSCRATCH_REGISTER0_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER1: u32 = 0x01be;
pub const regSCRATCH_REGISTER1_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER2: u32 = 0x01bf;
pub const regSCRATCH_REGISTER2_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER3: u32 = 0x01c0;
pub const regSCRATCH_REGISTER3_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER4: u32 = 0x01c1;
pub const regSCRATCH_REGISTER4_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER5: u32 = 0x01c2;
pub const regSCRATCH_REGISTER5_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER6: u32 = 0x01c3;
pub const regSCRATCH_REGISTER6_BASE_IDX: u32 = 1;
pub const regSCRATCH_REGISTER7: u32 = 0x01c4;
pub const regSCRATCH_REGISTER7_BASE_IDX: u32 = 1;


// addressBlock: smuio_smuio_i2c_SmuSmuioDec
// base address: 0x5a100
pub const regCKSVII2C_IC_CON: u32 = 0x0040;
pub const regCKSVII2C_IC_CON_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_TAR: u32 = 0x0041;
pub const regCKSVII2C_IC_TAR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_SAR: u32 = 0x0042;
pub const regCKSVII2C_IC_SAR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_HS_MADDR: u32 = 0x0043;
pub const regCKSVII2C_IC_HS_MADDR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_DATA_CMD: u32 = 0x0044;
pub const regCKSVII2C_IC_DATA_CMD_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_SS_SCL_HCNT: u32 = 0x0045;
pub const regCKSVII2C_IC_SS_SCL_HCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_SS_SCL_LCNT: u32 = 0x0046;
pub const regCKSVII2C_IC_SS_SCL_LCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_FS_SCL_HCNT: u32 = 0x0047;
pub const regCKSVII2C_IC_FS_SCL_HCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_FS_SCL_LCNT: u32 = 0x0048;
pub const regCKSVII2C_IC_FS_SCL_LCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_HS_SCL_HCNT: u32 = 0x0049;
pub const regCKSVII2C_IC_HS_SCL_HCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_HS_SCL_LCNT: u32 = 0x004a;
pub const regCKSVII2C_IC_HS_SCL_LCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_INTR_STAT: u32 = 0x004b;
pub const regCKSVII2C_IC_INTR_STAT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_INTR_MASK: u32 = 0x004c;
pub const regCKSVII2C_IC_INTR_MASK_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_RAW_INTR_STAT: u32 = 0x004d;
pub const regCKSVII2C_IC_RAW_INTR_STAT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_RX_TL: u32 = 0x004e;
pub const regCKSVII2C_IC_RX_TL_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_TX_TL: u32 = 0x004f;
pub const regCKSVII2C_IC_TX_TL_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_INTR: u32 = 0x0050;
pub const regCKSVII2C_IC_CLR_INTR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_RX_UNDER: u32 = 0x0051;
pub const regCKSVII2C_IC_CLR_RX_UNDER_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_RX_OVER: u32 = 0x0052;
pub const regCKSVII2C_IC_CLR_RX_OVER_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_TX_OVER: u32 = 0x0053;
pub const regCKSVII2C_IC_CLR_TX_OVER_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_RD_REQ: u32 = 0x0054;
pub const regCKSVII2C_IC_CLR_RD_REQ_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_TX_ABRT: u32 = 0x0055;
pub const regCKSVII2C_IC_CLR_TX_ABRT_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_RX_DONE: u32 = 0x0056;
pub const regCKSVII2C_IC_CLR_RX_DONE_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_ACTIVITY: u32 = 0x0057;
pub const regCKSVII2C_IC_CLR_ACTIVITY_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_STOP_DET: u32 = 0x0058;
pub const regCKSVII2C_IC_CLR_STOP_DET_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_START_DET: u32 = 0x0059;
pub const regCKSVII2C_IC_CLR_START_DET_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_GEN_CALL: u32 = 0x005a;
pub const regCKSVII2C_IC_CLR_GEN_CALL_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_ENABLE: u32 = 0x005b;
pub const regCKSVII2C_IC_ENABLE_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_STATUS: u32 = 0x005c;
pub const regCKSVII2C_IC_STATUS_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_TXFLR: u32 = 0x005d;
pub const regCKSVII2C_IC_TXFLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_RXFLR: u32 = 0x005e;
pub const regCKSVII2C_IC_RXFLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_SDA_HOLD: u32 = 0x005f;
pub const regCKSVII2C_IC_SDA_HOLD_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_TX_ABRT_SOURCE: u32 = 0x0060;
pub const regCKSVII2C_IC_TX_ABRT_SOURCE_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_SLV_DATA_NACK_ONLY: u32 = 0x0061;
pub const regCKSVII2C_IC_SLV_DATA_NACK_ONLY_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_DMA_CR: u32 = 0x0062;
pub const regCKSVII2C_IC_DMA_CR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_DMA_TDLR: u32 = 0x0063;
pub const regCKSVII2C_IC_DMA_TDLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_DMA_RDLR: u32 = 0x0064;
pub const regCKSVII2C_IC_DMA_RDLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_SDA_SETUP: u32 = 0x0065;
pub const regCKSVII2C_IC_SDA_SETUP_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_ACK_GENERAL_CALL: u32 = 0x0066;
pub const regCKSVII2C_IC_ACK_GENERAL_CALL_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_ENABLE_STATUS: u32 = 0x0067;
pub const regCKSVII2C_IC_ENABLE_STATUS_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_FS_SPKLEN: u32 = 0x0068;
pub const regCKSVII2C_IC_FS_SPKLEN_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_HS_SPKLEN: u32 = 0x0069;
pub const regCKSVII2C_IC_HS_SPKLEN_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_CLR_RESTART_DET: u32 = 0x006a;
pub const regCKSVII2C_IC_CLR_RESTART_DET_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_COMP_PARAM_1: u32 = 0x006d;
pub const regCKSVII2C_IC_COMP_PARAM_1_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_COMP_VERSION: u32 = 0x006e;
pub const regCKSVII2C_IC_COMP_VERSION_BASE_IDX: u32 = 0;
pub const regCKSVII2C_IC_COMP_TYPE: u32 = 0x006f;
pub const regCKSVII2C_IC_COMP_TYPE_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CON: u32 = 0x0080;
pub const regCKSVII2C1_IC_CON_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_TAR: u32 = 0x0081;
pub const regCKSVII2C1_IC_TAR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_SAR: u32 = 0x0082;
pub const regCKSVII2C1_IC_SAR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_HS_MADDR: u32 = 0x0083;
pub const regCKSVII2C1_IC_HS_MADDR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_DATA_CMD: u32 = 0x0084;
pub const regCKSVII2C1_IC_DATA_CMD_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_SS_SCL_HCNT: u32 = 0x0085;
pub const regCKSVII2C1_IC_SS_SCL_HCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_SS_SCL_LCNT: u32 = 0x0086;
pub const regCKSVII2C1_IC_SS_SCL_LCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_FS_SCL_HCNT: u32 = 0x0087;
pub const regCKSVII2C1_IC_FS_SCL_HCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_FS_SCL_LCNT: u32 = 0x0088;
pub const regCKSVII2C1_IC_FS_SCL_LCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_HS_SCL_HCNT: u32 = 0x0089;
pub const regCKSVII2C1_IC_HS_SCL_HCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_HS_SCL_LCNT: u32 = 0x008a;
pub const regCKSVII2C1_IC_HS_SCL_LCNT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_INTR_STAT: u32 = 0x008b;
pub const regCKSVII2C1_IC_INTR_STAT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_INTR_MASK: u32 = 0x008c;
pub const regCKSVII2C1_IC_INTR_MASK_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_RAW_INTR_STAT: u32 = 0x008d;
pub const regCKSVII2C1_IC_RAW_INTR_STAT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_RX_TL: u32 = 0x008e;
pub const regCKSVII2C1_IC_RX_TL_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_TX_TL: u32 = 0x008f;
pub const regCKSVII2C1_IC_TX_TL_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_INTR: u32 = 0x0090;
pub const regCKSVII2C1_IC_CLR_INTR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_RX_UNDER: u32 = 0x0091;
pub const regCKSVII2C1_IC_CLR_RX_UNDER_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_RX_OVER: u32 = 0x0092;
pub const regCKSVII2C1_IC_CLR_RX_OVER_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_TX_OVER: u32 = 0x0093;
pub const regCKSVII2C1_IC_CLR_TX_OVER_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_RD_REQ: u32 = 0x0094;
pub const regCKSVII2C1_IC_CLR_RD_REQ_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_TX_ABRT: u32 = 0x0095;
pub const regCKSVII2C1_IC_CLR_TX_ABRT_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_RX_DONE: u32 = 0x0096;
pub const regCKSVII2C1_IC_CLR_RX_DONE_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_ACTIVITY: u32 = 0x0097;
pub const regCKSVII2C1_IC_CLR_ACTIVITY_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_STOP_DET: u32 = 0x0098;
pub const regCKSVII2C1_IC_CLR_STOP_DET_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_START_DET: u32 = 0x0099;
pub const regCKSVII2C1_IC_CLR_START_DET_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_GEN_CALL: u32 = 0x009a;
pub const regCKSVII2C1_IC_CLR_GEN_CALL_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_ENABLE: u32 = 0x009b;
pub const regCKSVII2C1_IC_ENABLE_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_STATUS: u32 = 0x009c;
pub const regCKSVII2C1_IC_STATUS_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_TXFLR: u32 = 0x009d;
pub const regCKSVII2C1_IC_TXFLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_RXFLR: u32 = 0x009e;
pub const regCKSVII2C1_IC_RXFLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_SDA_HOLD: u32 = 0x009f;
pub const regCKSVII2C1_IC_SDA_HOLD_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_TX_ABRT_SOURCE: u32 = 0x00a0;
pub const regCKSVII2C1_IC_TX_ABRT_SOURCE_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_SLV_DATA_NACK_ONLY: u32 = 0x00a1;
pub const regCKSVII2C1_IC_SLV_DATA_NACK_ONLY_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_DMA_CR: u32 = 0x00a2;
pub const regCKSVII2C1_IC_DMA_CR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_DMA_TDLR: u32 = 0x00a3;
pub const regCKSVII2C1_IC_DMA_TDLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_DMA_RDLR: u32 = 0x00a4;
pub const regCKSVII2C1_IC_DMA_RDLR_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_SDA_SETUP: u32 = 0x00a5;
pub const regCKSVII2C1_IC_SDA_SETUP_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_ACK_GENERAL_CALL: u32 = 0x00a6;
pub const regCKSVII2C1_IC_ACK_GENERAL_CALL_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_ENABLE_STATUS: u32 = 0x00a7;
pub const regCKSVII2C1_IC_ENABLE_STATUS_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_FS_SPKLEN: u32 = 0x00a8;
pub const regCKSVII2C1_IC_FS_SPKLEN_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_HS_SPKLEN: u32 = 0x00a9;
pub const regCKSVII2C1_IC_HS_SPKLEN_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_CLR_RESTART_DET: u32 = 0x00aa;
pub const regCKSVII2C1_IC_CLR_RESTART_DET_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_COMP_PARAM_1: u32 = 0x00ad;
pub const regCKSVII2C1_IC_COMP_PARAM_1_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_COMP_VERSION: u32 = 0x00ae;
pub const regCKSVII2C1_IC_COMP_VERSION_BASE_IDX: u32 = 0;
pub const regCKSVII2C1_IC_COMP_TYPE: u32 = 0x00af;
pub const regCKSVII2C1_IC_COMP_TYPE_BASE_IDX: u32 = 0;
pub const regSMUIO_PWRMGT: u32 = 0x018c;
pub const regSMUIO_PWRMGT_BASE_IDX: u32 = 0;


// addressBlock: smuio_smuio_rom_SmuSmuioDec
// base address: 0x5a380
pub const regROM_CNTL: u32 = 0x00e0;
pub const regROM_CNTL_BASE_IDX: u32 = 0;
pub const regPAGE_MIRROR_CNTL: u32 = 0x00e1;
pub const regPAGE_MIRROR_CNTL_BASE_IDX: u32 = 0;
pub const regROM_STATUS: u32 = 0x00e2;
pub const regROM_STATUS_BASE_IDX: u32 = 0;
pub const regCGTT_ROM_CLK_CTRL0: u32 = 0x00e3;
pub const regCGTT_ROM_CLK_CTRL0_BASE_IDX: u32 = 0;
pub const regROM_INDEX: u32 = 0x00e4;
pub const regROM_INDEX_BASE_IDX: u32 = 0;
pub const regROM_DATA: u32 = 0x00e5;
pub const regROM_DATA_BASE_IDX: u32 = 0;
pub const regROM_START: u32 = 0x00e6;
pub const regROM_START_BASE_IDX: u32 = 0;
pub const regROM_SW_CNTL: u32 = 0x00e8;
pub const regROM_SW_CNTL_BASE_IDX: u32 = 0;
pub const regROM_SW_STATUS: u32 = 0x00e9;
pub const regROM_SW_STATUS_BASE_IDX: u32 = 0;
pub const regROM_SW_COMMAND: u32 = 0x00ea;
pub const regROM_SW_COMMAND_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_1: u32 = 0x00ec;
pub const regROM_SW_DATA_1_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_2: u32 = 0x00ed;
pub const regROM_SW_DATA_2_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_3: u32 = 0x00ee;
pub const regROM_SW_DATA_3_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_4: u32 = 0x00ef;
pub const regROM_SW_DATA_4_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_5: u32 = 0x00f0;
pub const regROM_SW_DATA_5_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_6: u32 = 0x00f1;
pub const regROM_SW_DATA_6_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_7: u32 = 0x00f2;
pub const regROM_SW_DATA_7_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_8: u32 = 0x00f3;
pub const regROM_SW_DATA_8_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_9: u32 = 0x00f4;
pub const regROM_SW_DATA_9_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_10: u32 = 0x00f5;
pub const regROM_SW_DATA_10_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_11: u32 = 0x00f6;
pub const regROM_SW_DATA_11_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_12: u32 = 0x00f7;
pub const regROM_SW_DATA_12_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_13: u32 = 0x00f8;
pub const regROM_SW_DATA_13_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_14: u32 = 0x00f9;
pub const regROM_SW_DATA_14_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_15: u32 = 0x00fa;
pub const regROM_SW_DATA_15_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_16: u32 = 0x00fb;
pub const regROM_SW_DATA_16_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_17: u32 = 0x00fc;
pub const regROM_SW_DATA_17_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_18: u32 = 0x00fd;
pub const regROM_SW_DATA_18_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_19: u32 = 0x00fe;
pub const regROM_SW_DATA_19_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_20: u32 = 0x00ff;
pub const regROM_SW_DATA_20_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_21: u32 = 0x0100;
pub const regROM_SW_DATA_21_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_22: u32 = 0x0101;
pub const regROM_SW_DATA_22_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_23: u32 = 0x0102;
pub const regROM_SW_DATA_23_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_24: u32 = 0x0103;
pub const regROM_SW_DATA_24_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_25: u32 = 0x0104;
pub const regROM_SW_DATA_25_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_26: u32 = 0x0105;
pub const regROM_SW_DATA_26_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_27: u32 = 0x0106;
pub const regROM_SW_DATA_27_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_28: u32 = 0x0107;
pub const regROM_SW_DATA_28_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_29: u32 = 0x0108;
pub const regROM_SW_DATA_29_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_30: u32 = 0x0109;
pub const regROM_SW_DATA_30_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_31: u32 = 0x010a;
pub const regROM_SW_DATA_31_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_32: u32 = 0x010b;
pub const regROM_SW_DATA_32_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_33: u32 = 0x010c;
pub const regROM_SW_DATA_33_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_34: u32 = 0x010d;
pub const regROM_SW_DATA_34_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_35: u32 = 0x010e;
pub const regROM_SW_DATA_35_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_36: u32 = 0x010f;
pub const regROM_SW_DATA_36_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_37: u32 = 0x0110;
pub const regROM_SW_DATA_37_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_38: u32 = 0x0111;
pub const regROM_SW_DATA_38_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_39: u32 = 0x0112;
pub const regROM_SW_DATA_39_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_40: u32 = 0x0113;
pub const regROM_SW_DATA_40_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_41: u32 = 0x0114;
pub const regROM_SW_DATA_41_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_42: u32 = 0x0115;
pub const regROM_SW_DATA_42_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_43: u32 = 0x0116;
pub const regROM_SW_DATA_43_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_44: u32 = 0x0117;
pub const regROM_SW_DATA_44_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_45: u32 = 0x0118;
pub const regROM_SW_DATA_45_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_46: u32 = 0x0119;
pub const regROM_SW_DATA_46_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_47: u32 = 0x011a;
pub const regROM_SW_DATA_47_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_48: u32 = 0x011b;
pub const regROM_SW_DATA_48_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_49: u32 = 0x011c;
pub const regROM_SW_DATA_49_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_50: u32 = 0x011d;
pub const regROM_SW_DATA_50_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_51: u32 = 0x011e;
pub const regROM_SW_DATA_51_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_52: u32 = 0x011f;
pub const regROM_SW_DATA_52_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_53: u32 = 0x0120;
pub const regROM_SW_DATA_53_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_54: u32 = 0x0121;
pub const regROM_SW_DATA_54_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_55: u32 = 0x0122;
pub const regROM_SW_DATA_55_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_56: u32 = 0x0123;
pub const regROM_SW_DATA_56_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_57: u32 = 0x0124;
pub const regROM_SW_DATA_57_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_58: u32 = 0x0125;
pub const regROM_SW_DATA_58_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_59: u32 = 0x0126;
pub const regROM_SW_DATA_59_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_60: u32 = 0x0127;
pub const regROM_SW_DATA_60_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_61: u32 = 0x0128;
pub const regROM_SW_DATA_61_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_62: u32 = 0x0129;
pub const regROM_SW_DATA_62_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_63: u32 = 0x012a;
pub const regROM_SW_DATA_63_BASE_IDX: u32 = 0;
pub const regROM_SW_DATA_64: u32 = 0x012b;
pub const regROM_SW_DATA_64_BASE_IDX: u32 = 0;


// addressBlock: smuio_smuio_gpio_SmuSmuioDec
// base address: 0x5a500
pub const regSMU_GPIOPAD_SW_INT_STAT: u32 = 0x0140;
pub const regSMU_GPIOPAD_SW_INT_STAT_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_MASK: u32 = 0x0141;
pub const regSMU_GPIOPAD_MASK_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_A: u32 = 0x0142;
pub const regSMU_GPIOPAD_A_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_TXIMPSEL: u32 = 0x0143;
pub const regSMU_GPIOPAD_TXIMPSEL_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_EN: u32 = 0x0144;
pub const regSMU_GPIOPAD_EN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_Y: u32 = 0x0145;
pub const regSMU_GPIOPAD_Y_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_RXEN: u32 = 0x0146;
pub const regSMU_GPIOPAD_RXEN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_RCVR_SEL0: u32 = 0x0147;
pub const regSMU_GPIOPAD_RCVR_SEL0_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_RCVR_SEL1: u32 = 0x0148;
pub const regSMU_GPIOPAD_RCVR_SEL1_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_PU_EN: u32 = 0x0149;
pub const regSMU_GPIOPAD_PU_EN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_PD_EN: u32 = 0x014a;
pub const regSMU_GPIOPAD_PD_EN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_PINSTRAPS: u32 = 0x014b;
pub const regSMU_GPIOPAD_PINSTRAPS_BASE_IDX: u32 = 0;
pub const regDFT_PINSTRAPS: u32 = 0x014c;
pub const regDFT_PINSTRAPS_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_INT_STAT_EN: u32 = 0x014d;
pub const regSMU_GPIOPAD_INT_STAT_EN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_INT_STAT: u32 = 0x014e;
pub const regSMU_GPIOPAD_INT_STAT_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_INT_STAT_AK: u32 = 0x014f;
pub const regSMU_GPIOPAD_INT_STAT_AK_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_INT_EN: u32 = 0x0150;
pub const regSMU_GPIOPAD_INT_EN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_INT_TYPE: u32 = 0x0151;
pub const regSMU_GPIOPAD_INT_TYPE_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_INT_POLARITY: u32 = 0x0152;
pub const regSMU_GPIOPAD_INT_POLARITY_BASE_IDX: u32 = 0;
pub const regSMUIO_PCC_GPIO_SELECT: u32 = 0x0155;
pub const regSMUIO_PCC_GPIO_SELECT_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_S0: u32 = 0x0156;
pub const regSMU_GPIOPAD_S0_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_S1: u32 = 0x0157;
pub const regSMU_GPIOPAD_S1_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_SCHMEN: u32 = 0x0158;
pub const regSMU_GPIOPAD_SCHMEN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_SCL_EN: u32 = 0x0159;
pub const regSMU_GPIOPAD_SCL_EN_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_SDA_EN: u32 = 0x015a;
pub const regSMU_GPIOPAD_SDA_EN_BASE_IDX: u32 = 0;
pub const regSMUIO_GPIO_INT0_SELECT: u32 = 0x015b;
pub const regSMUIO_GPIO_INT0_SELECT_BASE_IDX: u32 = 0;
pub const regSMUIO_GPIO_INT1_SELECT: u32 = 0x015c;
pub const regSMUIO_GPIO_INT1_SELECT_BASE_IDX: u32 = 0;
pub const regSMUIO_GPIO_INT2_SELECT: u32 = 0x015d;
pub const regSMUIO_GPIO_INT2_SELECT_BASE_IDX: u32 = 0;
pub const regSMUIO_GPIO_INT3_SELECT: u32 = 0x015e;
pub const regSMUIO_GPIO_INT3_SELECT_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_MP_INT0_STAT: u32 = 0x015f;
pub const regSMU_GPIOPAD_MP_INT0_STAT_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_MP_INT1_STAT: u32 = 0x0160;
pub const regSMU_GPIOPAD_MP_INT1_STAT_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_MP_INT2_STAT: u32 = 0x0161;
pub const regSMU_GPIOPAD_MP_INT2_STAT_BASE_IDX: u32 = 0;
pub const regSMU_GPIOPAD_MP_INT3_STAT: u32 = 0x0162;
pub const regSMU_GPIOPAD_MP_INT3_STAT_BASE_IDX: u32 = 0;
pub const regSMIO_INDEX: u32 = 0x0163;
pub const regSMIO_INDEX_BASE_IDX: u32 = 0;
pub const regS0_VID_SMIO_CNTL: u32 = 0x0164;
pub const regS0_VID_SMIO_CNTL_BASE_IDX: u32 = 0;
pub const regS1_VID_SMIO_CNTL: u32 = 0x0165;
pub const regS1_VID_SMIO_CNTL_BASE_IDX: u32 = 0;
pub const regOPEN_DRAIN_SELECT: u32 = 0x0166;
pub const regOPEN_DRAIN_SELECT_BASE_IDX: u32 = 0;
pub const regSMIO_ENABLE: u32 = 0x0167;
pub const regSMIO_ENABLE_BASE_IDX: u32 = 0;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
