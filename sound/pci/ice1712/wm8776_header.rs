/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for ICEnsemble VT17xx
 *
 *   Lowlevel functions for WM8776 codec
 *
 *	Copyright (c) 2012 Ondrej Zary <linux@rainbow-software.org>
 */

use core::ffi::c_char;

/* External dependency types from the surrounding driver/kernel bindings. */
use crate::{snd_card, snd_ctl_elem_type_t};

pub const WM8776_REG_HPLVOL: u32 = 0x00;
pub const WM8776_REG_HPRVOL: u32 = 0x01;
pub const WM8776_REG_HPMASTER: u32 = 0x02;
pub const WM8776_HPVOL_MASK: u32 = 0x17f; /* incl. update bit */
pub const WM8776_VOL_HPZCEN: u32 = 1 << 7; /* zero cross detect */
pub const WM8776_VOL_UPDATE: u32 = 1 << 8; /* update volume */
pub const WM8776_REG_DACLVOL: u32 = 0x03;
pub const WM8776_REG_DACRVOL: u32 = 0x04;
pub const WM8776_REG_DACMASTER: u32 = 0x05;
pub const WM8776_DACVOL_MASK: u32 = 0x1ff; /* incl. update bit */
pub const WM8776_REG_PHASESWAP: u32 = 0x06;
pub const WM8776_PHASE_INVERTL: u32 = 1 << 0;
pub const WM8776_PHASE_INVERTR: u32 = 1 << 1;
pub const WM8776_REG_DACCTRL1: u32 = 0x07;
pub const WM8776_DAC_DZCEN: u32 = 1 << 0;
pub const WM8776_DAC_ATC: u32 = 1 << 1;
pub const WM8776_DAC_IZD: u32 = 1 << 2;
pub const WM8776_DAC_TOD: u32 = 1 << 3;
pub const WM8776_DAC_PL_MASK: u32 = 0xf0;
pub const WM8776_DAC_PL_LL: u32 = 1 << 4; /* L chan: L signal */
pub const WM8776_DAC_PL_LR: u32 = 2 << 4; /* L chan: R signal */
pub const WM8776_DAC_PL_LB: u32 = 3 << 4; /* L chan: both */
pub const WM8776_DAC_PL_RL: u32 = 1 << 6; /* R chan: L signal */
pub const WM8776_DAC_PL_RR: u32 = 2 << 6; /* R chan: R signal */
pub const WM8776_DAC_PL_RB: u32 = 3 << 6; /* R chan: both */
pub const WM8776_REG_DACMUTE: u32 = 0x08;
pub const WM8776_DACMUTE: u32 = 1 << 0;
pub const WM8776_REG_DACCTRL2: u32 = 0x09;
pub const WM8776_DAC2_DEEMPH: u32 = 1 << 0;
pub const WM8776_DAC2_ZFLAG_DISABLE: u32 = 0 << 1;
pub const WM8776_DAC2_ZFLAG_OWN: u32 = 1 << 1;
pub const WM8776_DAC2_ZFLAG_BOTH: u32 = 2 << 1;
pub const WM8776_DAC2_ZFLAG_EITHER: u32 = 3 << 1;
pub const WM8776_REG_DACIFCTRL: u32 = 0x0a;
pub const WM8776_FMT_RIGHTJ: u32 = 0 << 0;
pub const WM8776_FMT_LEFTJ: u32 = 1 << 0;
pub const WM8776_FMT_I2S: u32 = 2 << 0;
pub const WM8776_FMT_DSP: u32 = 3 << 0;
pub const WM8776_FMT_DSP_LATE: u32 = 1 << 2; /* in DSP mode */
pub const WM8776_FMT_LRC_INVERTED: u32 = 1 << 2; /* in other modes */
pub const WM8776_FMT_BCLK_INVERTED: u32 = 1 << 3;
pub const WM8776_FMT_16BIT: u32 = 0 << 4;
pub const WM8776_FMT_20BIT: u32 = 1 << 4;
pub const WM8776_FMT_24BIT: u32 = 2 << 4;
pub const WM8776_FMT_32BIT: u32 = 3 << 4;
pub const WM8776_REG_ADCIFCTRL: u32 = 0x0b;
pub const WM8776_FMT_ADCMCLK_INVERTED: u32 = 1 << 6;
pub const WM8776_FMT_ADCHPD: u32 = 1 << 8;
pub const WM8776_REG_MSTRCTRL: u32 = 0x0c;
pub const WM8776_IF_ADC256FS: u32 = 2 << 0;
pub const WM8776_IF_ADC384FS: u32 = 3 << 0;
pub const WM8776_IF_ADC512FS: u32 = 4 << 0;
pub const WM8776_IF_ADC768FS: u32 = 5 << 0;
pub const WM8776_IF_OVERSAMP64: u32 = 1 << 3;
pub const WM8776_IF_DAC128FS: u32 = 0 << 4;
pub const WM8776_IF_DAC192FS: u32 = 1 << 4;
pub const WM8776_IF_DAC256FS: u32 = 2 << 4;
pub const WM8776_IF_DAC384FS: u32 = 3 << 4;
pub const WM8776_IF_DAC512FS: u32 = 4 << 4;
pub const WM8776_IF_DAC768FS: u32 = 5 << 4;
pub const WM8776_IF_DAC_MASTER: u32 = 1 << 7;
pub const WM8776_IF_ADC_MASTER: u32 = 1 << 8;
pub const WM8776_REG_PWRDOWN: u32 = 0x0d;
pub const WM8776_PWR_PDWN: u32 = 1 << 0;
pub const WM8776_PWR_ADCPD: u32 = 1 << 1;
pub const WM8776_PWR_DACPD: u32 = 1 << 2;
pub const WM8776_PWR_HPPD: u32 = 1 << 3;
pub const WM8776_PWR_AINPD: u32 = 1 << 6;
pub const WM8776_REG_ADCLVOL: u32 = 0x0e;
pub const WM8776_REG_ADCRVOL: u32 = 0x0f;
pub const WM8776_ADC_GAIN_MASK: u32 = 0xff;
pub const WM8776_ADC_ZCEN: u32 = 1 << 8;
pub const WM8776_REG_ALCCTRL1: u32 = 0x10;
pub const WM8776_ALC1_LCT_MASK: u32 = 0x0f; /* 0=-16dB, 1=-15dB..15=-1dB */
pub const WM8776_ALC1_MAXGAIN_MASK: u32 = 0x70; /* 0,1=0dB, 2=+4dB...7=+24dB */
pub const WM8776_ALC1_LCSEL_MASK: u32 = 0x180;
pub const WM8776_ALC1_LCSEL_LIMITER: u32 = 0 << 7;
pub const WM8776_ALC1_LCSEL_ALCR: u32 = 1 << 7;
pub const WM8776_ALC1_LCSEL_ALCL: u32 = 2 << 7;
pub const WM8776_ALC1_LCSEL_ALCSTEREO: u32 = 3 << 7;
pub const WM8776_REG_ALCCTRL2: u32 = 0x11;
pub const WM8776_ALC2_HOLD_MASK: u32 = 0x0f; /*0=0ms, 1=2.67ms, 2=5.33ms.. */
pub const WM8776_ALC2_ZCEN: u32 = 1 << 7;
pub const WM8776_ALC2_LCEN: u32 = 1 << 8;
pub const WM8776_REG_ALCCTRL3: u32 = 0x12;
pub const WM8776_ALC3_ATK_MASK: u32 = 0x0f;
pub const WM8776_ALC3_DCY_MASK: u32 = 0xf0;
pub const WM8776_ALC3_FDECAY: u32 = 1 << 8;
pub const WM8776_REG_NOISEGATE: u32 = 0x13;
pub const WM8776_NGAT_ENABLE: u32 = 1 << 0;
pub const WM8776_NGAT_THR_MASK: u32 = 0x1c; /*0=-78dB, 1=-72dB...7=-36dB */
pub const WM8776_REG_LIMITER: u32 = 0x14;
pub const WM8776_LIM_MAXATTEN_MASK: u32 = 0x0f;
pub const WM8776_LIM_TRANWIN_MASK: u32 = 0x70; /*0=0us, 1=62.5us, 2=125us.. */
pub const WM8776_REG_ADCMUX: u32 = 0x15;
pub const WM8776_ADC_MUX_AIN1: u32 = 1 << 0;
pub const WM8776_ADC_MUX_AIN2: u32 = 1 << 1;
pub const WM8776_ADC_MUX_AIN3: u32 = 1 << 2;
pub const WM8776_ADC_MUX_AIN4: u32 = 1 << 3;
pub const WM8776_ADC_MUX_AIN5: u32 = 1 << 4;
pub const WM8776_ADC_MUTER: u32 = 1 << 6;
pub const WM8776_ADC_MUTEL: u32 = 1 << 7;
pub const WM8776_ADC_LRBOTH: u32 = 1 << 8;
pub const WM8776_REG_OUTMUX: u32 = 0x16;
pub const WM8776_OUTMUX_DAC: u32 = 1 << 0;
pub const WM8776_OUTMUX_AUX: u32 = 1 << 1;
pub const WM8776_OUTMUX_BYPASS: u32 = 1 << 2;
pub const WM8776_REG_RESET: u32 = 0x17;

pub const WM8776_REG_COUNT: usize = 0x17; /* don't cache the RESET register */

#[repr(C)]
pub struct snd_wm8776_ops {
    pub write: Option<unsafe extern "C" fn(wm: *mut snd_wm8776, addr: u8, data: u8)>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum snd_wm8776_ctl_id {
    WM8776_CTL_DAC_VOL,
    WM8776_CTL_DAC_SW,
    WM8776_CTL_DAC_ZC_SW,
    WM8776_CTL_HP_VOL,
    WM8776_CTL_HP_SW,
    WM8776_CTL_HP_ZC_SW,
    WM8776_CTL_AUX_SW,
    WM8776_CTL_BYPASS_SW,
    WM8776_CTL_DAC_IZD_SW,
    WM8776_CTL_PHASE_SW,
    WM8776_CTL_DEEMPH_SW,
    WM8776_CTL_ADC_VOL,
    WM8776_CTL_ADC_SW,
    WM8776_CTL_INPUT1_SW,
    WM8776_CTL_INPUT2_SW,
    WM8776_CTL_INPUT3_SW,
    WM8776_CTL_INPUT4_SW,
    WM8776_CTL_INPUT5_SW,
    WM8776_CTL_AGC_SEL,
    WM8776_CTL_LIM_THR,
    WM8776_CTL_LIM_ATK,
    WM8776_CTL_LIM_DCY,
    WM8776_CTL_LIM_TRANWIN,
    WM8776_CTL_LIM_MAXATTN,
    WM8776_CTL_ALC_TGT,
    WM8776_CTL_ALC_ATK,
    WM8776_CTL_ALC_DCY,
    WM8776_CTL_ALC_MAXGAIN,
    WM8776_CTL_ALC_MAXATTN,
    WM8776_CTL_ALC_HLD,
    WM8776_CTL_NGT_SW,
    WM8776_CTL_NGT_THR,

    WM8776_CTL_COUNT,
}

pub const WM8776_CTL_COUNT: usize = snd_wm8776_ctl_id::WM8776_CTL_COUNT as usize;

pub const WM8776_ENUM_MAX: usize = 16;

pub const WM8776_FLAG_STEREO: u32 = 1 << 0;
pub const WM8776_FLAG_VOL_UPDATE: u32 = 1 << 1;
pub const WM8776_FLAG_INVERT: u32 = 1 << 2;
pub const WM8776_FLAG_LIM: u32 = 1 << 3;
pub const WM8776_FLAG_ALC: u32 = 1 << 4;

#[repr(C)]
pub struct snd_wm8776_ctl {
    pub name: *const c_char,
    pub type_: snd_ctl_elem_type_t,
    pub enum_names: [*const c_char; WM8776_ENUM_MAX],
    pub tlv: *const ::core::ffi::c_uint,
    pub reg1: u16,
    pub reg2: u16,
    pub mask1: u16,
    pub mask2: u16,
    pub min: u16,
    pub max: u16,
    pub flags: u16,
    pub set: Option<unsafe extern "C" fn(wm: *mut snd_wm8776, ch1: u16, ch2: u16)>,
    pub get: Option<unsafe extern "C" fn(wm: *mut snd_wm8776, ch1: *mut u16, ch2: *mut u16)>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum snd_wm8776_agc_mode {
    WM8776_AGC_OFF,
    WM8776_AGC_LIM,
    WM8776_AGC_ALC_R,
    WM8776_AGC_ALC_L,
    WM8776_AGC_ALC_STEREO,
}

#[repr(C)]
pub struct snd_wm8776 {
    pub card: *mut snd_card,
    pub ctl: [snd_wm8776_ctl; WM8776_CTL_COUNT],
    pub agc_mode: snd_wm8776_agc_mode,
    pub ops: snd_wm8776_ops,
    pub regs: [u16; WM8776_REG_COUNT], /* 9-bit registers */
}

unsafe extern "C" {
    pub fn snd_wm8776_init(wm: *mut snd_wm8776);
    pub fn snd_wm8776_resume(wm: *mut snd_wm8776);
    pub fn snd_wm8776_set_power(wm: *mut snd_wm8776, power: u16);
    pub fn snd_wm8776_volume_restore(wm: *mut snd_wm8776);
    pub fn snd_wm8776_build_controls(wm: *mut snd_wm8776) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
