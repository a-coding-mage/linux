/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for ICEnsemble VT17xx
 *
 *   Lowlevel functions for WM8766 codec
 *
 *	Copyright (c) 2012 Ondrej Zary <linux@rainbow-software.org>
 */

pub const WM8766_REG_DACL1: u16 = 0x00;
pub const WM8766_REG_DACR1: u16 = 0x01;
pub const WM8766_VOL_MASK: u16 = 0x1ff; /* incl. update bit */
pub const WM8766_VOL_UPDATE: u16 = 1 << 8; /* update volume */
pub const WM8766_REG_DACCTRL1: u16 = 0x02;
pub const WM8766_DAC_MUTEALL: u16 = 1 << 0;
pub const WM8766_DAC_DEEMPALL: u16 = 1 << 1;
pub const WM8766_DAC_PDWN: u16 = 1 << 2;
pub const WM8766_DAC_ATC: u16 = 1 << 3;
pub const WM8766_DAC_IZD: u16 = 1 << 4;
pub const WM8766_DAC_PL_MASK: u16 = 0x1e0;
pub const WM8766_DAC_PL_LL: u16 = 1 << 5; /* L chan: L signal */
pub const WM8766_DAC_PL_LR: u16 = 2 << 5; /* L chan: R signal */
pub const WM8766_DAC_PL_LB: u16 = 3 << 5; /* L chan: both */
pub const WM8766_DAC_PL_RL: u16 = 1 << 7; /* R chan: L signal */
pub const WM8766_DAC_PL_RR: u16 = 2 << 7; /* R chan: R signal */
pub const WM8766_DAC_PL_RB: u16 = 3 << 7; /* R chan: both */
pub const WM8766_REG_IFCTRL: u16 = 0x03;
pub const WM8766_IF_FMT_RIGHTJ: u16 = 0 << 0;
pub const WM8766_IF_FMT_LEFTJ: u16 = 1 << 0;
pub const WM8766_IF_FMT_I2S: u16 = 2 << 0;
pub const WM8766_IF_FMT_DSP: u16 = 3 << 0;
pub const WM8766_IF_DSP_LATE: u16 = 1 << 2; /* in DSP mode */
pub const WM8766_IF_LRC_INVERTED: u16 = 1 << 2; /* in other modes */
pub const WM8766_IF_BCLK_INVERTED: u16 = 1 << 3;
pub const WM8766_IF_IWL_16BIT: u16 = 0 << 4;
pub const WM8766_IF_IWL_20BIT: u16 = 1 << 4;
pub const WM8766_IF_IWL_24BIT: u16 = 2 << 4;
pub const WM8766_IF_IWL_32BIT: u16 = 3 << 4;
pub const WM8766_IF_MASK: u16 = 0x3f;
pub const WM8766_PHASE_INVERT1: u16 = 1 << 6;
pub const WM8766_PHASE_INVERT2: u16 = 1 << 7;
pub const WM8766_PHASE_INVERT3: u16 = 1 << 8;
pub const WM8766_REG_DACL2: u16 = 0x04;
pub const WM8766_REG_DACR2: u16 = 0x05;
pub const WM8766_REG_DACL3: u16 = 0x06;
pub const WM8766_REG_DACR3: u16 = 0x07;
pub const WM8766_REG_MASTDA: u16 = 0x08;
pub const WM8766_REG_DACCTRL2: u16 = 0x09;
pub const WM8766_DAC2_ZCD: u16 = 1 << 0;
pub const WM8766_DAC2_ZFLAG_ALL: u16 = 0 << 1;
pub const WM8766_DAC2_ZFLAG_1: u16 = 1 << 1;
pub const WM8766_DAC2_ZFLAG_2: u16 = 2 << 1;
pub const WM8766_DAC2_ZFLAG_3: u16 = 3 << 1;
pub const WM8766_DAC2_MUTE1: u16 = 1 << 3;
pub const WM8766_DAC2_MUTE2: u16 = 1 << 4;
pub const WM8766_DAC2_MUTE3: u16 = 1 << 5;
pub const WM8766_DAC2_DEEMP1: u16 = 1 << 6;
pub const WM8766_DAC2_DEEMP2: u16 = 1 << 7;
pub const WM8766_DAC2_DEEMP3: u16 = 1 << 8;
pub const WM8766_REG_DACCTRL3: u16 = 0x0a;
pub const WM8766_DAC3_DACPD1: u16 = 1 << 1;
pub const WM8766_DAC3_DACPD2: u16 = 1 << 2;
pub const WM8766_DAC3_DACPD3: u16 = 1 << 3;
pub const WM8766_DAC3_PWRDNALL: u16 = 1 << 4;
pub const WM8766_DAC3_POWER_MASK: u16 = 0x1e;
pub const WM8766_DAC3_MASTER: u16 = 1 << 5;
pub const WM8766_DAC3_DAC128FS: u16 = 0 << 6;
pub const WM8766_DAC3_DAC192FS: u16 = 1 << 6;
pub const WM8766_DAC3_DAC256FS: u16 = 2 << 6;
pub const WM8766_DAC3_DAC384FS: u16 = 3 << 6;
pub const WM8766_DAC3_DAC512FS: u16 = 4 << 6;
pub const WM8766_DAC3_DAC768FS: u16 = 5 << 6;
pub const WM8766_DAC3_MSTR_MASK: u16 = 0x1e0;
pub const WM8766_REG_MUTE1: u16 = 0x0c;
pub const WM8766_MUTE1_MPD: u16 = 1 << 6;
pub const WM8766_REG_MUTE2: u16 = 0x0f;
pub const WM8766_MUTE2_MPD: u16 = 1 << 5;
pub const WM8766_REG_RESET: u16 = 0x1f;

pub const WM8766_REG_COUNT: usize = 0x10; /* don't cache the RESET register */

#[repr(C)]
pub struct snd_wm8766_ops {
    pub write: Option<unsafe extern "C" fn(wm: *mut snd_wm8766, addr: u16, data: u16)>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum snd_wm8766_ctl_id {
    WM8766_CTL_CH1_VOL,
    WM8766_CTL_CH2_VOL,
    WM8766_CTL_CH3_VOL,
    WM8766_CTL_CH1_SW,
    WM8766_CTL_CH2_SW,
    WM8766_CTL_CH3_SW,
    WM8766_CTL_PHASE1_SW,
    WM8766_CTL_PHASE2_SW,
    WM8766_CTL_PHASE3_SW,
    WM8766_CTL_DEEMPH1_SW,
    WM8766_CTL_DEEMPH2_SW,
    WM8766_CTL_DEEMPH3_SW,
    WM8766_CTL_IZD_SW,
    WM8766_CTL_ZC_SW,

    WM8766_CTL_COUNT,
}

pub const WM8766_CTL_COUNT: usize = snd_wm8766_ctl_id::WM8766_CTL_COUNT as usize;

pub const WM8766_ENUM_MAX: usize = 16;

pub const WM8766_FLAG_STEREO: u16 = 1 << 0;
pub const WM8766_FLAG_VOL_UPDATE: u16 = 1 << 1;
pub const WM8766_FLAG_INVERT: u16 = 1 << 2;
pub const WM8766_FLAG_LIM: u16 = 1 << 3;
pub const WM8766_FLAG_ALC: u16 = 1 << 4;

#[repr(C)]
pub struct snd_wm8766_ctl {
    pub kctl: *mut crate::snd_kcontrol,
    pub name: *const ::core::ffi::c_char,
    pub type_: crate::snd_ctl_elem_type_t,
    pub enum_names: [*const ::core::ffi::c_char; WM8766_ENUM_MAX],
    pub tlv: *const ::core::ffi::c_uint,
    pub reg1: u16,
    pub reg2: u16,
    pub mask1: u16,
    pub mask2: u16,
    pub min: u16,
    pub max: u16,
    pub flags: u16,
    pub set: Option<unsafe extern "C" fn(wm: *mut snd_wm8766, ch1: u16, ch2: u16)>,
    pub get: Option<unsafe extern "C" fn(wm: *mut snd_wm8766, ch1: *mut u16, ch2: *mut u16)>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum snd_wm8766_agc_mode {
    WM8766_AGC_OFF,
    WM8766_AGC_LIM,
    WM8766_AGC_ALC,
}

#[repr(C)]
pub struct snd_wm8766 {
    pub card: *mut crate::snd_card,
    pub ctl: [snd_wm8766_ctl; WM8766_CTL_COUNT],
    pub agc_mode: snd_wm8766_agc_mode,
    pub ops: snd_wm8766_ops,
    pub regs: [u16; WM8766_REG_COUNT], /* 9-bit registers */
}

unsafe extern "C" {
    pub fn snd_wm8766_init(wm: *mut snd_wm8766);
    pub fn snd_wm8766_resume(wm: *mut snd_wm8766);
    pub fn snd_wm8766_set_if(wm: *mut snd_wm8766, dac: u16);
    pub fn snd_wm8766_volume_restore(wm: *mut snd_wm8766);
    pub fn snd_wm8766_build_controls(wm: *mut snd_wm8766) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
