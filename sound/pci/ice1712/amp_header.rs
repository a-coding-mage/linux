/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for VIA VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Advanced Micro Peripherals Ltd AUDIO2000
 *
 *      Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

pub const AMP_AUDIO2000_DEVICE_DESC: &str = "{AMP Ltd,AUDIO2000},{Chaintech,AV-710},";

/*
 * Original C header used `#if 0` to disable:
 * `VT1724_SUBDEVICE_AUDIO2000 = 0x12142417`
 * Advanced Micro Peripherals Ltd AUDIO2000.
 */
pub const VT1724_SUBDEVICE_AUDIO2000: u32 = 0x00030003; /* a dummy ID for AMP Audio2000 */
pub const VT1724_SUBDEVICE_AV710: u32 = 0x12142417; /* AV710 - the same ID with Audio2000! */

/* WM8728 on I2C for AV710 */
pub const WM_DEV: u32 = 0x36;

pub const WM_ATTEN_L: u32 = 0x00;
pub const WM_ATTEN_R: u32 = 0x01;
pub const WM_DAC_CTRL: u32 = 0x02;
pub const WM_INT_CTRL: u32 = 0x03;

unsafe extern "C" {
    pub static mut snd_vt1724_amp_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
