// SPDX-License-Identifier: GPL-2.0
//
// Translated from pci/ice1712/se.h.
// C header guard removed; declarations here depend on snd_ice1712_card_info
// being provided by the surrounding translation unit/crate.

/* ID */
pub const SE_DEVICE_DESC: &str = "{ONKYO INC,SE-90PCI},{ONKYO INC,SE-200PCI},";

pub const VT1724_SUBDEVICE_SE90PCI: u32 = 0x0b161000;
pub const VT1724_SUBDEVICE_SE200PCI: u32 = 0x0b160100;

/* entry struct */
unsafe extern "C" {
    pub static mut snd_vt1724_se_cards: [snd_ice1712_card_info; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
