/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from C header: pci/ice1712/psc724.h */
/* Original header guard: __SOUND_PSC724_H */

/* ID */
pub const PSC724_DEVICE_DESC: &str = "{Philips,PSC724 Ultimate Edge},";

pub const VT1724_SUBDEVICE_PSC724: u32 = 0xab170619;

/* entry struct */
unsafe extern "C" {
    pub static mut snd_vt1724_psc724_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
