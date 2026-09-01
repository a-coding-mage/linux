/* SPDX-License-Identifier: GPL-2.0 */

pub const JULI_DEVICE_DESC: &[u8] = b"{ESI,Juli@},\0";

pub const VT1724_SUBDEVICE_JULI: u32 = 0x31305345; /* Juli@ */

unsafe extern "C" {
    pub static mut snd_vt1724_juli_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
