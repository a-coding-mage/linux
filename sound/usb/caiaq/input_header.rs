// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn snd_usb_caiaq_input_dispatch(
        cdev: *mut snd_usb_caiaqdev,
        buf: *mut u8,
        len: u32,
    );

    pub fn snd_usb_caiaq_input_init(cdev: *mut snd_usb_caiaqdev) -> i32;

    pub fn snd_usb_caiaq_input_disconnect(cdev: *mut snd_usb_caiaqdev);

    pub fn snd_usb_caiaq_input_free(cdev: *mut snd_usb_caiaqdev);
}

// Opaque struct type defined elsewhere
pub struct snd_usb_caiaqdev;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
