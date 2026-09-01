/* SPDX-License-Identifier: GPL-2.0
 *
 * MediaTek 8365 audio driver reg definition
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

// Translated from C preprocessor helpers provided by <linux/bitfield.h>.
pub const fn GENMASK(h: u32, l: u32) -> u32 {
    if h >= 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & (u32::MAX << l)
    }
}

pub const fn FIELD_PREP(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}




pub const AUDIO_TOP_CON0: u32 = (0x0000);
pub const AUDIO_TOP_CON1: u32 = (0x0004);
pub const AUDIO_TOP_CON2: u32 = (0x0008);
pub const AUDIO_TOP_CON3: u32 = (0x000c);

pub const AFE_DAC_CON0: u32 = (0x0010);
pub const AFE_DAC_CON1: u32 = (0x0014);
pub const AFE_I2S_CON: u32 = (0x0018);
pub const AFE_CONN0: u32 = (0x0020);
pub const AFE_CONN1: u32 = (0x0024);
pub const AFE_CONN2: u32 = (0x0028);
pub const AFE_CONN3: u32 = (0x002c);
pub const AFE_CONN4: u32 = (0x0030);
pub const AFE_I2S_CON1: u32 = (0x0034);
pub const AFE_I2S_CON2: u32 = (0x0038);
pub const AFE_MRGIF_CON: u32 = (0x003c);
pub const AFE_DL1_BASE: u32 = (0x0040);
pub const AFE_DL1_CUR: u32 = (0x0044);
pub const AFE_DL1_END: u32 = (0x0048);
pub const AFE_I2S_CON3: u32 = (0x004c);
pub const AFE_DL2_BASE: u32 = (0x0050);
pub const AFE_DL2_CUR: u32 = (0x0054);
pub const AFE_DL2_END: u32 = (0x0058);
pub const AFE_CONN5: u32 = (0x005c);
pub const AFE_AWB_BASE: u32 = (0x0070);
pub const AFE_AWB_END: u32 = (0x0078);
pub const AFE_AWB_CUR: u32 = (0x007c);
pub const AFE_VUL_BASE: u32 = (0x0080);
pub const AFE_VUL_END: u32 = (0x0088);
pub const AFE_VUL_CUR: u32 = (0x008c);
pub const AFE_CONN6: u32 = (0x00bc);
pub const AFE_MEMIF_MSB: u32 = (0x00cc);
pub const AFE_MEMIF_MON0: u32 = (0x00d0);
pub const AFE_MEMIF_MON1: u32 = (0x00d4);
pub const AFE_MEMIF_MON2: u32 = (0x00d8);
pub const AFE_MEMIF_MON3: u32 = (0x00dc);
pub const AFE_MEMIF_MON4: u32 = (0x00e0);
pub const AFE_MEMIF_MON5: u32 = (0x00e4);
pub const AFE_MEMIF_MON6: u32 = (0x00e8);
pub const AFE_MEMIF_MON7: u32 = (0x00ec);
pub const AFE_MEMIF_MON8: u32 = (0x00f0);
pub const AFE_MEMIF_MON9: u32 = (0x00f4);
pub const AFE_MEMIF_MON10: u32 = (0x00f8);
pub const AFE_MEMIF_MON11: u32 = (0x00fc);
pub const AFE_ADDA_DL_SRC2_CON0: u32 = (0x0108);
pub const AFE_ADDA_DL_SRC2_CON1: u32 = (0x010c);
pub const AFE_ADDA_UL_SRC_CON0: u32 = (0x0114);
pub const AFE_ADDA_UL_SRC_CON1: u32 = (0x0118);
pub const AFE_ADDA_TOP_CON0: u32 = (0x0120);
pub const AFE_ADDA_UL_DL_CON0: u32 = (0x0124);
pub const AFE_ADDA_SRC_DEBUG: u32 = (0x012c);
pub const AFE_ADDA_SRC_DEBUG_MON0: u32 = (0x0130);
pub const AFE_ADDA_SRC_DEBUG_MON1: u32 = (0x0134);
pub const AFE_ADDA_UL_SRC_MON0: u32 = (0x0148);
pub const AFE_ADDA_UL_SRC_MON1: u32 = (0x014c);
pub const AFE_SRAM_BOUND: u32 = (0x0170);
pub const AFE_SECURE_CON: u32 = (0x0174);
pub const AFE_SECURE_CONN0: u32 = (0x0178);
pub const AFE_SIDETONE_DEBUG: u32 = (0x01d0);
pub const AFE_SIDETONE_MON: u32 = (0x01d4);
pub const AFE_SIDETONE_CON0: u32 = (0x01e0);
pub const AFE_SIDETONE_COEFF: u32 = (0x01e4);
pub const AFE_SIDETONE_CON1: u32 = (0x01e8);
pub const AFE_SIDETONE_GAIN: u32 = (0x01ec);
pub const AFE_SGEN_CON0: u32 = (0x01f0);
pub const AFE_SINEGEN_CON_TDM: u32 = (0x01f8);
pub const AFE_SINEGEN_CON_TDM_IN: u32 = (0x01fc);
pub const AFE_TOP_CON0: u32 = (0x0200);
pub const AFE_BUS_CFG: u32 = (0x0240);
pub const AFE_BUS_MON0: u32 = (0x0244);
pub const AFE_ADDA_PREDIS_CON0: u32 = (0x0260);
pub const AFE_ADDA_PREDIS_CON1: u32 = (0x0264);
pub const AFE_CONN_MON0: u32 = (0x0280);
pub const AFE_CONN_MON1: u32 = (0x0284);
pub const AFE_CONN_MON2: u32 = (0x0288);
pub const AFE_CONN_MON3: u32 = (0x028c);
pub const AFE_ADDA_IIR_COEF_02_01: u32 = (0x0290);
pub const AFE_ADDA_IIR_COEF_04_03: u32 = (0x0294);
pub const AFE_ADDA_IIR_COEF_06_05: u32 = (0x0298);
pub const AFE_ADDA_IIR_COEF_08_07: u32 = (0x029c);
pub const AFE_ADDA_IIR_COEF_10_09: u32 = (0x02a0);
pub const AFE_VUL_D2_BASE: u32 = (0x0350);
pub const AFE_VUL_D2_END: u32 = (0x0358);
pub const AFE_VUL_D2_CUR: u32 = (0x035c);
pub const AFE_HDMI_OUT_CON0: u32 = (0x0370);
pub const AFE_HDMI_OUT_BASE: u32 = (0x0374);
pub const AFE_HDMI_OUT_CUR: u32 = (0x0378);
pub const AFE_HDMI_OUT_END: u32 = (0x037c);
pub const AFE_SPDIF_OUT_CON0: u32 = (0x0380);
pub const AFE_SPDIF_OUT_BASE: u32 = (0x0384);
pub const AFE_SPDIF_OUT_CUR: u32 = (0x0388);
pub const AFE_SPDIF_OUT_END: u32 = (0x038c);
pub const AFE_HDMI_CONN0: u32 = (0x0390);
pub const AFE_HDMI_CONN1: u32 = (0x0398);
pub const AFE_CONN_TDMIN_CON: u32 = (0x039c);
pub const AFE_IRQ_MCU_CON: u32 = (0x03a0);
pub const AFE_IRQ_MCU_STATUS: u32 = (0x03a4);
pub const AFE_IRQ_MCU_CLR: u32 = (0x03a8);
pub const AFE_IRQ_MCU_CNT1: u32 = (0x03ac);
pub const AFE_IRQ_MCU_CNT2: u32 = (0x03b0);
pub const AFE_IRQ_MCU_EN: u32 = (0x03b4);
pub const AFE_IRQ_MCU_MON2: u32 = (0x03b8);
pub const AFE_IRQ_MCU_CNT5: u32 = (0x03bc);
pub const AFE_IRQ1_MCU_CNT_MON: u32 = (0x03c0);
pub const AFE_IRQ2_MCU_CNT_MON: u32 = (0x03c4);
pub const AFE_IRQ1_MCU_EN_CNT_MON: u32 = (0x03c8);
pub const AFE_IRQ5_MCU_CNT_MON: u32 = (0x03cc);
pub const AFE_MEMIF_MINLEN: u32 = (0x03d0);
pub const AFE_MEMIF_MAXLEN: u32 = (0x03d4);
pub const AFE_MEMIF_PBUF_SIZE: u32 = (0x03d8);
pub const AFE_IRQ_MCU_CNT7: u32 = (0x03dc);
pub const AFE_IRQ7_MCU_CNT_MON: u32 = (0x03e0);
pub const AFE_MEMIF_PBUF2_SIZE: u32 = (0x03ec);
pub const AFE_APLL_TUNER_CFG: u32 = (0x03f0);
pub const AFE_APLL_TUNER_CFG1: u32 = (0x03f4);
pub const AFE_IRQ_MCU_CON2: u32 = (0x03f8);
pub const IRQ13_MCU_CNT: u32 = (0x0408);
pub const IRQ13_MCU_CNT_MON: u32 = (0x040c);
pub const AFE_GAIN1_CON0: u32 = (0x0410);
pub const AFE_GAIN1_CON1: u32 = (0x0414);
pub const AFE_GAIN1_CON2: u32 = (0x0418);
pub const AFE_GAIN1_CON3: u32 = (0x041c);
pub const AFE_GAIN2_CON0: u32 = (0x0428);
pub const AFE_GAIN2_CON1: u32 = (0x042c);
pub const AFE_GAIN2_CON2: u32 = (0x0430);
pub const AFE_GAIN2_CON3: u32 = (0x0434);
pub const AFE_GAIN2_CUR: u32 = (0x043c);
pub const AFE_CONN11: u32 = (0x0448);
pub const AFE_CONN12: u32 = (0x044c);
pub const AFE_CONN13: u32 = (0x0450);
pub const AFE_CONN14: u32 = (0x0454);
pub const AFE_CONN15: u32 = (0x0458);
pub const AFE_CONN16: u32 = (0x045c);
pub const AFE_CONN7: u32 = (0x0460);
pub const AFE_CONN8: u32 = (0x0464);
pub const AFE_CONN9: u32 = (0x0468);
pub const AFE_CONN10: u32 = (0x046c);
pub const AFE_CONN21: u32 = (0x0470);
pub const AFE_CONN22: u32 = (0x0474);
pub const AFE_CONN23: u32 = (0x0478);
pub const AFE_CONN24: u32 = (0x047c);
pub const AFE_IEC_CFG: u32 = (0x0480);
pub const AFE_IEC_NSNUM: u32 = (0x0484);
pub const AFE_IEC_BURST_INFO: u32 = (0x0488);
pub const AFE_IEC_BURST_LEN: u32 = (0x048c);
pub const AFE_IEC_NSADR: u32 = (0x0490);
pub const AFE_CONN_RS: u32 = (0x0494);
pub const AFE_CONN_DI: u32 = (0x0498);
pub const AFE_IEC_CHL_STAT0: u32 = (0x04a0);
pub const AFE_IEC_CHL_STAT1: u32 = (0x04a4);
pub const AFE_IEC_CHR_STAT0: u32 = (0x04a8);
pub const AFE_IEC_CHR_STAT1: u32 = (0x04ac);
pub const AFE_CONN25: u32 = (0x04b0);
pub const AFE_CONN26: u32 = (0x04b4);
pub const FPGA_CFG2: u32 = (0x04b8);
pub const FPGA_CFG3: u32 = (0x04bc);
pub const FPGA_CFG0: u32 = (0x04c0);
pub const FPGA_CFG1: u32 = (0x04c4);
pub const AFE_SRAM_DELSEL_CON0: u32 = (0x04f0);
pub const AFE_SRAM_DELSEL_CON1: u32 = (0x04f4);
pub const AFE_SRAM_DELSEL_CON2: u32 = (0x04f8);
pub const FPGA_CFG4: u32 = (0x04fc);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON0: u32 = (0x0500);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON1: u32 = (0x0504);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON2: u32 = (0x0508);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON3: u32 = (0x050c);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON4: u32 = (0x0510);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON5: u32 = (0x0514);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON6: u32 = (0x0518);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON7: u32 = (0x051c);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON8: u32 = (0x0520);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON9: u32 = (0x0524);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON10: u32 = (0x0528);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON12: u32 = (0x0530);
pub const AFE_TDM_GASRC4_ASRC_2CH_CON13: u32 = (0x0534);
pub const PCM_INTF_CON2: u32 = (0x0538);
pub const PCM2_INTF_CON: u32 = (0x053c);
pub const AFE_APB_MON: u32 = (0x0540);
pub const AFE_CONN34: u32 = (0x0544);
pub const AFE_TDM_CON1: u32 = (0x0548);
pub const AFE_TDM_CON2: u32 = (0x054c);
pub const PCM_INTF_CON1: u32 = (0x0550);
pub const AFE_SECURE_MASK_CONN47_1: u32 = (0x0554);
pub const AFE_SECURE_MASK_CONN48_1: u32 = (0x0558);
pub const AFE_SECURE_MASK_CONN49_1: u32 = (0x055c);
pub const AFE_SECURE_MASK_CONN50_1: u32 = (0x0560);
pub const AFE_SECURE_MASK_CONN51_1: u32 = (0x0564);
pub const AFE_SECURE_MASK_CONN52_1: u32 = (0x0568);
pub const AFE_SECURE_MASK_CONN53_1: u32 = (0x056c);
pub const AFE_SE_SECURE_CON: u32 = (0x0570);
pub const AFE_TDM_IN_CON1: u32 = (0x0588);
pub const AFE_TDM_IN_CON2: u32 = (0x058c);
pub const AFE_TDM_IN_MON1: u32 = (0x0590);
pub const AFE_TDM_IN_MON2: u32 = (0x0594);
pub const AFE_TDM_IN_MON3: u32 = (0x0598);
pub const AFE_DMIC0_UL_SRC_CON0: u32 = (0x05b4);
pub const AFE_DMIC0_UL_SRC_CON1: u32 = (0x05b8);
pub const AFE_DMIC0_SRC_DEBUG: u32 = (0x05bc);
pub const AFE_DMIC0_SRC_DEBUG_MON0: u32 = (0x05c0);
pub const AFE_DMIC0_UL_SRC_MON0: u32 = (0x05c8);
pub const AFE_DMIC0_UL_SRC_MON1: u32 = (0x05cc);
pub const AFE_DMIC0_IIR_COEF_02_01: u32 = (0x05d0);
pub const AFE_DMIC0_IIR_COEF_04_03: u32 = (0x05d4);
pub const AFE_DMIC0_IIR_COEF_06_05: u32 = (0x05d8);
pub const AFE_DMIC0_IIR_COEF_08_07: u32 = (0x05dc);
pub const AFE_DMIC0_IIR_COEF_10_09: u32 = (0x05e0);
pub const AFE_DMIC1_UL_SRC_CON0: u32 = (0x0620);
pub const AFE_DMIC1_UL_SRC_CON1: u32 = (0x0624);
pub const AFE_DMIC1_SRC_DEBUG: u32 = (0x0628);
pub const AFE_DMIC1_SRC_DEBUG_MON0: u32 = (0x062c);
pub const AFE_DMIC1_UL_SRC_MON0: u32 = (0x0634);
pub const AFE_DMIC1_UL_SRC_MON1: u32 = (0x0638);
pub const AFE_DMIC1_IIR_COEF_02_01: u32 = (0x063c);
pub const AFE_DMIC1_IIR_COEF_04_03: u32 = (0x0640);
pub const AFE_DMIC1_IIR_COEF_06_05: u32 = (0x0644);
pub const AFE_DMIC1_IIR_COEF_08_07: u32 = (0x0648);
pub const AFE_DMIC1_IIR_COEF_10_09: u32 = (0x064c);
pub const AFE_SECURE_MASK_CONN39_1: u32 = (0x068c);
pub const AFE_SECURE_MASK_CONN40_1: u32 = (0x0690);
pub const AFE_SECURE_MASK_CONN41_1: u32 = (0x0694);
pub const AFE_SECURE_MASK_CONN42_1: u32 = (0x0698);
pub const AFE_SECURE_MASK_CONN43_1: u32 = (0x069c);
pub const AFE_SECURE_MASK_CONN44_1: u32 = (0x06a0);
pub const AFE_SECURE_MASK_CONN45_1: u32 = (0x06a4);
pub const AFE_SECURE_MASK_CONN46_1: u32 = (0x06a8);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON0: u32 = (0x06c0);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON1: u32 = (0x06c4);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON2: u32 = (0x06c8);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON3: u32 = (0x06cc);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON4: u32 = (0x06d0);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON5: u32 = (0x06d4);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON6: u32 = (0x06d8);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON7: u32 = (0x06dc);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON8: u32 = (0x06e0);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON9: u32 = (0x06e4);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON10: u32 = (0x06e8);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON12: u32 = (0x06f0);
pub const AFE_TDM_GASRC1_ASRC_2CH_CON13: u32 = (0x06f4);
pub const AFE_TDM_ASRC_CON0: u32 = (0x06f8);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON0: u32 = (0x0700);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON1: u32 = (0x0704);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON2: u32 = (0x0708);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON3: u32 = (0x070c);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON4: u32 = (0x0710);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON5: u32 = (0x0714);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON6: u32 = (0x0718);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON7: u32 = (0x071c);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON8: u32 = (0x0720);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON9: u32 = (0x0724);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON10: u32 = (0x0728);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON12: u32 = (0x0730);
pub const AFE_TDM_GASRC2_ASRC_2CH_CON13: u32 = (0x0734);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON0: u32 = (0x0740);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON1: u32 = (0x0744);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON2: u32 = (0x0748);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON3: u32 = (0x074c);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON4: u32 = (0x0750);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON5: u32 = (0x0754);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON6: u32 = (0x0758);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON7: u32 = (0x075c);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON8: u32 = (0x0760);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON9: u32 = (0x0764);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON10: u32 = (0x0768);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON12: u32 = (0x0770);
pub const AFE_TDM_GASRC3_ASRC_2CH_CON13: u32 = (0x0774);
pub const AFE_DMIC2_UL_SRC_CON0: u32 = (0x0780);
pub const AFE_DMIC2_UL_SRC_CON1: u32 = (0x0784);
pub const AFE_DMIC2_SRC_DEBUG: u32 = (0x0788);
pub const AFE_DMIC2_SRC_DEBUG_MON0: u32 = (0x078c);
pub const AFE_DMIC2_UL_SRC_MON0: u32 = (0x0794);
pub const AFE_DMIC2_UL_SRC_MON1: u32 = (0x0798);
pub const AFE_DMIC2_IIR_COEF_02_01: u32 = (0x079c);
pub const AFE_DMIC2_IIR_COEF_04_03: u32 = (0x07a0);
pub const AFE_DMIC2_IIR_COEF_06_05: u32 = (0x07a4);
pub const AFE_DMIC2_IIR_COEF_08_07: u32 = (0x07a8);
pub const AFE_DMIC2_IIR_COEF_10_09: u32 = (0x07ac);
pub const AFE_DMIC3_UL_SRC_CON0: u32 = (0x07ec);
pub const AFE_DMIC3_UL_SRC_CON1: u32 = (0x07f0);
pub const AFE_DMIC3_SRC_DEBUG: u32 = (0x07f4);
pub const AFE_DMIC3_SRC_DEBUG_MON0: u32 = (0x07f8);
pub const AFE_DMIC3_UL_SRC_MON0: u32 = (0x0800);
pub const AFE_DMIC3_UL_SRC_MON1: u32 = (0x0804);
pub const AFE_DMIC3_IIR_COEF_02_01: u32 = (0x0808);
pub const AFE_DMIC3_IIR_COEF_04_03: u32 = (0x080c);
pub const AFE_DMIC3_IIR_COEF_06_05: u32 = (0x0810);
pub const AFE_DMIC3_IIR_COEF_08_07: u32 = (0x0814);
pub const AFE_DMIC3_IIR_COEF_10_09: u32 = (0x0818);
pub const AFE_SECURE_MASK_CONN25_1: u32 = (0x0858);
pub const AFE_SECURE_MASK_CONN26_1: u32 = (0x085c);
pub const AFE_SECURE_MASK_CONN27_1: u32 = (0x0860);
pub const AFE_SECURE_MASK_CONN28_1: u32 = (0x0864);
pub const AFE_SECURE_MASK_CONN29_1: u32 = (0x0868);
pub const AFE_SECURE_MASK_CONN30_1: u32 = (0x086c);
pub const AFE_SECURE_MASK_CONN31_1: u32 = (0x0870);
pub const AFE_SECURE_MASK_CONN32_1: u32 = (0x0874);
pub const AFE_SECURE_MASK_CONN33_1: u32 = (0x0878);
pub const AFE_SECURE_MASK_CONN34_1: u32 = (0x087c);
pub const AFE_SECURE_MASK_CONN35_1: u32 = (0x0880);
pub const AFE_SECURE_MASK_CONN36_1: u32 = (0x0884);
pub const AFE_SECURE_MASK_CONN37_1: u32 = (0x0888);
pub const AFE_SECURE_MASK_CONN38_1: u32 = (0x088c);
pub const AFE_IRQ_MCU_SCP_EN: u32 = (0x0890);
pub const AFE_IRQ_MCU_DSP_EN: u32 = (0x0894);
pub const AFE_IRQ3_MCU_CNT_MON: u32 = (0x0898);
pub const AFE_IRQ4_MCU_CNT_MON: u32 = (0x089c);
pub const AFE_IRQ8_MCU_CNT_MON: u32 = (0x08a0);
pub const AFE_IRQ_MCU_CNT3: u32 = (0x08a4);
pub const AFE_IRQ_MCU_CNT4: u32 = (0x08a8);
pub const AFE_IRQ_MCU_CNT8: u32 = (0x08ac);
pub const AFE_IRQ_MCU_CNT11: u32 = (0x08b0);
pub const AFE_IRQ_MCU_CNT12: u32 = (0x08b4);
pub const AFE_IRQ11_MCU_CNT_MON: u32 = (0x08b8);
pub const AFE_IRQ12_MCU_CNT_MON: u32 = (0x08bc);
pub const AFE_VUL3_BASE: u32 = (0x08c0);
pub const AFE_VUL3_CUR: u32 = (0x08c4);
pub const AFE_VUL3_END: u32 = (0x08c8);
pub const AFE_VUL3_BASE_MSB: u32 = (0x08d0);
pub const AFE_VUL3_END_MSB: u32 = (0x08d4);
pub const AFE_IRQ10_MCU_CNT_MON: u32 = (0x08d8);
pub const AFE_IRQ_MCU_CNT10: u32 = (0x08dc);
pub const AFE_IRQ_ACC1_CNT: u32 = (0x08e0);
pub const AFE_IRQ_ACC2_CNT: u32 = (0x08e4);
pub const AFE_IRQ_ACC1_CNT_MON1: u32 = (0x08e8);
pub const AFE_IRQ_ACC2_CNT_MON: u32 = (0x08ec);
pub const AFE_TSF_CON: u32 = (0x08f0);
pub const AFE_TSF_MON: u32 = (0x08f4);
pub const AFE_IRQ_ACC1_CNT_MON2: u32 = (0x08f8);
pub const AFE_SPDIFIN_CFG0: u32 = (0x0900);
pub const AFE_SPDIFIN_CFG1: u32 = (0x0904);
pub const AFE_SPDIFIN_CHSTS1: u32 = (0x0908);
pub const AFE_SPDIFIN_CHSTS2: u32 = (0x090c);
pub const AFE_SPDIFIN_CHSTS3: u32 = (0x0910);
pub const AFE_SPDIFIN_CHSTS4: u32 = (0x0914);
pub const AFE_SPDIFIN_CHSTS5: u32 = (0x0918);
pub const AFE_SPDIFIN_CHSTS6: u32 = (0x091c);
pub const AFE_SPDIFIN_DEBUG1: u32 = (0x0920);
pub const AFE_SPDIFIN_DEBUG2: u32 = (0x0924);
pub const AFE_SPDIFIN_DEBUG3: u32 = (0x0928);
pub const AFE_SPDIFIN_DEBUG4: u32 = (0x092c);
pub const AFE_SPDIFIN_EC: u32 = (0x0930);
pub const AFE_SPDIFIN_CKLOCK_CFG: u32 = (0x0934);
pub const AFE_SPDIFIN_BR: u32 = (0x093c);
pub const AFE_SPDIFIN_BR_DBG1: u32 = (0x0940);
pub const AFE_SPDIFIN_INT_EXT: u32 = (0x0948);
pub const AFE_SPDIFIN_INT_EXT2: u32 = (0x094c);
pub const SPDIFIN_FREQ_INFO: u32 = (0x0950);
pub const SPDIFIN_FREQ_INFO_2: u32 = (0x0954);
pub const SPDIFIN_FREQ_INFO_3: u32 = (0x0958);
pub const SPDIFIN_FREQ_STATUS: u32 = (0x095c);
pub const SPDIFIN_USERCODE1: u32 = (0x0960);
pub const SPDIFIN_USERCODE2: u32 = (0x0964);
pub const SPDIFIN_USERCODE3: u32 = (0x0968);
pub const SPDIFIN_USERCODE4: u32 = (0x096c);
pub const SPDIFIN_USERCODE5: u32 = (0x0970);
pub const SPDIFIN_USERCODE6: u32 = (0x0974);
pub const SPDIFIN_USERCODE7: u32 = (0x0978);
pub const SPDIFIN_USERCODE8: u32 = (0x097c);
pub const SPDIFIN_USERCODE9: u32 = (0x0980);
pub const SPDIFIN_USERCODE10: u32 = (0x0984);
pub const SPDIFIN_USERCODE11: u32 = (0x0988);
pub const SPDIFIN_USERCODE12: u32 = (0x098c);
pub const SPDIFIN_MEMIF_CON0: u32 = (0x0990);
pub const SPDIFIN_BASE_ADR: u32 = (0x0994);
pub const SPDIFIN_END_ADR: u32 = (0x0998);
pub const SPDIFIN_APLL_TUNER_CFG: u32 = (0x09a0);
pub const SPDIFIN_APLL_TUNER_CFG1: u32 = (0x09a4);
pub const SPDIFIN_APLL2_TUNER_CFG: u32 = (0x09a8);
pub const SPDIFIN_APLL2_TUNER_CFG1: u32 = (0x09ac);
pub const SPDIFIN_TYPE_DET: u32 = (0x09b0);
pub const MPHONE_MULTI_CON0: u32 = (0x09b4);
pub const SPDIFIN_CUR_ADR: u32 = (0x09b8);
pub const AFE_SINEGEN_CON_SPDIFIN: u32 = (0x09bc);
pub const AFE_HDMI_IN_2CH_CON0: u32 = (0x09c0);
pub const AFE_HDMI_IN_2CH_BASE: u32 = (0x09c4);
pub const AFE_HDMI_IN_2CH_END: u32 = (0x09c8);
pub const AFE_HDMI_IN_2CH_CUR: u32 = (0x09cc);
pub const AFE_MEMIF_BUF_MON0: u32 = (0x09d0);
pub const AFE_MEMIF_BUF_MON1: u32 = (0x09d4);
pub const AFE_MEMIF_BUF_MON2: u32 = (0x09d8);
pub const AFE_MEMIF_BUF_MON3: u32 = (0x09dc);
pub const AFE_MEMIF_BUF_MON6: u32 = (0x09e8);
pub const AFE_MEMIF_BUF_MON7: u32 = (0x09ec);
pub const AFE_MEMIF_BUF_MON8: u32 = (0x09f0);
pub const AFE_MEMIF_BUF_MON10: u32 = (0x09f8);
pub const AFE_MEMIF_BUF_MON11: u32 = (0x09fc);
pub const SYSTOP_STC_CONFIG: u32 = (0x0a00);
pub const AUDIO_STC_STATUS: u32 = (0x0a04);
pub const SYSTOP_W_STC_H: u32 = (0x0a08);
pub const SYSTOP_W_STC_L: u32 = (0x0a0c);
pub const SYSTOP_R_STC_H: u32 = (0x0a10);
pub const SYSTOP_R_STC_L: u32 = (0x0a14);
pub const AUDIO_W_STC_H: u32 = (0x0a18);
pub const AUDIO_W_STC_L: u32 = (0x0a1c);
pub const AUDIO_R_STC_H: u32 = (0x0a20);
pub const AUDIO_R_STC_L: u32 = (0x0a24);
pub const SYSTOP_W_STC2_H: u32 = (0x0a28);
pub const SYSTOP_W_STC2_L: u32 = (0x0a2c);
pub const SYSTOP_R_STC2_H: u32 = (0x0a30);
pub const SYSTOP_R_STC2_L: u32 = (0x0a34);
pub const AUDIO_W_STC2_H: u32 = (0x0a38);
pub const AUDIO_W_STC2_L: u32 = (0x0a3c);
pub const AUDIO_R_STC2_H: u32 = (0x0a40);
pub const AUDIO_R_STC2_L: u32 = (0x0a44);

pub const AFE_CONN17: u32 = (0x0a48);
pub const AFE_CONN18: u32 = (0x0a4c);
pub const AFE_CONN19: u32 = (0x0a50);
pub const AFE_CONN20: u32 = (0x0a54);
pub const AFE_CONN27: u32 = (0x0a58);
pub const AFE_CONN28: u32 = (0x0a5c);
pub const AFE_CONN29: u32 = (0x0a60);
pub const AFE_CONN30: u32 = (0x0a64);
pub const AFE_CONN31: u32 = (0x0a68);
pub const AFE_CONN32: u32 = (0x0a6c);
pub const AFE_CONN33: u32 = (0x0a70);
pub const AFE_CONN35: u32 = (0x0a74);
pub const AFE_CONN36: u32 = (0x0a78);
pub const AFE_CONN37: u32 = (0x0a7c);
pub const AFE_CONN38: u32 = (0x0a80);
pub const AFE_CONN39: u32 = (0x0a84);
pub const AFE_CONN40: u32 = (0x0a88);
pub const AFE_CONN41: u32 = (0x0a8c);
pub const AFE_CONN42: u32 = (0x0a90);
pub const AFE_CONN44: u32 = (0x0a94);
pub const AFE_CONN45: u32 = (0x0a98);
pub const AFE_CONN46: u32 = (0x0a9c);
pub const AFE_CONN47: u32 = (0x0aa0);
pub const AFE_CONN_24BIT: u32 = (0x0aa4);
pub const AFE_CONN0_1: u32 = (0x0aa8);
pub const AFE_CONN1_1: u32 = (0x0aac);
pub const AFE_CONN2_1: u32 = (0x0ab0);
pub const AFE_CONN3_1: u32 = (0x0ab4);
pub const AFE_CONN4_1: u32 = (0x0ab8);
pub const AFE_CONN5_1: u32 = (0x0abc);
pub const AFE_CONN6_1: u32 = (0x0ac0);
pub const AFE_CONN7_1: u32 = (0x0ac4);
pub const AFE_CONN8_1: u32 = (0x0ac8);
pub const AFE_CONN9_1: u32 = (0x0acc);
pub const AFE_CONN10_1: u32 = (0x0ad0);
pub const AFE_CONN11_1: u32 = (0x0ad4);
pub const AFE_CONN12_1: u32 = (0x0ad8);
pub const AFE_CONN13_1: u32 = (0x0adc);
pub const AFE_CONN14_1: u32 = (0x0ae0);
pub const AFE_CONN15_1: u32 = (0x0ae4);
pub const AFE_CONN16_1: u32 = (0x0ae8);
pub const AFE_CONN17_1: u32 = (0x0aec);
pub const AFE_CONN18_1: u32 = (0x0af0);
pub const AFE_CONN19_1: u32 = (0x0af4);
pub const AFE_CONN43: u32 = (0x0af8);
pub const AFE_CONN43_1: u32 = (0x0afc);
pub const AFE_CONN21_1: u32 = (0x0b00);
pub const AFE_CONN22_1: u32 = (0x0b04);
pub const AFE_CONN23_1: u32 = (0x0b08);
pub const AFE_CONN24_1: u32 = (0x0b0c);
pub const AFE_CONN25_1: u32 = (0x0b10);
pub const AFE_CONN26_1: u32 = (0x0b14);
pub const AFE_CONN27_1: u32 = (0x0b18);
pub const AFE_CONN28_1: u32 = (0x0b1c);
pub const AFE_CONN29_1: u32 = (0x0b20);
pub const AFE_CONN30_1: u32 = (0x0b24);
pub const AFE_CONN31_1: u32 = (0x0b28);
pub const AFE_CONN32_1: u32 = (0x0b2c);
pub const AFE_CONN33_1: u32 = (0x0b30);
pub const AFE_CONN34_1: u32 = (0x0b34);
pub const AFE_CONN35_1: u32 = (0x0b38);
pub const AFE_CONN36_1: u32 = (0x0b3c);
pub const AFE_CONN37_1: u32 = (0x0b40);
pub const AFE_CONN38_1: u32 = (0x0b44);
pub const AFE_CONN39_1: u32 = (0x0b48);
pub const AFE_CONN40_1: u32 = (0x0b4c);
pub const AFE_CONN41_1: u32 = (0x0b50);
pub const AFE_CONN42_1: u32 = (0x0b54);
pub const AFE_CONN44_1: u32 = (0x0b58);
pub const AFE_CONN45_1: u32 = (0x0b5c);
pub const AFE_CONN46_1: u32 = (0x0b60);
pub const AFE_CONN47_1: u32 = (0x0b64);
pub const AFE_CONN_RS_1: u32 = (0x0b68);
pub const AFE_CONN_DI_1: u32 = (0x0b6c);
pub const AFE_CONN_24BIT_1: u32 = (0x0b70);
pub const AFE_GAIN1_CUR: u32 = (0x0b78);
pub const AFE_CONN20_1: u32 = (0x0b7c);
pub const AFE_DL1_BASE_MSB: u32 = (0x0b80);
pub const AFE_DL1_END_MSB: u32 = (0x0b84);
pub const AFE_DL2_BASE_MSB: u32 = (0x0b88);
pub const AFE_DL2_END_MSB: u32 = (0x0b8c);
pub const AFE_AWB_BASE_MSB: u32 = (0x0b90);
pub const AFE_AWB_END_MSB: u32 = (0x0b94);
pub const AFE_VUL_BASE_MSB: u32 = (0x0ba0);
pub const AFE_VUL_END_MSB: u32 = (0x0ba4);
pub const AFE_VUL_D2_BASE_MSB: u32 = (0x0ba8);
pub const AFE_VUL_D2_END_MSB: u32 = (0x0bac);
pub const AFE_HDMI_OUT_BASE_MSB: u32 = (0x0bb8);
pub const AFE_HDMI_OUT_END_MSB: u32 = (0x0bbc);
pub const AFE_HDMI_IN_2CH_BASE_MSB: u32 = (0x0bc0);
pub const AFE_HDMI_IN_2CH_END_MSB: u32 = (0x0bc4);
pub const AFE_SPDIF_OUT_BASE_MSB: u32 = (0x0bc8);
pub const AFE_SPDIF_OUT_END_MSB: u32 = (0x0bcc);
pub const SPDIFIN_BASE_MSB: u32 = (0x0bd0);
pub const SPDIFIN_END_MSB: u32 = (0x0bd4);
pub const AFE_DL1_CUR_MSB: u32 = (0x0bd8);
pub const AFE_DL2_CUR_MSB: u32 = (0x0bdc);
pub const AFE_AWB_CUR_MSB: u32 = (0x0be8);
pub const AFE_VUL_CUR_MSB: u32 = (0x0bf8);
pub const AFE_VUL_D2_CUR_MSB: u32 = (0x0c04);
pub const AFE_HDMI_OUT_CUR_MSB: u32 = (0x0c0c);
pub const AFE_HDMI_IN_2CH_CUR_MSB: u32 = (0x0c10);
pub const AFE_SPDIF_OUT_CUR_MSB: u32 = (0x0c14);
pub const SPDIFIN_CUR_MSB: u32 = (0x0c18);
pub const AFE_CONN_REG: u32 = (0x0c20);
pub const AFE_SECURE_MASK_CONN14_1: u32 = (0x0c24);
pub const AFE_SECURE_MASK_CONN15_1: u32 = (0x0c28);
pub const AFE_SECURE_MASK_CONN16_1: u32 = (0x0c2c);
pub const AFE_SECURE_MASK_CONN17_1: u32 = (0x0c30);
pub const AFE_SECURE_MASK_CONN18_1: u32 = (0x0c34);
pub const AFE_SECURE_MASK_CONN19_1: u32 = (0x0c38);
pub const AFE_SECURE_MASK_CONN20_1: u32 = (0x0c3c);
pub const AFE_SECURE_MASK_CONN21_1: u32 = (0x0c40);
pub const AFE_SECURE_MASK_CONN22_1: u32 = (0x0c44);
pub const AFE_SECURE_MASK_CONN23_1: u32 = (0x0c48);
pub const AFE_SECURE_MASK_CONN24_1: u32 = (0x0c4c);
pub const AFE_ADDA_DL_SDM_DCCOMP_CON: u32 = (0x0c50);
pub const AFE_ADDA_DL_SDM_TEST: u32 = (0x0c54);
pub const AFE_ADDA_DL_DC_COMP_CFG0: u32 = (0x0c58);
pub const AFE_ADDA_DL_DC_COMP_CFG1: u32 = (0x0c5c);
pub const AFE_ADDA_DL_SDM_FIFO_MON: u32 = (0x0c60);
pub const AFE_ADDA_DL_SRC_LCH_MON: u32 = (0x0c64);
pub const AFE_ADDA_DL_SRC_RCH_MON: u32 = (0x0c68);
pub const AFE_ADDA_DL_SDM_OUT_MON: u32 = (0x0c6c);
pub const AFE_ADDA_DL_SDM_DITHER_CON: u32 = (0x0c70);

pub const AFE_VUL3_CUR_MSB: u32 = (0x0c78);
pub const AFE_ASRC_2CH_CON0: u32 = (0x0c80);
pub const AFE_ASRC_2CH_CON1: u32 = (0x0c84);
pub const AFE_ASRC_2CH_CON2: u32 = (0x0c88);
pub const AFE_ASRC_2CH_CON3: u32 = (0x0c8c);
pub const AFE_ASRC_2CH_CON4: u32 = (0x0c90);
pub const AFE_ASRC_2CH_CON5: u32 = (0x0c94);
pub const AFE_ASRC_2CH_CON6: u32 = (0x0c98);
pub const AFE_ASRC_2CH_CON7: u32 = (0x0c9c);
pub const AFE_ASRC_2CH_CON8: u32 = (0x0ca0);
pub const AFE_ASRC_2CH_CON9: u32 = (0x0ca4);
pub const AFE_ASRC_2CH_CON10: u32 = (0x0ca8);
pub const AFE_ASRC_2CH_CON12: u32 = (0x0cb0);
pub const AFE_ASRC_2CH_CON13: u32 = (0x0cb4);

pub const AFE_PCM_TX_ASRC_2CH_CON0: u32 = (0x0cc0);
pub const AFE_PCM_TX_ASRC_2CH_CON1: u32 = (0x0cc4);
pub const AFE_PCM_TX_ASRC_2CH_CON2: u32 = (0x0cc8);
pub const AFE_PCM_TX_ASRC_2CH_CON3: u32 = (0x0ccc);
pub const AFE_PCM_TX_ASRC_2CH_CON4: u32 = (0x0cd0);
pub const AFE_PCM_TX_ASRC_2CH_CON5: u32 = (0x0cd4);
pub const AFE_PCM_TX_ASRC_2CH_CON6: u32 = (0x0cd8);
pub const AFE_PCM_TX_ASRC_2CH_CON7: u32 = (0x0cdc);
pub const AFE_PCM_TX_ASRC_2CH_CON8: u32 = (0x0ce0);
pub const AFE_PCM_TX_ASRC_2CH_CON9: u32 = (0x0ce4);
pub const AFE_PCM_TX_ASRC_2CH_CON10: u32 = (0x0ce8);
pub const AFE_PCM_TX_ASRC_2CH_CON12: u32 = (0x0cf0);
pub const AFE_PCM_TX_ASRC_2CH_CON13: u32 = (0x0cf4);
pub const AFE_PCM_RX_ASRC_2CH_CON0: u32 = (0x0d00);
pub const AFE_PCM_RX_ASRC_2CH_CON1: u32 = (0x0d04);
pub const AFE_PCM_RX_ASRC_2CH_CON2: u32 = (0x0d08);
pub const AFE_PCM_RX_ASRC_2CH_CON3: u32 = (0x0d0c);
pub const AFE_PCM_RX_ASRC_2CH_CON4: u32 = (0x0d10);
pub const AFE_PCM_RX_ASRC_2CH_CON5: u32 = (0x0d14);
pub const AFE_PCM_RX_ASRC_2CH_CON6: u32 = (0x0d18);
pub const AFE_PCM_RX_ASRC_2CH_CON7: u32 = (0x0d1c);
pub const AFE_PCM_RX_ASRC_2CH_CON8: u32 = (0x0d20);
pub const AFE_PCM_RX_ASRC_2CH_CON9: u32 = (0x0d24);
pub const AFE_PCM_RX_ASRC_2CH_CON10: u32 = (0x0d28);
pub const AFE_PCM_RX_ASRC_2CH_CON12: u32 = (0x0d30);
pub const AFE_PCM_RX_ASRC_2CH_CON13: u32 = (0x0d34);

pub const AFE_ADDA_PREDIS_CON2: u32 = (0x0d40);
pub const AFE_ADDA_PREDIS_CON3: u32 = (0x0d44);
pub const AFE_SECURE_MASK_CONN4_1: u32 = (0x0d48);
pub const AFE_SECURE_MASK_CONN5_1: u32 = (0x0d4c);
pub const AFE_SECURE_MASK_CONN6_1: u32 = (0x0d50);
pub const AFE_SECURE_MASK_CONN7_1: u32 = (0x0d54);
pub const AFE_SECURE_MASK_CONN8_1: u32 = (0x0d58);
pub const AFE_SECURE_MASK_CONN9_1: u32 = (0x0d5c);
pub const AFE_SECURE_MASK_CONN10_1: u32 = (0x0d60);
pub const AFE_SECURE_MASK_CONN11_1: u32 = (0x0d64);
pub const AFE_SECURE_MASK_CONN12_1: u32 = (0x0d68);
pub const AFE_SECURE_MASK_CONN13_1: u32 = (0x0d6c);
pub const AFE_MEMIF_MON12: u32 = (0x0d70);
pub const AFE_MEMIF_MON13: u32 = (0x0d74);
pub const AFE_MEMIF_MON14: u32 = (0x0d78);
pub const AFE_MEMIF_MON15: u32 = (0x0d7c);
pub const AFE_SECURE_MASK_CONN42: u32 = (0x0dbc);
pub const AFE_SECURE_MASK_CONN43: u32 = (0x0dc0);
pub const AFE_SECURE_MASK_CONN44: u32 = (0x0dc4);
pub const AFE_SECURE_MASK_CONN45: u32 = (0x0dc8);
pub const AFE_SECURE_MASK_CONN46: u32 = (0x0dcc);
pub const AFE_HD_ENGEN_ENABLE: u32 = (0x0dd0);
pub const AFE_SECURE_MASK_CONN47: u32 = (0x0dd4);
pub const AFE_SECURE_MASK_CONN48: u32 = (0x0dd8);
pub const AFE_SECURE_MASK_CONN49: u32 = (0x0ddc);
pub const AFE_SECURE_MASK_CONN50: u32 = (0x0de0);
pub const AFE_SECURE_MASK_CONN51: u32 = (0x0de4);
pub const AFE_SECURE_MASK_CONN52: u32 = (0x0de8);
pub const AFE_SECURE_MASK_CONN53: u32 = (0x0dec);
pub const AFE_SECURE_MASK_CONN0_1: u32 = (0x0df0);
pub const AFE_SECURE_MASK_CONN1_1: u32 = (0x0df4);
pub const AFE_SECURE_MASK_CONN2_1: u32 = (0x0df8);
pub const AFE_SECURE_MASK_CONN3_1: u32 = (0x0dfc);

pub const AFE_ADDA_MTKAIF_CFG0: u32 = (0x0e00);
pub const AFE_ADDA_MTKAIF_SYNCWORD_CFG: u32 = (0x0e14);
pub const AFE_ADDA_MTKAIF_RX_CFG0: u32 = (0x0e20);
pub const AFE_ADDA_MTKAIF_RX_CFG1: u32 = (0x0e24);
pub const AFE_ADDA_MTKAIF_RX_CFG2: u32 = (0x0e28);
pub const AFE_ADDA_MTKAIF_MON0: u32 = (0x0e34);
pub const AFE_ADDA_MTKAIF_MON1: u32 = (0x0e38);
pub const AFE_AUD_PAD_TOP: u32 = (0x0e40);

pub const AFE_CM1_CON4: u32 = (0x0e48);
pub const AFE_CM2_CON4: u32 = (0x0e4c);
pub const AFE_CM1_CON0: u32 = (0x0e50);
pub const AFE_CM1_CON1: u32 = (0x0e54);
pub const AFE_CM1_CON2: u32 = (0x0e58);
pub const AFE_CM1_CON3: u32 = (0x0e5c);
pub const AFE_CM2_CON0: u32 = (0x0e60);
pub const AFE_CM2_CON1: u32 = (0x0e64);
pub const AFE_CM2_CON2: u32 = (0x0e68);
pub const AFE_CM2_CON3: u32 = (0x0e6c);
pub const AFE_CM2_CONN0: u32 = (0x0e70);
pub const AFE_CM2_CONN1: u32 = (0x0e74);
pub const AFE_CM2_CONN2: u32 = (0x0e78);

pub const AFE_GENERAL1_ASRC_2CH_CON0: u32 = (0x0e80);
pub const AFE_GENERAL1_ASRC_2CH_CON1: u32 = (0x0e84);
pub const AFE_GENERAL1_ASRC_2CH_CON2: u32 = (0x0e88);
pub const AFE_GENERAL1_ASRC_2CH_CON3: u32 = (0x0e8c);
pub const AFE_GENERAL1_ASRC_2CH_CON4: u32 = (0x0e90);
pub const AFE_GENERAL1_ASRC_2CH_CON5: u32 = (0x0e94);
pub const AFE_GENERAL1_ASRC_2CH_CON6: u32 = (0x0e98);
pub const AFE_GENERAL1_ASRC_2CH_CON7: u32 = (0x0e9c);
pub const AFE_GENERAL1_ASRC_2CH_CON8: u32 = (0x0ea0);
pub const AFE_GENERAL1_ASRC_2CH_CON9: u32 = (0x0ea4);
pub const AFE_GENERAL1_ASRC_2CH_CON10: u32 = (0x0ea8);
pub const AFE_GENERAL1_ASRC_2CH_CON12: u32 = (0x0eb0);
pub const AFE_GENERAL1_ASRC_2CH_CON13: u32 = (0x0eb4);
pub const GENERAL_ASRC_MODE: u32 = (0x0eb8);
pub const GENERAL_ASRC_EN_ON: u32 = (0x0ebc);

pub const AFE_CONN48: u32 = (0x0ec0);
pub const AFE_CONN49: u32 = (0x0ec4);
pub const AFE_CONN50: u32 = (0x0ec8);
pub const AFE_CONN51: u32 = (0x0ecc);
pub const AFE_CONN52: u32 = (0x0ed0);
pub const AFE_CONN53: u32 = (0x0ed4);
pub const AFE_CONN48_1: u32 = (0x0ee0);
pub const AFE_CONN49_1: u32 = (0x0ee4);
pub const AFE_CONN50_1: u32 = (0x0ee8);
pub const AFE_CONN51_1: u32 = (0x0eec);
pub const AFE_CONN52_1: u32 = (0x0ef0);
pub const AFE_CONN53_1: u32 = (0x0ef4);

pub const AFE_GENERAL2_ASRC_2CH_CON0: u32 = (0x0f00);
pub const AFE_GENERAL2_ASRC_2CH_CON1: u32 = (0x0f04);
pub const AFE_GENERAL2_ASRC_2CH_CON2: u32 = (0x0f08);
pub const AFE_GENERAL2_ASRC_2CH_CON3: u32 = (0x0f0c);
pub const AFE_GENERAL2_ASRC_2CH_CON4: u32 = (0x0f10);
pub const AFE_GENERAL2_ASRC_2CH_CON5: u32 = (0x0f14);
pub const AFE_GENERAL2_ASRC_2CH_CON6: u32 = (0x0f18);
pub const AFE_GENERAL2_ASRC_2CH_CON7: u32 = (0x0f1c);
pub const AFE_GENERAL2_ASRC_2CH_CON8: u32 = (0x0f20);
pub const AFE_GENERAL2_ASRC_2CH_CON9: u32 = (0x0f24);
pub const AFE_GENERAL2_ASRC_2CH_CON10: u32 = (0x0f28);
pub const AFE_GENERAL2_ASRC_2CH_CON12: u32 = (0x0f30);
pub const AFE_GENERAL2_ASRC_2CH_CON13: u32 = (0x0f34);

pub const AFE_SECURE_MASK_CONN28: u32 = (0x0f48);
pub const AFE_SECURE_MASK_CONN29: u32 = (0x0f4c);
pub const AFE_SECURE_MASK_CONN30: u32 = (0x0f50);
pub const AFE_SECURE_MASK_CONN31: u32 = (0x0f54);
pub const AFE_SECURE_MASK_CONN32: u32 = (0x0f58);
pub const AFE_SECURE_MASK_CONN33: u32 = (0x0f5c);
pub const AFE_SECURE_MASK_CONN34: u32 = (0x0f60);
pub const AFE_SECURE_MASK_CONN35: u32 = (0x0f64);
pub const AFE_SECURE_MASK_CONN36: u32 = (0x0f68);
pub const AFE_SECURE_MASK_CONN37: u32 = (0x0f6c);
pub const AFE_SECURE_MASK_CONN38: u32 = (0x0f70);
pub const AFE_SECURE_MASK_CONN39: u32 = (0x0f74);
pub const AFE_SECURE_MASK_CONN40: u32 = (0x0f78);
pub const AFE_SECURE_MASK_CONN41: u32 = (0x0f7c);
pub const AFE_SIDEBAND0: u32 = (0x0f80);
pub const AFE_SIDEBAND1: u32 = (0x0f84);
pub const AFE_SECURE_SIDEBAND0: u32 = (0x0f88);
pub const AFE_SECURE_SIDEBAND1: u32 = (0x0f8c);
pub const AFE_SECURE_MASK_CONN0: u32 = (0x0f90);
pub const AFE_SECURE_MASK_CONN1: u32 = (0x0f94);
pub const AFE_SECURE_MASK_CONN2: u32 = (0x0f98);
pub const AFE_SECURE_MASK_CONN3: u32 = (0x0f9c);
pub const AFE_SECURE_MASK_CONN4: u32 = (0x0fa0);
pub const AFE_SECURE_MASK_CONN5: u32 = (0x0fa4);
pub const AFE_SECURE_MASK_CONN6: u32 = (0x0fa8);
pub const AFE_SECURE_MASK_CONN7: u32 = (0x0fac);
pub const AFE_SECURE_MASK_CONN8: u32 = (0x0fb0);
pub const AFE_SECURE_MASK_CONN9: u32 = (0x0fb4);
pub const AFE_SECURE_MASK_CONN10: u32 = (0x0fb8);
pub const AFE_SECURE_MASK_CONN11: u32 = (0x0fbc);
pub const AFE_SECURE_MASK_CONN12: u32 = (0x0fc0);
pub const AFE_SECURE_MASK_CONN13: u32 = (0x0fc4);
pub const AFE_SECURE_MASK_CONN14: u32 = (0x0fc8);
pub const AFE_SECURE_MASK_CONN15: u32 = (0x0fcc);
pub const AFE_SECURE_MASK_CONN16: u32 = (0x0fd0);
pub const AFE_SECURE_MASK_CONN17: u32 = (0x0fd4);
pub const AFE_SECURE_MASK_CONN18: u32 = (0x0fd8);
pub const AFE_SECURE_MASK_CONN19: u32 = (0x0fdc);
pub const AFE_SECURE_MASK_CONN20: u32 = (0x0fe0);
pub const AFE_SECURE_MASK_CONN21: u32 = (0x0fe4);
pub const AFE_SECURE_MASK_CONN22: u32 = (0x0fe8);
pub const AFE_SECURE_MASK_CONN23: u32 = (0x0fec);
pub const AFE_SECURE_MASK_CONN24: u32 = (0x0ff0);
pub const AFE_SECURE_MASK_CONN25: u32 = (0x0ff4);
pub const AFE_SECURE_MASK_CONN26: u32 = (0x0ff8);
pub const AFE_SECURE_MASK_CONN27: u32 = (0x0ffc);

pub const MAX_REGISTER: u32 = AFE_SECURE_MASK_CONN27;

pub const AFE_IRQ_STATUS_BITS: u32 = 0x3ff;

/* AUDIO_TOP_CON0 (0x0000) */
pub const AUD_TCON0_PDN_TML: u32 = (1u32 << 27);
pub const AUD_TCON0_PDN_DAC_PREDIS: u32 = (1u32 << 26);
pub const AUD_TCON0_PDN_DAC: u32 = (1u32 << 25);
pub const AUD_TCON0_PDN_ADC: u32 = (1u32 << 24);
pub const AUD_TCON0_PDN_TDM_IN: u32 = (1u32 << 23);
pub const AUD_TCON0_PDN_TDM_OUT: u32 = (1u32 << 22);
pub const AUD_TCON0_PDN_SPDIF: u32 = (1u32 << 21);
pub const AUD_TCON0_PDN_APLL_TUNER: u32 = (1u32 << 19);
pub const AUD_TCON0_PDN_APLL2_TUNER: u32 = (1u32 << 18);
pub const AUD_TCON0_PDN_INTDIR: u32 = (1u32 << 15);
pub const AUD_TCON0_PDN_24M: u32 = (1u32 << 9);
pub const AUD_TCON0_PDN_22M: u32 = (1u32 << 8);
pub const AUD_TCON0_PDN_I2S_IN: u32 = (1u32 << 6);
pub const AUD_TCON0_PDN_AFE: u32 = (1u32 << 2);

/* AUDIO_TOP_CON1 (0x0004) */
pub const AUD_TCON1_PDN_TDM_ASRC: u32 = (1u32 << 15);
pub const AUD_TCON1_PDN_GENERAL2_ASRC: u32 = (1u32 << 14);
pub const AUD_TCON1_PDN_GENERAL1_ASRC: u32 = (1u32 << 13);
pub const AUD_TCON1_PDN_CONNSYS_I2S_ASRC: u32 = (1u32 << 12);
pub const AUD_TCON1_PDN_DMIC3_ADC: u32 = (1u32 << 11);
pub const AUD_TCON1_PDN_DMIC2_ADC: u32 = (1u32 << 10);
pub const AUD_TCON1_PDN_DMIC1_ADC: u32 = (1u32 << 9);
pub const AUD_TCON1_PDN_DMIC0_ADC: u32 = (1u32 << 8);
pub const AUD_TCON1_PDN_I2S4_BCLK: u32 = (1u32 << 7);
pub const AUD_TCON1_PDN_I2S3_BCLK: u32 = (1u32 << 6);
pub const AUD_TCON1_PDN_I2S2_BCLK: u32 = (1u32 << 5);
pub const AUD_TCON1_PDN_I2S1_BCLK: u32 = (1u32 << 4);

/* AUDIO_TOP_CON3 (0x000C) */
pub const AUD_TCON3_HDMI_BCK_INV: u32 = (1u32 << 3);

/* AFE_I2S_CON (0x0018) */
pub const AFE_I2S_CON_PHASE_SHIFT_FIX: u32 = (1u32 << 31);
pub const AFE_I2S_CON_FROM_IO_MUX: u32 = (1u32 << 28);
pub const AFE_I2S_CON_LOW_JITTER_CLK: u32 = (1u32 << 12);
pub const AFE_I2S_CON_RATE_MASK: u32 = GENMASK(11, 8);
pub const AFE_I2S_CON_FORMAT_I2S: u32 = (1u32 << 3);
pub const AFE_I2S_CON_SRC_SLAVE: u32 = (1u32 << 2);

/* AFE_ASRC_2CH_CON0 */
pub const ONE_HEART: u32 = (1u32 << 31);
pub const CHSET_STR_CLR: u32 = (1u32 << 4);
pub const COEFF_SRAM_CTRL: u32 = (1u32 << 1);
pub const ASM_ON: u32 = (1u32 << 0);

/* CON2 */
pub const O16BIT: u32 = (1u32 << 19);
pub const CLR_IIR_HISTORY: u32 = (1u32 << 17);
pub const IS_MONO: u32 = (1u32 << 16);
pub const IIR_EN: u32 = (1u32 << 11);
pub const IIR_STAGE_MASK: u32 = GENMASK(10, 8);

/* CON5 */
pub const CALI_CYCLE_MASK: u32 = GENMASK(31, 16);
pub const CALI_64_CYCLE: u32 = FIELD_PREP(CALI_CYCLE_MASK, 0x3F);
pub const CALI_96_CYCLE: u32 = FIELD_PREP(CALI_CYCLE_MASK, 0x5F);
pub const CALI_441_CYCLE: u32 = FIELD_PREP(CALI_CYCLE_MASK, 0x1B8);

pub const CALI_AUTORST: u32 = (1u32 << 15);
pub const AUTO_TUNE_FREQ5: u32 = (1u32 << 12);
pub const COMP_FREQ_RES: u32 = (1u32 << 11);

pub const CALI_SEL_MASK: u32 = GENMASK(9, 8);
pub const CALI_SEL_00: u32 = FIELD_PREP(CALI_SEL_MASK, 0);
pub const CALI_SEL_01: u32 = FIELD_PREP(CALI_SEL_MASK, 1);

pub const CALI_BP_DGL: u32 = (1u32 << 7); /* Bypass the deglitch circuit */
pub const AUTO_TUNE_FREQ4: u32 = (1u32 << 3);
pub const CALI_AUTO_RESTART: u32 = (1u32 << 2);
pub const CALI_USE_FREQ_OUT: u32 = (1u32 << 1);
pub const CALI_ON: u32 = (1u32 << 0);

pub const AFE_I2S_CON_WLEN_32BIT: u32 = (1u32 << 1);
pub const AFE_I2S_CON_EN: u32 = (1u32 << 0);

pub const AFE_CONN3_I03_O03_S: u32 = (1u32 << 3);
pub const AFE_CONN4_I04_O04_S: u32 = (1u32 << 4);
pub const AFE_CONN4_I03_O04_S: u32 = (1u32 << 3);

/* AFE_I2S_CON1 (0x0034) */
pub const AFE_I2S_CON1_I2S2_TO_PAD: u32 = (1u32 << 18);
pub const AFE_I2S_CON1_TDMOUT_TO_PAD: u32 = 0u32;
pub const AFE_I2S_CON1_RATE: u32 = GENMASK(11, 8);
pub const AFE_I2S_CON1_FORMAT_I2S: u32 = (1u32 << 3);
pub const AFE_I2S_CON1_WLEN_32BIT: u32 = (1u32 << 1);
pub const AFE_I2S_CON1_EN: u32 = (1u32 << 0);

/* AFE_I2S_CON2 (0x0038) */
pub const AFE_I2S_CON2_LOW_JITTER_CLK: u32 = (1u32 << 12);
pub const AFE_I2S_CON2_RATE: u32 = GENMASK(11, 8);
pub const AFE_I2S_CON2_FORMAT_I2S: u32 = (1u32 << 3);
pub const AFE_I2S_CON2_WLEN_32BIT: u32 = (1u32 << 1);
pub const AFE_I2S_CON2_EN: u32 = (1u32 << 0);

/* AFE_I2S_CON3 (0x004C) */
pub const AFE_I2S_CON3_LOW_JITTER_CLK: u32 = (1u32 << 12);
pub const AFE_I2S_CON3_RATE: u32 = GENMASK(11, 8);
pub const AFE_I2S_CON3_FORMAT_I2S: u32 = (1u32 << 3);
pub const AFE_I2S_CON3_WLEN_32BIT: u32 = (1u32 << 1);
pub const AFE_I2S_CON3_EN: u32 = (1u32 << 0);

/* AFE_ADDA_DL_SRC2_CON0 (0x0108) */
pub const AFE_ADDA_DL_SAMPLING_RATE: u32 = GENMASK(31, 28);
pub const AFE_ADDA_DL_8X_UPSAMPLE: u32 = GENMASK(25, 24);
pub const AFE_ADDA_DL_MUTE_OFF_CH1: u32 = (1u32 << 12);
pub const AFE_ADDA_DL_MUTE_OFF_CH2: u32 = (1u32 << 11);
pub const AFE_ADDA_DL_VOICE_DATA: u32 = (1u32 << 5);
pub const AFE_ADDA_DL_DEGRADE_GAIN: u32 = (1u32 << 1);

/* AFE_ADDA_UL_SRC_CON0 (0x0114) */
pub const AFE_ADDA_UL_SAMPLING_RATE: u32 = GENMASK(19, 17);

/* AFE_ADDA_UL_DL_CON0 */
pub const AFE_ADDA_UL_DL_ADDA_AFE_ON: u32 = (1u32 << 0);
pub const AFE_ADDA_UL_DL_DMIC_CLKDIV_ON: u32 = (1u32 << 1);

/* AFE_APLL_TUNER_CFG (0x03f0) */
pub const AFE_APLL_TUNER_CFG_MASK: u32 = GENMASK(15, 1);
pub const AFE_APLL_TUNER_CFG_EN_MASK: u32 = (1u32 << 0);

/* AFE_APLL_TUNER_CFG1 (0x03f4) */
pub const AFE_APLL_TUNER_CFG1_MASK: u32 = GENMASK(15, 1);
pub const AFE_APLL_TUNER_CFG1_EN_MASK: u32 = (1u32 << 0);

/* PCM_INTF_CON1 (0x0550) */
pub const PCM_INTF_CON1_EXT_MODEM: u32 = (1u32 << 17);
pub const PCM_INTF_CON1_16BIT: u32 = 0u32;
pub const PCM_INTF_CON1_24BIT: u32 = (1u32 << 16);
pub const PCM_INTF_CON1_32BCK: u32 = 0u32;
pub const PCM_INTF_CON1_64BCK: u32 = (1u32 << 14);
pub const PCM_INTF_CON1_MASTER_MODE: u32 = 0u32;
pub const PCM_INTF_CON1_SLAVE_MODE: u32 = (1u32 << 5);
pub const PCM_INTF_CON1_FS_MASK: u32 = GENMASK(4, 3);
pub const PCM_INTF_CON1_FS_8K: u32 = FIELD_PREP(PCM_INTF_CON1_FS_MASK, 0);
pub const PCM_INTF_CON1_FS_16K: u32 = FIELD_PREP(PCM_INTF_CON1_FS_MASK, 1);
pub const PCM_INTF_CON1_FS_32K: u32 = FIELD_PREP(PCM_INTF_CON1_FS_MASK, 2);
pub const PCM_INTF_CON1_FS_48K: u32 = FIELD_PREP(PCM_INTF_CON1_FS_MASK, 3);
pub const PCM_INTF_CON1_SYNC_LEN_MASK: u32 = GENMASK(13, 9);
pub const fn PCM_INTF_CON1_SYNC_LEN(x: u32) -> u32 { FIELD_PREP(PCM_INTF_CON1_SYNC_LEN_MASK, (x - 1)) }
pub const PCM_INTF_CON1_FORMAT_MASK: u32 = GENMASK(2, 1);
pub const PCM_INTF_CON1_SYNC_OUT_INV: u32 = (1u32 << 23);
pub const PCM_INTF_CON1_BCLK_OUT_INV: u32 = (1u32 << 22);
pub const PCM_INTF_CON1_SYNC_IN_INV: u32 = (1u32 << 21);
pub const PCM_INTF_CON1_BCLK_IN_INV: u32 = (1u32 << 20);
pub const PCM_INTF_CON1_BYPASS_ASRC: u32 = (1u32 << 6);
pub const PCM_INTF_CON1_EN: u32 = (1u32 << 0);
pub const PCM_INTF_CON1_CONFIG_MASK: u32 = (0xf3fffe);

/* AFE_DMIC0_UL_SRC_CON0 (0x05b4)
 * AFE_DMIC1_UL_SRC_CON0 (0x0620)
 * AFE_DMIC2_UL_SRC_CON0 (0x0780)
 * AFE_DMIC3_UL_SRC_CON0 (0x07ec)
 */
pub const DMIC_TOP_CON_CK_PHASE_SEL_CH1: u32 = GENMASK(29, 27);
pub const DMIC_TOP_CON_CK_PHASE_SEL_CH2: u32 = GENMASK(26, 24);
pub const DMIC_TOP_CON_TWO_WIRE_MODE: u32 = (1u32 << 23);
pub const DMIC_TOP_CON_CH2_ON: u32 = (1u32 << 22);
pub const DMIC_TOP_CON_CH1_ON: u32 = (1u32 << 21);
pub const DMIC_TOP_CON_VOICE_MODE_MASK: u32 = GENMASK(19, 17);
pub const DMIC_TOP_CON_VOICE_MODE_8K: u32 = FIELD_PREP(DMIC_TOP_CON_VOICE_MODE_MASK, 0);
pub const DMIC_TOP_CON_VOICE_MODE_16K: u32 = FIELD_PREP(DMIC_TOP_CON_VOICE_MODE_MASK, 1);
pub const DMIC_TOP_CON_VOICE_MODE_32K: u32 = FIELD_PREP(DMIC_TOP_CON_VOICE_MODE_MASK, 2);
pub const DMIC_TOP_CON_VOICE_MODE_48K: u32 = FIELD_PREP(DMIC_TOP_CON_VOICE_MODE_MASK, 3);
pub const DMIC_TOP_CON_LOW_POWER_MODE_MASK: u32 = GENMASK(15, 14);
pub const fn DMIC_TOP_CON_LOW_POWER_MODE(x: u32) -> u32 { FIELD_PREP(DMIC_TOP_CON_LOW_POWER_MODE_MASK, x) }
pub const DMIC_TOP_CON_IIR_ON: u32 = (1u32 << 10);
pub const DMIC_TOP_CON_IIR_MODE: u32 = GENMASK(9, 7);
pub const DMIC_TOP_CON_INPUT_MODE: u32 = (1u32 << 5);
pub const DMIC_TOP_CON_SDM3_LEVEL_MODE: u32 = (1u32 << 1);
pub const DMIC_TOP_CON_SRC_ON: u32 = (1u32 << 0);
pub const DMIC_TOP_CON_SDM3_DE_SELECT: u32 = 0u32;
pub const DMIC_TOP_CON_CONFIG_MASK: u32 = (0x3f8ed7a6);

/* AFE_CONN_24BIT (0x0AA4) */
pub const AFE_CONN_24BIT_O10: u32 = (1u32 << 10);
pub const AFE_CONN_24BIT_O09: u32 = (1u32 << 9);
pub const AFE_CONN_24BIT_O06: u32 = (1u32 << 6);
pub const AFE_CONN_24BIT_O05: u32 = (1u32 << 5);
pub const AFE_CONN_24BIT_O04: u32 = (1u32 << 4);
pub const AFE_CONN_24BIT_O03: u32 = (1u32 << 3);
pub const AFE_CONN_24BIT_O02: u32 = (1u32 << 2);
pub const AFE_CONN_24BIT_O01: u32 = (1u32 << 1);
pub const AFE_CONN_24BIT_O00: u32 = (1u32 << 0);

/* AFE_HD_ENGEN_ENABLE */
pub const AFE_22M_PLL_EN: u32 = (1u32 << 0);
pub const AFE_24M_PLL_EN: u32 = (1u32 << 1);

/* AFE_GAIN1_CON0 (0x0410) */
pub const AFE_GAIN1_CON0_EN_MASK: u32 = GENMASK(0, 0);
pub const AFE_GAIN1_CON0_MODE_MASK: u32 = GENMASK(7, 4);
pub const AFE_GAIN1_CON0_SAMPLE_PER_STEP_MASK: u32 = GENMASK(15, 8);

/* AFE_GAIN1_CON1 (0x0414) */
pub const AFE_GAIN1_CON1_MASK: u32 = GENMASK(19, 0);

/* AFE_GAIN1_CUR (0x0B78) */
pub const AFE_GAIN1_CUR_MASK: u32 = GENMASK(19, 0);

/* AFE_CM1_CON0 (0x0e50) */
/* AFE_CM2_CON0 (0x0e60) */
pub const CM_AFE_CM_CH_NUM_MASK: u32 = GENMASK(3, 0);
pub const fn CM_AFE_CM_CH_NUM(x: u32) -> u32 { FIELD_PREP(CM_AFE_CM_CH_NUM_MASK, (x - 1)) }
pub const CM_AFE_CM_ON: u32 = (1u32 << 4);
pub const CM_AFE_CM_START_DATA_MASK: u32 = GENMASK(11, 8);

pub const CM_AFE_CM1_VUL_SEL: u32 = (1u32 << 12);
pub const CM_AFE_CM1_IN_MODE_MASK: u32 = GENMASK(19, 16);
pub const CM_AFE_CM2_TDM_SEL: u32 = (1u32 << 12);
pub const CM_AFE_CM2_CLK_SEL: u32 = (1u32 << 13);
pub const CM_AFE_CM2_GASRC1_OUT_SEL: u32 = (1u32 << 17);
pub const CM_AFE_CM2_GASRC2_OUT_SEL: u32 = (1u32 << 16);

/* AFE_CM2_CONN* */
pub const fn CM2_AFE_CM2_CONN_CFG1(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG1_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG1_MASK: u32 = GENMASK(4, 0);
pub const fn CM2_AFE_CM2_CONN_CFG2(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG2_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG2_MASK: u32 = GENMASK(9, 5);
pub const fn CM2_AFE_CM2_CONN_CFG3(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG3_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG3_MASK: u32 = GENMASK(14, 10);
pub const fn CM2_AFE_CM2_CONN_CFG4(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG4_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG4_MASK: u32 = GENMASK(19, 15);
pub const fn CM2_AFE_CM2_CONN_CFG5(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG5_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG5_MASK: u32 = GENMASK(24, 20);
pub const fn CM2_AFE_CM2_CONN_CFG6(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG6_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG6_MASK: u32 = GENMASK(29, 25);
pub const fn CM2_AFE_CM2_CONN_CFG7(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG7_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG7_MASK: u32 = GENMASK(4, 0);
pub const fn CM2_AFE_CM2_CONN_CFG8(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG8_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG8_MASK: u32 = GENMASK(9, 5);
pub const fn CM2_AFE_CM2_CONN_CFG9(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG9_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG9_MASK: u32 = GENMASK(14, 10);
pub const fn CM2_AFE_CM2_CONN_CFG10(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG10_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG10_MASK: u32 = GENMASK(19, 15);
pub const fn CM2_AFE_CM2_CONN_CFG11(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG11_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG11_MASK: u32 = GENMASK(24, 20);
pub const fn CM2_AFE_CM2_CONN_CFG12(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG12_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG12_MASK: u32 = GENMASK(29, 25);
pub const fn CM2_AFE_CM2_CONN_CFG13(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG13_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG13_MASK: u32 = GENMASK(4, 0);
pub const fn CM2_AFE_CM2_CONN_CFG14(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG14_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG14_MASK: u32 = GENMASK(9, 5);
pub const fn CM2_AFE_CM2_CONN_CFG15(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG15_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG15_MASK: u32 = GENMASK(14, 10);
pub const fn CM2_AFE_CM2_CONN_CFG16(x: u32) -> u32 { FIELD_PREP(CM2_AFE_CM2_CONN_CFG16_MASK, x) }
pub const CM2_AFE_CM2_CONN_CFG16_MASK: u32 = GENMASK(19, 15);

/* AFE_CM1_CON* */
pub const CM_AFE_CM_UPDATE_CNT1_MASK: u32 = GENMASK(15, 0);
pub const fn CM_AFE_CM_UPDATE_CNT1(x: u32) -> u32 { FIELD_PREP(CM_AFE_CM_UPDATE_CNT1_MASK, x) }
pub const CM_AFE_CM_UPDATE_CNT2_MASK: u32 = GENMASK(31, 16);
pub const fn CM_AFE_CM_UPDATE_CNT2(x: u32) -> u32 { FIELD_PREP(CM_AFE_CM_UPDATE_CNT2_MASK, x) }


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
