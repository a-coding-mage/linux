/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Routines for Cirrus Logic CS8427
 * Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 *
 * Translated from the C header. The snd_i2c types are supplied externally.
 */

pub const CS8427_BASE_ADDR: u8 = 0x10;

pub const CS8427_REG_AUTOINC: u8 = 0x80;
pub const CS8427_REG_CONTROL1: u8 = 0x01;
pub const CS8427_REG_CONTROL2: u8 = 0x02;
pub const CS8427_REG_DATAFLOW: u8 = 0x03;
pub const CS8427_REG_CLOCKSOURCE: u8 = 0x04;
pub const CS8427_REG_SERIALINPUT: u8 = 0x05;
pub const CS8427_REG_SERIALOUTPUT: u8 = 0x06;
pub const CS8427_REG_INT1STATUS: u8 = 0x07;
pub const CS8427_REG_INT2STATUS: u8 = 0x08;
pub const CS8427_REG_INT1MASK: u8 = 0x09;
pub const CS8427_REG_INT1MODEMSB: u8 = 0x0a;
pub const CS8427_REG_INT1MODELSB: u8 = 0x0b;
pub const CS8427_REG_INT2MASK: u8 = 0x0c;
pub const CS8427_REG_INT2MODEMSB: u8 = 0x0d;
pub const CS8427_REG_INT2MODELSB: u8 = 0x0e;
pub const CS8427_REG_RECVCSDATA: u8 = 0x0f;
pub const CS8427_REG_RECVERRORS: u8 = 0x10;
pub const CS8427_REG_RECVERRMASK: u8 = 0x11;
pub const CS8427_REG_CSDATABUF: u8 = 0x12;
pub const CS8427_REG_UDATABUF: u8 = 0x13;
pub const CS8427_REG_QSUBCODE: u8 = 0x14;
pub const CS8427_REG_OMCKRMCKRATIO: u8 = 0x1e;
pub const CS8427_REG_CORU_DATABUF: u8 = 0x20;
pub const CS8427_REG_ID_AND_VER: u8 = 0x7f;

/* CS8427_REG_CONTROL1 bits */
pub const CS8427_SWCLK: u8 = 1 << 7;
pub const CS8427_VSET: u8 = 1 << 6;
pub const CS8427_MUTESAO: u8 = 1 << 5;
pub const CS8427_MUTEAES: u8 = 1 << 4;
pub const CS8427_INTMASK: u8 = 3 << 1;
pub const CS8427_INTACTHIGH: u8 = 0 << 1;
pub const CS8427_INTACTLOW: u8 = 1 << 1;
pub const CS8427_INTOPENDRAIN: u8 = 2 << 1;
pub const CS8427_TCBLDIR: u8 = 1 << 0;

/* CS8427_REQ_CONTROL2 bits */
pub const CS8427_HOLDMASK: u8 = 3 << 5;
pub const CS8427_HOLDLASTSAMPLE: u8 = 0 << 5;
pub const CS8427_HOLDZERO: u8 = 1 << 5;
pub const CS8427_HOLDNOCHANGE: u8 = 2 << 5;
pub const CS8427_RMCKF: u8 = 1 << 4;
pub const CS8427_MMR: u8 = 1 << 3;
pub const CS8427_MMT: u8 = 1 << 2;
pub const CS8427_MMTCS: u8 = 1 << 1;
pub const CS8427_MMTLR: u8 = 1 << 0;

/* CS8427_REG_DATAFLOW */
pub const CS8427_TXOFF: u8 = 1 << 6;
pub const CS8427_AESBP: u8 = 1 << 5;
pub const CS8427_TXDMASK: u8 = 3 << 3;
pub const CS8427_TXDSERIAL: u8 = 1 << 3;
pub const CS8427_TXAES3DRECEIVER: u8 = 2 << 3;
pub const CS8427_SPDMASK: u8 = 3 << 1;
pub const CS8427_SPDSERIAL: u8 = 1 << 1;
pub const CS8427_SPDAES3RECEIVER: u8 = 2 << 1;

/* CS8427_REG_CLOCKSOURCE */
pub const CS8427_RUN: u8 = 1 << 6;
pub const CS8427_CLKMASK: u8 = 3 << 4;
pub const CS8427_CLK256: u8 = 0 << 4;
pub const CS8427_CLK384: u8 = 1 << 4;
pub const CS8427_CLK512: u8 = 2 << 4;
pub const CS8427_OUTC: u8 = 1 << 3;
pub const CS8427_INC: u8 = 1 << 2;
pub const CS8427_RXDMASK: u8 = 3;
pub const CS8427_RXDILRCK: u8 = 0;
pub const CS8427_RXDAES3INPUT: u8 = 1;
pub const CS8427_EXTCLOCKRESET: u8 = 2;
pub const CS8427_EXTCLOCK: u8 = 3;

/* CS8427_REG_SERIALINPUT */
pub const CS8427_SIMS: u8 = 1 << 7;
pub const CS8427_SISF: u8 = 1 << 6;
pub const CS8427_SIRESMASK: u8 = 3 << 4;
pub const CS8427_SIRES24: u8 = 0 << 4;
pub const CS8427_SIRES20: u8 = 1 << 4;
pub const CS8427_SIRES16: u8 = 2 << 4;
pub const CS8427_SIJUST: u8 = 1 << 3;
pub const CS8427_SIDEL: u8 = 1 << 2;
pub const CS8427_SISPOL: u8 = 1 << 1;
pub const CS8427_SILRPOL: u8 = 1;

/* CS8427_REG_SERIALOUTPUT */
pub const CS8427_SOMS: u8 = 1 << 7;
pub const CS8427_SOSF: u8 = 1 << 6;
pub const CS8427_SORESMASK: u8 = 3 << 4;
pub const CS8427_SORES24: u8 = 0 << 4;
pub const CS8427_SORES20: u8 = 1 << 4;
pub const CS8427_SORES16: u8 = 2 << 4;
pub const CS8427_SORESDIRECT: u8 = 2 << 4;
pub const CS8427_SOJUST: u8 = 1 << 3;
pub const CS8427_SODEL: u8 = 1 << 2;
pub const CS8427_SOSPOL: u8 = 1 << 1;
pub const CS8427_SOLRPOL: u8 = 1;

/* CS8427_REG_INT1STATUS */
pub const CS8427_TSLIP: u8 = 1 << 7;
pub const CS8427_OSLIP: u8 = 1 << 6;
pub const CS8427_DETC: u8 = 1 << 2;
pub const CS8427_EFTC: u8 = 1 << 1;
pub const CS8427_RERR: u8 = 1;
/* CS8427_REG_INT2STATUS */
pub const CS8427_DETU: u8 = 1 << 3;
pub const CS8427_EFTU: u8 = 1 << 2;
pub const CS8427_QCH: u8 = 1 << 1;
/* CS8427_REG_INT1MODEMSB && CS8427_REG_INT1MODELSB: bits are defined in status */
/* CS8427_REG_INT2MODEMSB && CS8427_REG_INT2MODELSB: bits are defined in status */
pub const CS8427_INTMODERISINGMSB: u8 = 0;
pub const CS8427_INTMODERESINGLSB: u8 = 0;
pub const CS8427_INTMODEFALLINGMSB: u8 = 0;
pub const CS8427_INTMODEFALLINGLSB: u8 = 1;
pub const CS8427_INTMODELEVELMSB: u8 = 1;
pub const CS8427_INTMODELEVELLSB: u8 = 0;

/* CS8427_REG_RECVCSDATA */
pub const CS8427_AUXMASK: u8 = 15 << 4;
pub const CS8427_AUXSHIFT: u8 = 4;
pub const CS8427_PRO: u8 = 1 << 3;
pub const CS8427_AUDIO: u8 = 1 << 2;
pub const CS8427_COPY: u8 = 1 << 1;
pub const CS8427_ORIG: u8 = 1;
/* CS8427_REG_RECVERRORS / CS8427_REG_RECVERRMASK for CS8427_RERR */
pub const CS8427_QCRC: u8 = 1 << 6;
pub const CS8427_CCRC: u8 = 1 << 5;
pub const CS8427_UNLOCK: u8 = 1 << 4;
pub const CS8427_V: u8 = 1 << 3;
pub const CS8427_CONF: u8 = 1 << 2;
pub const CS8427_BIP: u8 = 1 << 1;
pub const CS8427_PAR: u8 = 1;

/* CS8427_REG_CSDATABUF */
pub const CS8427_BSEL: u8 = 1 << 5;
pub const CS8427_CBMR: u8 = 1 << 4;
pub const CS8427_DETCI: u8 = 1 << 3;
pub const CS8427_EFTCI: u8 = 1 << 2;
pub const CS8427_CAM: u8 = 1 << 1;
pub const CS8427_CHS: u8 = 1;
/* CS8427_REG_UDATABUF */
pub const CS8427_UD: u8 = 1 << 4;
pub const CS8427_UBMMASK: u8 = 3 << 2;
pub const CS8427_UBMZEROS: u8 = 0 << 2;
pub const CS8427_UBMBLOCK: u8 = 1 << 2;
pub const CS8427_DETUI: u8 = 1 << 1;
pub const CS8427_EFTUI: u8 = 1 << 1;
/* CS8427_REG_ID_AND_VER */
pub const CS8427_IDMASK: u8 = 15 << 4;
pub const CS8427_IDSHIFT: u8 = 4;
pub const CS8427_VERMASK: u8 = 15;
pub const CS8427_VERSHIFT: u8 = 0;
pub const CS8427_VER8427A: u8 = 0x71;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_i2c_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_i2c_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn snd_cs8427_init(bus: *mut snd_i2c_bus, device: *mut snd_i2c_device) -> i32;
    pub fn snd_cs8427_create(
        bus: *mut snd_i2c_bus,
        addr: u8,
        reset_timeout: u32,
        r_cs8427: *mut *mut snd_i2c_device,
    ) -> i32;
    pub fn snd_cs8427_reg_write(
        device: *mut snd_i2c_device,
        reg: u8,
        val: u8,
    ) -> i32;
    pub fn snd_cs8427_iec958_build(
        cs8427: *mut snd_i2c_device,
        playback_substream: *mut snd_pcm_substream,
        capture_substream: *mut snd_pcm_substream,
    ) -> i32;
    pub fn snd_cs8427_iec958_active(cs8427: *mut snd_i2c_device, active: i32) -> i32;
    pub fn snd_cs8427_iec958_pcm(cs8427: *mut snd_i2c_device, rate: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
