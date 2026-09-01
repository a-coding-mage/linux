// SPDX-License-Identifier: GPL-2.0-or-later

/*
 *   ALSA driver for VT1720/VT1724 (Envy24PT/Envy24HT)
 *
 *   Lowlevel functions for VT1720-based motherboards
 *
 *	Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

pub const VT1720_MOBO_DEVICE_DESC: &str = concat!(
    "{Albatron,K8X800 Pro II},",
    "{Chaintech,ZNF3-150},",
    "{Chaintech,ZNF3-250},",
    "{Chaintech,9CJS},",
    "{Shuttle,SN25P},",
);

pub const VT1720_SUBDEVICE_K8X800: u32 = 0xf217052c;
pub const VT1720_SUBDEVICE_ZNF3_150: u32 = 0x0f2741f6;
pub const VT1720_SUBDEVICE_ZNF3_250: u32 = 0x0f2745f6;
pub const VT1720_SUBDEVICE_9CJS: u32 = 0x0f272327;
pub const VT1720_SUBDEVICE_SN25P: u32 = 0x97123650;

unsafe extern "C" {
    pub static mut snd_vt1720_mobo_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
