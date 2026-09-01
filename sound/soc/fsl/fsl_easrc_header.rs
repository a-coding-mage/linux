/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 NXP
 */

/* C header dependencies: sound/asound.h, linux/dma/imx-dma.h, fsl_asrc_common.h */

use core::ffi::{c_char, c_uint, c_void};

pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type snd_pcm_format_t = crate::snd_pcm_format_t;
pub type firmware = crate::firmware;

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

/* EASRC Register Map */

/* ASRC Input Write FIFO */
pub const fn REG_EASRC_WRFIFO(ctx: u32) -> u32 { 0x000 + 4 * ctx }
/* ASRC Output Read FIFO */
pub const fn REG_EASRC_RDFIFO(ctx: u32) -> u32 { 0x010 + 4 * ctx }
/* ASRC Context Control */
pub const fn REG_EASRC_CC(ctx: u32) -> u32 { 0x020 + 4 * ctx }
/* ASRC Context Control Extended 1 */
pub const fn REG_EASRC_CCE1(ctx: u32) -> u32 { 0x030 + 4 * ctx }
/* ASRC Context Control Extended 2 */
pub const fn REG_EASRC_CCE2(ctx: u32) -> u32 { 0x040 + 4 * ctx }
/* ASRC Control Input Access */
pub const fn REG_EASRC_CIA(ctx: u32) -> u32 { 0x050 + 4 * ctx }
/* ASRC Datapath Processor Control Slot0 */
pub const fn REG_EASRC_DPCS0R0(ctx: u32) -> u32 { 0x060 + 4 * ctx }
pub const fn REG_EASRC_DPCS0R1(ctx: u32) -> u32 { 0x070 + 4 * ctx }
pub const fn REG_EASRC_DPCS0R2(ctx: u32) -> u32 { 0x080 + 4 * ctx }
pub const fn REG_EASRC_DPCS0R3(ctx: u32) -> u32 { 0x090 + 4 * ctx }
/* ASRC Datapath Processor Control Slot1 */
pub const fn REG_EASRC_DPCS1R0(ctx: u32) -> u32 { 0x0A0 + 4 * ctx }
pub const fn REG_EASRC_DPCS1R1(ctx: u32) -> u32 { 0x0B0 + 4 * ctx }
pub const fn REG_EASRC_DPCS1R2(ctx: u32) -> u32 { 0x0C0 + 4 * ctx }
pub const fn REG_EASRC_DPCS1R3(ctx: u32) -> u32 { 0x0D0 + 4 * ctx }
/* ASRC Context Output Control */
pub const fn REG_EASRC_COC(ctx: u32) -> u32 { 0x0E0 + 4 * ctx }
/* ASRC Control Output Access */
pub const fn REG_EASRC_COA(ctx: u32) -> u32 { 0x0F0 + 4 * ctx }
/* ASRC Sample FIFO Status */
pub const fn REG_EASRC_SFS(ctx: u32) -> u32 { 0x100 + 4 * ctx }
/* ASRC Resampling Ratio Low */
pub const fn REG_EASRC_RRL(ctx: u32) -> u32 { 0x110 + 8 * ctx }
/* ASRC Resampling Ratio High */
pub const fn REG_EASRC_RRH(ctx: u32) -> u32 { 0x114 + 8 * ctx }
/* ASRC Resampling Ratio Update Control */
pub const fn REG_EASRC_RUC(ctx: u32) -> u32 { 0x130 + 4 * ctx }
/* ASRC Resampling Ratio Update Rate */
pub const fn REG_EASRC_RUR(ctx: u32) -> u32 { 0x140 + 4 * ctx }
/* ASRC Resampling Center Tap Coefficient Low */
pub const REG_EASRC_RCTCL: u32 = 0x150;
/* ASRC Resampling Center Tap Coefficient High */
pub const REG_EASRC_RCTCH: u32 = 0x154;
/* ASRC Prefilter Coefficient FIFO */
pub const fn REG_EASRC_PCF(ctx: u32) -> u32 { 0x160 + 4 * ctx }
/* ASRC Context Resampling Coefficient Memory */
pub const REG_EASRC_CRCM: u32 = 0x170;
/* ASRC Context Resampling Coefficient Control*/
pub const REG_EASRC_CRCC: u32 = 0x174;
/* ASRC Interrupt Control */
pub const REG_EASRC_IRQC: u32 = 0x178;
/* ASRC Interrupt Status Flags */
pub const REG_EASRC_IRQF: u32 = 0x17C;
/* ASRC Channel Status 0 */
pub const fn REG_EASRC_CS0(ctx: u32) -> u32 { 0x180 + 4 * ctx }
/* ASRC Channel Status 1 */
pub const fn REG_EASRC_CS1(ctx: u32) -> u32 { 0x190 + 4 * ctx }
/* ASRC Channel Status 2 */
pub const fn REG_EASRC_CS2(ctx: u32) -> u32 { 0x1A0 + 4 * ctx }
/* ASRC Channel Status 3 */
pub const fn REG_EASRC_CS3(ctx: u32) -> u32 { 0x1B0 + 4 * ctx }
/* ASRC Channel Status 4 */
pub const fn REG_EASRC_CS4(ctx: u32) -> u32 { 0x1C0 + 4 * ctx }
/* ASRC Channel Status 5 */
pub const fn REG_EASRC_CS5(ctx: u32) -> u32 { 0x1D0 + 4 * ctx }
/* ASRC Debug Control Register */
pub const REG_EASRC_DBGC: u32 = 0x1E0;
/* ASRC Debug Status Register */
pub const REG_EASRC_DBGS: u32 = 0x1E4;

pub const fn REG_EASRC_FIFO(x: u32, ctx: u32) -> u32 {
    if x == crate::IN { REG_EASRC_WRFIFO(ctx) } else { REG_EASRC_RDFIFO(ctx) }
}

/* ASRC Context Control (CC) */
pub const EASRC_CC_EN_SHIFT: u32 = 31;
pub const EASRC_CC_EN_MASK: u32 = BIT(EASRC_CC_EN_SHIFT);
pub const EASRC_CC_EN: u32 = BIT(EASRC_CC_EN_SHIFT);
pub const EASRC_CC_STOP_SHIFT: u32 = 29;
pub const EASRC_CC_STOP_MASK: u32 = BIT(EASRC_CC_STOP_SHIFT);
pub const EASRC_CC_STOP: u32 = BIT(EASRC_CC_STOP_SHIFT);
pub const EASRC_CC_FWMDE_SHIFT: u32 = 28;
pub const EASRC_CC_FWMDE_MASK: u32 = BIT(EASRC_CC_FWMDE_SHIFT);
pub const EASRC_CC_FWMDE: u32 = BIT(EASRC_CC_FWMDE_SHIFT);
pub const EASRC_CC_FIFO_WTMK_SHIFT: u32 = 16;
pub const EASRC_CC_FIFO_WTMK_WIDTH: u32 = 7;
pub const EASRC_CC_FIFO_WTMK_MASK: u32 = ((BIT(EASRC_CC_FIFO_WTMK_WIDTH) - 1) << EASRC_CC_FIFO_WTMK_SHIFT);
pub const fn EASRC_CC_FIFO_WTMK(v: u32) -> u32 { (v << EASRC_CC_FIFO_WTMK_SHIFT) & EASRC_CC_FIFO_WTMK_MASK }
pub const EASRC_CC_SAMPLE_POS_SHIFT: u32 = 11;
pub const EASRC_CC_SAMPLE_POS_WIDTH: u32 = 5;
pub const EASRC_CC_SAMPLE_POS_MASK: u32 = ((BIT(EASRC_CC_SAMPLE_POS_WIDTH) - 1) << EASRC_CC_SAMPLE_POS_SHIFT);
pub const fn EASRC_CC_SAMPLE_POS(v: u32) -> u32 { (v << EASRC_CC_SAMPLE_POS_SHIFT) & EASRC_CC_SAMPLE_POS_MASK }
pub const EASRC_CC_ENDIANNESS_SHIFT: u32 = 10;
pub const EASRC_CC_ENDIANNESS_MASK: u32 = BIT(EASRC_CC_ENDIANNESS_SHIFT);
pub const EASRC_CC_ENDIANNESS: u32 = BIT(EASRC_CC_ENDIANNESS_SHIFT);
pub const EASRC_CC_BPS_SHIFT: u32 = 8;
pub const EASRC_CC_BPS_WIDTH: u32 = 2;
pub const EASRC_CC_BPS_MASK: u32 = ((BIT(EASRC_CC_BPS_WIDTH) - 1) << EASRC_CC_BPS_SHIFT);
pub const fn EASRC_CC_BPS(v: u32) -> u32 { (v << EASRC_CC_BPS_SHIFT) & EASRC_CC_BPS_MASK }
pub const EASRC_CC_FMT_SHIFT: u32 = 7;
pub const EASRC_CC_FMT_MASK: u32 = BIT(EASRC_CC_FMT_SHIFT);
pub const EASRC_CC_FMT: u32 = BIT(EASRC_CC_FMT_SHIFT);
pub const EASRC_CC_INSIGN_SHIFT: u32 = 6;
pub const EASRC_CC_INSIGN_MASK: u32 = BIT(EASRC_CC_INSIGN_SHIFT);
pub const EASRC_CC_INSIGN: u32 = BIT(EASRC_CC_INSIGN_SHIFT);
pub const EASRC_CC_CHEN_SHIFT: u32 = 0;
pub const EASRC_CC_CHEN_WIDTH: u32 = 5;
pub const EASRC_CC_CHEN_MASK: u32 = ((BIT(EASRC_CC_CHEN_WIDTH) - 1) << EASRC_CC_CHEN_SHIFT);
pub const fn EASRC_CC_CHEN(v: u32) -> u32 { (v << EASRC_CC_CHEN_SHIFT) & EASRC_CC_CHEN_MASK }

/* ASRC Context Control Extended 1 (CCE1) */
pub const EASRC_CCE1_COEF_WS_SHIFT: u32 = 25;
pub const EASRC_CCE1_COEF_WS_MASK: u32 = BIT(EASRC_CCE1_COEF_WS_SHIFT);
pub const EASRC_CCE1_COEF_WS: u32 = BIT(EASRC_CCE1_COEF_WS_SHIFT);
pub const EASRC_CCE1_COEF_MEM_RST_SHIFT: u32 = 24;
pub const EASRC_CCE1_COEF_MEM_RST_MASK: u32 = BIT(EASRC_CCE1_COEF_MEM_RST_SHIFT);
pub const EASRC_CCE1_COEF_MEM_RST: u32 = BIT(EASRC_CCE1_COEF_MEM_RST_SHIFT);
pub const EASRC_CCE1_PF_EXP_SHIFT: u32 = 16;
pub const EASRC_CCE1_PF_EXP_WIDTH: u32 = 8;
pub const EASRC_CCE1_PF_EXP_MASK: u32 = ((BIT(EASRC_CCE1_PF_EXP_WIDTH) - 1) << EASRC_CCE1_PF_EXP_SHIFT);
pub const fn EASRC_CCE1_PF_EXP(v: u32) -> u32 { (v << EASRC_CCE1_PF_EXP_SHIFT) & EASRC_CCE1_PF_EXP_MASK }
pub const EASRC_CCE1_PF_ST1_WBFP_SHIFT: u32 = 9;
pub const EASRC_CCE1_PF_ST1_WBFP_MASK: u32 = BIT(EASRC_CCE1_PF_ST1_WBFP_SHIFT);
pub const EASRC_CCE1_PF_ST1_WBFP: u32 = BIT(EASRC_CCE1_PF_ST1_WBFP_SHIFT);
pub const EASRC_CCE1_PF_TSEN_SHIFT: u32 = 8;
pub const EASRC_CCE1_PF_TSEN_MASK: u32 = BIT(EASRC_CCE1_PF_TSEN_SHIFT);
pub const EASRC_CCE1_PF_TSEN: u32 = BIT(EASRC_CCE1_PF_TSEN_SHIFT);
pub const EASRC_CCE1_RS_BYPASS_SHIFT: u32 = 7;
pub const EASRC_CCE1_RS_BYPASS_MASK: u32 = BIT(EASRC_CCE1_RS_BYPASS_SHIFT);
pub const EASRC_CCE1_RS_BYPASS: u32 = BIT(EASRC_CCE1_RS_BYPASS_SHIFT);
pub const EASRC_CCE1_PF_BYPASS_SHIFT: u32 = 6;
pub const EASRC_CCE1_PF_BYPASS_MASK: u32 = BIT(EASRC_CCE1_PF_BYPASS_SHIFT);
pub const EASRC_CCE1_PF_BYPASS: u32 = BIT(EASRC_CCE1_PF_BYPASS_SHIFT);
pub const EASRC_CCE1_RS_STOP_SHIFT: u32 = 5;
pub const EASRC_CCE1_RS_STOP_MASK: u32 = BIT(EASRC_CCE1_RS_STOP_SHIFT);
pub const EASRC_CCE1_RS_STOP: u32 = BIT(EASRC_CCE1_RS_STOP_SHIFT);
pub const EASRC_CCE1_PF_STOP_SHIFT: u32 = 4;
pub const EASRC_CCE1_PF_STOP_MASK: u32 = BIT(EASRC_CCE1_PF_STOP_SHIFT);
pub const EASRC_CCE1_PF_STOP: u32 = BIT(EASRC_CCE1_PF_STOP_SHIFT);
pub const EASRC_CCE1_RS_INIT_SHIFT: u32 = 2;
pub const EASRC_CCE1_RS_INIT_WIDTH: u32 = 2;
pub const EASRC_CCE1_RS_INIT_MASK: u32 = ((BIT(EASRC_CCE1_RS_INIT_WIDTH) - 1) << EASRC_CCE1_RS_INIT_SHIFT);
pub const fn EASRC_CCE1_RS_INIT(v: u32) -> u32 { (v << EASRC_CCE1_RS_INIT_SHIFT) & EASRC_CCE1_RS_INIT_MASK }
pub const EASRC_CCE1_PF_INIT_SHIFT: u32 = 0;
pub const EASRC_CCE1_PF_INIT_WIDTH: u32 = 2;
pub const EASRC_CCE1_PF_INIT_MASK: u32 = ((BIT(EASRC_CCE1_PF_INIT_WIDTH) - 1) << EASRC_CCE1_PF_INIT_SHIFT);
pub const fn EASRC_CCE1_PF_INIT(v: u32) -> u32 { (v << EASRC_CCE1_PF_INIT_SHIFT) & EASRC_CCE1_PF_INIT_MASK }

/* ASRC Context Control Extended 2 (CCE2) */
pub const EASRC_CCE2_ST2_TAPS_SHIFT: u32 = 16;
pub const EASRC_CCE2_ST2_TAPS_WIDTH: u32 = 9;
pub const EASRC_CCE2_ST2_TAPS_MASK: u32 = ((BIT(EASRC_CCE2_ST2_TAPS_WIDTH) - 1) << EASRC_CCE2_ST2_TAPS_SHIFT);
pub const fn EASRC_CCE2_ST2_TAPS(v: u32) -> u32 { (v << EASRC_CCE2_ST2_TAPS_SHIFT) & EASRC_CCE2_ST2_TAPS_MASK }
pub const EASRC_CCE2_ST1_TAPS_SHIFT: u32 = 0;
pub const EASRC_CCE2_ST1_TAPS_WIDTH: u32 = 9;
pub const EASRC_CCE2_ST1_TAPS_MASK: u32 = ((BIT(EASRC_CCE2_ST1_TAPS_WIDTH) - 1) << EASRC_CCE2_ST1_TAPS_SHIFT);
pub const fn EASRC_CCE2_ST1_TAPS(v: u32) -> u32 { (v << EASRC_CCE2_ST1_TAPS_SHIFT) & EASRC_CCE2_ST1_TAPS_MASK }

/* ASRC Control Input Access (CIA) */
pub const EASRC_CIA_ITER_SHIFT: u32 = 16;
pub const EASRC_CIA_ITER_WIDTH: u32 = 6;
pub const EASRC_CIA_ITER_MASK: u32 = ((BIT(EASRC_CIA_ITER_WIDTH) - 1) << EASRC_CIA_ITER_SHIFT);
pub const fn EASRC_CIA_ITER(v: u32) -> u32 { (v << EASRC_CIA_ITER_SHIFT) & EASRC_CIA_ITER_MASK }
pub const EASRC_CIA_GRLEN_SHIFT: u32 = 8;
pub const EASRC_CIA_GRLEN_WIDTH: u32 = 6;
pub const EASRC_CIA_GRLEN_MASK: u32 = ((BIT(EASRC_CIA_GRLEN_WIDTH) - 1) << EASRC_CIA_GRLEN_SHIFT);
pub const fn EASRC_CIA_GRLEN(v: u32) -> u32 { (v << EASRC_CIA_GRLEN_SHIFT) & EASRC_CIA_GRLEN_MASK }
pub const EASRC_CIA_ACCLEN_SHIFT: u32 = 0;
pub const EASRC_CIA_ACCLEN_WIDTH: u32 = 6;
pub const EASRC_CIA_ACCLEN_MASK: u32 = ((BIT(EASRC_CIA_ACCLEN_WIDTH) - 1) << EASRC_CIA_ACCLEN_SHIFT);
pub const fn EASRC_CIA_ACCLEN(v: u32) -> u32 { (v << EASRC_CIA_ACCLEN_SHIFT) & EASRC_CIA_ACCLEN_MASK }

/* ASRC Datapath Processor Control Slot0 Register0 (DPCS0R0) */
pub const EASRC_DPCS0R0_MAXCH_SHIFT: u32 = 24;
pub const EASRC_DPCS0R0_MAXCH_WIDTH: u32 = 5;
pub const EASRC_DPCS0R0_MAXCH_MASK: u32 = ((BIT(EASRC_DPCS0R0_MAXCH_WIDTH) - 1) << EASRC_DPCS0R0_MAXCH_SHIFT);
pub const fn EASRC_DPCS0R0_MAXCH(v: u32) -> u32 { (v << EASRC_DPCS0R0_MAXCH_SHIFT) & EASRC_DPCS0R0_MAXCH_MASK }
pub const EASRC_DPCS0R0_MINCH_SHIFT: u32 = 16;
pub const EASRC_DPCS0R0_MINCH_WIDTH: u32 = 5;
pub const EASRC_DPCS0R0_MINCH_MASK: u32 = ((BIT(EASRC_DPCS0R0_MINCH_WIDTH) - 1) << EASRC_DPCS0R0_MINCH_SHIFT);
pub const fn EASRC_DPCS0R0_MINCH(v: u32) -> u32 { (v << EASRC_DPCS0R0_MINCH_SHIFT) & EASRC_DPCS0R0_MINCH_MASK }
pub const EASRC_DPCS0R0_NUMCH_SHIFT: u32 = 8;
pub const EASRC_DPCS0R0_NUMCH_WIDTH: u32 = 5;
pub const EASRC_DPCS0R0_NUMCH_MASK: u32 = ((BIT(EASRC_DPCS0R0_NUMCH_WIDTH) - 1) << EASRC_DPCS0R0_NUMCH_SHIFT);
pub const fn EASRC_DPCS0R0_NUMCH(v: u32) -> u32 { (v << EASRC_DPCS0R0_NUMCH_SHIFT) & EASRC_DPCS0R0_NUMCH_MASK }
pub const EASRC_DPCS0R0_CTXNUM_SHIFT: u32 = 1;
pub const EASRC_DPCS0R0_CTXNUM_WIDTH: u32 = 2;
pub const EASRC_DPCS0R0_CTXNUM_MASK: u32 = ((BIT(EASRC_DPCS0R0_CTXNUM_WIDTH) - 1) << EASRC_DPCS0R0_CTXNUM_SHIFT);
pub const fn EASRC_DPCS0R0_CTXNUM(v: u32) -> u32 { (v << EASRC_DPCS0R0_CTXNUM_SHIFT) & EASRC_DPCS0R0_CTXNUM_MASK }
pub const EASRC_DPCS0R0_EN_SHIFT: u32 = 0;
pub const EASRC_DPCS0R0_EN_MASK: u32 = BIT(EASRC_DPCS0R0_EN_SHIFT);
pub const EASRC_DPCS0R0_EN: u32 = BIT(EASRC_DPCS0R0_EN_SHIFT);

/* ASRC Datapath Processor Control Slot0 Register1 (DPCS0R1) */
pub const EASRC_DPCS0R1_ST1_EXP_SHIFT: u32 = 0;
pub const EASRC_DPCS0R1_ST1_EXP_WIDTH: u32 = 13;
pub const EASRC_DPCS0R1_ST1_EXP_MASK: u32 = ((BIT(EASRC_DPCS0R1_ST1_EXP_WIDTH) - 1) << EASRC_DPCS0R1_ST1_EXP_SHIFT);
pub const fn EASRC_DPCS0R1_ST1_EXP(v: u32) -> u32 { (v << EASRC_DPCS0R1_ST1_EXP_SHIFT) & EASRC_DPCS0R1_ST1_EXP_MASK }

/* ASRC Datapath Processor Control Slot0 Register2 (DPCS0R2) */
pub const EASRC_DPCS0R2_ST1_MA_SHIFT: u32 = 16;
pub const EASRC_DPCS0R2_ST1_MA_WIDTH: u32 = 13;
pub const EASRC_DPCS0R2_ST1_MA_MASK: u32 = ((BIT(EASRC_DPCS0R2_ST1_MA_WIDTH) - 1) << EASRC_DPCS0R2_ST1_MA_SHIFT);
pub const fn EASRC_DPCS0R2_ST1_MA(v: u32) -> u32 { (v << EASRC_DPCS0R2_ST1_MA_SHIFT) & EASRC_DPCS0R2_ST1_MA_MASK }
pub const EASRC_DPCS0R2_ST1_SA_SHIFT: u32 = 0;
pub const EASRC_DPCS0R2_ST1_SA_WIDTH: u32 = 13;
pub const EASRC_DPCS0R2_ST1_SA_MASK: u32 = ((BIT(EASRC_DPCS0R2_ST1_SA_WIDTH) - 1) << EASRC_DPCS0R2_ST1_SA_SHIFT);
pub const fn EASRC_DPCS0R2_ST1_SA(v: u32) -> u32 { (v << EASRC_DPCS0R2_ST1_SA_SHIFT) & EASRC_DPCS0R2_ST1_SA_MASK }

/* ASRC Datapath Processor Control Slot0 Register3 (DPCS0R3) */
pub const EASRC_DPCS0R3_ST2_MA_SHIFT: u32 = 16;
pub const EASRC_DPCS0R3_ST2_MA_WIDTH: u32 = 13;
pub const EASRC_DPCS0R3_ST2_MA_MASK: u32 = ((BIT(EASRC_DPCS0R3_ST2_MA_WIDTH) - 1) << EASRC_DPCS0R3_ST2_MA_SHIFT);
pub const fn EASRC_DPCS0R3_ST2_MA(v: u32) -> u32 { (v << EASRC_DPCS0R3_ST2_MA_SHIFT) & EASRC_DPCS0R3_ST2_MA_MASK }
pub const EASRC_DPCS0R3_ST2_SA_SHIFT: u32 = 0;
pub const EASRC_DPCS0R3_ST2_SA_WIDTH: u32 = 13;
pub const EASRC_DPCS0R3_ST2_SA_MASK: u32 = ((BIT(EASRC_DPCS0R3_ST2_SA_WIDTH) - 1) << EASRC_DPCS0R3_ST2_SA_SHIFT);
pub const fn EASRC_DPCS0R3_ST2_SA(v: u32) -> u32 { (v << EASRC_DPCS0R3_ST2_SA_SHIFT) & EASRC_DPCS0R3_ST2_SA_MASK }

/* ASRC Context Output Control (COC) */
pub const EASRC_COC_FWMDE_SHIFT: u32 = 28;
pub const EASRC_COC_FWMDE_MASK: u32 = BIT(EASRC_COC_FWMDE_SHIFT);
pub const EASRC_COC_FWMDE: u32 = BIT(EASRC_COC_FWMDE_SHIFT);
pub const EASRC_COC_FIFO_WTMK_SHIFT: u32 = 16;
pub const EASRC_COC_FIFO_WTMK_WIDTH: u32 = 7;
pub const EASRC_COC_FIFO_WTMK_MASK: u32 = ((BIT(EASRC_COC_FIFO_WTMK_WIDTH) - 1) << EASRC_COC_FIFO_WTMK_SHIFT);
pub const fn EASRC_COC_FIFO_WTMK(v: u32) -> u32 { (v << EASRC_COC_FIFO_WTMK_SHIFT) & EASRC_COC_FIFO_WTMK_MASK }
pub const EASRC_COC_SAMPLE_POS_SHIFT: u32 = 11;
pub const EASRC_COC_SAMPLE_POS_WIDTH: u32 = 5;
pub const EASRC_COC_SAMPLE_POS_MASK: u32 = ((BIT(EASRC_COC_SAMPLE_POS_WIDTH) - 1) << EASRC_COC_SAMPLE_POS_SHIFT);
pub const fn EASRC_COC_SAMPLE_POS(v: u32) -> u32 { (v << EASRC_COC_SAMPLE_POS_SHIFT) & EASRC_COC_SAMPLE_POS_MASK }
pub const EASRC_COC_ENDIANNESS_SHIFT: u32 = 10;
pub const EASRC_COC_ENDIANNESS_MASK: u32 = BIT(EASRC_COC_ENDIANNESS_SHIFT);
pub const EASRC_COC_ENDIANNESS: u32 = BIT(EASRC_COC_ENDIANNESS_SHIFT);
pub const EASRC_COC_BPS_SHIFT: u32 = 8;
pub const EASRC_COC_BPS_WIDTH: u32 = 2;
pub const EASRC_COC_BPS_MASK: u32 = ((BIT(EASRC_COC_BPS_WIDTH) - 1) << EASRC_COC_BPS_SHIFT);
pub const fn EASRC_COC_BPS(v: u32) -> u32 { (v << EASRC_COC_BPS_SHIFT) & EASRC_COC_BPS_MASK }
pub const EASRC_COC_FMT_SHIFT: u32 = 7;
pub const EASRC_COC_FMT_MASK: u32 = BIT(EASRC_COC_FMT_SHIFT);
pub const EASRC_COC_FMT: u32 = BIT(EASRC_COC_FMT_SHIFT);
pub const EASRC_COC_OUTSIGN_SHIFT: u32 = 6;
pub const EASRC_COC_OUTSIGN_MASK: u32 = BIT(EASRC_COC_OUTSIGN_SHIFT);
pub const EASRC_COC_OUTSIGN_OUT: u32 = BIT(EASRC_COC_OUTSIGN_SHIFT);
pub const EASRC_COC_IEC_VDATA_SHIFT: u32 = 2;
pub const EASRC_COC_IEC_VDATA_MASK: u32 = BIT(EASRC_COC_IEC_VDATA_SHIFT);
pub const EASRC_COC_IEC_VDATA: u32 = BIT(EASRC_COC_IEC_VDATA_SHIFT);
pub const EASRC_COC_IEC_EN_SHIFT: u32 = 1;
pub const EASRC_COC_IEC_EN_MASK: u32 = BIT(EASRC_COC_IEC_EN_SHIFT);
pub const EASRC_COC_IEC_EN: u32 = BIT(EASRC_COC_IEC_EN_SHIFT);
pub const EASRC_COC_DITHER_EN_SHIFT: u32 = 0;
pub const EASRC_COC_DITHER_EN_MASK: u32 = BIT(EASRC_COC_DITHER_EN_SHIFT);
pub const EASRC_COC_DITHER_EN: u32 = BIT(EASRC_COC_DITHER_EN_SHIFT);

/* ASRC Control Output Access (COA) */
pub const EASRC_COA_ITER_SHIFT: u32 = 16;
pub const EASRC_COA_ITER_WIDTH: u32 = 6;
pub const EASRC_COA_ITER_MASK: u32 = ((BIT(EASRC_COA_ITER_WIDTH) - 1) << EASRC_COA_ITER_SHIFT);
pub const fn EASRC_COA_ITER(v: u32) -> u32 { (v << EASRC_COA_ITER_SHIFT) & EASRC_COA_ITER_MASK }
pub const EASRC_COA_GRLEN_SHIFT: u32 = 8;
pub const EASRC_COA_GRLEN_WIDTH: u32 = 6;
pub const EASRC_COA_GRLEN_MASK: u32 = ((BIT(EASRC_COA_GRLEN_WIDTH) - 1) << EASRC_COA_GRLEN_SHIFT);
pub const fn EASRC_COA_GRLEN(v: u32) -> u32 { (v << EASRC_COA_GRLEN_SHIFT) & EASRC_COA_GRLEN_MASK }
pub const EASRC_COA_ACCLEN_SHIFT: u32 = 0;
pub const EASRC_COA_ACCLEN_WIDTH: u32 = 6;
pub const EASRC_COA_ACCLEN_MASK: u32 = ((BIT(EASRC_COA_ACCLEN_WIDTH) - 1) << EASRC_COA_ACCLEN_SHIFT);
pub const fn EASRC_COA_ACCLEN(v: u32) -> u32 { (v << EASRC_COA_ACCLEN_SHIFT) & EASRC_COA_ACCLEN_MASK }

/* ASRC Sample FIFO Status (SFS) */
pub const EASRC_SFS_IWTMK_SHIFT: u32 = 23;
pub const EASRC_SFS_IWTMK_MASK: u32 = BIT(EASRC_SFS_IWTMK_SHIFT);
pub const EASRC_SFS_IWTMK: u32 = BIT(EASRC_SFS_IWTMK_SHIFT);
pub const EASRC_SFS_NSGI_SHIFT: u32 = 16;
pub const EASRC_SFS_NSGI_WIDTH: u32 = 7;
pub const EASRC_SFS_NSGI_MASK: u32 = ((BIT(EASRC_SFS_NSGI_WIDTH) - 1) << EASRC_SFS_NSGI_SHIFT);
pub const fn EASRC_SFS_NSGI(v: u32) -> u32 { (v << EASRC_SFS_NSGI_SHIFT) & EASRC_SFS_NSGI_MASK }
pub const EASRC_SFS_OWTMK_SHIFT: u32 = 7;
pub const EASRC_SFS_OWTMK_MASK: u32 = BIT(EASRC_SFS_OWTMK_SHIFT);
pub const EASRC_SFS_OWTMK: u32 = BIT(EASRC_SFS_OWTMK_SHIFT);
pub const EASRC_SFS_NSGO_SHIFT: u32 = 0;
pub const EASRC_SFS_NSGO_WIDTH: u32 = 7;
pub const EASRC_SFS_NSGO_MASK: u32 = ((BIT(EASRC_SFS_NSGO_WIDTH) - 1) << EASRC_SFS_NSGO_SHIFT);
pub const fn EASRC_SFS_NSGO(v: u32) -> u32 { (v << EASRC_SFS_NSGO_SHIFT) & EASRC_SFS_NSGO_MASK }

/* ASRC Resampling Ratio Low (RRL) */
pub const EASRC_RRL_RS_RL_SHIFT: u32 = 0;
pub const EASRC_RRL_RS_RL_WIDTH: u32 = 32;
pub const fn EASRC_RRL_RS_RL(v: u32) -> u32 { v << EASRC_RRL_RS_RL_SHIFT }

/* ASRC Resampling Ratio High (RRH) */
pub const EASRC_RRH_RS_VLD_SHIFT: u32 = 31;
pub const EASRC_RRH_RS_VLD_MASK: u32 = BIT(EASRC_RRH_RS_VLD_SHIFT);
pub const EASRC_RRH_RS_VLD: u32 = BIT(EASRC_RRH_RS_VLD_SHIFT);
pub const EASRC_RRH_RS_RH_SHIFT: u32 = 0;
pub const EASRC_RRH_RS_RH_WIDTH: u32 = 12;
pub const EASRC_RRH_RS_RH_MASK: u32 = ((BIT(EASRC_RRH_RS_RH_WIDTH) - 1) << EASRC_RRH_RS_RH_SHIFT);
pub const fn EASRC_RRH_RS_RH(v: u32) -> u32 { (v << EASRC_RRH_RS_RH_SHIFT) & EASRC_RRH_RS_RH_MASK }

/* ASRC Resampling Ratio Update Control (RSUC) */
pub const EASRC_RSUC_RS_RM_SHIFT: u32 = 0;
pub const EASRC_RSUC_RS_RM_WIDTH: u32 = 32;
pub const fn EASRC_RSUC_RS_RM(v: u32) -> u32 { v << EASRC_RSUC_RS_RM_SHIFT }

/* ASRC Resampling Ratio Update Rate (RRUR) */
pub const EASRC_RRUR_RRR_SHIFT: u32 = 0;
pub const EASRC_RRUR_RRR_WIDTH: u32 = 31;
pub const EASRC_RRUR_RRR_MASK: u32 = ((BIT(EASRC_RRUR_RRR_WIDTH) - 1) << EASRC_RRUR_RRR_SHIFT);
pub const fn EASRC_RRUR_RRR(v: u32) -> u32 { (v << EASRC_RRUR_RRR_SHIFT) & EASRC_RRUR_RRR_MASK }

/* ASRC Resampling Center Tap Coefficient Low (RCTCL) */
pub const EASRC_RCTCL_RS_CL_SHIFT: u32 = 0;
pub const EASRC_RCTCL_RS_CL_WIDTH: u32 = 32;
pub const fn EASRC_RCTCL_RS_CL(v: u32) -> u32 { v << EASRC_RCTCL_RS_CL_SHIFT }

/* ASRC Resampling Center Tap Coefficient High (RCTCH) */
pub const EASRC_RCTCH_RS_CH_SHIFT: u32 = 0;
pub const EASRC_RCTCH_RS_CH_WIDTH: u32 = 32;
pub const fn EASRC_RCTCH_RS_CH(v: u32) -> u32 { v << EASRC_RCTCH_RS_CH_SHIFT }

/* ASRC Prefilter Coefficient FIFO (PCF) */
pub const EASRC_PCF_CD_SHIFT: u32 = 0;
pub const EASRC_PCF_CD_WIDTH: u32 = 32;
pub const fn EASRC_PCF_CD(v: u32) -> u32 { v << EASRC_PCF_CD_SHIFT }

/* ASRC Context Resampling Coefficient Memory (CRCM) */
pub const EASRC_CRCM_RS_CWD_SHIFT: u32 = 0;
pub const EASRC_CRCM_RS_CWD_WIDTH: u32 = 32;
pub const fn EASRC_CRCM_RS_CWD(v: u32) -> u32 { v << EASRC_CRCM_RS_CWD_SHIFT }

/* ASRC Context Resampling Coefficient Control (CRCC) */
pub const EASRC_CRCC_RS_CA_SHIFT: u32 = 16;
pub const EASRC_CRCC_RS_CA_WIDTH: u32 = 11;
pub const EASRC_CRCC_RS_CA_MASK: u32 = ((BIT(EASRC_CRCC_RS_CA_WIDTH) - 1) << EASRC_CRCC_RS_CA_SHIFT);
pub const fn EASRC_CRCC_RS_CA(v: u32) -> u32 { (v << EASRC_CRCC_RS_CA_SHIFT) & EASRC_CRCC_RS_CA_MASK }
pub const EASRC_CRCC_RS_TAPS_SHIFT: u32 = 1;
pub const EASRC_CRCC_RS_TAPS_WIDTH: u32 = 2;
pub const EASRC_CRCC_RS_TAPS_MASK: u32 = ((BIT(EASRC_CRCC_RS_TAPS_WIDTH) - 1) << EASRC_CRCC_RS_TAPS_SHIFT);
pub const fn EASRC_CRCC_RS_TAPS(v: u32) -> u32 { (v << EASRC_CRCC_RS_TAPS_SHIFT) & EASRC_CRCC_RS_TAPS_MASK }
pub const EASRC_CRCC_RS_CPR_SHIFT: u32 = 0;
pub const EASRC_CRCC_RS_CPR_MASK: u32 = BIT(EASRC_CRCC_RS_CPR_SHIFT);
pub const EASRC_CRCC_RS_CPR: u32 = BIT(EASRC_CRCC_RS_CPR_SHIFT);

/* ASRC Interrupt_Control (IC) */
pub const EASRC_IRQC_RSDM_SHIFT: u32 = 8;
pub const EASRC_IRQC_RSDM_WIDTH: u32 = 4;
pub const EASRC_IRQC_RSDM_MASK: u32 = ((BIT(EASRC_IRQC_RSDM_WIDTH) - 1) << EASRC_IRQC_RSDM_SHIFT);
pub const fn EASRC_IRQC_RSDM(v: u32) -> u32 { (v << EASRC_IRQC_RSDM_SHIFT) & EASRC_IRQC_RSDM_MASK }
pub const EASRC_IRQC_OERM_SHIFT: u32 = 4;
pub const EASRC_IRQC_OERM_WIDTH: u32 = 4;
pub const EASRC_IRQC_OERM_MASK: u32 = ((BIT(EASRC_IRQC_OERM_WIDTH) - 1) << EASRC_IRQC_OERM_SHIFT);
pub const fn EASRC_IRQC_OERM(v: u32) -> u32 { (v << EASRC_IRQC_OERM_SHIFT) & crate::EASRC_IEQC_OERM_MASK }
pub const EASRC_IRQC_IOM_SHIFT: u32 = 0;
pub const EASRC_IRQC_IOM_WIDTH: u32 = 4;
pub const EASRC_IRQC_IOM_MASK: u32 = ((BIT(EASRC_IRQC_IOM_WIDTH) - 1) << EASRC_IRQC_IOM_SHIFT);
pub const fn EASRC_IRQC_IOM(v: u32) -> u32 { (v << EASRC_IRQC_IOM_SHIFT) & EASRC_IRQC_IOM_MASK }

/* ASRC Interrupt Status Flags (ISF) */
pub const EASRC_IRQF_RSD_SHIFT: u32 = 8;
pub const EASRC_IRQF_RSD_WIDTH: u32 = 4;
pub const EASRC_IRQF_RSD_MASK: u32 = ((BIT(EASRC_IRQF_RSD_WIDTH) - 1) << EASRC_IRQF_RSD_SHIFT);
pub const fn EASRC_IRQF_RSD(v: u32) -> u32 { (v << EASRC_IRQF_RSD_SHIFT) & EASRC_IRQF_RSD_MASK }
pub const EASRC_IRQF_OER_SHIFT: u32 = 4;
pub const EASRC_IRQF_OER_WIDTH: u32 = 4;
pub const EASRC_IRQF_OER_MASK: u32 = ((BIT(EASRC_IRQF_OER_WIDTH) - 1) << EASRC_IRQF_OER_SHIFT);
pub const fn EASRC_IRQF_OER(v: u32) -> u32 { (v << EASRC_IRQF_OER_SHIFT) & EASRC_IRQF_OER_MASK }
pub const EASRC_IRQF_IFO_SHIFT: u32 = 0;
pub const EASRC_IRQF_IFO_WIDTH: u32 = 4;
pub const EASRC_IRQF_IFO_MASK: u32 = ((BIT(EASRC_IRQF_IFO_WIDTH) - 1) << EASRC_IRQF_IFO_SHIFT);
pub const fn EASRC_IRQF_IFO(v: u32) -> u32 { (v << EASRC_IRQF_IFO_SHIFT) & EASRC_IRQF_IFO_MASK }

/* ASRC Context Channel STAT */
pub const EASRC_CSx_CSx_SHIFT: u32 = 0;
pub const EASRC_CSx_CSx_WIDTH: u32 = 32;
pub const fn EASRC_CSx_CSx(v: u32) -> u32 { v << EASRC_CSx_CSx_SHIFT }

/* ASRC Debug Control Register */
pub const EASRC_DBGC_DMS_SHIFT: u32 = 0;
pub const EASRC_DBGC_DMS_WIDTH: u32 = 6;
pub const EASRC_DBGC_DMS_MASK: u32 = ((BIT(EASRC_DBGC_DMS_WIDTH) - 1) << EASRC_DBGC_DMS_SHIFT);
pub const fn EASRC_DBGC_DMS(v: u32) -> u32 { (v << EASRC_DBGC_DMS_SHIFT) & EASRC_DBGC_DMS_MASK }

/* ASRC Debug Status Register */
pub const EASRC_DBGS_DS_SHIFT: u32 = 0;
pub const EASRC_DBGS_DS_WIDTH: u32 = 32;
pub const fn EASRC_DBGS_DS(v: u32) -> u32 { v << EASRC_DBGS_DS_SHIFT }

/* General Constants */
pub const EASRC_CTX_MAX_NUM: usize = 4;
pub const EASRC_RS_COEFF_MEM: u32 = 0;
pub const EASRC_PF_COEFF_MEM: u32 = 1;

/* Prefilter constants */
pub const EASRC_PF_ST1_ONLY: u32 = 0;
pub const EASRC_PF_TWO_STAGE_MODE: u32 = 1;
pub const EASRC_PF_ST1_COEFF_WR: u32 = 0;
pub const EASRC_PF_ST2_COEFF_WR: u32 = 1;
pub const EASRC_MAX_PF_TAPS: u32 = 384;

/* Resampling constants */
pub const EASRC_RS_32_TAPS: u32 = 0;
pub const EASRC_RS_64_TAPS: u32 = 1;
pub const EASRC_RS_128_TAPS: u32 = 2;

/* Initialization mode */
pub const EASRC_INIT_MODE_SW_CONTROL: u32 = 0;
pub const EASRC_INIT_MODE_REPLICATE: u32 = 1;
pub const EASRC_INIT_MODE_ZERO_FILL: u32 = 2;

/* FIFO watermarks */
pub const FSL_EASRC_INPUTFIFO_WML: u32 = 0x4;
pub const FSL_EASRC_OUTPUTFIFO_WML: u32 = 0x1;

pub const EASRC_INPUTFIFO_THRESHOLD_MIN: u32 = 0;
pub const EASRC_INPUTFIFO_THRESHOLD_MAX: u32 = 127;
pub const EASRC_OUTPUTFIFO_THRESHOLD_MIN: u32 = 0;
pub const EASRC_OUTPUTFIFO_THRESHOLD_MAX: u32 = 63;

pub const EASRC_DMA_BUFFER_SIZE: u32 = 1024 * 48 * 9;
pub const EASRC_MAX_BUFFER_SIZE: u32 = 1024 * 48;

pub const FIRMWARE_MAGIC: u32 = 0xDEAD;
pub const FIRMWARE_VERSION: u32 = 1;

pub const PREFILTER_MEM_LEN: u32 = 0x1800;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum easrc_word_width {
    EASRC_WIDTH_16_BIT = 0,
    EASRC_WIDTH_20_BIT = 1,
    EASRC_WIDTH_24_BIT = 2,
    EASRC_WIDTH_32_BIT = 3,
}

#[repr(C, packed)]
pub struct asrc_firmware_hdr {
    pub magic: u32,
    pub interp_scen: u32,
    pub prefil_scen: u32,
    pub firmware_version: u32,
}

#[repr(C, packed)]
pub struct interp_params {
    pub magic: u32,
    pub num_taps: u32,
    pub num_phases: u32,
    pub center_tap: u64,
    pub coeff: [u64; 8192],
}

#[repr(C, packed)]
pub struct prefil_params {
    pub magic: u32,
    pub insr: u32,
    pub outsr: u32,
    pub st1_taps: u32,
    pub st2_taps: u32,
    pub st1_exp: u32,
    pub coeff: [u64; 256],
}

#[repr(C)]
pub struct dma_block {
    pub dma_vaddr: *mut c_void,
    pub length: c_uint,
    pub max_buf_size: c_uint,
}

#[repr(C)]
pub struct fsl_easrc_data_fmt {
    pub bitfield: c_uint,
    pub addexp: c_uint,
}

pub const FSL_EASRC_DATA_FMT_WIDTH_SHIFT: u32 = 0;
pub const FSL_EASRC_DATA_FMT_WIDTH_WIDTH: u32 = 2;
pub const FSL_EASRC_DATA_FMT_WIDTH_MASK: c_uint = ((1u32 << FSL_EASRC_DATA_FMT_WIDTH_WIDTH) - 1) as c_uint;
pub const FSL_EASRC_DATA_FMT_ENDIANNESS_SHIFT: u32 = 2;
pub const FSL_EASRC_DATA_FMT_ENDIANNESS_MASK: c_uint = (1u32 << FSL_EASRC_DATA_FMT_ENDIANNESS_SHIFT) as c_uint;
pub const FSL_EASRC_DATA_FMT_UNSIGN_SHIFT: u32 = 3;
pub const FSL_EASRC_DATA_FMT_UNSIGN_MASK: c_uint = (1u32 << FSL_EASRC_DATA_FMT_UNSIGN_SHIFT) as c_uint;
pub const FSL_EASRC_DATA_FMT_FLOATING_POINT_SHIFT: u32 = 4;
pub const FSL_EASRC_DATA_FMT_FLOATING_POINT_MASK: c_uint = (1u32 << FSL_EASRC_DATA_FMT_FLOATING_POINT_SHIFT) as c_uint;
pub const FSL_EASRC_DATA_FMT_IEC958_SHIFT: u32 = 5;
pub const FSL_EASRC_DATA_FMT_IEC958_MASK: c_uint = (1u32 << FSL_EASRC_DATA_FMT_IEC958_SHIFT) as c_uint;
pub const FSL_EASRC_DATA_FMT_SAMPLE_POS_SHIFT: u32 = 6;
pub const FSL_EASRC_DATA_FMT_SAMPLE_POS_WIDTH: u32 = 5;
pub const FSL_EASRC_DATA_FMT_SAMPLE_POS_MASK: c_uint = (((1u32 << FSL_EASRC_DATA_FMT_SAMPLE_POS_WIDTH) - 1) << FSL_EASRC_DATA_FMT_SAMPLE_POS_SHIFT) as c_uint;

impl fsl_easrc_data_fmt {
    pub const fn width(&self) -> c_uint {
        (self.bitfield >> FSL_EASRC_DATA_FMT_WIDTH_SHIFT) & FSL_EASRC_DATA_FMT_WIDTH_MASK
    }

    pub fn set_width(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !FSL_EASRC_DATA_FMT_WIDTH_MASK)
            | ((value << FSL_EASRC_DATA_FMT_WIDTH_SHIFT) & FSL_EASRC_DATA_FMT_WIDTH_MASK);
    }

    pub const fn endianness(&self) -> c_uint {
        (self.bitfield & FSL_EASRC_DATA_FMT_ENDIANNESS_MASK) >> FSL_EASRC_DATA_FMT_ENDIANNESS_SHIFT
    }

    pub fn set_endianness(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !FSL_EASRC_DATA_FMT_ENDIANNESS_MASK)
            | ((value << FSL_EASRC_DATA_FMT_ENDIANNESS_SHIFT) & FSL_EASRC_DATA_FMT_ENDIANNESS_MASK);
    }

    pub const fn unsign(&self) -> c_uint {
        (self.bitfield & FSL_EASRC_DATA_FMT_UNSIGN_MASK) >> FSL_EASRC_DATA_FMT_UNSIGN_SHIFT
    }

    pub fn set_unsign(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !FSL_EASRC_DATA_FMT_UNSIGN_MASK)
            | ((value << FSL_EASRC_DATA_FMT_UNSIGN_SHIFT) & FSL_EASRC_DATA_FMT_UNSIGN_MASK);
    }

    pub const fn floating_point(&self) -> c_uint {
        (self.bitfield & FSL_EASRC_DATA_FMT_FLOATING_POINT_MASK) >> FSL_EASRC_DATA_FMT_FLOATING_POINT_SHIFT
    }

    pub fn set_floating_point(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !FSL_EASRC_DATA_FMT_FLOATING_POINT_MASK)
            | ((value << FSL_EASRC_DATA_FMT_FLOATING_POINT_SHIFT) & FSL_EASRC_DATA_FMT_FLOATING_POINT_MASK);
    }

    pub const fn iec958(&self) -> c_uint {
        (self.bitfield & FSL_EASRC_DATA_FMT_IEC958_MASK) >> FSL_EASRC_DATA_FMT_IEC958_SHIFT
    }

    pub fn set_iec958(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !FSL_EASRC_DATA_FMT_IEC958_MASK)
            | ((value << FSL_EASRC_DATA_FMT_IEC958_SHIFT) & FSL_EASRC_DATA_FMT_IEC958_MASK);
    }

    pub const fn sample_pos(&self) -> c_uint {
        (self.bitfield & FSL_EASRC_DATA_FMT_SAMPLE_POS_MASK) >> FSL_EASRC_DATA_FMT_SAMPLE_POS_SHIFT
    }

    pub fn set_sample_pos(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !FSL_EASRC_DATA_FMT_SAMPLE_POS_MASK)
            | ((value << FSL_EASRC_DATA_FMT_SAMPLE_POS_SHIFT) & FSL_EASRC_DATA_FMT_SAMPLE_POS_MASK);
    }
}

#[repr(C)]
pub struct fsl_easrc_io_params {
    pub fmt: fsl_easrc_data_fmt,
    pub group_len: c_uint,
    pub iterations: c_uint,
    pub access_len: c_uint,
    pub fifo_wtmk: c_uint,
    pub sample_rate: c_uint,
    pub sample_format: snd_pcm_format_t,
    pub norm_rate: c_uint,
}

#[repr(C)]
pub struct fsl_easrc_slot {
    pub busy: bool,
    pub ctx_index: i32,
    pub slot_index: i32,
    pub num_channel: i32,  /* maximum is 8 */
    pub min_channel: i32,
    pub max_channel: i32,
    pub pf_mem_used: i32,
}

/**
 * struct fsl_easrc_ctx_priv - EASRC context private data
 *
 * @in_params: input parameter
 * @out_params:  output parameter
 * @st1_num_taps: tap number of stage 1
 * @st2_num_taps: tap number of stage 2
 * @st1_num_exp: exponent number of stage 1
 * @pf_init_mode: prefilter init mode
 * @rs_init_mode:  resample filter init mode
 * @ctx_streams: stream flag of ctx
 * @rs_ratio: resampler ratio
 * @st1_coeff: pointer of stage 1 coeff
 * @st2_coeff: pointer of stage 2 coeff
 * @in_filled_sample: input filled sample
 * @out_missed_sample: sample missed in output
 * @st1_addexp: exponent added for stage1
 * @st2_addexp: exponent added for stage2
 * @ratio_mod: update ratio
 * @in_filled_len: input filled length
 */
#[repr(C)]
pub struct fsl_easrc_ctx_priv {
    pub in_params: fsl_easrc_io_params,
    pub out_params: fsl_easrc_io_params,
    pub st1_num_taps: c_uint,
    pub st2_num_taps: c_uint,
    pub st1_num_exp: c_uint,
    pub pf_init_mode: c_uint,
    pub rs_init_mode: c_uint,
    pub ctx_streams: c_uint,
    pub rs_ratio: u64,
    pub st1_coeff: *mut u64,
    pub st2_coeff: *mut u64,
    pub in_filled_sample: i32,
    pub out_missed_sample: i32,
    pub st1_addexp: i32,
    pub st2_addexp: i32,
    pub ratio_mod: i32,
    pub in_filled_len: c_uint,
}

/**
 * struct fsl_easrc_priv - EASRC private data
 *
 * @slot: slot setting
 * @firmware_hdr:  the header of firmware
 * @interp: pointer to interpolation filter coeff
 * @prefil: pointer to prefilter coeff
 * @fw: firmware of coeff table
 * @fw_name: firmware name
 * @rs_num_taps:  resample filter taps, 32, 64, or 128
 * @bps_iec958: bits per sample of iec958
 * @rs_coeff: resampler coefficient
 * @const_coeff: one tap prefilter coefficient
 * @firmware_loaded: firmware is loaded
 */
#[repr(C)]
pub struct fsl_easrc_priv {
    pub slot: [[fsl_easrc_slot; 2]; EASRC_CTX_MAX_NUM],
    pub firmware_hdr: *mut asrc_firmware_hdr,
    pub interp: *mut interp_params,
    pub prefil: *mut prefil_params,
    pub fw: *const firmware,
    pub fw_name: *const c_char,
    pub rs_num_taps: c_uint,
    pub bps_iec958: [c_uint; EASRC_CTX_MAX_NUM],
    pub rs_coeff: *mut u64,
    pub const_coeff: u64,
    pub firmware_loaded: i32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
