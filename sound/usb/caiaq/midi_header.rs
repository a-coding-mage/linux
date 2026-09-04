// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn snd_usb_caiaq_midi_init(cdev: *mut snd_usb_caiaqdev) -> i32;
    pub fn snd_usb_caiaq_midi_handle_input(
        cdev: *mut snd_usb_caiaqdev,
        port: i32,
        buf: *const i8,
        len: i32,
    );
    pub fn snd_usb_caiaq_midi_output_done(urb: *mut urb);
}

// External types referenced from other modules
pub enum snd_usb_caiaqdev {}
pub enum urb {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
