// SPDX-License-Identifier: GPL-2.0

// Opaque types defined in other kernel headers
pub struct snd_card;
pub struct usb_device;

extern "C" {
    pub fn usx2y_hwdep_new(card: *mut snd_card, device: *mut usb_device) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
