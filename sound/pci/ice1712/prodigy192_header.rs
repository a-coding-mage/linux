/* SPDX-License-Identifier: GPL-2.0 */

/* Header guard __SOUND_PRODIGY192_H omitted in Rust. */

pub const PRODIGY192_DEVICE_DESC: &str = "{AudioTrak,Prodigy 192},";
pub const PRODIGY192_STAC9460_ADDR: u32 = 0x54;

pub const VT1724_SUBDEVICE_PRODIGY192VE: u32 = 0x34495345; /* PRODIGY 192 VE */

/*
 *  AudioTrak Prodigy192 GPIO definitions for MI/ODI/O card with
 *  AK4114 (SPDIF-IN)
 */
pub const VT1724_PRODIGY192_CS: u32 = 1u32 << 8; /* GPIO8, pin 75 */
pub const VT1724_PRODIGY192_CCLK: u32 = 1u32 << 9; /* GPIO9, pin 76 */
pub const VT1724_PRODIGY192_CDOUT: u32 = 1u32 << 10; /* GPIO10, pin 77 */
pub const VT1724_PRODIGY192_CDIN: u32 = 1u32 << 11; /* GPIO11, pin 86 */

unsafe extern "C" {
    /*
     * C declaration:
     * extern struct snd_ice1712_card_info snd_vt1724_prodigy192_cards[];
     *
     * Rust cannot express an extern static incomplete array length locally;
     * declare the first element symbol with the same external name.
     */
    pub static mut snd_vt1724_prodigy192_cards: snd_ice1712_card_info;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
