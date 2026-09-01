/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fsl_asrc.h - Freescale ASRC ALSA SoC header file
 *
 * Copyright (C) 2014 Freescale Semiconductor, Inc.
 *
 * Author: Nicolin Chen <nicoleotsuka@gmail.com>
 */

/* Depends on declarations from "fsl_asrc_common.h". */

pub const ASRC_M2M_INPUTFIFO_WML: u32 = 0x4;
pub const ASRC_M2M_OUTPUTFIFO_WML: u32 = 0x2;
pub const ASRC_DMA_BUFFER_NUM: u32 = 2;
pub const ASRC_INPUTFIFO_THRESHOLD: u32 = 32;
pub const ASRC_OUTPUTFIFO_THRESHOLD: u32 = 32;
pub const ASRC_FIFO_THRESHOLD_MIN: u32 = 0;
pub const ASRC_FIFO_THRESHOLD_MAX: u32 = 63;
pub const ASRC_DMA_BUFFER_SIZE: u32 = 1024 * 48 * 4;
pub const ASRC_MAX_BUFFER_SIZE: u32 = 1024 * 48;
pub const ASRC_OUTPUT_LAST_SAMPLE: u32 = 8;

pub const IDEAL_RATIO_RATE: u32 = 1000000;

pub const REG_ASRCTR: u32 = 0x00;
pub const REG_ASRIER: u32 = 0x04;
pub const REG_ASRCNCR: u32 = 0x0C;
pub const REG_ASRCFG: u32 = 0x10;
pub const REG_ASRCSR: u32 = 0x14;

pub const REG_ASRCDR1: u32 = 0x18;
pub const REG_ASRCDR2: u32 = 0x1C;
pub const fn REG_ASRCDR(i: u32) -> u32 {
    if i < 2 { REG_ASRCDR1 } else { REG_ASRCDR2 }
}

pub const REG_ASRSTR: u32 = 0x20;
pub const REG_ASRRA: u32 = 0x24;
pub const REG_ASRRB: u32 = 0x28;
pub const REG_ASRRC: u32 = 0x2C;
pub const REG_ASRPM1: u32 = 0x40;
pub const REG_ASRPM2: u32 = 0x44;
pub const REG_ASRPM3: u32 = 0x48;
pub const REG_ASRPM4: u32 = 0x4C;
pub const REG_ASRPM5: u32 = 0x50;
pub const REG_ASRTFR1: u32 = 0x54;
pub const REG_ASRCCR: u32 = 0x5C;

pub const REG_ASRDIA: u32 = 0x60;
pub const REG_ASRDOA: u32 = 0x64;
pub const REG_ASRDIB: u32 = 0x68;
pub const REG_ASRDOB: u32 = 0x6C;
pub const REG_ASRDIC: u32 = 0x70;
pub const REG_ASRDOC: u32 = 0x74;
pub const fn REG_ASRDI(i: u32) -> u32 {
    REG_ASRDIA + (i << 3)
}
pub const fn REG_ASRDO(i: u32) -> u32 {
    REG_ASRDOA + (i << 3)
}
pub fn REG_ASRDx(x: u32, i: u32) -> u32 {
    if x == IN { REG_ASRDI(i) } else { REG_ASRDO(i) }
}

pub const REG_ASRIDRHA: u32 = 0x80;
pub const REG_ASRIDRLA: u32 = 0x84;
pub const REG_ASRIDRHB: u32 = 0x88;
pub const REG_ASRIDRLB: u32 = 0x8C;
pub const REG_ASRIDRHC: u32 = 0x90;
pub const REG_ASRIDRLC: u32 = 0x94;
pub const fn REG_ASRIDRH(i: u32) -> u32 {
    REG_ASRIDRHA + (i << 3)
}
pub const fn REG_ASRIDRL(i: u32) -> u32 {
    REG_ASRIDRLA + (i << 3)
}

pub const REG_ASR76K: u32 = 0x98;
pub const REG_ASR56K: u32 = 0x9C;

pub const REG_ASRMCRA: u32 = 0xA0;
pub const REG_ASRFSTA: u32 = 0xA4;
pub const REG_ASRMCRB: u32 = 0xA8;
pub const REG_ASRFSTB: u32 = 0xAC;
pub const REG_ASRMCRC: u32 = 0xB0;
pub const REG_ASRFSTC: u32 = 0xB4;
pub const fn REG_ASRMCR(i: u32) -> u32 {
    REG_ASRMCRA + (i << 3)
}
pub const fn REG_ASRFST(i: u32) -> u32 {
    REG_ASRFSTA + (i << 3)
}

pub const REG_ASRMCR1A: u32 = 0xC0;
pub const REG_ASRMCR1B: u32 = 0xC4;
pub const REG_ASRMCR1C: u32 = 0xC8;
pub const fn REG_ASRMCR1(i: u32) -> u32 {
    REG_ASRMCR1A + (i << 2)
}

/* REG0 0x00 REG_ASRCTR */
pub const fn ASRCTR_ATSi_SHIFT(i: u32) -> u32 { 20 + i }
pub const fn ASRCTR_ATSi_MASK(i: u32) -> u32 { 1 << ASRCTR_ATSi_SHIFT(i) }
pub const fn ASRCTR_ATS(i: u32) -> u32 { 1 << ASRCTR_ATSi_SHIFT(i) }
pub const fn ASRCTR_USRi_SHIFT(i: u32) -> u32 { 14 + (i << 1) }
pub const fn ASRCTR_USRi_MASK(i: u32) -> u32 { 1 << ASRCTR_USRi_SHIFT(i) }
pub const fn ASRCTR_USR(i: u32) -> u32 { 1 << ASRCTR_USRi_SHIFT(i) }
pub const fn ASRCTR_IDRi_SHIFT(i: u32) -> u32 { 13 + (i << 1) }
pub const fn ASRCTR_IDRi_MASK(i: u32) -> u32 { 1 << ASRCTR_IDRi_SHIFT(i) }
pub const fn ASRCTR_IDR(i: u32) -> u32 { 1 << ASRCTR_IDRi_SHIFT(i) }
pub const ASRCTR_SRST_SHIFT: u32 = 4;
pub const ASRCTR_SRST_MASK: u32 = 1 << ASRCTR_SRST_SHIFT;
pub const ASRCTR_SRST: u32 = 1 << ASRCTR_SRST_SHIFT;
pub const fn ASRCTR_ASRCEi_SHIFT(i: u32) -> u32 { 1 + i }
pub const fn ASRCTR_ASRCEi_MASK(i: u32) -> u32 { 1 << ASRCTR_ASRCEi_SHIFT(i) }
pub const fn ASRCTR_ASRCE(i: u32) -> u32 { 1 << ASRCTR_ASRCEi_SHIFT(i) }
pub const ASRCTR_ASRCEi_ALL_MASK: u32 = 0x7 << ASRCTR_ASRCEi_SHIFT(0);
pub const ASRCTR_ASRCEN_SHIFT: u32 = 0;
pub const ASRCTR_ASRCEN_MASK: u32 = 1 << ASRCTR_ASRCEN_SHIFT;
pub const ASRCTR_ASRCEN: u32 = 1 << ASRCTR_ASRCEN_SHIFT;

/* REG1 0x04 REG_ASRIER */
pub const ASRIER_AFPWE_SHIFT: u32 = 7;
pub const ASRIER_AFPWE_MASK: u32 = 1 << ASRIER_AFPWE_SHIFT;
pub const ASRIER_AFPWE: u32 = 1 << ASRIER_AFPWE_SHIFT;
pub const ASRIER_AOLIE_SHIFT: u32 = 6;
pub const ASRIER_AOLIE_MASK: u32 = 1 << ASRIER_AOLIE_SHIFT;
pub const ASRIER_AOLIE: u32 = 1 << ASRIER_AOLIE_SHIFT;
pub const fn ASRIER_ADOEi_SHIFT(i: u32) -> u32 { 3 + i }
pub const fn ASRIER_ADOEi_MASK(i: u32) -> u32 { 1 << ASRIER_ADOEi_SHIFT(i) }
pub const fn ASRIER_ADOE(i: u32) -> u32 { 1 << ASRIER_ADOEi_SHIFT(i) }
pub const fn ASRIER_ADIEi_SHIFT(i: u32) -> u32 { 0 + i }
pub const fn ASRIER_ADIEi_MASK(i: u32) -> u32 { 1 << ASRIER_ADIEi_SHIFT(i) }
pub const fn ASRIER_ADIE(i: u32) -> u32 { 1 << ASRIER_ADIEi_SHIFT(i) }

/* REG2 0x0C REG_ASRCNCR */
pub const fn ASRCNCR_ANCi_SHIFT(i: u32, b: u32) -> u32 { b * i }
pub const fn ASRCNCR_ANCi_MASK(i: u32, b: u32) -> u32 {
    ((1 << b) - 1) << ASRCNCR_ANCi_SHIFT(i, b)
}
pub const fn ASRCNCR_ANCi(i: u32, v: u32, b: u32) -> u32 {
    (v << ASRCNCR_ANCi_SHIFT(i, b)) & ASRCNCR_ANCi_MASK(i, b)
}

/* REG3 0x10 REG_ASRCFG */
pub const fn ASRCFG_INIRQi_SHIFT(i: u32) -> u32 { 21 + i }
pub const fn ASRCFG_INIRQi_MASK(i: u32) -> u32 { 1 << ASRCFG_INIRQi_SHIFT(i) }
pub const fn ASRCFG_INIRQi(i: u32) -> u32 { 1 << ASRCFG_INIRQi_SHIFT(i) }
pub const fn ASRCFG_NDPRi_SHIFT(i: u32) -> u32 { 18 + i }
pub const fn ASRCFG_NDPRi_MASK(i: u32) -> u32 { 1 << ASRCFG_NDPRi_SHIFT(i) }
pub const ASRCFG_NDPRi_ALL_SHIFT: u32 = 18;
pub const ASRCFG_NDPRi_ALL_MASK: u32 = 7 << ASRCFG_NDPRi_ALL_SHIFT;
pub const fn ASRCFG_NDPRi(i: u32) -> u32 { 1 << ASRCFG_NDPRi_SHIFT(i) }
pub const fn ASRCFG_POSTMODi_SHIFT(i: u32) -> u32 { 8 + (i << 2) }
pub const ASRCFG_POSTMODi_WIDTH: u32 = 2;
pub const fn ASRCFG_POSTMODi_MASK(i: u32) -> u32 {
    ((1 << ASRCFG_POSTMODi_WIDTH) - 1) << ASRCFG_POSTMODi_SHIFT(i)
}
pub const ASRCFG_POSTMODi_ALL_MASK: u32 = ASRCFG_POSTMODi_MASK(0) | ASRCFG_POSTMODi_MASK(1) | ASRCFG_POSTMODi_MASK(2);
pub const fn ASRCFG_POSTMOD(i: u32, v: u32) -> u32 { v << ASRCFG_POSTMODi_SHIFT(i) }
pub const fn ASRCFG_POSTMODi_UP(i: u32) -> u32 { 0 << ASRCFG_POSTMODi_SHIFT(i) }
pub const fn ASRCFG_POSTMODi_DCON(i: u32) -> u32 { 1 << ASRCFG_POSTMODi_SHIFT(i) }
pub const fn ASRCFG_POSTMODi_DOWN(i: u32) -> u32 { 2 << ASRCFG_POSTMODi_SHIFT(i) }
pub const fn ASRCFG_PREMODi_SHIFT(i: u32) -> u32 { 6 + (i << 2) }
pub const ASRCFG_PREMODi_WIDTH: u32 = 2;
pub const fn ASRCFG_PREMODi_MASK(i: u32) -> u32 {
    ((1 << ASRCFG_PREMODi_WIDTH) - 1) << ASRCFG_PREMODi_SHIFT(i)
}
pub const ASRCFG_PREMODi_ALL_MASK: u32 = ASRCFG_PREMODi_MASK(0) | ASRCFG_PREMODi_MASK(1) | ASRCFG_PREMODi_MASK(2);
pub const fn ASRCFG_PREMOD(i: u32, v: u32) -> u32 { v << ASRCFG_PREMODi_SHIFT(i) }
pub const fn ASRCFG_PREMODi_UP(i: u32) -> u32 { 0 << ASRCFG_PREMODi_SHIFT(i) }
pub const fn ASRCFG_PREMODi_DCON(i: u32) -> u32 { 1 << ASRCFG_PREMODi_SHIFT(i) }
pub const fn ASRCFG_PREMODi_DOWN(i: u32) -> u32 { 2 << ASRCFG_PREMODi_SHIFT(i) }
pub const fn ASRCFG_PREMODi_BYPASS(i: u32) -> u32 { 3 << ASRCFG_PREMODi_SHIFT(i) }

/* REG4 0x14 REG_ASRCSR */
pub const ASRCSR_AxCSi_WIDTH: u32 = 4;
pub const ASRCSR_AxCSi_MASK: u32 = (1 << ASRCSR_AxCSi_WIDTH) - 1;
pub const fn ASRCSR_AOCSi_SHIFT(i: u32) -> u32 { 12 + (i << 2) }
pub const fn ASRCSR_AOCSi_MASK(i: u32) -> u32 {
    ((1 << ASRCSR_AxCSi_WIDTH) - 1) << ASRCSR_AOCSi_SHIFT(i)
}
pub const fn ASRCSR_AOCS(i: u32, v: u32) -> u32 { v << ASRCSR_AOCSi_SHIFT(i) }
pub const fn ASRCSR_AICSi_SHIFT(i: u32) -> u32 { i << 2 }
pub const fn ASRCSR_AICSi_MASK(i: u32) -> u32 {
    ((1 << ASRCSR_AxCSi_WIDTH) - 1) << ASRCSR_AICSi_SHIFT(i)
}
pub const fn ASRCSR_AICS(i: u32, v: u32) -> u32 { v << ASRCSR_AICSi_SHIFT(i) }

/* REG5&6 0x18 & 0x1C REG_ASRCDR1 & ASRCDR2 */
pub const ASRCDRi_AxCPi_WIDTH: u32 = 3;
pub const fn ASRCDRi_AICPi_SHIFT(i: u32) -> u32 { 0 + (i % 2) * 6 }
pub const fn ASRCDRi_AICPi_MASK(i: u32) -> u32 {
    ((1 << ASRCDRi_AxCPi_WIDTH) - 1) << ASRCDRi_AICPi_SHIFT(i)
}
pub const fn ASRCDRi_AICP(i: u32, v: u32) -> u32 { v << ASRCDRi_AICPi_SHIFT(i) }
pub const fn ASRCDRi_AICDi_SHIFT(i: u32) -> u32 { 3 + (i % 2) * 6 }
pub const fn ASRCDRi_AICDi_MASK(i: u32) -> u32 {
    ((1 << ASRCDRi_AxCPi_WIDTH) - 1) << ASRCDRi_AICDi_SHIFT(i)
}
pub const fn ASRCDRi_AICD(i: u32, v: u32) -> u32 { v << ASRCDRi_AICDi_SHIFT(i) }
pub const fn ASRCDRi_AOCPi_SHIFT(i: u32) -> u32 {
    if i < 2 { 12 + i * 6 } else { 6 }
}
pub const fn ASRCDRi_AOCPi_MASK(i: u32) -> u32 {
    ((1 << ASRCDRi_AxCPi_WIDTH) - 1) << ASRCDRi_AOCPi_SHIFT(i)
}
pub const fn ASRCDRi_AOCP(i: u32, v: u32) -> u32 { v << ASRCDRi_AOCPi_SHIFT(i) }
pub const fn ASRCDRi_AOCDi_SHIFT(i: u32) -> u32 {
    if i < 2 { 15 + i * 6 } else { 9 }
}
pub const fn ASRCDRi_AOCDi_MASK(i: u32) -> u32 {
    ((1 << ASRCDRi_AxCPi_WIDTH) - 1) << ASRCDRi_AOCDi_SHIFT(i)
}
pub const fn ASRCDRi_AOCD(i: u32, v: u32) -> u32 { v << ASRCDRi_AOCDi_SHIFT(i) }

/* REG7 0x20 REG_ASRSTR */
pub const ASRSTR_DSLCNT_SHIFT: u32 = 21;
pub const ASRSTR_DSLCNT_MASK: u32 = 1 << ASRSTR_DSLCNT_SHIFT;
pub const ASRSTR_DSLCNT: u32 = 1 << ASRSTR_DSLCNT_SHIFT;
pub const ASRSTR_ATQOL_SHIFT: u32 = 20;
pub const ASRSTR_ATQOL_MASK: u32 = 1 << ASRSTR_ATQOL_SHIFT;
pub const ASRSTR_ATQOL: u32 = 1 << ASRSTR_ATQOL_SHIFT;
pub const fn ASRSTR_AOOLi_SHIFT(i: u32) -> u32 { 17 + i }
pub const fn ASRSTR_AOOLi_MASK(i: u32) -> u32 { 1 << ASRSTR_AOOLi_SHIFT(i) }
pub const fn ASRSTR_AOOL(i: u32) -> u32 { 1 << ASRSTR_AOOLi_SHIFT(i) }
pub const fn ASRSTR_AIOLi_SHIFT(i: u32) -> u32 { 14 + i }
pub const fn ASRSTR_AIOLi_MASK(i: u32) -> u32 { 1 << ASRSTR_AIOLi_SHIFT(i) }
pub const fn ASRSTR_AIOL(i: u32) -> u32 { 1 << ASRSTR_AIOLi_SHIFT(i) }
pub const fn ASRSTR_AODOi_SHIFT(i: u32) -> u32 { 11 + i }
pub const fn ASRSTR_AODOi_MASK(i: u32) -> u32 { 1 << ASRSTR_AODOi_SHIFT(i) }
pub const fn ASRSTR_AODO(i: u32) -> u32 { 1 << ASRSTR_AODOi_SHIFT(i) }
pub const fn ASRSTR_AIDUi_SHIFT(i: u32) -> u32 { 8 + i }
pub const fn ASRSTR_AIDUi_MASK(i: u32) -> u32 { 1 << ASRSTR_AIDUi_SHIFT(i) }
pub const fn ASRSTR_AIDU(i: u32) -> u32 { 1 << ASRSTR_AIDUi_SHIFT(i) }
pub const ASRSTR_FPWT_SHIFT: u32 = 7;
pub const ASRSTR_FPWT_MASK: u32 = 1 << ASRSTR_FPWT_SHIFT;
pub const ASRSTR_FPWT: u32 = 1 << ASRSTR_FPWT_SHIFT;
pub const ASRSTR_AOLE_SHIFT: u32 = 6;
pub const ASRSTR_AOLE_MASK: u32 = 1 << ASRSTR_AOLE_SHIFT;
pub const ASRSTR_AOLE: u32 = 1 << ASRSTR_AOLE_SHIFT;
pub const fn ASRSTR_AODEi_SHIFT(i: u32) -> u32 { 3 + i }
pub const fn ASRSTR_AODFi_MASK(i: u32) -> u32 { 1 << ASRSTR_AODEi_SHIFT(i) }
pub const fn ASRSTR_AODF(i: u32) -> u32 { 1 << ASRSTR_AODEi_SHIFT(i) }
pub const fn ASRSTR_AIDEi_SHIFT(i: u32) -> u32 { 0 + i }
pub const fn ASRSTR_AIDEi_MASK(i: u32) -> u32 { 1 << ASRSTR_AIDEi_SHIFT(i) }
pub const fn ASRSTR_AIDE(i: u32) -> u32 { 1 << ASRSTR_AIDEi_SHIFT(i) }

/* REG10 0x54 REG_ASRTFR1 */
pub const ASRTFR1_TF_BASE_WIDTH: u32 = 7;
pub const ASRTFR1_TF_BASE_SHIFT: u32 = 6;
pub const ASRTFR1_TF_BASE_MASK: u32 = ((1 << ASRTFR1_TF_BASE_WIDTH) - 1) << ASRTFR1_TF_BASE_SHIFT;
pub const fn ASRTFR1_TF_BASE(i: u32) -> u32 { i << ASRTFR1_TF_BASE_SHIFT }

/*
 * REG22 0xA0 REG_ASRMCRA
 * REG24 0xA8 REG_ASRMCRB
 * REG26 0xB0 REG_ASRMCRC
 */
pub const ASRMCRi_ZEROBUFi_SHIFT: u32 = 23;
pub const ASRMCRi_ZEROBUFi_MASK: u32 = 1 << ASRMCRi_ZEROBUFi_SHIFT;
pub const ASRMCRi_ZEROBUFi: u32 = 1 << ASRMCRi_ZEROBUFi_SHIFT;
pub const ASRMCRi_EXTTHRSHi_SHIFT: u32 = 22;
pub const ASRMCRi_EXTTHRSHi_MASK: u32 = 1 << ASRMCRi_EXTTHRSHi_SHIFT;
pub const ASRMCRi_EXTTHRSHi: u32 = 1 << ASRMCRi_EXTTHRSHi_SHIFT;
pub const ASRMCRi_BUFSTALLi_SHIFT: u32 = 21;
pub const ASRMCRi_BUFSTALLi_MASK: u32 = 1 << ASRMCRi_BUFSTALLi_SHIFT;
pub const ASRMCRi_BUFSTALLi: u32 = 1 << ASRMCRi_BUFSTALLi_SHIFT;
pub const ASRMCRi_BYPASSPOLYi_SHIFT: u32 = 20;
pub const ASRMCRi_BYPASSPOLYi_MASK: u32 = 1 << ASRMCRi_BYPASSPOLYi_SHIFT;
pub const ASRMCRi_BYPASSPOLYi: u32 = 1 << ASRMCRi_BYPASSPOLYi_SHIFT;
pub const ASRMCRi_OUTFIFO_THRESHOLD_WIDTH: u32 = 6;
pub const ASRMCRi_OUTFIFO_THRESHOLD_SHIFT: u32 = 12;
pub const ASRMCRi_OUTFIFO_THRESHOLD_MASK: u32 =
    ((1 << ASRMCRi_OUTFIFO_THRESHOLD_WIDTH) - 1) << ASRMCRi_OUTFIFO_THRESHOLD_SHIFT;
pub const fn ASRMCRi_OUTFIFO_THRESHOLD(v: u32) -> u32 {
    (v << ASRMCRi_OUTFIFO_THRESHOLD_SHIFT) & ASRMCRi_OUTFIFO_THRESHOLD_MASK
}
pub const ASRMCRi_RSYNIFi_SHIFT: u32 = 11;
pub const ASRMCRi_RSYNIFi_MASK: u32 = 1 << ASRMCRi_RSYNIFi_SHIFT;
pub const ASRMCRi_RSYNIFi: u32 = 1 << ASRMCRi_RSYNIFi_SHIFT;
pub const ASRMCRi_RSYNOFi_SHIFT: u32 = 10;
pub const ASRMCRi_RSYNOFi_MASK: u32 = 1 << ASRMCRi_RSYNOFi_SHIFT;
pub const ASRMCRi_RSYNOFi: u32 = 1 << ASRMCRi_RSYNOFi_SHIFT;
pub const ASRMCRi_INFIFO_THRESHOLD_WIDTH: u32 = 6;
pub const ASRMCRi_INFIFO_THRESHOLD_SHIFT: u32 = 0;
pub const ASRMCRi_INFIFO_THRESHOLD_MASK: u32 =
    ((1 << ASRMCRi_INFIFO_THRESHOLD_WIDTH) - 1) << ASRMCRi_INFIFO_THRESHOLD_SHIFT;
pub const fn ASRMCRi_INFIFO_THRESHOLD(v: u32) -> u32 {
    (v << ASRMCRi_INFIFO_THRESHOLD_SHIFT) & ASRMCRi_INFIFO_THRESHOLD_MASK
}

/*
 * REG23 0xA4 REG_ASRFSTA
 * REG25 0xAC REG_ASRFSTB
 * REG27 0xB4 REG_ASRFSTC
 */
pub const ASRFSTi_OAFi_SHIFT: u32 = 23;
pub const ASRFSTi_OAFi_MASK: u32 = 1 << ASRFSTi_OAFi_SHIFT;
pub const ASRFSTi_OAFi: u32 = 1 << ASRFSTi_OAFi_SHIFT;
pub const ASRFSTi_OUTPUT_FIFO_WIDTH: u32 = 7;
pub const ASRFSTi_OUTPUT_FIFO_SHIFT: u32 = 12;
pub const ASRFSTi_OUTPUT_FIFO_MASK: u32 =
    ((1 << ASRFSTi_OUTPUT_FIFO_WIDTH) - 1) << ASRFSTi_OUTPUT_FIFO_SHIFT;
pub const fn ASRFSTi_OUTPUT_FIFO_FILL(v: u32) -> u32 {
    (v & ASRFSTi_OUTPUT_FIFO_MASK) >> ASRFSTi_OUTPUT_FIFO_SHIFT
}
pub const ASRFSTi_IAEi_SHIFT: u32 = 11;
pub const ASRFSTi_IAEi_MASK: u32 = 1 << ASRFSTi_IAEi_SHIFT;
pub const ASRFSTi_IAEi: u32 = 1 << ASRFSTi_IAEi_SHIFT;
pub const ASRFSTi_INPUT_FIFO_WIDTH: u32 = 7;
pub const ASRFSTi_INPUT_FIFO_SHIFT: u32 = 0;
pub const ASRFSTi_INPUT_FIFO_MASK: u32 = (1 << ASRFSTi_INPUT_FIFO_WIDTH) - 1;

/* REG28 0xC0 & 0xC4 & 0xC8 REG_ASRMCR1i */
pub const ASRMCR1i_IWD_WIDTH: u32 = 3;
pub const ASRMCR1i_IWD_SHIFT: u32 = 9;
pub const ASRMCR1i_IWD_MASK: u32 = ((1 << ASRMCR1i_IWD_WIDTH) - 1) << ASRMCR1i_IWD_SHIFT;
pub const fn ASRMCR1i_IWD(v: u32) -> u32 { v << ASRMCR1i_IWD_SHIFT }
pub const ASRMCR1i_IMSB_SHIFT: u32 = 8;
pub const ASRMCR1i_IMSB_MASK: u32 = 1 << ASRMCR1i_IMSB_SHIFT;
pub const ASRMCR1i_IMSB_MSB: u32 = 1 << ASRMCR1i_IMSB_SHIFT;
pub const ASRMCR1i_IMSB_LSB: u32 = 0 << ASRMCR1i_IMSB_SHIFT;
pub const ASRMCR1i_OMSB_SHIFT: u32 = 2;
pub const ASRMCR1i_OMSB_MASK: u32 = 1 << ASRMCR1i_OMSB_SHIFT;
pub const ASRMCR1i_OMSB_MSB: u32 = 1 << ASRMCR1i_OMSB_SHIFT;
pub const ASRMCR1i_OMSB_LSB: u32 = 0 << ASRMCR1i_OMSB_SHIFT;
pub const ASRMCR1i_OSGN_SHIFT: u32 = 1;
pub const ASRMCR1i_OSGN_MASK: u32 = 1 << ASRMCR1i_OSGN_SHIFT;
pub const ASRMCR1i_OSGN: u32 = 1 << ASRMCR1i_OSGN_SHIFT;
pub const ASRMCR1i_OW16_SHIFT: u32 = 0;
pub const ASRMCR1i_OW16_MASK: u32 = 1 << ASRMCR1i_OW16_SHIFT;
pub const fn ASRMCR1i_OW16(v: u32) -> u32 { v << ASRMCR1i_OW16_SHIFT }

pub const ASRC_PAIR_MAX_NUM: u32 = ASRC_PAIR_C + 1;

pub type asrc_inclk = u32;
pub const INCLK_NONE: asrc_inclk = 0x03;
pub const INCLK_ESAI_RX: asrc_inclk = 0x00;
pub const INCLK_SSI1_RX: asrc_inclk = 0x01;
pub const INCLK_SSI2_RX: asrc_inclk = 0x02;
pub const INCLK_SSI3_RX: asrc_inclk = 0x07;
pub const INCLK_SPDIF_RX: asrc_inclk = 0x04;
pub const INCLK_MLB_CLK: asrc_inclk = 0x05;
pub const INCLK_PAD: asrc_inclk = 0x06;
pub const INCLK_ESAI_TX: asrc_inclk = 0x08;
pub const INCLK_SSI1_TX: asrc_inclk = 0x09;
pub const INCLK_SSI2_TX: asrc_inclk = 0x0a;
pub const INCLK_SSI3_TX: asrc_inclk = 0x0b;
pub const INCLK_SPDIF_TX: asrc_inclk = 0x0c;
pub const INCLK_ASRCK1_CLK: asrc_inclk = 0x0f;

/* clocks for imx8 */
pub const INCLK_AUD_PLL_DIV_CLK0: asrc_inclk = 0x10;
pub const INCLK_AUD_PLL_DIV_CLK1: asrc_inclk = 0x11;
pub const INCLK_AUD_CLK0: asrc_inclk = 0x12;
pub const INCLK_AUD_CLK1: asrc_inclk = 0x13;
pub const INCLK_ESAI0_RX_CLK: asrc_inclk = 0x14;
pub const INCLK_ESAI0_TX_CLK: asrc_inclk = 0x15;
pub const INCLK_SPDIF0_RX: asrc_inclk = 0x16;
pub const INCLK_SPDIF1_RX: asrc_inclk = 0x17;
pub const INCLK_SAI0_RX_BCLK: asrc_inclk = 0x18;
pub const INCLK_SAI0_TX_BCLK: asrc_inclk = 0x19;
pub const INCLK_SAI1_RX_BCLK: asrc_inclk = 0x1a;
pub const INCLK_SAI1_TX_BCLK: asrc_inclk = 0x1b;
pub const INCLK_SAI2_RX_BCLK: asrc_inclk = 0x1c;
pub const INCLK_SAI3_RX_BCLK: asrc_inclk = 0x1d;
pub const INCLK_ASRC0_MUX_CLK: asrc_inclk = 0x1e;

pub const INCLK_ESAI1_RX_CLK: asrc_inclk = 0x20;
pub const INCLK_ESAI1_TX_CLK: asrc_inclk = 0x21;
pub const INCLK_SAI6_TX_BCLK: asrc_inclk = 0x22;
pub const INCLK_HDMI_RX_SAI0_RX_BCLK: asrc_inclk = 0x24;
pub const INCLK_HDMI_TX_SAI0_TX_BCLK: asrc_inclk = 0x25;

pub const INCLK_SAI2_TX_BCLK: asrc_inclk = 0x26;
pub const INCLK_SAI3_TX_BCLK: asrc_inclk = 0x27;
pub const INCLK_SAI4_RX_BCLK: asrc_inclk = 0x28;
pub const INCLK_SAI4_TX_BCLK: asrc_inclk = 0x29;
pub const INCLK_SAI5_RX_BCLK: asrc_inclk = 0x2a;
pub const INCLK_SAI5_TX_BCLK: asrc_inclk = 0x2b;

pub type asrc_outclk = u32;
pub const OUTCLK_NONE: asrc_outclk = 0x03;
pub const OUTCLK_ESAI_TX: asrc_outclk = 0x00;
pub const OUTCLK_SSI1_TX: asrc_outclk = 0x01;
pub const OUTCLK_SSI2_TX: asrc_outclk = 0x02;
pub const OUTCLK_SSI3_TX: asrc_outclk = 0x07;
pub const OUTCLK_SPDIF_TX: asrc_outclk = 0x04;
pub const OUTCLK_MLB_CLK: asrc_outclk = 0x05;
pub const OUTCLK_PAD: asrc_outclk = 0x06;
pub const OUTCLK_ESAI_RX: asrc_outclk = 0x08;
pub const OUTCLK_SSI1_RX: asrc_outclk = 0x09;
pub const OUTCLK_SSI2_RX: asrc_outclk = 0x0a;
pub const OUTCLK_SSI3_RX: asrc_outclk = 0x0b;
pub const OUTCLK_SPDIF_RX: asrc_outclk = 0x0c;
pub const OUTCLK_ASRCK1_CLK: asrc_outclk = 0x0f;

/* clocks for imx8 */
pub const OUTCLK_AUD_PLL_DIV_CLK0: asrc_outclk = 0x10;
pub const OUTCLK_AUD_PLL_DIV_CLK1: asrc_outclk = 0x11;
pub const OUTCLK_AUD_CLK0: asrc_outclk = 0x12;
pub const OUTCLK_AUD_CLK1: asrc_outclk = 0x13;
pub const OUTCLK_ESAI0_RX_CLK: asrc_outclk = 0x14;
pub const OUTCLK_ESAI0_TX_CLK: asrc_outclk = 0x15;
pub const OUTCLK_SPDIF0_RX: asrc_outclk = 0x16;
pub const OUTCLK_SPDIF1_RX: asrc_outclk = 0x17;
pub const OUTCLK_SAI0_RX_BCLK: asrc_outclk = 0x18;
pub const OUTCLK_SAI0_TX_BCLK: asrc_outclk = 0x19;
pub const OUTCLK_SAI1_RX_BCLK: asrc_outclk = 0x1a;
pub const OUTCLK_SAI1_TX_BCLK: asrc_outclk = 0x1b;
pub const OUTCLK_SAI2_RX_BCLK: asrc_outclk = 0x1c;
pub const OUTCLK_SAI3_RX_BCLK: asrc_outclk = 0x1d;
pub const OUTCLK_ASRCO_MUX_CLK: asrc_outclk = 0x1e;

pub const OUTCLK_ESAI1_RX_CLK: asrc_outclk = 0x20;
pub const OUTCLK_ESAI1_TX_CLK: asrc_outclk = 0x21;
pub const OUTCLK_SAI6_TX_BCLK: asrc_outclk = 0x22;
pub const OUTCLK_HDMI_RX_SAI0_RX_BCLK: asrc_outclk = 0x24;
pub const OUTCLK_HDMI_TX_SAI0_TX_BCLK: asrc_outclk = 0x25;

pub const OUTCLK_SAI2_TX_BCLK: asrc_outclk = 0x26;
pub const OUTCLK_SAI3_TX_BCLK: asrc_outclk = 0x27;
pub const OUTCLK_SAI4_RX_BCLK: asrc_outclk = 0x28;
pub const OUTCLK_SAI4_TX_BCLK: asrc_outclk = 0x29;
pub const OUTCLK_SAI5_RX_BCLK: asrc_outclk = 0x2a;
pub const OUTCLK_SAI5_TX_BCLK: asrc_outclk = 0x2b;

pub const ASRC_CLK_MAX_NUM: usize = 16;
pub const ASRC_CLK_MAP_LEN: u32 = 0x30;

pub type asrc_word_width = u32;
pub const ASRC_WIDTH_24_BIT: asrc_word_width = 0;
pub const ASRC_WIDTH_16_BIT: asrc_word_width = 1;
pub const ASRC_WIDTH_8_BIT: asrc_word_width = 2;

#[repr(C)]
pub struct asrc_config {
    pub pair: asrc_pair_index,
    pub channel_num: ::core::ffi::c_uint,
    pub buffer_num: ::core::ffi::c_uint,
    pub dma_buffer_size: ::core::ffi::c_uint,
    pub input_sample_rate: ::core::ffi::c_uint,
    pub output_sample_rate: ::core::ffi::c_uint,
    pub input_format: snd_pcm_format_t,
    pub output_format: snd_pcm_format_t,
    pub inclk: asrc_inclk,
    pub outclk: asrc_outclk,
}

#[repr(C)]
pub struct asrc_req {
    pub chn_num: ::core::ffi::c_uint,
    pub index: asrc_pair_index,
}

#[repr(C)]
pub struct asrc_querybuf {
    pub buffer_index: ::core::ffi::c_uint,
    pub input_length: ::core::ffi::c_uint,
    pub output_length: ::core::ffi::c_uint,
    pub input_offset: ::core::ffi::c_ulong,
    pub output_offset: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct asrc_convert_buffer {
    pub input_buffer_vaddr: *mut ::core::ffi::c_void,
    pub output_buffer_vaddr: *mut ::core::ffi::c_void,
    pub input_buffer_length: ::core::ffi::c_uint,
    pub output_buffer_length: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct asrc_status_flags {
    pub index: asrc_pair_index,
    pub overload_error: ::core::ffi::c_uint,
}

pub type asrc_error_status = u32;
pub const ASRC_TASK_Q_OVERLOAD: asrc_error_status = 0x01;
pub const ASRC_OUTPUT_TASK_OVERLOAD: asrc_error_status = 0x02;
pub const ASRC_INPUT_TASK_OVERLOAD: asrc_error_status = 0x04;
pub const ASRC_OUTPUT_BUFFER_OVERFLOW: asrc_error_status = 0x08;
pub const ASRC_INPUT_BUFFER_UNDERRUN: asrc_error_status = 0x10;

#[repr(C)]
pub struct dma_block {
    pub dma_paddr: dma_addr_t,
    pub dma_vaddr: *mut ::core::ffi::c_void,
    pub length: ::core::ffi::c_uint,
}

/**
 * struct fsl_asrc_soc_data - soc specific data
 *
 * @use_edma: using edma as dma device or not
 * @channel_bits: width of ASRCNCR register for each pair
 * @start_before_dma: start asrc before dma
 */
#[repr(C)]
pub struct fsl_asrc_soc_data {
    pub use_edma: bool,
    pub channel_bits: ::core::ffi::c_uint,
    pub start_before_dma: bool,
}

/**
 * struct fsl_asrc_pair_priv - ASRC Pair private data
 *
 * @config: configuration profile
 */
#[repr(C)]
pub struct fsl_asrc_pair_priv {
    pub config: *mut asrc_config,
}

/**
 * struct fsl_asrc_priv - ASRC private data
 *
 * @asrck_clk: clock sources to driver ASRC internal logic
 * @soc: soc specific data
 * @clk_map: clock map for input/output clock
 * @regcache_cfg: store register value of REG_ASRCFG
 */
#[repr(C)]
pub struct fsl_asrc_priv {
    pub asrck_clk: [*mut clk; ASRC_CLK_MAX_NUM],
    pub soc: *const fsl_asrc_soc_data,
    pub clk_map: [*mut ::core::ffi::c_uchar; 2],
    pub regcache_cfg: u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
