// SPDX-License-Identifier: GPL-2.0
/*
 * fsl_esai.h - ALSA ESAI interface for the Freescale i.MX SoC
 *
 * Copyright (C) 2014 Freescale Semiconductor, Inc.
 *
 * Author: Nicolin Chen <Guangyu.Chen@freescale.com>
 */

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

/* ESAI Register Map */
pub const REG_ESAI_ETDR: u32 = 0x00;
pub const REG_ESAI_ERDR: u32 = 0x04;
pub const REG_ESAI_ECR: u32 = 0x08;
pub const REG_ESAI_ESR: u32 = 0x0C;
pub const REG_ESAI_TFCR: u32 = 0x10;
pub const REG_ESAI_TFSR: u32 = 0x14;
pub const REG_ESAI_RFCR: u32 = 0x18;
pub const REG_ESAI_RFSR: u32 = 0x1C;
pub const fn REG_ESAI_xFCR(tx: bool) -> u32 {
    if tx { REG_ESAI_TFCR } else { REG_ESAI_RFCR }
}
pub const fn REG_ESAI_xFSR(tx: bool) -> u32 {
    if tx { REG_ESAI_TFSR } else { REG_ESAI_RFSR }
}
pub const REG_ESAI_TX0: u32 = 0x80;
pub const REG_ESAI_TX1: u32 = 0x84;
pub const REG_ESAI_TX2: u32 = 0x88;
pub const REG_ESAI_TX3: u32 = 0x8C;
pub const REG_ESAI_TX4: u32 = 0x90;
pub const REG_ESAI_TX5: u32 = 0x94;
pub const REG_ESAI_TSR: u32 = 0x98;
pub const REG_ESAI_RX0: u32 = 0xA0;
pub const REG_ESAI_RX1: u32 = 0xA4;
pub const REG_ESAI_RX2: u32 = 0xA8;
pub const REG_ESAI_RX3: u32 = 0xAC;
pub const REG_ESAI_SAISR: u32 = 0xCC;
pub const REG_ESAI_SAICR: u32 = 0xD0;
pub const REG_ESAI_TCR: u32 = 0xD4;
pub const REG_ESAI_TCCR: u32 = 0xD8;
pub const REG_ESAI_RCR: u32 = 0xDC;
pub const REG_ESAI_RCCR: u32 = 0xE0;
pub const fn REG_ESAI_xCR(tx: bool) -> u32 {
    if tx { REG_ESAI_TCR } else { REG_ESAI_RCR }
}
pub const fn REG_ESAI_xCCR(tx: bool) -> u32 {
    if tx { REG_ESAI_TCCR } else { REG_ESAI_RCCR }
}
pub const REG_ESAI_TSMA: u32 = 0xE4;
pub const REG_ESAI_TSMB: u32 = 0xE8;
pub const REG_ESAI_RSMA: u32 = 0xEC;
pub const REG_ESAI_RSMB: u32 = 0xF0;
pub const fn REG_ESAI_xSMA(tx: bool) -> u32 {
    if tx { REG_ESAI_TSMA } else { REG_ESAI_RSMA }
}
pub const fn REG_ESAI_xSMB(tx: bool) -> u32 {
    if tx { REG_ESAI_TSMB } else { REG_ESAI_RSMB }
}
pub const REG_ESAI_PRRC: u32 = 0xF8;
pub const REG_ESAI_PCRC: u32 = 0xFC;

/* ESAI Control Register -- REG_ESAI_ECR 0x8 */
pub const ESAI_ECR_ETI_SHIFT: u32 = 19;
pub const ESAI_ECR_ETI_MASK: u32 = 1 << ESAI_ECR_ETI_SHIFT;
pub const ESAI_ECR_ETI: u32 = 1 << ESAI_ECR_ETI_SHIFT;
pub const ESAI_ECR_ETO_SHIFT: u32 = 18;
pub const ESAI_ECR_ETO_MASK: u32 = 1 << ESAI_ECR_ETO_SHIFT;
pub const ESAI_ECR_ETO: u32 = 1 << ESAI_ECR_ETO_SHIFT;
pub const ESAI_ECR_ERI_SHIFT: u32 = 17;
pub const ESAI_ECR_ERI_MASK: u32 = 1 << ESAI_ECR_ERI_SHIFT;
pub const ESAI_ECR_ERI: u32 = 1 << ESAI_ECR_ERI_SHIFT;
pub const ESAI_ECR_ERO_SHIFT: u32 = 16;
pub const ESAI_ECR_ERO_MASK: u32 = 1 << ESAI_ECR_ERO_SHIFT;
pub const ESAI_ECR_ERO: u32 = 1 << ESAI_ECR_ERO_SHIFT;
pub const ESAI_ECR_ERST_SHIFT: u32 = 1;
pub const ESAI_ECR_ERST_MASK: u32 = 1 << ESAI_ECR_ERST_SHIFT;
pub const ESAI_ECR_ERST: u32 = 1 << ESAI_ECR_ERST_SHIFT;
pub const ESAI_ECR_ESAIEN_SHIFT: u32 = 0;
pub const ESAI_ECR_ESAIEN_MASK: u32 = 1 << ESAI_ECR_ESAIEN_SHIFT;
pub const ESAI_ECR_ESAIEN: u32 = 1 << ESAI_ECR_ESAIEN_SHIFT;

/* ESAI Status Register -- REG_ESAI_ESR 0xC */
pub const ESAI_ESR_TINIT_SHIFT: u32 = 10;
pub const ESAI_ESR_TINIT_MASK: u32 = 1 << ESAI_ESR_TINIT_SHIFT;
pub const ESAI_ESR_TINIT: u32 = 1 << ESAI_ESR_TINIT_SHIFT;
pub const ESAI_ESR_RFF_SHIFT: u32 = 9;
pub const ESAI_ESR_RFF_MASK: u32 = 1 << ESAI_ESR_RFF_SHIFT;
pub const ESAI_ESR_RFF: u32 = 1 << ESAI_ESR_RFF_SHIFT;
pub const ESAI_ESR_TFE_SHIFT: u32 = 8;
pub const ESAI_ESR_TFE_MASK: u32 = 1 << ESAI_ESR_TFE_SHIFT;
pub const ESAI_ESR_TFE: u32 = 1 << ESAI_ESR_TFE_SHIFT;
pub const ESAI_ESR_TLS_SHIFT: u32 = 7;
pub const ESAI_ESR_TLS_MASK: u32 = 1 << ESAI_ESR_TLS_SHIFT;
pub const ESAI_ESR_TLS: u32 = 1 << ESAI_ESR_TLS_SHIFT;
pub const ESAI_ESR_TDE_SHIFT: u32 = 6;
pub const ESAI_ESR_TDE_MASK: u32 = 1 << ESAI_ESR_TDE_SHIFT;
pub const ESAI_ESR_TDE: u32 = 1 << ESAI_ESR_TDE_SHIFT;
pub const ESAI_ESR_TED_SHIFT: u32 = 5;
pub const ESAI_ESR_TED_MASK: u32 = 1 << ESAI_ESR_TED_SHIFT;
pub const ESAI_ESR_TED: u32 = 1 << ESAI_ESR_TED_SHIFT;
pub const ESAI_ESR_TD_SHIFT: u32 = 4;
pub const ESAI_ESR_TD_MASK: u32 = 1 << ESAI_ESR_TD_SHIFT;
pub const ESAI_ESR_TD: u32 = 1 << ESAI_ESR_TD_SHIFT;
pub const ESAI_ESR_RLS_SHIFT: u32 = 3;
pub const ESAI_ESR_RLS_MASK: u32 = 1 << ESAI_ESR_RLS_SHIFT;
pub const ESAI_ESR_RLS: u32 = 1 << ESAI_ESR_RLS_SHIFT;
pub const ESAI_ESR_RDE_SHIFT: u32 = 2;
pub const ESAI_ESR_RDE_MASK: u32 = 1 << ESAI_ESR_RDE_SHIFT;
pub const ESAI_ESR_RDE: u32 = 1 << ESAI_ESR_RDE_SHIFT;
pub const ESAI_ESR_RED_SHIFT: u32 = 1;
pub const ESAI_ESR_RED_MASK: u32 = 1 << ESAI_ESR_RED_SHIFT;
pub const ESAI_ESR_RED: u32 = 1 << ESAI_ESR_RED_SHIFT;
pub const ESAI_ESR_RD_SHIFT: u32 = 0;
pub const ESAI_ESR_RD_MASK: u32 = 1 << ESAI_ESR_RD_SHIFT;
pub const ESAI_ESR_RD: u32 = 1 << ESAI_ESR_RD_SHIFT;

/*
 * Transmit FIFO Configuration Register -- REG_ESAI_TFCR 0x10
 * Receive FIFO Configuration Register -- REG_ESAI_RFCR 0x18
 */
pub const ESAI_xFCR_TIEN_SHIFT: u32 = 19;
pub const ESAI_xFCR_TIEN_MASK: u32 = 1 << ESAI_xFCR_TIEN_SHIFT;
pub const ESAI_xFCR_TIEN: u32 = 1 << ESAI_xFCR_TIEN_SHIFT;
pub const ESAI_xFCR_REXT_SHIFT: u32 = 19;
pub const ESAI_xFCR_REXT_MASK: u32 = 1 << ESAI_xFCR_REXT_SHIFT;
pub const ESAI_xFCR_REXT: u32 = 1 << ESAI_xFCR_REXT_SHIFT;
pub const ESAI_xFCR_xWA_SHIFT: u32 = 16;
pub const ESAI_xFCR_xWA_WIDTH: u32 = 3;
pub const ESAI_xFCR_xWA_MASK: u32 = ((1 << ESAI_xFCR_xWA_WIDTH) - 1) << ESAI_xFCR_xWA_SHIFT;
pub const fn ESAI_xFCR_xWA(v: u32) -> u32 {
    ((8 - (v >> 2)) << ESAI_xFCR_xWA_SHIFT) & ESAI_xFCR_xWA_MASK
}
pub const ESAI_xFCR_xFWM_SHIFT: u32 = 8;
pub const ESAI_xFCR_xFWM_WIDTH: u32 = 8;
pub const ESAI_xFCR_xFWM_MASK: u32 = ((1 << ESAI_xFCR_xFWM_WIDTH) - 1) << ESAI_xFCR_xFWM_SHIFT;
pub const fn ESAI_xFCR_xFWM(v: u32) -> u32 {
    ((v - 1) << ESAI_xFCR_xFWM_SHIFT) & ESAI_xFCR_xFWM_MASK
}
pub const ESAI_xFCR_xE_SHIFT: u32 = 2;
pub const ESAI_xFCR_TE_WIDTH: u32 = 6;
pub const ESAI_xFCR_RE_WIDTH: u32 = 4;
pub const ESAI_xFCR_TE_MASK: u32 = ((1 << ESAI_xFCR_TE_WIDTH) - 1) << ESAI_xFCR_xE_SHIFT;
pub const ESAI_xFCR_RE_MASK: u32 = ((1 << ESAI_xFCR_RE_WIDTH) - 1) << ESAI_xFCR_xE_SHIFT;
pub const fn ESAI_xFCR_TE(x: u32) -> u32 {
    (ESAI_xFCR_TE_MASK >> (ESAI_xFCR_TE_WIDTH - x)) & ESAI_xFCR_TE_MASK
}
pub const fn ESAI_xFCR_RE(x: u32) -> u32 {
    (ESAI_xFCR_RE_MASK >> (ESAI_xFCR_RE_WIDTH - x)) & ESAI_xFCR_RE_MASK
}
pub const ESAI_xFCR_xFR_SHIFT: u32 = 1;
pub const ESAI_xFCR_xFR_MASK: u32 = 1 << ESAI_xFCR_xFR_SHIFT;
pub const ESAI_xFCR_xFR: u32 = 1 << ESAI_xFCR_xFR_SHIFT;
pub const ESAI_xFCR_xFEN_SHIFT: u32 = 0;
pub const ESAI_xFCR_xFEN_MASK: u32 = 1 << ESAI_xFCR_xFEN_SHIFT;
pub const ESAI_xFCR_xFEN: u32 = 1 << ESAI_xFCR_xFEN_SHIFT;

/*
 * Transmit FIFO Status Register -- REG_ESAI_TFSR 0x14
 * Receive FIFO Status Register --REG_ESAI_RFSR 0x1C
 */
pub const ESAI_xFSR_NTFO_SHIFT: u32 = 12;
pub const ESAI_xFSR_NRFI_SHIFT: u32 = 12;
pub const ESAI_xFSR_NTFI_SHIFT: u32 = 8;
pub const ESAI_xFSR_NRFO_SHIFT: u32 = 8;
pub const ESAI_xFSR_NTFx_WIDTH: u32 = 3;
pub const ESAI_xFSR_NRFx_WIDTH: u32 = 2;
pub const ESAI_xFSR_NTFO_MASK: u32 = ((1 << ESAI_xFSR_NTFx_WIDTH) - 1) << ESAI_xFSR_NTFO_SHIFT;
pub const ESAI_xFSR_NTFI_MASK: u32 = ((1 << ESAI_xFSR_NTFx_WIDTH) - 1) << ESAI_xFSR_NTFI_SHIFT;
pub const ESAI_xFSR_NRFO_MASK: u32 = ((1 << ESAI_xFSR_NRFx_WIDTH) - 1) << ESAI_xFSR_NRFO_SHIFT;
pub const ESAI_xFSR_NRFI_MASK: u32 = ((1 << ESAI_xFSR_NRFx_WIDTH) - 1) << ESAI_xFSR_NRFI_SHIFT;
pub const ESAI_xFSR_xFCNT_SHIFT: u32 = 0;
pub const ESAI_xFSR_xFCNT_WIDTH: u32 = 8;
pub const ESAI_xFSR_xFCNT_MASK: u32 = ((1 << ESAI_xFSR_xFCNT_WIDTH) - 1) << ESAI_xFSR_xFCNT_SHIFT;

/* ESAI Transmit Slot Register -- REG_ESAI_TSR 0x98 */
pub const ESAI_TSR_SHIFT: u32 = 0;
pub const ESAI_TSR_WIDTH: u32 = 24;
pub const ESAI_TSR_MASK: u32 = ((1 << ESAI_TSR_WIDTH) - 1) << ESAI_TSR_SHIFT;

/* Serial Audio Interface Status Register -- REG_ESAI_SAISR 0xCC */
pub const ESAI_SAISR_TODFE_SHIFT: u32 = 17;
pub const ESAI_SAISR_TODFE_MASK: u32 = 1 << ESAI_SAISR_TODFE_SHIFT;
pub const ESAI_SAISR_TODFE: u32 = 1 << ESAI_SAISR_TODFE_SHIFT;
pub const ESAI_SAISR_TEDE_SHIFT: u32 = 16;
pub const ESAI_SAISR_TEDE_MASK: u32 = 1 << ESAI_SAISR_TEDE_SHIFT;
pub const ESAI_SAISR_TEDE: u32 = 1 << ESAI_SAISR_TEDE_SHIFT;
pub const ESAI_SAISR_TDE_SHIFT: u32 = 15;
pub const ESAI_SAISR_TDE_MASK: u32 = 1 << ESAI_SAISR_TDE_SHIFT;
pub const ESAI_SAISR_TDE: u32 = 1 << ESAI_SAISR_TDE_SHIFT;
pub const ESAI_SAISR_TUE_SHIFT: u32 = 14;
pub const ESAI_SAISR_TUE_MASK: u32 = 1 << ESAI_SAISR_TUE_SHIFT;
pub const ESAI_SAISR_TUE: u32 = 1 << ESAI_SAISR_TUE_SHIFT;
pub const ESAI_SAISR_TFS_SHIFT: u32 = 13;
pub const ESAI_SAISR_TFS_MASK: u32 = 1 << ESAI_SAISR_TFS_SHIFT;
pub const ESAI_SAISR_TFS: u32 = 1 << ESAI_SAISR_TFS_SHIFT;
pub const ESAI_SAISR_RODF_SHIFT: u32 = 10;
pub const ESAI_SAISR_RODF_MASK: u32 = 1 << ESAI_SAISR_RODF_SHIFT;
pub const ESAI_SAISR_RODF: u32 = 1 << ESAI_SAISR_RODF_SHIFT;
pub const ESAI_SAISR_REDF_SHIFT: u32 = 9;
pub const ESAI_SAISR_REDF_MASK: u32 = 1 << ESAI_SAISR_REDF_SHIFT;
pub const ESAI_SAISR_REDF: u32 = 1 << ESAI_SAISR_REDF_SHIFT;
pub const ESAI_SAISR_RDF_SHIFT: u32 = 8;
pub const ESAI_SAISR_RDF_MASK: u32 = 1 << ESAI_SAISR_RDF_SHIFT;
pub const ESAI_SAISR_RDF: u32 = 1 << ESAI_SAISR_RDF_SHIFT;
pub const ESAI_SAISR_ROE_SHIFT: u32 = 7;
pub const ESAI_SAISR_ROE_MASK: u32 = 1 << ESAI_SAISR_ROE_SHIFT;
pub const ESAI_SAISR_ROE: u32 = 1 << ESAI_SAISR_ROE_SHIFT;
pub const ESAI_SAISR_RFS_SHIFT: u32 = 6;
pub const ESAI_SAISR_RFS_MASK: u32 = 1 << ESAI_SAISR_RFS_SHIFT;
pub const ESAI_SAISR_RFS: u32 = 1 << ESAI_SAISR_RFS_SHIFT;
pub const ESAI_SAISR_IF2_SHIFT: u32 = 2;
pub const ESAI_SAISR_IF2_MASK: u32 = 1 << ESAI_SAISR_IF2_SHIFT;
pub const ESAI_SAISR_IF2: u32 = 1 << ESAI_SAISR_IF2_SHIFT;
pub const ESAI_SAISR_IF1_SHIFT: u32 = 1;
pub const ESAI_SAISR_IF1_MASK: u32 = 1 << ESAI_SAISR_IF1_SHIFT;
pub const ESAI_SAISR_IF1: u32 = 1 << ESAI_SAISR_IF1_SHIFT;
pub const ESAI_SAISR_IF0_SHIFT: u32 = 0;
pub const ESAI_SAISR_IF0_MASK: u32 = 1 << ESAI_SAISR_IF0_SHIFT;
pub const ESAI_SAISR_IF0: u32 = 1 << ESAI_SAISR_IF0_SHIFT;

/* Serial Audio Interface Control Register -- REG_ESAI_SAICR 0xD0 */
pub const ESAI_SAICR_ALC_SHIFT: u32 = 8;
pub const ESAI_SAICR_ALC_MASK: u32 = 1 << ESAI_SAICR_ALC_SHIFT;
pub const ESAI_SAICR_ALC: u32 = 1 << ESAI_SAICR_ALC_SHIFT;
pub const ESAI_SAICR_TEBE_SHIFT: u32 = 7;
pub const ESAI_SAICR_TEBE_MASK: u32 = 1 << ESAI_SAICR_TEBE_SHIFT;
pub const ESAI_SAICR_TEBE: u32 = 1 << ESAI_SAICR_TEBE_SHIFT;
pub const ESAI_SAICR_SYNC_SHIFT: u32 = 6;
pub const ESAI_SAICR_SYNC_MASK: u32 = 1 << ESAI_SAICR_SYNC_SHIFT;
pub const ESAI_SAICR_SYNC: u32 = 1 << ESAI_SAICR_SYNC_SHIFT;
pub const ESAI_SAICR_OF2_SHIFT: u32 = 2;
pub const ESAI_SAICR_OF2_MASK: u32 = 1 << ESAI_SAICR_OF2_SHIFT;
pub const ESAI_SAICR_OF2: u32 = 1 << ESAI_SAICR_OF2_SHIFT;
pub const ESAI_SAICR_OF1_SHIFT: u32 = 1;
pub const ESAI_SAICR_OF1_MASK: u32 = 1 << ESAI_SAICR_OF1_SHIFT;
pub const ESAI_SAICR_OF1: u32 = 1 << ESAI_SAICR_OF1_SHIFT;
pub const ESAI_SAICR_OF0_SHIFT: u32 = 0;
pub const ESAI_SAICR_OF0_MASK: u32 = 1 << ESAI_SAICR_OF0_SHIFT;
pub const ESAI_SAICR_OF0: u32 = 1 << ESAI_SAICR_OF0_SHIFT;

/*
 * Transmit Control Register -- REG_ESAI_TCR 0xD4
 * Receive Control Register -- REG_ESAI_RCR 0xDC
 */
pub const ESAI_xCR_xLIE_SHIFT: u32 = 23;
pub const ESAI_xCR_xLIE_MASK: u32 = 1 << ESAI_xCR_xLIE_SHIFT;
pub const ESAI_xCR_xLIE: u32 = 1 << ESAI_xCR_xLIE_SHIFT;
pub const ESAI_xCR_xIE_SHIFT: u32 = 22;
pub const ESAI_xCR_xIE_MASK: u32 = 1 << ESAI_xCR_xIE_SHIFT;
pub const ESAI_xCR_xIE: u32 = 1 << ESAI_xCR_xIE_SHIFT;
pub const ESAI_xCR_xEDIE_SHIFT: u32 = 21;
pub const ESAI_xCR_xEDIE_MASK: u32 = 1 << ESAI_xCR_xEDIE_SHIFT;
pub const ESAI_xCR_xEDIE: u32 = 1 << ESAI_xCR_xEDIE_SHIFT;
pub const ESAI_xCR_xEIE_SHIFT: u32 = 20;
pub const ESAI_xCR_xEIE_MASK: u32 = 1 << ESAI_xCR_xEIE_SHIFT;
pub const ESAI_xCR_xEIE: u32 = 1 << ESAI_xCR_xEIE_SHIFT;
pub const ESAI_xCR_xPR_SHIFT: u32 = 19;
pub const ESAI_xCR_xPR_MASK: u32 = 1 << ESAI_xCR_xPR_SHIFT;
pub const ESAI_xCR_xPR: u32 = 1 << ESAI_xCR_xPR_SHIFT;
pub const ESAI_xCR_PADC_SHIFT: u32 = 17;
pub const ESAI_xCR_PADC_MASK: u32 = 1 << ESAI_xCR_PADC_SHIFT;
pub const ESAI_xCR_PADC: u32 = 1 << ESAI_xCR_PADC_SHIFT;
pub const ESAI_xCR_xFSR_SHIFT: u32 = 16;
pub const ESAI_xCR_xFSR_MASK: u32 = 1 << ESAI_xCR_xFSR_SHIFT;
pub const ESAI_xCR_xFSR: u32 = 1 << ESAI_xCR_xFSR_SHIFT;
pub const ESAI_xCR_xFSL_SHIFT: u32 = 15;
pub const ESAI_xCR_xFSL_MASK: u32 = 1 << ESAI_xCR_xFSL_SHIFT;
pub const ESAI_xCR_xFSL: u32 = 1 << ESAI_xCR_xFSL_SHIFT;
pub const ESAI_xCR_xSWS_SHIFT: u32 = 10;
pub const ESAI_xCR_xSWS_WIDTH: u32 = 5;
pub const ESAI_xCR_xSWS_MASK: u32 = ((1 << ESAI_xCR_xSWS_WIDTH) - 1) << ESAI_xCR_xSWS_SHIFT;
pub const fn ESAI_xCR_xSWS(s: u32, w: u32) -> u32 {
    (if w < 24 {
        s - w + ((w - 8) >> 2)
    } else if s < 32 {
        0x1e
    } else {
        0x1f
    }) << ESAI_xCR_xSWS_SHIFT
}
pub const ESAI_xCR_xMOD_SHIFT: u32 = 8;
pub const ESAI_xCR_xMOD_WIDTH: u32 = 2;
pub const ESAI_xCR_xMOD_MASK: u32 = ((1 << ESAI_xCR_xMOD_WIDTH) - 1) << ESAI_xCR_xMOD_SHIFT;
pub const ESAI_xCR_xMOD_ONDEMAND: u32 = 0x1 << ESAI_xCR_xMOD_SHIFT;
pub const ESAI_xCR_xMOD_NETWORK: u32 = 0x1 << ESAI_xCR_xMOD_SHIFT;
pub const ESAI_xCR_xMOD_AC97: u32 = 0x3 << ESAI_xCR_xMOD_SHIFT;
pub const ESAI_xCR_xWA_SHIFT: u32 = 7;
pub const ESAI_xCR_xWA_MASK: u32 = 1 << ESAI_xCR_xWA_SHIFT;
pub const ESAI_xCR_xWA: u32 = 1 << ESAI_xCR_xWA_SHIFT;
pub const ESAI_xCR_xSHFD_SHIFT: u32 = 6;
pub const ESAI_xCR_xSHFD_MASK: u32 = 1 << ESAI_xCR_xSHFD_SHIFT;
pub const ESAI_xCR_xSHFD: u32 = 1 << ESAI_xCR_xSHFD_SHIFT;
pub const ESAI_xCR_xE_SHIFT: u32 = 0;
pub const ESAI_xCR_TE_WIDTH: u32 = 6;
pub const ESAI_xCR_RE_WIDTH: u32 = 4;
pub const ESAI_xCR_TE_MASK: u32 = ((1 << ESAI_xCR_TE_WIDTH) - 1) << ESAI_xCR_xE_SHIFT;
pub const ESAI_xCR_RE_MASK: u32 = ((1 << ESAI_xCR_RE_WIDTH) - 1) << ESAI_xCR_xE_SHIFT;
pub const fn ESAI_xCR_TE(x: u32) -> u32 {
    (ESAI_xCR_TE_MASK >> (ESAI_xCR_TE_WIDTH - x)) & ESAI_xCR_TE_MASK
}
pub const fn ESAI_xCR_RE(x: u32) -> u32 {
    (ESAI_xCR_RE_MASK >> (ESAI_xCR_RE_WIDTH - x)) & ESAI_xCR_RE_MASK
}

/*
 * Transmit Clock Control Register -- REG_ESAI_TCCR 0xD8
 * Receive Clock Control Register -- REG_ESAI_RCCR 0xE0
 */
pub const ESAI_xCCR_xHCKD_SHIFT: u32 = 23;
pub const ESAI_xCCR_xHCKD_MASK: u32 = 1 << ESAI_xCCR_xHCKD_SHIFT;
pub const ESAI_xCCR_xHCKD: u32 = 1 << ESAI_xCCR_xHCKD_SHIFT;
pub const ESAI_xCCR_xFSD_SHIFT: u32 = 22;
pub const ESAI_xCCR_xFSD_MASK: u32 = 1 << ESAI_xCCR_xFSD_SHIFT;
pub const ESAI_xCCR_xFSD: u32 = 1 << ESAI_xCCR_xFSD_SHIFT;
pub const ESAI_xCCR_xCKD_SHIFT: u32 = 21;
pub const ESAI_xCCR_xCKD_MASK: u32 = 1 << ESAI_xCCR_xCKD_SHIFT;
pub const ESAI_xCCR_xCKD: u32 = 1 << ESAI_xCCR_xCKD_SHIFT;
pub const ESAI_xCCR_xHCKP_SHIFT: u32 = 20;
pub const ESAI_xCCR_xHCKP_MASK: u32 = 1 << ESAI_xCCR_xHCKP_SHIFT;
pub const ESAI_xCCR_xHCKP: u32 = 1 << ESAI_xCCR_xHCKP_SHIFT;
pub const ESAI_xCCR_xFSP_SHIFT: u32 = 19;
pub const ESAI_xCCR_xFSP_MASK: u32 = 1 << ESAI_xCCR_xFSP_SHIFT;
pub const ESAI_xCCR_xFSP: u32 = 1 << ESAI_xCCR_xFSP_SHIFT;
pub const ESAI_xCCR_xCKP_SHIFT: u32 = 18;
pub const ESAI_xCCR_xCKP_MASK: u32 = 1 << ESAI_xCCR_xCKP_SHIFT;
pub const ESAI_xCCR_xCKP: u32 = 1 << ESAI_xCCR_xCKP_SHIFT;
pub const ESAI_xCCR_xFP_SHIFT: u32 = 14;
pub const ESAI_xCCR_xFP_WIDTH: u32 = 4;
pub const ESAI_xCCR_xFP_MASK: u32 = ((1 << ESAI_xCCR_xFP_WIDTH) - 1) << ESAI_xCCR_xFP_SHIFT;
pub const fn ESAI_xCCR_xFP(v: u32) -> u32 {
    ((v - 1) << ESAI_xCCR_xFP_SHIFT) & ESAI_xCCR_xFP_MASK
}
pub const ESAI_xCCR_xDC_SHIFT: u32 = 9;
pub const ESAI_xCCR_xDC_WIDTH: u32 = 5;
pub const ESAI_xCCR_xDC_MASK: u32 = ((1 << ESAI_xCCR_xDC_WIDTH) - 1) << ESAI_xCCR_xDC_SHIFT;
pub const fn ESAI_xCCR_xDC(v: u32) -> u32 {
    ((v - 1) << ESAI_xCCR_xDC_SHIFT) & ESAI_xCCR_xDC_MASK
}
pub const ESAI_xCCR_xPSR_SHIFT: u32 = 8;
pub const ESAI_xCCR_xPSR_MASK: u32 = 1 << ESAI_xCCR_xPSR_SHIFT;
pub const ESAI_xCCR_xPSR_BYPASS: u32 = 1 << ESAI_xCCR_xPSR_SHIFT;
pub const ESAI_xCCR_xPSR_DIV8: u32 = 0 << ESAI_xCCR_xPSR_SHIFT;
pub const ESAI_xCCR_xPM_SHIFT: u32 = 0;
pub const ESAI_xCCR_xPM_WIDTH: u32 = 8;
pub const ESAI_xCCR_xPM_MASK: u32 = ((1 << ESAI_xCCR_xPM_WIDTH) - 1) << ESAI_xCCR_xPM_SHIFT;
pub const fn ESAI_xCCR_xPM(v: u32) -> u32 {
    ((v - 1) << ESAI_xCCR_xPM_SHIFT) & ESAI_xCCR_xPM_MASK
}

/* Transmit Slot Mask Register A/B -- REG_ESAI_TSMA/B 0xE4 ~ 0xF0 */
pub const ESAI_xSMA_xS_SHIFT: u32 = 0;
pub const ESAI_xSMA_xS_WIDTH: u32 = 16;
pub const ESAI_xSMA_xS_MASK: u32 = ((1 << ESAI_xSMA_xS_WIDTH) - 1) << ESAI_xSMA_xS_SHIFT;
pub const fn ESAI_xSMA_xS(v: u32) -> u32 {
    v & ESAI_xSMA_xS_MASK
}
pub const ESAI_xSMB_xS_SHIFT: u32 = 0;
pub const ESAI_xSMB_xS_WIDTH: u32 = 16;
pub const ESAI_xSMB_xS_MASK: u32 = ((1 << ESAI_xSMB_xS_WIDTH) - 1) << ESAI_xSMB_xS_SHIFT;
pub const fn ESAI_xSMB_xS(v: u32) -> u32 {
    (v >> ESAI_xSMA_xS_WIDTH) & ESAI_xSMB_xS_MASK
}

/* Port C Direction Register -- REG_ESAI_PRRC 0xF8 */
pub const ESAI_PRRC_PDC_SHIFT: u32 = 0;
pub const ESAI_PRRC_PDC_WIDTH: u32 = 12;
pub const ESAI_PRRC_PDC_MASK: u32 = ((1 << ESAI_PRRC_PDC_WIDTH) - 1) << ESAI_PRRC_PDC_SHIFT;
pub const fn ESAI_PRRC_PDC(v: u32) -> u32 {
    v & ESAI_PRRC_PDC_MASK
}

/* Port C Control Register -- REG_ESAI_PCRC 0xFC */
pub const ESAI_PCRC_PC_SHIFT: u32 = 0;
pub const ESAI_PCRC_PC_WIDTH: u32 = 12;
pub const ESAI_PCRC_PC_MASK: u32 = ((1 << ESAI_PCRC_PC_WIDTH) - 1) << ESAI_PCRC_PC_SHIFT;
pub const fn ESAI_PCRC_PC(v: u32) -> u32 {
    v & ESAI_PCRC_PC_MASK
}

pub const ESAI_GPIO: u32 = 0xfff;

/* ESAI clock source */
pub const ESAI_HCKT_FSYS: u32 = 0;
pub const ESAI_HCKT_EXTAL: u32 = 1;
pub const ESAI_HCKR_FSYS: u32 = 2;
pub const ESAI_HCKR_EXTAL: u32 = 3;

/* ESAI clock divider */
pub const ESAI_TX_DIV_PSR: u32 = 0;
pub const ESAI_TX_DIV_PM: u32 = 1;
pub const ESAI_TX_DIV_FP: u32 = 2;
pub const ESAI_RX_DIV_PSR: u32 = 3;
pub const ESAI_RX_DIV_PM: u32 = 4;
pub const ESAI_RX_DIV_FP: u32 = 5;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
