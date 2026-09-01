/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * PDM Microphone Interface for the NXP i.MX SoC
 * Copyright 2018 NXP
 */

/* MICFIL Register Map */
pub const REG_MICFIL_CTRL1: u32 = 0x00;
pub const REG_MICFIL_CTRL2: u32 = 0x04;
pub const REG_MICFIL_STAT: u32 = 0x08;
pub const REG_MICFIL_FIFO_CTRL: u32 = 0x10;
pub const REG_MICFIL_FIFO_STAT: u32 = 0x14;
pub const REG_MICFIL_DATACH0: u32 = 0x24;
pub const REG_MICFIL_DATACH1: u32 = 0x28;
pub const REG_MICFIL_DATACH2: u32 = 0x2C;
pub const REG_MICFIL_DATACH3: u32 = 0x30;
pub const REG_MICFIL_DATACH4: u32 = 0x34;
pub const REG_MICFIL_DATACH5: u32 = 0x38;
pub const REG_MICFIL_DATACH6: u32 = 0x3C;
pub const REG_MICFIL_DATACH7: u32 = 0x40;
pub const REG_MICFIL_DC_CTRL: u32 = 0x64;
pub const REG_MICFIL_DC_OUT_CTRL: u32 = 0x68;
pub const REG_MICFIL_OUT_CTRL: u32 = 0x74;
pub const REG_MICFIL_OUT_STAT: u32 = 0x7C;
pub const REG_MICFIL_FSYNC_CTRL: u32 = 0x80;
pub const REG_MICFIL_VERID: u32 = 0x84;
pub const REG_MICFIL_PARAM: u32 = 0x88;
pub const REG_MICFIL_VAD0_CTRL1: u32 = 0x90;
pub const REG_MICFIL_VAD0_CTRL2: u32 = 0x94;
pub const REG_MICFIL_VAD0_STAT: u32 = 0x98;
pub const REG_MICFIL_VAD0_SCONFIG: u32 = 0x9C;
pub const REG_MICFIL_VAD0_NCONFIG: u32 = 0xA0;
pub const REG_MICFIL_VAD0_NDATA: u32 = 0xA4;
pub const REG_MICFIL_VAD0_ZCD: u32 = 0xA8;

pub const fn BIT(nr: u32) -> u32 {
    1u32.wrapping_shl(nr)
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    if h >= 31 {
        u32::MAX.wrapping_shl(l)
    } else {
        u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
    }
}

/* MICFIL Control Register 1 -- REG_MICFILL_CTRL1 0x00 */
pub const MICFIL_CTRL1_MDIS: u32 = BIT(31);
pub const MICFIL_CTRL1_DOZEN: u32 = BIT(30);
pub const MICFIL_CTRL1_PDMIEN: u32 = BIT(29);
pub const MICFIL_CTRL1_DBG: u32 = BIT(28);
pub const MICFIL_CTRL1_SRES: u32 = BIT(27);
pub const MICFIL_CTRL1_DBGE: u32 = BIT(26);
pub const MICFIL_CTRL1_DECFILS: u32 = BIT(20);
pub const MICFIL_CTRL1_FSYNCEN: u32 = BIT(16);

pub const MICFIL_CTRL1_DISEL_DISABLE: u32 = 0;
pub const MICFIL_CTRL1_DISEL_DMA: u32 = 1;
pub const MICFIL_CTRL1_DISEL_IRQ: u32 = 2;
pub const MICFIL_CTRL1_DISEL: u32 = GENMASK(25, 24);
pub const MICFIL_CTRL1_ERREN: u32 = BIT(23);
pub const fn MICFIL_CTRL1_CHEN(ch: u32) -> u32 {
    BIT(ch)
}

/* MICFIL Control Register 2 -- REG_MICFILL_CTRL2 0x04 */
pub const MICFIL_CTRL2_DEC_BYPASS: u32 = BIT(31);
pub const MICFIL_CTRL2_QSEL_SHIFT: u32 = 25;
pub const MICFIL_CTRL2_QSEL: u32 = GENMASK(27, 25);
pub const MICFIL_QSEL_MEDIUM_QUALITY: u32 = 0;
pub const MICFIL_QSEL_HIGH_QUALITY: u32 = 1;
pub const MICFIL_QSEL_LOW_QUALITY: u32 = 7;
pub const MICFIL_QSEL_VLOW0_QUALITY: u32 = 6;
pub const MICFIL_QSEL_VLOW1_QUALITY: u32 = 5;
pub const MICFIL_QSEL_VLOW2_QUALITY: u32 = 4;

pub const MICFIL_CTRL2_CICOSR: u32 = GENMASK(20, 16);
pub const MICFIL_CTRL2_CLKDIV: u32 = GENMASK(7, 0);

/* MICFIL Status Register -- REG_MICFIL_STAT 0x08 */
pub const MICFIL_STAT_BSY_FIL: u32 = BIT(31);
pub const MICFIL_STAT_FIR_RDY: u32 = BIT(30);
pub const MICFIL_STAT_LOWFREQF: u32 = BIT(29);
pub const fn MICFIL_STAT_CHXF(ch: u32) -> u32 {
    BIT(ch)
}

/* MICFIL FIFO Control Register -- REG_MICFIL_FIFO_CTRL 0x10 */
pub const MICFIL_FIFO_CTRL_FIFOWMK: u32 = GENMASK(4, 0);

/* MICFIL FIFO Status Register -- REG_MICFIL_FIFO_STAT 0x14 */
pub const fn MICFIL_FIFO_STAT_FIFOX_OVER(ch: u32) -> u32 {
    BIT(ch)
}

pub const fn MICFIL_FIFO_STAT_FIFOX_UNDER(ch: u32) -> u32 {
    BIT(ch + 8)
}

/* MICFIL DC Remover Control Register -- REG_MICFIL_DC_CTRL */
pub const MICFIL_DC_CTRL_CONFIG: u32 = GENMASK(15, 0);
pub const fn MICFIL_DC_CHX_SHIFT(ch: u32) -> u32 {
    ch << 1
}

pub const fn MICFIL_DC_CHX(ch: u32) -> u32 {
    GENMASK((ch << 1) + 1, ch << 1)
}

pub const MICFIL_DC_CUTOFF_21HZ: u32 = 0;
pub const MICFIL_DC_CUTOFF_83HZ: u32 = 1;
pub const MICFIL_DC_CUTOFF_152Hz: u32 = 2;
pub const MICFIL_DC_BYPASS: u32 = 3;

/* MICFIL VERID Register -- REG_MICFIL_VERID */
pub const MICFIL_VERID_MAJOR_SHIFT: u32 = 24;
pub const MICFIL_VERID_MAJOR_MASK: u32 = GENMASK(31, 24);
pub const MICFIL_VERID_MINOR_SHIFT: u32 = 16;
pub const MICFIL_VERID_MINOR_MASK: u32 = GENMASK(23, 16);
pub const MICFIL_VERID_FEATURE_SHIFT: u32 = 0;
pub const MICFIL_VERID_FEATURE_MASK: u32 = GENMASK(15, 0);

/* MICFIL PARAM Register -- REG_MICFIL_PARAM */
pub const MICFIL_PARAM_NUM_HWVAD_SHIFT: u32 = 24;
pub const MICFIL_PARAM_NUM_HWVAD_MASK: u32 = GENMASK(27, 24);
pub const MICFIL_PARAM_HWVAD_ZCD: u32 = BIT(19);
pub const MICFIL_PARAM_HWVAD_ENERGY_MODE: u32 = BIT(17);
pub const MICFIL_PARAM_HWVAD: u32 = BIT(16);
pub const MICFIL_PARAM_DC_OUT_BYPASS: u32 = BIT(11);
pub const MICFIL_PARAM_DC_IN_BYPASS: u32 = BIT(10);
pub const MICFIL_PARAM_LOW_POWER: u32 = BIT(9);
pub const MICFIL_PARAM_FIL_OUT_WIDTH: u32 = BIT(8);
pub const MICFIL_PARAM_FIFO_PTRWID_SHIFT: u32 = 4;
pub const MICFIL_PARAM_FIFO_PTRWID_MASK: u32 = GENMASK(7, 4);
pub const MICFIL_PARAM_NPAIR_SHIFT: u32 = 0;
pub const MICFIL_PARAM_NPAIR_MASK: u32 = GENMASK(3, 0);

/* MICFIL HWVAD0 Control 1 Register -- REG_MICFIL_VAD0_CTRL1*/
pub const MICFIL_VAD0_CTRL1_CHSEL: u32 = GENMASK(26, 24);
pub const MICFIL_VAD0_CTRL1_CICOSR: u32 = GENMASK(19, 16);
pub const MICFIL_VAD0_CTRL1_INITT: u32 = GENMASK(12, 8);
pub const MICFIL_VAD0_CTRL1_ST10: u32 = BIT(4);
pub const MICFIL_VAD0_CTRL1_ERIE: u32 = BIT(3);
pub const MICFIL_VAD0_CTRL1_IE: u32 = BIT(2);
pub const MICFIL_VAD0_CTRL1_RST: u32 = BIT(1);
pub const MICFIL_VAD0_CTRL1_EN: u32 = BIT(0);

/* MICFIL HWVAD0 Control 2 Register -- REG_MICFIL_VAD0_CTRL2*/
pub const MICFIL_VAD0_CTRL2_FRENDIS: u32 = BIT(31);
pub const MICFIL_VAD0_CTRL2_PREFEN: u32 = BIT(30);
pub const MICFIL_VAD0_CTRL2_FOUTDIS: u32 = BIT(28);
pub const MICFIL_VAD0_CTRL2_FRAMET: u32 = GENMASK(21, 16);
pub const MICFIL_VAD0_CTRL2_INPGAIN: u32 = GENMASK(11, 8);
pub const MICFIL_VAD0_CTRL2_HPF: u32 = GENMASK(1, 0);

/* MICFIL HWVAD0 Signal CONFIG Register -- REG_MICFIL_VAD0_SCONFIG */
pub const MICFIL_VAD0_SCONFIG_SFILEN: u32 = BIT(31);
pub const MICFIL_VAD0_SCONFIG_SMAXEN: u32 = BIT(30);
pub const MICFIL_VAD0_SCONFIG_SGAIN: u32 = GENMASK(3, 0);

/* MICFIL HWVAD0 Noise CONFIG Register -- REG_MICFIL_VAD0_NCONFIG */
pub const MICFIL_VAD0_NCONFIG_NFILAUT: u32 = BIT(31);
pub const MICFIL_VAD0_NCONFIG_NMINEN: u32 = BIT(30);
pub const MICFIL_VAD0_NCONFIG_NDECEN: u32 = BIT(29);
pub const MICFIL_VAD0_NCONFIG_NOREN: u32 = BIT(28);
pub const MICFIL_VAD0_NCONFIG_NFILADJ: u32 = GENMASK(12, 8);
pub const MICFIL_VAD0_NCONFIG_NGAIN: u32 = GENMASK(3, 0);

/* MICFIL HWVAD0 Zero-Crossing Detector - REG_MICFIL_VAD0_ZCD */
pub const MICFIL_VAD0_ZCD_ZCDTH: u32 = GENMASK(25, 16);
pub const MICFIL_VAD0_ZCD_ZCDADJ: u32 = GENMASK(11, 8);
pub const MICFIL_VAD0_ZCD_ZCDAND: u32 = BIT(4);
pub const MICFIL_VAD0_ZCD_ZCDAUT: u32 = BIT(2);
pub const MICFIL_VAD0_ZCD_ZCDEN: u32 = BIT(0);

/* MICFIL HWVAD0 Status Register - REG_MICFIL_VAD0_STAT */
pub const MICFIL_VAD0_STAT_INITF: u32 = BIT(31);
pub const MICFIL_VAD0_STAT_INSATF: u32 = BIT(16);
pub const MICFIL_VAD0_STAT_EF: u32 = BIT(15);
pub const MICFIL_VAD0_STAT_IF: u32 = BIT(0);

/* MICFIL Output Control Register */
pub const fn MICFIL_OUTGAIN_CHX_SHIFT(v: u32) -> u32 {
    4 * v
}

/* Constants */
pub const MICFIL_OUTPUT_CHANNELS: u32 = 8;
pub const MICFIL_FIFO_NUM: u32 = 8;

pub const FIFO_PTRWID: u32 = 3;
pub const FIFO_LEN: u32 = BIT(FIFO_PTRWID);

pub const MICFIL_IRQ_LINES: u32 = 4;
pub const MICFIL_MAX_RETRY: u32 = 25;
pub const MICFIL_SLEEP_MIN: u32 = 90000; /* in us */
pub const MICFIL_SLEEP_MAX: u32 = 100000; /* in us */
pub const MICFIL_DMA_MAXBURST_RX: u32 = 6;

/* HWVAD Constants */
pub const MICFIL_HWVAD_ENVELOPE_MODE: u32 = 0;
pub const MICFIL_HWVAD_ENERGY_MODE: u32 = 1;

/**
 * struct fsl_micfil_verid - version id data
 * @version: version number
 * @feature: feature specification number
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fsl_micfil_verid {
    pub version: u32,
    pub feature: u32,
}

/**
 * struct fsl_micfil_param - parameter data
 * @hwvad_num: the number of HWVADs
 * @hwvad_zcd: HWVAD zero-cross detector is active
 * @hwvad_energy_mode: HWVAD energy mode is active
 * @hwvad: HWVAD is active
 * @dc_out_bypass: points out if the output DC remover is disabled
 * @dc_in_bypass: points out if the input DC remover is disabled
 * @low_power: low power decimation filter
 * @fil_out_width: filter output width
 * @fifo_ptrwid: FIFO pointer width
 * @npair: number of microphone pairs
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fsl_micfil_param {
    pub hwvad_num: u32,
    pub hwvad_zcd: bool,
    pub hwvad_energy_mode: bool,
    pub hwvad: bool,
    pub dc_out_bypass: bool,
    pub dc_in_bypass: bool,
    pub low_power: bool,
    pub fil_out_width: bool,
    pub fifo_ptrwid: u32,
    pub npair: u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
