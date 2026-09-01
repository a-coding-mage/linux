// SPDX-License-Identifier: GPL-2.0-or-later

/*
 *   ALSA driver for VIA VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Audiotrak Prodigy Hifi
 *
 *      Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

pub const PRODIGY_HIFI_DEVICE_DESC: &str = "{Audiotrak,Prodigy 7.1 HIFI},\
                                           {Audiotrak Prodigy HD2},\
                                           {Hercules Fortissimo IV},";

pub const VT1724_SUBDEVICE_PRODIGY_HIFI: u32 = 0x38315441; /* PRODIGY 7.1 HIFI */
pub const VT1724_SUBDEVICE_PRODIGY_HD2: u32 = 0x37315441; /* PRODIGY HD2 */
pub const VT1724_SUBDEVICE_FORTISSIMO4: u32 = 0x81160100; /* Fortissimo IV */

unsafe extern "C" {
    pub static mut snd_vt1724_prodigy_hifi_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
