// SPDX-License-Identifier: GPL-2.0-only
/*
 * PCM3168A codec driver header
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

use core::ffi::c_int;

unsafe extern "C" {
    pub static pcm3168a_pm_ops: dev_pm_ops;
    pub static pcm3168a_regmap: regmap_config;

    pub fn pcm3168a_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    pub fn pcm3168a_remove(dev: *mut device);
}

pub const PCM3168A_RST_SMODE: u32 = 0x40;
pub const PCM3168A_MRST_MASK: u32 = 0x80;
pub const PCM3168A_SRST_MASK: u32 = 0x40;
pub const PCM3168A_DAC_SRDA_SHIFT: u32 = 0;
pub const PCM3168A_DAC_SRDA_MASK: u32 = 0x3;

pub const PCM3168A_DAC_PWR_MST_FMT: u32 = 0x41;
pub const PCM3168A_DAC_PSMDA_SHIFT: u32 = 7;
pub const PCM3168A_DAC_PSMDA_MASK: u32 = 0x80;
pub const PCM3168A_DAC_MSDA_SHIFT: u32 = 4;
pub const PCM3168A_DAC_MSDA_MASK: u32 = 0x70;
pub const PCM3168A_DAC_FMT_SHIFT: u32 = 0;
pub const PCM3168A_DAC_FMT_MASK: u32 = 0xf;

pub const PCM3168A_DAC_OP_FLT: u32 = 0x42;
pub const PCM3168A_DAC_OPEDA_SHIFT: u32 = 4;
pub const PCM3168A_DAC_OPEDA_MASK: u32 = 0xf0;
pub const PCM3168A_DAC_FLT_SHIFT: u32 = 0;
pub const PCM3168A_DAC_FLT_MASK: u32 = 0xf;

pub const PCM3168A_DAC_INV: u32 = 0x43;

pub const PCM3168A_DAC_MUTE: u32 = 0x44;

pub const PCM3168A_DAC_ZERO: u32 = 0x45;

pub const PCM3168A_DAC_ATT_DEMP_ZF: u32 = 0x46;
pub const PCM3168A_DAC_ATMDDA_MASK: u32 = 0x80;
pub const PCM3168A_DAC_ATMDDA_SHIFT: u32 = 7;
pub const PCM3168A_DAC_ATSPDA_MASK: u32 = 0x40;
pub const PCM3168A_DAC_ATSPDA_SHIFT: u32 = 6;
pub const PCM3168A_DAC_DEMP_SHIFT: u32 = 4;
pub const PCM3168A_DAC_DEMP_MASK: u32 = 0x30;
pub const PCM3168A_DAC_AZRO_SHIFT: u32 = 1;
pub const PCM3168A_DAC_AZRO_MASK: u32 = 0xe;
pub const PCM3168A_DAC_ZREV_MASK: u32 = 0x1;
pub const PCM3168A_DAC_ZREV_SHIFT: u32 = 0;

pub const PCM3168A_DAC_VOL_MASTER: u32 = 0x47;

pub const PCM3168A_DAC_VOL_CHAN_START: u32 = 0x48;

pub const PCM3168A_ADC_SMODE: u32 = 0x50;
pub const PCM3168A_ADC_SRAD_SHIFT: u32 = 0;
pub const PCM3168A_ADC_SRAD_MASK: u32 = 0x3;

pub const PCM3168A_ADC_MST_FMT: u32 = 0x51;
pub const PCM3168A_ADC_MSAD_SHIFT: u32 = 4;
pub const PCM3168A_ADC_MSAD_MASK: u32 = 0x70;
pub const PCM3168A_ADC_FMTAD_SHIFT: u32 = 0;
pub const PCM3168A_ADC_FMTAD_MASK: u32 = 0x7;

pub const PCM3168A_ADC_PWR_HPFB: u32 = 0x52;
pub const PCM3168A_ADC_PSVAD_SHIFT: u32 = 4;
pub const PCM3168A_ADC_PSVAD_MASK: u32 = 0x70;
pub const PCM3168A_ADC_BYP_SHIFT: u32 = 0;
pub const PCM3168A_ADC_BYP_MASK: u32 = 0x7;

pub const PCM3168A_ADC_SEAD: u32 = 0x53;

pub const PCM3168A_ADC_INV: u32 = 0x54;

pub const PCM3168A_ADC_MUTE: u32 = 0x55;

pub const PCM3168A_ADC_OV: u32 = 0x56;

pub const PCM3168A_ADC_ATT_OVF: u32 = 0x57;
pub const PCM3168A_ADC_ATMDAD_MASK: u32 = 0x80;
pub const PCM3168A_ADC_ATMDAD_SHIFT: u32 = 7;
pub const PCM3168A_ADC_ATSPAD_MASK: u32 = 0x40;
pub const PCM3168A_ADC_ATSPAD_SHIFT: u32 = 6;
pub const PCM3168A_ADC_OVFP_MASK: u32 = 0x1;
pub const PCM3168A_ADC_OVFP_SHIFT: u32 = 0;

pub const PCM3168A_ADC_VOL_MASTER: u32 = 0x58;

pub const PCM3168A_ADC_VOL_CHAN_START: u32 = 0x59;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
