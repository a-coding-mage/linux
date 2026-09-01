/* SPDX-License-Identifier: GPL-2.0 */

pub const QTET_DEVICE_DESC: &str = "{Infrasonic,Quartet},";

/* Infrasonic Quartet */
pub const VT1724_SUBDEVICE_QTET: u32 = 0x30305349;

unsafe extern "C" {
    pub static mut snd_vt1724_qtet_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
