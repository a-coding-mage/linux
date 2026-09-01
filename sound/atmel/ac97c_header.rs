/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Register definitions for Atmel AC97C
 *
 * Copyright (C) 2005-2009 Atmel Corporation
 */

pub const AC97C_MR: u32 = 0x08;
pub const AC97C_ICA: u32 = 0x10;
pub const AC97C_OCA: u32 = 0x14;
pub const AC97C_CARHR: u32 = 0x20;
pub const AC97C_CATHR: u32 = 0x24;
pub const AC97C_CASR: u32 = 0x28;
pub const AC97C_CAMR: u32 = 0x2c;
pub const AC97C_CORHR: u32 = 0x40;
pub const AC97C_COTHR: u32 = 0x44;
pub const AC97C_COSR: u32 = 0x48;
pub const AC97C_COMR: u32 = 0x4c;
pub const AC97C_SR: u32 = 0x50;
pub const AC97C_IER: u32 = 0x54;
pub const AC97C_IDR: u32 = 0x58;
pub const AC97C_IMR: u32 = 0x5c;
pub const AC97C_VERSION: u32 = 0xfc;

pub const AC97C_CATPR: u32 = PDC_TPR;
pub const AC97C_CATCR: u32 = PDC_TCR;
pub const AC97C_CATNPR: u32 = PDC_TNPR;
pub const AC97C_CATNCR: u32 = PDC_TNCR;
pub const AC97C_CARPR: u32 = PDC_RPR;
pub const AC97C_CARCR: u32 = PDC_RCR;
pub const AC97C_CARNPR: u32 = PDC_RNPR;
pub const AC97C_CARNCR: u32 = PDC_RNCR;
pub const AC97C_PTCR: u32 = PDC_PTCR;

pub const AC97C_MR_ENA: u32 = 1 << 0;
pub const AC97C_MR_WRST: u32 = 1 << 1;
pub const AC97C_MR_VRA: u32 = 1 << 2;

pub const AC97C_CSR_TXRDY: u32 = 1 << 0;
pub const AC97C_CSR_TXEMPTY: u32 = 1 << 1;
pub const AC97C_CSR_UNRUN: u32 = 1 << 2;
pub const AC97C_CSR_RXRDY: u32 = 1 << 4;
pub const AC97C_CSR_OVRUN: u32 = 1 << 5;
pub const AC97C_CSR_ENDTX: u32 = 1 << 10;
pub const AC97C_CSR_ENDRX: u32 = 1 << 14;

pub const AC97C_CMR_SIZE_20: u32 = 0 << 16;
pub const AC97C_CMR_SIZE_18: u32 = 1 << 16;
pub const AC97C_CMR_SIZE_16: u32 = 2 << 16;
pub const AC97C_CMR_SIZE_10: u32 = 3 << 16;
pub const AC97C_CMR_CEM_LITTLE: u32 = 1 << 18;
pub const AC97C_CMR_CEM_BIG: u32 = 0 << 18;
pub const AC97C_CMR_CENA: u32 = 1 << 21;
pub const AC97C_CMR_DMAEN: u32 = 1 << 22;

pub const AC97C_SR_CAEVT: u32 = 1 << 3;
pub const AC97C_SR_COEVT: u32 = 1 << 2;
pub const AC97C_SR_WKUP: u32 = 1 << 1;
pub const AC97C_SR_SOF: u32 = 1 << 0;

macro_rules! AC97C_CH_MASK {
    ($slot:expr) => {
        (0x7u32 << (3u32 * (($slot as u32) - 3u32)))
    };
}

macro_rules! AC97C_CH_ASSIGN {
    ($slot:expr, $channel:expr) => {
        (($channel as u32) << (3u32 * (($slot as u32) - 3u32)))
    };
}

pub(crate) use AC97C_CH_ASSIGN;
pub(crate) use AC97C_CH_MASK;

pub const AC97C_CHANNEL_NONE: u32 = 0x0;
pub const AC97C_CHANNEL_A: u32 = 0x1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
