/* SPDX-License-Identifier: GPL-2.0 */

pub const MAYA44_DEVICE_DESC: &str = "{ESI,Maya44},";

/* Maya44 */
pub const VT1724_SUBDEVICE_MAYA44: u32 = 0x34315441;

unsafe extern "C" {
    /*
     * C declaration:
     * extern struct snd_ice1712_card_info snd_vt1724_maya44_cards[];
     *
     * Rust cannot declare an extern static with an incomplete array type.
     * This preserves the symbol as the first element of the external array.
     */
    pub static mut snd_vt1724_maya44_cards: snd_ice1712_card_info;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
