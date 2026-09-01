/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for VIA VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Pontis MS300 boards
 *
 *	Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

pub const PONTIS_DEVICE_DESC: &str = "{Pontis,MS300},";

pub const VT1720_SUBDEVICE_PONTIS_MS300: u32 = 0x00020002; /* a dummy id for MS300 */

unsafe extern "C" {
    pub static mut snd_vt1720_pontis_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
